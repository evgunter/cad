FIX PASS 2 REPORT, part 1 of 3.

**Fix head: 68873a44b5ff8d17e9dd05ec8479ccf44ed0eb00.** The tree is clean, `git ls-remote` matches, and nothing is owed.

It contains:
- merge edef0cf9bd: origin/main 184e484ed2, PR 7052 and after;
- items A–J (05f1926985, 6b3a068546, 593f852519, 721c05555e, 043d0f9acd);
- merge 98ec7ad7a6: origin/main 362cd72fd2, 73 commits, clean, nothing in mesh or the props doors;
- the from-f64 fix (672e300e3a);
- the clippy fix (c541c7195a);
- the slow-set entry (68873a44b5).

**The PR 7052 merge.** One conflict, in props/curved.rs: main's `Curve3::Periodic` beside `Nurbs` in three arms. Those arms go through the kind table here, so the table now names `Periodic`, refused `Unimplemented` on every chart. Main's mesh `Periodic` arms (chords, memo, sizing) and its reach_probes and quadric_pair_sections changes are kept whole.

**Per item.**

**A. Door classes** (props/curved.rs):
- `IsoChart` now carries each surface's geometry and a `distance(p)` for each kind (cylinder |ρ−R|, cone |ρcosα−|h|sinα|, sphere |‖p−c‖−R|, torus |√((ρ−R)²+h²)−r|).
- `kind_refusal(e, band)`:
  - A kind that lies on such a surface keeps its class: ellipse on a cylinder or cone → `NotIsoRectangle`; spiric on a torus → `NotIsoRectangle`; spline or periodic → `Unimplemented`.
  - A kind that lies on no such surface (line or ellipse on a sphere or torus, spiric on a cylinder, cone or sphere) is decided by `props_carrier_on_surface`, `Premise::OnSurface`, over nine points of its span stepped in `T` (`step = (t1−t0)/T::from_f64(8.0)`):
    - decided miss → `OffSurface`;
    - in band → `Escalated`;
    - within band → the kind's class, as before.
  - Nine points are a lower bound on the distance, so a miss is sound.
  - NaN: `Real::max` propagates it, so a poisoned sample poisons the margin and the decision answers `Escalated`. That's documented, with a row.
- Cylinder, axis class zero: the circle's distance from the cylinder at four points a quarter turn apart (its radius alone was not enough: r = R in an axial plane answered "not a rim").
- Sphere miss path: `props_rim_fit` now reads `max(|√(‖w‖²+r²)−R|, ‖w×n‖)`, so a centre off its own normal is a miss.
- Rows in geom-brep `incidence_at_the_shape_door.rs`, one per site:
  - the cylinder circle with r = R;
  - the sphere circle with the right norm and off its normal;
  - chords of the sphere and torus, ellipses on each, a spiric on a cylinder;
  - the NaN line;
  - B's row below.
- Guard: the mesh suite at three ε and the earlier geom-brep run were green with no re-pin. No face that meshes on main refuses, and no ellipse edge on a sphere or torus face moved (the armed differential re-run is in part 3).

**B. `torus_classify`.** A missed fit's incidence is matched:
- `Ok` → `OffSurface`;
- `NotIsoRectangle` → the fit's verdict;
- anything else (undecided) → returned.
The comment is one statement now. Row: `a_rim_whose_incidence_the_band_cannot_decide_escalates_on_the_torus`, a coaxial r = 7 circle three zero-bands off axis. It answers `Escalated`; with the old `.is_ok()` it gave `NotIsoRectangle { props_rim_fit }`, red, which I checked.

**C. Branch door: cure kept.**
- `props_cone_apex` and `props_meridian_pole` read through `classify_as(.., PropsCheck::Inventory)`.
- A definite crossing anywhere in the loop refuses `NotOneChartBranch`. An undecided apex or pole answers `Escalated` only if no crossing is definite. My first version returned on the first escalation, which reddened `mesh12_saturated_span` (a definite pole beside an undecided one), so the order matters.
- Rows:
  - geom-brep `mesh11_arc_branch::an_apex_the_band_cannot_place_escalates_at_the_branch_door` (new, cone);
  - `mesh11r1_probes::r1_arcs_ending_past_a_pole_across_the_whole_band`. Its indeterminate rung pinned the old admission; it now pins `Escalated { props_meridian_pole }`.
- No mesh, sweep or tour row moved. The work/conearm row is closed.
- The `route_of` and `Rectangle` docs' "certify" now holds.
