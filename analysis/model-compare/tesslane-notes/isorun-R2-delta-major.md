MAJOR, PR 7526 delta (a79bf304d5), DEMONSTRATED: the unit's own new file fails clippy, so the head cannot go green.

- Where: `crates/mesh/tests/common/rowless.rs:42` (`less`): `try_less(a, b).unwrap_or_else(|e| panic!("a ∖ b builds: {e}"))`.
- The error: "`panic` should not be present in production code" (`-D clippy::panic`). The file has no `#![allow(clippy::panic)]`; `rowless_cylinders.rs` has one, `common/rowless.rs` does not.
- Hosted: run 45175373840, job "mesh budget meter + certificate falsifier" (137713298684) fails at exactly this line.
- Hosted lint never reached its clippy step: it stopped first at main's `payload-rung-sweep` red. So no hosted clippy has seen this file.
- Reproduced locally on the clean head: `cargo clippy -p mesh --all-targets -- -D warnings` fails, with and without `--features budget`.
- The PR body's "cargo clippy --workspace --all-targets --all-features -D warnings" is therefore not true of this head.
- The fix is one attribute; nothing else in my runs is red. Full report follows.
