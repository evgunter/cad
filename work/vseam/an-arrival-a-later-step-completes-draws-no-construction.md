---
id: an-arrival-a-later-step-completes-draws-no-construction
kind: issue
title: a bare fillet, whose arrival a later step completes, draws no construction
status: open
opened: 2026-11-02
priority: P3
cost: M
refs: [a-fused-fillet-verb-draws-nothing-of-its-parts-so-a-refusal-is-unreadable]
---

A fused fillet step (`fillet_arc`, `arc_fillet`, `arc_fillet_arc`)
draws the two sides its fit is handed and the fillet circle it finds,
recorded at the call that hands the fit its sides
(`Core::fit_arc_fillet` in `crates/profile/src/path.rs`, read by
`profile::path::program::replay_drawn`). That covers every arrival a
later step completes for those verbs — `arc_fillet`'s line arrival,
which the next leg or `.at(..).toward(..)` binds, and a `Radius` or
`Via` arrival awaiting its binder — on success and on refusal.

What is left is the bare `fillet(r)`. Its two sides are both lines,
and its arrival is wholly a later step's: it resolves line×line in
`PartialPath::resolve_fillet` (`crates/profile/src/path.rs`), whose
only discrete record is `Guide::line_fits`, and records no
construction. So when that resolution refuses — "this corner lies
behind the incoming ray's start", "… behind the arrival side's
anchor", parallel carriers — the incoming leg is drawn as the kept
prefix, and neither the ray the fit read nor the arrival line it was
handed is on screen.

A fix records the two rays at `resolve_fillet` the way
`fit_arc_fillet` records arc sides — before the corner is derived, so
a refusal keeps them — and widens the record (today named for fused
steps, `FusedConstruction`) to say which verb it is for.
