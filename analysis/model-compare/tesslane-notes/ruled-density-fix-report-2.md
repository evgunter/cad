(4c implementer) REPORT, part 2 of 2: the digests. **One STOP for your ruling.**

**Correction to part 1:** drop "It is maximised by the narrowest cone with the smallest gap" from the needle bound. I did not measure that. The PR-body paragraph does not carry it.

**Digests.** Base 54938d6a95 against head 3f0f9434a8 (mesh code identical to 7a6f5bfe6e). Run: `-p mesh -p sweep -p step-import` whole, every tessellate call hashing each patch's triangle positions.
- 86190 base patch keys. Every base key is present on head except one row's (below).
- **No stops of the refusal kind**: no patch that base meshes is refused on head, and no refusal changed.
- Reds: the 4 sweep reds are the same on base and head (germ_tilted_rod, germ_torus_components, cylinder_sphere_frame r_0_1, torus_doors_join outer equator). The 5th red on head is `route_rows::a_body_that_meshes_records_mains_k_stream`, and it is the hook's own. Its classifier calls `walkable`, which records into the k-stream. On the clean head the row passes (part 1).
- Two fuzz rows move on their unseeded draws, so they are not comparable: `an_annular_tube_through_a_plate::the_annulus_builds_at_random_azimuth_pairs` (216 patches) and `review_chamfer_r1_probes::a_skewed_wedge_chamfers_with_every_face_outward` (80, all planes and other faces).
- Six non-fuzz tests moved. I re-ran them with each patch classed by the route's own lane (`route_of(..).lane()`) and its surface kind:
  - 130 walked patches moved. They are the class.
  - 172 plane neighbours moved.
  - **24 patches on the CURVED lane moved: 16 Cone + 8 Cylinder**, every one sharing an edge with a walked face, all in `curved::tests::a_multiply_carried_iso_side_is_bitwise_straight_and_meshes_watertight` at δ 0.1. All got coarser: cone 12/13→11 triangles, cylinder 11→10. That row asserts only Ok+check_mesh and passes.
  - Unchanged in the same six tests: 434 Curved Cylinder, 24 Curved Cone, 84 Sphere, 16 Torus, 36 curved and 45 Trimmed patches beside a walked face, and 1658 planes.
  - The other non-fuzz movers are `a_box_along_a_fillets_line_arc_junction::a_box_along_an_inflection_ruling`, `reach_probes::a_cone_face_met_on_its_axis_meshes_at_every_delta`, the two split_rulings development-resolution rows, and `curved::tests::a_split_then_placed_swept_face_is_not_refused`. In them only walked faces and plane neighbours moved.

**The STOP, and my reading of it (not verified beyond the counts).**
- The walked face's rim chords are now sized at δ by the rectangle lane's step. Before, the chart lane chorded it at UNPROVED_HEADROOM, so it asked for more chords on the shared rim. The curved neighbour holds that rim, so it meshes with fewer rim points, i.e. at what the walked face now asks. The decrease is consistent with that reading.
- Under your rule, a patch outside the class and its plane neighbours is a stop, so I have not changed anything.
- Options I see:
  - (i) Accept: curved neighbours of a walked face are in scope as "the class's neighbours". The PR body would say so, with the counts above.
  - (ii) Make it go away: keep the old chord demand on a walked face's shared rims. That gives back the density 4c removes on the walked face's own rim.
- I'd take (i).

Everything is pushed. The head is 7a6f5bfe6e. The digest hook lives only in /home/user/wt-head (uncommitted), and nothing of it is on the branch. The work owed is your ruling on the STOP.
