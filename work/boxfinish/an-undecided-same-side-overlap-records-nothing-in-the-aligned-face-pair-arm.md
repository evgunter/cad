---
id: an-undecided-same-side-overlap-records-nothing-in-the-aligned-face-pair-arm
kind: issue
title: An undecided same-side overlap records nothing in the census's aligned face-pair arm, and what reads the gap it leaves is untraced
status: open
opened: 2026-11-15
priority: P2
cost: E
refs: [6958]
---

Found by the aimed-ray unit's closing pass (PR 6958), sweeping `census.rs`
for an undecided reading turned into a fact (`.ok()`, `matches!`,
`if let Ok(`). It is the one hit that sweep left open.

**What is there.** In the census's aligned face-pair arm
(`CarrierRelation::SameOriented`, `crates/topo/src/census.rs:6554`),
`if let Ok(true) = same_side_overlap(..)` records a finding on a
definite overlap and records nothing on `Err`. So an overlap the
reading could not decide leaves no row.

**What is not known.** The arm's doc says the conformal arm and the
backstop read what it leaves. Nobody traced that: whether an undecided
overlap here is always picked up there, at every scalar, or is silently
"no overlap".

**The class.** PR 6958's MAJOR was an undecided reading read as a fact
(`edge_meets`' `Err(gap)` as "met in band"). This is the opposite
spelling, undecided read as absent, in a reader that decides what a
census records.

**What closes it.** Trace the two arms named, with a fixture whose
`same_side_overlap` is undecided (a pair in band; a pair over a box),
and show the pair is still reported; or make the arm refuse.
