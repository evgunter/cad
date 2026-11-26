# Lane `heldrecords/minted-door` — P1

**Row:** `work/heldrecords/the-boolean-door-decides-no-minted-vertex-on-edge-meeting.md` on origin/main. Read it in full, especially its "Way through".

The design is decided: two designers converged, a probe confirmed it, and it is logged in `work/heldrecords/log.md` (2026-11-25/26). It sits within Ev's ruling `a2090eeb4d5` ("the boolean's tool decides every such meeting at its door", no cutter field). **Do not add a maker or cutter field.**

**Base:** branch off origin/main and open the PR to main.

**Owed:**
1. **Decide minted vertex-on-edge meetings at the door.** The boolean's carried door decides a minted point meeting between a vertex and an edge with the census's own interior reading: `census.rs` `on_edge_interior_reading`, or `curved::on_curved_interior_reading` for a curved edge. Both return the margin they decided Zero.
   - Zero: land, and cite the decision beside the record.
   - In band: escalate, as `vertices_meet` does.
   - Off: not a landing.
2. **Write the decision row on the record's own two input cells (E, F)**, not on cells minted in the op's clones. Move the vertex–vertex door's row (`decide_one` on `uncopied`) the same way.
   - Before building, re-read `crates/topo/README.md` [coincidence] and the carry paragraph for any text saying a decision row names the meeting's own cells. If any exists, stop and report.
   - Check the `unproven-coincidence` lint and every reader of `pending` (`cite_rows`, `place_found`) for an assumption that a vertex–vertex row names two vertices.
3. **Name section vertices on the carried path too**, now that no row names a clone-minted vertex. The panic this used to cause is in `a_vertex_through_a_declared_seat::a_wall_vertexs_edges_are_in_the_seat_on_both_sides`, and it must pass. Close `a-carried-records-section-vertices-are-not-named-in-the-booleans-division` if this covers it.
4. **Witnesses:**
   - `sweep a_pierce_held_in_a_curved_face::a_plane_face_holding_the_pierce_over_a_cylinder_builds_in_every_op_and_order` builds. Its t3′ admits only its pinned kinds.
   - `three_solids_on_one_line::a_contact_line_beside_a_face_holding_it_builds_in_every_op` still builds.
   - Add a **stress row** where the tool face X grazes the record's face F at the section, so the v–s distance is amplified. Show what it does: Zero, escalated or refused, never a wrong body.
   - Probe code for margins is on `origin/heldrecords/probe-minted-margin` (`PROBE_MARGIN`); reuse it.
5. **Evidence**, because this is a carry change:
   - the full **default** profile (slow set included) for topo and sweep, per test, head vs base;
   - `ci` at 1e-12.
   Built→refused must be empty. Known main reds are listed in work/ (including INSIDE's new loft row).

**Review tier:** single FULL review.

**Standing rules:** `briefs/common.md` on this branch.
