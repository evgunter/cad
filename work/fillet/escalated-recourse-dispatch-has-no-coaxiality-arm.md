---
id: escalated-recourse-dispatch-has-no-coaxiality-arm
kind: issue
title: blend: an in-band fillet3_support_coaxiality escalation renders no recourse
status: open
opened: 2026-09-05
---

## Finding

`BlendError::Escalated`'s `Display` picks its recourse sentence by the
escalating predicate's name (`crates/sweep/src/blend/mod.rs`, the
`match source.predicate` inside `impl fmt::Display for BlendError`). The
match has arms for the six battery predicates, `fillet3_ring_clearance`
and (since FILLET-H7) `fillet3_cap_transverse`, and a deliberate F6
fall-through that renders "no recourse is recorded for predicate …; this
is a gap in the error table, not advice to act on".

`fillet3_support_coaxiality` (`crates/sweep/src/blend/battery.rs`,
`support_coaxiality`) is not in the match. Its DEFINITE arm refuses
`SpineUnsupported` with `FILLET3_SPINE_KIND_RECOURSE`; its IN-BAND arm
escalates through `esc(BlendSite::Chain, e)` and renders the F6 gap
sentence — so the two-tolerance pair (D4 ¶1 addendum: both arms carry
one recourse) is broken for this one routing decision. Found while
adding the `fillet3_cap_transverse` arm in the same match (FILLET-H7,
PR 1897), which is why the gap was visible; not changed there because
the sentence the in-band arm should carry (`FILLET3_SPINE_KIND_RECOURSE`,
the canal-surface door) is a design statement about which door a
near-coaxial pair belongs to, and that pairing is the arms' owner's.

## Fix shape

One arm: `Some("fillet3_support_coaxiality") => FILLET3_SPINE_KIND_RECOURSE`,
with a row in `crates/sweep/tests/m5_pr12_refusals.rs`'s trio family
that reaches the in-band arm through `run_battery` on a pair whose
stored axes are misaligned by an in-band amount, asserting the sentence.

## Cross-program note

The code is FILLET's (`sweep::blend`); the arm it names is VERBS-ARMS-2's,
which is closed. Filed here as the owner of `blend/mod.rs`.
