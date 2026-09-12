# Cargo workspace mapping for hive cells

As of 2026-09-11.

## Verdict

Use a virtual Cargo workspace, edition 2024, resolver 3, rustc 1.98.1.

Give each cell one `rlib` crate. Keep the kernel one framework-free `rlib`. Put composition in one `bin` crate that is the only crate allowed to depend on cells.

Do not split a cell into domain/application/infra crates. Intra-cell layers are modules. `pub` is Open Host plus leaving SPI traits. Everything else is `pub(crate)` or private.

Hand-roll DI in the binary. Do not take nject, shaku, or inventory.

The compile-time wall is rustc crate privacy plus cargo-deny 0.20.2 `bans.deny` wrappers. Feature flags are not a wall.

Pinned:

| Pin                     | Version |
| ----------------------- | ------- |
| rustc (stable)          | 1.98.1  |
| edition                 | 2024    |
| workspace resolver      | 3       |
| cargo-deny              | 0.20.2  |
| nject (do not take)     | 0.5.1   |
| shaku (do not take)     | 0.6.3   |
| inventory (do not take) | 0.3.24  |

Recommended layout:

```
Cargo.toml                 # virtual workspace, no [package]
deny.toml                  # cargo-deny
crates/
  kernel/                  # rlib, framework-free
  cells/
    <domain>/              # path segment, not a crate
      <cell>/              # one rlib per cell
  app/                     # bin composition root
```

Root manifest:

```toml
[workspace]
resolver = "3"
members = ["crates/kernel", "crates/cells/*/*", "crates/app"]
default-members = ["crates/app"]

[workspace.package]
edition = "2024"
rust-version = "1.98"
publish = false

[workspace.dependencies]
kernel = { path = "crates/kernel" }

[workspace.lints.rust]
unsafe_code = "forbid"
unreachable_pub = "warn"
```

Cell crates set `crate-type` to the library default (`"lib"` / rlib). The app crate is a binary (`src/main.rs`). The kernel crate is a library. Domain folders under `crates/cells/` have no `Cargo.toml`.

## Compared

| Option                                               | Role            | Why it lost or won                                                                                                                                                                                                                                         |
| ---------------------------------------------------- | --------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| One rlib per cell + kernel rlib + app bin            | Winner          | Cargo forbids a crate from using another crate it does not depend on. That is the hive import wall. A package may have only one library. Virtual workspaces keep the binary from becoming a root package that other crates inherit by accident.            |
| One crate for the whole hive (modules only)          | Loser           | `pub(crate)` would leak every cell to every other cell. Nest `exports` has no analogue.                                                                                                                                                                    |
| Four crates per cell (domain/app/infra/presentation) | Loser           | Hive identity is the cell, not a layer. Extra crates duplicate `workspace.dependencies` and slow `cargo check`. Intra-cell rustc cannot see sqlx if the hex crate does not list it. That split is a later option if domain code starts importing adapters. |
| Cell-api crate + cell-hex crate                      | Loser           | Open Host is a `pub` re-export set, not a second package. Splitting now is speculative.                                                                                                                                                                    |
| Feature flags as the wall (`inproc-wire`)            | Loser           | Cargo unifies features across the graph. Enabling a feature in one crate enables it for every user of that crate. Features are additive. They cannot express "cell A must not see cell B".                                                                 |
| nject 0.5.1                                          | Loser for cells | Zero-cost and compile-time, but `#[module]` / `#[import]` makes a provider name the imported module. Hive law: the static cell never names a provider. A `#[provider]` struct belongs only in `app` if anyone wants the sugar later.                       |
| shaku 0.6.3                                          | Loser           | `module! { components = [...] }` is Nest-shaped. Submodules make cell modules import provider modules. Traits must extend `Interface` (`'static`, optionally `Send + Sync`). That bound does not belong on domain ports.                                   |
| inventory 0.3.24                                     | Loser           | `submit!` registers plugins from any linked crate with no central list. That is `@Global()`. Hive composition is explicit imports only.                                                                                                                    |
| teloc 0.2.0 / waiter_di 1.6.6                        | Loser           | Stale relative to nject/shaku. Same Nest-shaped module graph.                                                                                                                                                                                              |
| cargo-hakari 0.9.38                                  | Not a wall      | Workspace-hack unifies third-party versions. It does not forbid cell-to-cell edges.                                                                                                                                                                        |
| Hand-rolled `app` constructors                       | Winner for DI   | The binary names every cell and every leaving SPI. Cell `::new` takes those SPIs. AFIT (stable since rustc 1.75) lets API-port and SPI traits use `async fn` without `async-trait`.                                                                        |

cargo-deny versus a one-off `cargo metadata` script: cargo-deny already owns bans, licenses, advisories, and sources. Use it. A metadata script that asserts `cell_deps ∩ cells == ∅` is optional duplication.

## Fit to hive

chapter 2 names three kinds besides legacy: cell, composition root, kernel. Map them 1:1 onto crates. A domain folder stays a path segment. It is not a workspace member.

chapter 3: the composition root is the only place that imports every cell and binds leaving SPIs. The static cell module never names a provider. In Cargo, `crates/app/Cargo.toml` is that place. A cell `Cargo.toml` lists `kernel` and in-cell tech only. It does not list another cell.

In Rust, `crates/cells/<domain>/<cell>/src/lib.rs` re-exports only:

- Open Host traits (API ports) and their Published Language types
- leaving SPI traits so `app` can implement InProc
- a constructor `fn new(...) -> Self` that takes those SPIs

Unpublished ports (GraphQL-only, MCP-only, ticks) stay `pub(crate)`. Driving adapters in the same crate call them. Other crates cannot.

