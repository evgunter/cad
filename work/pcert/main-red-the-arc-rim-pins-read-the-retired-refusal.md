---
id: main-red-the-arc-rim-pins-read-the-retired-refusal
kind: issue
title: Main red since PR 6484: dm1's arc-rim pins read the retired 'not an arc rim' refusal, and the lofted arc wall's Zero count went 10 to 18
status: closed
closed: 2026-11-04
opened: 2026-11-04
priority: P1
cost: M
---


Filed by the ONEDOOR main-red lane. Four slow-set rows are red on main
from PR 6484's merge (`bc927bc4b3`, PCERT E5) on: green at its first
parent `142bc8e616`, red at the merge and at the PR's own head
`e0b89be2ce` (so no concurrent merge is involved), and still red at
`aef1e58929`. All four are in `.config/nextest.toml`'s slow set, and
6484 edited none of their files, so the per-PR gate never ran them;
no nightly has run on a main that holds 6484 yet.

## What

**1. The arc-rim pick's refusal changed, and the dm1 pins read the old
words.** E5 retired the arc-rim arm's
"no boundary row of this chart … — not an arc rim of this chart" refusal:
`nurbs_iso_derive`'s `arc_rim` now answers a pick that shows no row, and
a row certificate that escalates or refuses at its residual, as
`PcurveCertifyError::NoGeneralLane { gap: LaneGap::Unwritten }`
(`crates/topo/src/pcurves.rs:1323`, `:1334`). `dm1-id-214.stp`'s
half-edge `HalfEdgeKey(15v1)` (a circle on the l-bracket wall `#382`)
now refuses as

    pcurve certification: the circle carrier is on the nurbs chart but
    is none of its own curves, so no closed form images it, and its
    general image is not written yet for this pair. There is no way
    through yet

and three rows still want "not an arc rim of this chart":

- `step-import::all r1_dm1_probe::dm1_no_longer_refuses_at_the_instancing_gate`
  (`crates/step-import/tests/r1_dm1_probe.rs:134`);
- `step-import::all r1_dm1_probe::the_l_bracket_alone_adopts_its_reversed_slit`
  (`crates/step-import/tests/r1_dm1_probe.rs:326`);
- `step-import::all tier_gate::every_corpus_import_passes_the_shared_gate`
  (`crates/step-import/tests/tier_gate.rs:342`–`:344`, `:462`
  `ARC_RIM_NOT_HELD`).

`crates/step-import/tests/wild.rs:91` names the same words; its dm1 row
is skipped there, so it is stale rather than red.

**2. The lofted arc wall's rim checks answer more Zeros.**
`sweep::all a_loft_computes_its_walls_in_t::a_lofted_arc_wall_is_a_function_of_the_sections_bulge`
collects 18 symbolic `pcurve_iso_boundary` Zeros where it pins 10
(`crates/sweep/tests/a_loft_computes_its_walls_in_t.rs:116`–`:118`).
All 18 are proven, so the property the row states holds; what moved is
the count. E5's `arc_rim` now certifies the picked row itself
(`crates/topo/src/pcurves.rs:1334`) before the mint's own
certification, which would add the extra iso-boundary checks.

## The question for PCERT

Whether each new reading is the right one. For dm1: is "not yet, no
general lane" the right ending for a circle the pick shows on no row,
or does it hide a rim the row does hold (the comment at
`crates/step-import/tests/wild.rs:85`–`:90` says the rows state the
circle with unnormalized weights)? If it is right, re-pin the three
rows and `wild.rs:91` to the new words. For the loft: is the second
certification of the same row intended, or a repeated read the mint
should reuse? If intended, the pin becomes 18 with the reason.

## Reproduce

    cargo nextest run -p step-import -p sweep --no-fail-fast -E \
      'test(r1_dm1_probe) | test(=tier_gate::every_corpus_import_passes_the_shared_gate) |
       test(a_lofted_arc_wall_is_a_function_of_the_sections_bulge)'

## Closed (2026-11-04)

Neither new reading was right, so neither pin was re-pinned to it.

**dm1: the pick missed a rim the row holds.** Measured on dm1's own
bits, the sub-patch of wall `#382` that `HalfEdgeKey(15v1)` bounds has
the row at `v₁` with weights `(1, 1, 2)` on `u ∈ [0, 0.27027…]`. That
row holds the quarter rim exactly: the radial residual is zero in exact
rationals over 101 samples. At `u = ½` it sits at 53.13°, where the
standard form sits at 45°. The chart parameter is the standard one
through `u = λs/(λs + 1 − s)`, `λ = √(w₀/w₂)` (≤ 3.4e-16 in f64). The
pick read `w₁/w₀ = 1` as "no turn" and answered `Ok(None)`. Incidence
read not-off, so the arm ended "not yet, no general lane".

The fix: `Pcurve::IsoArc` carries the row's end weights (`ends`, empty
for a row at one end weight, so the loft's symbolic path is unchanged).
Its map applies each sub-arc's Möbius change. The certificate compares
`ends` to the row on the bits and decides each sub-arc's
`wₖ/√(eₖ·eₖ₊₁) = cos(h/2)`. The pick's images carry the row's end
weights, and the measured step reads the turn at unit end weights. The
mesher's chord rate is scaled by `max(λ, 1/λ)`.

The l-bracket now imports (census `(1, 1, 10, 21, 14)`). At ε 1e-9 its
volume enclosure `[3.43388e-6, 3.43846e-6]` holds an independent
admesh-clean mesh volume of `3.43577e-6` (δ 1e-5). At ε 1e-12 the
enclosure is `3.4361170e-6 ± 5e-13`, and the mesh is off it by its
chords, 3.4e-10, inside δ times the area (1.7e-8). The row pins that
bound at every ε. Full dm1 refuses one door later, at the
aggregate gate, on the instances' touches, and is filed on importgate
(`dm1s-touching-instances-refuse-at-the-aggregate-gate-unrecorded`). The
three rows and `wild.rs`'s fragment are pinned there. The row
`imported_chart_arc_rim::a_row_at_unequal_end_weights_holds_its_arc_rim`
is red under each of five mutants: the Möbius map off, the `ends` bits
unchecked, the turn read unnormalized, one margin for a skewed row, and
the pick dropping `ends`. isotail's
`dm1-arc-rim-pick-reads-no-row-on-an-unnormalized-rational-span` is
closed by the same fix.

The same fix reaches a second case. A degree-1 loft whose narrow
interior arc reaches its row refined, at uniform sub-arcs with end
weights `1, (1 + w)/2, 1`, now lofts. Its pin
`degree1_loft_slabs::an_interior_arc_refined_by_its_neighbours_refuses_typed`
became `…_lofts`: every case builds and is at rest, and the first
meshes to its integrated volume. A line rim on such a creased wall does
not mesh, which predates this change (`[0, 1, 0]` meets it with no
refined row). It is filed as
`work/tess/a-line-rim-on-a-creased-loft-wall-does-not-mesh.md`, and
isotail's `a-loft-whose-arc-bulge-varies-per-section-refuses-its-arc-rim`
carries the note.

**The loft: a repeated read.** `arc_rim` certified the picked row, and
the mint's pass 2 (`certify_walked`) certified the same image over the
same span again: the walk passes a derived image through unchanged.
The derivation now hands its certificate to pass 2 (`Derivation`,
`Walked::certified`), and the count is 10 again. At main `143d99ec29`
it is 18; with the reuse it is 10.

