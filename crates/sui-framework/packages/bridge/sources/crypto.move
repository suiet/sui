// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

module bridge::crypto {

    use std::vector;
    use sui::ecdsa_k1;
    use sui::hash::keccak256;
    #[test_only]
    use sui::hex;

    public fun ecdsa_pub_key_to_eth_address(compressed_pub_key: vector<u8>): vector<u8> {
        // Decompress pub key
        let decompressed = ecdsa_k1::decompress_pubkey(&compressed_pub_key);

        // Remove first byte
        let (i, decompressed_64) = (1, vector[]);
        while (i < 65) {
            let value = vector::borrow(&decompressed, i);
            vector::push_back(&mut decompressed_64, *value);
            i = i + 1;
        };

        // Hash
        let hash = keccak256(&decompressed_64);

        // Take last 20 bytes
        let address = vector[];
        let i = 12;
        while (i < 32) {
            vector::push_back(&mut address, *vector::borrow(&hash, i));
            i = i + 1;
        };
        address
    }

    #[test]
    fun test_pub_key_to_eth_address() {
        let validator_pub_key = hex::decode(b"029bef8d556d80e43ae7e0becb3a7e6838b95defe45896ed6075bb9035d06c9964");
        let expected_address = hex::decode(b"b14d3c4f5fbfbcfb98af2d330000d49c95b93aa7");
        assert!(ecdsa_pub_key_to_eth_address(validator_pub_key) == expected_address, 0);
    }
}
