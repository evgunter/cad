---
id: wasm-row-warning-debt-comment-names-a-closed-item-and-a-deleted-symbol
kind: issue
title: ci.yml's wasm-row warning-debt comment describes a state that has been resolved, and names a symbol that no longer exists
status: review
opened: 2026-09-10
pr: 2326
branch: ciw/wasm-warning-debt
---


Reported by VIEW's `view/chooser-arm` lane (PR 2278) as an
implementer-discipline §6 finding: it is on CIW's territory, so it is
routed here and **not fixed across the fence**. The finding is not
about that lane's diff — the first half of it is true on `main`
already, at `be0d8b3af`.

## What the comment says

`.github/workflows/ci.yml:2165-2174`, above the
`wasm32 check (viewer app feature - the browser entry point)` step at
`:2177-2180`:

> `check`, NOT `-D warnings`, AND THAT IS A DEBT WITH A FILE. This
> crate's wasm build carries two dead-code warnings today —
> `WINDOW_TITLE` and `ViewerApp::deliver_status`, whose users are
> all `cfg(not(target_family = "wasm"))` — so denying warnings here
> would red on those before it ever reached a real break, and
> fixing them is the crate owners' call, not a guard's. It leaves
> this as the ONE viewer row in this workflow that cannot fail on a
> warning, which is not a resting state:
> `work/view/viewer-items-unreferenced-at-wasm32.md` holds the flip,
> and this row becomes `-D warnings` when that item closes.

Every sentence of it is present tense, and it names its own trigger.

## Both halves of it have fired

**The item closed.** `work/view/viewer-items-unreferenced-at-wasm32.md`
is `status: closed`, `closed: 2026-09-09`, `pr: 2272`. That PR put
`#[cfg(not(target_family = "wasm"))]` on both named items, so the wasm
build carries neither warning. Its `## Closed (2026-09-09)` section
records the reproduction — `cargo check -p viewer --features app
--target wasm32-unknown-unknown` at `1d29a8eeb` gave exactly the two,
at `app.rs:114:7` and `app.rs:1066:8` — which is the evidence the flip
is clean. So *"this row becomes `-D warnings` when that item closes"*
is a condition that has been met, and the paragraph above it describes
a resolved state in the present tense.

**One of the two named symbols is being deleted.** PR 2278 (VIEW,
building Ev's ruling that the absent-chooser status arm is
misclassified) deletes `ViewerApp::deliver_status` outright, along with
`frame::dialog_status` and both call sites. Once it lands, the
comment's *"two dead-code warnings today"* names one item that is
`cfg`-ed and one that does not exist.

## Why it is worth a row rather than a one-line edit

The comment is not decoration: it is the written reason this is the ONE
viewer row in the workflow that cannot fail on a warning, and it names
the condition under which that stops. Deleting the stale sentences
without taking the flip would leave the row unguarded with nothing left
saying why. So the shapes are:

1. **Take the flip** — the condition the comment set has been met —
   and rewrite `:2165-2174` as the reason the row now denies warnings.
2. **Keep `check`** for some reason the comment does not record, and
   write *that* reason, retiring the
   `viewer-items-unreferenced-at-wasm32` trigger since it can never
   fire again.

## Three things to settle before spelling the flip, if shape 1

None of these is a blocker; all three are places where the obvious edit
and the intended guard differ.

- **The two candidate spellings differ on one axis, and it is not
  scope.** ~~`cargo clippy -p viewer … -- -D warnings` is scoped to
  `viewer`~~ — CORRECTED at the flip (`## Taken`), by running both.
  BOTH spellings compile every crate in the graph under the deny,
  because workspace path dependencies (`kernel`, `editor-core`,
  `bvh`, …) are not `--cap-lints allow`-ed the way registry
  dependencies are, and `cargo clippy`'s post-`--` arguments reach
  every such unit rather than the `-p` package alone. So both red on a
  warning anywhere in the workspace at that target. What clippy adds is
  clippy's own lints, which `cargo check` never runs and which fire on
  path dependencies too. Run both once and choose deliberately rather
  than picking the shorter diff — this bullet originally asserted the
  scope difference from how cargo caps lints, not from having run it,
  and that is exactly how it came to be false.
