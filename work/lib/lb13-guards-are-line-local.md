---
id: lb13-guards-are-line-local
kind: issue
title: Both LB13 guards are line-local, and the façade's dominant idiom is the multi-line pub use list
status: open
opened: 2026-09-04
refs: [696, 1841]
---


## Filed on LIB's slate, by CIW

`crates/pncad/tests/all.rs` is **LIB's** file and this is LIB's fix, so
the item is LIB's from the start. It was first filed into
`work/issues/` for LIB to claim by moving; **Ev ruled on 2026-09-04
that filing straight onto the owning program's slate is correct**, so
it was re-homed by header edit and `git mv`, never by copying.
`work/issues/` is for items no program owns *yet* — not a waiting room
for items whose owner is obvious.

Found by CIW while asking the ruling on
`work/ciw/facade-guards-defer-to-rustdoc-json` (#696, PR 1841). That
PR rewrote the three guards' doc comments — the deferral half — and
left this, the logic half, here.

## The finding

Both LB13 guards scan **line by line** and require the key to appear on
the same line as the `pub use`:

- `no_arena_key_is_nameable_through_the_facade_document_surface` —
  iterates at `crates/pncad/tests/all.rs:826`, and skips any line that
  does not itself contain `pub use` (`:835`) before looking for
  `EntityRef` / `EntityKey` / `Entry` on it.
- `no_raw_loop_minting_door_is_nameable_through_the_facade` — iterates
  at `:917`, and tests `t.contains("pub use") && t.contains("RawLoop")`
  on one line (`:926`).

**The façade's dominant idiom is the multi-line brace list.** Derived
against `7db483d4` by a statement-based scan of the eleven files in
`FACADE_SOURCES` — line comments stripped the way `code_without_comments`
strips them, then every `pub use` accumulated to its terminating `;`:

| | |
|---|---|
| `pub use` statements in the façade's sources | **74** |
| of those, statements whose `;` is on a later line | **33** |
| of those 33, statements naming an `editor_core::` path | **17** |

So a key added *inside an existing brace list* — the natural spelling
of the regression both guards exist to catch — is invisible to them.
The hazard is not hypothetical shape: 33 of 74 statements are already
written that way, and 17 of those already reach into `editor_core`, so
the regression's cheapest spelling is also its invisible one.

**Correction to the numbers in `work/ciw/facade-guards-defer-to-rustdoc-json`,
which stated 33 of **77**, **15** naming `editor_core::`.** Both were
re-derived here and the check wins:

- **77** is `grep -c 'pub use'` over the eleven files, which counts
  three lines that are prose *about* `pub use` inside comments
  (`document.rs:86`, `profile.rs:5`, `profile.rs:102`) and which the
  guards themselves never see, because they strip comments first. The
  statement count in code is **74**. The 33 is unaffected — every
  multi-line statement is a real one.
- **15** counts only the statements spelled `pub use editor_core::{`.
  Two more reach the same crate through a deeper path
  (`analysis.rs`: `editor_core::mc::{`, `editor_core::report::{`), and
  they carry the same exposure, so **17** is the figure for the claim
  being made.

## The composite mitigation, and where it stops

The file already contains a statement-based scanner: `pub_use_names`
(`all.rs:3200`) accumulates a `pub use` to its `;` before splitting the
brace list, so it sees multi-line statements. The two mechanisms
overlap **partially**:

- A multi-line `pub use editor_core::{… EntityRef …}` in the façade is
  invisible to the LB13 guard, but it puts `EntityRef` into the carried
  set, which makes its `NOT_CARRIED` entry stale, which reds
  `every_document_layer_root_export_is_carried_or_listed`. So the
  regression does not reach main silently — but the message a reader
  gets is *"NOT_CARRIED lists 1 name(s) that are no longer uncarried
  root exports — remove them"* (`all.rs`, the staleness assert), which
  reads as a bookkeeping chore and whose remedy, followed literally,
  deletes the seal's own exclusion entry. Nothing in that failure says
  LB13.
- A multi-line `pub use` through a path that is **not** `editor_core::`
  — a façade-internal path, or another crate that re-exports the key —
  names the key on a continuation line and is seen by **neither**
  guard: `pub_use_names` is called with `layer = "editor_core"` and
  skips any statement without that prefix (`all.rs:3211`). No crate
  outside `pncad` re-exports an `editor_core` path today
  (`grep -rn 'pub use editor_core' crates/ | grep -v '^crates/pncad/'`
  is empty), so this route has no live vehicle — it is the shape that
  would open one.

## The fix

Make both LB13 guards statement-based, by the mechanism already in the
file — accumulate from `pub use` to `;` as `pub_use_names` does, then
match the key names in the accumulated statement rather than in a line.
Reporting a line number stays possible (the line the statement opens
on). This is LIB's call on wording and factoring; the scanner exists.

## Second instance of the same class, same file

`root_declared_pub_names`'s doc (`all.rs:3393`) says the document
layer's root declarations are *"the crate's twenty-six interior
modules"*. Re-derived at `7db483d4`: `crates/editor-core/src/lib.rs`
declares **32** `pub mod` at column 0
(`grep -c '^pub mod ' crates/editor-core/src/lib.rs`), **four** of them
behind `#[cfg(feature = "interval")]`, so **28** survive
`code_without_cfg_gated`. Neither 32 nor 28 is twenty-six, and the
count matters at exactly the point the sentence is making: it is the
number of names that would join the export set if that helper were ever
applied to this root, and therefore the size of the carrying-or-listing
bill for closing that guard's second blind spot. (The same root declares
**no** `pub` item of any other kind, which is what the completeness
guard's blind spot 2 is really about; PR 1841 corrected that guard's
wording, which read *"no direct `pub` items"*, and left the count here
for this item.)

## Why this matters more than it looks

Ev ruled on 2026-09-04 that these guards' text scans are the permanent
mechanism and that no rustdoc-JSON pass will be built (#696). The
strongest argument on the other side was precisely this hole: a
structural check reading the compiler's API would never have been
line-local. Fixing it in text closes that hole **and** the class it
belongs to — every future scanner in this file that reads a statement
instead of a line — for the price of an afternoon in one file, with no
second toolchain, no second compiler pin and nothing added to any gate.
It buys more real coverage than the rejected nightly would have.
