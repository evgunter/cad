# PCERT — the plan

pcurve certification and what validate_pcurves actually checks

Opened 2026-09-20 by CHART's priority-seam cut
(`work/README.md`, Track size). Nothing dispatched.

## The slate

**26.5 budget points** of dispatchable work against a ceiling of 30.

| pri | item | cost | title |
|---|---|---|---|
| P0 | `S331` | H | validate_pcurves answers a clean bill on a body whose pcurve mint just failed — a vacuous green through a public door |
| P0 | `placeholder-chart-sup-arms-are-not-a-bound` | H | chart_stretch_sup answers unit arms for a placeholder chart while chart_stretch_inf answers certifies-nothing |
| P0 | `validate-pcurves-cannot-tell-a-never-minted-face-from-an-emptied-one` | D | validate_pcurves reads a face a door emptied exactly as it reads one never minted, so a drop that re-charters a whole loop is indistinguishable from a body the pass has not run on |
| P0 | `validate-pcurves-never-recertifies-a-face-it-finds-incomplete` | D | validate_pcurves skips its re-certification and continuity passes on any face missing a row, so a stale row on an incomplete face is never measured |
| P1 | `D36` | E | PcurveCertifyError::UnsupportedCarrier is payload-free and means three things across 22 construction sites |
| P1 | `pcurve-chart-box-is-looser-than-harmonic-extent` | H | Pcurve::chart_box is p0 +- |pl|*max|t| - twice the true span of a Harmonic image and what the mint's check 5 reads |
| P1 | `pcurve-fit-refusal-drops-the-domain-doors-reason` | E | PlaneNurbsRefusal::PcurveFit discards the domain door's typed SplineError |
| P3 | `D305` | E | Re-derive and dispose of the sole-T-Bounds doors the D1 census enumerated in geom-brep and did not take |
| P3 | `S83` | E | seam_tol / MarchTolMismatch is unreachable by construction and its acceptance row asserts a forced identity |
| P3 | `pcurve-posture-guard-is-blind-to-body-producing-doors` | D | The pcurve posture guard walks only &mut Body doors, so every &self -> Body producer's row posture is prose checked by nothing |

## Order

`S331` first — `validate_pcurves` answers a clean bill on a body whose
pcurve mint just FAILED, which makes every downstream reader's trust in
it false. Its two siblings
(`validate-pcurves-never-recertifies-a-face-it-finds-incomplete`,
`validate-pcurves-cannot-tell-a-never-minted-face-from-an-emptied-one`)
are the same door seen from two other angles and are specified with it,
not after it.

Then `placeholder-chart-sup-arms-are-not-a-bound` — unit arms answered
for a placeholder chart are a number wearing a bound's name. The four
`E` rows are drive-bys for whoever opens `pcurve_cache.rs` first.

## Review posture

OPEN, for this program's first dispatch. CHART inherits protocol v7
(`docs/MODEL-AB-LOG.md`, Ev 2026-09-19): the dual on triaged-in units
only, opus/opus outside it. Nobody has re-asked the triage question for
this slate, so the first orchestrator answers it here rather than
inheriting an answer.
