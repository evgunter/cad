PR 7041 DELTA RE-REVIEW, fix head 68873a44b5

VERDICT: APPROVE. No MAJOR and no MINOR in the code. Four NOTEs, and one stale sentence in the PR description to correct.

## Claims
1 HELD (PR 7049 integration).
- Base/head differential against origin/main af9d8c67a0: 764 rows, mesh digest + check_mesh + K on a run and on a memo hit, δ 5e-2 and 1e-2.
- New split-side rows: 7 profiles (cylinder wall, frustum wall, cone through apex, frustum solid, slender wall, needle, double cone), each full and partial turn; the first ruling or every ruling split at 1, 2 and 5 points and at 1e-4, 1-1e-4 and 1e-6; placed obliquely, plus unplaced. 391 rows: 388 mesh, 2 BoundaryEar, 1 no-build, every one byte- or refusal-identical to main.
- Fact 4 is read only in the `Shape::Rectangle` arm, and only a non-split rectangle gets `Route::Curved`.
- Main's `splits_a_straight_side` also answered true only when `require_iso_rectangle_face` passed, which is the same two doors at the same band. So dropping that confirmation changes no answer; only the K count changes.
- Mutants, mesh `--profile ci`:
  - fact 4 off → 6 reds (`split_rulings` ×3, `curved::tests` ×2, K stream). The PR's 8 include slow-set rows.
  - split read before the doors → 2 (`a_split_side_is_not_read_before_the_doors_answer`, K stream).
2 HELD, for ruled bodies with no split side. I dumped the K sequences on 326c40c3f0, main and head:
- Head's run is 326c40c3f0's plus pure insertions: washer +8 `pcurve_chart_cut_side` +8 `chart_point_on_axis`; prism +16 / +16; cone +14 chart verdicts. Head's memo hit equals 326c40c3f0's exactly.
- On a split-side body (cone, one ruling split), head's stream is a strict sub-multiset of main's, reordered: 80 against 720 on a run, 0 against 640 on a hit. Main repeats the doors' and split reads roughly nine times.
- Nothing I read depends on main's count: replay compares within one build, and the memo key holds no K.
3 HELD (door classes).
- My four rows now answer `OffSurface`: axial-plane circle r=R on a cylinder and the skew sphere circle → `props_rim_fit`; a line 5 off a sphere and off a torus → `props_carrier_on_surface`.
- A line tangent to the unit sphere, half-span 1e-1 / 1e-3 → OffSurface; 1e-4 → Escalated; 3e-5 / 1e-5 (inside the band) → the old class.
- On a line, nine samples include both ends and the distance is convex, so the maximum is exact. On an ellipse or spiric a crossing between samples is possible in principle. That is no worse than before, since the old code asked nothing, and the PR gives no reason for nine beyond "a lower bound".
- The plane → `Defect` change reaches no user: every topo caller of the four doors discards the variant (`Err(_) => {}` or `.ok()`).
- `Periodic` → `Unimplemented` on every chart is consistent with Nurbs.
- No face that meshes on main refuses (differential).
- Mutants: nine samples removed → 2 geom-brep incidence rows red; torus back to `.is_ok()` → 1 (`a_rim_whose_incidence_the_band_cannot_decide_escalates_on_the_torus`). Neither reds a mesh row; both are pinned at their level.
4 HELD.
- Branch-door mutant (an undecided apex or pole → Ok) reds 2 geom-brep rows.
- Not reachable through the boolean: cone, cone_wedge and sphere_wedge less a cap at h ∈ {1e-5, 1e-7, 1e-8, 3e-9, 1e-9, 3e-10, 1e-10}. At 1e-8 and 3e-9 the boolean refuses to build. Below that the cone and cone wedge snap to the apex and mesh as main does; the sphere wedge doesn't build at any h ≤ 1e-8. Nothing moved.
- `route_of`'s doc ("A face they certify … on one chart branch") now holds.
5 HELD, with the reason partly true.
- My base/head witness rows match the D9 description: unminted, all four answer `DomainUnread` (Gap) on head against the doors' verdicts on main; minted, the keyway face meshes and its complement refuses `TrimLoopOrientation`, the lens refuses `OuterOpen` on both trees, and the half cap and bow tie give main's refusals.
- D9 constants: main's set plus the five new ones.
- The ruling's reason: `DomainUnread` is true of each body. "More useful" holds for the keyway only: minted, it meshes. The lens and the half cap, minted, still refuse; the bow tie, minted, gives the doors' verdict again. For three of four, minting leads back to the same verdict main gave.
- NOTE: the unminted lens and half cap now share one digest (`0x72fb…`), so D9 can no longer tell those two rows apart.
6 SOUND, but a symptom. By inspection: a route-time read error makes `Routes` mark the face Refused, and the chord pass skips Refused faces. Reading at the face's turn would let the chord pass refuse first on the missing image. So the same missing-image state refuses `DomainUnread` for a face the doors decided, and a chord-pass refusal for a `Trim::Structural` face. Two readers of image presence (NOTE).
7 HELD. Warm hits, main af9d8c67a0 → head (2 runs): washer 178–257 → 226–317 µs, prism 210–230 → 309–378, cone 87–151 → 137–152, ball and donut equal within noise. That is 1.0–1.6× against current main, in the description's range; PR 7049 made main's own hits slower. Filing P3 is defensible.
8 HELD.
- The renamed row is honest.
- widened-harmonic (my M8) → 1 mesh red (`an_obliquely_placed_rectangle_meshes_on_the_rectangle_lane_to_mains_bytes`).
- far_walls' `early` takes (class, name) pairs.
- My mutants on this head: K splice on a hit → 1 (K stream row); Escalated to the chart lane → 2; lift check dropped → 7.
- I did not re-run the implementer's other mutants one by one.
9 Fate of my first findings:
- MINORs:
  - door classes: fixed (verified above);
  - escalated torus incidence: fixed (mutant red);
  - memo-hit K docs: fixed (`tessellate_with`, `Routes`, `FaceRoute` read true);
  - D9 edit: fixed differently, by ruling;
  - memo cost: measured and filed;
  - un-reddable row: renamed, pin moved to geom-brep.
