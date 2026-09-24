---
id: census-verdict-golden-hashes-an-anchor-ordered-stream
kind: issue
title: the shell census golden hashes voided_rod's verdicts in decision order, so a pure re-anchoring of a loop moves it and a sign change is indistinguishable from a move
status: open
opened: 2026-09-14
priority: P3
cost: E
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

## Re-derived (2026-09-15, lane C)

**VERDICT: REPRODUCES** — the instrument is unchanged, the re-cut hash
is the one this row names, and the open question (what `common::channels`
should hash) is untouched. One factual correction below.

**The instrument.** `crates/sweep/tests/common/mod.rs`'s
`pub fn channels(r: &Recorded) -> String` still builds its verdict byte
stream by iterating `r.verdicts` in order, pushing
`predicate` / `1` / `{sign:?}` / `0` per verdict, and folding it with
`fnv1a`. Its own doc names what it is: *"the counts (readable) and the
order-sensitive hash (complete)"*, over *"an order- and element-sensitive"*
fold. Nothing hashes a sorted multiset.

**The golden.** `crates/sweep/tests/shell_census_is_thread_count_invariant.rs`'s
`check_against_golden` still compares `digest()` against
`expected(eps)`, and its message is still the one the row quotes —
*"the shell census's readings or its recorded channels differ … say in
the PR what moved and why"* — with no instrument that lets a re-cutter
answer. All three committed rows carry the post-2573 value:
`verdicts n=40 h=c40da6c25f7ffe61 esc n=0 h=cbf29ce484222325` in
`shell-census-digest/eps-1e-6.txt`, `eps-1e-9.txt` and `eps-1e-12.txt`,
identically. So the `8c117c40d5b4fc29 → c40da6c25f7ffe61` move and the
n=40 verdict count are both confirmed against the tree.

**The row the fix pass added is there and says exactly what this row
says.** `voided_rods_verdicts_as_a_sorted_multiset` in the same file,
with the rustdoc *"`common::channels` hashes the verdict stream in
DECISION order, so a verdict whose sign changed and a verdict that merely
moved are the same kind of miss there; this row pins each
`(predicate, sign)` with its count, order-free, so the two are told
apart when the golden moves."* It also carries the anchor-relative
argument for `props_rim_side` and `props_rim_dir_group` and names
thirteen predicates, of which two are anchor-relative — consistent with
the four sign changes this row reports (two rows each).

**Correction: one of the two "other goldens that share the fold" does
not.** `grep -rln "channels(" crates/*/tests/` returns three files:
`common/mod.rs` itself, `shell_census_is_thread_count_invariant.rs`, and
`crates/sweep/tests/mass_props_are_thread_count_invariant.rs` (which
calls `channels(&bracket.finish())` in its own `fn line`).
**`crates/sweep/tests/reporting_door_bit_digest.rs` does not call
`channels` at all** — it builds its own `fn digest_line` and compares
against `reporting-door-digest/eps-*.txt`. It shares the golden SHAPE
(its module doc and `shell_census…`'s both say so) but not the fold, so
changing `common::channels` costs **one** other golden, not two. That
narrows the open question's blast radius and should be said when a unit
is cut.

**Unmeasured, by design.** This lane runs nothing, so the claim that the
multiset row and the golden actually disagree in the predicted way — and
the 40-verdict / four-sign-change figures — are taken from PR 2573's two
blinded reviews, not re-derived here. Re-deriving them needs
`cargo nextest run -p sweep --features interval -E
'test(/shell_census_is_thread_count_invariant/)'` at each of the three ε
rows with a verdict dump, which is exactly the out-of-tree probe this row
says the instrument should not require.

**The FNV copy this leans on.** `common::fnv1a` is one of the copies
`work/perf/fnv-digest-and-memo-machinery-copies.md` lists; that row is
still open, and `crates/sweep/tests/blend4_r1_probes.rs` and
`crates/sweep/tests/verbs_tubewall_r1_fingerprint.rs` each still name it
while folding their own way (byte-wise in place, and word-wise).

**Recommendation (orchestrator's call).** Keep open, unchanged in
substance; amend the "two goldens" count to one when the unit is cut.
