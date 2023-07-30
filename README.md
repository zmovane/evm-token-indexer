# EVM token indexer

A lightweight way to index all tokens which derived from ERC165 standard, based on the ordered logs of EVM-compatible chain.

### DB migration

```
cargo run --bin prisma -- migrate dev
```

### Running indexer

```
RUST_LOG=info cargo run --bin indexer
```
