# Task 3 — how far a wrong answer reaches: the boolean, split and STEP doors

Clean `origin/main` at `dc263f855` (worktree, own target directory), `f64`,
`CAD_TOLERANCE_EPS` 1e-9 and 1e-12, K = 10. Nothing here is read off the
code unless it says so.

## What decides

- **No public door builds a body on a section large enough for a measured
  reader to go wrong.** Largest cone-section semi-axis built, any door, any
  pose tried: **1.0e6 m at ε 1e-9, 1.0e3 m at ε 1e-12.** The measured-wrong
  readers go wrong from 1e8 (ε 1e-9) and 1e5 (ε 1e-12): two decades above,
  at both ε.
- **The boolean's own mint is no wider than `split`'s.** `union`,
  `intersect`, `subtract` and `split` build and refuse at exactly the same
  sizes in all 12 poses × 60 tilts.
- **Every body built is right**: 832 bodies, each section within 5e-16 m of
  the operands' cone and plane, each boolean's volume within 8e-16
  (relative) of the closed form. No wrong body.
- **The refusal that stops it is about the carrier, not the pose.** Past the
  largest built size the next refusals are `props_cone_ellipse_on_cone`
  escalating (one or two tilts wide) and then, for every larger section,
  `KernelLimit` on the minted edge's description. Its bound is
  5e-16 … 1e-15 × the semi-axis over six decades and three body scales.
- **The measured-wrong readers ARE asked about the section in the bodies
  that build** (`plane_conic` through `height`; `on_carrier` and
  `solid_contain::passes` through `Curve3::clearance`;
  `require_cone_ellipse`), at sizes below where they go wrong. In the
  booleans `KernelLimit` refuses, none of them is reached.
- **At ε 1e-12, no body with a section of 1e5 m or more is built by any
  door I found**: split, the three booleans, STEP import (including a file
  written at ε 1e-9 and read at 1e-12). Extrude and revolve mint no ellipse.

So the P0 stays a wrong answer at the geometry doors (`curve_pair::meet`,
the plane door, `on_carrier`) and reaches no built body on main today. What
keeps it so is one certificate's limit at about 2 ulp of the semi-axis
against ε, with a margin of two decades; it is not a re-read of any answer.

## The poses

A frustum (half-angle α, height S, top radius S/2, revolved about `y`)
against a box one face of which is the plane through `(0, S/2, 0)` with
normal `φ = (1 − gap)(π/2 − α)` off the axis; `gap` = {1, 1.5, 2, 3, 5,
7} × 10⁻² … 10⁻¹¹ (60 tilts), so the section's semi-axis runs 30 m to past
1e10 m while the arc inside the body stays metres long. α ∈ {0.46, 1.0,
1.3}; the pair as built, and both moved by one generic rigid motion
("rot"); S = 1, and S = 10, 100 at α = 1.0. Doors: `topo::union`,
`topo::intersect`, `topo::subtract` and `topo::splitting::split` on the
same plane. Source: `src/suspects_t3_boolean.rs` (an example of `sweep`).

A body cannot be larger: `topo::finish` refuses anything past the session
box (`OutsideSessionBox`, `SESSION_BOX_EXTENT` = 1e3 m; the S = 1e3 … 1e9
frustums and their boxes all refuse there). So a "fat" far section (b/a =
0.3, the earlier lane's ladder) cannot be an edge of a body beyond a ≈ 1e3;
the thin, near-parabolic section is the only way to a large semi-axis.

## Largest section built, and the refusal past it

All four doors agree in every row, so one column. "props" is
`…VolumeUncomputable{… Escalated … predicate: "props_cone_ellipse_on_cone"}`
(the finish's volume read, through `ResultInvalid` for a boolean and
`Finish(ResultNotAtRest)` for split); "KL" is
`Join(BooleanJoin(Euler(DescriptionUncertified{edge: Minted, error:
AnalyticRung3(KernelLimit{operand: Plane | Cone, …})})))`.

| ε | α | rot | largest `a` built | first refusal | every larger section |
|---|---|---|---|---|---|
| 1e-9 | 0.46 | no | 1.01e6 | props (1 tilt) | KL Plane |
| 1e-9 | 0.46 | yes | 3.02e5 | props (1) | KL Cone (2), then KL Plane |
| 1e-9 | 1.0 | no | 6.05e5 | props (1) | KL Cone (1), then KL Plane |
| 1e-9 | 1.0 | yes | 4.03e5 | KL Cone (9) | KL Plane |
| 1e-9 | 1.3 | no | 7.58e5 | props (2) | KL Plane |
| 1e-9 | 1.3 | yes | 3.79e5 | props (1) | KL Cone (3), then KL Plane |
| 1e-12 | 0.46 | no | 1.01e3 | KL Cone (2) | KL Plane |
| 1e-12 | 0.46 | yes | 3.02e2 | props (1) | KL Cone (2), then KL Plane |
| 1e-12 | 1.0 | no | 6.06e2 | props (1) | KL Cone (1), then KL Plane |
| 1e-12 | 1.0 | yes | 4.04e2 | KL Cone (9) | KL Plane |
| 1e-12 | 1.3 | no | 7.58e2 | props (2) | KL Cone (1), then KL Plane |
| 1e-12 | 1.3 | yes | 3.79e2 | props (1) | KL Cone (3), then KL Plane |
| 1e-9 | 1.0, S = 10 | no | 6.05e5 | props (1) | KL Plane |
| 1e-9 | 1.0, S = 100 | no | 6.05e5 | props (1) | KL Plane |
| 1e-12 | 1.0, S = 10 | no | 6.09e2 | props (1) | KL Cone (1), then KL Plane |
| 1e-12 | 1.0, S = 100 | no | none (smallest tried 9.0e2) | KL Cone (2) | KL Plane |

Nothing builds again past the first refusal in any row. At ε 1e-9 the
ladder's last tilts refuse at the mint itself (`pn_conic_type` escalated
for `gap` ≤ 7e-9, then `GermSectionOpenConic{conic: Parabola}`), as they
must. Full classes per file: `out/t3_summary.txt`.

