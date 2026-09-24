---
id: cavity-module-doc-restates-a-census-that-is-now-wrong
kind: issue
title: sweep/tests/common/cavity.rs restates the brick/prism census in its own module doc, and both equalities it asserts are false today
status: open
opened: 2026-09-15
priority: P4
cost: E
---


(S-TINT orchestrator, 2026-09-15) Found by re-derivation lane D while
checking `sweep-boolean-suite-brick-and-prism-copies`. Filed separately
because it is a defect **in the guard**, not in the population that row
counts.

## The defect

`crates/sweep/tests/common/cavity.rs`'s module doc restates the
brick/prism duplication census in the tree, asserting two byte-identical
pairs —

- `sf2a_r2_probes.rs` = `verbs_shell.rs`, and
- `m8_4_intersection_iso.rs` = `r1_p2_probes.rs`,

*"plus six singletons"*. **Neither equality holds today.** The first pair
is one line apart (`pub(crate)`). The second is still byte-identical but
of a **different fixture**: both are now `fn prism(scale: f64)`, a
lofted offset square prism through `sweep::loft_body`, so the sentence
is true of a name and false of the thing. `tcost_k3_certificate.rs::prism`
has left the class entirely (`fn prism()` → `arc_prism`), and
`shell5_r1_dump.rs` has joined it. The class is nine today.

**This is the program's own named shape sitting inside the file that
declares the rule.** A hand-kept census, in prose, with nothing that goes
red when the population moves — and it moved in four separate ways while
the sentence stayed put. The standing rule is *a census has one
executable home and every other site points at it*; this is a second,
non-executable home.

## The trivial sibling in the same directory

Two in-tree comments cite tracker paths that no longer exist, both
naming rows that moved to `work/tint/` on 2026-09-11:

- `crates/sweep/tests/common/cavity.rs` → `work/tcost/sweep-boolean-suite-brick-and-prism-copies.md`
- `crates/sweep/tests/common/oracles.rs` → `work/tcost/chamfered-cube-and-steiner-oracles-outside-sweep.md`

Recorded here rather than as a row of their own because a lane opening
`cavity.rs` for the census above fixes one of them in the same edit and
`oracles.rs` is its neighbour. **Not swept beyond these two**: lane D
observed that `scripts/ci-filter.py` and several `docs/` files also
carry `work/tcost/` paths, but those name S-TCOST's own rows and were
not checked — `scripts/` is S-TCOST's territory regardless.

## Disposition, unresolved

Whether the module doc should be **corrected** or **deleted** is the
question, and it is the same one the parent row asks: if the population
gets an executable home, a prose restatement of it is exactly the copy
this program exists to remove; if it does not, the prose is the only
record and wants a keeper. A taker decides that once, for both.
