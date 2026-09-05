---
id: face-kind-read-has-two-homes
kind: unit
title: topo::query::face_surface_kind and readback::face_carrier_kind read one tag from two homes
status: review
opened: 2026-09-04
branch: topo/two-homes-face-kind
pr: 1959
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

## Re-homed (2026-09-04)

Moved from `work/issues/` to `work/topo/` in the tracker-wide
re-home sweep of 2026-09-04 (Ev's direction, in-chat), which read every
open `work/issues/` file against every open program's `paths` and
against the code-quality K–X fences. Id, body and header are unchanged;
the directory is the claim (`work/README.md`). Any `## Home` section
above naming `work/issues/` is superseded by this line and is kept as
the record of why the file was parked there.

## Ruling proposed (TOPO, 2026-09-05) — for Ev

The two doors, re-read on main `c5137c6f`:

- `topo::readback::face_carrier_kind(body, face) -> Result<SurfaceKind, ReadbackError>`
  (`crates/topo/src/readback.rs:359-370`): face lookup, surface lookup,
  `SurfaceKind::of`; refuses `Dangling` naming WHICH lookup missed. The
  module header (`:40-46`) states the rule: kernel doors are the one
  reading, and twins elsewhere "delegate here, so there is one reading
  of any given piece of geometry, not two".
- `topo::query::face_surface_kind(body, f) -> Option<SurfaceKind>`
  (`crates/topo/src/query.rs:306-310`): the same two lookups and the
  same `SurfaceKind::of`, flattened to `None`; consumed by
  `face_surface_matches` (`:332`). And a THIRD spelling in the same
  file, `face_kind_across(body, he)` (`:314-319`): half-edge → loop →
  face → surface → kind, feeding the edge-kind predicate (`:350-351`).

**Recommendation: (a) — the predicate seat reads through the typed
door and flattens.** `face_surface_kind` becomes
`face_carrier_kind(body, f).ok()`, and `face_kind_across` resolves the
half-edge to its face (the two lookups that are its own) and then calls
the same door. The predicate's `None` is a legitimate consumer choice
("no carrier is an honest no", `query.rs:294`) and flattening a typed
refusal loses nothing a predicate wants; the lookups run once; and the
readback header's rule is already ratified text, so this is its
faithful elaboration rather than a new decision.

Why not the others. **(b)** the typed door delegating to the query one
and re-diagnosing the miss re-runs the lookups to learn which failed —
two readings in disguise, with the diagnosis reconstructed after the
fact. **(c)** two seats keeping two readings is exactly what the
readback header says the crate does not do; the `readback` doc's
citation of the query twin acknowledges the pair, it does not license
it.

Scope of the edit, if ratified: `crates/topo/src/query.rs` only (two
function bodies; no signature changes; the predicate rows at
`query.rs:1181-1182` keep their `None` assertions), announced on
SEAT's board and taken there by TOPO as a one-door seam, or by SEAT if
it prefers — SEAT's call. No `readback.rs` change. `edge_carrier_kind`
(`query.rs:296`) has no readback twin today and is out of scope.

Kind stays `issue` until ratified; then it becomes the unit above.

## Ruled (2026-09-05, PR 1948)

Ev: "ok cool sounds good" — (a) ratified, after confirming it is the
same shape as SEAT's direction-normalization ruling (PR 1902: one
kernel door, callers keep their vocabulary). The item is now a unit:
`query::face_surface_kind` becomes `readback::face_carrier_kind(body, f).ok()`,
and `query::face_kind_across` resolves the half-edge to its face and
calls the same door; no signature changes; the predicate rows keep
their `None` assertions; `readback.rs` untouched. Branch
`topo/two-homes-face-kind`; single style review; no A/B row. Landed by
TOPO as a one-door seam on `crates/topo/src/query.rs` (SEAT's file),
announced on SEAT's board.
