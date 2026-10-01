---
id: no-approx-faced-body-is-both-movable-and-valid
kind: issue
title: an Approx face has no cache route with straight carriers, and the body whose chart does have one cannot be moved - so nothing weighs, meshes or exports
status: open
opened: 2026-09-04
priority: P0
cost: H
---


Found while writing SHELL-2's acceptance rows (`docs/SHELL-2-SPEC.md`
§3), whose rows 1, 2, 3 and 6 all assume the OFF-C consumer suite's
`prism_with_approx_walls` body can be moved by `topo::transform_rigid`.
It cannot. A box with an `Approx` cap CAN be moved and is
description-clean, but its chart mints no pcurve caches, and three
doors want those caches. The transform door's `Approx` arm itself is
fine — what is thin is what an `Approx`-faced body can then be used
for.

## The walls, measured

**1. The lofted prism cannot move at all, and never could.** Its four
vertical wall seams carry `Curve3::Nurbs`, which
`crates/topo/src/transform.rs`'s `map_carrier` refuses
(`TransformError::NurbsPlaceholder`) whatever any face carries.
`transform_rigid(&prism(), &translation, witness)` refuses on the
UNCONVERTED prism too, so this is not about the `Approx` surgery. That
arm is `transform-rigid-refuses-described-nurbs` (issue record 1346),
already filed and still open.

**2. The `Approx` face's boundary can be re-described, but its chart
mints no caches.** *(Corrected 2026-09-04 from the first filing, which
said the re-description itself was impossible and quoted the wrong
refusal for it. Both halves were wrong; what follows is measured.)*

`set_face_surface(face, FaceSurface::New(...))` mints a fresh key, so
every description naming the old surface goes stale. Two
re-descriptions are available and they do NOT fail the same way:

- remapping to `Intersection { approx, wall }` refuses at the attach
  gate — "a Nurbs described surface, or a Nurbs carrier under a
  conventional description, cannot be certified in this build". That is
  right: an approximating surface's implicit layer is poison, so an
  `Intersection` residual has nothing to measure against;
- **`Chart` + `Pcurve::IsoLine` IS accepted.** The cap's chart is
  `(u, v) ↦ (2u, 2v, z)`, each straight cap edge's image is its own
  carrier read in chart coordinates, and `set_edge_curve` certifies all
  four. `crates/sweep/tests/common/approx.rs::box_with_approx_cap` does
  exactly this, so the body is `DescriptionNotAdjacent`-free and moves.

What is left is the CACHES. `mint_pcurves` refuses the two cap edges
that hold `u` constant while `v` traverses — the iso lane's SEAM class,
`crates/geom-brep/src/pcurve_cache.rs:3775`,
`IsoUnsupported { what: "a seam-class iso line over a non-spline
carrier — no construction mints one" }` — because its
control-difference hull compares the carrier against the chart's own
boundary ROW and so needs the carrier in that spline space. The CAP
class beside it (`v` constant, `u` traversing) is the one that admits a
`Curve3::Line`, and a quadrilateral chart boundary always has two edges
of each.

**3. Three doors then want those caches, and refuse without them.** All
three measured on the moved fixture and pinned by
`verbs_offc_consumer::the_walls_a_placed_approx_capped_part_still_meets`:

- **tier 3 check 7** reports `VolumeUncomputable { QuadratureUnsupported }`
  — the ONE finding on this body, either side of a rigid map;
- **`topo::mass_properties`** refuses the same way, at the face;
- **`mesh::tessellate`** refuses: "NURBS-face half-edge carries no
  stored pcurve cache — caches mint at loft/sweep assembly and STEP
  adoption; without one the chord schedule has no certified UV step
  bound".

**4. STEP export refuses the kind outright**, cache or no cache:
`StepExportError::UnsupportedSurface { kind: "approximating surface" }`.
The writer has no printer for one — `OFFSET_SURFACE` is the AP203/AP214
entity it would need — so a user who places an `Approx`-capped part
cannot export it at all. That is a separate gap from the cache wall and
belongs to whoever owns the STEP writer's surface table.

## What it costs