- **The row already carries a `RUSTFLAGS`, and `:2160-2163` explains
  why it is scoped to the command** rather than exported or set as an
  `env:` key: `RUSTFLAGS` silently REPLACES any `.cargo/config.toml`
  rustflags. A flip spelled through `RUSTFLAGS` appends to
  `--cfg getrandom_backend="wasm_js"` in that same string and inherits
  that reasoning; one spelled through clippy carries the `RUSTFLAGS`
  for the `getrandom` cfg regardless, so it keeps that reasoning too
  and only the DENY moves out of the string (corrected at the flip).
- **`--all-targets` must not be added.** `crates/viewer/tests/` reaches
  `ThreadEvaluator`, which `crates/viewer/src/lib.rs:139` re-exports
  under `#[cfg(not(target_family = "wasm"))]`, so a wasm row that
  builds the test targets fails to compile for a reason that has
  nothing to do with warnings. (Understated, corrected at the flip:
  `crates/viewer/examples/` is in the population too and fails first,
  and `prefs::file` is a second, independent cause.)

## The mirror

`local-scripts/ci-local.sh:1151-1153` (`wasm_check_viewer`, run as
`wasm32 check (viewer app)` at `:1311`) carries the same command, and
`ci.yml:2176` names it as the mirror. The CI-half parity gate reads
both, so whatever this row becomes, that one becomes with it.

## Taken: shape 1, spelled as clippy (2026-09-11)

Line numbers above are the item's, taken at `be0d8b3af`; they do not
resolve on later trees. The block is `ci.yml`'s
`THE VIEWER'S OWN WASM ROW` comment through the step it sits above —
grep for that heading rather than a line number.

**Shape 1, and the flip is spelled `cargo clippy -p viewer --features
app --target wasm32-unknown-unknown -- -D warnings` with the existing
`RUSTFLAGS` prefix kept.** The row is renamed `wasm32 clippy (viewer app
feature - the browser entry point)`; the mirror becomes
`wasm32 clippy (viewer app)` / `wasm_clippy_viewer`.

**Both candidate spellings were run, and the item's reason for
preferring one over the other is wrong.** At the pinned toolchain
(1.97.0), a `dead_code` item planted in `crates/editor-core/src/lib.rs`
reds BOTH: `RUSTFLAGS='-Dwarnings …' cargo check -p viewer` exits 101,
and so does `cargo clippy -p viewer … -- -D warnings`. Clippy's own
lints fire on path dependencies too — a planted `clippy::ptr_arg` in the
same file reds the clippy spelling. `cargo clippy -p <pkg> -- -D
warnings` is therefore **not** scoped to `<pkg>`: the `--` args reach
every non-`--cap-lints` unit in the graph. The two spellings differ on
exactly one axis, clippy's own lints, and cost the same — 66 s (check)
against 65 s (clippy) from an empty `CARGO_TARGET_DIR` on 4 vCPU. So
the choice was made on coverage: clippy is strictly wider for nothing,
and it makes this row the wasm sibling of
`clippy (viewer app feature - eframe + wgpu)`, which sits in the same
job and is what "the ONE viewer row that cannot fail on a warning" was
measured against.

**The `--all-targets` warning is right and understated**: the failure is
E0432, and the FIRST unit to fail is an example (`r1_e2e`), not a test.
`crates/viewer/examples/` is in the population too.

**Neither named warning exists.** `cargo check -p viewer --features app
--target wasm32-unknown-unknown` at `aa628e5` is clean; `WINDOW_TITLE`
carries `#[cfg(not(target_family = "wasm"))]` at `app.rs:128`, and
`deliver_status` has no occurrence anywhere under `crates/viewer/`.

The three bullets under *"Three things to settle"* are corrected where
they stand rather than only here: two of them were false and a third
understated, and a reader quoting a bullet alone must not quote a
falsehood.

Residue, each its own file:
`work/ciw/kernel-wasm-row-denies-no-warnings.md`,
`work/ciw/a-source-attribute-can-silence-a-ci-deny-unread.md`.
