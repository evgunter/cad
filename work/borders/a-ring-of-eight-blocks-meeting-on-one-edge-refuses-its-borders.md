---
id: a-ring-of-eight-blocks-meeting-on-one-edge-refuses-its-borders
kind: issue
title: A union of eight unit blocks turned about an edge they share refuses naming: a face piece held as several borders
status: open
opened: 2026-10-28
priority: P2
---

Found by INTENT stage 3 G's review (R1, PR 5954). The defect predates G.

**What happens.** Take a unit block `[0,1] × [0,1] × [0,1]` and turn it `k/8` of a turn about the z axis, for `k` in `0..8`. Every copy meets every other along the z axis edge, so every pair is judged (28 of 28). The union of the eight copies refuses `Naming(Emission { what: "a piece of a face held as several borders no recorded discard between them" })`, raised in `crates/editor-core/src/names/borders.rs:466`.

**Where it lives.** It is not G's. A spelled union of the eight turned copies refuses identically on main at `a195f777` and at G's head. The family form of the same ring refuses identically too (`g_family_review_rows::a_ring_of_unit_blocks_meeting_on_its_axis_judges_every_pair` pins it). Rings of 5 and 6 build one solid on both paths.

**Done when.** The ring of eight builds one solid, and the row's `n = 8` arm is moved from the refusal to the build.
