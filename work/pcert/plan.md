# PCERT — the plan

pcurve certification and what validate_pcurves actually checks

Opened 2026-09-20 by CHART's priority-seam cut (`work/README.md`,
Track size). Its P3 and P4 rows went to PCTAIL on 2026-10-01, along
the same seam, when the track measured 36.5 points against its 30.

## The slate

**22 budget points** of dispatchable work against a ceiling of 30.

| pri | item | cost | title |
|---|---|---|---|
| P0 | `S331` | H +design | validate_pcurves answers a clean bill on a body whose pcurve mint just failed |
| P0 | `validate-pcurves-cannot-tell-a-never-minted-face-from-an-emptied-one` | M +design | a face a door emptied reads exactly as one never minted |
| P0 | `validate-pcurves-never-recertifies-a-face-it-finds-incomplete` | M +design | the re-certification and continuity passes skip any face missing a row |
| P0 | `placeholder-chart-sup-arms-are-not-a-bound` | H | chart_stretch_sup answers unit arms for a placeholder chart |
| P1 | `D36` | E | PcurveCertifyError::UnsupportedCarrier is payload-free and means three things |
| P1 | `pcurve-chart-box-is-looser-than-harmonic-extent` | H | Pcurve::chart_box is twice the true span of a Harmonic image |
| P1 | `pcurve-fit-refusal-drops-the-domain-doors-reason` | E | PlaneNurbsRefusal::PcurveFit discards the domain door's typed SplineError |

## The design question, and what rides on it

`S331`, `validate-pcurves-cannot-tell-a-never-minted-face-from-an-emptied-one`
and `validate-pcurves-never-recertifies-a-face-it-finds-incomplete` are
one question asked from three sides: **what a face storing no row, or
some of its rows, may mean at rest, and what `validate_pcurves` may
claim about it.** Today a face storing nothing reads the same whether
the minting pass never ran, refused the face and cleared it
(`mint_faces` swallowing `UnsupportedCarrier`), or a door emptied it;
and a face storing some rows is refused for the gaps and never
re-certified. Two designers weigh it before any lane builds it.

`D36` rides on the answer: the variant it would split is the one
`mint_faces` keys its swallow on, so what `UnsupportedCarrier` means is
part of what the mint may leave behind. It is specified with the
question's outcome, not before it.

## Independent of it

`placeholder-chart-sup-arms-are-not-a-bound`,
`pcurve-chart-box-is-looser-than-harmonic-extent` and
`pcurve-fit-refusal-drops-the-domain-doors-reason` touch neither the
mint's leave-behind nor the at-rest pass, and run in parallel with the
weighing.

## The D3 lane-choice units (E)

D3's `[verdicts.consumers]` clause says a lane choice ends as a lane and
never as a defect. These units carry it through the pcurve layer, in
this order:

- **E1** (`pcert/lane-choices`): the class readings become lane choices.
  `PcurveCheck::ChartClass` routes to lever-only "not yet". Check 4's
  copy readings split out as `ImageIdentity`. The grazer reading goes.
  The cylinder arm is left to E2.
- **E1b**: incidence ends by `EdgeAuthority`. It threads the authority
  into topo's ending function and the mint's routes. E1 keeps today's
  `CarrierOffChart` / `ChartIncidence` endings.
- **E2**: a cylinder line gets a projected lane, together with the
  cylinder `pcurve_chart_radial_moving` tie-break and
  `chart_iso_family`'s re-read. D3's "own envelope carries the miss"
  clause comes out of the doc.
- **E3**: `edge_join` records its class Zeros.
- **E4**: a circle's general lane on a cone, torus and cylinder
  (`a-circles-general-lane-is-written-only-on-the-sphere`). Until it
  lands, such a circle refuses `GeneralLaneUnwritten` "not yet", and
  D3's "every class has one" is still owed.

## Review posture

Per unit, at dispatch, by the review tiers of
`memories/orchestration-model.md` (protocol v7's A/B triage is moot:
the A/B experiment is suspended). The log names each tier and its
reason.
