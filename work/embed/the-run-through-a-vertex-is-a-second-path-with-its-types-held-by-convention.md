---
id: the-run-through-a-vertex-is-a-second-path-with-its-types-held-by-convention
kind: issue
title: The run through a vertex is a second path beside the corner block, and several of its types are held by convention: the structure items of PR 7320's review, not done
status: open
opened: 2026-11-23
priority: P2
cost: M
---

## Finding

From lane `embed/tangent-corner` (PR 7320, 2026-11-23): what its two
reviews asked of the structure of the run through a vertex in
`crates/topo/src/injective.rs` and the lane did not do. None is a
soundness finding; each is a place the next reader has to hold a
convention in their head.

- **A second path.** The run is a branch inside `graph_read`'s corner
  block (`if th.corner.run || l.leaves != Leaves::Face`), beside the
  old reading. The sentence that says which of the two a corner gets
  is spread over that branch, `leaving` and `present_plane_corner`'s
  fallback. Wanted: the sentence at the branch, and
  `Leaves::Undecided` / `Leaves::Reflex` read by one `match` (today
  they are folded for the reading and told apart, two lines on, for
  the typing).
- **`leaves_graph`** survives as a wrapper over `leaving` for
  `present_wall` alone.
- **The things named `run`.** `Graph::run`, `Corner::run`,
  `Curved::run` (`RunOn`), `On::run` (`RunEnd`), the `runs` of a
  curve's pieces and `run_on` (a side's edges past a corner) are six
  different things. `RunEnd` is `RunOn` plus the vertex and its
  parameter.
- **`Curved` holding two edges by convention.** In the `Corner::run`
  case the zero's curve is the conic and its part on the shared cell
  is the line's; `sign_on` keeps it off `Curved::psi`, and
  `a_run_is_not_zero_under_the_plane_of_its_conic` pins that, but no
  type does.
- **One row guards five mutants.** `the_graph_runs_on_along_a_runs_other_edge`
  is the only red row for three of the lane's mutants (a run zero
  under a graph not run on; the slopes its own edge's alone; a run
  `stuck`) and one of two for two more (the graph's own edge not
  stopped at the vertex; a side read to the vertex whatever it stops
  at). It should be several rows.
- **The proof's steps** at `present_corner` ("One run through the
  vertex") are not marked as guard, type or prose.
- `docs/predicate-dimension-audit.md` names `leaves_graph` and does
  not list the quantities `SELF_INTERSECTION_COLUMN` and
  `SELF_INTERSECTION_GRAPH` gained (the lane was not to touch `docs/`).

## What is needed

The items above, as one change with no behaviour in it: the 14 mutants
of PR 7320's body red on the same or finer rows afterwards.
