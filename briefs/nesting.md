# Lane `heldrecords/nesting` — P1

Rows (read both in full):
- `work/heldrecords/a-hole-in-an-elliptic-section-keeps-a-face-of-its-own-that-tier-3-prime-refuses.md` (P1, M)
- `work/heldrecords/a-plane-touching-a-counterbores-floor-rim-ships-sides-tier-three-prime-refuses.md` (P1, M)

Branch `heldrecords/nesting` off `origin/main`; PR base `main`. Review tier: single FULL review.

Orchestrator's hypothesis, to be checked against the tree, not built on: both rows are one class. The split's section assembly (`crates/topo/src/splitting/finish.rs` `nest_hole_sections`, `section_loops::outlines_disjoint` / `line_clears_conic`) fails to nest a loop inside another, so the side carries a cut face plus a face of the other sense over it, "sound by cancellation": volumes and tier 3 pass, tier 3′ refuses. In the elliptic row the cause is measured (a segment is cleared from an ellipse only when the whole carrier is on one side of its line). In the counterbore row it is measured only as far as "face 11 is a second cut-plane face of the other sense over face 13" — why the trace is not one notched face is unmeasured. Measure it first and say whether it is the same cause.

Owed: the section is one face per region, notched where it should be (the hole nests as a ring of the outline); the "sound by cancellation" allowance retires if nothing else needs it (say what you found). The decision must be certified and fail loud (typed refusal) where it cannot be decided, never a silent second face. Lift the tilted arm of `a_keel_ring_in_an_elliptic_section.rs`'s 3′ pin; pin the counterbore poses at tier 3′ (`validate_pseudomanifold` with no records) on each half, both normals, both azimuths, both leans. Sweep for every other path that can still leave a cancelling face pair, and give the hit list.

Coupling: PR 7124 (`embed/records`, open, EMBED's lane) touches `splitting/*` for records carry; your change is in the section assembly and should be logically independent. Say so in the PR if it is not.

Standing rules: `briefs/common.md` on this branch (`origin/heldrecords/briefs`).
