RING-PLACEMENT fix pass. Pushed head a9618dd72c, after merging origin/main (which brought in PR 6941).
    
    ITEMS
    1. TAKEN. The roster gets a door `RING_PLACE` (prefix `ring_place_`, error `pcurves::RingPlacementError`) and carriers `Const("RING_PLACE_ARM")` / `Const("RING_PLACE_CLEAR")`. `recourse_roster::every_decided_name_has_its_door_and_every_site_is_read` now names only the three `ring_path` indirect sites (`ring_path.rs: spline_crossings`, `ring_path/spline_wall.rs: paths`, `.../spline_wall.rs: reach`). Those are main's: this branch's `ring_path*`, roster and test-utils files are identical to origin/main, so main's nightly is red on them. Not fixed here; it's your call.
    2. TAKEN. A twin-sheet ring answers `Undecided(None)`. Pinned by `a_sphere_ring_on_the_twin_sheet_answers_undecided`. `Undecided`'s doc names the twin case, and the PR body is corrected.
    3. TAKEN, adapted. `Unread` and `ring_decks` no longer exist; `SplineChart` replaced them. The `SHIFTS`, `PIECE_DEPTH`, `PIECE_VISITS` and `HALVINGS` docs now say each is a work bound and what it answers (`Copies` / `Near` → `Undecided`).
    4. TAKEN. The module doc names the liveness hole. Filed `work/tesslane/an-analytic-outer-loops-whole-span-boxes-never-tighten-under-the-ring-reader` (P2), which lists what each form's tightening box would take and points at unit 1's owed span boxes.
    5. TAKEN. `pcurves::{SHIFTS, PIECE_DEPTH, PIECE_VISITS, HALVINGS, WINDOW_SHARE}` are the one home. `spline_region` imports them (`SHIFTS as COPIES`, the `0..HALVINGS` loop, `/ WINDOW_SHARE`).
    6. TAKEN, doc only. The module doc says there are two readers today and names the unit that makes them one. The reviewer's point and the parity-reader sweep class are added to the spline-face unit.
    7. TAKEN. A dated section in `charttail/azimuth-inf-arm-has-two-homes` covers the third assembly and the pole disagreement. The claim "adds no third spelling" is dropped.
    8. TAKEN. Rows added:
       - `Gap` at the door, in mesh `the_unminted_witnesses_store_no_lift_and_mint_one`.
       - The reviewer's adversarial chains as asserting rows: `the_analytic_reader_reads_notches_vertices_slivers_poles_and_apexes` and `a_torus_places_a_ring_in_both_channels`.
       - Mutant check, run locally: counting any positive raw gap as clear reds two rows.
       - The second-In row's doc says only a scripted closure reaches it.
       Unreachable from a body:
       - `RingOpen`: a ring that winds needs an outer loop that winds, and the door answers `OuterOpen` first.
       - `SplineChart`: no at-rest body has a ring on a spline face. Sweep's lofted-wall booleans close one mid-operation, then refuse at containment.
       - `NoShift`/`Shifts`/`Copies`/`Undecided`: pinned at the rule and the reader, since no measured body reaches them.
    9. TAKEN. `whole(m)` is a checked f64→i32 conversion; a count with no i32 answers `Copies`. The window's latitude count in `Outer::side` uses it too, and answers `Near` there.
    10. TAKEN. A ring whose deck is the identity keeps its first read; only a moved ring is re-read through `read_from`.
    11. TAKEN. `PlacedRing::lift` and `Lift::read_from` say the moved lift is rounded afresh from the stored row, not the read image plus k·p bit for bit. The spline arm no longer hands out a lift. `Outer::side`'s isometry argument is cut to one sentence.
    12. TAKEN. The README row is present tense, with no work id.
    13. TAKEN, as two rows by owner:
       - `charttail/the-interior-witness-rung-reads-a-ring-where-it-is-stored-not-in-its-outer-loops-sheet` (P3);
       - `tesslane/the-torus-chart-window-abandons-a-face-that-holds-a-ring` (P3; cover.rs is not charttail's ground).
       Second pass: `cell_region` and `station_region` read the face in model space and draw no chart polygon. `validate.rs`' ray parity is the point-in-face door's model-space cast. Neither lifts loops into one chart, so neither is in the class.
    
    JUDGEMENT
    - `Placed`: removed. With `ring_decks` gone, the decks and the `(LoopKey, RingMiss)` detail had no reader. `Trim::of` and `with_rings` return `Option` exactly as on main, so the spline reader's signatures are unchanged. The rows test is back to main's text and still detects the shift: the hole reads Out only once moved.
    - `RingMiss`: kept. It is the rule's answer before any ring key exists, so the engine can read the rule without a door. `RingMiss::of` is the door's one mapping.
    - `PlacedRing`: now has accessors. A `PlacedRing` asserts a decided placement, so only the door builds it, as with `FaceLift` and `Lift`.
    
    DEPARTURES
    None beyond 3 (no `Unread` left to document) and 13 (two rows, not one).
    
    LINES CHANGED (last commit)
    code +292/−? across pcurves_rings.rs (+390/−79 including tests), spline_region.rs (−88/+?, net smaller), pcurves.rs, roster +9; tests in pcurves_rings (~250 lines) and lift_probes (+17); work/ +104 across 5 files.
    
    RAN
    - `recourse_roster::every_decided_name_has_its_door_and_every_site_is_read`: only main's three ring_path sites remain.
    - `scripts/run-gates.sh`: rc 0.
    - `clippy -p geom-brep -p topo -p mesh --all-targets --all-features -D warnings`: clean.
    - topo `pcurves::rings | spline_region | lift`: 60/60 pass.
    - mesh `lift_probes::`: 7/7 pass.
    - sweep `test(lofted_wall) | test(lofted_walls)`: 7/7 pass, including both ringed-spline rows.
    - `work.py lint`: ok.
    
    PR BODY
    =====
    ## Measurement
    `Trim::with_rings` (`topo::boolean::spline_region`) places a ring by its chain's first point:
    - On each closed channel it tries the whole numbers of periods `floor((r0−x)/p) ..= ceil((r1−x)/p)` over the outer chain's hull, at most 9 per channel.
    - It reads each with `place_in_cover`: the ray parity of the point along +u over the outer loop's Bézier pieces alone. Every piece is first decided clear along the chart past the band (`bool_spline_region_trim`, metered by the window's inf arms over the chart's cells, every exit of the window decided first).
    - One In places the ring. No In, a second In, any undecided reading, or a channel past the cap answers None.
    
    The rule is right on every chart: a ring meets no edge of its face, so one point places it. As written it cannot run on an analytic chart, because it reads only iso and NURBS images and only a spline chart's cells. Read on an analytic chart, it also tries a sphere's v-period copies, whose windows lie past the pole rows, where the azimuth arm is zero. Those arms are read at the copy's principal latitude, since a deck translation is an isometry.
    
    A ring stored on the sphere's twin sheet has no translation that reaches it, and its window has no arm, so it answers `Undecided`; no door-built ring carries the twin. Measured on the reach probes' 24 rings (`crates/mesh/tests/lift_probes.rs`): 23 are inside as stored, and ball ∖ tube's whole projected circle is one u period below its outer loop.
    
    ## Change
    - `Body::face_lift(face, band)`, beside `Body::loop_lift`, answers a face's outer lift and each ring's lift moved into the outer loop's sheet (`pcurves::FaceLift`, `PlacedRing`).
    - The rule is `pcurves::place_ring`, factored out of `with_rings`, with its candidates, order and answers unchanged. `with_rings` calls it with its own reading.
    - On an analytic chart the point is read by the same ray parity over the lifted rows' `Pcurve::chart_box`, closed by straight joint segments. Each clearance is decided through the band (`ring_place_clear`), metered by the chart's inf arms: `geom_brep::azimuth_lever_inf` (made public) and the polar radius or 1, gated on `ring_place_arm`.
    - The two readers' work bounds have one home at the root, which the trim reads.
    - A spline face's trim places its own rings inside the boolean engine, which the root door may not reach (`[topo.homes]`). The door answers a spline face `SplineChart`: the door does not read that chart yet, which says nothing about the rings.
    - Typed answers (`pcurves::RingPlacementError`):
      - `Gap`;
      - `OuterOpen` (a seamless band's outer loop does not close);
      - `RingOpen`;
      - `NoShift`;
      - `Shifts`;
      - `Copies` (past the work bound, or a count with no i32);
      - `Undecided` (in band, past a work bound, or no window with arms);
      - `SplineChart`.
    - New roster door `ring_place_`.
    
    ## What moved
    - The ring probe now reads every ring through the door. It places all 24, the ball ∖ tube ring by (1, 0) and the rest by (0, 0), and every sample of each moved ring lies inside its outer polygon.
    - The seamless pipe, frustum and torus band each answer `OuterOpen`, and an unminted face answers `Gap`.
    - The spline reader's answers do not move: same candidates, same reader, same order, and its signatures are main's.
    - The analytic reader's rows read a concave notch, rays along an iso row and through vertices, points 1e-6 and 1e-12 inside a boundary, a sliver thinner than the band, a polar cap, a torus in both channels, and a cone's apex row.
    - Two near answers are the reader being conservative: 1e-4 below a pole row and 1e-3 above an apex row, where no window holding a positive azimuth arm fits.
    
    ## Seams
    - `pcurves.rs` (CHARTTAIL): a `#[path]` submodule `pcurves_rings.rs`, `Lift::read_from` (`read_with` takes a start deck), and re-exports.
      - `charttail/azimuth-inf-arm-has-two-homes` reads the arm and now records this third assembly.
    - `boolean/spline_region.rs` (SPLICE and the boolean programs): `with_rings` delegates to the root rule, and the work bounds are read from the root. No new crate-visible item.
    - `geom-brep/src/pcurve_cache.rs` (PCERT): `azimuth_lever_inf` is public; its body is unchanged. `honeone/projected-turn-reads-short-at-a-window-wide-azimuth-inf` reads it.
    - `body.rs` (TOPO): one new method.
    - `tests/recourse_roster.rs`: one door, two carriers.
    
    ## Sweep
    Shape: a reader that takes more than one loop of a face into one chart region.
    - `spline_region::Trim`: fixed (calls the rule).
    - `cover::TorusChartWindow::open_loop`: abandons the window on a ring (wider, not wrong). Filed `tesslane/the-torus-chart-window-abandons-a-face-that-holds-a-ring`.
    - `chart_region::extract_face_uv`: its area deductions are translation-invariant. The interior-witness rung arranges ring segments where they are stored, so a displaced ring costs a witness. Filed `charttail/the-interior-witness-rung-reads-a-ring-where-it-is-stored-not-in-its-outer-loops-sheet`.
    - `clearance.rs` boundary images: ring images seed meeting stations, which are evaluated back on the carrier. Placement-invariant.
    - `mesh::memo` and `mesh::trimmed`: each loop is read alone, and a ring on a curved face refuses today. Unit 3 reads them through the door.
    - `props::quad_lane`: each closed loop's integral is invariant under a whole-period shift.
    - `chart_window`, `cover.rs`'s cone apex read, `chord_join`: one loop or an open run, so not this class.
    
    What the pattern cannot match: readers that assemble loops from stored rows without the lift API. Second pass over files reading `.pcurve(`/`pcurve_of`/`chart_pcurve` that also iterate a face's loops: the Euler files, `null.rs` and `boxes.rs` move rows per half-edge and read no region. Readers that place loops through `cell_region`/`station_region` or the point-in-face ray cast work in model space and draw no chart polygon. No further reader.
    
    ## Filed
    - `tesslane/the-ring-door-reads-a-spline-face-through-the-trims-point-reader-at-the-root` (P1): the trim's point reader moves to the root, and the two parity loops become one.
    - `tesslane/an-analytic-outer-loops-whole-span-boxes-never-tighten-under-the-ring-reader` (P2).
    - `tesslane/the-torus-chart-window-abandons-a-face-that-holds-a-ring` (P3).
    - `charttail/the-interior-witness-rung-reads-a-ring-wh
