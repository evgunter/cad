# FOREIGN — the plan

## The slate

**24 budget points** of dispatchable work against a ceiling of 30
(EXCH's 2026-10-29 semantic-seam split).

| pri | item | cost | status |
|---|---|---|---|
| P0 | `import-normalizes-the-rim-only-cap` | M | open |
| P1 | `import-splits-a-foreign-face-or-edge-at-each-c0-knot` | M | open |
| P3 | `infer-outer-reads-an-arcs-sag-off-a-sample-polygon` | E | open |
| P3 | `step-import-l-shaped-curved-face-has-no-containment-row` | M | open |
| P3 | `step-import-reads-area-signs-raw-outside-decide` | E | open |
| P3 | `step-import-reads-no-hyperbola-or-parabola` | M | open |
| P3 | `step-import-refuses-a-ring-the-kernel-measures` | M | open |
| P3 | `step-import-refuses-a-ringed-cylinder-wall-the-kernel-measures` | M | open |
| P3 | `step-scaffold-strut-offset-is-absolute-in-a-unit-free-format` | M | open |
| P4 | `step-import-unwrap-reads-a-whole-period-off-a-value` | E | open |

## Order

`import-normalizes-the-rim-only-cap` first (P0, `design: true`: whether `seamed.step` is accepted, normalized or stays refused goes to the designer pair). Then `import-splits-a-foreign-face-or-edge-at-each-c0-knot` (P1, ruled by Ev on [ev] PR 5913). The two ring rows are one gate and are dispatched together, with the outer-loop inference rows riding along on `chart.rs`.

## Review posture

The repo-wide tiers (`memories/orchestration-model.md`), named per unit
at dispatch in this log.