SPI, entities, handlers, and persistence stay `pub(crate)` or private. The app crate cannot name them. That is stronger than Nest: `AppModule` could import internals. A binary crate cannot see `pub(crate)`.

Hive allow/reject mapped onto Cargo:

| Hive edge                                       | Rust                                                                                                                                                         |
| ----------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| composition root → every cell                   | `app` path-depends on every cell crate                                                                                                                       |
| cell → kernel                                   | cell `Cargo.toml` depends on `kernel`                                                                                                                        |
| consumer inproc → provider `domain/api`         | InProc adapters live in `app`, not in the cell crate. The cell therefore never path-depends on a provider. `app` calls the provider's `pub` Open Host trait. |
| cell A application/domain/presentation → cell B | Impossible: no path dep, and internals are not `pub`.                                                                                                        |
| cell module → other cell module                 | Impossible: no path dep.                                                                                                                                     |
| neighbour → cell internals                      | No neighbour crates in a greenfield hive.                                                                                                                    |

Leaving SPI binding is `app` constructing the InProc adapter and passing it into `payouts::new(wire_spi, ...)`. Persistence adapters stay inside the cell crate and are not leaving SPIs.

Kernel (chapter 18): framework-free, no Open Host, no datastore. The kernel `Cargo.toml` lists no tokio, no HTTP crate, no SQL crate, no serde unless a later grill proves PL codecs belong there. Hive forbids kernel → cell. Cargo enforces that if kernel does not depend on cells.

Tests: unit tests inside the cell crate see `pub(crate)`. Integration tests under `tests/` see only the public API. That matches "harness does not boot AppModule". `app` tests boot the binary.

Wall tool:

1. rustc. Missing path dependency is a compile error. `pub(crate)` is invisible outside the cell.
2. `#![warn(unreachable_pub)]` via `workspace.lints`. A `pub` item that `lib.rs` does not re-export is a leak of Open Host.
3. cargo-deny 0.20.2. In `deny.toml`:

```toml
[bans]
multiple-versions = "warn"
wildcards = "deny"
allow-wildcard-paths = true

[bans.workspace-dependencies]
duplicates = "deny"
include-path-dependencies = false
unused = "deny"

# One entry per cell crate. Only app may depend on a cell.
deny = [
  { crate = "payouts", wrappers = ["app"], reason = "hive: only composition root depends on cells" },
]
```

`wrappers` lets the named crate take a direct dependency on the banned crate and denies every other direct edge. That is the cruise allow-list: composition root → cells, never cell → cell.

Also run `cargo deny check` for licenses, advisories, and sources. That is supply-chain, not hive law, and it is the same tool.

`cargo tree -p payouts --depth workspace` is the human view of the same graph. It is not the gate.

Do not use cargo features to hide InProc. Feature unification would turn on the provider edge for every consumer of that cell crate.

## Sources

- Rust 1.98.1 stable: https://blog.rust-lang.org/2026/09/03/Rust-1.98.1/ and https://static.rust-lang.org/dist/channel-rust-stable.toml (date 2026-09-03, version 1.98.1)
- Edition 2024 stable in rustc 1.85.0: https://blog.rust-lang.org/2025/02/20/Rust-1.85.0/ and https://doc.rust-lang.org/edition-guide/rust-2024/index.html
- Editions overview: https://doc.rust-lang.org/edition-guide/editions/index.html
- `edition = "2024"` implies resolver 3; virtual workspaces must set `resolver` explicitly: https://doc.rust-lang.org/edition-guide/rust-2024/cargo-resolver.html
- Cargo workspaces (virtual manifest, `members` globs, `workspace.package`, `workspace.dependencies`, `workspace.lints`, `default-members`): https://doc.rust-lang.org/cargo/reference/workspaces.html
- Manifest `edition` (cargo new currently writes 2024): https://doc.rust-lang.org/cargo/reference/manifest.html
- Cargo targets (one library per package, default crate-type `"lib"`, binaries, unit vs integration tests): https://doc.rust-lang.org/cargo/reference/cargo-targets.html
- Visibility (`pub`, `pub(crate)`, `pub(in path)`, re-exports): https://doc.rust-lang.org/reference/visibility-and-privacy.html
- `unreachable_pub` lint: https://doc.rust-lang.org/rustc/lints/listing/allowed-by-default.html
- Feature unification (features are additive; union across the graph): https://doc.rust-lang.org/cargo/reference/features.html
- `cargo tree`: https://doc.rust-lang.org/cargo/commands/cargo-tree.html
- AFIT / async fn in traits, rustc 1.75: https://blog.rust-lang.org/2023/12/28/Rust-1.75.0/
- cargo-deny 0.20.2: https://crates.io/crates/cargo-deny and https://github.com/EmbarkStudios/cargo-deny/releases/tag/0.20.2
- cargo-deny bans (`deny`, `wrappers`, `workspace-dependencies`): https://embarkstudios.github.io/cargo-deny/checks/bans/index.html and https://embarkstudios.github.io/cargo-deny/checks/bans/cfg.html
- nject 0.5.1: https://crates.io/crates/nject and https://docs.rs/nject/0.5.1/nject/
- shaku 0.6.3: https://crates.io/crates/shaku and https://docs.rs/shaku/0.6.3/shaku/ and https://docs.rs/shaku/0.6.3/shaku/guide/index.html
- inventory 0.3.24: https://crates.io/crates/inventory and https://docs.rs/inventory/0.3.24/inventory/
- RFC 3501 (edition 2024): https://rust-lang.github.io/rfcs/3501-edition-2024.html
