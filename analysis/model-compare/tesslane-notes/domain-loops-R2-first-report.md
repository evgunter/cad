# Review — PR 6960 `tesslane/domain-loops` @ `4bd6709df9` (full + style)
    
    ## Verdict: APPROVE-WITH-FIXES
    No MAJOR. Every ringed mesh I could build is watertight and right; nothing ringless moved. Conditional on CI, which is NOT green: when I last looked, `viewer app feature (eframe + wgpu, wasm32)` had FAILED (run 43622354940, this head) and kernel/lint/demos/python/budget were still running. See m1.
    
    ## Claims
    1. **Held.** Differential, merge base `dcfe5b20` vs head, one probe file in both trees: 1602 lines (89 poses × 6 ops × 3 δ; cone/sphere/cylinder/torus booleans): all 1098 ringless or no-body lines identical (mesh digest of positions+triangles, `check_mesh` outcome, refusal text); all 504 ringed lines were `RingOnCurvedFace` on base. By reading: a ringless face's `face_lift` is `Lift::read(outer)` plus the winding refusal; a winding ringless polygon did not close on base either. Goldens/budget/tour ride CI (unfinished).
    2. **Held, two corrections.** The lane places nothing and re-reads nothing. But the door is read twice per trimmed face (`memo::FaceInputs::gather` and the lane), and two refusal texts are not true of bodies that raise them (m4, m5). Nothing in `mesh` decides closure any more.
    3. **Held on what builds.** Two rings on one face (pipe ± two bars, sequential), moved rings (cylinder `by (1,0)`, sphere, torus `u`), reversed ring → `TrimLoopOrientation`. NOT built: a ring touching the outer loop or another ring at a vertex (my diamond poses merged into the outer loop or the boolean escalated); a torus ring moved in `v` (none of 16 poses stores one).
    4. **Held.** Mutants by env switch, run over the five pinned ring bodies × 3 δ:
       - rings dropped: debug chord census panics ("edge of 1 face triangles"); all rows red.
       - ring sides counted twice: census panics ("3 face triangles"); all red.
       - ring read where stored (`loop_lift`): ONLY ball ∖ tube red, `MismatchedWinding`, at all three δ; 41 other runs green.
       - ring +2π in `u`: every chart red, `MismatchedWinding`. +2π in `v`: torus `MismatchedWinding`; cone `BothNappes`; sphere `PastPole`; cylinder `FoldedTriangle`/`BoundaryEar`.
       - ring reversed: `TrimLoopOrientation` everywhere.
       A misplaced ring is a mesh `tessellate` returns `Ok`; only `check_mesh`'s winding sees it. I found no body where a wrong ring passes `check_mesh`.
    5. **Held.** 441 ringed meshes pass `check_mesh`; ~600 runs pass `reach_probes::geometry` (volume, winding, deviation). Ring edges exercised: spiric + latitude circles (torus), rulings/arcs and ellipse arcs (cylinder, cone), one whole projected circle and two arcs (sphere). One count per edge is by construction (`chords::tighten`/`settle`); not instrumented separately.
    6. **Held by inspection**, `PoleInside` unreachable given the door's winding. Ringed faces that still refuse: m2, m3, N2, the seamless bands.
    7. **Held by inspection.** `v` is linear in the chart, so its extremes over the lifted disc are on the outer loop; `trim_orientation` excludes the complement; the windows read 3-D heights/radii or `v` boxes, which no placement moves.
    8. **Partly.** Rings ride the development (T1, 7 δ). A ring near the apex refuses (m2). A ringed face through the apex: not built.
    9. **Held by inspection.** Key = images as placed + chord params + positions + ε, K, so the mesh is a function of it; loop order matches `domain_loops`. A ringless closing face's key is unchanged.
    10. **Corrected** by m1 and m6. Ringed spline face not buildable here (booleans refuse `KindUnsupported`/`Escalated`). Rename complete by grep; the remaining `ring_on_curved_face` is `MassPropsError`'s.
    11. **Corrected.** Class verified on an outer loop: tilted pair at δ 0.04, two triangles, 1.1e-15 and 5.9e-15 (head; that path is byte-identical to base). Census: pinned rows at 7 δ (5e-2…5e-4): only the torus at 1e-3. 16 torus ring poses × 3 ringed ops × 3 δ: 12 of 144, all at 1e-3, poses B2@0, B2@π, low@0, low@π (bar azimuth on a lattice column and face on latitude ±π/6). Sphere, cylinder, cone: 0. One class; a ring adds no mechanism. Downstream takes it as sound (m7).
    12. **Held by reading**; `loop-boundary-discards.sh` passes with the line gone (ran it; the register pins a count per site, and the site left). Sweep and tour suites not re-run by me.
    13. Deviations all reported by the PR and the no-area one is filed. My own outer-only sweep found the same readers the PR lists, plus S1/S2.
    
    ## Findings
    - **m1 MINOR (gating until explained), DEMONSTRATED as a fact, cause unsure.** CI job above fails with 62 clippy `result_large_err` errors in `viewer` (`session::refuse::Refusal`, "at least 128 bytes"). `TessellateError` is now 88 bytes, `RingPlacementError` 72 by value in `DomainUnread` (measured). I did not establish that the growth is the cause; the PR's pre-push list has no viewer clippy. Check the same job on main.
    - **m2 MINOR, DEMONSTRATED.** `trimmed.rs` cone arm: T1's wedge moved toward the apex (edge at `(0.5s, h−1.5s, 0)`): s=0.3 meshes; s=0.1 and 0.03 refuse `CertificateExceeded` at δ 5e-3 and 1e-3; s=0.01 at all three; s=1e-3 `DomainUnread{Undecided, cause None}`. The ringless ops of the same poses mesh. Unreported; probably `a-cone-face-ending-near-its-apex-refuses-at-fine-delta`'s class reached by a ring.
    - **m3 MINOR, DEMONSTRATED.** A valid ringed torus face still refuses: (donut ∖ brick x±0.6, y±1, z 1.2..2.8) ∪/∖ stub (x±0.02, y±0.1, z 2.3..3) turned 0.33 about `y` → `DomainUnread{Undecided}`, predicate `ring_place_clear`. The outer loop holds a spiric, whose box never tightens (the door's filed limit). The PR's account of what still refuses omits it.
    - **m4 MINOR, BY-INSPECTION + mutant.** `types.rs` `TrimLoopOrientation` Display still says the boundary polygon "winds against the face's orientation, so it bounds the face's complement". The new cause (a ring wound with its outer loop) gets that sentence; only the variant doc was updated.
    - **m5 MINOR, BY-INSPECTION.** `types.rs` `DomainUnread` Display: `Gap` says "a finished body stores every one, so this is a kernel defect", but `tessellate` takes any `Body` and the PR's own row reaches it by stripping caches; it also drops `LiftGap`'s third cause (an image the lift does not apply to). `Undecided` reads as nearness; in m2 and m3 the ring is nowhere near the outer loop.
    - **m6 MINOR, DEMONSTRATED.** A typed refusal became a panic. Cylinder wall + `mev_line` strut + `kemr` + `mint_pcurves` leaves an empty ring: base `RingOnCurvedFace`; head panics `unreachable: a ring's loop holds a half-edge` (`pcurves_rings.rs` `first_point`) from inside `mesh::tessellate`. The lane keeps `EmptyLoop` for an empty lift, but the door (called first, also by `memo`) never returns for an empty ring.
    - **m7 MINOR, DEMONSTRATED.** The filed row says an STL export "would refuse `DegenerateTriangle`". `stl` refuses only an exactly zero cross product; the two torus triangles have cross 4.17e-18 (sides 0.05, 0.05, 6.0e-16), so STL writes them with a rounding-noise normal. `check_mesh` and `check_volume` pass too.
    - **m8 MINOR (tests), DEMONSTRATED.** Only ball ∖ tube witnesses "placed, not stored". Moved rings exist and mesh right on the torus (B2 turned 0.7 about `y`: `by (1,0)`) and cylinder (brick (0.5..1.1, 0.15..0.7, −0.4..0.1) turned 5.0 about `z`); neither is pinned.
    - **N1 NOTE, DEMONSTRATED, not this unit.** Ringless wrong meshes on base and head alike: T1 ∩ wedge at s=0.03 (δ 5e-3), s=0.01 and 1e-3 with xf=0 → `Ok`, `check_mesh` `NonManifoldEdge{count 4}` (11 of 1098 lines). Needs a row.
    - **N2 NOTE, DEMONSTRATED.** Sphere ring 0.006 rad from a pole (tube r 0.3, tilt 0.22, az 3): `FoldedTriangle` at δ 2e-2 only.
    
    ## Spec deviations
    Reported by the PR: torus done-when unmet at 1e-3 (filed P1); measurement row removed; notch left to unit 4. SILENT: m2, m3, m6.
    
    ## Style
    Exercised: Q1, Q2, Q3, Q4, Q5, Q6, Q7; Q8 in part (read `trimmed.rs` 1–760, the domain/polygon/orientation/singular passes and `memo.rs` gather/key whole; not the emit pass or the test module end to end).
    - S1 `memo.rs:762`, `trimmed.rs:423` — the door is called twice per face, and `memo` mints its own band (`Band::linear(tol).ok()`) beside `tessellate.rs:335`'s; a door miss is swallowed to "no images". Two reads of one value held equal by convention. **sure**
    - S2 `curved.rs:186` — the ring guard went and nothing in the curved lane says it reads the outer loop only; safety is the arm order in `lane_of`. A ringed face reaching it would mesh with its holes filled. **likely**
    - S3 `trimmed.rs` `TrimPolygon`/`TrimmedFace` — the struct is unpacked into parallel `polygon`/`next` slices; two unit rows build `next: &amp;[]` with a non-empty polygon. `starts` and `next` encode the loops twice. I would have kept a list of loops. **sure**
    - S4 `trimmed.rs` `trim_orientation` — its "no outer loop" arm cannot be reached after the `len &lt; 3` check one call earlier; a ring of no area is `MissingEntity`. **likely**
    - S5 `types.rs` Display, `pncad-py/src/py/mesh.rs`, and the two probes' `Debug`-string split — three spellings of `RingPlacementError`'s variants; `SphereTrimCause` has `name()`, this has none. **sure**
    - S6 `types.rs:613` — the falsifier paragraph had `PoleInside` swapped for `SeamTwice` mechanically; the sentence no longer describes a real pair of responses. **unsure**
    - S7 `trimmed.rs` module docs — "a boundary point inside another segment of its loop" and "a trim polygon wound against the face" predate rings; `TrimLoopOrientation`'s doc still says "(cylinder or sphere)". **likely**
    - S8 `reach_probes.rs` `Got::Unread` — no row expects it; the m3 body would. `Got::NoArea` moved the no-area check after every other check, so a row cannot say how many. **likely**
    - S9 `demos/tour/src/lily.rs` — a retired wall stays as numbered, panic-armed code in the wall list. **unsure**
    - S10 `trimmed.rs:530` — the "VACUOUS TODAY" `debug_assert` argument counts 19 ringless cylinder faces; a ring changes neither premise but the comment's census is now stale. **unsure**
    
    ## Rubric
    - Idiom/structure 4 — the door is the only placement and the diff is small; S1–S3.
    - Test quality 3 — ring rows pin real geometry and the mutants go red, but one body carries the placement claim and m2/m3/m6 have no row.
    - Doc/comment honesty 3 — m4, m5, m7.
    
    ## Executed / read
    Executed: mesh test build at head and merge base; 5 pinned ring bodies at 7 δ; 16 torus, 27 sphere, 12 cylinder, 28 cone poses through `got`/`geometry`; six mutants; the 1602-line differential; empty-ring probe on both trees; tilted-pair and STL-criterion measurements; the loop-boundary gate; type sizes. Read only: `pcurves_rings.rs`, `sizing.rs` windows, `chords::tighten`, sweep/tour/pncad-py diffs, work rows. Not run: sweep, tour, pncad-py suites, ε 1e-6/1e-12.
    Test worth keeping (m6), in `lift_probes.rs`: `let mut b = rod(1.0,-2.0,4.0);` take the cylinder face's outer `first`; `let s = b.mev_line(MevSite::Fan{he1:first,he2:first}, Point3::new(0.6f64.cos(),0.6f64.sin(),0.3), tol()).unwrap(); b.kemr(s.he_plus,s.he_minus).unwrap(); topo::mint_pcurves(&amp;mut b,tol()).unwrap();` then assert `mesh::tessellate(&amp;b,1e-2,tol())` is `Err`, not a panic.
    I saw no other review's material; of PR 6960 I read the description, check runs and one failed job's log only.
</cross-session-message>

0 notifications remain queued.
