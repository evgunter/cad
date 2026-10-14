# SCALAR leaves the tracker — 2026-09-30

SCALAR covered the scalar lane: the lift doors, the newtypes that would carry an invariant, and the generic-scalar questions the kernel had been answering by hand. It opened on 2026-09-11 in the cut recorded in `docs/WORK-TRACKS-2026-09.md` (addendum 3). It closed on 2026-09-30 with all 47 rows closed.

It closed **without an exit walk**, because none was owed. `work/README.md` says *"a plan that set no criteria leaves a walk nothing to check, and its program closes without one"*, and `work/scalar/plan.md` had no `## Exit criteria` section. The finished-state check is that every row landed or was ruled; the table below records that. Ev asked for the close in chat on 2026-09-30.

`work/scalar/` is deleted whole: `program.md`, `plan.md`, `log.md` and the 47 row files. No row was re-homed, because the slate was emptied first. Recover the directory with `git show e8d0883eda:work/scalar/<file>`; that commit is the last one with the directory present, and it closes the final row.

## Where the rulings live

The rulings SCALAR carried were Ev's, answered on `[ev]` PRs. The PRs are the record, and the row files are recoverable at `e8d0883eda`:
- `D6` (the sense sign), `D283` (ε-typing) and `unit-vector-invariants-carried-as-prose`: PR 2457, the first sitting.
- `H5`'s `## RATIFIED` section: PR 2701, the second sitting. Ruling 1 retired `RingInterval` in favour of the backend's arithmetic and dropped the `interval` feature. Ruling 2 has a tighter certified bound re-baseline. Ruling 3 made the full no-trait cut: door values read at `AtRestPolicy`, with `_structural` twins.
- RING-5's re-wording of C9 (`crates/geom-brep/README.md`), signed off by Ev on PR 3174.

## Rows

| row | kind | closed | by |
|---|---|---|---|
| `D283` | ruling | 2026-09-15 | #2457: Ev's ruling (first `[ev]` sitting) |
| `D290` | unit | 2026-09-15 | #2461 |
| `D6` | ruling | 2026-09-15 | #2457: Ev's ruling (first `[ev]` sitting) |
| `H5` | unit | 2026-09-29 | rulings #2701 (Ev); its eleven units, the last #3194 |
| `S393` | issue | 2026-09-15 | #2466 |
| `ab-sample-230-claimed-twice-on-main-and-branch-side` | issue | 2026-09-30 | Ev in chat: renumber either; SYM-11 → #245 in `docs/MODEL-AB-LOG.md` |
| `certification-contains-admits-an-infinite-probe` | issue | 2026-09-29 | #3451 |
| `certification-doors-have-no-differential` | issue | 2026-09-29 | #3451 |
| `certification-gate-gaps-3-and-5-have-no-follow-up` | issue | 2026-09-29 | #3449 |
| `certification-refusal-called-poison-in-the-remaining-certification-tests` | issue | 2026-09-29 | #3448 |
| `certification-refusal-still-called-poison-outside-the-importers` | issue | 2026-09-29 | #3448 |
| `certification-value-hygiene-has-no-gate` | issue | 2026-09-29 | #3174 (RING-5); items 4 and 5 split to their own rows, since closed |
| `curve3-eval-and-deriv-at-one-t-run-two-basis-passes` | issue | 2026-09-21 | #2708 |
| `door-wiring-rows-justification-written-four-times` | issue | 2026-09-29 | #3449 |
| `exhaustiveness-receipt-carries-its-lane` | unit | 2026-09-15 | #2667 |
| `fitted-door-and-scalar-name-travel-as-a-pair-by-convention` | issue | 2026-09-29 | #3461 |
| `fitted-lane-refusal-text-omits-symbolic-and-cites-a-retired-hull` | issue | 2026-09-29 | #3449 |
| `frame-witness-and-the-tube-door` | unit | 2026-09-15 | #2675 |
| `gate-on-the-type-in-prose-outside-geom-core` | issue | 2026-09-24 | #3154 |
| `lane-0-offset-fit-hook` | unit | 2026-09-21 | #2981 |
| `lane-1-props-quad-lane-deleted` | unit | 2026-09-21 | #3010 |
| `lane-2-chart-region-lane-deleted` | unit | 2026-09-21 | #3038 |
| `lane-3-shell-lane-folded` | unit | 2026-09-24 | #3049 |
| `lane-4-fitted-lane-folded` | unit | 2026-09-29 | #3194 |
| `lane-4p-door-pins-in-one-shape` | unit | 2026-09-24 | #3165 |
| `point-at-drops-the-frame-witness` | issue | 2026-09-20 | #2896 |
| `public-ring-names-spell-the-retired-type` | issue | 2026-09-29 | #3448 |
| `quad-lane-formation-sites-in-its-own-module-are-unpinned` | issue | 2026-09-29 | #3449 |
| `rate-pair-in-geom-core` | unit | 2026-09-15 | #2657 |
| `ring-0-poison-differential` | unit | 2026-09-21 | #2993 |
| `ring-1-interval-type-ungated` | unit | 2026-09-21 | #2971 |
| `ring-2-newtype-over-dinterval` | unit | 2026-09-22 | #3032 |
| `ring-2-red-rows-that-are-not-re-pins` | issue | 2026-09-29 | #3032 |
| `ring-3-residue-outside-its-fence` | issue | 2026-09-29 | #3449 |
| `ring-3-ring-dissolves-into-interval` | unit | 2026-09-24 | #3153 |
| `ring-4-interval-feature-dropped` | unit | 2026-09-24 | #3154 |
| `ring-5-certification-doors-as-a-trait` | unit | 2026-09-29 | #3174 |
| `ring-nan-poison-is-load-bearing-at-unguarded-reads` | issue | 2026-09-29 | #3032 |
| `rustc-suggests-importing-real-in-a-certification-file` | issue | 2026-09-29 | #3449 |
| `sense-sign-doors-take-the-bit` | unit | 2026-09-15 | #2649 |
| `sense-sign-multiplies-fold-onto-outward-normal` | unit | 2026-09-15 | #2668 |
| `sweep-test-rebuilds-validated-net-for-v-reversal` | issue | 2026-09-15 | #2627 |
| `the-shell-door-is-a-third-door-value-the-certified-enclosure-census-does-not-know` | issue | 2026-09-24 | #3165 |
| `tracker-rows-cite-the-deleted-pcurve-fitted-lane-trait` | issue | 2026-09-29 | #3449 |
| `tracker-rows-cite-the-dissolved-ring-interval-type` | issue | 2026-09-29 | #3468 |
| `unit-vector-invariants-carried-as-prose` | issue | 2026-09-15 | #2457: Ev's ruling (first `[ev]` sitting) |
| `unit-vector-witness-in-geom-core` | unit | 2026-09-15 | #2646 |

