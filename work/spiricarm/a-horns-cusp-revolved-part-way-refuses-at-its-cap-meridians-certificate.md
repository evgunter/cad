---
id: a-horns-cusp-revolved-part-way-refuses-at-its-cap-meridians-certificate
kind: issue
title: A horn's cusp revolved part way refuses at its cap meridians: the horn's two sheets are tangent at the cusp, so limb 2 reads only the distance from it over a stretch near it
status: open
opened: 2026-11-04
priority: P2
cost: H
---

(SPIRICARM, a residue of `a-football-revolved-part-way-refuses-at-its-cap-meridians-certificate`.)

## What

The needle tip (a horn reached at its cusp from one side) builds through
`revolve(Full)`; revolved `Partial(θ)`, it refuses
`RevolveError::Op` with `AnalyticRung3Refusal::KernelLimit` against the
`Spindle` operand (bound about `6.6e-8` at the default ε). So does the
horn's circle drawn as four quarter arcs, a face reaching the cusp from
each side. Pinned by `crates/sweep/tests/the_football_revolves.rs`
`a_horns_cusp_revolved_part_way_refuses_at_its_cap_meridians`.

A cap meets the horn along a meridian that ends at the cusp. On a lemon or
an apple the sheet's two sheets cross at an apex, and limb 2 reads a part
up to it by its certified levers (`geom-brep`'s
`projected::Meters::certified`), the part at the apex by its distance from
the apex (`Meters::apex_root`), which halving takes below the band. On the
horn the two sheets are tangent at the cusp: a point of the sheet at
distance `t` from it has `ρ ≈ t²/2r`, so the side check
`|m − r|·|R| ≤ r·ρ` (`Meters::over`) fails over a stretch of length about
`√(2r·d)` (`d` the `|m − r|` bound, near the net's width), and there the
reading is the distance from the cusp, which halving shrinks only as
that stretch allows; the rounds stop above the band. (The mechanism is
the arithmetic above, not a traced run.)

The refusal reads as the kernel's limit rather than a lane not written.

## Done when

The needle and the four-arc horn revolved part way build, pass tier 3 and
measure `θ/2π` of the full revolve's volume and sheet area plus their caps,
or refuse typed with a reason that names the cusp. A bound near the cusp
that reads the tangency (the sheets part as `t²`, not `t`) is the likely
lane; it needs a row that goes red when its load-bearing check is removed.
