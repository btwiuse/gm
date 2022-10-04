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
.
├── lib.rs            // High level abstractions and trait definitions: IERC1155, IERC1155GearExt, ITokenMetadataRegistry, ...
├── codec.rs          // Encoder and Decoder types for contract IO: initialization, transaction input, events, state query, token metadata
├── config.rs         // Provides implementations to IConfig for standard and testing environment: GearConfig, MockConfig
├── contract.rs       // Contract<T: IConfig> implements IERC1155, IERC1155GearExt, ITokenMetadataRegistry, ...
├── contract_test.rs  // contract core logic related tests, using MockConfig, without explicit dependency on gstd, gtest

├── state.rs          // pub static mut STATE: Option<Contract<GearConfig>>
├── metadata.rs       // gstd::metadata!
├── handle.rs         // fn handle()
├── init.rs           // fn init()
├── query.rs          // fn meta_state()

├── handle_test.rs    // fn handle() related tests
├── init_test.rs      // fn init() related tests
├── query_test.rs     // fn meta_state() related tests

└── build.rs          // cargo build script
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
- [ ] Comprehensive testing covering all possible bad cases. Currently there are
      only a few good cases (Help wanted: what is the preferred way to handle
      panic cases in `handle.rs`?)
- [ ] Submit result

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
