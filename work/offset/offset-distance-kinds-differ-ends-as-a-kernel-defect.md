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

(ENCL implementer, from the census in
`work/encl/a-typed-kernel-defect-ending-may-launder-a-bug-d2-forbids.md`,
on main 5df6e6c854.)

`[failure]` (D2) splits every state in two: one input reaches, which
is a typed refusal whose ending must name the decision's lever, and one
only a kernel bug reaches, which MUST panic ("downgrading one … to a
typed error launders a bug into a supported outcome"). A typed refusal
ending in `geom_core::KERNEL_DEFECT_ENDING` is therefore wrong in either
class. The one exception is a kernel approximation limit, which takes
`[tolerance.recourse]`'s last resort (`KERNEL_LIMIT_RECOURSE` /
`KERNEL_LIMIT_LAST_RESORT`), not this ending.

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

- **Class 2** (only a kernel bug reaches it): `unreachable!` or
  `debug_assert` per `[failure.taxonomy]` rows 4–5, answering row 0
  first (a type that cannot hold the state is preferred). The tests
  that pin the refusal's text move to `#[should_panic]`, or are deleted
  where they only built the error value by hand.
- **Class 1** (input reaches it): the decision's own lever, written in
  its own words (`[tolerance.recourse]` (i)), and a test that reaches it
  through the door.
- **Class 3** (an approximation limit): the last-resort recourse.

Re-check each `likely` or `unsure` argument before acting on it: one
that fails moves the site to class 1.
