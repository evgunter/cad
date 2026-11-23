PR 7041 SECOND FULL REVIEW — frozen head 3494faec95 (merge base 6396c45978 = PR 7051's merge)

VERDICT: APPROVE-WITH-FIXES. No MAJOR. On 340 base/head probe rows, nothing that meshes on main moved: bytes, K stream on a run, and K stream on a memo hit. Fixes owed: the false memo-hit K docs; the door-class misclassing (demonstrated); a ruling on the D9 edit; the memo-path cost.

## Claims
1 HELD. My differential: one probe file compiled on both trees, 170 bodies × δ {5e-2, 1e-2}, giving mesh digest + check_mesh + K run + K memo hit. Bodies: ball less a slab in a vertical plane (x0 ∈ {0, 1e-10, 3e-9, 0.3}) and less a horizontal slab (z0 ∈ {0, 1e-10, 0.3, -0.6}), each tilted θ ∈ {0, 1e-13 … 2e-9, 2e-8, 1e-7, 1e-5, 1e-3}; pipe and cone cut near a rim at the same θ; donut less a tilted slab; 12 corpus bodies each posed 4 ways (oblique 0.7, oblique 1e-9, quarter turn, x 0.3); oblique-posed cut bodies. 302/302 non-witness rows are identical, meshing and refusing alike. Only the hand-built witnesses move (claim 4).
2 FALSIFIED IN PART (DEMONSTRATED). Stored data that lies on no such surface gets `NotIsoRectangle`, which now means "valid face, chart lane". My geom-brep probe on `require_iso_rectangle`:
 - circle in an axial plane with r = R on a cylinder → "cylinder boundary circle is not a rim" (no such circle lies on a cylinder);
 - sphere circle in an axial plane with |w|²+r² = R² but w not along its normal → `props_meridian_great` (the new miss path checks the norm, never the direction);
 - a Line on a sphere, and a Line on a torus → "…is not a circle".
 By reading: `IsoChart::kind_refusal` likewise classes an Ellipse on a sphere or torus `NotIsoRectangle`, and a spiric on a cylinder, cone or sphere `Unimplemented` (row 2). The torus rim path (`torus_classify`) answers `NotIsoRectangle` when `require_rim_incidence` ESCALATES (`.is_ok()` is false), so an undecided incidence is read as a decided "no rim". Held: a Villarceau circle stays "no rim"; a coaxial circle off the torus is `OffSurface`.
3 HELD with a caveat (DEMONSTRATED). Each text is true of its body. But a body with no stored images (the four Euler-built witnesses, unminted) now refuses `DomainUnread Gap` where main gave the doors' verdict (`props_rim_level`, `props_rim_axis_parallel`, `NotOneChartBranch`, `props_rim_side`). So that caller no longer learns the shape cause.
4 CORRECTED. The moved pairs are as stated: keyway and lens × 2 δ. My runs on both trees: minted keyway is the doors' `props_rim_level` on main; on head the U face meshes and face 2 refuses `TrimLoopOrientation`. That is a correction: the complement on an unbounded wall is not a bounded face, and the variant's text says so. Minted lens is `OuterOpen` on both trees. Half cap and bow tie, minted, keep main's refusal but their K streams differ (main e8f2…/f483…, head 8f44…/b732…): the lift's read is now recorded after the doors.
 Swapping the inputs to minted does hide that all four UNMINTED witnesses moved (claim 3). And the spec says "D9 goldens … pass unedited. A byte that moves there is a stop: report it, do not re-baseline". The PR re-baselined and edited the corpus without naming either as a spec deviation. Ruling owed.
5 HELD (grep). The doors are called only in `iso_rectangle_shape`, and every lane read goes through `Routes`. The domain read has two homes (`route_of` and the worker), joined by `Route::Trimmed(Option)` and an `unreachable!` (see Style).
6 HELD. `Rectangle(())`'s field is private to `curved`; the one other constructor is in `curved`'s test module; it has no Default. M3 reds 6, as the PR says.
7 HELD (DEMONSTRATED). Extra bodies, each through the six ops (∪, ∩, ∖, both orders), at δ 5e-2…2e-4 and ε 1e-9, 1e-6, 1e-12, read with `reach_probes::got` (check_mesh, volume, winding, sampled deviation, no area): notch touching one rim, both rims, a rim and the seam; two notches; a 2 mm notch; sphere wedge cornered on pole + seam; a notch through both poles. All mesh. Exceptions: the seam-cornered pipe notch refuses `CertificateExceeded` at every δ (the PR filed this class); at ε 1e-6 the thin notch's boolean refuses before meshing.
8 HELD. I re-ran mutants M1–M5, mesh suite whole each (M5 with geom-brep): 6 / 38 / 6 / 5 / 1 reds, matching the table. M5 also reddened a random-seed fuzz (`curve_pair::pose::window::oracle`, seed 0x5f7e…) that passed in my whole run.
 Mine: M6 lift check dropped → 7 reds. M7 Escalated sent to the chart lane → 1 red (`a_door_in_band_refuses_with_its_cause` alone). M8 harmonic widened to nonzero cos/sin terms (the early head's rule) → 0 mesh reds; red ONLY in step-import `split_iso_side` (1/329); my posed corpus moves no byte under it. M9 K splice on a memo hit → 1 red (`a_body_that_meshes_records_mains_k_stream`).
9 HELD. Props' passing paths add no decision. Mass properties: spans and frame states now read DEFECT instead of "cannot yet measure", argued at the split. Python tags are at the `MassPropsError` level, so no new tag. `cover-kind-match`: `props/curved.rs` holds 10 `.cover()` sites on both trees and the torus read is still one site. That is a duplicate spelling folded back, not a gate blind spot.
10 CORRECTED (DEMONSTRATED). The doors are read once per face per run, but serially (`Routes::of` runs before the parallel stage), and now on EVERY `tessellate_with` memo hit, where main skipped them with the lane. Warm-memo hit, base → head, over 3 runs: washer 86–138 → 185–188 µs, rounded prism 128–157 → 288–346, cone 43–64 → 110–163, donut 102–171 → 230–298, ball 73–117 → 130–170. The PR's cost section does not measure this path.
11 Hosted CI on 3494faec95, run 44034618006: all 13 jobs that ran are green; the interval oracle and cache prime were skipped. PR 7051 IS in the head. Mine: `nextest -p geom-brep -p topo --ignore-default-filter` gives 7035/7036, 151 skipped. The one red is topo `section_cert::square_search::the_square_wall_arm_agrees_with_the_traced_section`, random seed 0xfe9d87b3c0bdd953 (torus fold_half tangents); the diff touches nothing there and I did not replay it on base.
12 The three disclosed spec errors are accepted. Silent deviation: the D9 edit (claim 4). §5 sweep: topo's `trims` / `curved_rim` is a second table of non-iso kinds beside `IsoChart` (see Style Q1).
13 HELD by my A rows and B rows (θ down to 1e-13), identical. By reading, every structural route on main stays structural: the kind pre-pass decides nothing, and `is_non_iso_form` is a superset of main's sphere test.
14 HELD, except for claim 2.
15 HELD on 150 meshing bodies × 2 δ, run and hit, not only on nine.
16 HELD in geom-brep. But mesh's `route_rows::a_carrier_of_no_iso_kind_is_read_before_anything_in_band` stays green with the pre-pass removed (M5), despite its name and doc (MINOR).
17 HELD (M7).
18 HELD (claim 4).
19 HELD.
20 The body is true; the TITLE still asserts the refuted premise ("has no witness carrying chart images").
21 Two notes held. The branch-door escalation is not shown reachable (BY-INSPECTION). It breaks the spec's own "a route is never a default taken because a predicate could not answer", since `Shape::Rectangle` rides the door's `Ok`. Fine to file, but the route doc and `Rectangle`'s doc ("certify") overclaim until it is fixed.

## Findings
MINOR `crates/geom-brep/src/props/curved.rs` (`IsoChart::kind_refusal`, `cylinder_boundary`'s s-zero path, `sphere_boundary`'s `props_meridian_great` path): off-surface data gets the valid-face class — DEMONSTRATED (4 rows).
MINOR `curved.rs` `torus_classify`: an escalated rim incidence is read as decided "no rim" — BY-INSPECTION.
MINOR `crates/mesh/src/tessellate.rs` `tessellate_with` doc ("a hit's stream is the chord pass's alone") and `Routes`' doc ("a memo hit records nothing"): false, the fold splices `read` on a hit, and the PR body says the opposite — BY-INSPECTION, sure.
MINOR D9 corpus re-baselined against the spec's stop rule; the witness swap masks the unminted moves — DEMONSTRATED.
MINOR memo-hit path 1.5–2.5× slower, unmeasured in the PR — DEMONSTRATED.
MINOR `crates/mesh/tests/route_rows.rs` `a_carrier_of_no_iso_kind_is_read_before_anything_in_band` cannot go red on its subject — DEMONSTRATED (M5).
NOTE The oblique 1e-17-sine class is guarded only by step-import `split_iso_side` (M8). Escalated routing is guarded by one body (M7).
NOTE Refusing bodies' K streams change (the lift is read after the doors' decided verdict).
NOTE `work/tesslane/a-rectangle-the-walk-refuses-has-no-witness-the-chart-lane-can-read.md`: stale title.

## Style
- Q1 `tessellate.rs` `route_of` + worker (`Route::Trimmed(None)` → `face_domain` again; `unreachable!` in `run`): two homes of the domain read, and an invariant held by convention, not by type. sure.
- Q1 topo `props.rs` `trims` / `curved_rim` vs `IsoChart::reads` vs `Pcurve::is_non_iso_form`: three readings of "a non-iso boundary", which already disagree (a Line on a sphere). likely.
- Q5 `sizing.rs` `SizingTols::band` doc says "consumed by two sites", but `tessellate.rs:651` reads `tol.band` for `face_domain`. sure.
- Q5 `topo/src/validate.rs` `classify_mass_props`: the comment above the `P::Defect` arm describes `SpanUncertified`. sure.
- Q5 `props/mod.rs` `NotIsoRectangle` doc calls the plane arm "caller's misuse" while `refusal()` ranks it row 2 and `iso_rectangle_shape` would route it to the chart lane. likely.
- Q3 `geom-brep/tests/far_walls_of_many_arcs.rs` `early`: accepts `OffSurface | NotIsoRectangle | SpanUncertified` by name, so it cannot catch a class regression. likely.
- Q7 Running the doors serially before the chord pass, and on memo hits, is not how I'd do it: route in the parallel stage, or key the memo on the route's inputs. unsure.
- Q2 `torus_classify`'s two stacked comments ("Which, is decided on that path only." / "Read where an incidence is decided…"): the justification is longer than the closure it defends. likely.
- Q6 The branch-door escalation is filed and unscheduled, but it touches the route's own guarantee. likely.
Questions exercised: Q1, Q2, Q3, Q5, Q6, Q7, Q8 (read `tessellate.rs`'s routing, `iso_rectangle_shape`, the `IsoChart` pre-pass and `curved.rs`'s header whole). Q4: grep of cited premises only.

## Rubric
- idiom/structure 4: the token and the exhaustive match are good; the `Option` domain plus `unreachable!` lets it down.
- tests 3: strong mutant coverage, but one row cannot fail on its subject and two classes have a single guard.
- doc honesty 3: two memo-hit K docs are false, and the sizing doc is stale.

Executed: whole geom-brep + topo run; base/head differential (merge-base worktree); notch probes at 3 ε; 9 mutants; M8 differential plus step-import under M8; warm-memo timing ×3; geom-brep door-class probe. Read only: PR body and checks, the spec Unit section, the filings, D9, the props/topo/py diffs. Nothing committed or pushed; the tree is restored.
