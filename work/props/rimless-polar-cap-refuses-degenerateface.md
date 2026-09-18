---
id: rimless-polar-cap-refuses-degenerateface
kind: issue
title: props: a rimless-boundary polar cap — one circular edge, no meridian — refuses DegenerateFace; its extent has no arc to derive from
status: closed
closed: 2026-09-16
pr: 2741
branch: props/sphere-pole-side
opened: 2026-08-29
github: 1250
refs: [723, 1220]
---

## From GitHub issue 1250

Opened 2026-08-29; 0 comments.

**Filed from CERT-1's fix pass (S-CERT), found independently by both blinded reviewers.** Deliberately not fixed in PR 1220, whose ground is the meridian-arc extent (issue 723) and the rim lever (issue 893); this is the sphere arm the span-derived fold cannot reach because there is no span to read.

## The input

A spherical cap face bounded by **one rim circle and nothing else** — `[0, 2π] × [v₀, π/2]`, the pole interior to the face, no meridian edge. Entirely ordinary STEP (a ball cut by one plane produces exactly this face) and an ordinary native construction.

## The disposition, executed (identical before and after PR 1220)

```
full cap: one rim circle, no meridian    REFUSE DegenerateFace
half cap: rim + pole-crossing arc        ACCEPT rel=-1.66e-16
```

(`r2_probe_sphere_polar.rs::probe_polar_cap_no_meridian`, committed on the PR 1220 branch.) The face's levels list holds only the rim's own latitude sine, so `min_max` gives `lo == hi` and `require_extent` refuses `DegenerateFace`. The half of the same cap bounded by a pole-crossing arc certifies exactly — so the whole is refused while its half is served, the same one-edge-flips-the-answer alarm shape issue 723 recorded for the split vertex.

## Why PR 1220's fold cannot see it

The span-derived extent (`sphere_meridian_pole` fold) reads pole latitudes out of **meridian arc spans**. A rim-only boundary has no meridian: the fact "the pole is interior to the face" is encoded in the loop's winding around the chart pole, not in any edge's parameter span. Serving it needs a different derivation — e.g. the rim's `d_u` traversal direction plus which pole the face's chart side contains (the material-side machinery already distinguishes this), setting the missing extreme to ±1.

## Classification

D2 addendum **row 2**: reachable by input, valid, lane unbuilt — the refusal is typed (`DegenerateFace`, arguably the wrong name for a face that is not degenerate; renaming or retyping belongs to whoever builds the lane). The honest serving alternative today is the certified-quadrature lane at the cost of a `pad > 0` enclosure.

## Sibling to check

The **cone apex cap** — a cone face bounded by one rim with the apex interior — has the same shape: no generator edge, extent from `min_max` over one level, `lo == hi`. Whether it reaches the same refusal (or is caught by `props_cone_nappe` first) was not measured here; whoever takes this issue should check it alongside.

Refs: issue 723 (the meridian-arc half of the extent premise), PR 1220 (CERT-1), the two reviewer probe branches `cert/1r1-probes` (`probe_full_polar_cap_disposition`) and `cert/1r2-probes`.

## Home

`work/cert/` — `crates/geom-brep/src/props/*` is S-CERT territory and the charter names the sphere polar acceptance defects; filed from CERT-1's fix pass.

## Re-homed (2026-09-06)

