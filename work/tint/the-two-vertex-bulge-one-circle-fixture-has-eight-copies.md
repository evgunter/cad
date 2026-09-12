---
id: the-two-vertex-bulge-one-circle-fixture-has-eight-copies
kind: issue
title: The two-vertex bulge-1 circle section is hand-written in eight sweep/profile test files, the newest added by a PR whose subject was retiring hand-spelling
status: open
opened: 2026-09-12
refs: [2409]
---


## Finding

From the full review of WIRE's PR 2409 (S6, confidence `sure` on the
duplication, `unsure` on the disposition — which is why it is filed
rather than fixed). Filed here by the WIRE orchestrator because
`crates/*/tests/*` is S-TINT's glob and this is a question about what
the suite asserts and how, not about kernel behaviour.

`crates/sweep/tests/wire_loft_end_profile_lift.rs:29-34`'s `circle(r)`
is **byte-for-byte** `crates/sweep/tests/m7_skin_integral.rs:368`'s
`circle_section`. The same two-vertex bulge-1 circle recurs as a
`circle_loop`-shaped helper at:

- `crates/sweep/tests/extrude_acceptance.rs:52`
- `crates/sweep/tests/fillet_h6_cap_rim.rs:106`
- `crates/sweep/tests/p1b_r1_probes.rs:115`
- `crates/sweep/tests/review_m2_pr4.rs:46`
- `crates/sweep/tests/blend6_verb_vocab.rs:65`
- `crates/sweep/tests/review_blend6_r2_probes.rs:264`

Eight copies of one fixture.

## The part worth keeping

It is **pre-existing**, and the newest instance was added by a PR whose
entire subject was retiring hand-spelled constructions — the standing
trap, at the fixture layer this time rather than in `src/`. That is the
fifth time this shape has fired on WIRE's units, and the first at the
test layer, which is worth knowing because every previous instrument
pointed at `src/`.

## What a taker owes, and what it should NOT assume

**Not automatically a shared helper.** `memories/test-suite-cost.md`
asks which SHAPE a test is before giving it a seed, and reviewer-authored
probe suites have an independence-from-the-library argument that a
shared fixture would consume — `review_m2_pr4.rs`,
`p1b_r1_probes.rs` and `review_blend6_r2_probes.rs` are review lanes'
own rows and may be exactly the files that should keep their copy. The
same retain/retire criterion `D385` applies to hand-written lifts
applies here.

So: decide **per file**, and where a copy stays, say at the copy site
why — which is the thing none of the eight currently does, and the
reason a reader cannot tell an independent fixture from an unnoticed
duplicate.
