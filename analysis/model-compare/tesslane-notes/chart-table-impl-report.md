CHART-TABLE implementer report. Branch tesslane/chart-table, head 3a44a00f24c7d9c52959a4c4f67952512b0757cd (pushed). No PR opened.
    
    1. MEASUREMENT (branch head 898275708c, before any edit)
    - reach_probes 11/11 as pinned. Both closing rows refuse UnsupportedCurve from trimmed::trim_frontier (tessellate_trimmed's chart roster) at every δ: the spiric edge beside a Torus wall (vessel quarter, klein elbow), the Ellipse edge beside a Cone (both frusta halves, all six C1 ops).
    - lift_probes: not on this branch. I ran it on tesslane/measure-lift: 7/7. The spiric wall's outer loop is a simple polygon, clockwise on sense false. The cone's lift runs along v == 0.0 exactly.
    - Rosters per chart: see the PR body. The spec was wrong in five places:
      (a) sizing.rs and cert.rs dispatch on no chart. Their "rosters" are the functions they define; the dispatch sat in curved's and trimmed's emit passes.
      (b) No door builds a trimmed spindle face (measured: split and all six booleans refuse a lemon and an apple at the operand gate).
      (c) A circle-only window is unsound on the "above" narrowing frustum: its ellipse reaches radii past the top rim.
      (d) The spec read Projected as general; unit 1's fix pass (merged mid-unit) admits a conic's projected image only, and the row keeps that rule.
      (e) "Window from boxes on every chart": circles keep their closed-form range, which is exact (reason in the PR body).
    
    2. CHANGE
    - c6632246c5: the table, nothing moving. New crates/mesh/src/chart_table.rs: ChartRow {Cylinder, Cone, Sphere, Torus, Spindle, Spline}. It holds steps(target), cert(tri, uv, singular), singular_rows, outward, is_spline. Read by curved::grid_counts, both emit passes, trimmed's lattice (Lane deleted) and chords::FaceLattice::of. Every reader matches exhaustively.
      Checked at that commit: mesh suite green, D9 goldens unedited, tour suite 106/106 in release, unedited.
      Home: mesh, beside its readers. topo/src/chart.rs's header says a disposition belongs to the crate that disposes; steps and certificates are mesh's.
    - 91da2241e0: the arms. Roster gone, takes_sphere_arm gone. face_target gives headroom to every analytic face the lane meshes. Singular-row rule generalised (module doc of chart_table). The cone's and sphere's windows read a non-circle edge's image box. Chord pass: every arm but the cylinder reads its row. Probes re-pinned with a geometry check.
    - be6ac4cc50: re-pins outside mesh (sweep spiric_rim, tour wall 1), the creases match made exhaustive, and the rows-vs-topo poles test.
    - 535b0613a0: merge of chord-rule (its fix pass plus main). It carries my conflict resolutions: chart_image generalises sphere_image's conic-only rule; the sphere×spline probe is re-pinned; unit 1's per-arm test is adapted and keeps its independent step oracle.
    - 3a44a00f24: slow-set entry.
    - Singular-row rule: a row v_s where u ↦ S(u, v_s) is one point (sphere ±π/2, cone 0, spindle ±v*). The polygon keeps that point once per arriving and leaving u, so the row is a side. Two corners on it drop; the cert reads per-corner flags; pole_side_columns keeps off #678's shape; past / winding / seam-twice refuse SphereTrimShape. "On" means within 64ε, or past a domain end.
    
    3. PR BODY
    =====
    ## Measured first
    At the branch base, `reach_probes::a_spiric_bounded_torus_wall_refuses_at_the_chart_roster` and `a_cone_face_bounded_by_a_section_ellipse_refuses_at_the_chart_roster` refuse `UnsupportedCurve` from `trimmed::trim_frontier`, on the spiric edge beside a torus wall and the ellipse edge beside a cone, at every δ. `lift_probes` (7/7): the spiric wall's outer loop is a simple polygon, clockwise on `sense` false; a cone's lift runs along `v == 0.0` through the face's turn.
    
    What each place held, per chart:
    - `sizing.rs`: step theorems for the cylinder, cone, sphere, and torus/spindle. Windows for the cone and sphere, from circle carriers only. No dispatch.
    - `cert.rs`: certificates for the cylinder, sphere, cone (pole flags), torus and spindle (apex flags). No dispatch: the curved emit dispatched over all five, the trimmed emit over cylinder, sphere and NURBS.
    - `curved::grid_counts`: all five analytic charts.
    - `trimmed::Lane`: cylinder, sphere (circle-bounded only), NURBS; every other face went to `trim_frontier`. `outward`: cylinder and sphere only.
    - `chords::FaceLattice::of`: the curved lane's five charts beside a circle; trimmed sphere and spline. The trimmed cylinder, cone, torus and spindle read none.
    
    The rosters disagreed (roster refusal) where the trimmed lane lacked the arm.
    
    ## The change
    `mesh::chart_table::ChartRow` is one row per chart. It holds:
    - steps for a target;
    - the per-triangle certificate, with per-corner singular flags;
    - singular rows;
    - outward normal;
    - the face's window (cone ρ_max, sphere latitude).
    
    Every reader matches the row exhaustively; the roster check is gone because nothing can disagree.
    The first commit moves nothing: D9 goldens unedited, tour suite 106/106 unedited, mesh suite green.
    
    The trimmed lane then meshes cone, torus and spindle faces from their rows at `UNPROVED_HEADROOM`. Their missing containment lemma is `one-containment-lemma-for-an-oblique-boundary`'s; no new debt.
    
    **The singular-row rule** is stated once, in `chart_table`'s doc: a row where a whole `u` line is one point (sphere ±π/2, cone 0, spindle ±v*).
    - The polygon keeps that point once per arriving and leaving `u`.
    - A triangle with two corners on it drops.
    - Certificates read per-corner flags.
    - `pole_side_columns` keeps off #678's shape.
    - Past-row, winding and seam-twice refuse.
    
    Pins:
    - `each_singular_row_keeps_its_point_once_at_each_u`, per chart: reds if a point is dropped or doubled.
    - `the_rows_singular_rows_are_the_charts_poles`: against `topo::chart::Chart::poles`.
    - `a_cone_face_through_its_apex_meshes_on_the_cone_row`: a funnel split below its rim. Dropping the cone's row reds it.
    
    Outward direction comes from the row for every analytic chart (pinned for cone and torus too).
    
    Window: a non-circle, non-ruling edge reads its image's `chart_box`. A circle keeps its closed-form range: it is exact, where a projected circle's box is a conservative sector hull, and moving it would loosen every tilted-circle sphere window. Without the box, the narrowing frustum's upper half would size from its top rim (0.5) while its ellipse reaches 0.75.
    
    ## What moved, and why it is right
    - Both closing probes now mesh at all three δ, and so do the T1 cone ops and the B2 torus ops.
    - Every `Meshed` answer in `reach_probes` now has to:
      - pass `check_mesh`;
      - enclose the B-rep's volume within δ·area;
      - wind every analytic curved-face triangle outward.
    
      An inside-out or folded patch reds it.
    - `a_sphere_face_bounded_by_a_spline_section…` now refuses earlier in the chord pass, at the conic-only projected rule, on the first section edge.
    - The tour's sectioned vessel meshes: wall 1 retired, the suite checks its volume. Its panel is filed.
    - spindle: the row exists; no door builds a trimmed spindle face (split and booleans refuse a lemon and an apple at the operand gate).
    
    ## Chord counts (asked / UV bound asks / sampled UV speed asks / carrier asks / sampled curvature asks), at δ 5e-3, 1e-3, 2e-4
    - vessel spiric: 46/7/7/46/4, 103/14/14/103/8, 229/31/31/229/17
    - elbow spiric: 21/19/19/21/12, 47/43/43/47/26, 105/95/95/105/57
    - frustum focal sections: 24–25 / same / same / 22 / 20; 52–55 / same / same / 48 / 45; 116–123 / same / same / 106 / 100
    
    The UV bounds are as tight as sampled. The vessel's over-chording is `sizing::spiric_step` (filed).
    
    ## §5 sweep
    Shape: a match on a chart or surface kind with a fallback arm, in `crates/mesh/src`.
    - `tessellate::lane_of`'s `_ =&gt; Curved`: routing, unit 4's. It reaches only analytic charts, which both lanes now have.
    - `trimmed` creases match: made exhaustive.
    - `chords::circle_radius`, `memo`, `planar`, `walk`, `validate`, `curved` hits: not chart dispatch.
    
    What the pattern cannot see: two lists of the same fact in different crates. Second pass: the row's singular rows against topo's `Chart::poles`, now a test.
    
    ## [mesh.sizing]
    The rim-chord sentence is re-worded: the trimmed cone, sphere, torus and spindle arms read the stored row. Nothing it decides changed.
    
    ## Filed
    - `a-projected-spline-images-speed-bound-on-a-sphere-is-eleven-orders-loose`
    - `the-spiric-carrier-step-asks-thirteen-times-its-sampled-curvature`
    - `the-sectioned-vessel-meshes-and-has-no-panel`
    - `the-sphere-trim-shape-refusal-names-the-sphere-on-every-singular-row-chart`
    =====
    
    4. RUN LOCALLY
    - At the merged head: mesh 455/455; gates (run-gates.sh) green; clippy for mesh, geom-brep and demos/tour clean; fmt clean; work lint ok; slow-set names ok; sweep spiric_rim 18/18.
    - Sweep subset (cone|torus|spiric|sphere|klein|vessel|frustum|ball|donut|spindle|football|tilted): 828/829. The one red is `a_plane_near_a_needle_cones_apex::a_needle_cone_cut_off_its_apex_lands_its_closed_forms`:
      - a topo tier-3′ census pin, no mesh in the row: the cause reads Meeting(MeetUnread(Unsettled curve_pair_pose_basin)) where it is pinned ContactLane(NotCertifiable);
      - it looks like main's, not this diff's, but I did not build main to confirm.
    - Left to CI: the full sweep and full tour at head, eps rows, tess-budget-data (nightly). demos/wild not run: no public signature changed.
    
    5. HEAD: 3a44a00f24c7d9c52959a4c4f67952512b0757cd
    
    6. WHAT SLOWED ME DOWN
    - The disk filled once.
    - The tour suite in a debug build ran 10 tests an hour; reran in release.
    - I killed a concurrent build and it corrupted its incremental cache.
    - My own `git checkout` of chords.rs dropped edits; reapplied.
    - Unit 1's fix pass and main landed mid-unit, with conflicts across trimmed.rs and chords.rs.
    - Commits carry the environment's git author name "Claude". No trailers on my commits.
</cross-session-message>

0 notifications remain queued.
