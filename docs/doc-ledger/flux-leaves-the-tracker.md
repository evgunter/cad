# FLUX and FLUXTAIL leave the tracker — 2026-10-27

FLUX held the curved closed-form area and volume arms. It opened
2026-10-01 as PROPS' successor. FLUXTAIL held those arms' numeric
honesty and was cut out of FLUX on its priority seam on 2026-10-10.
Both closed on 2026-10-27, when Ev asked in chat for the two to be
joined and re-split by logical thread rather than by priority, so that
the results could run in parallel safely.

Neither plan set `## Exit criteria`, so both close without an exit walk
(`work/README.md`, "A closed program's directory is deleted").

## Why re-cut

FLUX held 51 budget points against 30 and FLUXTAIL 44.5. The priority
split had put both programs' rows into the same files: each had about
eight rows in `crates/geom-brep/src/props/curved.rs`. So two
orchestrators could not run at once without colliding. Joined, the 42
live rows fall into five threads, each under budget. Each thread mostly
owns its own files, and where two share a file they work on different
functions of it.

## Where the rows went

Every live row moved by `git mv`, keeping its id and body; only the
header changes listed below were made.

| program | band | rows | points | thread | files it owns |
|---|---|---|---|---|---|
| CONEARM | 15200–15299 | 6 | 13 | the cone arm: rings and holes that leave their region, the apex loop, the section ellipse's elevation and frame, the slant's second spelling | the cone half of `props/curved.rs` |
| NEARPOINT | 15300–15399 | 12 | 24 | the reference point and frame: planar, cylinder, cone and sphere sums about a far point, rim levels that cancel, incidence and side reads decided one coordinate at a time | the cylinder, sphere and planar arms of `props/curved.rs` and `props/mod.rs`, plus `props/closure.rs` |
| SPIRICARM | 15400–15499 | 8 | 20 | the spiric and torus lanes | `props/spiric.rs`, `topo/src/props/quad_lane.rs` |
| SPANFOLD | 15500–15599 | 8 | 15.5 | quadrature span folds and loop-area arms | `props/quad/trimmed.rs`, `props/loop_area.rs`, `props/rational_rim.rs` |
| MEASUREDOOR | 15800–15899 | 7 | 13 | the measurement door, tier 3 beside it, and the backstop's main red | `topo/src/props.rs`, `topo/src/validate.rs` |
| BOXES (live) | — | +1 | +2.5 | `an-edges-extent-has-four-dispatchers`: one question, four dispatchers | — |

The apex-lever row (`a-cone-wall-far-from-its-apex-measures-its-rounding-at-the-apex-lever`)
went to NEARPOINT with the other far-reference rows, although it edits
`cone_face_closed_form` beside CONEARM's region rows; both plans say so.

Header changes at the cut:
- `an-edges-extent-has-four-dispatchers`: P3 → **P1** (Ev, in chat).
  It is the "two implementations of one underlying logic" class, and
  three per-site folds wait on its home.
- Priced: `frame-read-off-ellipse-fields-in-curved-props` (P3, E),
  `props-reads-a-cone-points-slant-by-its-own-spelling` (P3, E),
  `a-thin-corner-bars-spiric-cap-exhausts-the-quadrature-budget-at-1e-12`
  (P3, M).
- Closed as overtaken: `props-converged-calls-the-enclosures-own-width-a-fit`.
  PR 5741 deleted `PropsCheck::Converged`.
- Closed as duplicates of `main-red-reach-volume-backstop-rows-after-the-exact-sliver-bracket`:
  `main-red-reach-volume-backstop-midpoint-premise-and-planted-subtraction`
  and `main-red-5734-moves-tilted-cut-upper-and-two-reach-backstop-rows`.
  The latter's digest half was already re-cut by QUAD (PR 5818).
