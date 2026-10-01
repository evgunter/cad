---
id: boundary-on-the-new-chart-has-two-homes-in-the-attach-doors
kind: issue
title: whether a moved face's boundary lies on its new chart has two homes in the attach doors, and they disagree on curved charts and the lone vertex
status: open
opened: 2026-10-01
priority: P3
cost: M
refs: [set-face-surface-passes-a-swap-off-the-faces-own-boundary, mef-and-mfkrh-onto-a-new-chart-strand-the-edges-they-move, validate-tier3-curved-boundary-containment, 3598]
---

## What

Found by the review of PR 3598 (style Q1). The question "does a moved
face's boundary lie on the chart it moves onto?" is answered twice in
`crates/topo/src/attach.rs`, by two different proxies:

- **`Body::set_face_surface`** (keys-only) reads it from keys. In
  `Body::rechart_edges`' walk, a certified edge on the face is vouched
  for when its description names the key the face wears after the move
  (`Sides::vouched`). The door refuses `RechartUnvouched` otherwise.
- **`Body::set_face_surfaces_describing`** reads it from band residuals
  (`Body::check_moved_boundary`): every vertex and interior
  certification sample against the new plane, refused as
  `RechartOffBoundary` / `RechartBoundaryEscalated`.

They disagree in two places.

- **Curved charts.** The keys-only door asks, and refuses every curved
  move its edges do not name. The describing door asks nothing on a
  curved chart, so when it is handed no re-descriptions it takes a move
  off the boundary unchecked. Pinned by
  `attach::tests::a_curved_swap_is_refused_keys_only_and_taken_unchecked_by_the_describing_door`.
  That residual is #638's
  (`work/restfront/validate-tier3-curved-boundary-containment`).
- **The lone vertex.** The describing door asks an empty loop's vertex.
  The keys-only door does not, and passes nothing tier 3 flags, since
  tier 2 bans empty loops at rest. The review's probe: an `mvfs` seed
  moved onto a plane at z = 5 is `Ok` keys-only and `RechartOffBoundary`
  describing.

## Why it matters

`mef_chords`, `mfkrh_with` and `ring_move` need the same answer (the
off-boundary half of
`mef-and-mfkrh-onto-a-new-chart-strand-the-edges-they-move`). A third
and a fourth home would drift the same way. The fix is one function that
answers "is this moved boundary vouched for on its new chart", reading
keys where no band is in hand and residuals where one is, with the two
disagreements above decided once.

## Adjacent, older

`Body::description_surfaces` (`crates/topo/src/body.rs`) and
`Named::keys` (`attach.rs`) are two spellings of "the keys a description
names". They predate PR 3598. Fold them together when this row is taken.

## Evidence: the Euler doors take the keys home (PR 3673)

`mef_chords` / `mef_lone`, `mfkrh_with` and `ring_move_with` now ask
the keys-only question through `Body::vouch_move`
(`crates/topo/src/attach.rs`), which `set_face_surface` calls too: one
walk (`Body::rechart_edges`, now over the edges a door hands it and a
per-half-edge move) and one decision (`Sides::vouched`), which also
answers for the chord `mef` mints (it has no key yet, so it is asked
as an edge whose two halves lay on the parent's chart). So the keys
reading has one home across five doors; no door added a residual
reading. The residual reading is still the describing door's alone
(`check_moved_boundary`), and the two disagreements this row names —
curved charts and the lone vertex — are unchanged. `kef` and `kfmrh`
ask neither yet
(`kef-and-kfmrh-across-keys-want-a-describing-door-or-reordered-callers`).