Moved from `work/cert/` to `work/props/` on S-CERT's exit walk PR
(#1924, its handoffs ledger; merged by Ev 2026-09-06 = ratified), before
`work/cert/` was deleted at sweep 7 of `docs/DOC-LEDGER.md`. Id, body
and header are unchanged; the directory is the claim (`work/README.md`).
The `## Home` section above naming `work/cert/` is superseded by this
line and is kept as the record of why the file was filed there.

## Served (PROPS sphere-pole-side, 2026-09-15)

The missing extreme is the pole on the interior side of the rim, and
the fact that names it is the rim's own traversal:
`σ = d_u_sign × sense`, the side of the rim the material lies on.
`sphere_rim_only_pole_level` pushes `σ·1` into a meridian-free rim
boundary's levels when they carry no extent of their own, so the cap
folds to `[v₀, +1]` or `[−1, v₀]` and `R²·Δu·(sin v_hi − sin v_lo)`
measures it with `Δu = 2π` summed from the rim's spans.

Executed, `r2_probe_sphere_polar::probe_polar_cap_no_meridian`:

```
  full cap: one rim circle, no meridian   ACCEPT area=3.270865807134998e-4 exact=3.270865807134999e-4 rel=-1.6574e-16
```

Through the public doors: `crates/topo/tests/props_sphere_cap_door.rs`
assembles a ball cut by one plane, certifies it at all three tiers and
weighs it against `πh²(3R − h)/3` with `volume_pad = 0`; the same rim
traversed the other way weighs the rest of the ball; a sphere split by
one rim into two rim-only caps weighs `4π/3`.

`DegenerateFace` stays for the true zero-extent patch — rims at one
level whose traversals disagree name two opposite poles, so nothing is
pushed (`rims_at_one_level_with_opposite_traversals_stay_degenerate`).

**A pole is interior only to a rim that CLOSES.** Found by the dual
review (R1's `probe_c1b_partial_and_doubled_rims`, PR 2741): the fold
reads a traversal DIRECTION, which says nothing about how far the rim
goes, and no other premise on the arm was watching either. Until
`props_rim_only_closed` was added — `(Δu − τ)·R`, the arc the rim fails
to close by — half a rim answered half the cap's area, a quarter a
quarter, the same full rim stated twice double, and a full rim plus a
half arc 1.5×. Each was a `DegenerateFace` before the fold, so the
first landing of this fix turned four typed refusals into wrong numbers
at the public door; the row is
`a_rim_only_cap_refuses_a_rim_that_does_not_close`.

**The sibling checked, as the issue asked.** The cone apex cap refuses
`DegenerateFace` by the same `lo == hi` path and is NOT served here.
Filed as `cone-apex-cap-refuses-degenerateface` with the measurement
and with the shape the fold would take. The cylinder's rim-only face is
genuinely extent-less and needs no lane.

## Correction to the imported text above

The line *"The honest serving alternative today is the
certified-quadrature lane at the cost of a `pad > 0` enclosure"* is
**false**, and was when it was written. `topo::props`' per-face
dispatch routes STRUCTURALLY on the carrier kind: only an
`Ellipse`/`Nurbs`-trimmed boundary or a spline chart enters `quad(…)`,
and a circle-bounded sphere face refused by `curved_face` is
`map_err(wrap)?` — final, with no enclosure to fall back to. A face
this lane refuses has no measuring path at all; what a caller can do is
STATE it differently (split the notch out with a meridian). Found by
the R2 review lane on PR 2741, which read the dispatch rather than the
sentence; the same sentence stood in `require_iso_rectangle`'s docs and
is corrected there too.

## Closed

Landed on PR #2741 (run 35081606164 green). A rim's TRAVERSAL names
which side of it the face's interior lies on — σ = the rim's direction
sign times the face's sense — so when the levels carry no extent of
their own the pole on the interior side is pushed into them and the
cap measures `2πR²(1 ∓ sin v₀)` instead of refusing `DegenerateFace`.
Both poles, both traversals, five latitudes, and through the public
doors: a ball cut by one plane certifies at all three tiers against
`πh²(3R − h)/3` with both pads zero.

The dual found that the first landing pushed that pole without ever
deciding the rim CLOSES: a half rim measured half the cap, a quarter
rim a quarter, the same rim stated twice measured double. All three
refused `DegenerateFace` before this unit, so the first pass turned
typed refusals into wrong numbers at a public door — the spec asserted
`du_of_rims` sums a full rim to `τ` rather than requiring a decide, and
the unit inherited the assertion. `props_rim_only_closed` now decides
it, metering `Δu − τ` at the sphere radius: the arc the rim fails to
close by.

The sibling is measured and filed rather than served
(`cone-apex-cap-refuses-degenerateface`): the cone's missing extreme is
the apex, the only candidate, so the fold needs no σ — but an unguarded
fold would answer the UNBOUNDED complement with the cap's area, which
is this unit's own defect repeated, and `fn cone` takes no sense bit to
guard it with.
