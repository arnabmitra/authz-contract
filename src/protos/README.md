# Protos description

For this HackWasm hackathon, I found this repo:
https://github.com/jgarzik/rust-protobuf-example

I cloned the repo, created this new file:

### rust-protobuf-example/src/protos/CosmosDistributionV1beta1MsgWithdrawDelegatorReward.proto

```protobuf
syntax = "proto3";

/* This is taken from:
 * https://github.com/cosmos/cosmos-sdk/blob/5deb137f7a83506381472928d898cd823735aade/proto/cosmos/distribution/v1beta1/tx.proto#L66-L74
 * With the annotations removed */

message MsgWithdrawDelegatorReward {
  string delegator_address = 1;
  string validator_address = 2;
}
```

Then modified `rust-protobuf-example/build.rs` to be:

```diff
fn main() {
    println!("top of main");
    protobuf_codegen::Codegen::new()
        .cargo_out_dir("protos")
        .include("src")
+        .input("src/protos/CosmosDistributionV1beta1MsgWithdrawDelegatorReward.proto")
        .input("src/protos/example.proto")
        .run_from_script();
}
```

Then in that project run:

    cargo build

and you'll see this new file:
`rust-protobuf-example/target/debug/build/rust-protobuf-example-26ee9cb57adda940/out/protos/CosmosDistributionV1beta1MsgWithdrawDelegatorReward.rs`

I copied that file into the directory where this file is, and we include it in our `contract.rs` file.