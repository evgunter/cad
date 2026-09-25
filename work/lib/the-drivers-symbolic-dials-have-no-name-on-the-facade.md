---
id: the-drivers-symbolic-dials-have-no-name-on-the-facade
kind: issue
title: the driver's symbolic dials are readable off DriveConfig::default() and not nameable: SymbolicDials is not on the pncad facade, so a consumer can read the tier's budget and rules but cannot declare or modify them
status: open
opened: 2026-09-22
priority: P1
cost: E
refs: [SYM-14]
---



## What

**Specifically requested by Ev, 2026-09-22** — found by SYM-14, the
chain demo Ev asked for, while writing its certified cell
(`demos/tour/src/chaintol.rs`).

A cell that runs ONE leaf at `Sym<Interval>` has to hand
`geom_core::sym::with_session_rules` the same budget and rules the E6
driver would have used, or it is measuring a different tier than the
one that ships. `DriveConfig` is on the façade
(`pncad::analysis::DriveConfig`) and its `symbolic` field is public —
but the field's TYPE, `editor_core::drive::SymbolicDials`, is not
re-exported by `crates/pncad/src/analysis.rs`, and neither are
`DEFAULT_SYM_MAX_TERMS` and `DEFAULT_SYM_MAX_DEGREE`.

What that leaves a consumer:

- reading `DriveConfig::default().symbolic.max_terms`, `.max_degree`
  and `.rules` works, because Rust lets a field of an unnameable type
  be read;
- writing `let dials: SymbolicDials = …`, building a modified copy, or
  storing one in a struct does not, because the type has no spelling
  out here;
- so does restating `4096` and `128` as literals, which is the thing a
  consumer reaches for next and is exactly the drift the constants
  exist to prevent.

`chaintol.rs`'s `drive_dials` is the readable half, written out in
full with this row cited. The same surface already carries
`DriveConfig`, `SymRules` (through `geom_core`) and `SymBudget`, so
the missing name is the one joining them.

## What answers it

Either `SymbolicDials` and the two `DEFAULT_SYM_*` constants join the
`#[cfg(feature = "interval")]` block in `crates/pncad/src/analysis.rs`
beside `DriveConfig`, or the façade states why a consumer is meant to
be able to read the driver's dials and not to name them.

## Home

LIB. The surface is `crates/pncad/src/analysis.rs`, which is LIB's
ground by its `paths` (`crates/pncad/*`) and outside SYM's by its
`keep_out`. Found by SYM-14 measuring the tier from outside
`editor_core`, and filed here rather than on the finder's slate
because `work/README.md` puts a finding on the slate of the program
whose ground it lands on. It sat in `work/sym/` for one review cycle,
which is what its own `## Home` sentence had already said was wrong.
