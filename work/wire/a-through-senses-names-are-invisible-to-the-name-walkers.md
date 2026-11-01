---
id: a-through-senses-names-are-invisible-to-the-name-walkers
kind: issue
title: Sense::Through's entering names are invisible to select.rs name_args and resolve's embedded walk, and Sense::other_half keeps a Through that names neither half
status: closed
opened: 2026-10-26
priority: P3
cost: E
closed: 2026-10-27
---


Found by PR 5758's FULL review (2026-10-26), by inspection.

**Walkers that skip the new names.** `select.rs:429` (`name_args`) and `resolve/mod.rs:2826` (`embedded`, which feeds `walk_names`) both match `Crossing { edge, face, .. }`, so the names inside a `Through { entering }` are invisible to them.
- `select.rs` documents that "a … variant embedding names must be classified here or the compile breaks". The new names went into a field of an existing variant, so nothing broke.
- This is harmless today only because each `entering` name repeats a name the edge or line already embeds.
- Sweep `attribute.rs`, `derivation_nodes` and `read_edge` for the same class.

**`Sense::other_half` (role.rs).** It silently keeps `self` when a `Through` names neither of the seam's sides, and for a `Through` on a non-seam line. Its doc says "A `Through` on any other line stays", which is wrong as a rule: what enters one half leaves the other. This is unreachable today, since a split's crossed edge reads `Stored`.

**Small style residue from the same review:**
- `canonical::form` and `minted_segment` are one-line forwards to private functions.
- `RemapError` now has one variant, destructured three times.
- `EdgeSense.sense` drops `edge_reading`'s error (`.ok()`), and `sense_of` then re-raises it as `UNSIDED`.
- `Through { entering: [] }` reads as "entering along none of its things".

Closed by PR 5870 (2026-10-27). `embedded` walks a `Through`'s names as partners, on the Include walk only. `name_args` classifies the sense, because a pattern has no sense axis. Both are destructured field by field. `other_half` returns `Option` and refuses `Unwalked::Misplaced` where it cannot complement.
