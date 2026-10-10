---
id: the-two-site-vertex-measure-reads-across-two-spaces
kind: issue
title: The two-site vertex measure test reads one vertex in a body and its transformed copy, two spaces under D10
status: open
opened: 2026-10-10
priority: P2
cost: E
---


Found off-question by the FORK-VTX designers. `test_a_measure_reads_the_placed_carrier` (`crates/pncad-py/tests/test_measures.py`) and `a_measure_at_a_transform_reads_the_placed_carrier` (`crates/editor-core/tests/m10_2_measure.rs`, the "SAME vertex name, read at the two sites" block) measure one vertex in a body and in its transformed copy, and assert the translation as the distance.

Under D10 the copy is a placement's output, and the two bodies are in two spaces unless the copy is placed against the original, so the distance either keeps its meaning (the copy is placed against the prism) or refuses as a measure across spaces. Stage 2 E only re-spells the two operands as two `Vertex` selects on two `Body` variables, with the same numbers. Stage 3 D (`transform-retires-into-a-placement`), which re-spells `Transform` as a placement, decides which.
