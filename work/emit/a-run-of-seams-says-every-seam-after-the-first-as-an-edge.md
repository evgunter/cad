---
id: a-run-of-seams-says-every-seam-after-the-first-as-an-edge
kind: issue
title: The words for an EdgeCrossing run of seams say every seam after the first as 'an edge runs through', so two seam-run vertices can read alike
status: open
opened: 2026-10-13
priority: P3
cost: E
---


## What

The designer pair on fork-log row 130 found this; it is flagged and not yet measured.

`words.rs`'s `EdgeCrossing` arm says the first seam in a run by its faces. Every seam after the first it says only as "an edge runs through". So two vertices whose runs share a first seam and differ in the rest can read alike to a person.

The converged recommendation keeps exactly such vertices, for example the three-seam run where `c` cuts the glued rim.

**Owed:** words that say each seam in a run by its faces, and a `name_words_corpus` row that would catch two runs reading alike.
