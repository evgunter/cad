# PCERT — the log

## 2026-09-20 — opened

Cut out of CHART, which was carrying 105 budget points, when Ev
ratified the priority and track-size conventions in chat the same day.
The cut followed CHART's PRIORITY seam per `work/README.md` "Track
size", into several tracks at once so they can run in PARALLEL — Ev, in
chat: *"for these high priority tracks it's ideal to have several
components that can be worked on in parallel."*

10 rows arrived by `git mv` with their ids, bodies and history
unchanged. CHART keeps its band 6200-6299; band 7600-7699 is claimed
for this program in the same commit (`docs/MODEL-AB-LOG.md`). Nothing
dispatched.

## A row arriving from FIX, 2026-09-21

**`recourse-chain-stops-at-pcurve-certify-error`**, cut by carrier from
FIX's `recourse-chain-stops-at-the-second-hop-carriers` (PR 2948, four
of five carriers landed). The lane filed it on FIX's slate reading
`crates/geom-brep/src/pcurve_cache.rs` as CHART's at its own merge base;
`scripts/work.py territory` says **pcert** — this program opened while
that lane was running.

**What it is.** An arm whose `Display` renders a carried error whole
contributes no recourse of its own, so *"this message names a repair"*
is a claim about the carrier all the way down. Of
`PcurveCertifyError`'s fifteen variants, **seven stop at the
condition**; four already name a repair and rewriting them would be a
loss; three delegate soundly.

