
<a name="0x2_bcs"></a>

# Module `0x2::bcs`



-  [Struct `BCS`](#0x2_bcs_BCS)
-  [Constants](#@Constants_0)
-  [Function `to_bytes`](#0x2_bcs_to_bytes)
-  [Function `new`](#0x2_bcs_new)
-  [Function `into_remainder_bytes`](#0x2_bcs_into_remainder_bytes)
-  [Function `peel_address`](#0x2_bcs_peel_address)
-  [Function `peel_bool`](#0x2_bcs_peel_bool)
-  [Function `peel_u8`](#0x2_bcs_peel_u8)
-  [Function `peel_u64`](#0x2_bcs_peel_u64)
-  [Function `peel_u128`](#0x2_bcs_peel_u128)
-  [Function `peel_u256`](#0x2_bcs_peel_u256)
-  [Function `peel_vec_length`](#0x2_bcs_peel_vec_length)
-  [Function `peel_vec_address`](#0x2_bcs_peel_vec_address)
-  [Function `peel_vec_bool`](#0x2_bcs_peel_vec_bool)
-  [Function `peel_vec_u8`](#0x2_bcs_peel_vec_u8)
-  [Function `peel_vec_vec_u8`](#0x2_bcs_peel_vec_vec_u8)
-  [Function `peel_vec_u64`](#0x2_bcs_peel_vec_u64)
-  [Function `peel_vec_u128`](#0x2_bcs_peel_vec_u128)
-  [Function `peel_option_address`](#0x2_bcs_peel_option_address)
-  [Function `peel_option_bool`](#0x2_bcs_peel_option_bool)
-  [Function `peel_option_u8`](#0x2_bcs_peel_option_u8)
-  [Function `peel_option_u64`](#0x2_bcs_peel_option_u64)
-  [Function `peel_option_u128`](#0x2_bcs_peel_option_u128)


<pre><code><b>use</b> <a href="../../dependencies/move-stdlib/bcs.md#0x1_bcs">0x1::bcs</a>;
<b>use</b> <a href="../../dependencies/move-stdlib/option.md#0x1_option">0x1::option</a>;
<b>use</b> <a href="../../dependencies/move-stdlib/vector.md#0x1_vector">0x1::vector</a>;
<b>use</b> <a href="../../dependencies/sui-framework/address.md#0x2_address">0x2::address</a>;
</code></pre>



<a name="0x2_bcs_BCS"></a>

## Struct `BCS`



<pre><code><b>struct</b> <a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_BCS">BCS</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>bytes: <a href="../../dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="0x2_bcs_ELenOutOfRange"></a>



<pre><code><b>const</b> <a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_ELenOutOfRange">ELenOutOfRange</a>: u64 = 2;
</code></pre>



<a name="0x2_bcs_ENotBool"></a>



<pre><code><b>const</b> <a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_ENotBool">ENotBool</a>: u64 = 1;
</code></pre>



<a name="0x2_bcs_EOutOfRange"></a>



<pre><code><b>const</b> <a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_EOutOfRange">EOutOfRange</a>: u64 = 0;
</code></pre>



<a name="0x2_bcs_to_bytes"></a>

## Function `to_bytes`



<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_to_bytes">to_bytes</a>&lt;T&gt;(value: &T): <a href="../../dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_to_bytes">to_bytes</a>&lt;T&gt;(value: &T): <a href="../../dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt; {
    <a href="../../dependencies/move-stdlib/bcs.md#0x1_bcs_to_bytes">bcs::to_bytes</a>(value)
}
</code></pre>



</details>

<a name="0x2_bcs_new"></a>

## Function `new`



<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_new">new</a>(bytes: <a href="../../dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;): bcs::BCS
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_new">new</a>(bytes: <a href="../../dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;): <a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_BCS">BCS</a> {
    v::reverse(&<b>mut</b> bytes);
    <a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_BCS">BCS</a> { bytes }
}
</code></pre>



</details>

<a name="0x2_bcs_into_remainder_bytes"></a>

## Function `into_remainder_bytes`



<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_into_remainder_bytes">into_remainder_bytes</a>(<a href="../../dependencies/move-stdlib/bcs.md#0x1_bcs">bcs</a>: bcs::BCS): <a href="../../dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_into_remainder_bytes">into_remainder_bytes</a>(<a href="../../dependencies/move-stdlib/bcs.md#0x1_bcs">bcs</a>: <a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_BCS">BCS</a>): <a href="../../dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt; {
    <b>let</b> <a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_BCS">BCS</a> { bytes } = <a href="../../dependencies/move-stdlib/bcs.md#0x1_bcs">bcs</a>;
    v::reverse(&<b>mut</b> bytes);
    bytes
}
</code></pre>



</details>

<a name="0x2_bcs_peel_address"></a>

## Function `peel_address`



<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_peel_address">peel_address</a>(<a href="../../dependencies/move-stdlib/bcs.md#0x1_bcs">bcs</a>: &<b>mut</b> bcs::BCS): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_peel_address">peel_address</a>(<a href="../../dependencies/move-stdlib/bcs.md#0x1_bcs">bcs</a>: &<b>mut</b> <a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_BCS">BCS</a>): <b>address</b> {
    <b>assert</b>!(v::length(&<a href="../../dependencies/move-stdlib/bcs.md#0x1_bcs">bcs</a>.bytes) &gt;= <a href="../../dependencies/move-stdlib/address.md#0x1_address_length">address::length</a>(), <a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_EOutOfRange">EOutOfRange</a>);
    <b>let</b> (addr_bytes, i) = (v::empty(), 0);
    <b>while</b> (i &lt; <a href="../../dependencies/move-stdlib/address.md#0x1_address_length">address::length</a>()) {
        v::push_back(&<b>mut</b> addr_bytes, v::pop_back(&<b>mut</b> <a href="../../dependencies/move-stdlib/bcs.md#0x1_bcs">bcs</a>.bytes));
        i = i + 1;
    };
    address::from_bytes(addr_bytes)
}
</code></pre>



</details>

<a name="0x2_bcs_peel_bool"></a>

## Function `peel_bool`



<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_peel_bool">peel_bool</a>(<a href="../../dependencies/move-stdlib/bcs.md#0x1_bcs">bcs</a>: &<b>mut</b> bcs::BCS): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_peel_bool">peel_bool</a>(<a href="../../dependencies/move-stdlib/bcs.md#0x1_bcs">bcs</a>: &<b>mut</b> <a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_BCS">BCS</a>): bool {
    <b>let</b> value = <a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_peel_u8">peel_u8</a>(<a href="../../dependencies/move-stdlib/bcs.md#0x1_bcs">bcs</a>);
    <b>if</b> (value == 0) {
        <b>false</b>
    } <b>else</b> <b>if</b> (value == 1) {
        <b>true</b>
    } <b>else</b> {
        <b>abort</b> <a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_ENotBool">ENotBool</a>
    }
}
</code></pre>



</details>

<a name="0x2_bcs_peel_u8"></a>

## Function `peel_u8`



<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_peel_u8">peel_u8</a>(<a href="../../dependencies/move-stdlib/bcs.md#0x1_bcs">bcs</a>: &<b>mut</b> bcs::BCS): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_peel_u8">peel_u8</a>(<a href="../../dependencies/move-stdlib/bcs.md#0x1_bcs">bcs</a>: &<b>mut</b> <a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_BCS">BCS</a>): u8 {
    <b>assert</b>!(v::length(&<a href="../../dependencies/move-stdlib/bcs.md#0x1_bcs">bcs</a>.bytes) &gt;= 1, <a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_EOutOfRange">EOutOfRange</a>);
    v::pop_back(&<b>mut</b> <a href="../../dependencies/move-stdlib/bcs.md#0x1_bcs">bcs</a>.bytes)
}
</code></pre>



</details>

<a name="0x2_bcs_peel_u64"></a>

## Function `peel_u64`



<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_peel_u64">peel_u64</a>(<a href="../../dependencies/move-stdlib/bcs.md#0x1_bcs">bcs</a>: &<b>mut</b> bcs::BCS): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_peel_u64">peel_u64</a>(<a href="../../dependencies/move-stdlib/bcs.md#0x1_bcs">bcs</a>: &<b>mut</b> <a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_BCS">BCS</a>): u64 {
    <b>assert</b>!(v::length(&<a href="../../dependencies/move-stdlib/bcs.md#0x1_bcs">bcs</a>.bytes) &gt;= 8, <a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_EOutOfRange">EOutOfRange</a>);

    <b>let</b> (value, i) = (0u64, 0u8);
    <b>while</b> (i &lt; 64) {
        <b>let</b> byte = (v::pop_back(&<b>mut</b> <a href="../../dependencies/move-stdlib/bcs.md#0x1_bcs">bcs</a>.bytes) <b>as</b> u64);
        value = value + (byte &lt;&lt; i);
        i = i + 8;
    };

    value
}
</code></pre>



</details>

<a name="0x2_bcs_peel_u128"></a>

## Function `peel_u128`



<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_peel_u128">peel_u128</a>(<a href="../../dependencies/move-stdlib/bcs.md#0x1_bcs">bcs</a>: &<b>mut</b> bcs::BCS): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_peel_u128">peel_u128</a>(<a href="../../dependencies/move-stdlib/bcs.md#0x1_bcs">bcs</a>: &<b>mut</b> <a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_BCS">BCS</a>): u128 {
    <b>assert</b>!(v::length(&<a href="../../dependencies/move-stdlib/bcs.md#0x1_bcs">bcs</a>.bytes) &gt;= 16, <a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_EOutOfRange">EOutOfRange</a>);

    <b>let</b> (value, i) = (0u128, 0u8);
    <b>while</b> (i &lt; 128) {
        <b>let</b> byte = (v::pop_back(&<b>mut</b> <a href="../../dependencies/move-stdlib/bcs.md#0x1_bcs">bcs</a>.bytes) <b>as</b> u128);
        value = value + (byte &lt;&lt; i);
        i = i + 8;
    };

    value
}
</code></pre>



</details>

<a name="0x2_bcs_peel_u256"></a>

## Function `peel_u256`



<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_peel_u256">peel_u256</a>(<a href="../../dependencies/move-stdlib/bcs.md#0x1_bcs">bcs</a>: &<b>mut</b> bcs::BCS): u256
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_peel_u256">peel_u256</a>(<a href="../../dependencies/move-stdlib/bcs.md#0x1_bcs">bcs</a>: &<b>mut</b> <a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_BCS">BCS</a>): u256 {
    <b>assert</b>!(v::length(&<a href="../../dependencies/move-stdlib/bcs.md#0x1_bcs">bcs</a>.bytes) &gt;= 32, <a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_EOutOfRange">EOutOfRange</a>);

    <b>let</b> (value, i) = (0u256, 0u16);
    <b>while</b> (i &lt; 256) {
        <b>let</b> byte = (v::pop_back(&<b>mut</b> <a href="../../dependencies/move-stdlib/bcs.md#0x1_bcs">bcs</a>.bytes) <b>as</b> u256);
        value = value + (byte &lt;&lt; (i <b>as</b> u8));
        i = i + 8;
    };

    value
}
</code></pre>



</details>

<a name="0x2_bcs_peel_vec_length"></a>

## Function `peel_vec_length`



<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_peel_vec_length">peel_vec_length</a>(<a href="../../dependencies/move-stdlib/bcs.md#0x1_bcs">bcs</a>: &<b>mut</b> bcs::BCS): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_peel_vec_length">peel_vec_length</a>(<a href="../../dependencies/move-stdlib/bcs.md#0x1_bcs">bcs</a>: &<b>mut</b> <a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_BCS">BCS</a>): u64 {
    <b>let</b> (total, shift, len) = (0u64, 0, 0);
    <b>while</b> (<b>true</b>) {
        <b>assert</b>!(len &lt;= 4, <a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_ELenOutOfRange">ELenOutOfRange</a>);
        <b>let</b> byte = (v::pop_back(&<b>mut</b> <a href="../../dependencies/move-stdlib/bcs.md#0x1_bcs">bcs</a>.bytes) <b>as</b> u64);
        len = len + 1;
        total = total | ((byte & 0x7f) &lt;&lt; shift);
        <b>if</b> ((byte & 0x80) == 0) {
            <b>break</b>
        };
        shift = shift + 7;
    };
    total
}
</code></pre>



</details>

<a name="0x2_bcs_peel_vec_address"></a>

## Function `peel_vec_address`



<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_peel_vec_address">peel_vec_address</a>(<a href="../../dependencies/move-stdlib/bcs.md#0x1_bcs">bcs</a>: &<b>mut</b> bcs::BCS): <a href="../../dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;<b>address</b>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_peel_vec_address">peel_vec_address</a>(<a href="../../dependencies/move-stdlib/bcs.md#0x1_bcs">bcs</a>: &<b>mut</b> <a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_BCS">BCS</a>): <a href="../../dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;<b>address</b>&gt; {
    <b>let</b> (len, i, res) = (<a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_peel_vec_length">peel_vec_length</a>(<a href="../../dependencies/move-stdlib/bcs.md#0x1_bcs">bcs</a>), 0, <a href="../../dependencies/move-stdlib/vector.md#0x1_vector">vector</a>[]);
    <b>while</b> (i &lt; len) {
        v::push_back(&<b>mut</b> res, <a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_peel_address">peel_address</a>(<a href="../../dependencies/move-stdlib/bcs.md#0x1_bcs">bcs</a>));
        i = i + 1;
    };
    res
}
</code></pre>



</details>

<a name="0x2_bcs_peel_vec_bool"></a>

## Function `peel_vec_bool`



<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_peel_vec_bool">peel_vec_bool</a>(<a href="../../dependencies/move-stdlib/bcs.md#0x1_bcs">bcs</a>: &<b>mut</b> bcs::BCS): <a href="../../dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;bool&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_peel_vec_bool">peel_vec_bool</a>(<a href="../../dependencies/move-stdlib/bcs.md#0x1_bcs">bcs</a>: &<b>mut</b> <a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_BCS">BCS</a>): <a href="../../dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;bool&gt; {
    <b>let</b> (len, i, res) = (<a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_peel_vec_length">peel_vec_length</a>(<a href="../../dependencies/move-stdlib/bcs.md#0x1_bcs">bcs</a>), 0, <a href="../../dependencies/move-stdlib/vector.md#0x1_vector">vector</a>[]);
    <b>while</b> (i &lt; len) {
        v::push_back(&<b>mut</b> res, <a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_peel_bool">peel_bool</a>(<a href="../../dependencies/move-stdlib/bcs.md#0x1_bcs">bcs</a>));
        i = i + 1;
    };
    res
}
</code></pre>



</details>

<a name="0x2_bcs_peel_vec_u8"></a>

## Function `peel_vec_u8`



<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_peel_vec_u8">peel_vec_u8</a>(<a href="../../dependencies/move-stdlib/bcs.md#0x1_bcs">bcs</a>: &<b>mut</b> bcs::BCS): <a href="../../dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_peel_vec_u8">peel_vec_u8</a>(<a href="../../dependencies/move-stdlib/bcs.md#0x1_bcs">bcs</a>: &<b>mut</b> <a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_BCS">BCS</a>): <a href="../../dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt; {
    <b>let</b> (len, i, res) = (<a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_peel_vec_length">peel_vec_length</a>(<a href="../../dependencies/move-stdlib/bcs.md#0x1_bcs">bcs</a>), 0, <a href="../../dependencies/move-stdlib/vector.md#0x1_vector">vector</a>[]);
    <b>while</b> (i &lt; len) {
        v::push_back(&<b>mut</b> res, <a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_peel_u8">peel_u8</a>(<a href="../../dependencies/move-stdlib/bcs.md#0x1_bcs">bcs</a>));
        i = i + 1;
    };
    res
}
</code></pre>



</details>

<a name="0x2_bcs_peel_vec_vec_u8"></a>

## Function `peel_vec_vec_u8`



<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_peel_vec_vec_u8">peel_vec_vec_u8</a>(<a href="../../dependencies/move-stdlib/bcs.md#0x1_bcs">bcs</a>: &<b>mut</b> bcs::BCS): <a href="../../dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../../dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_peel_vec_vec_u8">peel_vec_vec_u8</a>(<a href="../../dependencies/move-stdlib/bcs.md#0x1_bcs">bcs</a>: &<b>mut</b> <a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_BCS">BCS</a>): <a href="../../dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../../dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt; {
    <b>let</b> (len, i, res) = (<a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_peel_vec_length">peel_vec_length</a>(<a href="../../dependencies/move-stdlib/bcs.md#0x1_bcs">bcs</a>), 0, <a href="../../dependencies/move-stdlib/vector.md#0x1_vector">vector</a>[]);
    <b>while</b> (i &lt; len) {
        v::push_back(&<b>mut</b> res, <a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_peel_vec_u8">peel_vec_u8</a>(<a href="../../dependencies/move-stdlib/bcs.md#0x1_bcs">bcs</a>));
        i = i + 1;
    };
    res
}
</code></pre>



</details>

<a name="0x2_bcs_peel_vec_u64"></a>

## Function `peel_vec_u64`



<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_peel_vec_u64">peel_vec_u64</a>(<a href="../../dependencies/move-stdlib/bcs.md#0x1_bcs">bcs</a>: &<b>mut</b> bcs::BCS): <a href="../../dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u64&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_peel_vec_u64">peel_vec_u64</a>(<a href="../../dependencies/move-stdlib/bcs.md#0x1_bcs">bcs</a>: &<b>mut</b> <a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_BCS">BCS</a>): <a href="../../dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u64&gt; {
    <b>let</b> (len, i, res) = (<a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_peel_vec_length">peel_vec_length</a>(<a href="../../dependencies/move-stdlib/bcs.md#0x1_bcs">bcs</a>), 0, <a href="../../dependencies/move-stdlib/vector.md#0x1_vector">vector</a>[]);
    <b>while</b> (i &lt; len) {
        v::push_back(&<b>mut</b> res, <a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_peel_u64">peel_u64</a>(<a href="../../dependencies/move-stdlib/bcs.md#0x1_bcs">bcs</a>));
        i = i + 1;
    };
    res
}
</code></pre>



</details>

<a name="0x2_bcs_peel_vec_u128"></a>

## Function `peel_vec_u128`



<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_peel_vec_u128">peel_vec_u128</a>(<a href="../../dependencies/move-stdlib/bcs.md#0x1_bcs">bcs</a>: &<b>mut</b> bcs::BCS): <a href="../../dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u128&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_peel_vec_u128">peel_vec_u128</a>(<a href="../../dependencies/move-stdlib/bcs.md#0x1_bcs">bcs</a>: &<b>mut</b> <a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_BCS">BCS</a>): <a href="../../dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u128&gt; {
    <b>let</b> (len, i, res) = (<a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_peel_vec_length">peel_vec_length</a>(<a href="../../dependencies/move-stdlib/bcs.md#0x1_bcs">bcs</a>), 0, <a href="../../dependencies/move-stdlib/vector.md#0x1_vector">vector</a>[]);
    <b>while</b> (i &lt; len) {
        v::push_back(&<b>mut</b> res, <a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_peel_u128">peel_u128</a>(<a href="../../dependencies/move-stdlib/bcs.md#0x1_bcs">bcs</a>));
        i = i + 1;
    };
    res
}
</code></pre>



</details>

<a name="0x2_bcs_peel_option_address"></a>

## Function `peel_option_address`



<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_peel_option_address">peel_option_address</a>(<a href="../../dependencies/move-stdlib/bcs.md#0x1_bcs">bcs</a>: &<b>mut</b> bcs::BCS): <a href="../../dependencies/move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;<b>address</b>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_peel_option_address">peel_option_address</a>(<a href="../../dependencies/move-stdlib/bcs.md#0x1_bcs">bcs</a>: &<b>mut</b> <a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_BCS">BCS</a>): Option&lt;<b>address</b>&gt; {
    <b>if</b> (<a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_peel_bool">peel_bool</a>(<a href="../../dependencies/move-stdlib/bcs.md#0x1_bcs">bcs</a>)) {
        <a href="../../dependencies/move-stdlib/option.md#0x1_option_some">option::some</a>(<a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_peel_address">peel_address</a>(<a href="../../dependencies/move-stdlib/bcs.md#0x1_bcs">bcs</a>))
    } <b>else</b> {
        <a href="../../dependencies/move-stdlib/option.md#0x1_option_none">option::none</a>()
    }
}
</code></pre>



</details>

<a name="0x2_bcs_peel_option_bool"></a>

## Function `peel_option_bool`



<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_peel_option_bool">peel_option_bool</a>(<a href="../../dependencies/move-stdlib/bcs.md#0x1_bcs">bcs</a>: &<b>mut</b> bcs::BCS): <a href="../../dependencies/move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;bool&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_peel_option_bool">peel_option_bool</a>(<a href="../../dependencies/move-stdlib/bcs.md#0x1_bcs">bcs</a>: &<b>mut</b> <a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_BCS">BCS</a>): Option&lt;bool&gt; {
    <b>if</b> (<a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_peel_bool">peel_bool</a>(<a href="../../dependencies/move-stdlib/bcs.md#0x1_bcs">bcs</a>)) {
        <a href="../../dependencies/move-stdlib/option.md#0x1_option_some">option::some</a>(<a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_peel_bool">peel_bool</a>(<a href="../../dependencies/move-stdlib/bcs.md#0x1_bcs">bcs</a>))
    } <b>else</b> {
        <a href="../../dependencies/move-stdlib/option.md#0x1_option_none">option::none</a>()
    }
}
</code></pre>



</details>

<a name="0x2_bcs_peel_option_u8"></a>

## Function `peel_option_u8`



<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_peel_option_u8">peel_option_u8</a>(<a href="../../dependencies/move-stdlib/bcs.md#0x1_bcs">bcs</a>: &<b>mut</b> bcs::BCS): <a href="../../dependencies/move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_peel_option_u8">peel_option_u8</a>(<a href="../../dependencies/move-stdlib/bcs.md#0x1_bcs">bcs</a>: &<b>mut</b> <a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_BCS">BCS</a>): Option&lt;u8&gt; {
    <b>if</b> (<a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_peel_bool">peel_bool</a>(<a href="../../dependencies/move-stdlib/bcs.md#0x1_bcs">bcs</a>)) {
        <a href="../../dependencies/move-stdlib/option.md#0x1_option_some">option::some</a>(<a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_peel_u8">peel_u8</a>(<a href="../../dependencies/move-stdlib/bcs.md#0x1_bcs">bcs</a>))
    } <b>else</b> {
        <a href="../../dependencies/move-stdlib/option.md#0x1_option_none">option::none</a>()
    }
}
</code></pre>



</details>

<a name="0x2_bcs_peel_option_u64"></a>

## Function `peel_option_u64`



<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_peel_option_u64">peel_option_u64</a>(<a href="../../dependencies/move-stdlib/bcs.md#0x1_bcs">bcs</a>: &<b>mut</b> bcs::BCS): <a href="../../dependencies/move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;u64&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_peel_option_u64">peel_option_u64</a>(<a href="../../dependencies/move-stdlib/bcs.md#0x1_bcs">bcs</a>: &<b>mut</b> <a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_BCS">BCS</a>): Option&lt;u64&gt; {
    <b>if</b> (<a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_peel_bool">peel_bool</a>(<a href="../../dependencies/move-stdlib/bcs.md#0x1_bcs">bcs</a>)) {
        <a href="../../dependencies/move-stdlib/option.md#0x1_option_some">option::some</a>(<a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_peel_u64">peel_u64</a>(<a href="../../dependencies/move-stdlib/bcs.md#0x1_bcs">bcs</a>))
    } <b>else</b> {
        <a href="../../dependencies/move-stdlib/option.md#0x1_option_none">option::none</a>()
    }
}
</code></pre>



</details>

<a name="0x2_bcs_peel_option_u128"></a>

## Function `peel_option_u128`



<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_peel_option_u128">peel_option_u128</a>(<a href="../../dependencies/move-stdlib/bcs.md#0x1_bcs">bcs</a>: &<b>mut</b> bcs::BCS): <a href="../../dependencies/move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;u128&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_peel_option_u128">peel_option_u128</a>(<a href="../../dependencies/move-stdlib/bcs.md#0x1_bcs">bcs</a>: &<b>mut</b> <a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_BCS">BCS</a>): Option&lt;u128&gt; {
    <b>if</b> (<a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_peel_bool">peel_bool</a>(<a href="../../dependencies/move-stdlib/bcs.md#0x1_bcs">bcs</a>)) {
        <a href="../../dependencies/move-stdlib/option.md#0x1_option_some">option::some</a>(<a href="../../dependencies/sui-framework/bcs.md#0x2_bcs_peel_u128">peel_u128</a>(<a href="../../dependencies/move-stdlib/bcs.md#0x1_bcs">bcs</a>))
    } <b>else</b> {
        <a href="../../dependencies/move-stdlib/option.md#0x1_option_none">option::none</a>()
    }
}
</code></pre>



</details>
