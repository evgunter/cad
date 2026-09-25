---
id: set-surface-source-stamps-a-recipe-without-checking-the-descriptions-agree
kind: issue
title: set_surface_source stamps a GeomSource on any live key without checking the description agrees with the recipe's other holders, while GeomSource equality now licenses a row carry at five doors
status: open
opened: 2026-09-24
priority: P3
cost: D
---


Found by both reviewers of PR 2603
(`mef-and-kef-move-half-edge-runs-between-charts-and-leave-their-rows`)
while measuring `Body::same_chart`'s rungs; filed by that PR's fix
pass. No row on this slate covers it:
`two-provenance-free-keys-holding-one-surface-read-as-two-charts` is
the opposite direction (equal descriptions the channel cannot see as
one), and `graft-copies-provenance-keys-verbatim` is about keys, not
recipes.

**The door.** `Body::set_surface_source` (`crates/topo/src/body.rs`)
stamps a `GeomSource` on any surface key that resolves, replacing
whatever origin the key had. It reads the recipe and the key; it does
not read the surface, and it does not ask whether another live key
already carries the same recipe over a different description. Its doc
is about precedence over `Imported`, not about agreement.

**What now reads the stamp as a licence.** `same_chart`'s second rung
— two keys carrying one `GeomSource` are one chart, by N6's source
theorem — is what carries pcurve rows across a key change at five
doors: `kfmrh`, `mfkrh` and `ring_move` through
`Body::drop_rows_on_chart_change`, and `mef`'s chord surgery and
`kef`'s unsplice at their own sites (PR 2603); `set_face_surface`
joins them when PR 2594 lands. A row is a curve stated in a chart, so
a stamp that lies moves rows onto a face whose surface they are not
about, and nothing between the stamp and the carry checks.

**Measured** (R1 of PR 2603, on the sheet of
`crates/topo/tests/loop_reparenting_pcurve_rows.rs`): the sheet's
cylinder key and the PLANE's key given one recipe each, then `mef`
with `Shared(plane key)` — two cylinder rows carried onto a planar
face, `(2, 1)`, and the pass says nothing because a plane mints
nothing. Test-only today: every production stamp is the recipe
layer's, over a description it just evaluated. The theorem the rung
trusts is "one recipe evaluates to one description", and this door is
the one place that theorem can be made false from outside the recipe
layer.

**What would close it.** Either the door checks: a second live key
carrying the recipe must hold an equal description (structural
equality of the `Surface` — which needs the `Bounds` question
`two-provenance-free-keys-…` already asks, or byte equality as a
weaker floor), refused typed when it does not; or the door's doc
states the theorem it relies on the caller for, and the test-only
callers that stamp two unequal descriptions with one recipe are
audited. ORIGIN's file (`source.rs` owns `GeomSource`, and this door
is its post-op attachment), so filed here; the doors that trust the
stamp are TOPO's.