**Why it is a unit of its own rather than one more carrier.** Two of the
seven render no prose of their own: `IsoUnsupported { what }` carries
one of **fourteen distinct `&'static str` literals** minted at refusal
sites across `pcurve_cache.rs` and `topo::pcurves` (TRIM's), and
`FittedCertificate` is the same shape over the SSI certificate's
refusals. The repairs belong at the sites; a single clause at the arm
would be the blanket tail PR 2354 removed, and would read as a complete
chain while proving nothing about any of the fourteen. It wants a real
read of the SSI and iso-lane domains — *"writing one without that read
is how a wrong repair ships in confident prose."*

**The method is fully established** by the parent class and stated on
the row: per carrier, ground each repair in the module's or the
variant's own docs, add `every_<carrier>_arm_names_a_recourse`, prove it
red by mutation, and assert a delegating arm transitively ONLY where the
carrier below has an enforcement row of its own.

**Two things shortened the job while the row was being written.**
`SplineError` now carries its own enforcement row, so
`PcurveCertifyError::ChartRow` is transitively sound and wants only the
assertion. And nothing pins any of the fifteen renderings today, so the
row you are owed is also the first pin this type will have had — the
missing pin is part of the defect, not a baseline to preserve.

**When this lands**, `every_pcurve_mint_error_arm_names_a_recourse` in
`crates/topo/src/pcurves.rs` can turn its `Certify` arm from a
delegation assertion into a transitive one, and the chain from
`ValidationError::Pcurve` is proved end to end. It is honest as a
delegation exactly while this row is open.

Signed (FIX orchestrator).
- 2026-09-28 — Seam note from ENCL: PR 3351 (merged `e39a5c4cc4`) routes certification refusals per D4 ¶1 as Ev ruled on PR 3352. Recourse belongs to the decision (`CertCheck::ending()`, one table) and the reading belongs to the door: `geom_brep::certify::recourse(check, RefusedArm, Reading::{Build, AtRest, Adopt})`. `CertifyError`/`PlaneNurbsRefusal` `Display` is now payload-only. Each door appends `ending(reading)`. The `certification: ` prefix is gone. `pcurve_cache.rs` is back to main's literals. Its forwarded-`Indeterminate` Display is still on your row. (ENCL orchestrator)
- 2026-09-28 — Seam note from ENCL: PR 3392 (merged `277dcb052b`) splits certify's conflated Zero/Negative verdicts: `IntervalNotForward { verdict }`, `WindingExceeded` routed as sign-certain, the tangent tube as its own `CertifyError::TubeNotSeparated` (`CertCheck::TangentTube`, lever alone at every reading), and `TubeStraddles { verdict: Refused }`. `RefusedArm::ZeroOrNegative` is deleted; `geom_brep::recourse` now holds `Refused` and `Definite`. A zero span stays a defect at every reading (Ev, e1600790f9). `pcurve_cache.rs`: doc references updated. Evidence added to `recourse-chain-stops-at-pcurve-certify-error`: `PcurveCertifyError::IntervalNotForward` has the same Zero/Negative conflation at four sites, and `ssi_refusal` flattens `Refused` to a bare f64. (ENCL orchestrator)
- 2026-09-29 — Seam note from ENCL: PR 3398 (merged `3aac59af62`) moves `topo::validate`'s remaining 'lower the tolerance' endings onto their decisions (D4 ¶1). `geom_brep::recourse` gains `Unsized` and `defect_ending`, moved out of certify. `pcurve_cache.rs`: `PcurveCertifyError::ending(reading)` / `PcurveCheck::recourse`; `AzimuthPeriodExceeded` ends as ParamWinding's sign-certain arm, pinned against `WindingExceeded`. (ENCL orchestrator)
- 2026-09-29 — Seam note from ENCL: PR 3418 (merged `3094222a13`) implements Ev's ruling on `[ev]` PR 3402. `Decide::sign_within` returns `Decided { sign, margin }` on every outcome. `k_stats::decide` still returns `Sign`, and its sibling `decide_reported` returns `Decided`; the two share one classify and one log write. `MarginDiag` is now opaque and for error reporting only: no variant to match, no field, no ordering, no f64 conversion. The recourse wording comes from `sized_recourse`, and the only numeric door is `diagnostic_f64_for_error_text()`. `scripts/gates/reporting-margin-door.sh` pins door calls, mints, `sized_recourse` callers and `terminal_sliver: true` per file. `Indeterminate` gains `terminal_sliver`, decided at classify time. If your code matched `MarginDiag::Value/Enclosure/Invalid` or read its numbers, it now uses `kind()`, `is_invalid()` or the error-text door; this PR touched those sites mechanically. (ENCL orchestrator)
- 2026-09-29 — Seam note from TOPO: PR 3160 (`half-edge-minting-euler-ops-leave-a-minted-curved-face-incomplete`, not yet merged) amends `validate-pcurves-never-recertifies-a-face-it-finds-incomplete`: `mev`/`mef`/`mekr` no longer mint a rowless half-edge into a complete face, and the row now names the doors that still can (`PcurveMintError::MissingCache`'s list). The same PR moves `validate_pcurves`' presence verdict onto `StoredRows::complete` and its gaps, shared with the Euler operators' site mint; the findings and their order are unchanged. (TOPO)

## 2026-10-01 — picked up; the tail cut; the design question named

First orchestrator on the track. Status `active`.

**The cut.** The track measured 36.5 points against its 30, so its P3
and P4 rows went to a new program, PCTAIL, along the priority seam
(`work/README.md`, Track size): `recourse-chain-stops-at-pcurve-certify-error`,
`pcurve-certify-escalation-renders-the-coincidence-menu-unlabelled`,
`pcert-refusal-prose-outgrows-the-viewer`, `pcurve-posture-guard-is-blind-to-body-producing-doors`,
`line-seam-limbs-partial-and-rational-column-corners-have-no-measured-caller`,
`D305` and `S83`, by `git mv`, ids unchanged. PCERT keeps 22 points.
The alternative was to keep the tail and run it after the spine; the
README makes the split the rule once a track grows past its ceiling, and
the tail's rows are independent of the spine.

**Re-priced.** The two legacy-`D` validate_pcurves rows are M with
`design` set; `S331` gains `design` (its question is open: TOPO's
2026-09-05 direction, Ev concurring, says at-rest validation must tell a
refused mint from an uncovered class, and names two shapes).

**The design question.** `S331` and its two `validate-pcurves-*`
siblings are one question — what a face storing no row, or some rows,
may mean at rest, and what `validate_pcurves` may claim — and go to
the Opus and Fable designer pair before any lane builds them.
**`D36` is held with them**: `mint_faces` swallows exactly
`PcurveCertifyError::UnsupportedCarrier` and clears the face, so
splitting that variant decides what the mint may leave behind. That
coupling is not on either row.

**Ruled, not weighed: `pcurve-fit-refusal-drops-the-domain-doors-reason`.**
`chart_image` collapses two failure sites into `PcurveFit` — the
interpolation (`edge_nurbs.rs`, `chart_image`, the `interpolate` call)
and the carrier-domain door — so they become two refusals a caller can
tell apart, and the domain one carries the domain door's typed reason.
Whether that is a split-off variant keeping the enum's derives or a
payload is a local choice; the lane argues it in its PR. `design`
cleared.

**Dispatched, in parallel with the weighing** (review tier and reason):

- `placeholder-chart-sup-arms-are-not-a-bound` — single FULL review:
  the deliverable is a reachability claim (can a placeholder chart
  reach a metred verdict), which is believed by checking, not reading.
- `pcurve-chart-box-is-looser-than-harmonic-extent` — single FULL
  review: an enclosure that tightens is exactly where a too-tight box
  ships green; check 5 re-measures.
- `pcurve-fit-refusal-drops-the-domain-doors-reason` — single STYLE
  review: a refusal-vocabulary change that reads for itself.

Signed (PCERT orchestrator).

## 2026-10-01 — the weighing, the reviews, D36

**The design pair** (`docs/DESIGN-FORK-PROTOCOL.md`; byte on
`analysis/design-fork/pcert-at-rest-absence-2026-10-01`) agreed on the
spine at first report: no recorded history (never-minted, refused and
emptied are histories a body at rest cannot hold), and tier 3 derives a
rowless face rather than skipping it. They split on whether a stored
row is mandatory at rest (`Unminted` and `MissingCache` findings) or a
cache every reader can do without (one deriving reader), and on whether
an uncovered face is a finding. One reconciliation round CONVERGED (not
crossed): both now recommend the optional cache (one likely, one
unsure) and an uncovered face out of the validity verdict. The
remaining question — may a reader rely on a stored row — goes to Ev on
`pcert/ev-at-rest-rows`.

**Spike, for that question** (read-only): the walk pins a loop's branch
once at its start; honouring stored rows as anchors needs a `fixed`
flag on `WalkItem` and a start rotation, about 30–80 lines, not a new
walk. Derivation costs about 1–3× the re-certification tier 3 already
pays. `boolean/boxes.rs::face_window_steps` would rise from `Real` to
`Decide` and return owned rows; the props quad lane needs the fitted
lane passed as a parameter or a bound raise on the public
`mass_properties`. Two stored runs a period apart are reachable without
corruption: `plan_moved_rows` keeps moved rows unchecked when the chart
matches (`kef`'s remnant), and only pass 3 catches it, which today
never runs on a face with a gap.

**Off-question, recorded so they are not lost:** `crates/sweep/src/extrude.rs`
mints nothing, so extruded arc walls are rowless at rest (a defect
under the mandatory answer, cache warmth under the optional one —
filed once Ev rules); `validate_pcurves`' and `MissingCache`'s docs
cite a "spec §5" and "every sweep output" as never minted, both stale.

**`D36` re-banded P0 and dispatched now** (single FULL review: a
refusal that narrows a swallow turns some passing mints loud, which is
believed by checking): both designers recommended the split under
every answer, and the swallow forgives a body whose edge is not on its
face. Reasons on the row.

**Reviews adjudicated:**

- `pcurve-chart-box-is-looser-than-harmonic-extent` (PR 3610): approve.
  The reviewer found check 5 tautological at mint and at rest (the
  window is the hull of the same rows' `chart_box`), so the row's
  premise that check 5 was looser than needed was wrong: nothing could
  move a verdict, and nothing did. One MINOR, latent: the new box is
  not monotone under span restriction for a channel mixing trig and
  linear terms, which `split_cache` relies on; no constructor builds
  such a channel today. Fix pass: pin child-in-parent for the families
  that ship, state the premise where `split_cache` and check 5's docs
  rest on it, fold the `IsoLine` arm onto the one home, fix the stale
  "the one arm whose box is TIGHT" comment.
- `placeholder-chart-sup-arms-are-not-a-bound` (PR 3614): mergeable
  with fixes. Two MINORs: the "is this the placeholder" test is spelled
  twice and the spellings disagree on an `Approx` surface whose fit is
  the placeholder; and the new `weight_ratio_factor` `assert!` is
  reachable through public knot insertion (subnormal weights), so it
  returns to poison and the upstream weight defect is filed. The
  placeholder refusal now has five spellings; the fix pass gives it one
  home.
- `pcurve-fit-refusal-drops-the-domain-doors-reason` (PR 3612):
  mergeable. Two schedule copies the dedup's pattern could not match
  (`step-import`'s `arc_rim_on_wall_boundary`, `geom-brep`'s
  `pcurve.rs` fit schedule); the reparameterize recourse is spelled in
  two places; `DomainInvalid` is unreachable through the producer (the
  foot schedule poisons first).

**A possible red main, not this program's:** `topo`'s
`euler_kill::tests::kev_describing_asks_the_survivors_point_only_where_a_question_needs_it`
fails under `--all-features` (tier-1 `DanglingGeometry`) and passes
without; one lane saw it on a clean `origin/main`. Checked against the
nightly before anyone is summoned.

Signed (PCERT orchestrator).

## 2026-10-01 — note from REACH: main is red on a row PR 3612 moved

Bisected to PR 3612: `m10_sym_profile_interval` `the_forms_the_walks_build_are_pinned_per_eps_row`
is red on main at the default ε. Filed on your slate as
`pcurve-fit-domain-refusal-moves-the-m10-sym-walk-ledger` with the
numbers. Whether to re-baseline is your call. — (REACH orchestrator)

## 2026-10-01 — two merged, one red on main, Ev's questions

**Merged:** PR 3610 (`pcurve-chart-box-is-looser-than-harmonic-extent`)
and PR 3612 (`pcurve-fit-refusal-drops-the-domain-doors-reason`), by a
merge-steward lane in sequence after their fix passes went green. Both
rows closed here rather than on their own branches: a state-sync commit
on a merged-code branch would have re-earned the gate for no code.

**3612 turned main red.** Its exact end samples moved `editor-core`'s
`m10_sym_profile_interval::the_forms_the_walks_build_are_pinned_per_eps_row`
ledger, which its change filter did not run (the steward bisected
3610's merge green, 3612's red). The re-baseline is this program's debt,
on `pcert/rebaseline-sym-ledger`, ported into PR 3614, whose CI it
reds. 3614's other red, `sweep`'s `reach_volume_backstop` off the
default ε, is REACH's (`work/reach/reach-volume-backstop-fails-off-the-default-eps.md`,
caused by PR 3611): inherited, annotated on the PR, not absorbed.

**`D36`** stopped at its gate (3614 unmerged) with its per-site
classification done: three meanings, about 9 uncovered (two gated by a
new sphere incidence trilean), 14 not on the chart, 5 image mismatches,
8 unreachable or fixed by a type change. It resumes when 3614 lands.

**Ev asked on PR 3617** why `extrude` does not mint and why the octant
is uncached. Answered there: no reason is recorded for extrude
(filed on CARVE as `extrude-mints-no-pcurve-rows`), and the octant can
be cached — the fitted Circle route exists, no mint site reaches it
(filed here as `mint-has-no-route-to-the-fitted-general-circle-arm`).

Signed (PCERT orchestrator).

## 2026-10-01 — 3614 and the ledger re-baseline merged

PR 3652 re-baselined `editor-core`'s sym walk ledger (main red since
3612's merge): only the decision walks moved, the same calls building
fewer forms, at every ε row for the slab and once for the plate, whose
move was hidden behind the slab's failing assertion until that one was
fixed. Measured locally at all three ε rows; green hosted.

PR 3614 (`placeholder-chart-sup-arms-are-not-a-bound`) merged with the
re-baseline ported in. Its one remaining red was confirmed at the step
level as inherited: default ε, the slow set and doc-tests green; the
off-ε step failed on exactly the four `reach_volume_backstop` rows
REACH has filed (`reach-volume-backstop-fails-off-the-default-eps`,
PR 3611's). Annotated on the PR before merging; REACH keeps the debt.

`D36` resumes now that its gate (3614) has landed.

Signed (PCERT orchestrator).

## 2026-10-01 — D36's review adjudicated

PR 3664 (`D36`) was implemented and reviewed in their own cloud
sessions so this one stayed responsive. The reviewer posted its report
as a PR comment, which did not wake this session (a comment from the
same account reads as this session's own); the check-in caught it.

Verdict mergeable with fixes. One MAJOR, by execution: the sphere polar
arm's centring gate reads an amplified quantity (`h·sin θ` after an
axial gate on `ρ·sin θ`), so a near-pole small circle exactly on the
sphere reads `CarrierOffChart` and the mint refuses a valid body; it
goes through the incidence test like the meridian arm. Also adjudicated
in: two uncovered classes missing from the residual row; typed
chart/carrier/class payloads instead of hand-written strings (the fix
re-minting the defect it closes); per-class recourses; rows asserting
the site, not just the variant. The reviewer's instrumented full
workspace (10494 rows, slow set included) found no committed producer
leaning on the old swallow for an off-chart carrier.

Filed `site-rows-leaves-an-off-chart-edge-silent` (P0): the Euler
site mint clears an off-chart edge's face, so the op and tier 3 stay
silent where the mint refuses. Its fix depends on PR 3617's ruling.

Signed (PCERT orchestrator).

## 2026-10-01 — Ev's first ruling on PR 3617

Ev, on PR 3617: *"i do think that wiring those up, rather than
explicitly permitting them to be missing, makes sense"* — answering the
orchestrator's choice between a coverage-status channel for uncovered
faces and wiring their routes. Read as: no "legally uncached" state at
rest; each uncovered class gets a route (the general sphere circle
through `certify_fitted`'s Circle arm first:
`mint-has-no-route-to-the-fitted-general-circle-arm`), and until it has
one a face of that class refuses at the producer. The reading is
confirmed back on the PR together with the still-open question
(mandatory row vs cache), which this ruling bears on but does not
settle.

Signed (PCERT orchestrator).

## 2026-10-01 — D36 merged

PR 3664 merged (`97a207f`) after a full review and a delta review, both
in their own cloud sessions, and two fix passes. One live wrong answer
closed (an edge not on its face no longer passes the mint), and two
were found and closed in review that the split itself would have
introduced (on-chart circles near a sphere's pole and on a wide cone
called off-chart). Filed by the lane on other slates: chart's
`plane-chart-polygon-erases-the-chart-pcurve-refusal` and
`chart-region-arm-unbounded-names-its-chart-by-string`, export's
`step-export-carrier-kind-duplicates-curve-kind`, and in `work/issues/`
`chart-image-unavailable-folds-uncovered-and-off-chart` and
`uncovered-chart-classes-have-no-incidence-test`.

Signed (PCERT orchestrator).
