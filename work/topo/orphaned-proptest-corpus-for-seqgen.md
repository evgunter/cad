---
id: orphaned-proptest-corpus-for-seqgen
kind: issue
title: the tracked seqgen proptest-regressions corpus is orphaned by the row's file move and never replayed
status: open
opened: 2026-09-06
track: P
refs: [S69, 2014]
---

## What

`crates/topo/proptest-regressions/seqgen.txt` is tracked, 4.5 KB, and
holds **five `cc` entries** — counterexamples proptest saved from real
failures of `random_op_sequences_hold_all_properties`, two of them with
long prose headers recording what they found and how it was fixed. Its
own header says the file "is automatically read and these particular
cases re-run before any novel cases are generated."

It is not. Proptest derives the regression path from the **source file
of the test**, and that row now lives in
`crates/topo/src/seqgen/random_op_sequences.rs` — split out of
`seqgen.rs` so a `gated_to!` marker could gate the whole module. So
proptest reads and appends
`crates/topo/proptest-regressions/seqgen/random_op_sequences.txt`; the
tracked `seqgen.txt` is the pre-move path and nothing opens it. Observed
directly: a deliberately planted mutant during `S69` (PR 2014) produced
a failure, and proptest created the `seqgen/` subdirectory and wrote the
new file there, leaving `seqgen.txt` untouched.

So five saved counterexamples stopped being replayed at the file move,
silently, and the file has been reading as a live corpus ever since.

## Which entries have a pin, and which do not

`memories/test-suite-cost.md` and `test_utils::fuzz`'s discipline both
say the same thing: *"A sweep that finds a real defect has produced a
specific counterexample. Pin that case as an ordinary deterministic
test alongside the fix — the sweep's job is to find it, not to be the
regression gate for it afterwards."* Measured against that:

| entry | header | deterministic twin |
|---|---|---|
| `cc b4743f94…` | none | **none** |
| `cc 68b4cfd4…` | none | **none** |
| `cc 07fc1979…` | none | **none** |
| `cc dda6d5e0…` | issue #60, iso oracle tie-break | `seqgen::tests::issue_60_kef_roundtrip_on_coincident_ring_twins` |
| `cc 32125c56…` | the `split_edge` one-ulp split point | **none** — the header describes the fix (`split_site`'s definite-separation filter) but no test pins the case |

**Only one of five has a twin.** Four defects are recorded nowhere but
in a file nothing reads, which is the worst of the two states: the
corpus is not gating, and its existence has been the reason not to
write the pins.

## What closing it looks like

Not "move the file". The corpus was never the right regression gate —
per the rule above it is the pins that gate — so:

1. Write the deterministic pin for `cc 32125c56…` first: its header
   names the class exactly (two edges over the same point pair, split
   points one ulp apart, the next `mef_chord` refusing the chord as
   `IntervalNotForward`), and that is concisely constructible, so it
   wants a fixture and not a replayed seed.
2. Replay the three unlabelled entries against current `main` to find
   out what they were, then either pin what they found or record that
   the class is already covered.
3. Then delete `seqgen.txt`, and decide deliberately whether the live
   `seqgen/random_op_sequences.txt` is checked in at all — a file
   proptest appends to on every red run is a corpus that grows without
   review, and the tree's other proptest rows
   (`crates/profile/tests/*.proptest-regressions`) already carry that
   question.

Step 3 must not come first: deleting the file before step 2 loses the
only record of what those three runs found.
