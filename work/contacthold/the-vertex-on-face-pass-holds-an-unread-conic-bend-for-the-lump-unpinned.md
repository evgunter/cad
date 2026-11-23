---
id: the-vertex-on-face-pass-holds-an-unread-conic-bend-for-the-lump-unpinned
kind: issue
title: The vertex-on-face pass holds a tied conic's unread bend for the lump, and no row reaches the hold
status: open
opened: 2026-10-10
priority: P4
cost: M
refs: [boolean-conic-side-code-zero-is-first-order, 4575]
---


## Finding

From PR 4575's fix pass. `vtxfac::classify_vertex_on_face` holds a
bound whose `sectors::pierce_side` refuses `BooleanDecision::ConicBend`
as its first-order `On` (tied) until the Eq. 15.3 lumps have run, and
surfaces the refusal only for a bound no lump took: a tangent sector's
lump overwrites both bounds' readings, so a bend nobody reads should
not refuse the vertex. The census on that PR (every `ConicBend` logged
across topo, sweep and editor-core) found no row that reaches the hold:
the only raises were `sectors.rs`'s own unit rows and
`conic_bend_short_of_the_band`. So nothing goes red if the hold is
taken back out. A pin wants a tangent sector whose conic bound's bend
is in the band (a sliver arc, or a spiric bound) on a lumping pair,
answering where the unheld reading refuses.
