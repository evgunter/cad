---
id: planar-mesher-posture-rests-on-a-refusal-check-6-does-not-make
kind: issue
title: mesh/planar.rs's assume-don't-certify posture rests on a check-6 refusal that does not exist for arc-bounded planar faces
status: open
opened: 2026-09-21
priority: P2
cost: D
---


Filed by ATREST-2 (a measurement unit), outside its fence. The
sentence half is easy; the posture half is TESS's design call, which
is why this is a row and not a wording fix landed by the finder.

## The false sentence

`crates/mesh/src/planar.rs`'s module header, under *"What checks the
rule, and what does not"*:

> A body whose stored `sense` disagrees with its stored winding
> violates it, and `topo`'s tier-3 validator refuses such a body by
> name (check 6, `LoopRoleInverted`). **This crate does not run that
> check and does not re-derive it** — tessellation does not
> re-validate […]. So the honest statement is: the rule is ratified,
> its violation is *refusable* upstream, and the mesher assumes a body
> that has been through the door rather than certifying one that has
> not.

Check 6's planar arm does not refuse such a body in general. Its
`all_lines` gate `continue`s past any loop whose certified carriers are
not all `geom::Curve3::Line`, and the skip is deliberate and
banner-documented at the arm (*"an arc's vertex chord is not the
boundary, and its winding is not the region's"*). So for a planar face
whose loop carries a `Circle` or an `Ellipse` — an arc-capped loft,
a merged face out of `merge_coplanar_faces`, an imported cap — the
stored bit and the stored winding may disagree and tier 3 says
nothing.

## The measurement

ATREST-2 built the disagreement through the PUBLIC door
(`topo::Body::set_face_sense`, no `_for_tests` anywhere) on
`sweep::loft_body` output and measured the at-rest battery:

| body | whole-body sense inversion | loops check 6's planar arm examines |
|---|---|---|
| `arc_prism` (bulged section) | `validate_geometric` = `Ok(())`, enclosure bit-identical and positive | 0 |
| `square_prism` (unbulged) | 2 x `LoopRoleInverted`, one per cap | 2 |

A 2x2 over {bulged, unbulged} x {3 stations at v-degree 2, 2 stations
at v-degree 1} isolates it: station count and v-degree move nothing;
the `Circle` carrier on the cap loop is the whole difference.

Pinned, red when any of it changes, in
`crates/sweep/tests/m5_s10_face_sense.rs`:
`only_the_line_bounded_cap_refuses_a_whole_body_sense_inversion`,
`every_sense_reading_gate_shuts_on_the_arc_loft`,
`the_public_sense_door_builds_an_inverted_arc_loft_tier_3_accepts`.
Write-up:
`work/atrest/sense-inversion-is-invisible-to-tier-3-on-arc-capped-lofts.md`.
The exemption itself is a RECORDED residual, not news —
`work/verdict/m6-sense-gate-recorded-residuals.md` residual 4 has held
it since 2026-08-07, and residual 3 holds the spline-chart half. What
is new is that a body carries both over its whole face population, and
that a downstream crate's posture is written against the refusal.

## What is actually at stake for `mesh`

**Probably nothing, and the row says "probably" on purpose.**
`planar.rs`'s own header argues two paragraphs earlier that
[`chart_frame`] reads no chart — only walk indices and 3-D points — so
the outward normal it uses is Newell over the stored WALK and the
`sense` bit does not enter this lane at all. If that holds end to end,
an inverted bit on an arc-bounded planar face is invisible to the
planar mesher and the posture is safe for a reason that has nothing to
do with check 6.

But that is the sentence the header did NOT make. What it wrote is
that the violation is refusable upstream, and that is the load-bearing
claim: it is what lets every other lane in the crate say "the mesher
assumes a body that has been through the door". Two things follow and
neither is the finder's to decide:

1. **Which of `mesh`'s lanes actually read `Face::sense`**, as opposed
   to the stored walk. Wherever one does, the upstream refusal it is
   leaning on is absent for this face class and the assumption is
   unbacked. That is a census, not an argument.
2. **What the posture becomes when the refusal is absent.** The
   options are not equivalent: re-derive the coherence locally in the
   lanes that need it; state the narrower premise (Newell over the
   walk, bit-free) and drop the check-6 citation entirely; or keep
   assume-don't-certify and record the class as a residual with its
   own flip condition. The third is honest only if (1) comes back
   empty.

## The class

This is one of three instances of **prose justifying itself by a check
that does not reach this population**, all found from the one
measurement:

- `crates/step-import/src/normalize.rs`'s guard and
  `crates/step-import/src/lib.rs`'s step-5 contract sentence — filed on
  EXCH as
  `step-import-adopts-an-inverted-same-sense-outside-the-cylinder-cone-guard`;
- `topo::Body::set_face_sense`'s rustdoc — narrowed by ATREST-2 in
  `crates/topo/src/attach.rs`;
- this one.

The failure mode is that the sentence reads correct because the check
it names is real and spelled right. ATREST-2 read this file during the
measurement that falsified its premise and did not notice, which is
the class working as advertised.

## Flip condition

The sentence stops being false the day check 6's planar arm widens
past line carriers — `work/zip/verbs-1031b-assigner-checker-divergence.md`
holds that open question and its cost. Until then the wording and the
posture are this row's.

**2026-09-24 (ATREST-4).** Check 6's planar arm now examines planar
loops of `Line` and `Circle` carriers: an arc-bounded planar face whose
stored `sense` disagrees with its stored winding refuses
`LoopRoleInverted` by name (the arc loft's caps, an extruded washer's
outline and ring — pinned in `crates/sweep/tests/m5_s10_face_sense.rs`).
The header's sentence is now true for those faces. It is still false
for a planar loop riding an `Ellipse`, spiric or NURBS carrier (an
oblique cut of a cylinder, an imported cap):
`work/atrest/check-6-planar-arm-skips-ellipse-and-nurbs-loops.md`. The
posture question stands on that narrower population.
