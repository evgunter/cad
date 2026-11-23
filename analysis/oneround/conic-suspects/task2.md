# Task 2 — the eight not traced

`origin/main` at `dc263f855`. **The classifications are READ, not run** (a
delegated read-only pass over each function and its consumers, `file:line`
below; where one of my runs touches a row it says so). **None of the
eight reads as a firm suspect**, so none was owed a run; three were
touched by runs anyway.

| # (table row) | function | verdict | deciding consumer, and its meter | reached in a run? |
|---|---|---|---|---|
| 1 (23) | `boolean/join.rs::bool_connect` → `GermLane::EdgePlane.plane` | SUSPECT thin | `chord_join.rs::chart_island_winding` (3854-3870): `k = n·(origin − centre)/n_a − anchor` into `decide_reported(WINDING_PREDICATE, Margin::over_lever(2·R·area, length))`, bare. Also `wall_section` on the displaced plane (2466), whose mint gate was not read | not posed: needs a ring-lane island on a cylinder holder of mean width under 2·δ (2e-7 m at a = 1e9) |
| 2 (47) | `join.rs::germ_section_loops` → `SectionLoops::conic(axis_line)` | SUSPECT thin, no wrong pairing derived | `travel` (2298-2318): `bool_join_arc_ahead`, `_travel`, `_clear` on `turned_past`, bare; `off_conic_slack` is 2ε·cot ψ, no semi-axis. `bool_join_arc_facing` is covered by magnitude (≥ b) | — |
| 3 (48) | `join.rs::pk_section` → `PkSection::Loop(axis_line)` | as 2, reachable by construction | the same consumers | **yes: read in every traced boolean** (task 3: 204 / 855 `KernelLimit` refusals included, once each); every body built on it is right |
| 4 (46) | `census/depth_gap.rs::face_read` → `Around::Line(axis_line)` | N/A for the line (a proposal: a depth is sound about any line) | `clears` (202-208): `decide("census_backstop_depth_gap", Margin::of(lo − hi − pad))`, `pad` = 24 ε, no semi-axis | — |
| 5 (45) | `stands.rs::line_meetings` | N/A (seeds) | every candidate is re-read by `certified_in_face` → `point_in_face` (608-620) | — |
| 6 (42) | `pcurve_cache.rs::carrier_harmonic` → `Harmonic3.c` | SUSPECT weak (uncharged, conservative) | `envelope_verdict` (7524, `pcurve_envelope`, bare) over `d_c` (6451), the `Centre` terms, `focal_section_envelope` (8062); `cone_conic_incidence` (14021, `pcurve_cone_chart_nappe`, bare). At `f64` the cross-check `pcurve_map_residual` (8881-8888) reads the evaluator, so a false certificate would need δ ≤ 2 ε | **yes: run** (`chart_pcurve` + `PcurveCache::certify`, task 1): certifies to 1e6 / 1e7 (ε 1e-9) and 1e3 (1e-12), then refuses (`Escalated`, `ResidualExceeded`); never certifies a plane 5 ε or 30 ε off. **REFUSES** |
| 7 (49) | `step-import/on_built.rs::Conic::of` → `center` | SUSPECT thin, scale-limited | `bound_from` (403-441) → `file.agree_at(id, "step_edge_on_description", gap)` (`build.rs:1381`); the f64 centres entered as exact. An imported plane × cone edge is re-minted arc-held (`read_section` → `topo::section_near` → `plane_cone_section`) | **partly: run** (task 3's STEP rows): files to a = 6e3 import at ε 1e-9; larger are refused by `SingularitySideUndecided` before this reader matters (it needs δ > ε_in: a ≥ 1e7 at ε_in 1e-9) |
| 8 (63) | `blend/open/end_face.rs::CapSliver::of` → `center` | N/A for `center`: the cut-off's ellipse is `Curve3::ellipse` → `Ellipse::new` (`battery.rs:2956-2960`), centre-held, on a cylinder band only | rows 14 to 18 read the OTHER operand's centre (a stored end-face edge, which can be arc-held): `classify(.., RingClearance, Margin::of(margin))` (`surgery.rs:3012`), bare. They stay SUSPECT thin | not posed: needs a fillet on a body whose plane end face carries a far cone-section edge |

## One finding outside the eight

**On main an arc-held ellipse's cover is built about the derived centre**:
`geom/src/cover/readings.rs:155-157` is
`Cover::arc(&ellipse.frame(), ellipse.semi_axes(), window)`, and
`geom/src/cover.rs:352-357`'s `Arc` has no `on` field (the readers table's
row 40, "COVERED", describes PR 7510's branch, not main). Confirmed by
run: `cover_over@geom/src/cover/readings.rs:156` reads the derived centre
in every traced boolean, the `KernelLimit` refusals included (task 3). It
feeds `census_backstop_depth_gap` (item 4's row) bare against a 24 ε pad.
SUSPECT, not run: the tier-3 census's public entry was not traced.

## Not read (where a trail went cold)

The chord mint's certification gate and the zip's congruence check
(item 1); `germ_section` / `rides_carried` and the chain from the public
boolean doors to `bool_connect` (items 1 to 3); the census's public entry
and `face_cover`'s other constraints (item 4); `point_in_face` (item 5);
`Residual::bound`'s consumers and the projected lane's full check
(item 6); `section_of` / `section_closed` (item 7, skimmed); the chain
from `fillet_edges` to `ring_clearance_pass` (item 8).