- SHELL-2 §3.2's mass-properties invariance and §3.6's "every
  half-edge of a mapped `Approx` face carries a re-derived cache" are
  unassertable. The rows that landed instead read the DIFFERENCE either
  side of the map (same props verdict, same pcurve verdict), which is
  the honest claim available. §3.1's tier-3 claim landed as a
  finding-SET comparison against a one-finding baseline.
- A user who places an `Approx`-capped part can neither weigh it, mesh
  it, nor export it.
- More generally: an `Approx` face is reachable today only on a lofted
  body, and a lofted body is immovable. The first consumer that wants
  to place a shelled or offset part will meet both walls at once.

## What would lift it

Either leg alone is enough for SHELL-2's rows:

- leg 1 (`transform-rigid-refuses-described-nurbs`) would make the OFF-C
  prism movable, and its `Approx` walls already carry iso caches;
- leg 2 would need the seam class to admit a straight carrier — the
  hull argument would have to be made against something other than the
  chart's boundary row. That one lift unblocks tier 3 check 7, mass
  properties and tessellation together, since all three want the same
  caches.

The STEP gap (leg 4) is independent of both and blocks export whatever
either does.

Leg 1 is the cheaper of the two and is already scheduled.

**Update at SHELL-2's merge (2026-09-04):** leg 1 lifted on main
meanwhile — FIX's transform unit (`transform_rigid: gate on
is_placeholder, map described nets`) maps a DESCRIBED `Curve3::Nurbs`
carrier and refuses only the placeholder, so the OFF-C loft can now
move. The three OFF-C rows marked "decorative until 1346 lifts" can
be strengthened on the loft; the cache walls (legs 2–3) and the STEP
printer stand. Not done at merge — Ev asked the session to close out.

## A second measurement of the same class (TRIM-2 PR-1, 2026-09-13)

`work/trim/general-pcurve-face-props-and-tess-refuse.md` (#1179) claimed
`replace_face_offset` as one of its frontiers. It is not, and the
measurement belongs here rather than in a second row.

Fixture: the P-2 body's degree-2 re-widening
(`sweep/tests/m8_4_intersection_iso.rs::degree_two_body`) at scale
1/1024 — a described NURBS wall carrying a `Pcurve::General` chart image
on an interior column — and, as the control, the ORACLE prism's own
bowed wall, which carries no `General` anywhere. `f64` lane,
`replace_face_offset(bowed, scale/16)`:

| ε | the `General`-faced wall | the oracle prism's bowed wall |
| --- | --- | --- |
| 1e-6 | `FittedBoundaryUnsupported { what: "a chart image of a neighbour's chart" }` | the same |
| 1e-9 | the same | the same |
| 1e-12 | `Fit { BudgetExhausted { budget 6, grid (40, 27), achieved 2.8649835504510487e-10, tolerance 1e-12 } }` | `FittedBoundaryUnsupported` |

**Read:** the refusal is the fitted offset's own boundary rule —
`replace_face.rs`'s `matches!(new_surface, Surface::Approx(_))` arm —
and it fires on a described NURBS face whether or not the face's loop
carries a `General` image. That is this row's class: an `Approx` face's
boundary has no route. At the tight cell the widened body reaches the
FIT first and exhausts its budget at 2.86e-10 against a 1e-12 target,
which is the same class one door earlier.

TRIM-2 PR-1 records it (row `a_degree_two_widening_measures_against_the_oracle`'s
E3 half, which prints both and asserts only that both refuse — pinning
a variant there would pin SHELL's frontier from TRIM's slate) and
corrects #1179's offset claim. Nothing is scheduled here by this note.

## Legs 2–3 lift on a measurement (TESS orchestrator, 2026-09-22)

The `Approx`-capped box meshes, weighs and passes tier-3 check 7 once
`topo::mint_pcurves` runs — the LINE-carrier limb of the seam class
(EXCH's PR 1798, 2026-09-17) mints the cap's straight-carrier caches.
Measured at six (d, ε) rows on `tess/approx-face-survey` (`d423d1b46`).
Leg 4 (STEP export refuses the kind whatever the caches) stands. Also
found there: `replace_face.rs` asserts `EdgeDescriptionSpec::iso` is
the only door minting a chart `IsoLine` description and that it fixes
`u`; `EdgeDescriptionSpec::chart_image` is a second door and
`box_with_approx_cap` uses it for two u-moving images.
