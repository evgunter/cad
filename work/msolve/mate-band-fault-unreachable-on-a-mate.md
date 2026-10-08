---
id: mate-band-fault-unreachable-on-a-mate
kind: issue
title: MateFault::Band on a MATE is unreachable through the insert door, so the solve's and the tree's mate-side Band arms are measured by no row — only a loaded snapshot reaches them
status: closed
opened: 2026-09-20
priority: P0
cost: E
parent: MSOLVE-12
pr: 3698
closed: 2026-10-01
---


## What

Found by MSOLVE-10 while moving the coset table's per-mate refusals
to the edit door. `admit_mate` (`crates/editor-core/src/mate/solve.rs`)
begins where the solve begins, at `Band::linear(tol)`, and a tolerance
that admits no band refuses the mate `MateFault::Band` at the INSERT
door — the same answer the solve records, met one evaluation earlier.
Replay runs the same door, so no logged insert of a mate replays under
such a tolerance either.

The consequence: **no insert lands a mate under a bandless
tolerance**. The solve's `Band` arm on a MATE (`solve_with_env`'s
first loop, which records `Band` against every mate and every
instance) and the tree's rendering of a mate row carrying `Band`
(`crates/viewer/src/tree.rs`, `blamed_mates`' empty answer for it)
stay reachable — a SNAPSHOT is a state, not an edit, and a document
whose snapshot holds a mate loads under a bandless ε and evaluates to
`Band` on that mate — but no row in the tree reaches them that way.
The rows that exercised the mate half through the insert were
re-stated:
`msolve8_levered_clash::c4_band_refuses_every_mate_at_the_door_and_reaches_every_instance_{overflow,empty}`
(the door refuses each mate `Band`; the solve reaches every instance)
and `tree_badges::child_band_refusal_rows` (the insert refuses; the
three instance rows carry `Band`, blame no row and point nowhere).

## Why it matters

A `Band` on a mate now stands only in hand-built values
(`crates/editor-core/tests/display_contract.rs`,
`crates/pncad-py/src/tests.rs`'s arm table), which is the shape
discipline §2 calls documentation: no row's runtime value can make
the mate half of those arms false. The arms stay — the state that
reaches them exists. What is owed is a row that reaches it the way it
is reached: a document with a mate saved under a sound ε and loaded
in a process whose witness admits no band (the `tree_badges` band
child's re-exec shape, with a snapshot in hand rather than an insert),
asserting `Band` on the mate row and the tree's no-blame rendering of
it, so the mate half is measured rather than hand-built.

## Where

- `crates/editor-core/src/mate/solve.rs` — `solve_with_env`, the
  `Band` loop over `Node::Mate | Node::InstantiatePart`.
- `crates/viewer/src/tree.rs` — `blamed_mates`, the `MateFault::Band`
  arm (chrome's and view's territory; the finding is the kernel's).
- `crates/viewer/tests/tree_badges.rs` — `child_band_refusal_rows`,
  the row the snapshot road extends.

## Closed (2026-10-01, PR 3698)

`crates/viewer/tests/tree_badges.rs`,
`a_loaded_snapshot_reaches_a_mates_band_refusal`: one child authors a
document holding a mate at ε = 16 m under the default K (a band
exists) and saves it through the save door; a second commits the same
ε with a `CAD_AMBIGUITY_K` that makes K·ε overflow, loads the text, and
asserts the mate's own evaluated value is `MateFault::Band` and that
the tree draws the mate row FAILED in the payload's own words with no
row pointed at it (`blamed_mates`' empty answer). A document records
its ε and not its K, which is how a file authored where a band existed
meets a process with none. The kernel half and the viewer half are one
row: the loader asserts both. Nothing retired: the hand-built values
in `display_contract.rs` and the Python arm table stand.