## Is the refusal about the carrier

Yes, both of them, by measurement:

- `KernelLimit`'s `bound` (the certificate's upper bound on the minted
  edge's distance from the operand, decided against the band's zero) over
  the semi-axis extrapolated from the built rows (`a·gap` is constant to 1%):

  | ε | a | bound | bound / a |
  |---|---|---|---|
  | 1e-9 | 2.0e6 | 1.16e-9 | 5.7e-16 |
  | 1e-9 | 6.1e6 | 3.73e-9 | 6.1e-16 |
  | 1e-9 | 6.1e7 | 2.98e-8 | 4.9e-16 |
  | 1e-12 | 2.0e4 | 1.46e-11 | 7.2e-16 |
  | 1e-12 | 6.1e5 | 4.07e-10 | 6.7e-16 |
  | 1e-12 | 6.1e8 | 5.96e-7 | 9.8e-16 |
  | 1e-12 | 6.1e10 | 3.43e-5 | 5.6e-16 |

  It is the same at S = 1, 10, 100 (the limit sits at the same `a`, not the
  same tilt or the same arc length). So it refuses any cone-section edge
  whose semi-axis passes about ε / 6e-16: 1.7e6 at ε 1e-9, 1.7e3 at 1e-12.
  It is `geom_brep::edge_nurbs::upper_bound` (`ssi_hull_sup`), run on every
  minted edge by the Euler operator and on every edge by `topo::finish`
  and by STEP adoption (`Adoption{… DescriptionUncertified{edge: Keyed …
  KernelLimit}}` below), so no door I found goes round it. Read, not
  measured: it bounds the carrier's enclosing net, whose centre is the
  derived one (`carrier_net::harmonic`, row 21 of the readers table).
- `props_cone_ellipse_on_cone` is row 11's reader refusing: its margin is an
  enclosure `[0, 1.19e-9]` at a ≈ 8.6e5 (ε 1e-9), read against ε. It is the
  `VolumeUncomputable` the earlier lane met in `split` at a ≈ 1.1e6: the
  same predicate, the same door. It never answered `OffSurface` or a side
  here; it escalates in a window one or two tilts wide and `KernelLimit`
  takes over above it.

## Which readers are asked, and when

Main plus a tally only (`src/inst.patch`, `src/inst_suspects.rs`: the arc
arm of `Ellipse::frame`, the one place a derived centre is formed, records
its caller chain for a semi-axis ≥ 100 m; no behaviour changes), the three
booleans over the same 12 poses. Reads of the derived centre, summed over
the booleans of each
outcome (`out/t3_trace.txt` has the chains):