- Given an update section, without a status change:
  `a-spiric-elbow-a-hundred-kilometres-out-…` and
  `a-thin-corner-bars-spiric-cap-…`. Both name a `QuadratureBudget`
  refusal that PR 5741 removed, so each is to be re-measured first.

Live rows across the tracker that cited a closed FLUX row in `refs` now
cite the PR that closed it (13 references). Path citations in code,
docs and items were re-pointed:
- a moved row to its new directory;
- a closed row to its bare id and closing PR (table below);
- three already-stale paths (rows long since moved to SCALAR) to where
  those rows live now.

Logs, `docs/DUAL-REVIEW-LOG.md`, `docs/DESIGN-FORK-LOG.md` and the
doc-ledger keep their paths, which resolve at the SHA below.

Bands **12600–12699** (FLUX) and **12200–12299** (FLUXTAIL) close with
them UNSPENT: no A/B ordinal was drawn from either.

Recover SHA `f200582a3bf0bf053ec5c800b02543715fd0f18d`.

    git show f200582a3bf0bf053ec5c800b02543715fd0f18d:work/flux/<FILE>
    git show f200582a3bf0bf053ec5c800b02543715fd0f18d:work/fluxtail/<FILE>

## The closed rows

Each closed row with the PR that closed it. The PR was read from the
row's `pr:` field where it has one, and otherwise from the
first-parent merge that brought its closing commit to main.

