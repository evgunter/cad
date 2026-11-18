---
id: step-import-singularity-pass-reads-curve-meetings-by-hand
kind: issue
title: the import's singularity pass carries its own curve and line meeting reader beside the kernel's curve_pair::meet
status: closed
opened: 2026-11-17
priority: P1
cost: M
branch: importgate/singularity-meet
closed: 2026-11-18
---

(Found by the review of PR 7084, `importgate/knot-place`, whose C⁰-knot split reads its line against the trim through `geom_brep::curve_pair::meet`; filed by its implementer.)

## What

`crates/step-import/src/normalize/singularity.rs` reads where a ray or minted line meets a face's boundary with its own meeting reader:

- `param_on` (`singularity.rs:750`) places a point on a carrier by a foot and an `|C(t) − x| ≤ read.eps` test;
- `crossings` (`singularity.rs:761`–`897`) hand-writes circle × circle, line × circle and line × line meetings, with its own parallel threshold `NO_CROSSING_SIN2 = 1e-12` (`singularity.rs:52`, read at `:849`);
- `first_hit` (`singularity.rs:639`) is its consumer.

The kernel's home for this question is `geom_brep::curve_pair::meet` (`crates/geom-brep/src/curve_pair.rs`). It is certified at a band, has one total table of kinds (closed forms for line, circle and ellipse pairs, the walk otherwise), and refuses typed. A second reader decides the same question by different thresholds, so two imports can disagree on one pose.

## Repair shape

Read `first_hit`'s meetings through `meet` at the file's chart band. Map its refusals onto the pass's typed errors, and drop `crossings`, `param_on` and `NO_CROSSING_SIN2`. Check `tests/chart_singularity_vertex.rs` and the pole and apex rows (`halfcap_pole`, `poleguard`, `poles_inside_a_seam_region`) before and after.
