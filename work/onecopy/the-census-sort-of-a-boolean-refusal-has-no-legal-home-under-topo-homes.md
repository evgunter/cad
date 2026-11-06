---
id: the-census-sort-of-a-boolean-refusal-has-no-legal-home-under-topo-homes
kind: issue
title: The census's sort of a boolean FaceUnread has no legal home under [topo.homes]: pub(crate) in boolean is refused by engine-leaves, pub by unreachable_pub
status: open
opened: 2026-11-03
priority: P2
cost: E
refs: [6451]
---

Found in PR 6451, which raised `census.rs`'s `names-boolean` pin from 506 to 510 in `scripts/gates/engine-leaves.counts`, in plain sight.

`census.rs` `line_unread` sorts a `crate::boolean::FaceUnread` (why one face left a tangent-line station or its boundary unread, `boolean/mod.rs`) into the census's `ValidationError`s. PR 6451 added two arms to it (`FaceUnread::Boundary`, `FaceUnread::Uncertified`), and those raised the pin.

`[topo.homes]` (`crates/topo/README.md`) says code naming an engine's item belongs to that engine, so the sort's home is `boolean/`. It cannot be written there:
- an engine may declare nothing `pub(crate)` (`engine-leaves.sh`'s crate-visible kind), so `pub(crate) fn census` on `FaceUnread` is refused;
- a `pub fn` on `FaceUnread`, a `pub(crate)` enum, is refused by clippy's `unreachable_pub` (`-D warnings`);
- a `pub` door re-exported at the crate root would need `FaceUnread` and `contain::PlacementError` (both `pub(crate)` today) in topo's public API, or `private_interfaces` refuses it. That would publish crate-internal plumbing of the census confirm as product surface.

The sort also uses the census's private `pair_lane_refused` and `read_containment`.

So the census–boolean seam has no door shape for a crate-internal sort of an engine type. The question is which way it should go:
- `[topo.homes]` admits a crate-internal door, a `pub(crate)` item the gate accepts where only the root's non-engine modules consume it;
- the census confirm's tangent-line reading moves wholly into `boolean/` and returns crate-root types;
- the pin stands as the record.

This is a design question on a ratified page (PR 6426), so it goes to Ev's channel before any code moves.

