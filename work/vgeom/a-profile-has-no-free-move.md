---
id: a-profile-has-no-free-move
kind: issue
title: viewer: a profile cannot be free-moved, so one drawn on the origin hides under whatever sits there
status: open
opened: 2026-11-22
priority: P1
cost: M
---

**Ev reported this** (in chat, 2026-11-22): "it'd be cool to have 'free move' for profiles so that they can be visible even if something's on the origin".

Free move is display-only, a per-space frame (`crates/viewer/src/display.rs:1373` `free_move_of(space)`; `crates/viewer/src/session/op.rs:444-499` Begin/Preview/CommitFreeMove). Before PR 7771 it was offered only to a body (citations as of then): `display.rs:607-632` (`free_move_check`) goes through `body_check` (`display.rs:478-488`, `world::builds_a_body` at `crates/viewer/src/world.rs:263-268`), and `crates/viewer/src/pane/properties.rs:636` returns before the free-move probe (`properties.rs:1803-1843`) for anything else.

A profile reads no frame (`op.rs:609-610`). Its outline is drawn on `profile.validated.plane()` (`crates/viewer/src/sketch.rs:2010-2046`), which defaults to xy (`sketch.rs:926-928`), by `crates/viewer/src/pane/viewport.rs:290-304` (`push_committed`) and `:1442-1454` (`committed_lane`) with no `DisplayView`. The pick index builds its outlines unframed too (`crates/viewer/src/pickindex.rs:1422`). So a display offset has to move both the drawing and the picks, or a moved outline stops being pickable where it is drawn.

This pairs with `a-profile-has-no-display-toggle`: both need committed profile outlines to go through the display state. No kernel work is needed. `display.rs` is shared with CHROME, OFFER and VNEWS.
