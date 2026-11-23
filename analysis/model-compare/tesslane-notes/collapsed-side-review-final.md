REVIEW — PR 7517 (tesslane/collapsed-side) @ frozen head a87d047b5a. Reviewer lane, FULL.

**Verdict: NOT-MERGEABLE-AS-IS.** One MAJOR: the bit-identity tripwire is red, and behind it sits a design conflict that only Ev or a re-seam can clear. Everything measured about the mesh itself holds.

## Claims
1. HELD. On base (fa42c60bf7), with a probe calling trim_polygon/tessellate_trimmed on the wedge wall 3v1:
   - vertex 0 is held once at (0,0); prev (0.0333,1), next (0.0333,0), so the side is the diagonal chord;
   - the refusal bound is 0.016494 at δ 1e-2 (the PR's 0.01649) and 0.0052085 at 1e-3.
   - Not separately checked: that the bound is constant across retries. I saw only the final bound.
2. HELD, with a NOTE.
   - `budget` Deviation mode, 8 samples per edge, on 14 bodies (the PR's 9 plus corner 0.1 and 1.2, corner through 4 at v-deg 2, corner through 5 at v-deg 3, wedge 1.0) at δ 0.1/1e-2/1e-3/3e-4: 112 face-runs, max worst_ratio 0.50, no worst_cert over δ.
   - Dropped two-copy triangles: I instrumented the emit drop and sampled S over each dropped chart triangle against the 3-D segment it collapses to. 2184 drops on the class, max 0.061·δ at every δ, so no geometric gap. Those regions are certified by nothing, though: measured, not proved (NOTE below).
3. HELD.
   - All 14 bodies × 4 δ (down to 3e-4): check_mesh and check_winding pass.
   - Wedges equal 4·sin a to 9 digits.
   - Pinned volume against twins lifted 1e-6 and 1e-4: the difference is linear in the lift (3.76e-6, 3.76e-4), so the pinned volume is exact.
   - Curved v-deg 2/3 bodies converge monotonically with δ.
   - Not covered: a hinge at an END section (the class has hinges in the first, middle and last slab only).
4. HELD as rim-chord parity, which is not minimal. Mutants at 2× step and at a 4×δ target leave the class row green, so the certificate tolerates a coarser side. "Not coarser than the certificate needs" holds; "not finer than the face needs" does not. One piece (span ≤ h) inserts nothing and is sound. A very fine step hits ceil_count's cap, typed (by inspection).
5. HELD (no panic). Probe: the pinned vertex's mesh position moved 1/4/64 ulps before the chord pass, lift unchanged, tessellate_trimmed per wall:
   - one-corner wall: meshes at 0.1 (77 tris against 81), CertificateExceeded at 1e-2/1e-3;
   - v-deg-2 corner wall: CertificateExceeded at every δ.
   - The lift still turns at the side (topo `singular_at` decides Sides at band) while the polygon keeps one copy. The outcome is typed, never a panic, fold or unreachable.
   - The missing body-level row and filing ARE a finding (MINOR). Mutant M5 (1e-9 band) is caught only by the unit `holds` row.
6. HELD.
   - Per-patch digest (bits of each triangle's corner positions) hooked into tessellate_impl on both trees.
   - `mesh` suite: 54 115 patch lines, identical except the PR's own new row.
   - `sweep`, the 611 tests in the 67 modules that tessellate plus pinned_corners_build: 32 249 lines. Only two random-seed fuzz rows differed; re-run at CAD_FUZZ_SEED=0x1234, they are IDENTICAL.
7. HELD, but no row pins the BYTES. My digest of all 14 bodies × 4 δ is byte-identical at ε 1e-9, 1e-6 and 1e-12. The spec asked for the three ε, and the rows only pass/fail there: MINOR, owed.
8. HELD.
   - My 9 mutants, each red on at least one row: M1 sides never read, M2 no walk, M3 axis swapped, M5 1e-9 band, M6 +1 piece, M7 2× step, M8 closing pair skipped (mine), M9 half step (mine), M10 4·δ target (mine). No row is inert.
   - The class row does not pin the count: it stays green under M6, M7, M9 and M10.
   - The budget row runs on a hosted job: ci.yml `mesh-budget` runs `cargo test -p mesh --features budget` when mesh is in the change set, and nightly's dev-budget row runs it too. The plain cargo test there ignores nextest's default-filter, so the slow rows run as well.
9. Mostly true.
   - The filed corner issue is accurate: topo `SideWalk::Corner` exists at decision.rs:968/975.
   - It has no priority or cost (`work.py lint` ok).
   - The source issue is left `status: dispatched` while the body says "Closes".
   - §5 dispositions read true as far as I followed them. The blind spot (readers keyed on a repeated id) is backed only by the census passing.
10. NO TEST JOB HAS RUN. The run is now 45113927250 (it replaced 45112719065): "change filter" is queued and "prime the build cache" was skipped.
    - Locally: `mesh` whole with `--ignore-default-filter` at ε 1e-9, 1e-6 and 1e-12 is 538/538 green each.
    - Locally: sweep's 611 meshing tests have one red, bisector_nodes::the_near_node_poses_at_the_witness_band, main's known red, red on base too.
    - The lint gate below WILL be red.
11. See Spec deviations.

## Findings
- MAJOR, DEMONSTRATED: the bit-identity tripwire is red, and the decision behind it is a design conflict.
  - `bash scripts/gates/bit-identity-consumer.sh` exits 1 on `crates/mesh/src/trimmed.rs:3236` (`CollapsedSides::of_point`, which calls `one_stored_point`). ci.yml's lint job runs every gate.
  - The gate header and `geom-core/src/bit_identity.rs` docs say the channel is retired from production coincidence (D10/N6), that the listed collapsed-side readers "decide no coincidence between two cells", and to "add nothing without a DESIGN.md revision".
  - This reader compares a body VERTEX against a face net's side point: a coincidence between two cells. So the exact-identity ruling owes a DESIGN `[extensibility]` revision before Ev, or the mesh takes the side decision from elsewhere, e.g. the lift's own Sides read, which would also remove claim 5's disagreement.
- MINOR, DEMONSTRATED (M5 is caught by the unit row only): there is no body-level row or filed issue for a vertex within the band of a side but not bitwise on it, the case the dispatcher asked for. Today's behaviour is a typed refusal (claim 5).
- MINOR, BY-INSPECTION: no row pins bytes across the three ε (claim 7). No committed digest row shows that faces without a collapsed side do not move: the spec asked for "patch digests" and the PR gives only a scratch measurement.
- NOTE, measured: emit drops two-copy triangles before any certificate, so their chart regions are uncertified slivers. They measure ≤0.061·δ, but nothing proves it, and the SideWalk doc's soundness paragraph does not mention them. `trimmed.rs` ~1480 and the SideWalk doc.
- NOTE: the class row does not pin the walk count (claim 8); only the relation and pieces rows do.
- NOTE: the source issue's status is not updated to match "Closes"; the new issue has no priority.

## Spec deviations
- Reported:
  - the cause corrected (a diagonal chord, not a column chord);
  - the 3-section v-deg-2 body added;
  - `singular_row_shape` and `pole_side_columns` not applied to spline, with a disposition;
  - the corner case filed.
- SILENT:
  - "patch digests showing no face without a collapsed side moves" was delivered as a PR-body measurement, not a row;
  - "the three ε with no ε read" was delivered as pass/fail, not bytes;
  - the spec's class asked for "a collapsed side on each chart axis": the v axis is covered by unit rows only (the PR says no door builds one: true of loft, unverified elsewhere);
  - nothing tells the reader the exact-identity decision crosses the bit-identity retirement (the MAJOR).

## Style
Questions exercised: Q1, Q2, Q3, Q4, Q5, Q7, Q8 (partial: diff hunks, emit, flat_points, loop_polygon, one_location_per_branch, CdtPlane and cdt_point read whole; the 6 600-line file not end to end).
- sure — Q4: `bit_identity.rs:10-19,76-79` and the gate header claim the collapsed-side readers decide no coincidence between two cells. `trimmed.rs:3236` makes that false. The doc is not rotted: this is the code drifting from a ratified invariant.
- sure — Q1: two types named `SideWalk` for one idea in sibling crates: topo `pcurves/decision.rs:968` (an enum, Off/Along/Corner, banded) and mesh `trimmed.rs` (a struct of steps). The mesh also re-derives "which side is this vertex on" (`of_point`) after topo has already decided it at band. Two spellings of one decision, and they can disagree (claim 5).
- sure — Q7: `SideWalk.steps: HashMap<u32,(usize,f64)>` encodes the axis as 0/1 with `ChartAxis` in hand, an invariant held by convention where a type would do.
- likely — Q7: `face_polygon` has 9 parameters under `allow(too_many_arguments)` and packs `(surface,&row)` and `(&singular,&walk)` into tuples, laundering the argument count in `trim_polygon` and `face_polygon` alike.
- likely — Q2: the emit comment ("a spline chart's singular points are its collapsed sides, which bound its chart and hold no point past them") is offered as the asserted "singular half", but nothing checks it on the spline arm.
- sure — Q2/Q5: the `flatten_traversals` and `flat_points` docs still say "leaves from another `u`". The code compares (u,v), and the spline side is walked along v. The PR edited the neighbouring trim_polygon doc and left this.
- likely — Q2: the SideWalk doc says "every copy lies on the side, where the surface is the vertex". The inserted points interpolate BOTH coordinates between two pcurve-evaluated copies, so they sit on the side only if the copies share the fixed coordinate bitwise. One body's row checks that.
- unsure — `CollapsedSides::walk` drops a missing `shared[id]` silently (`filter_map ?`), and a vertex on two sides silently: a quiet skip in a fail-loud codebase.
- likely — Q1 (a class): the chord-pass test setup (positions/vids/FaceBounds/Routes::witness/compute_chords) is pasted again in `trimmed_collapsed_rows::chord_pass`. That makes about 14 copies of the vertex_points→vids loop in crates/mesh/src; look in trimmed.rs tests, curved.rs and chords.rs.
- likely — Q3: the relation row checks max(pieces) == the twin's strut chord count, both read from the same `face_target`/steps formula. It pins parity with the chord pass's implementation rather than the certificate contract, and the class row is green under a 2× or 4·δ walk.
- unsure — Q7: I would have taken the side's identity and axis from the lift's own joint decision rather than re-deciding it in mesh.

## Rubric
- idiom/structure: 3. Small, well-placed seam (face_polygon, a closure for the decision), but tuple-packed parameters, an axis as usize, and a second `SideWalk`.
- test quality: 3. Every ingredient is mutant-covered, but the class row does not pin the count, there is no byte or ε row, and there is no body-level near-side witness.
- doc/comment honesty: 3. Docs are accurate about the walk, but they claim soundness without the dropped slivers, and they leave bit_identity's and the gate's premise false.

## Executed vs read
- Executed:
  - the base probe (claim 1);
  - the head probes: class × 4 δ × 3 ε digest, twin volumes, ulps-off, budget multi-δ, sliver instrumentation;
  - 9 mutants;
  - digest differentials over mesh (534 tests) and sweep's meshing modules (611), plus a fixed-seed re-run;
  - `mesh` whole with the slow set at three ε;
  - the bit-identity gate;
  - `work.py lint`.
- Read only: the work files, ci.yml and nightly.yml budget and slow-set steps, topo `singular_at`, the PR description and check runs. Nothing was committed or pushed.
