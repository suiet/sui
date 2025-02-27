// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::{collections::HashSet, fmt::Debug, sync::Arc, thread};

use async_trait::async_trait;
use mysten_metrics::{metered_channel, monitored_scope};
use thiserror::Error;
use tokio::sync::oneshot;
use tokio::sync::oneshot::error::RecvError;
use tracing::warn;

use crate::{
    block::{BlockRef, Round, VerifiedBlock},
    context::Context,
    core::Core,
    core_thread::CoreError::Shutdown,
};

const CORE_THREAD_COMMANDS_CHANNEL_SIZE: usize = 32;

enum CoreThreadCommand {
    /// Add blocks to be processed and accepted
    AddBlocks(Vec<VerifiedBlock>, oneshot::Sender<Vec<BlockRef>>),
    /// Called when a leader timeout occurs and a block should be produced
    ForceNewBlock(Round, oneshot::Sender<()>),
    /// Request missing blocks that need to be synced.
    GetMissing(oneshot::Sender<Vec<HashSet<BlockRef>>>),
}

#[derive(Error, Debug)]
pub(crate) enum CoreError {
    #[error("Core thread shutdown: {0}")]
    Shutdown(RecvError),
}

/// The interface to adhere the implementations of the core thread dispatcher. Also allows the easier mocking during unit tests.
#[async_trait]
pub(crate) trait CoreThreadDispatcher: Sync + Send + 'static {
    async fn add_blocks(&self, blocks: Vec<VerifiedBlock>) -> Result<Vec<BlockRef>, CoreError>;

    async fn force_new_block(&self, round: Round) -> Result<(), CoreError>;

    async fn get_missing_blocks(&self) -> Result<Vec<HashSet<BlockRef>>, CoreError>;
}

#[allow(unused)]
pub(crate) struct CoreThreadHandle {
    sender: metered_channel::Sender<CoreThreadCommand>,
    join_handle: thread::JoinHandle<()>,
}

impl CoreThreadHandle {
    #[allow(unused)]
    pub fn stop(self) {
        // drop the sender, that will force all the other weak senders to not able to upgrade.
        drop(self.sender);
        self.join_handle.join().ok();
    }
}

#[allow(unused)]
struct CoreThread {
    core: Core,
    receiver: metered_channel::Receiver<CoreThreadCommand>,
    context: Arc<Context>,
}

impl CoreThread {
    pub fn run(mut self) {
        tracing::debug!("Started core thread");

        while let Some(command) = self.receiver.blocking_recv() {
            let _scope = monitored_scope("CoreThread::loop");
            self.context.metrics.node_metrics.core_lock_dequeued.inc();
            match command {
                CoreThreadCommand::AddBlocks(blocks, sender) => {
                    let missing_blocks = self.core.add_blocks(blocks);
                    sender.send(missing_blocks).ok();
                }
                CoreThreadCommand::ForceNewBlock(round, sender) => {
                    self.core.force_new_block(round);
                    sender.send(()).ok();
                }
                CoreThreadCommand::GetMissing(sender) => {
                    // TODO: implement the logic to fetch the missing blocks.
                    sender.send(vec![]).ok();
                }
            }
        }
    }
}

#[derive(Clone)]
pub(crate) struct ChannelCoreThreadDispatcher {
    sender: metered_channel::WeakSender<CoreThreadCommand>,
    context: Arc<Context>,
}

impl ChannelCoreThreadDispatcher {
    pub fn start(core: Core, context: Arc<Context>) -> (Self, CoreThreadHandle) {
        let (sender, receiver) = metered_channel::channel_with_total(
            CORE_THREAD_COMMANDS_CHANNEL_SIZE,
            &context.metrics.channel_metrics.core_thread,
            &context.metrics.channel_metrics.core_thread_total,
        );
        let core_thread = CoreThread {
            core,
            receiver,
            context: context.clone(),
        };
        let join_handle = thread::Builder::new()
            .name("consensus-core".to_string())
            .spawn(move || core_thread.run())
            .unwrap();
        // Explicitly using downgraded sender in order to allow sharing the CoreThreadDispatcher but
        // able to shutdown the CoreThread by dropping the original sender.
        let dispatcher = ChannelCoreThreadDispatcher {
            sender: sender.downgrade(),
            context,
        };
        let handle = CoreThreadHandle {
            join_handle,
            sender,
        };
        (dispatcher, handle)
    }

    async fn send(&self, command: CoreThreadCommand) {
        self.context.metrics.node_metrics.core_lock_enqueued.inc();
        if let Some(sender) = self.sender.upgrade() {
            if let Err(err) = sender.send(command).await {
                warn!(
                    "Couldn't send command to core thread, probably is shutting down: {}",
                    err
                );
            }
        }
    }
}

#[async_trait]
impl CoreThreadDispatcher for ChannelCoreThreadDispatcher {
    async fn add_blocks(&self, blocks: Vec<VerifiedBlock>) -> Result<Vec<BlockRef>, CoreError> {
        let (sender, receiver) = oneshot::channel();
        self.send(CoreThreadCommand::AddBlocks(blocks, sender))
            .await;
        receiver.await.map_err(Shutdown)
    }

    async fn force_new_block(&self, round: Round) -> Result<(), CoreError> {
        let (sender, receiver) = oneshot::channel();
        self.send(CoreThreadCommand::ForceNewBlock(round, sender))
            .await;
        receiver.await.map_err(Shutdown)
    }

    async fn get_missing_blocks(&self) -> Result<Vec<HashSet<BlockRef>>, CoreError> {
        let (sender, receiver) = oneshot::channel();
        self.send(CoreThreadCommand::GetMissing(sender)).await;
        receiver.await.map_err(Shutdown)
    }
}

#[cfg(test)]
mod test {
    use parking_lot::RwLock;

    use super::*;
    use crate::block_manager::BlockManager;
    use crate::context::Context;
    use crate::core::CoreSignals;
    use crate::dag_state::DagState;
    use crate::storage::mem_store::MemStore;
    use crate::transaction::{TransactionClient, TransactionConsumer};

    #[tokio::test]
    async fn test_core_thread() {
        let (context, mut key_pairs) = Context::new_for_test(4);
        let context = Arc::new(context);
        let store = Arc::new(MemStore::new());
        let dag_state = Arc::new(RwLock::new(DagState::new(context.clone(), store.clone())));
        let block_manager = BlockManager::new(context.clone(), dag_state);
        let (_transaction_client, tx_receiver) = TransactionClient::new(context.clone());
        let transaction_consumer = TransactionConsumer::new(tx_receiver, context.clone(), None);
        let (signals, _signal_receivers) = CoreSignals::new();

        let core = Core::new(
            context.clone(),
            transaction_consumer,
            block_manager,
            signals,
            key_pairs.remove(context.own_index.value()).1,
            store,
        );

        let (core_dispatcher, handle) = ChannelCoreThreadDispatcher::start(core, context);

        // Now create some clones of the dispatcher
        let dispatcher_1 = core_dispatcher.clone();
        let dispatcher_2 = core_dispatcher.clone();

        // Try to send some commands
        assert!(dispatcher_1.add_blocks(vec![]).await.is_ok());
        assert!(dispatcher_2.add_blocks(vec![]).await.is_ok());

        // Now shutdown the dispatcher
        handle.stop();

        // Try to send some commands
        assert!(dispatcher_1.add_blocks(vec![]).await.is_err());
        assert!(dispatcher_2.add_blocks(vec![]).await.is_err());
    }
}
