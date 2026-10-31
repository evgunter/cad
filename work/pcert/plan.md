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
- **E1d**: incidence ends by the door that reads it (Ev, `[ev]` PR 5838; the ruling is in
  `work/pcert/who-placed-it-is-read-off-the-description-not-the-door.md`). Topo's construction
  tier becomes kernel-internal: pncad stops re-exporting the Euler operators, the description
  setters and their spec types. That is a seam with the `lib` program's pncad surface. E1b's
  authority routing comes out: `AuthorityKind`, `DeclaredOffChart`, `DeclaredIncidence` and
  `render_by`. A definite miss at a build is a kernel defect, and at rest a stored
  contradiction. A read in band ends in the decision's lever at every door. At the import door,
  the damaged-file ending applies until EXCH's ε_in healing lands (this fixes E1b's import
  regression).
- **E1d2**: the construction tier is sealed. The Euler operators and setters are
  inherent methods on `topo::Body`, so a re-export cannot hide them: they move into a
  pub trait `topo::construction::Construct`, and the inherent methods become
  `pub(crate)`. pncad's `topo` becomes a curated module that omits `construction` and its
  spec/result types, as `pncad::profile` already does. Kernel crates import the trait.
  E1d lands the endings first; this unit makes the ruling's "not public" true.
- **E1c**: every mint reads incidence before it meters a carrier's miss.
  Today the plane mint images any carrier; the cylinder mint reads
  incidence only where its orientation reading is in band (E1's general
  lane); and the CLASS arms of the sphere, cone and torus mints (the
  sphere's polar circle, the cone's rim, the torus's parallel) image a
  carrier their class gates accept without an incidence read (review A on
  E1d); and the sphere and torus refuse an ellipse by kind, before any
  incidence read (review on PR 5844). Each then meters the carrier's miss as a residual at the samples,
  so a carrier off its face ends `ChartResidual`'s defect, and at the
  import door its ε_in offer, where E1d's door rule would end it: a
  definite miss by the door, a read in band by its lever
  (`work/pcert/plane-and-cylinder-mints-meter-incidence-as-a-residual.md`;
  its cylinder half touches the arm E2 re-reads). D3's incidence clause
  is not finished on any chart until it lands: on the sphere, cone and
  torus it holds only on the carriers their class gates send to the
  incidence test. It lands before E1e, so the incidence clause is whole
  before the `Display` work.
- **E1e**: a refusal's `Display` carries only its payload, and its ending exists only
  through `render(at)`. A gate counts the production spelling sites per door. Before cutting
  E1e, check whether the refusal-shape guard and the step-import fixtures read `Display`.
- **E2**: a cylinder line gets a projected lane, together with the
  cylinder `pcurve_chart_radial_moving` tie-break and
  `chart_iso_family`'s re-read. D3's "own envelope carries the miss"
  clause comes out of the doc.
- **E3**: `edge_join` records its class Zeros.
- **E4**: a circle's general lane on a cone, torus and cylinder
  (`a-circles-general-lane-is-written-only-on-the-sphere`). Until it
  lands, such a circle refuses `NoGeneralLane` "not yet", and
  D3's "every class has one" is still owed.
- **E5**: the mint-side class picks on a spline chart (`pick_arc_rim`,
  `iso_schedule`'s `probe_pick`, the arc-rim closure) take the general
  lane in band
  (`work/pcert/a-mint-side-class-pick-escalates-in-band-where-its-general-lane-exists.md`).
  Independent of E1b–E4: it reads no carrier's incidence and no circle
  lane, so it can run beside any of them; it is placed last only because
  nothing waits on it.

## Review posture

Per unit, at dispatch, by the review tiers of
`memories/orchestration-model.md` (protocol v7's A/B triage is moot:
the A/B experiment is suspended). The log names each tier and its
reason.
