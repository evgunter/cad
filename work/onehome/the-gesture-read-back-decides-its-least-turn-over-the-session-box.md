---
id: the-gesture-read-back-decides-its-least-turn-over-the-session-box
kind: issue
title: A gesture's read-back decides its least turn over the session box, not the copy's reach, so within about 4e-4 rad of the pole it leaves a freedom unread that evaluation would place
status: open
opened: 2026-11-15
priority: P3
cost: E
---


Filed by ONEHOME's least-turn-mint lane (fork 270's Decision 3), with the orchestrator's agreement.

**What it does.** `read_back` (`crates/editor-core/src/mate/value.rs`) reads the values a gesture writes through the chart a placement is placed by. The chart's least turn is the one decided mint (`RigidMotion::least_turn`), decided over the arm it is consumed over. At evaluation that arm is the bundle's (`bundle_arm`: the copy's reach, its written slides and its poses' distances from their origins). `read_back`'s caller (`crates/viewer/src/session/snap.rs`) hands it no arm, so it decides over `Lever::session_box()`, the sanctioned lever with no geometry in hand, which refuses widest: within about `2·box/reach` of the pole (about 4e-4 rad at the default ε, wider at smaller ε) it reads the spin and slides as `None` where evaluation, over the copy's own arm, would place the copy.

**Why it is P3.** It is conservative: a freedom `read_back` reads, evaluation reads too. And it costs no gesture today: the snap writes the nearest orientation (`Orientation::nearest`), so no gesture lands that near the pole.

**Owed.** `Charted` carries the copy's arm (its reach, read off the body the viewer holds, plus the poses' distances) and `read_back` decides over it, through the lever's sources.
