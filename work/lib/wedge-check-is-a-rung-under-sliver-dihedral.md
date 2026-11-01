---
id: wedge-check-is-a-rung-under-sliver-dihedral
kind: issue
title: pncad: topo::WedgeCheck, SliverDihedral's decision, is a payload rung on no curated list and has no Python word
status: open
opened: 2026-09-29
---


(ENCL implementer, PR 3398.) `scripts/payload-rung-sweep.py --check`
names the rung; its `DISPOSITIONS` row is `argued` non-carriage, and the
argument with its falsifier sits beside the declaration
(`topo::WedgeCheck`, `crates/topo/src/validate.rs`).

## What

`ValidationError::SliverDihedral { edge, check: WedgeCheck, cause }`
now names which of check 4's decisions escalated: `Dihedral` (the
first-order wedge), `SecondOrder` (the tangent second-order separation)
or `MaterialSide`. Their recourses differ. `pncad` re-exports `topo`
whole, so a Rust caller can match it, but the prelude does not list it
and `ValidationError.findings` has no word for it, so a Python caller
reads the decision out of the sentence.

## Repair shape

As `RingContact` is carried (`crates/pncad/src/prelude.rs` group 5): add
`WedgeCheck` to the prelude's validation `pub use`, extend
`carried_refusal_payloads_are_matchable_through_the_prelude`, add a
`wedge_check` word on the Python finding (`crates/pncad-py/src/validation.rs`,
`tags.rs`, `py/value.rs`, `pncad.pyi`) with its arm table, then delete
the non-carriage paragraph on `WedgeCheck` and its `DISPOSITIONS` row.
