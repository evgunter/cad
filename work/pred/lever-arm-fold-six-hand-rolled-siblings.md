---
id: lever-arm-fold-six-hand-rolled-siblings
kind: issue
title: The two-surface lever-arm fold has one documented home and six hand-rolled siblings; contact_tangent_opposed is classify_material_pairing's undisclosed twin
status: open
opened: 2026-09-01
github: 1439
refs: [1423]
priority: P1
cost: D
---

## From GitHub issue 1439

Opened 2026-09-01; 0 comments.

Filed from the MATE-3 dual review (PR #1423; found bilaterally — one arm from the prose, one from the constants/data sweep). MATE-3 introduced `geom_brep::folded_lever_arm` documented as "one home for the fold" (three consumers named, margins comparable only if levered against the same arm) — but the identical three-way `min` fold survives hand-rolled at six sites: `topo/boolean/contact_verify.rs:351-353` (the fold's own stated origin), `geom-brep/certify.rs:1685-1687` and `geom-brep/ssi.rs:894-896` (the helper's OWN crate), `topo/boolean/ops.rs:1020-1022`, `sweep/revolve/upgrade.rs:233-235`, `sweep/extrude.rs:882-884`. Two of eight sites use the home.

Deeper half: `contact_tangent_opposed` (`contact_verify.rs:349-360`) is arithmetic-for-arithmetic the new `classify_material_pairing` (`normalize(∇F)·sense`, dot, `Margin::levered`) under a second predicate name — the dependency direction admits calling the shared home.

All six sites were legitimately outside MATE-3's fence; scheduling this here rather than half-fixing one crate in passing. The consolidation is mechanical but crosses four crates and re-homes a predicate name, so it wants its own small unit (any program touching those lanes may take it; S-MATE has no claim on the boolean/sweep sites).

Signed: (S-MATE orchestrator)

## Progress (FILLET-H6, PR 1891)

Three of the six are gone, without this issue being taken: FILLET-H6 hoisted
the second-order margin into `geom_brep::tangent_second_order`, which folds the
arm through the documented home, and migrated the three sites that read that
margin — `geom-brep/certify.rs` (the tangency certificate's interior samples),
`sweep/extrude.rs` (the strut join) and `sweep/revolve/upgrade.rs`
(`jet_determinate`). It also migrated `topo/boolean/rim_wedge.rs`, which was a
hand-rolled sibling of the SECOND-ORDER rule but already levered against
`folded_lever_arm`.

**The three that remain**, and why each was left:

- `topo/boolean/contact_verify.rs:351-353` — the fold's own stated origin, and
  the site whose `contact_tangent_opposed` is `classify_material_pairing`'s
  undisclosed twin. Re-homing that predicate name is the deeper half of this
  issue and wants its own decision.
- `geom-brep/ssi.rs:894-896` — surface–surface intersection's own arm, not a
  second-order margin, so `tangent_second_order` does not reach it; it needs the
  bare `folded_lever_arm` swap.
- `topo/boolean/ops.rs:1037-1039` — folds the margin into a per-sample rebuild
  walk it already runs, like the tier-3 validator's.

The tier-3 validator (`topo/validate.rs`) was never on this list — it already
levers against `folded_lever_arm` — but it does hand-roll the second-order
DECIDE, and it and `boolean/ops.rs` are now the only two that do.

## Home

`work/code-quality/` — a duplicated-spelling structural finding (one documented home, six hand-rolled twins) crossing four crates, which the issue explicitly declines to route to any one program.

## Re-homed to PRED (2026-09-11, the cut in `docs/WORK-TRACKS-2026-09.md` addendum 3)

PRED collects the rows where one numeric fact is decided in several
places, each with its own margin. This row is one of them.

Its class at the cut was **M** — three sites left; two are swaps, one
re-homes a decide-name predicate. The class is a dispatch estimate made
by reading the row against the tree on 2026-09-11, not a verdict on the
finding, and a lane that finds it wrong says so in its PR. The id, the
`track:` letter where the row carries one, and the body above are
unchanged by the move.

## A seventh spelling pair: the parallelism fallback lever (ENCL, PR 3334)

The tangency certificate's second-order refusal arm
(`geom-brep/src/certify.rs`, `run_checks`'s `Resolved::Tangent` arm,
the `renamed` reading ~1950) now spells the rule
`topo/boolean/contact_verify.rs`'s `tangent_locus_relation` spells at
~394–430: "`levered_inv(sin θ, |κ_rel|)` when the second-order margin
is definitely positive, else `levered(sin θ, folded arm)`", under the
same kind of predicate name (`tangent_normal_parallel` /
`contact_tangent_parallel`). They differ deliberately in role —
certify reads the fallback only after the second-order margin has
refused and lets only a definite reading rename that refusal, while
contact_verify decides first-order on it and escalates an in-band
reading — and incidentally in arm: certify's comes from
`folded_lever_arm` through `tangent_second_order`, contact_verify's is
its hand-rolled fold (the first bullet above). Each site's comment
names the other. A consolidation would put the lever choice (which arm,
given a second-order verdict) in one home beside `folded_lever_arm` and
leave the role difference at the call sites.

## Progress (SSI lever-arm lane)

`geom-brep/ssi.rs`'s ℝ³ finisher now calls `folded_lever_arm`, and so does
a site this row never counted: `ssi::system`'s `ImplicitPairR3::lever_arm`
(the march's transversality arm), which folded the two curvature arms by
hand and left the extent to the march. Both were concrete `f64` code, so
their `.min` was the inherent `f64::min`, which drops a NaN operand — a
NURBS operand's poison arm fell to the sibling's radius. That was the
reason to move them, not just the duplication.

**The two that remain are generic** (`T: Decide` in `contact_verify.rs`
and `boolean/ops.rs`), so their `.min` resolves to `Real::min` and
propagates poison already: a clippy `disallowed-methods` pass over
`f64::min`/`f64::max` does not flag either. They are duplications only,
not the poison-dropping class.
