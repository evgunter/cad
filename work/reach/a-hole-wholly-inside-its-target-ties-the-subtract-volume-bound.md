---
id: a-hole-wholly-inside-its-target-ties-the-subtract-volume-bound
kind: issue
title: A subtract whose tool lies wholly inside its target ties vol(A ∖ B) ≥ vol(A) − vol(B) exactly, and the certified lane decides that tie on no box
status: open
opened: 2026-10-03
priority: P2
cost: M
---


Found by SHOW's `the-plate-document-never-cuts-its-holes`, re-authoring
the tour's two-hole plate in its natural spelling.

## What

`demos/tour/src/plate.rs` `cut_plate`: an 8 × 4 × 1 mm blank, two
full-depth hole extrudes (r = 1.25 mm, at x = ∓`half_spacing`), each
subtracted by a `Boolean(Subtract)` with its flush caps declared
(`find_flush_candidates` → `declare_node`). At `f64` it builds, and the
Monte-Carlo lane answers over it (512 samples, the same web population
as the uncut study). The certified lane certifies NO box of it:

- `demos/tour/src/tolerance.rs` `cut_wall` (the live wall probe): the
  whole box at `1e-9` of the study, one leaf — `stackup` refuses
  `NothingCertified`, `0 certified`. The study's uncut document
  certifies that same box whole (asserted beside the wall).
- The real study (±0.05 mm, σ = 0.01 mm), 512 leaves: 0 certified, 512
  refused at the budget; the uncut document certifies 193.

## Cause, measured

A shape-report replay of the cut plate's whole box at
`Sym<Interval>` (`geom_core::sym::report`, the
`m10_8_harness::over_band_set` reading), the drive's own dials:

| box (fraction of the study) | over the band |
|---|---|
| `1e-3` | the first cut fails in the join: `Euler(Certification(Escalated { check: EndpointStart }))`, `carrier_endpoint_start` `[0, 8.0e-7]`, form `sqrt(?a + ?b + ?c)` over frozen atoms (1200 forms frozen) |
| `1e-6` | `point_in_arc_loop_conic_disc` `[-1.12e-9, 1.12e-9]` 6/114; `volume_backstop_violation` `[-1.39e-11, 1.39e-11]` 2/4 |
| `1e-9` | `volume_backstop_violation` `[-1.39e-14, 1.39e-14]` 2/4 |
| `1e-12` | `volume_backstop_violation` `[-1.55e-17, 1.54e-17]` 2/4 |

The enclosure is symmetric about zero and shrinks linearly with the
box: the margin is IDENTICALLY zero. Each hole lies wholly inside the
blank, so `vol(A ∖ B) ≥ vol(A) − vol(B)` holds with equality — one of
each cut's two bounds, hence 2 of 4. `boolean/ops.rs` `Posture::read`
reads a padded margin `Held` only when its lower end is certified
`Zero | Positive`, so a straddling tie is `Open` at every width, and
the symbolic tier does not discharge it (the volumes are flux sums
over forms the term budget freezes; 3700 forms frozen at `1e-9`).

This is the certified-scalar face of the family
`a-settled-declared-coincidence-crosses-a-tight-volume-bound.md`
names ("∖ ≥ A − B when B ⊂ A"): there the f64 result crosses the tie
by a settled gap; here nothing crosses it, and the interval lane
cannot say so on any box. Every hole, pocket or bore SUBTRACTED by a
tool that lies wholly inside the part is this shape. A hole sketched
as an inner loop of the part's own profile never reaches the bound
(no boolean): the same plate as one extrude of a profile with two
`LoopProgram::Circle` inner loops certifies the `1e-9` whole box
(and whole boxes up to `1e-2` of the study), and its wider frontier is
`work/paths/inner-loop-circles-bound-the-plate-study-at-arc-span.md`.

## A tool that overshoots refuses too, elsewhere

Measured by the PR 3922 review: hole extrudes at z = −0.5 mm, 2 mm
deep (no flush pair, so B ⊄ A and this bound is slack) still refuse
the `1e-9` whole box, `FlipCrossing` with `chart_bound_outer_span`
diverging 0 → 2 on both boolean nodes — a second certified-lane
frontier, independent of this tie, filed as
`work/chart/an-overshooting-subtract-flips-chart-bound-outer-span-over-a-tiny-box.md`.
Overshoot is not the plate's spelling (its holes are its depth); it is
evidence that clearing the tie alone would not certify a cut part.
