REVIEWER lane, PR 7573 @ 24ee9a28fb — MAJOR (claims 1, 8, 9), DEMONSTRATED. A body that main meshes with positive volume comes back from this head as a flat sheet counted twice: zero volume, negative sign. It is the outcome the unit was decided against, on a body the unit's measurements did not cover, and the row that holds the body cannot see it.

**The body.** `sweep`'s `snowman::a_cut_ends_at_the_nearest_of_several_boundary_hits` (snowman.rs:1247), its inner loop: the cap of height 0.001 of the unit ball, one sphere face and one plane face on one circle, at lat +60 and −60. `cap_volume(1.0, 0.001)` = 3.1405e-6. The row's own comment (PR 5920's review, B4) says of it: "the cap meshes manifold and encloses its volume at every δ the review swept".

**Raw lines** (one `eprintln!` added after `mesh::tessellate(&cap, delta, ..)` in that loop, same edit in both trees; `cargo test -p sweep --test all snowman::a_cut_ends_at_the_nearest -- --nocapture`):

Merge-base `dd8e70198d` (per-patch triangles [sphere, plane], positions, `mesh::validate::signed_volume`):

    lat 60  δ 0.04, 0.02, 0.01: [18, 6], 14, +2.2794e-6      lat -60 δ 0.04, 0.02, 0.01: [18, 6], 14, +2.2794e-6
    lat 60  δ 0.005:            [24, 8], 18, +2.3894e-6      lat -60 δ 0.005:            [24, 8], 18, +2.3894e-6
    lat 60  δ 0.004:            [27, 9], 20, +2.4415e-6      lat -60 δ 0.004:            [27, 9], 20, +2.4491e-6

Frozen head `24ee9a28fb`:

    lat 60  δ 0.04, 0.02, 0.01: [14, 14], 16, +1.8649e-6     lat -60 δ 0.04, 0.02, 0.01: [6, 6], 8,  -7.33e-20
    lat 60  δ 0.005:            [12, 12], 14, +1.1026e-6     lat -60 δ 0.005:            [8, 8], 10, -1.95e-19
    lat 60  δ 0.004:            [17, 17], 19, +1.7735e-6     lat -60 δ 0.004:            [9, 9], 11, -1.93e-19

`test result: ok. 1 passed` on both trees.

**What happens.** At lat −60 on the head each patch has n − 2 triangles on the circle's n chord points and no other point (8 positions = the 8 boundary points): both faces are triangulated on the same points, all in the cap's plane, so the mesh is the plane disc twice. The join split nothing there, so the two patches share no inside side: they lay different diagonals. No side is overused, `check_mesh` passes, the certificate holds (the cap is 0.001 high, δ is 0.004 or more), and the mesh encloses nothing; the sign is rounding's (negative on all five cells). At lat +60 the two patches do share sides, the join splits them (16 positions on 8 chord points at δ 0.04), and the cap has 35 to 59% of its volume where main had 73 to 78%.

On main the sphere arm's shared-side insert put a point of the sphere on every side between two non-neighbour boundary points the plane face held, whether or not the plane made it. That is the "997 points ... where about 502 sides are in fact made by both" the unit calls a wrong prediction; on this body those points are what gave the sphere patch any point off the plane.

**Why it matters for the claims.**
- Claim 1: this body moves in a way the PR does not list. Its step 4 names only spline walls ("on every moved body the join splits nothing, `check_mesh` passes, the signed volume is unchanged to 10⁻⁹"). Here the volume goes from 2.28e-6 to zero.
- Claim 8: "is anything else [the insert] did incidentally now uncovered?" Yes: it kept a face thinner than δ off the plane of its boundary when its neighbour triangulated the same points with other diagonals.
- Claim 9: the unit's reason for the own-surface split over the straight midpoint is that the latter "leaves the two sheets on one set of chords" with "the sign of a zero volume ... rounding's", and the owner's word on PR 7485 was "allowing zero volume sounds annoying". The built design has the same end state whenever the two patches make no common side. The shell row cannot see it (both faces there lay the same chords), and `work/tesslane/a-body-thinner-than-delta-meshes-inside-out-and-nothing-refuses-it.md` gains a body that was right on main.
- The row holding the body passes on both trees: `check_mesh` and `assert_meshed_geometry`, whose volume bound (δ · area) is far above 3.1e-6. Its comment's "encloses its volume" is no longer true at lat −60 and nothing reads it.

**How found.** My per-patch differential (below) over `sweep`'s mesh-reaching modules flagged one body key whose signed volume changed sign; I then named it with the thread name and reran the row on both trees. Position bits of every patch of every `tessellate` call, keyed by a hash of the body's vertex points, face surfaces and edge count with δ, merge-base against head:

    mesh suite, slow set, ε default / 1e-6 / 1e-12: 12,932 / 12,312 / 12,396 body×δ keys in both trees; 12,669 / 12,069 / 12,133 identical; a mesh on base and a refusal on head: 0 / 0 / 0; a refusal on base and a mesh on head: 90 / 89 / 90 (all `check_mesh` Ok, all positive volume); moved mesh to mesh: 172 / 154 / 172.
    sweep's 68 mesh-reaching modules, ε default: 3,119 keys in both; 3,032 identical; mesh to refusal 0; refusal to mesh 0; moved 87, of which 4 are this body (the cells above).

In the `mesh` suite the other direction dominates: 23 body×δ keys that were NEGATIVE on the base (−0.0715 to −0.037, the 1.05 bored ball's cells) are positive on the head. So the unit cures the family it measured and regresses this one.

I have not searched for further bodies of this class beyond what the two suites hold. A cap, lens or sliver between a plane and one curved face, thinner than δ, with few chord points, is the shape to look at.
