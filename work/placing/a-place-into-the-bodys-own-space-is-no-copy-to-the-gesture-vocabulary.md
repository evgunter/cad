---
id: a-place-into-the-bodys-own-space-is-no-copy-to-the-gesture-vocabulary
kind: issue
title: A placement Place makes stands in its body's own space, which the gesture vocabulary does not count as a copy
status: open
opened: 2026-11-08
priority: P1
cost: M
---



Found by `placing-place-live` (the unit `the-placement-editor-commits-on-apply-not-live`), building Place.

**What.** `SessionOp::Place` inserts its placement into the body's own space (`DocSession::place`, `crates/viewer/src/session.rs`), as the Transform tool's Create door did before it (README, *The placement editor commits a placement's whole bundle*). The gesture vocabulary's copies are the placements into a *copy space* (`world::copy_spaces`, `crates/viewer/src/world.rs`): the world, or a `Space` no placement's source stands in. The body's own space holds the body, so it is no copy space, and a placement Place makes is not a copy there:
- `world::copy_source` answers `None` for it, so a pick on the copy is seated as the copy itself, not as the body it places;
- `world::placements_of` and `world::is_placed` leave it out, so the feature tree's placed badge does not mark the body (traced, not run).

**Seen.** The placement editor's `on` side read the copy's own output and the door refused the mate `MateFault::NotOnTheCopy`. The unit patched the editor's own read (`matetool::on_read` falls back to the source read for a `Place` `copy_source` declines) and wrote its rows against `doc.placements()` filtered by `copied_read`, not `placements_of`.

**Owed (design, light).** Decide whether a placement in its body's own space is a copy to the gesture vocabulary (widen `copy_spaces`) or whether Place should place into a space of its own (a `new_space()`, as an inserted part is). Either way the seats, the badge and the editor's `on` read then agree, and `on_read`'s fallback goes.

**Why not in PR 6578's fix pass.** `world::copy_spaces`' doc states the rule on purpose: a placement into a space whose bodies a placement copies out — "a body moved against its own geometry, a pattern in its body's own space" — "is a relation inside that space, and a gesture reads it as the node it is". Counting an own-space placement as a copy would flip that for a pattern's family and every moved body too: takeovers, seats and the badge would all change for them. The other road, Place into a `new_space()`, moves where Place stands its copy. Either is a design choice about the gesture vocabulary, not a one-line widening.

**Done when** a row Places a body, twice, and reads: each copy is badged placed (`world::is_placed` true, `placements_of` lists both), a pick on either copy seats the body it places (`copy_source` answers it), and the placement editor's `on` side reads the body without `matetool::on_read`'s fallback, which goes; the rows that pin a pattern's own-space family as a relation stay green, or are moved with the ruling that says why.
