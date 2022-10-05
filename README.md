<p align="center">
  <a href="https://gitpod.io/#https://github.com/btwiuse/gm" target="_blank">
    <img src="https://gitpod.io/button/open-in-gitpod.svg" width="240" alt="Gitpod">
  </a>
</p>

# ⚙️ GM

[![Build][build_badge]][build_href] [![License][lic_badge]][lic_href]

[build_badge]: https://github.com/btwiuse/gm/workflows/Build/badge.svg
[build_href]: https://github.com/btwiuse/gm/actions/workflows/build.yml
[lic_badge]: https://img.shields.io/badge/License-MIT-success
[lic_href]: https://github.com/btwiuse/gm/blob/master/LICENSE

<!-- Description starts here -->

GM stands for: Good Morning / Great Move / Gear Multitoken / General Mint / ...

A loose implementation of the ERC1155 multitoken standard for Gear

⚠️ Unaudited and not production ready

Directory layout:

```
src/
├── build.rs                     // cargo build script
├── codec.rs                     // Encoder and Decoder types for contract IO: Init, InitOk, Action, Event, Query, State, TokenMetadata
├── config.rs                    // Provides implementations to IConfig for standard and testing environment: GearConfig, MockConfig
├── contract                     
│   ├── contract_panic_test.rs   // contract core logic positive test cases, using MockConfig, without explicit dependency on gstd, gtest
│   ├── contract_test.rs         // contract core logic positive test cases, using MockConfig, without explicit dependency on gstd, gtest
│   └── mod.rs                   // Contract<T: IConfig> implements IERC1155, IERC1155GearExt, ITokenMetadataRegistry, ...
├── handle
│   ├── handle_panic_test.rs     // fn handle() negative test cases
│   ├── handle_test.rs           // fn handle() positive test cases
│   └── mod.rs                   // fn handle()
├── init
│   ├── init_test.rs             // fn init() test cases
│   └── mod.rs                   // fn init()
├── lib.rs                       // High level abstractions and trait definitions: IERC1155, IERC1155GearExt, ITokenMetadataRegistry, ...
├── metadata.rs                  // gstd::metadata!
├── query
│   ├── mod.rs                   // fn meta_state()
│   └── query_test.rs            // fn meta_state() test cases
└── state.rs                     // pub static mut STATE: Option<Contract<GearConfig>>

4 directories, 16 files
```

The main contract implementation is in [contract.rs](./contract.rs). It applies
the generics pattern mentioned in

https://github.com/shawntabrizi/substrate-trait-tutorial/blob/master/src/step5.rs

making the core contract logic testable without relying on gear specific crates.

features:

- support any combination of fungible and non-fungible tokens;
  - see [IERC1155](./lib.rs) trait and [implementation](./contract.rs)
  - this interface is kept as small as possible to make the code moduler.
  - mint, burn and other safety check methods are specified in IERC1155Ext,
    IERC1155Check, ...
- able to transfer, mint or burn several tokens at once;
  - see [IERC1155Ext](./lib.rs) trait and [implementation](./contract.rs)
  - in this implementation, minting the same token id twice is forbidden, which
    means the total supply of any minted token will not increase after they are
    minted
- emit events when transactions succeed
  - see [IERC1155GearExt](./lib.rs) trait and [implementation](./contract.rs)
  - events are always emitted after the transaction is made
- abort transaction early when the requirement isn't met
  - see [IERC1155Check](./lib.rs) trait and [implementation](./contract.rs)
  - checks are always performed before the transaction is made
- approval management and token metadata.
  - see [ITokenMetadataRegistry](./lib.rs) trait and
    [implementation](./contract.rs)
  - the token metadata manager is a simple KV store that works similar to
    Metaplex Token Metadata program on Solana. It manages token metadata for
    both fungible and non fungible tokens. The token metadata is empty by
    default and is supposed to be set manually after mint using the
    `update_token_metadata` method
  - in this implementation, all token owners are allowed to update token
    metadata.

TODO:

- [x] Decouple contract implementation from Gear specific types
- [x] Add codec types
- [x] Support contract state query
- [x] Emit events using `msg::reply`
- [x] Add basic tests for `contract.rs`, `init.rs`, `query.rs`, `handle.rs`
- [x] implement IERC1155 batch operations
- [x] implement IERC1155Check: perform sanity checks on user input before
      invoking `mint`, `mint_batch`, `safe_transfer_from`,
      `safe_batch_transfer_from`, etc. before any state mutation.
- [x] add methods for obtaining caller info in IConfig
- [x] refactor IERC1155 to make IERC1155Check a requirement:
      `pub trait IERC1155<T: IConfig>: IERC1155Check<T>`
- [x] Comprehensive testing covering all possible bad cases.
- [x] Submit result

Testing:

