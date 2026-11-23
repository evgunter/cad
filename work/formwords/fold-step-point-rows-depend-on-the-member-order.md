---
id: fold-step-point-rows-depend-on-the-member-order
kind: issue
title: A fold's step rows at shared vertices depend on the member order, for union and subtract alike
status: open
opened: 2026-11-23
priority: P3
cost: M
design: true
---


Found by the dual review of PR 7426 (a subtract's tool is a `Bodies`
seat), whose rows `review_subtract_bodies_a::every_order_of_the_tools_decides_and_records_alike`
and `a_fold_steps_rows_keep_their_relations` were narrowed to the
judged rows over it.

An n-ary fold (`eval/wire.rs` `fold_members`, over
`topo::boolean::nary` `run_members`) publishes two kinds of row
(`coincide::RowOrigin`): the pairwise judgements' rows, which are the
same in every member order modulo the members' places, and each fold
step's rows, which DM4 lets follow the order. The second kind is not
only bookkeeping: at a vertex where three or more members' faces meet,
which step decides the point, and so which member cells the
`Meet @ VertexFusion` row names (`in_member_space`), follows the list.

Measured on review A's "holes sharing faces" scenario (a plate and
three tools whose faces are shared pairwise):
- the subtract `plate − [t0, t1, t2]`: the order `[0, 1, 2]` publishes
  four four-cell `Meet` rows (an edge of `t2`, a plate cap, a face of
  `t0`, a face of `t1`) where `[0, 2, 1]` publishes six three-cell ones;
- the union of the same plate and three tools, probed over every order
  of the tools: rows differ from the first order's in 4 of the other 5
  orders. So this is the shared fold's property, not a subtract's.

Names, bodies, the judged rows and the records are the same in every
order; only the fold-step point rows move. Making them order-free means
changing the union fold itself: for example, deciding every
multi-member vertex once over the finished body, as `name_union`'s end
pass already does for the names, rather than at the step that first
meets it. That is a design question for the fold, and for what a row
at such a vertex is to name.