- NOTEs: the M8 class now has a mesh guard; Escalated routing now has 2 guards; the refusing K streams are stated in the description; the filing is retitled.
- Style:
  - two homes of the domain read: fixed (`read_domain`, `Trim`, `Ready`, no `Option`, no `unreachable!`);
  - three readings of non-iso: filed;
  - `SizingTols::band` doc: fixed (three consumers);
  - validate.rs comment: fixed;
  - plane misuse: now `Defect`;
  - far_walls: fixed;
  - serial doors: filed with the memo row;
  - branch door: cured.
  - Not addressed: the "Which, is decided…" comment style (cylinder path, props/curved.rs:2137). Harmless.
10 MOSTLY TRUE. Two corrections:
- (a) "## Measurement" still says a circle on a cylinder whose axis class reads zero "is off it only where its radius misses the cylinder's", which contradicts its own table row and the code (four points a quarter turn apart).
- (b) The K bullet is right for ruled bodies with no split side. It says nothing of split-side bodies, whose stream is main's minus the repeats, reordered.
- "Spec deviations" is accurate.
11 HELD. No mesh byte or refusal moved outside the four witnesses' 20 rows across all 764 rows, my old 340 included. K differs only on ruled bodies and the witnesses, as the description states.
12
- Hosted CI run 44150551989 on 68873a44b5: all 13 jobs that ran are green; the interval oracle and cache prime were skipped.
- Local whole runs on the head:
  - geom-brep + topo + step-import (`--ignore-default-filter`): 7385/7387. Both reds are random-seed fuzz rows, `ellipse_torus::…true_distance` (seed 0x45a8…) and `section_cert::square_search` (seed 0x9bb5…); the PR calls the first main's, and the second went red on my last head too.
  - mesh whole at ε 1e-9, 1e-6 and 1e-12: 517/517 at each.
- Viewer slow-set rows NOT run: disk is down to 5 GB free.

## Findings
NOTE PR description: the stale cylinder sentence; the K bullet's scope (claim 10). BY-INSPECTION.
NOTE d9_mesh_goldens: the lens and half-cap unminted digests are identical. DEMONSTRATED.
NOTE tessellate `read_domain` / chords: a missing image refuses differently by route (claim 6). BY-INSPECTION.
NOTE the props class mutants are guarded in geom-brep only (nine-sample, torus, branch door); no mesh row reds. DEMONSTRATED.

Executed: geom-brep, topo and step-import whole; mesh whole at 3 ε; the 764-row base/head differential; K dumps on three trees; 9 delta mutants; the door-class and tangent-line probes; warm-memo timing ×2 per tree.
Read only: the PR description and checks, the fix commits' diffs (tessellate route/Trim/Ready, walk `splits_a_straight_side` on both trees, props `kind_refusal`, branch door, torus), D9, the docs.
Nothing committed or pushed; the tree is back at 68873a44b5.