```
$ cargo test
   Compiling gm v0.1.0 (/home/btwiuse/gm)
    Finished test [unoptimized + debuginfo] target(s) in 4.12s
     Running unittests lib.rs (target/debug/deps/gm-aedc5d0e158877c4)

running 54 tests
test contract_panic_test::balance_of_batch_length_mismatch_panics - should panic ... ok
test contract_panic_test::burn_batch_from_non_owner_panics - should panic ... ok
test contract_panic_test::burn_exceeding_balance_panics - should panic ... ok
test contract_panic_test::burn_batch_exceeding_balance_panics - should panic ... ok
test contract_panic_test::burn_from_non_owner_panics - should panic ... ok
test contract_panic_test::mint_twice_panics - should panic ... ok
test contract_panic_test::transfer_batch_exceeding_balance_panics - should panic ... ok
test contract_panic_test::set_approval_for_all_from_non_owner_panics - should panic ... ok
test contract_panic_test::transfer_batch_length_mismatch_panics - should panic ... ok
test contract_panic_test::mint_batch_twice_panics - should panic ... ok
test contract_panic_test::transfer_batch_from_non_owner_panics - should panic ... ok
test contract_panic_test::burn_batch_length_mismatch_panics - should panic ... ok
test contract_panic_test::transfer_exceeding_balance_panics - should panic ... ok
test contract_panic_test::transfer_from_non_owner_panics - should panic ... ok
test contract_panic_test::update_token_metadata_from_non_owner_panics - should panic ... ok
test contract_test::balance_of_batch_works ... ok
test contract_test::balance_of_works ... ok
test contract_test::burn_batch_from_approved_works ... ok
test contract_test::burn_batch_works ... ok
test contract_test::burn_from_approved_works ... ok
test contract_test::burn_works ... ok
test contract_test::default_approval_for_all_is_false ... ok
test contract_test::default_token_metadata_is_none ... ok
test contract_test::is_approved_for_all_works ... ok
test contract_test::mint_batch_works ... ok
test contract_test::mint_works ... ok
test contract_test::remove_update_token_metadata_works ... ok
test contract_test::set_approval_for_all_from_sender_works ... ok
test contract_test::set_approval_for_all_works ... ok
test contract_test::transfer_batch_from_approved_works ... ok
test contract_test::transfer_batch_works ... ok
test contract_test::transfer_from_approved_works ... ok
test contract_test::transfer_works ... ok
test contract_test::update_token_metadata_works ... ok
[DEBUG handle_panic_test::mint_zero_panics] panic occurred: 'check failed: cannot mint 0 amount', /home/btwiuse/gm/contract.rs:92:13
test init_test::init_works ... ok
test handle_panic_test::mint_zero_panics ... ok
[DEBUG handle_panic_test::transfer_exceeding_balance_panics] panic occurred: 'check failed: insufficient balance', /home/btwiuse/gm/contract.rs:64:13
test handle_panic_test::transfer_exceeding_balance_panics ... ok
[DEBUG handle_panic_test::transfer_zero_panics] panic occurred: 'check failed: cannot transfer 0 amount', /home/btwiuse/gm/contract.rs:58:13
test handle_test::mint_works ... ok
test handle_panic_test::transfer_zero_panics ... ok
[DEBUG handle_panic_test::burn_zero_panics] panic occurred: 'check failed: cannot burn 0 amount', /home/btwiuse/gm/contract.rs:128:13
test handle_panic_test::burn_zero_panics ... ok
[DEBUG handle_panic_test::burn_exceeding_balance_panics] panic occurred: 'check failed: insufficient balance', /home/btwiuse/gm/contract.rs:134:13
test handle_panic_test::burn_exceeding_balance_panics ... ok
test handle_test::set_approval_for_all_works ... ok
test handle_test::burn_works ... ok
[DEBUG handle_panic_test::mint_twice_panics] panic occurred: 'check failed: cannot mint twice', /home/btwiuse/gm/contract.rs:95:13
test handle_test::mint_batch_works ... ok
test handle_panic_test::mint_twice_panics ... ok
test query_test::basic_query_works ... ok
test handle_test::burn_batch_works ... ok
[DEBUG handle_panic_test::transfer_batch_length_mismatch_panics] panic occurred: 'check failed: token and amount length mismatch', /home/btwiuse/gm/contract.rs:81:13
test handle_panic_test::transfer_batch_length_mismatch_panics ... ok
[DEBUG handle_panic_test::transfer_batch_length_mismatch_works] panic occurred: 'check failed: token and amount length mismatch', /home/btwiuse/gm/contract.rs:81:13
test handle_panic_test::transfer_batch_length_mismatch_works ... ok
[DEBUG handle_panic_test::transfer_from_non_owner_panics] panic occurred: 'check failed: needs approval', /home/btwiuse/gm/contract.rs:61:13
test handle_panic_test::transfer_from_non_owner_panics ... ok
test handle_test::transfer_works ... ok
[DEBUG handle_panic_test::burn_batch_length_mismatch_panics] panic occurred: 'check failed: token and amount length mismatch', /home/btwiuse/gm/contract.rs:144:13
test handle_panic_test::burn_batch_length_mismatch_panics ... ok
test handle_test::transfer_batch_works ... ok
test handle_test::update_token_metadata_works ... ok

test result: ok. 54 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.33s

   Doc-tests gm

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

<!-- End of description -->

## Prebuilt Binaries

Raw, optimized, and meta WASM binaries can be found in the
[Releases section](https://github.com/btwiuse/gm/releases).

## Building Locally

### ⚙️ Install Rust

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### ⚒️ Add specific toolchains

```shell
rustup toolchain add nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
```

... or ...

```shell
make init
```

### 🏗️ Build

```shell
cargo build --release
```

... or ...

```shell
make build
```

### ✅ Run tests

```shell
cargo test --release
```

... or ...

```shell
make test
```

### 🚀 Run everything with one command

```shell
make all
```

... or just ...

```shell
make
```

## License

The source code is licensed under [MIT license](LICENSE).
