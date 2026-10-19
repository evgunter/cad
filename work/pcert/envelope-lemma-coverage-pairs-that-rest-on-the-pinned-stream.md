---
id: envelope-lemma-coverage-pairs-that-rest-on-the-pinned-stream
kind: issue
title: envelope_lemma's coverage census still finds five (arm, term) pairs only in its pinned stream, two to five rows each
status: open
opened: 2026-10-18
priority: P3
cost: E
---

Found by PCERT unit B2a's fix pass (PR 5244), and deferred from it.

## What is there

`every_arm_and_term_is_load_bearing_in_a_pinned_sweep` (`crates/geom-brep/src/pcurve_cache/envelope_lemma_fuzz.rs:1111`) is a fixed-seed coverage row (implementer discipline §8, shape 3). For every arm `derive_harmonic` selects, each of its terms must sit over the row's noise in at least one row. Under B2a its stream reshuffled: a draw that refused before derives now. At ε 1e-12 the stream lost its only rows of (cone rim, zero winding, `Centre`), and the row went red.

B2a writes down the zero-winding axis circle with its centre off the axis (`zero_winding_axis_circle`, `:1158`) on a cone and on a torus. The census takes both witnesses. The pairs below still rest on the stream alone.

The counts are the fewest stream rows carrying each pair over the noise, across ε 1e-6, 1e-9 and 1e-12, out of 3055 rows. They were read on B2a's head with a temporary probe, not committed.

| arm | term | fewest rows | what a written-down witness needs |
|---|---|---|---|
| torus parallel, zero winding | `Frame` | 2 (1e-9, 1e-12) | the axis circle on a torus whose chart frame is moved off its convention (`frame_moved`, `:432`, the stream's mode 4) |
| torus parallel, zero winding | `FidelityU` | 3 (1e-12) | the axis circle with its stored image's `u` channel drifted off the derived one (the stream's modes 1 and 2) |
| sphere parallel | `Centre` | 3 (1e-12) | a sphere parallel whose centre is moved off the sphere's axis by a few noise floors: `zero_winding_axis_circle`'s shape at a latitude, with the winding definite |
| cone rim, zero winding | `Frame` | 4 (1e-9) | as the torus's, on a cone |
| cone rim, zero winding | `FidelityU` | 5 (1e-12) | as the torus's, on a cone |

## The fix

Write each witness down beside `zero_winding_axis_circle`, and have the census take it, as it takes that one's. Then no pair rests on the stream. The `Frame` rows need a deterministic frame move, since `frame_moved` draws from the stream. The `FidelityU` rows need a fixed drift of the stored image's `u` channel. Each asserts dominance at `f64` and at `Interval`, and its term over the noise at both, at every ε row.