## What the program handed on

These rows were filed where their owners read them:
- **EXCH** `work/exch/try-line-max-drops-a-refused-locus-certificate.md` (P3): a real defect. `f64::max` drops a refused NaN, so a degree-30 collinear Bézier promotes to `Line`.
- **PROPS** `work/props/invalid-margin-display-calls-a-refused-enclosure-poisoned.md`: the user-facing `MarginDiag` text.
- **CURVED** `work/curved/torus-meters-blocker-is-the-arithmetic-or-c9s-root-rule.md` (design).
- **CONTACT** `work/contact/c9-ring-class-phrase-names-a-retired-ring-or-the-exclusion-ring.md` (design).
- **RESTFRONT** `work/restfront/census-unexamined-recourse-lists-certifying-scalars-without-the-symbolic-tier.md`.
- **TINT** `work/tint/decoration-plane-mint-doc-names-the-c9-ring.md` and `work/tint/f64-exact-decomposition-has-four-test-homes.md`.
- **LINALG** `work/linalg/interval-backend-signed-zero-conventions.md` gained evidence: signed-zero codegen varies with the optimisation level.

## Experiment records

- **The model A/B protocol.** SCALAR's samples and blocks (SCALAR-B1 to SCALAR-B6) are in `docs/MODEL-AB-LOG.md`, which this sweep does not otherwise edit. The `ab_band` 4100–4199 stays claimed there. The one edit is the renumbering above.
- **The dual-review experiment.** SCALAR's pairs are DR-2 (RING-3), DR-20 (RING-5) and DR-21 (LANE-4, the first SCALAR tally candidate), all in `docs/DUAL-REVIEW-LOG.md`.
- **The design-fork experiment.** It holds no SCALAR row. The one fork weighed (the fitted door and the scalar's name) converged and was not Ev's, so its drawn byte on `analysis/design-fork/scalar-fitted-door-pair` is unused.
