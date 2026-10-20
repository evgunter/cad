---
id: offset-distance-kinds-differ-ends-as-a-kernel-defect
kind: issue
title: geom-brep: offset_distance's KindsDiffer ends in KERNEL_DEFECT_ENDING, though any caller of the pub fn reaches it
status: open
opened: 2026-10-20
parent: a-typed-kernel-defect-ending-may-launder-a-bug-d2-forbids
priority: P3
cost: E
---

A child of the census in `work/encl/a-typed-kernel-defect-ending-may-launder-a-bug-d2-forbids.md` (as of main 5df6e6c854); why a typed `KERNEL_DEFECT_ENDING` is wrong in either D2 class is that row's "What" and "Shape of the work".

## Sites

- `crates/geom-brep/src/offset.rs` `OffsetDistanceError` `Display`,
  `KindsDiffer` (~539). Door: `pub fn geom_brep::offset_distance(from,
  onto)`. **Class 1, sure**: the comment says "the caller asked for the
  distance between two surfaces it holds as an offset pair, so a kind
  mismatch is its defect", but the caller is anyone. Two test files
  (`tests/approx_surface.rs`, `tests/offset_mint.rs`) hand it unpaired
  kinds. Only the shell op's own call (`topo/src/shell.rs` ~3831)
  holds the pairing. The lever: pass an `onto` of the kind `from`
  offsets to, as `OffsetDistanceError::Offset`'s arms already name
  theirs.

## Repair shape

- **Class 1**: the decision's own lever, in its own words (`[tolerance.recourse]` (i)), with a test that reaches it through the door.

Re-check each `likely` or `unsure` argument before acting on it; one that fails moves the site to class 1.
