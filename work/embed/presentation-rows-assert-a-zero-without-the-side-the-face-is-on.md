---
id: presentation-rows-assert-a-zero-without-the-side-the-face-is-on
kind: issue
title: Seven presentation rows of the injectivity certificate assert a curve or a line as the zero without the side the face is on, so a presentation that puts the face on the zero's other side passes them
status: open
opened: 2026-11-23
priority: P2
cost: S
---


## Finding

From `embed/cone-image`'s dual review (2026-11-23). A presentation's
zero carries the side of it the face is on: `Zero::Curve(Side, _)` from
`present_curved` and `present_wall` (`above`, off `left_is_above`), and
`Zero::Side(Side, _)` from the line readings (`face_side`'s `sigma`).
A reading that presents the face on the zero's OTHER side signs every
separator wrongly in that cell. The cone rows of `injective::focal_rows`
were red under no such flip until they asserted the side (`Zeroed` in
that module); the rows below, on a plane, a cylinder and a torus,
destructure or match the zero and never read its side
(`crates/topo/src/injective.rs`, by row):

- `another_edge_in_reach_of_the_widened_rectangle_refuses`
  (`let Zero::Curve(_, c)`);
- `a_curve_is_zero_where_its_separator_lists_its_edge`
  (`let Zero::Curve(_, c)`);
- `the_walls_closed_form_encloses_the_separator_along_its_image`
  (`let Zero::Curve(_, c)`);
- `a_line_corner_cell_reaching_the_line_from_outside_is_read_on_the_line`
  (`matches!(p.zero, Zero::Side(..))`);
- `a_chart_side_run_on_past_its_edges_end_answers_nothing_under_the_graph`
  (`let Zero::Side(_, a)`);
- `a_plane_side_is_zero_under_the_graph_only_within_its_piece`
  (`matches!(q.zero, Zero::Side(..))`);
- `the_graph_does_not_run_on_along_an_edge_that_turns_back_along_t`
  (`let Zero::Curve(_, c)`).

Not measured: whether another assertion of each row (a sign read
through `sign_on`, a body row over the same reading) is red under the
flip on its chart. The flip was run on a cone only.

## What a fix needs

Per row, the side its fixture's face is on, asserted where the zero is
matched, and the flip of `above` in `graph_read` and `present_wall` and
of `sigma` in `present_line_corner` and `present_plane_corner` run
over the whole `injective::` set and the suite file with no survivor.
