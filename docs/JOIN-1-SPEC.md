# JOIN-1 — a section germ names the cell it lies in

Binds the implementer of `work/join/JOIN-1.md`. Deleted at merge.

## The defect

A section segment that runs along an existing edge `e` of operand X
lies in BOTH faces of X that `e` bounds. `HalfGerm`'s per-operand
identity is one `FaceKey`, so each site picks a flank by its own
tie-break:

- the vertex-on-face classifier, `vtxfac::resolve_on_entries`, takes
  mixed → In, so the germ sits on the Out flank;
- the vertex-vertex attribution in `recl` takes the sector holding the
  on-bound as its START, which depends on orbit direction and so lands on
  opposite flanks at the two ends of one edge.

`find_match` treats the face pair as identity, so the two ends of one
segment never pair and the join refuses `UnpairedLooseEnds`. Both rules
were agent conventions accepted in passing (M3 PR 4) and never ratified.
The full measurement is in
`work/join/an-edge-lying-in-a-cutter-face-past-its-end-wall-leaves-loose-ends-unpaired.md`,
with the probe on branch `join/inface-probe`.

## The design (weighed by two designers, converged after three rounds)

1. **A germ carries a per-operand locus**: `enum Locus { InFace(FaceKey),
   OnEdge(EdgeKey) }`, replacing the bare face in `HalfGerm`'s identity.
   ONE function derives it from the site's orbit and is called by every
   site kind (vertex-vertex and vertex-on-face):
   - a germ ray lying on a real edge bound → `OnEdge(that edge)`;
   - a germ ray inside a sector → `InFace(sector face)`.

   The sweep splits an edge at each crossing, so both ends of a segment
   along `e` name the same key with no numerics. Edge-edge coincidence is
   `OnEdge` on both operands.
2. **Matching** compares loci for equality on both operands and keeps the
   opposed-sense and facing tests. Two distinct edges between one vertex
   pair are two keys. `find_match` refuses `JoinDesync` for an `OnEdge`
   germ whose edge is not incident to the site vertex (a kernel bug).
3. **One fold rule for the on-bound**: at every site, in every op, the
   on-bound folds into the **In** run. Write it once, in `sectors.rs`,
   and have both classifiers call it. Retire `recl`'s START-holder
   attribution and `vtxfac`'s own spelling.
   - **Why In:** both designers measured that nothing downstream reads
     which copy keeps `e`'s key. The zip keeps A's loop edges and kills
     B's. `name_boolean_edges` names every `seam_edges` edge as a seam
     whatever its key. `ops::Descendants` has no edge rows. So this is a
     convention, and In is the op-independent one the certified
     no-end-wall pose already runs.
   - **If it is violated:** that surfaces as `NotSameFace`, never as a
     wrong body.
4. **Surgery**:
   - **The skip:** the chord core's adjacency skip
     (`chord_join.rs`, `skip_adjacent_chord`) fires on the boolean lanes
     iff the segment's locus on that solid is `OnEdge(that between
     edge)`. That is a structural test, and it replaces the boolean
     lanes' reading of `between_edge_in_plane`, whose `Line → true` arm
     picked the wrong edge on the rod and the blind D.
   - **The split lane** keeps its geometric test (a follow-on, not
     this unit).
   - **The owning solid** gets exactly one `mef`, which doubles `e` with
     `e`'s own curve; the zero-area sliver between them is the section
     face.
   - **The partner side's chord** in `InFace(F)` takes `e`'s curve too,
     where that falls out without a new lane. If it does not, say so in
     the PR and leave the curve-on-segment work to JOIN-3.
5. **The single-site closed loop**
   (`work/join/closed-in-face-section-loop-has-one-site.md`): do NOT let a
   record match itself in this unit. Both designers reasoned that, without
   a dedicated self-loop arm, the skip fires both ways and `cut` would
   return a real face as the null face, building a wrong body. Instead,
   refuse it TYPED before the loose ends are counted, naming the class: a
   new `SplitJoinError` arm or the existing frontier vocabulary, your
   choice, stated in the PR.

## Acceptance

- `axis_lap.rs` `axis_lap_refuses_where_its_planar_twin_does` flips:
  - the diamond and the rod, ∪/∖/∩, cutter on either side, build and
    pass tiers 2, 3 and 3′ at their closed-form volumes;
  - **in particular rod ∖ and rod ∩**, which the naive flank-rule
    experiment built WRONG (off by πR²/8). Pin both at the exact volume.
- The no-end-wall controls still certify.
- `germ_coplanar_conic.rs`'s closed-tube row refuses with the new typed
  refusal, not `UnpairedLooseEnds`.
- Report what the merged teapot cup
  (`verbs_1031b_arcwind.rs` `the_boolean_after_the_merge_reaches_the_join`)
  and the blind D top pose
  (`work/join/blind-d-pocket-subtract-refuses-with-join-internal-words.md`)
  do now. Neither is required to build.
- Drive-bys:
  - `rest.rs`'s module docs claim "the SAME mutual-facing tests as the
    join", which is stale. Fix the sentence, not the zip, which is JOIN-2.
  - `recl.rs`'s docs call the START attribution "symmetric". Retire it
    with the rule.
  - `vtxfac.rs`'s "flagged for ratification" sentence goes with its rule.

## Not this unit

- **JOIN-2:** the declared-REST zip reads the join's matched segments,
  and `enumerate_segments` / `fan_edge_between` go
  (`work/zip/rest-zip-segments-read-a-straight-chord-facing-test-and-a-vertex-pair-identity.md`,
  the dumbbell).
- **JOIN-3:** the matched segment carries its curve, and
  `choose_roles` / `ring_run_ccw` read it (the blind D's second defect).
- **The self-loop arm.**
- **CONTACT's `CurveContact` cell**
  (`work/contact/curve-contact-names-one-face-where-its-witness-edge-lies-in-two.md`).
