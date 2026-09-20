---
id: mate-band-fault-unreachable-on-a-mate
kind: issue
title: MateFault::Band on a MATE is unreachable through the doors: the insert refuses it, so the solve's and the tree's mate-side Band arms are guarded only by hand-built values
status: open
opened: 2026-09-20
---


## What

Found by MSOLVE-10 while moving the coset table's per-mate refusals
to the edit door. `admit_mate` (`crates/editor-core/src/mate/solve.rs`)
begins where the solve begins, at `Band::linear(tol)`, and a tolerance
that admits no band refuses the mate `MateFault::Band` at the insert
door — the same answer the solve records, met one evaluation earlier.
Replay runs the same door, so no log with a mate loads under such a
tolerance either.

The consequence: **no document can hold a mate under a bandless
tolerance**, so the solve's `Band` arm on a MATE (`solve_with_env`'s
first loop, which records `Band` against every mate and every
instance) and the tree's rendering of a mate row carrying `Band`
(`crates/viewer/src/tree.rs`, `blamed_mates`' empty answer for it) are
reachable through the doors only for INSTANCES now. The rows that
exercised the mate half were re-stated at the door:
`msolve8_levered_clash::c4_band_reaches_every_row_{overflow,empty}`
(the door refuses each mate `Band`; the solve reaches every instance)
and `tree_badges::child_band_refusal_rows` (the insert refuses; the
three instance rows carry `Band`, blame no row and point nowhere).

## Why it matters

A `Band` on a mate now stands only in hand-built values
(`display_contract.rs`, the Python arm table), which is the shape
discipline §2 calls documentation: no runtime value can make the
mate half of those arms false. Either the arm's mate half is retired
— the solve records `Band` against instances only, and the tree's
mate-side handling goes with it — or a door that produces it is
named. The instance half is unaffected and stays measured.

## Where

- `crates/editor-core/src/mate/solve.rs` — `solve_with_env`, the
  `Band` loop over `Node::Mate | Node::InstantiatePart`.
- `crates/viewer/src/tree.rs` — `blamed_mates`, the `MateFault::Band`
  arm (chrome's and view's territory; the finding is the kernel's).
