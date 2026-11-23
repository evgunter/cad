---
id: a-spline-section-is-swapped-for-its-chord-after-it-is-certified
kind: issue
title: rebuilt_in_surface replaces a certified spline section by the chord between its built ends on a 5-sample read, after on_description, so the carrier the body holds is not the one certified
status: open
opened: 2026-11-21
priority: P3
cost: E
---


Found by EXCH unit D's second reviewer (PR 7304). The same happens on
main.

In `build::spline_pair` the built spline section is certified against the
printed carrier (`on_description`, `crates/step-import/src/build.rs:734`)
and then flagged `between_ends` (`:738`). After the vertices are built,
`build::build` (`:296`) calls `rebuilt_in_surface` (`:2544`). That
replaces the section by the straight segment between the built ends
wherever the section's distance from that chord, at five samples, reads
Zero at the file's band.

So the carrier the body holds (the chord) is not the one the printed
carrier was certified on (the section). Its distance from the section is
read only at five samples, never certified over the edge. Adoption then
certifies the chord at ε against the surfaces. The edge is therefore not
wrong at rest, but the file's printed carrier was never verified against
what ships.

Fix: certify the printed carrier on the chord too (the cover arm of
`on_built::printed_on` serves a line), or certify the section-to-chord
distance by the same cover read rather than samples.
