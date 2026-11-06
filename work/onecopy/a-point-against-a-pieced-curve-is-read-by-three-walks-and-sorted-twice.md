---
id: a-point-against-a-pieced-curve-is-read-by-three-walks-and-sorted-twice
kind: issue
title: A point against a pieced curve is read by three clearance walks with three stop rules, its site refusal sorted at two sites, and the census spells the walk's refusal its own way
status: closed
priority: P3
opened: 2026-11-06
closed: 2026-11-06
pr: 6571
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

## Outcome (`onecopy/spline-walks`)

**Correction: the three walks were never one question.** They already share one home with the stop rule as a parameter: `geom::Curve3::point_walk`, whose visit is the stop rule. Each now states its question at its site, pointing at the others:
- `clear_of` decides against the band, logged, and returns the deciding reading.
- `ArcPieces::clear` reads the caller's unlogged lever as a bool.
- `pieced_arc::contact` places the point (End/On/Off) on its own ball cover.

**The two `SiteRefusal` sorts stay two vocabularies:**
- chord_join's inline match is now one `From<(FaceKey, SiteRefusal)> for SplitJoinError`.
- `join::site_refusal` states why the Boolean keeps its own: its in-band site is the section's coincidence decision, with that decision's recourse.

**The census's parallel spelling is narrowed, not gone:**
- `census::curved` pushes `geom::PieceWalkRefusal::sentence()` as its `ContactRefusal::NotCertifiable { what }`, and reads it back with `PieceWalkRefusal::of_sentence`. That parse runs over every variant (`strum::VariantArray`).
- `link::EdgeTangentUnread`'s `Exhausted`/`Unbounded` arms are now one `Unwalked(geom::PieceWalkRefusal)`. Its ladder standing is `geom_brep::ladder::Refusal::of_walk`, stated once: exhausted is unsettled, unbounded is a limit.
- Pinned by `census::curved::tests::an_unwalked_edge_reads_back_as_its_walk_refusal`, which checks the `what` text and the round trip, and by `census::link_faces_ranking` for both standings.
- Nothing rendered moved: `NotCertifiable` renders no `what`.

**What is left:** the census still identifies the walk refusal by **string equality on `what`** (`edge_tangent_refusal`), as it does for its other `NO_*` reasons. A typed `what` on `ContactRefusal::NotCertifiable`, or a typed census refusal, would end the round trip.
