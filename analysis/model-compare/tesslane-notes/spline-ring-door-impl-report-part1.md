TESSLANE implementer report: spline-ring-door. Final head 9f0e382e4a5175109e8ac61d181f9e61b6250b3b on tesslane/spline-ring-door (pushed).
    
    1. MEASUREMENT
    - Compiled at the root, the trim and the cells reach no engine. They read crate::pcurves, chart_region::spline_chart_arms and its gate, validate::decide, live::proven, joint::Winding and the keys. splitting::Escalation was used only by the two placements that stay.
    - What in the engine reaches them:
      - spline_region's two placements (Trim::of, Trim::place, bezier_cells);
      - carrier_walk (Patch floor/extent/quartered/rect, Rect, Caps);
      - boolean/mod.rs decision_words (the two names);
      - the rows (Trim/Patch/Piece insides, point_rounding, bezier_pieces, image_pieces).
      reduce.rs reads only the rows' fixtures.
    - Where the spec was wrong:
      - CELL_VISITS moves with Caps; it was not a root bound.
      - CELL_DEPTH and keep stay.
      - The spec's list missed Control, tracks, bezier_insertions, close_gap, walks_forward, half_width and the mutant hook. All of them move.
      - The ONEDOOR row needs no edit: it cites the error, Escalation and Uncrossable, which stay.
      - Nine more open rows on other slates cite moved items. All are updated (list in the PR body).
    - A judgement call, not a stop: the cells take no Body, so [topo.homes] rule 1 alone would put them in geom-brep. They move with the trim, which meters by them. SPLICE's open row the-spline-chart-cell-walk-is-written-twice already owns moving the cells to geom-brep's one home. If you read rule 1 as binding here, that is the question to raise.
    
    2. WHAT CHANGED
    - c9c9fdf6b6, the move:
      - New root module crates/topo/src/spline_trim.rs. The engine imports from it; the rows-only names come in through a #[cfg(test)] use.
      - spline_region_rows.rs is untouched in this commit.
      - Gate tables: engine-leaves crate-visible spline_region 11→5; bare-rounding's 8 re-pose sites move to the new file.
    - 23d46f06e1, the rename: spline_trim_clear and spline_trim_arm. The roster takes the new consts and lists both names as BOOLEAN's exceptions (first door to raise them). The rows' edit is the const rename only.
    - ff3e91ca76, the door reads a spline face:
      - TrimChart::{of, chain} are factored out of Trim::of. Trim::ring_shift is the ring rule over the trim, and with_rings now uses it.
      - spline_trim::ring_decks is the door's spline arm: the outer loop's trim built from the lift the door already read, and each ring placed by ring_shift. The door's deck and the trim's are one decision.
      - SplineChart retires for SplineTrimUnread (placeholder chart, no cells, stated≠decided period, or a chain with no pieces).
      - README rows updated.
      - The deck row is the_ring_door_moves_a_spline_faces_ring_into_its_outer_loops_region in spline_region_rows.rs. It uses the hand-built holed_sheet on square_tube, a sheet across the seam; that fixture lives there, and engine→root is allowed.
      - Measured: the outer lift spans u∈[−0.1,0.1]. The hole stored at [0.02,0.06] gets deck 0; the hole stored at [0.92,0.96] gets deck u=−1. The row asserts both decks, and that each moved ring lies inside the outer loop's u hull.
      - Forcing the deck to IDENTITY reds it, and so does negating it.
    - 90341a65eb, the one loop:
      - pcurves_rings.rs ray_parity_as over the ChainPiece trait (boxed→PieceBox{gap:[T;2], extent}, ends, halves), plus RayQuery and gap.
      - The analytic Piece (chart_box, gaps in T) and the spline Piece (rounded control box, from_computed f64 gaps) implement it. Outer::side and Trim::copy call it with their own names.
      - Place retires for Side, which touches the engine and the rows mechanically. The roster gains the funnel carrier and its FORWARDS site.
      - An inconsistent half-open mutant reds both families' rows: the analytic reader row plus two spline vertex rows.
    - 8872e608f5 and ad90bea652: the citing rows, and the filing below.
    - 9f0e382e4a: merge of origin/main (clean).
    
    3. PR BODY
    =====
    $(PR body follows verbatim)
    =====
    [The body is long. It is the file content below.]
    
    
</cross-session-message>

0 notifications remain queued.
