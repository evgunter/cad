---
id: a-plane-touching-a-bore-rim-splits-into-a-pinched-side-under-one-normal-and-refuses-under-the-other
kind: issue
title: a plane touching a tube's bore rim at one point splits into a pinched side under one normal and refuses NotAlternating under the other
status: closed
opened: 2026-10-07
priority: P1
refs: [a-rim-touching-split-escalates-on-the-side-of-plane-band, split-sides-are-not-finished-bodies]
cost: M
pr: 5951
branch: splitdoor/hole-rim-tangent-germs
closed: 2026-10-28
---


## What

Found by the sibling sweep of `cleave/rim-touch`, on `origin/main` at
`dfcd7f35`, at ε = 1e-9, 1e-6 and 1e-12 alike. The fixture is the tube
of `split_across_a_revolve_seam.rs`: `TUBE` revolved a full turn about
`y`, with the bore at r 0.5. The plane passes through the bore's top rim
point `P = (0.5 cos a, 1, 0.5 sin a)` with normal
`s·(ŷ cos 0.2 − (cos a, 0, sin a) sin 0.2)`. So the bore rim lies on the
plane's `+ŷ` side and touches it only at `P`, and on the top cap the
section line is tangent to the hole at `P`.

The cap-side half is therefore pinched at `P`. Near `P`, the material
above the plane is `−v² ≤ u < 0` (u radial, v tangential): two cusps
that meet at one point. The outer rim's touch (the parent row) leaves a
single region there, `u ≤ −v²/2`, and builds.

The two normals of the same plane answer differently:

- **`s = +1`** refuses at 95 of 96 azimuths with
  `Join(SectionCrossings { face, case: NotAlternating })`, from
  `splitting::join::conic_pairs` at its final alternation check. Only
  azimuth 0, the seam vertex, builds. `conic_pairs`' own doc says no
  shipped fixture reaches `NotAlternating`.
- **`s = −1`** builds at every azimuth, and tiers 1 and 3 pass. The
  cap side holds the wedge's closed form, 0.264337438 (by quadrature of
  `tan t ∫∫ (0.5 − d·x) dA` over the annulus where `d·x < 0.5`).
  - The cap side holds **two vertices** within 1e-6 of `P`.
  - Tier 3′ refuses both halves at every azimuth but azimuth 0:
    `UndeclaredContact { VertexOnFace { vertex 8v3, face 10v1 } }`
    below, and `{ vertex 9v1, face 12v1 }` above.
  - The witness is the bore's seam crossing, `(0.5, y, 0)`, e.g.
    `(0.5, 0.98292, 0)` at a = 6π/32. It is not `P`.
  - Why the contact names the seam crossing is unmeasured.

## Where to look

The `s = +1` refusal reads two crossings at one point (`P`, where the
section ellipse enters and leaves the bore wall's face) in insertion
order. `ConicCrossingsCase::NotAlternating`'s doc names that case
itself. So which arm a pose reaches is set by insertion order, not by
the geometry.

The `s = −1` build is the pinch side `split-sides-are-not-finished-bodies`
describes: a side whose touching pieces carry contacts the split
declares nowhere.

What the right answer is depends on D10's coincidence work. That is
whether the split builds the pinched side and reports its contact as an
`unproven-coincidence` finding, or refuses the pinch as one decision
with one recourse, as rule (a)'s knife edge does. Under D10's intent
hold, no declaration channel should be built for it. What does not
depend on D10 is that one plane under its two normals must give one
answer.

## Reproduce

In `split_across_a_revolve_seam.rs`, take
`a_section_touching_a_rim_splits_at_the_closed_form` and change the
touch point to `Point3::new(0.5 * d.x, 1.0, 0.5 * d.z)` with
`lean = 1`. Then run
`split(&finished(tube), &plane, tol)` at azimuth 0.3. `s = 1` refuses,
and `s = −1` builds halves that fail `validate_pseudomanifold`. The
sweep that found it ran 96 azimuths, `2πi/96 + 0.0123·(i mod 3)`.

## Slate triage (2026-10-26, CLEAVE)

- Re-banded P3 → P1: one plane refuses under one normal and ships sides tier 3′ refuses under the other: verb breadth beyond the everyday shapes (a split or boolean that refuses, or ships a body its own gate fails, on a legal input), P1 by `work/README.md` Priority.
- Priced `M`, the effort the body describes.
- Moved to SPLITDOOR (`work/splitdoor/`) by `git mv`, id and body unchanged: the P1 track opened for split and boolean shapes that refuse, or ship a side their own gate fails, where a uniform reading would build: the split door's tier-3′ bar, tangent and pinch poses, and the curved-carrier lanes that read lines and circles only. CLEAVE measured 270.5 budget points against 30 and split on its priority seam (Ev, in chat, 2026-10-26).

## Measured on `splitdoor/hole-rim-tangent-germs` (2026-10-28)

On `origin/main` the row's pose no longer refuses as above: at ε = 1e-9
all 192 runs (the 96 azimuths, both normals) refuse
`Reduce(SectionGermsUnordered { Coincident })`, the cap's trace and the
bore wall's section leaving `P` along one tangent
(`a-plane-tangent-to-a-holes-rim-at-a-vertex-refuses-its-germs-as-coincident`).
With that branch's two changes (the germs ordered by how their sections
curve, and `conic_pairs` taking a touch point's exits first), `s = +1`
builds at all 96 azimuths and `s = −1` at 95, every side passing tier 3′
with no contact records; `split_across_a_revolve_seam.rs`
`a_plane_touching_a_bore_rim_splits_alike_under_either_normal` pins six
azimuths at the cap side's closed form, `tan t ∫∫ (0.5 − d·x) dA` over
the annulus where `d·x < 0.5`. The seam azimuth `a = 0` under `s = −1`
still refuses, `Join(Euler(Certification { StatedInterval { refused:
Forward(Decided(Zero(..))) } }))`: unmeasured.

The same refusal is what the hole-rim tangent poses produced in PR
5951's review under its mutant M3 (the join's touch-point runs read in
insertion order rather than by the alternation round them), so the
seam residue may be a crossing-run pairing case at the seam. Ordering
each run by the crossing before it (`join.rs` `touch_order`) does not
clear it: `a = 0`, `s = −1` still refuses alike.

## Closed (2026-10-28, PR 5951)

The row's two refusals (`NotAlternating` under one normal, the pinched side under the other) are
gone: 191 of 192 poses build, every side passing tier 3′. The one left, the seam azimuth under
`s = −1`, refuses for another cause and is its own row,
`a-plane-touching-a-bore-rim-at-its-seam-refuses-a-stated-interval-zero-in-the-join`.
