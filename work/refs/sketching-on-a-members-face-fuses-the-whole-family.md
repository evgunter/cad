---
id: sketching-on-a-members-face-fuses-the-whole-family
kind: issue
title: Sketch on a face of one member of a family fuses the boss with the whole family
status: open
opened: 2026-11-10
priority: P3
cost: E
design: true
---

Found by the implementer of PR 6555. `Recording::sketch_on_face` (`crates/editor-core/src/edit.rs`) takes the face's body as a `BodyRead`, so a face of `xs[i]` is read in that member by its own name and the boss's mate side reads `xs[i]`. The union the gesture writes reads the body's variable whole, `Union([xs, boss])`, so the boss fuses with every member of the family, not with member `i` alone. The viewer's sketch-on-face reaches this from a pick on a drawn pattern (`crates/viewer/src/session.rs` `sketch_on_face`).

Which is meant is a design question: the boss joins the family's union (today), joins member `i` alone (a `Union([xs[i], boss])`, which leaves the family's other members apart), or the gesture refuses on a family. Done when one is chosen and pinned.