| reader of the derived centre | built (534 at 1e-9, 210 at 1e-12) | refused props (18, 15) | refused `KernelLimit` (204, 855) |
|---|---|---|---|
| `curve_face::pose::plane_conic` through `curve_pair::pose::height` (**measured wrong**, the plane door) | 8310, 1830 | 360, 280 | **0, 0** |
| `Foot::on_carrier` through `Curve3::clearance` (**measured wrong**) | 6642, 1458 | 288, 228 | **0, 0** |
| `solid_contain::curved_edges::passes` through `clearance` (firm suspect, row 8) | 3876, 852 | 168, 132 | **0, 0** |
| `props::curved::require_cone_ellipse` (firm suspect, row 11) | 3876, 852 | 168, 132 | **0, 0** |
| `pcurve_cache::carrier_harmonic`, `derive_class_zero`, `anchored` (rows 42, 41, 19) | 75848 / 40044 / 19608 at 1e-9 | asked | 0 |
| `cover_over` (`Cover::arc` about the derived centre) | asked | asked | asked |
| `carrier_net::harmonic` (row 21; the certificate's own net) | asked | asked | asked |
| `boolean::join::pk_section` (row 48), `chord_join::chord_spec` `in_plane`, `Curve3::seamed_near` | asked | asked | asked |

Not asked at all in these booleans: `curve_pair`'s `line_conic`,
`conic_conic`, `level_on`, `coplanar_ellipses`, `Window::*`;
`section_loops::line_clears_conic`, `conics_clear`;
`replace_face::cone_v_range`; `on_surface::plane`. (A plain cone against a
plane has one section loop and no second conic; those need other poses.)

So the answer to "is any measured-wrong or firm-suspect reader asked about
the section before a certificate refuses the body": **in the bodies that
build and in the props-refused ones, yes, four of them, at a ≤ 1.0e6
(ε 1e-9) and ≤ 1.0e3 (ε 1e-12); in the `KernelLimit` refusals, no: the
edge is refused at its mint, before any of the four reads it.** The four
were not wrong at those sizes as far as the bodies show (next section);
the earlier lane's ladder has the plane door right to 1e7 / 1e6 and
`on_carrier` right or refusing to 1e7 / 1e4. Task 1 re-measures both on the
thin shape at these sizes.

## Are the built bodies right

`oracle_t3.py` (60-digit `mpmath`, no kernel code), every built body of the
S = 1 matrix, 624 from the booleans and 208 from split:

- each section ellipse as the curve its stored numbers state (anchor, axis,
  seam, semi-axes, phase; `v = axis × seam` exactly), at its span's ends
  and two points between, against the operand frustum's stored cone and
  the result's stored cutting plane: worst **4.8e-16 m off the cone,
  3.6e-16 m off the plane**, at every size to 1.0e6;
- each boolean's volume (`topo::mass_properties`) against the closed form
  of frustum ∩ half-space integrated in `mpmath` (box and frustum volumes
  added for union and subtract): worst relative miss **8.1e-16**;
- face counts constant along each ladder (5 / 3 / 8 at α = 1.0).

No row past the band; `out/oracle_t3.txt`. Split's halves were judged on
the section only (no volume asked).

## ε 1e-12 and the STEP door

`src/suspects_t3_step.rs` (an example of `step-import`): the α = 1.0
intersect, exported by `step_export::step_string` in a process at ε_w and
imported by `step_import::import_step` in a process at ε_r (default
options, so ε_in is the file's).

| file (written at) | section `a` | read at 1e-9 | read at 1e-12 |
|---|---|---|---|
| 1e-9 | 6.06e2 | imported, volume to 2e-15 | refused: `TierInvalid{CensusEscalated … "curve_pair_pose_one_curve"}`, margin 2.2e-12 |
| 1e-9 | 6.05e3 | imported | refused: `Adoption{… DescriptionUncertified{edge: Keyed, KernelLimit{operand: Plane, bound: 4.5e-12}}}` |
| 1e-9 | 6.05e4 | refused: `SingularitySideUndecided{side: Straddle}` | the same |
| 1e-9 | 6.05e5 | refused: the same | the same |
| 1e-6 | 6.05e4 | refused: the same | the same |

A coarser writer does not help: at ε 1e-6 the mint itself stops at
`gap` ≈ 1e-6 (`pn_conic_type`), so the largest section written at 1e-6 is
6e4. An imported section is arc-held again (the import re-mints it), so
the same certificate meets it. I did not hand-write a file with a larger
ELLIPSE; `SingularitySideUndecided` is not about the carrier's size and I
did not trace it, so **a hand-written file is the one door here I have not
shown closed by the carrier's own limit** (at a ≥ 6e4 it was refused, by
something else).

## Not done

- `factored_crossing` on its own: the booleans above reach `plane_conic`
  only through the built sizes; I did not build a pose where a second
  plane crosses the far section's edge inside a boolean (the earlier
  lane's two-split probe is the nearest, and builds rightly to 6e4).
- `Interval`: every run is `f64`.
- A cylinder's section is centre-held (`Ellipse::new`), so it is outside
  the class; not run.
