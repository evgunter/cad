---
id: a-conic-locus-refused-member-gets-no-located-window
kind: issue
title: On a non-line span the boundary pass's undecided window is the whole span, so a refused conic edge piece is read In-or-refuse
status: open
opened: 2026-11-03
priority: P2
cost: E
refs: [6477, a-boundary-touch-refusal-locates-no-window]
---


Found by the full review of PR 6477 (`onecopy/meet-reads`), NOTE-4.

`geom_brep::curve_pair::Span::window_near` locates a refused member's window only on a line span. It takes the member cover's range along the line (`Cover::range_along`), widened by the escalation either way. On any other carrier it returns the whole span.

The line case covers every `boolean::boundary_cuts` call, since the tangent locus is a line. It does not cover `boolean::overlap::edge_witness`, which reads each boundary edge of either face, conics included, as the span. There a conic edge with one refused member of the other face's boundary has a window over the whole edge. So every piece of that edge is a witness only where it reads strictly In, and refuses otherwise. That is loud and never wrong, but narrower than the reading owes: the member's cover bounds where it can meet the conic, too.

Fix: give `window_near` a conic arm. The span's parameters whose points stand within the escalation of the member's cover are an arc set. The cover's `distance_range` from the conic's frame, or the conic's own extreme reading against the cover's ball, bounds it. Then add a row in `curve_face/rows.rs` beside `a_refused_members_near_points_lie_in_a_window`, on an arc span.
