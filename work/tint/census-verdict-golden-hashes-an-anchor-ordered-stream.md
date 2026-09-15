---
id: census-verdict-golden-hashes-an-anchor-ordered-stream
kind: issue
title: the shell census golden hashes voided_rod's verdicts in decision order, so a pure re-anchoring of a loop moves it and a sign change is indistinguishable from a move
status: open
opened: 2026-09-14
---

Filed by the TOPO revert-wrap fix pass (PR 2573, 2026-09-14), on this
slate because the golden's instrument lives in `crates/*/tests/*` and
the question is whether the row asserts what its name says.

## Finding

`crates/sweep/tests/shell_census_is_thread_count_invariant.rs` pins
`voided_rod`'s census against a committed digest whose verdict channel
is `common::channels` (`crates/sweep/tests/common/mod.rs`): an FNV-1a
over the `(predicate, sign)` stream in DECISION order. The census's
props walk reads each face's loop from `Cycle::first`
(`crates/topo/src/census.rs`, `face_cycles`), so the stream's order —
and therefore the hash — is a function of where every loop is
anchored. `voided_rod`'s void shell is the rod REVERTED, and PR 2573
made `Body::revert` move every loop's anchor to its source predecessor;
the hash moved (`8c117c40d5b4fc29 → c40da6c25f7ffe61`, all three ε
rows) with every reading bit-identical. Both blinded reviews of that
PR dumped the 40 verdicts on both trees and found that FOUR of them
changed SIGN (`props_rim_side ×2`, `props_rim_dir_group ×2`,
Negative → Positive; the props row on the same finding is
`work/props/rim-side-and-rim-dir-group-signs-are-facts-about-cycle-order.md`)
while 36 only moved — and the golden cannot tell those apart: an
order-preserving structural change and a sign change are the same
kind of miss against a hash of an ordered stream.

Two consequences, both this program's shape ("a row that cannot go
red for the reason its name gives"):

- the row's message says "the census's readings or its recorded
  channels differ" and asks the re-cutter to "say what moved and
  why" — but the instrument does not let a re-cutter answer that
  without an out-of-tree probe; PR 2573's first account of its own
  re-cut ("same multiset, same answers, different order") was false
  and was caught only because two reviewers wrote that probe;
- a verdict channel keyed by cycle order reds on every producer that
  re-anchors a loop (a reversal, a `kev`/`kef` that kills an anchor,
  a graft's key map) whether or not the census changed what it
  decides, which makes it a brittle pin for everyone downstream.

PR 2573's fix pass added the row the suite lacked beside the golden —
`voided_rods_verdicts_as_a_sorted_multiset`, the `(predicate, sign)`
counts order-free — so the two misses are now told apart. What is
still open is the instrument: whether `common::channels` should hash
the multiset (sorted stream) rather than the decision order, or carry
both, and what that costs the other two goldens that share the fold
(`mass_props_are_thread_count_invariant`, `reporting_door_bit_digest`).
The fold itself is one of the FNV copies
`work/perf/fnv-digest-and-memo-machinery-copies.md` lists.
