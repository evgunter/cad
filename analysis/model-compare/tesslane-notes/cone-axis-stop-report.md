cone-axis-winding — REPORT: STOP. The cure is a fork I can't settle from the code and the ratified docs. Nothing built or pushed; head unchanged at 22c7daa73a80d53763584439945d54514ff9ab4b (confirmed with ls-remote). Tree clean; every probe below was temporary instrumentation, now reverted.

1. MEASUREMENT (the spec's premise is wrong)
- The refused cone face goes through the CURVED lane (`tessellate::lane_of` → `Lane::Curved`, `curved::tessellate_curved`), not the trimmed lane's development. The spec's "Not this branch's" paragraph emulated the trimmed cone arm. That arm never sees this face, so the emulation couldn't move it.
- The face is half the cone: chart box u ∈ [π, 2π], v ∈ [−2.4, 0], apex row at v = 0. Its polygon is 23 points: rim chords at v = −2.4, plus each side ruling split once at slant v = −1.039 (ids 2 and 4). The apex is mesh id 0, entered twice: (π, 0) and (2π, 0), both pole entries. A split side means `bare_sides` is false, so `grid_counts`' cone arm keeps its lattice (the arm `a-cone-face-with-a-split-side-has-no-worst-triangle-theorem` describes). At s = 0.6 that is nu 17 × nv 11 at δ 5e-3 and nu 36 × nv 23 at δ 1e-3.
- Every unresolved triangle has the same three corners:
  - an apex copy, (π, 0) or (2π, 0);
  - lattice node (u₁, −j·hv);
  - lattice node (u₁, −(j+1)·hv), where u₁ is the interior column next to that side.
  In the chart it is a real triangle. In space all three corners lie on one ruling: the node offsets from the apex are exactly 1× and 2× one vector, e.g. (−0.0200, −0.1890, −0.1072) and (−0.0401, −0.3779, −0.2145). So it has no area by construction; it is not a thin triangle that a different δ resolves.
- Which step makes it: the chart CDT, which runs in raw (u, v). The side from the apex copy down to the split point is one bare constraint about 1.04 long. Delaunay fans that apex copy down column u₁ to roughly mid-side, and the split point fans the rest. Every fan triangle that takes two column nodes is degenerate, because the apex lies on every column's ruling and the chart can't express that.
- Counts: at nu 17 there is one such triangle per side; at nu 36, three to four per side. The coarse cells mesh because there the fan reaches only one node of column u₁ (hv is about half the 1.04 side or more).
- A sphere pole doesn't make this shape (a meridian is an arc, so the triangle has area). In the trimmed lane's development the apex is one CDT vertex and a ruling is an exact line, so the CDT can't build it.
- §5 sweep, pass 1: the whole mesh suite, instrumented at the join and in both lanes, base tree, 490/490 passed. Unresolved triangles occur only in T1's row (all of them curved, all this shape). The other hit is `trimmed::tests::a_lattice_laid_without_clearance_cuts_a_triangle_the_join_refuses`, which removes the clearance on purpose. Only two rows reach the curved apex + split-side cone arm at all: T1's row (18 face runs) and `curved::tests::a_multiply_carried_iso_side_is_bitwise_straight_and_meshes_watertight` (8 runs, δ 0.1, meshes).
- What pass 1 can't match: bodies the suite doesn't build. Pass 2 was aimed at that gap: plain cones and cone wedges (π/2, π, 3π/2, full; half-angles 45° and about 80°), each edge split at {0.5}, {0.3129, 0.15645} and {0.9}, placed obliquely, at δ 0.1 down to 0.001.
  - **Every cone apex face with a split ruling refuses `UnresolvedWinding` from δ 0.01 down: 84 cells.** This includes the plainest body there is, a full revolved cone with its seam ruling split at its midpoint.
  - The 80° cone also refuses `CertificateExceeded` at many δ. The mirror-nappe diamond refuses `CertificateExceeded` at every δ when split.
  - Ball and sphere band: clean.
  - So the class is the whole curved apex + split-side cone arm. T1 is one instance.

2. THE FORK (why I stopped). The cure is "a ruling is a line through the one apex point" reaching this arm, and there are three defensible homes for it:
  (a) Route apex + split-side cone faces (or all cone faces) to the trimmed lane, which already has the rule. Measured with a temporary route of every Cone face to Trimmed: all 18 T1 cells give `Got::Meshed` (`check_mesh` and `geometry` pass), and every pass-2 body meshes at every δ, including the ones the curved arm refuses `CertificateExceeded`. But this is a slice of unit 6, `rectangle-faces-move-to-the-chart-lane` (design: true, Ev's, parked behind units 4 and 4b). It moves curved bytes, and `lane_of` is PR 7041's.
  (b) Give the curved cone arm a development-plane CDT, borrowing `trimmed::developed`. That is a new mechanism in a lane unit 6 retires. Its sector is convex only up to an angle of π, and this lane keeps every CDT triangle, so a face whose sector is wider (sin α · Δu > π) would need either an inside test or a refusal (a face that meshes today newly refusing is a stop). A face spanning both nappes can't be developed at all.
  (c) Let `[mesh.winding]`'s refusal stand until unit 6. The nine cells stay pinned as `Unresolved(Cone)`, and the class is recorded with the plain split-seam cone as its witness.
  My recommendation: (c) now, with this measurement filed against unit 6 as evidence that apex cones should move. Or, if you'd rather cure it now, the narrow form of (a), put to Ev. I lean away from (b).

3. Ran locally: the T1 row, the instrumented whole mesh suite (twice, 490/490 both times), and the temporary probe bodies. No pre-push list was run, since nothing is to be pushed.
4. What slowed me: the spec named the trimmed lane. The first instrumentation went into `trimmed::emit` and printed nothing, which took one build to discover.
5. Waiting for your ruling. I have not filed the class (it is inside this unit's fence, and I may not edit the item file); say where you want it and I'll file it.