| row | file | closed by |
|---|---|---|
| `a-far-frustums-closed-form-volume-reports-a-zero-pad` | flux/a-far-frustums-closed-form-volume-reports-a-zero-pad.md | PR 5576 |
| `a-notched-cylinder-wall-has-no-volume-measurement` | flux/a-notched-cylinder-wall-has-no-volume-measurement.md | PR 5689 |
| `a-pair-of-seam-siblings-flipped-together-is-silent` | flux/a-pair-of-seam-siblings-flipped-together-is-silent.md | PR 4746 |
| `a-planar-face-bounded-by-a-rational-spline-rim-has-no-vector-area` | flux/a-planar-face-bounded-by-a-rational-spline-rim-has-no-vector-area.md | PR 5689 |
| `a-quadrature-faces-pad-scales-with-its-shells-size-and-stored-face-order` | flux/a-quadrature-faces-pad-scales-with-its-shells-size-and-stored-face-order.md | PR 5689 |
| `a-ring-on-a-cone-wall-escalates-props-chart-loop-whole-at-1e-12` | flux/a-ring-on-a-cone-wall-escalates-props-chart-loop-whole-at-1e-12.md | PR 5400 |
| `a-seam-region-covering-the-sphere-twice-is-unread` | flux/a-seam-region-covering-the-sphere-twice-is-unread.md | PR 5516 |
| `a-seam-region-whose-boundary-carries-both-poles-is-unread` | flux/a-seam-region-whose-boundary-carries-both-poles-is-unread.md | PR 5516 |
| `a-seamless-cylinder-band-measures-in-closed-form-and-refuses-once-it-carries-an-ellipse` | flux/a-seamless-cylinder-band-measures-in-closed-form-and-refuses-once-it-carries-an-ellipse.md | PR 4839 |
| `a-shells-pad-still-depends-on-face-order-where-one-quadrature-region-spans-the-lever` | flux/a-shells-pad-still-depends-on-face-order-where-one-quadrature-region-spans-the-lever.md | PR 5801 |
| `a-shells-pad-still-depends-on-which-quadrature-face-it-stores-first` | flux/a-shells-pad-still-depends-on-which-quadrature-face-it-stores-first.md | PR 5699 |
| `a-sphere-face-whose-boundary-encodes-no-side-is-measured-under-its-bit-alone` | flux/a-sphere-face-whose-boundary-encodes-no-side-is-measured-under-its-bit-alone.md | PR 4573 |
| `a-sphere-rim-level-reads-its-latitude-cosine-off-the-sine-near-a-pole` | flux/a-sphere-rim-level-reads-its-latitude-cosine-off-the-sine-near-a-pole.md | PR 5375 |
| `a-sphere-shell-flipped-whole-in-a-multi-shell-body-measures-wrong-at-the-door` | flux/a-sphere-shell-flipped-whole-in-a-multi-shell-body-measures-wrong-at-the-door.md | PR 5590 |
| `a-torus-face-with-a-spiric-ring-has-no-volume-lane` | flux/a-torus-face-with-a-spiric-ring-has-no-volume-lane.md | PR 5567 |
| `an-ellipse-trimmed-ring-on-a-cylinder-wall-has-no-volume-lane` | flux/an-ellipse-trimmed-ring-on-a-cylinder-wall-has-no-volume-lane.md | PR 4557 |
| `c12-7-reads-a-both-poles-sphere-face-through-its-seam-region` | flux/c12-7-reads-a-both-poles-sphere-face-through-its-seam-region.md | PR 5689 |
| `closed-form-loop-whole-charges-near-axis-rounding-at-the-largest-radius` | flux/closed-form-loop-whole-charges-near-axis-rounding-at-the-largest-radius.md | PR 5333 |
| `cone-ellipse-edges-carry-no-incidence-decide` | flux/cone-ellipse-edges-carry-no-incidence-decide.md | PR 4744 |
| `cone-face-closed-form-runs-an-iso-premise-its-formula-does-not-need` | flux/cone-face-closed-form-runs-an-iso-premise-its-formula-does-not-need.md | PR 4696 |
| `cone-incidence-decides-over-lever-toward-the-needle-and-the-flat` | flux/cone-incidence-decides-over-lever-toward-the-needle-and-the-flat.md | PR 4744 |
| `cone-ring-arc-spans-are-read-from-endpoints-not-decided` | flux/cone-ring-arc-spans-are-read-from-endpoints-not-decided.md | PR 4744 |
| `curve-projection-orthogonality-field-has-no-reader` | flux/curve-projection-orthogonality-field-has-no-reader.md | PR 4835 |
| `cylinder-axis-inside-reads-the-surface-radius-not-the-rims` | flux/cylinder-axis-inside-reads-the-surface-radius-not-the-rims.md | PR 5375 |
| `cylinder-meridian-axial-refuses-a-line-within-band-as-off-surface` | flux/cylinder-meridian-axial-refuses-a-line-within-band-as-off-surface.md | PR 5038 |
| `geom-brep-open-split-rim-probe-refuses-chart-loop-open-not-props-loop-closed` | flux/geom-brep-open-split-rim-probe-refuses-chart-loop-open-not-props-loop-closed.md | PR 5369 |
| `iso-parse-refuses-a-circle-within-band-of-the-cone-as-off-surface` | flux/iso-parse-refuses-a-circle-within-band-of-the-cone-as-off-surface.md | PR 4932 |
| `m-tier-miss-at-dr-185-the-cone-lane-dropped-its-incidence-checks` | flux/m-tier-miss-at-dr-185-the-cone-lane-dropped-its-incidence-checks.md | PR 5689 |
| `m-tier-miss-at-dr-321-the-backstop-passes-a-violated-bound-on-rounding` | flux/m-tier-miss-at-dr-321-the-backstop-passes-a-violated-bound-on-rounding.md | PR 5689 |
| `main-red-5734-moves-tilted-cut-upper-and-two-reach-backstop-rows` | flux/main-red-5734-moves-tilted-cut-upper-and-two-reach-backstop-rows.md | — (duplicate) |
| `main-red-reach-volume-backstop-midpoint-premise-and-planted-subtraction` | flux/main-red-reach-volume-backstop-midpoint-premise-and-planted-subtraction.md | — (duplicate) |
| `main-red-the-endpoint-census-since-flux-rational-rim-and-shell-pad` | flux/main-red-the-endpoint-census-since-flux-rational-rim-and-shell-pad.md | PR 5809 |
| `main-red-the-mass-props-thread-golden-since-4938` | flux/main-red-the-mass-props-thread-golden-since-4938.md | PR 5369 |
| `mass-properties-measures-a-flipped-seam-region-without-tier-3` | flux/mass-properties-measures-a-flipped-seam-region-without-tier-3.md | PR 5556 |
| `props-collapse-over-lands-a-nan-window-on-the-first-span` | flux/props-collapse-over-lands-a-nan-window-on-the-first-span.md | PR 5689 |
| `props-converged-calls-the-enclosures-own-width-a-fit` | fluxtail/props-converged-calls-the-enclosures-own-width-a-fit.md | PR 5741 |
| `props-torus-lift-and-gauss-bonnet-fan-pick-a-branch-by-floor` | flux/props-torus-lift-and-gauss-bonnet-fan-pick-a-branch-by-floor.md | PR 5375 |
| `rim-side-and-rim-dir-group-signs-are-facts-about-cycle-order` | flux/rim-side-and-rim-dir-group-signs-are-facts-about-cycle-order.md | PR 4573 |
| `second-derivative-sup-takes-a-control-net-beside-its-vector` | flux/second-derivative-sup-takes-a-control-net-beside-its-vector.md | PR 4838 |
| `sphere-face-with-a-hole-has-no-closed-form` | flux/sphere-face-with-a-hole-has-no-closed-form.md | PR 4573 |
| `sphere-flux-arm-carries-two-closed-forms-for-one-face-kind` | flux/sphere-flux-arm-carries-two-closed-forms-for-one-face-kind.md | PR 4573 |
| `sphere-flux-arm-refuses-partial-bands` | flux/sphere-flux-arm-refuses-partial-bands.md | PR 5689 |
| `sphere-wedge-arm-does-not-fold-split-meridians-by-lineage` | flux/sphere-wedge-arm-does-not-fold-split-meridians-by-lineage.md | PR 4573 |
| `spiric-bounded-face-area-is-unimplemented` | flux/spiric-bounded-face-area-is-unimplemented.md | PR 4554 |
| `spiric-face-volume-budget-depends-on-world-position` | flux/spiric-face-volume-budget-depends-on-world-position.md | PR 5442 |
| `steep-frustum-operand-refuses-its-own-gate-at-1e-12-since-5038` | flux/steep-frustum-operand-refuses-its-own-gate-at-1e-12-since-5038.md | PR 5400 |
| `stored-spans-read-raw-past-winding-bound` | flux/stored-spans-read-raw-past-winding-bound.md | PR 4680 |
| `the-cone-closure-decide-is-a-fifth-spelling-of-the-periodic-chart-question` | flux/the-cone-closure-decide-is-a-fifth-spelling-of-the-periodic-chart-question.md | PR 5333 |
| `the-door-panics-resolving-a-removed-face-loop-or-half-edge` | flux/the-door-panics-resolving-a-removed-face-loop-or-half-edge.md | PR 5590 |
| `the-interpolating-fit-solves-a-banded-collocation-system-densely` | flux/the-interpolating-fit-solves-a-banded-collocation-system-densely.md | PR 4023 |
| `the-measurement-doors-test-sites-still-read-through-the-transitional-alias` | flux/the-measurement-doors-test-sites-still-read-through-the-transitional-alias.md | PR 5689 |
| `the-periodic-chart-closure-decides-are-four-spellings-of-one-question` | flux/the-periodic-chart-closure-decides-are-four-spellings-of-one-question.md | PR 4938 |
| `the-whole-body-pad-sum-rounds-to-nearest-and-the-enclosure-doc-scales-the-wrong-term` | flux/the-whole-body-pad-sum-rounds-to-nearest-and-the-enclosure-doc-scales-the-wrong-term.md | PR 5375 |
| `the-whole-body-property-fold-depends-on-face-order` | flux/the-whole-body-property-fold-depends-on-face-order.md | PR 5402 |
