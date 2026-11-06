---
id: a-point-against-a-pieced-curve-is-read-by-three-walks-and-sorted-twice
kind: issue
title: A point against a pieced curve is read by three clearance walks with three stop rules, its site refusal sorted at two sites, and the census spells the walk's refusal its own way
status: dispatched
priority: P3
opened: 2026-11-06
branch: onecopy/spline-walks
---


Found in the review of PR 6548 (`onecopy/spline-walks`). There are three parallels.

**Three clearance walks of a point against a pieced curve, with three stop rules.** All three walk the carrier's pieces through `geom::Curve3::point_walk` or `geom_core::bisect`, and each decides for itself when a piece settles and when the walk stops:
- `geom::Curve3::clear_of` (`crates/geom/src/curves/on_carrier.rs`): settles a piece whose `gap − reach` clears the band (`curve_carrier_piece_clear`). It stops `Near` at the first piece that does not clear once its reach is within the band (`curve_carrier_piece`).
- `topo::splitting::containment` `ArcPieces::clear`: stops at the first piece whose own gap fails the caller's `clear`. It settles a piece whose `gap − reach` clears, and has no reach floor.
- `topo::splitting::pieced_arc` `contact`: stops `On` at a piece whose on-point reads zero. It settles a piece at its reach floor (`rows.leaf`), keeping the first in-band reading, and caps its depth at `MAX_DEPTH`.

They read different balls (a carrier's speed-bound reach, against `pieced_arc`'s ball of the piece). So whether one stop rule can serve all three is a question to settle, not a given.

**Two sorts of `geom_brep::SiteRefusal`** into topo errors:
- `topo::boolean::join::site_refusal` maps it to `BooleanError`.
- `topo::chord_join` `carried_chord_spec`'s `place` closure matches it inline into `SplitJoinError`.

The `Unwalked` arm reaches the same `SplitJoinError::SectionUnwalked` both ways, but the `Escalated` and `Desync` arms are sorted twice. A `From<SiteRefusal> for SplitJoinError` (with the face) would be the one home.

**The census's own spelling of a walk's refusal.** `topo::census::curved` `on_interior_at` (`crates/topo/src/census/curved.rs`) maps `geom::PieceWalkRefusal` to its own `WALK_EXHAUSTED` / `WALK_UNBOUNDED` strings, which it then parses back (`Why::Exhausted` / `Why::Unbounded`). It sits beside `PieceWalkRefusal`'s `Display` and `recourse`. It runs parallel to the join's rendering, not through it.
