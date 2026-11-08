# IMPORTGATE — the plan

## The slate

**19 budget points** of dispatchable work against a ceiling of 30
(EXCH's 2026-10-29 semantic-seam split).

| pri | item | cost | status |
|---|---|---|---|
| P1 | `step-parser-recurses-once-per-nested-list-and-a-deep-file-kills-the-process` | E | open |
| P2 | `can-an-import-reach-a-state-only-a-kernel-bug-otherwise-reaches` | M | open |
| P2 | `step-import-reads-a-negative-surface-radius-unchecked-before-edge-certification` | E | open |
| P3 | `adoption-error-header-names-a-lever-its-attempts-call-no-way-through` | M | open |
| P3 | `arc-rim-gate-reports-a-degenerate-carrier-as-an-infinite-residual` | E | open |
| P3 | `import-door-pcurve-escalation-ends-in-the-build-note` | M | open |
| P3 | `step-import-adopts-an-inverted-same-sense-outside-the-cylinder-cone-guard` | M | open |
| P3 | `step-import-placement-pcurve-and-rim-refusals-skip-the-import-doors-reading` | M | open |
| P3 | `the-per-instance-tier-3-gate-reads-every-assembly-face-twice` | M | parked |
| P4 | `exch-refusal-prose-outgrows-the-viewer` | E | open |

## Order

`step-parser-recurses-once-per-nested-list-and-a-deep-file-kills-the-process` first (P1, the fix is written). `can-an-import-reach-a-state-only-a-kernel-bug-otherwise-reaches` goes to the designer pair early: ENCL's panic arms and [ev] PR 5113's decision 2 wait on it. `the-per-instance-tier-3-gate-reads-every-assembly-face-twice` is parked on COINCIDE's `step-import-anchor-rows-are-recorded-and-not-linted`: the D10 refactor hold it first waited on lifted when INTENT closed (2026-10-30, #6103), and what it waits for now is an import that places its instances as a document operation.

## Review posture

The repo-wide tiers (`memories/orchestration-model.md`), named per unit
at dispatch in this log.

## INTERP's slate, merged in 2026-11-02

INTERP merged into IMPORTGATE on 2026-11-02 (`docs/doc-ledger/interp-leaves-the-tracker.md`). Its plan follows as it stood, headings one level down; its rows are now in `work/importgate/`.

### The slate

**28 budget points** of dispatchable work against a ceiling of 30
(EXCH's 2026-10-29 semantic-seam split).

| pri | item | cost | status |
|---|---|---|---|
| P1 | `adoption-rebuilds-caches-at-eps-from-eps-in-interpretation` | H | open |
| P1 | `step-import-eps-in-ambient-two-dial-strand` | M | open |
| P3 | `ellipse-edge-parameters-overstate-the-endpoint-miss` | M | open |
| P3 | `minted-seam-generator-can-put-endpoint-end-two-eps-in-off` | M | open |
| P3 | `partial-bspline-edges-are-refused-for-domain-pinned-parameters` | M | open |
| P3 | `step-adopt-reads-a-plane-origin-against-another-planes-normal` | E | open |
| P3 | `step-import-adopts-frame-directions-on-eps-in-that-check-1-refuses-at-eps` | M | open |
| P3 | `wild-nist-ftc-09-refuses-import-at-eps-1e-12` | M | open |
| P4 | `step-import-restates-the-cone-half-angle-convention` | E | open |

### Order

`adoption-rebuilds-caches-at-eps-from-eps-in-interpretation` first: it is the stage the rest are symptoms of, and its landing settles how the two-dial strand, the frame windows and `nist_ftc_09` close (some may close with it). The derived-parameter rows (ellipse, partial B-spline, minted seam) are independent of it and can run beside it.

### Review posture

The repo-wide tiers (`memories/orchestration-model.md`), named per unit
at dispatch in this log.

## FOREIGN's slate, merged in 2026-11-02

FOREIGN merged into IMPORTGATE on 2026-11-02 (`docs/doc-ledger/foreign-leaves-the-tracker.md`). Its plan follows as it stood, headings one level down; its rows are now in `work/importgate/`.

### The slate

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

### Order

`import-normalizes-the-rim-only-cap` first (P0, `design: true`: whether `seamed.step` is accepted, normalized or stays refused goes to the designer pair). Then `import-splits-a-foreign-face-or-edge-at-each-c0-knot` (P1, ruled by Ev on [ev] PR 5913). The two ring rows are one gate and are dispatched together, with the outer-loop inference rows riding along on `chart.rs`.

### Review posture

The repo-wide tiers (`memories/orchestration-model.md`), named per unit
at dispatch in this log.
