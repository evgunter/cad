---
id: face-kind-read-has-two-homes
kind: issue
title: topo::query::face_surface_kind and readback::face_carrier_kind read one tag from two homes
status: open
opened: 2026-09-04
---

DOCM-1 (PR #1829, DM2) added `topo::readback::face_carrier_kind(body,
face) -> Result<SurfaceKind, ReadbackError>` (`crates/topo/src/readback.rs`),
the typed read of a face's stored carrier tag, refusing only the two
`Dangling` arms. `topo::query::face_surface_kind(body, f) ->
Option<SurfaceKind>` (`crates/topo/src/query.rs:292`) already read the
same tag, total, for the predicate seat (`select_where`'s surface-kind
filter through `face_surface_matches`).

Two doors, one piece of geometry, two vocabularies for the empty case
(`None` versus which-lookup-came-back-empty). The readback doc cites
the query twin so the pair is acknowledged, not silent; what is NOT
decided is the home: whether the predicate seat should read through
the typed door and flatten (`.ok()`), whether the typed door should
delegate to the query one and re-diagnose the miss, or whether two
seats legitimately keep two readings (the readback module's layering
note says "one reading of any given piece of geometry, not two"). A
ruling, then one of the two delegates to the other.
