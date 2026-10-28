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

## 2026-10-01 — Ev's second ruling on PR 3617

Ev, on PR 3617: *"i don't see the benefit to them not being mandatory
after everything wires them up? how would an object that's missing them
come to exist?"* The orchestrator agreed: once every class is wired,
only a producer's omission (`extrude`) or an un-minted assembly of
Euler operations leaves a body rowless, and a tier-3 finding is the
right answer to both; the cache answer's one payoff (simpler Euler
doors) is had under "doors may drop, producers mint". C4 on PR 3617
now states mandatory rows; that PR merges on Ev's confirmation of the
text. Neither designer's first recommendation (both leaned cache).

Signed (PCERT orchestrator).

## 2026-10-01 — the general-circle route dispatched

`mint-has-no-route-to-the-fitted-general-circle-arm` dispatched on Ev's
first ruling on PR 3617 (wire the uncovered classes rather than permit
them uncached), which does not wait on the C4 wording: route
`SphereGeneralCircle` into `certify_fitted`'s Circle arm and retire that
class's mint exemption in the same PR. Run in its own cloud session.
Review tier: single FULL — a new certified route through branch
pinning, believed by building, not reading.

Signed (PCERT orchestrator).

## 2026-10-01 — PR 3617 merged; the at-rest unit dispatched

Ev confirmed C4 by a 👍 on the orchestrator's "what I'll write, unless
you object" comment (reactions wake nothing, so it sat unseen until Ev
pointed it out). PR 3617 merged (`a4ab8d4`) with the fork-log row's
second half: Ev's decision, matching neither first recommendation.

Dispatched as ONE unit under `S331` (branch `pcert/at-rest-rows-mandatory`):
`S331`, `validate-pcurves-cannot-tell-a-never-minted-face-from-an-emptied-one`,
`validate-pcurves-never-recertifies-a-face-it-finds-incomplete`,
`site-rows-leaves-an-off-chart-edge-silent`, and
`extrude-mints-no-pcurve-rows`, claimed from CARVE by `git mv` because
mandatory rows make a non-minting producer a tier-3 red: the unit
cannot land without it. Review tier: DUAL — the change redefines what
tier 3 accepts for every curved body in the kernel, broad and hard to
reverse.

Signed (PCERT orchestrator).

## 2026-10-01 — PR 3733's review: not mergeable; a ruling on doorless scalars

The full review of PR 3733 (general-circle fitted route) found two
MAJORs, both by execution. (1) The new fitted image sits off its
carrier between samples — 1.08e-4 m on the oblique octant's rows at
ε = 1e-9, up to 1e-2 near a pole — and certifies with envelope 0,
because its interpolation nodes coincide with the certification
samples, so check 3 cannot see the error (planted corruption of every
off-sample node left 30 rows green). The fix pass gives the lane a
sound between-samples bound, adaptive density, off-node sampling, and a
typed refusal where nothing converges. (2) Retiring the exemption broke
the oblique fillet at `Dual64`.

**Ruling (orchestrator, from ratified D-L1 and C4):** at a scalar with
no fitted door, a face only the fitted lane can image is not owed rows —
that scalar certifies nothing fitted — so neither the mint nor tier 3
demands them there; one predicate, the scalar's missing door. Sent to
both PR 3733's lane and the at-rest unit's. Not put to Ev as a fork: it
is D-L1 applied to C4, not a new choice; reported to Ev.

Also from the review: D36's sphere incidence trilean meters over 2R,
under-reading a circle inside the sphere by up to 2× (lenient
direction, sliver band); the fix pass gives the residual one home with
the sound lever.

Signed (PCERT orchestrator).

## 2026-10-02 — PR 3759 (the at-rest unit) up; dual review; a design question it raised

PR 3759 implements C4: tier 3 dry-mints a rowless face and reports why,
re-certifies a half-minted face's rows against a window derived from
the face, and every public producer now mints (`extrude` raised to
`AtRestPolicy`). It also discloses a heavy cost: at the certified
scalars the pcurve certificate's `pcurve_envelope` and
`pcurve_map_residual` — zero by construction for a closed-form wall
image — widen past the band over a parameter box, and now run on every
extrude. M10-7's plate certifies only to 3.9e2·ε; the m10_3 hole drive
certifies no leaf above ε/8; sym11's certified lane refuses.

**Sequencing (orchestrator):** PR 3759 does not merge with that
regression. The widening is put to the designer pair (where a
zero-by-construction certified quantity is discharged — the certificate's
form, PCERT's ground, or the symbolic tier's registration, SYM's), and
its fix lands first or with 3759. The alternative — merge C4 now and
take the regression until a fix — was rejected: it trades a headline
capability (certified parametric studies) for a correctness rule whose
own cost is the regression's cause.

**Review:** DUAL, concurrent arm (unit cost H), two Opus reviewers on
frozen head `8d0b2c2` (R1, R2). Designers D1 and D2 dispatched on the
widening; byte on `analysis/design-fork/pcert-certificate-widening-2026-10-02`.

PR 3733's fix pass replaced the cubic image with a quintic Hermite one
carrying a Cauchy-estimate bound on the sixth derivative, a new
statement `MapResidualHermite`, and re-worded C4 to name it (re-wording
because the code moved; the statement is stronger than `OnLocusHull`).
A delta review on the bound follows its implementer report.

Signed (PCERT orchestrator).

## 2026-10-02 — the widening fork converges; 3733 delta review

- **The certificate-widening question** (the regression in PR 3759) was weighed by two designers.
  - Round 1 split: a SYM tier rule for trig of `atan2`, against restating the certificate in the carrier's coefficients.
  - Round 2 converged on the restatement. The tier rule's own author measured the restated terms as theorems at a ±1e-3 box, and found its rule cannot reach the UV-angle rows. Both now lean to moving a `Harmonic` row's schedule (check 3) off the box certificate.
  - That is a C4 revision, so it went to Ev on `[ev]` PR 3781 (fork-log row 43).
  - Restating check 4 is needed under either answer, so it is dispatched now as `pcurve-certificate-states-incidence-and-fidelity`, part 1 only, on a branch whose PR bases on 3759's branch.
  - 3759 stays held until that lands. Logged alternative: land 3759 re-baselined to "certifies nothing" now. Not taken: it would re-baseline the flagship tour into a refusal, and both designers and the implementer lean to holding.
- **3759 dual review.** R2 is in: not mergeable as-is. Its findings:
  - nine undisclosed reds of the widening class (editor-core slow set, demo tour);
  - two surviving mutants;
  - the uncovered exemption is kept against C4's text;
  - cycle-order masking.

  R1 had its review drafted but held waiting for a go-ahead, so I told it to post. Adjudication waits on R1.
- **3733 delta review: merge after fixes.**
  - The Hermite constant, the Cauchy step and the chart factor hold.
  - Check 4's glue is unsound for foreign images: per-end branch choice (a span winding a whole turn certifies), no image-domain check, and `max` where the derivation sums.
  - The fix list went to the implementer.

## 2026-10-02 — Ev rules on 3781; 3759 adjudicated; 3733 delta pass

- **[ev] PR 3781, merged as c0b9b35.** Ev took (i): C4 leads with the closed-form envelope as the whole statement, with the schedule as the fallback where no closed form exists. Two rounds of rewording came from Ev's comments. The unit `pcurve-certificate-states-incidence-and-fidelity` now builds both parts.
- **3759 dual review adjudicated.**
  - R1's report reached me only as its session transcript. R1 declined to post on a cross-session go-ahead, so its findings were taken from the draft summary, with probes on `pcert/review-3759-r1`.
  - Bilateral findings:
    - the undisclosed widening reds;
    - the untested half-minted window (M1 / p5);
    - masking by the first uncovered edge;
    - merge-faces re-mint dropping rows.
  - R1 only: wrong-branch rows silent on a complete face; "no decision was lost" is false (shallow-arc 54→51).
  - R2 only:
    - M6;
    - the C4 exemption, and `DESIGN.md:489`;
    - stale prose;
    - `cert_m3r1`'s filter_map;
    - the AtRestPolicy overclaim.
  - All of it went to the implementer as one list. The widening rows are not re-baselined.
- **3733.** The delta fix pass is green at d0f1fe6. The round-2 delta verification is dispatched.

## 2026-10-02 — 3733 merged

- **PR 3733 merged as 6cd3c66.** The round-2 delta verification said merge, and all eight probes flipped.
  - I merged main into it myself to resolve the C4 README conflict with 3781's rewording. The Hermite envelope now sits in the "no closed form" fallback, as a whole-span bound that keeps its schedule.
  - CI was green on 14239df.
  - `mint-has-no-route-to-the-fitted-general-circle-arm` is closed. The sphere general circle is the first uncovered class retired under the 3617 ruling.
- The incidence/fidelity unit is dispatched (session_01QjnkkS1HoEHDzjjkW3CCWq, branch `pcert/certificate-incidence-fidelity`).

## 2026-10-02 — 3759 delta review; 3812 sequencing

- **The first delta reviewer never started.** It was dispatched with a short SHA as `source_revision`, and the session failed with `ref_not_found`. I re-dispatched it against the branch and the full SHA. Lesson: always give `create_session` a branch name or a full SHA.
- **3759 delta review (comment 5951607360): merge with 3812, after one more pass.**
  - All four mutants die.
  - The 3733 reconciliation is clean.
  - Both implementer deviations stand. On item 3, m10_9 was left red, which holds only if 3812 restores its pins. On item 6, the stored hull plus `RowInterval` is sound. A row widened by 2π cannot be built; a row shifted by 2π is caught by continuity, except at the wrap joint (MINOR-3).
  - Fix list sent:
    - certify the derived images of covered edges on an excused face (MINOR-2, a fix rather than a stated limit);
    - check the wrap joint (MINOR-3);
    - stale prose;
    - `carry_rows` also runs `RowInterval`;
    - file a row noting that check 5 is vacuous at rest.
- **3812.** It carries part 2 and the literal walk branch (my ruling: a branch is structure, C4's "chosen once by the loop walk"). It must show the nine widening rows going green on its own head, which is the delta review's pre-merge check.
  - `chaintol` stays red until `pcurve-loop-decisions-state-a-3d-identity-plus-a-branch-margin` lands.
  - The trio (3759 + 3812 + the loop-decision unit) lands together, or chaintol is re-baselined with that row named. I'll decide when 3812's dual review is in.

## 2026-10-02 — 3759 settled; 3812 dual review adjudicated

- **3759 settled.** Both delta rounds are in. MINOR-A (a spurious discontinuity at gap position 0) is fixed at `e3d330393`, with a row covering every gap position and `revert()`. I accepted it on my own read: a small fix with its own row.
- **3812 dual review, frozen head 47f6f723.** R1 (comment 5952460526): APPROVE-WITH-FIXES. R2 (comment 5953499653): NOT-MERGEABLE-AS-IS.
  - **Correspondence:**
    - Bilateral: `samples: 0` on point Interval; pin docs measured against the intermediate head; the narrowed dial-set gate; the `MAX_BRANCH_PERIODS` boundary; the `periodic_branch` class left at three sites; the duplicate unlogged door; the fuzz being f64-only and coverage-thin (R1 NOTE-6/S5 sits at NOTE, R2's at MAJOR).
    - R1 only: the sphere twin refusing at Interval (MAJOR).
    - R2 only: the frame premise unenforced over a box (MAJOR, a false accept); 4 of 12 mutants surviving, M8 unsound (MAJOR); CI red on the cut_cylinder sidecar (MAJOR); m10_9's bracket and pad refusing (a regression); stale work notes.
  - The fix list is the union, with all four MAJORs blocking. A confirming review follows the fix pass.
  - `m10_9`: fix the fillet-meridian radius term if it is a spelling question. Otherwise leave it as an asserted, disclosed regression at P0, like chaintol.
- 2026-10-02 — Seam note from TANG: main is red on `bounds_census::every_sole_bracket_bound_door_is_in_the_roster` because `pcurve_cache::circle_image_envelope` (PR 3733) has no roster line. Filed as `circle-image-envelope-is-a-sole-bracket-door-missing-from-the-bounds-roster` (P0, E) on your slate; it reds every geom-core-touching PR. (TANG orchestrator)

## 2026-10-02 — 3759 merged (82b52c36c), carrying 3812

- **Merged** on a green head (025d8a0), after two more merges of main. The DUAL-REVIEW-LOG rows landed with it: DR-51 (3759, excluded under 6(e), one review truncated) and DR-52 (3812).
- **Closed:** S331, `validate-pcurves-cannot-tell-…`, `validate-pcurves-never-recertifies-…`, `site-rows-leaves-an-off-chart-edge-silent`, `extrude-mints-no-pcurve-rows`, `pcurve-certificate-states-incidence-and-fidelity`, SYM's `pcurve-certificate-checks-widen-…` (its residue rowed), and `circle-image-envelope-is-a-sole-bracket-door-…` (fixed on main by 467d4b42f).
- **Regressions that landed, each rowed:** chaintol's certified wall (the tip box about 1.6e6× worse) until `pcurve-loop-decisions-state-a-3d-identity-plus-a-branch-margin`; M10-9's bracket and pad refusing (`fillet-meridian-radius-term-is-registered-only`, P0); the shallow-arc Interval grid (pctail's wide-arc row, given P2).
- **Next:** the loop-decision unit, with a designer pair first (it touches C4's trim containment against the caller's ChartWindow); then the fillet-meridian P0.

## 2026-10-03 — chart angular comparisons: designer pair, [ev] PR 3919

- **Designer pair** dispatched on `pcurve-loop-decisions-state-a-3d-identity-plus-a-branch-margin`, from main at 82b52c36c, with one problem statement and no candidate answers.
  - They agreed on the core: check 5 and the caller's `ChartWindow` retire, and joints and closure state the integer branch and winding.
  - They differed on three points: the pole joint, `chart_boundary`'s polygon, and spline charts. After one reconciliation round they converged (one designer moved on all three; they did not cross).
- **[ev] PR 3919** rewords C4's domain-validity sentence and records DESIGN-FORK-LOG row 54's recommendation half.
  - Provenance: the trim-window sentence was agent-written on 2026-09-03 (585b3422f, the docs-to-README sweep), and I found no ratification. PR 3781 left it untouched.
  - Deviation: the blinding byte (87) was drawn after the reports came in, not at dispatch. This is recorded on `analysis/design-fork/pcert-chart-angles-2026-10-03`.
  - My first commit on that branch overwrote README.md; the next two commits fixed it forward.
- **Renders:** the nightly re-baselined the kernel, uv, mc and freecad cells after 3759 (e416e33de through a9732a08c).

## 2026-10-03 — fillet-meridian: designer pair, not a fork; parked on PATHS 5b

- I dispatched the pair with the byte drawn at dispatch (101, recorded on `analysis/design-fork/pcert-fillet-meridian-2026-10-03`).
- Both designers came back with the same answer: the question is already ratified (D1, PR 3453), and the fix is PATHS 5b's fillet arm. So no `[ev]` PR goes out, and no fork-log row is written (protocol rule 1: only forks that go to Ev are rows).
- The row is corrected and parked on `store-constructed-carriers`. I left a seam note in `work/paths/log.md` asking PATHS to rank 5b.
- My call: don't take 5b into PCERT. It is PATHS' unit, already specced, and its fillet arm is the fix.

## 2026-10-03 — Ev ruled on 3919; implementer dispatched

- Ev approved PR 3919 ("sounds good! nice catch on 2"). Merged as c8ab2e37d. The fork-log row was renumbered 47 → 54 at the main merge, and Ev's decision and the A/B mapping were filled in.
- The implementer is dispatched on `pcert/chart-angle-integers` from c8ab2e37d (session_016i8tmtwuCgNvRV9PXr5GHq). The brief is the C4 sentence plus 3919's two reports, and it lists the measurements owed in the PR body. The unit is cost H, so the dual-review pair runs on a frozen head once CI is green.

## 2026-10-03 — PR 3945 dual review adjudicated

- Frozen head 78e55c70. Both reviews are delivered: R1 (comment 5968113330) and R2 (comment 5968109956). Both verdicts are APPROVE-WITH-FIXES, and both report that lane isolation held.
- **Correspondence pre-note (rule 7), written before the blinded coding returns:**
  - **Bilateral MAJOR:** the sphere twin is decided with zero margin, so at K below about 4 the wrong sheet certifies or is stored shifted by τ. Both reproduced it, by different probes.
  - **Bilateral MINOR:**
    - surviving mutants on the near-pole path, the undecided skip, the winding bound, `identity()` ignoring the twin, and the `joint_arm` lever;
    - stale `trim_containment` / continuity-margin prose;
    - torax's either-or pin.
  - **Unilateral, R2 only:**
    - `kv` dropped (a surviving mutant);
    - one predicate name for two questions;
    - the `pin_branch` wrapper;
    - the audit row's escape wording;
    - mesh12 has no upper bound;
    - the `lift_joint` doc matrix;
    - the on-axis, off-surface lever (a note).
  - **Unilateral, R1 only:**
    - `chart_boundary` cites the 4ε bound for uncertified images;
    - the gate test bypasses `singular_at`;
    - two lever functions;
    - the `loop_closes` combinations.
  - **Tally candidates:** none, since the only MAJOR is bilateral. Expected tally contribution: 0.
- **Fix list (the union) sent to the implementer:**
  - the blocking fix for the twin, following B's design: nearest orbit point, with margin half the separation;
  - the README re-word that follows from it;
  - killing rows for every surviving mutant;
  - the prose sweep;
  - the minors.
- Blinded coding dispatched (session_01JZy2Fs1EhBc7PrvXSvX4AE). The byte and mapping are recorded privately until merge.
## 2026-10-03 — HOLD: a refactor of dependency, placement and intent is underway (Ev, `[ev]` PR #3990)

Ev has opened a redesign of how a document says that one thing depends
on another and that things are meant to coincide. The question and Ev's
direction are `work/recipe/one-way-to-say-dependency-and-intent.md`;
the design lands through `[ev]` PR #3990. The direction, in short: no
node consumes another; no raw numbers (every slot holds a variable);
nodes are operations on typed variables; no absolute coordinates
(spaces are what is related to what, placements are relations); tangency
and coaxiality by construction; checked assertions replace declared
contacts; contact and tangency complaints become lints where the
answer is already known.

**Do not start a new unit that meaningfully uses** any of: the node
vocabulary's edges and consumption (`Node::inputs`, product roots),
`Expr`/document parameters and literals, placement (`Datum`
coordinates, `Transform`, `Pattern`/`PlacedUnion` frames, gauges,
offsets, mates and their solve), declared pairs and declared contact
(`Boolean`/`Union` `declare`, `ContactClass`, continuations, seams),
the undeclared-coincidence and undeclared-contact refusals, axis
declarations, `ParamSource`, the parameter-coincidence lint, or
`Measure`/`Assertion`.

**A unit already started may be finished**, even where it collides with
the above — land it as planned. Park each row the hold covers
(`status: parked`, `blocked_on: [one-way-to-say-dependency-and-intent]`,
so the row fires when the ruling closes). If that leaves your program
with nothing it may start, set its `status` to `blocked` and stop.

## 2026-10-03 — the intent refactor's hold now waits on the build, not the ruling (Ev ratified #3990)

Ev ratified DESIGN.md D10 on PR #3990, and the ruling
`one-way-to-say-dependency-and-intent` is closed. The hold announced in
the entry before this one CONTINUES until D10 is built: it now waits on
`work/recipe/d10-one-way-to-say-intent-is-unbuilt.md`. Every row that
was parked on the ruling or on #3990 has been re-pointed there, so
nothing fires at this merge. Park any further held row with
`blocked_on: [d10-one-way-to-say-intent-is-unbuilt]`. Units already
started may still finish. Read D10 before resuming work on this ground:
coincidence is now a margined verdict (no declarations), checked by the
`unproven-coincidence` lint.

## 2026-10-06 — 3945: re-port onto R

- The usage cap stalled 3945 after its confirming review (APPROVE-WITH-FIXES, 0/2). Both MINORs are fixed locally at ae364e91.
- Main meanwhile landed TOPO's R build (Ev's ruling on PR 4024: a stored per-half-edge joint element, `loop_lift`). That re-architected the code 3945 changes: 45 conflict hunks in `pcurves.rs`.
- TOPO filed the coordination row on my slate. My call: 3945 lands second and re-ports onto R, with one decider and R's storage, not identity-at-every-joint.
- The re-ported head gets a fresh dual pair. Most of the code under review will be new against 78e55c70, so a delta review would not cover it.

## 2026-10-06 — PR 3945 round-2 dual review adjudicated (the re-port onto R)

- Frozen head da1afd1f, a fresh pair because most of the code is new against 78e55c70. The later commit bbee7652 changes only a `work/` item.
- Both reviews delivered: R1 (comment 6015903724) and R2 (comment 6014803528). Both verdicts are APPROVE-WITH-FIXES with no MAJOR. Isolation held: R1 read no comments, and R2 read only those before the cutoff. Both reproduced the `pinch_faces` ε=1e-6 red on main 62bdd557.
- **Correspondence pre-note (rule 7), before the blinded coding returns.**
  - Bilateral MINORs:
    - `split_cache` turns a `decide_joint` refusal into a stored identity;
    - near-pole orbit-point mutants survive (R1 MB/…, R2 m1);
    - the README's C4 closure sentence is narrower than `Winding::closes`.
  - Unilateral, R1 only:
    - the further survivors: a Reset/Shift swap with the same deck, and the site mint's new-half vertex;
    - `chart_boundary`'s `is_zero` leans on a winding the decider's docs concede can be off by one orbit step near a pole or a narrow cone apex;
    - the unit's closure note is stale;
    - NOTEs: claim 1 is overstated (kills, revert and graft write elements by algebra or copy), and `chaintol`'s rows 3–4 moved without being listed, with a guard that can't tell the predicate names apart.
  - Unilateral, R2 only: `Reset` is written on an undecided reading on spline charts. That corrects my brief's premise; no code change.
  - Tally candidates: none, since neither review raised a MAJOR.
- The fix list (the union) has gone to the implementer. The blinded coder is session_01TeTFymE7xDPjooubHwkkPD; the byte is recorded privately.

## 2026-10-07 — PR 3945 merged (4711ad4f): the chart's angle checks are integers, on R

- **What landed:**
  - Check 5 and the caller's `ChartWindow` are retired.
  - One decider, `decide_joint`, decides each joint's deck element on TOPO's R storage. Tier 3 checks every stored element against it.
  - On a sphere the deck element is one orbit integer in half periods, with a quarter period of room. On other surfaces it is whole periods.
  - Poles are decided as 3-D incidence. `Reset` is written only on a decided incidence or an undecided spline gate.
  - A room fence: `chart_boundary` builds only where every joint's winding was decided with room.
  - `split_cache` and `turn_element` refuse rather than store a guess.
  - `chaintol` is back to `[1.0, 0.370, 0.185, 0.111]`, with `dihedral_wedge` as its wall.
- **Review:**
  - Two concurrent pairs. Round 1 on 78e55c70 found a bilateral MAJOR: the zero-margin sphere twin. Round 2 on the R re-port, da1afd1f, found no MAJOR.
  - Two confirming reviews, each APPROVE-WITH-FIXES 0/2, both fixed.
  - DR-95, tally 0. It was renumbered from DR-94 at merge, because SSI #4104 took 94 first.
- **Path:** the usage cap stalled the unit, and main landed R in the meantime. That forced the re-port, plus four merges of main, the last one combining our pins with CARVE #4187's cap orientation.
- **Rows closed by the PR:**
  - the unit itself;
  - `at-rest-trim-containment-…`;
  - the 3945/R coordination row.
- **Left open:** TOPO's kill-straddle row, with a note appended.
- **Next in PCERT:**
  - `fillet-meridian-radius-term-is-registered-only` (P0) stays parked on PATHS 5b.
  - pctail's wide-arc row is P2.
- **Fillet-meridian P0 closed by PATHS 5b (#3774):** the filleted bracket and the pad now certify whole at M10-9's scale at every ε. So all three regressions disclosed at 3759/3812 are resolved:
  - chaintol, by 3945;
  - the bracket and the pad, by 5b;
  - the shallow-arc grid stays as pctail's P2.
- **Priorities set on the remaining slate.** The three C4 route rows (torus general circle, cone section, spline carrier) are P1, per Ev's 3617 ruling that every uncovered class gets a route wired. The spiric / no-fitted classes, the fitted-kind option and the line-seam refusal are P2. The torus general circle goes next, as the nearest analogue of the sphere route (3733).

## 2026-10-07 — torus general circle: designer pair concurs, implementer dispatched

- **Designers:** an Opus/Fable pair (byte 220, on `analysis/design-fork/pcert-torus-general-circle-2026-10-07`). Both recommend the same final state, so this is not a fork and gets no fork-log row.
- **The geometry corrects the row's premise:**
  - A ring torus holds only parallels, meridians and Villarceau circles.
  - The class's "circle ⊥ the axis centred off it" lies on no torus; it only grazes the tube crest.
  - A right circular cone holds no circle but its rims, so `UncoveredClass::ConeSection` is a grazer too.
- **Adopted:**
  - A Villarceau circle gets an exact closed-form image (a Kepler azimuth with the focus on the axis, and an affine minor angle), certified as `MapResidualClosedForm`.
  - `Pcurve::ConeSection` is generalised into one focal-section image with one lemma. Opus leaned to a sibling variant; Fable to generalising. The orchestrator picked generalising, with a stop-and-report if any field would be meaningless for one instance.
  - Torus incidence goes through the existing `ConicTorusHarmonics` (degree 2, not the issue's degree 4), then structural Villarceau gates.
  - A grazer (on cone or torus) refuses typed.
  - `UncoveredClass::TorusGeneralCircle` and `::ConeSection` are deleted.
- **Rejected:** a fitted Hermite image for every band-close torus circle. It certifies at the fallback grade a curve that has a closed form, and it is conditional on the fitted door.
- **Folded in:** the cone lane's schedule `Record::Verdict` is brought in line with C4's witness-lane cross-check.
- **Implementer:** session_01Jf5pbwHG7dwojg183pt2vs, branch `pcert/torus-villarceau-route` from 9645b375. It is an H unit, so it gets a dual review.

## 2026-10-07 — PR 4227 dual review (frozen head 64b7def7)

- **The review pair:** two Opus reviewers ran concurrently. Both returned APPROVE-WITH-FIXES. The pre-note's tally candidates are none.
- **The one MAJOR is in both reports:** the torus incidence test decides "off the torus" from an *upper* bound on the distance. So circles within band of a Villarceau circle get refused as `CarrierOffChart`.
  - **Class finding:** the same one-sided idiom is in the cone's `pcurve_cone_chart_incident`. The sphere's test is to be checked.
  - The fix pass decides Off only from a lower bound, and sweeps all three.
- **Test gaps** (demonstrated by surviving mutants): five envelope terms and every Villarceau gate.
- **Fix pass** is with the implementer: session_01Jf5pbwHG7dwojg183pt2vs.

## 2026-10-07 — PR 4227 merged (65af3eca); spline-carrier designers dispatched

- **4227 merged.** `Pcurve::FocalSection` covers both cone sections and Villarceau circles under one closed-form envelope. Incidence on the sphere, cone and torus is one-sided: Off is decided only from a sampled lower bound. A circle on a torus or cone that is in band but is none of its circles refuses as `CarrierGrazesChart`. `UncoveredClass::TorusGeneralCircle` and `::ConeSection` are retired.
- **Rows:**
  - Closed: the torus and cone route rows.
  - Filed: `the-face-whole-excusal-has-no-fixture-on-an-analytic-chart`.
  - Still open: the incidence issue, for its spline-carrier and no-fitted bullets.
- **Review: DR-102.**
  - The concurrent pair's only MAJOR was raised by both reviewers, so the tally is 0.
  - A single confirming review followed the fix pass: 0 MAJOR, 3 MINOR.
  - The row was renumbered twice at merge (DR-100, then DR-101), because main took both numbers while CI ran.
- **Next P1, the spline-carrier route:** an Opus/Fable designer pair is running. The byte, 122, is on `analysis/design-fork/pcert-spline-carrier-route-2026-10-07`. Sessions: session_01QrKCiRg8fQ31Qwsdm8FQ2c and session_012cb8vsD3QyGVpV4oCTU2cZ.

## 2026-10-07 — spline-carrier route: designers converge, [ev] PR 4261

- **Round 1:** A recommended a Hermite fit on the curved charts and the affine net on the plane. B recommended the *projected image*, `ψ(C(t))`, exact on every analytic chart.
- **Reconciliation:** each designer was shown the other's report. A moved to B's route. B adopted A's routed-verdict point and its reach correction.
  - The orchestrator asked B to check its nearest-point claim on the cone. B found it false there by exactly 1/cos α (the cone's `v` was height), and moved the cone's `v` to the foot's coordinate on the generator.
  - Converged, not crossed.
- **Why it goes to Ev:** the route rewrites C4's `OnLocusHull` sentence, which Ev worded on PR 3781, and the sphere circle's Hermite sentence, which landed with 3733 without an `[ev]` review. PR 4261 carries the diff and fork-log row 85.
- **Still Ev's:** A versus A′ (whether the sphere's general circle leaves the Hermite route); both designers lean A. The P1/P2 question is in the PR body.
- **Off-question findings, filed:** `no-fitted-class-misuse-is-excused-by-not-owed`, `site-rows-derive-through-chart-pcurve-bypassing-the-routed-arm` and `on-locus-hull-is-named-for-charts-whose-metres-composite-refuses`. The excusal-fixture row's `m6_2` claim is corrected.

## 2026-10-07 — PR 4268 merged (6f5c5224): the LINE seam carries its chart row's refusal

- `line-seam-boundary-row-refusal-discarded-as-iso-unsupported` (P2) is closed. The LINE-seam arm now answers `ChartRow { source }`, as its sibling arms do.
- **The sweep found the same discard in SSI's `Pass::curve`, which was fixed.** `SsiError` gains `ChartRow`, numbered in SSI's roster, and the fitted lane maps it through to pcert's `ChartRow`.
- **A third site was filed for SSI** (P3): `ssi-one-arc-side-readers-discard-the-side-rows-refusal`.
- **The test the brief asked for cannot exist:** a mismatched net cannot reach the arm. A source-preservation test stands in for it.
- **Review:** orchestrator's read (an S unit, a one-line repair plus its sibling).
- `[ev]` PR 4261 (spline-carrier route) is still awaiting Ev.

## 2026-10-08 — Ev rules on 4261; merged (529a47bd); projected-image implementer dispatched

- **Ev's question:** first asked whether the PR concerned the intensional description or the cache. Answered: the cache, and D2 is unchanged.
- **Ev's ruling** ("ok all the recommendations sound good!"):
  - the projected image goes on every analytic chart, with the sphere's general circle included (A);
  - `OnLocusHull` and the Hermite route retire on analytic charts;
  - the tube belongs to the edge's certificate;
  - P1 is kept.
- **Record:** fork-log row 89 is completed (renumbered from 85 at merge, because main took rows 85–88).
- **Implementer:** session_01Ht23dQzhB1yisWXxau9tZF, on branch `pcert/projected-image` from 981fd34e. It is an H unit, with at most two PRs, and gets a dual review. It first establishes where the tube lives at rest.

## 2026-10-09 — PR 4304 merged (111782d6): the projected image on every analytic chart; dual review, three follow-up reviews, DR-116

- **Dual review** at `fdc8ea9b` (byte 13, odd: R1 = B, R2 = A). R1 NOT-MERGEABLE-AS-IS, R2 APPROVE-WITH-FIXES. Both found the same MAJOR, so the tally stays 0.
  - **The MAJOR:** retiring `OnLocusHull` dropped limbs 1–2 from every edge whose faces store no row. A plane face, or a mate named only in the description, left the edge with only the tube. Red probes on plane × cylinder.
  - **Class finding:** *the edge certificate must not rely on rows existing.* `edge_nurbs::analytic_rung3` now states limb 2 against each analytic operand at every door holding the lane.
- **A 13-item fix pass**, then a confirming single-Opus review: MERGEABLE-WITH-FIXES.
  - Limb 2 read the whole net, so an edge refused on geometry past its ends. The derivation's incidence gate and the piece-granular `met` did too. Class: *whole-net reads where the edge's interval is owed.*
  - An unratified no-lane sentence had entered C2 in the fix pass. It was removed, and the lane-free doors were filed as `lane-free-doors-skip-the-analytic-rung3-limbs` (P2).
- **Two delta reviews:**
  - The new cut and the hull refinement formed f64 knot-insertion plans at `Interval`, which is the item-9 class again. Fixed by windowing without a cut, breaks in the composite's own ring, and an exact equal-weight piece. The rational tube reads its whole net (P3). The tube chain's own f64 refinement is pre-existing and filed P2.
  - The implementer had reported "a real hole" in min/max over brackets. The reviewer showed there was none: Interval min/max propagate the refusal. The guard stays, its rationale is corrected, and MUT9 is pinned on a cylinder.
- **CI along the way:**
  - a pncad-py prose-census miss, fixed;
  - main's demos goldens left unrun after #4353's blend join, re-pinned on main (#4383 merged, plus d5a518b1b);
  - one default-ε `test` failure on `7c7b15e2` that never reproduced locally and could not be read, because the Actions log blob host is denied by the session network policy (Ev told). It was green on the next head.
  - a semantic conflict with JOIN's new near-pole row, which named the retired `ArcNearPole`. Its δ = 1e-7 cut now builds, with every volume holding.
- **DR-116:** recorded as DR-114 on the branch, renumbered twice (main took 114 and 115 first). Fair pairs that found any MAJOR: 54.
- **Next on the slate:** the mirror-torus spiric / no-fitted classes, the filed P2s (`lane-free-doors-…`, the tube chain's f64 refinement), and `a-reimported-spiric-rim-refuses-at-the-edge-tube`.
- 2026-10-09 — Seam note from ENCL (PR 4422, merged): `geom_brep::recourse::RefusedArm::SignCertain` now takes `Option<MarginDiag>`; construct with `SignCertain(None)` unless the decision is a residual miss, and match with `SignCertain(_)`. `certify::definite_miss_in_file` / `Unsized::definite_residual_in_file` are gone; `Unsized::residual_in_file` is the one door. (ENCL orchestrator)
- 2026-10-09 — Seam note from ENCL (PR 4427, `encl/limb-refusal-margin`, merged at `e3646226c0`): `SsiError::CertificateLimb`, `PlaneNurbsRefusal::Limb` and `AnalyticRung3Refusal::Limb` now carry `margin: MarginDiag` beside `value`, minted by `decide_reported`; the decision bit is identical to `decide`. Match them with `..`. The margin is for error text only: never branch on it (Bounds clause 2). `MissReading::DefiniteUnvalued` is gone, and `geom_core::FileCoincidence::new` now takes only `eps_in`. A new limb mint on your ground should use `decide_reported` and carry the margin, so that the import door can say whether the miss lies within the file's ε_in. Known gap: a hull-sup limb's margin is an upper bound, read today as the miss (ENCL row `hull-sup-limb-reads-its-bound-as-the-miss-at-the-import-door`). (ENCL orchestrator)
- 2026-10-10 — Seam note from ENCL: filed `pcurve-envelope-bound-ends-as-a-stored-contradiction` on your slate, from PR 4504's review. `Unsized::LastResort` is renamed `Fit` there, and a new `Unsized::Bound` ends a certified-bound refusal in the last resort at every reading. Your `PcurveCheck::Envelope` looks like a bound. (ENCL orchestrator)
- 2026-10-10 — Seam note from ENCL (PR 4504, merged): `geom_brep::recourse::Unsized::LastResort` is renamed `Unsized::Fit`, with the same behaviour, and a new `Unsized::Bound` ends a refusal on a certified upper bound in the kernel-limit last resort at every reading. At the import door, `MissReading::Bound` names the bound. `CertCheck::bounds_a_miss()` answers whether a check is a bound. A table on your ground that refuses on a bound, not a measured miss, should take `Bound`. (ENCL orchestrator)
- 2026-10-10 — Seam note from ENCL (PR 4520, merged): the PlaneNurbs limbs are decided once, in the lane (`ssi::certify` `nurbs_limbs`). `geom_brep::certify::run_checks` no longer re-decides them, and the k-stream names `plane_nurbs_on_locus` and `plane_nurbs_hull_sup` are gone. The certificate's `max_residual` is bit-identical. (ENCL orchestrator) Filed `fitted-pcurve-envelope-re-decides-the-ssi-hull` on your ground: `pcurve_cache::run_fitted_checks` re-decides `ssi.hull_sup` the same way.
- 2026-10-10 — Seam note from ENCL (PR 4529, merged): `SsiError::CertificateLimb`, `PlaneNurbsRefusal::Limb` and `AnalyticRung3Refusal::Limb` no longer carry `value: f64`; the margin is the only copy and their Displays print it. Refinement routes on a crate-private refused residual recorded with the refusal (`LimbRefusal`), not on the margin. The step-budget ending reads a decided `RefineStop::StepBudget { trend: ResidualTrend }`, and `STEP_BUDGET_FLOOR_RECOURSE` now says "the refused residual stopped falling". `RoundMargin` and `pcurve_cache::FittedMagnitude::LimbResidual` hold `MarginDiag`. Tests that read a margin's upper end use `geom_core::test_support::upper`, the one helper. This supersedes the earlier note saying the margin sits "beside `value`". (ENCL orchestrator)
- 2026-10-11 — Seam note from ENCL (PR 4675, merged at `e423573ba9`): `pcurve_cache::projected_hull_lane` takes a new `cuts: &[f64]` argument (extra parameter cuts, chained after the uniform part breaks), threaded through `FittedLane`'s `projected_lane` field. `projected::distance_floor` and `Meters` now give a certified distance floor on every stored analytic chart: the torus divisor is `max((m+r)|M²−r²|, (M+r)|m²−r²|)`, which is exact on ring, horn and spindle tori. A non-positive torus lever refuses `SectorRefused{Lever}`. Filed here by the lane: `projected-check4-incidence-refuses-on-fixed-parts-without-halving`. (ENCL orchestrator)
- 2026-10-12 — Seam note from ENCL (PR 4756, merged at `730e342025`): `IsoFamilyRefusal::Undecided`'s Display is payload-only (no ending), and its question has one spelling, the new public `geom_brep::ISO_FAMILY_CLAUSE`. No product reader uses that Display now; the join writes its own sentence on the same clause. Filed on HONE: `PcurveCertifyError::ending` is never called for a join's `Image` arm, although `Escalated` can reach it through `chart_iso_family` → `derive_harmonic` → `derive_class`. (ENCL orchestrator)
- 2026-10-12 — Seam note from CLEAVE: unit `cleave/a-section-ellipse-is-re-posed-at-its-nearest-principal-vertex` will change `Curve3::Ellipse`'s convention to "`u_ref` is a principal direction". It renames the semi-axis fields so neither claims "major", and retires `AxesSwapped`. Section chords put their seam at the principal vertex nearest their arc, with a signed span. It touches `geom/src/curves.rs` (CURVED), `geom-brep/src/intersect.rs` (GERM) and `chart_pcurve`'s focal-section image (PCERT), read in the major frame through `t0`. The PR will announce the seam. Watch for any reader of yours that assumes `u_ref` lies on the major axis.
- 2026-10-12 — Seam note from NURBS: PR 4866 (`nurbs/one-derivative-scale`) gives the span derivative-net scale `p/(u_{i+p+1} − u_{i+1})` one home, `geom_core::spline::derivative::span_derivative_scale`, beside a moved `derivative_net`. It routes the inline copies in `topo/src/shell.rs` `spline_speed`, `geom-brep/src/pcurve_cache.rs` (the surface net and `curve_rate_bound`) and `geom-brep/src/ssi/enclose.rs` (`pair_scales`, `bezier_scales`) through it. The claim is bit-identical, with no golden moved, and it is under single FULL review. (NURBS orchestrator)
- 2026-10-12 — Seam note from ENCL: PR 4874 merged (`encl/fitted-escalated-keeps-decision`).
  - `pcurve_cache`'s `FittedEscalated` keeps its SSI decision (`SsiDecision {Trace, Limb}`) and ends through `SsiDecision::ending`, which takes `impl Into<ReadAt>`.
  - Its Display and `PcurveCertifyError::Escalated`'s now end in the decision's own labelled recourse at `Reading::Build`, so the doors that print the Display (`PcurveMintError::Certify`, step-import, `replace_face`, loft, blend) carry one ending each. Both texts are pinned literally.
  - `PlaneNurbsRefusal::ending` ends `Escalated` through `SsiDecision::Limb`.
  - Rows filed:
    - PCERT `a-villarceau-tangent-parts-gated-zero-ends-as-a-kernel-defect`;
    - CLEAVE `chart-pcurve-and-props-escalations-lose-their-check-at-split-join-and-point-in-solid`;
    - `work/issues/describe-maps-every-chart-pcurve-escalation-to-chart-image`;
    - ENCL `unsized-recourse-takes-a-gate-tagged-arm-and-renders-no-gate-word`.
  - Pointers were added on the PCTAIL coincidence-menu row, the CHROME escalations-forwarded-whole row and the EXCH step-import row. (ENCL orchestrator)

- **2026-10-12 — seam note from NURBS: PR 4892 merged (one rational speed bound, `geom_core::spline::rational_speed_sup`, `NurbsCurve*::speed_upper_bound`).**
  - `pcurve_cache` `curve_rate_bound` (`seam_envelope`) reads the door, so the curve sup no longer applies the weight ratio².
  - Its only consumer is `slack_param`, an overshoot meter.
  - PR 4904 (part 2, in review) routes `nurbs_stretch_bounds` and the `chart_stretch_inf` sups through `NurbsSurface::stretch_sup`. The infs keep `weight_ratio_factor`. Expect prose edits in `pcurve_cache` there. — (NURBS orchestrator)

- **2026-10-12 — seam note from NURBS: PR 4904 merged** (one rational surface bound: `geom_core::spline::rational_stretch_sup`, `NurbsSurface::stretch_sup`, `RationalPairTerm`).
  - `pcurve_cache` `nurbs_stretch_bounds` and the `chart_stretch_inf` sups now read `stretch_sup`.
  - The infs still divide by `weight_ratio_factor`. That floor is unproven: `work/nurbs/chart-stretch-inf-rational-floor-is-unproven` (P2) is next on NURBS's slate, and it will touch `pcurve_cache` again.
  - The stale "sup multiplies by the weight ratio" prose is fixed. An infinite sup arm now means a weight rounded to 0. — (NURBS orchestrator)
- 2026-10-13 — `focal-section-vertex-azimuth-is-read-off-a-cancelling-sum` closed on `pcert/focal-section-vertex-azimuth`: the cone arm reads `u0` off `ra`. 1646 of 3600 cone-section mints in the six crates change `u0` bits (the patch-memo keys of those images move with them), and none misses by more than before. The parent (GERM) and the two sibling rows carry notes. Filed `cone-section-interval-mint-escalates-on-a-beta-enclosure-of-u-over-k-levered-near-a-parabola` and `torus-villarceau-arm-reads-its-vertex-direction-off-a-small-vector`.

- **2026-10-13 — seam note from NURBS: PR 4923 merged.** `geom_core::spline::weight_in_regime` (positive and finite) and `check_weight_values` are now the one weight predicate. Every knot-algebra plan that writes a weight (insert, refine, elevate, removal, surface twins) refuses with `WeightCollapse` through `written_weight`.
  - Your file's own spelling of the positive-and-finite check now reads `weight_in_regime`. Behaviour is unchanged. — (NURBS orchestrator)

- **2026-10-13 — seam note from NURBS: PR 4928 merged (proven rational stretch and area floors).** `chart_stretch_inf`'s rational infs now read `NurbsSurface::{stretch_floor, area_floor}` (`geom_core::spline::{rational_stretch_floor, rational_area_floor}`, proofs in the derivative module docs). `weight_ratio_factor`, `skipping_surface_derivative_net` and `floor_weights_admissible` are deleted, and `net_direction` reads `derivative_net`.
  - Reach: a rational chart whose in-cell weight spread passes about 1.7× now refuses the floor (the area floor binds). That was unproven before, not wrong.
  - `rational-inf-floor-reach-refines-before-reading` (P2, NURBS) is the way back. — (NURBS orchestrator)
- 2026-10-13 — Seam note from ENCL: PR 4925 merged. ε now has a floor: geom-core `tolerance::SESSION_BOX_EXTENT` = 1 km and `EPS_MIN` = box · 2⁻⁵² ≈ 2.2e-13 m, per D4 ¶4 (Ev, PR 4836).
  - Every door that sets a run's ε refuses below it. `tolerance::EpsFloor` is the one wording of the refusal.
  - `Band::offer_deciding(m)` is the one offer rule: no tighten offer at or below ε_min. Tests that predict an offer should call it rather than restate `|m|/K`.
  - At CI's 1e-12 row, every zero-band offer now withholds.
  - A run below the floor (for example `CAD_TOLERANCE_EPS=1e-14`) is refused. Outside geom-core it silently falls back to the default; that is filed on SCALAR. (ENCL orchestrator)
- 2026-10-13 — Seam note from ENCL: PR 4943 merged. A sized decision's ending now depends on one classification of the reading, `MarginDiag::shows` (geom-core): Offer, NoSize, Floor, Unresolved, Past or Unreadable.
  - The decision's note (`SizedWords::no_size`, renamed from `at_zero`/`otherwise`; geom-brep `NoSize`, renamed from `AtZero`) appears only on NoSize: an exact zero, the refused side, or a theorem.
  - A positive reading below the floor gets the one `floor_note` sentence.
  - A reading that touches or straddles zero gets the lever alone.
  - The import door gives its sentence plus the lever, and never a decision note.
  - PR 4855's rejected-sign filter is gone.
  - New sized decisions take `no_size:`.
  - Tests that predict an ending should call `MarginDiag::sized_recourse`, not restate the rule. (ENCL orchestrator)

- 2026-10-13 — **Seam note from NURBS: PR 4954 merged** (`092b30227f`). Every first-order rational derivative bound now reads one Bernstein quotient bound on exact dyadic Bézier pieces (`geom_core::spline::derivative::QuotientPiece`, with the pieces in `geom_core::spline::pieces`).
  - **Retired:**
    - `rational_speed_sup`, `rational_stretch_sup`, `rational_stretch_floor` and `rational_area_floor`;
    - `RationalPairTerm`, `pair_sup`, `Cell` and `over_cells`;
    - enclose's `pair_norm_sup`, `Centred`, `Affine` and `screen_bound`;
    - `pcurve_cache`'s `net_direction` and `floor_along`.
  - **API change:** `NurbsSurface::stretch_floor(along_u)` and `area_floor()` are norm floors and lost their `d` argument.
  - **Tighter readings:** `speed_upper_bound`, `stretch_sup`, `chart_stretch_inf` and enclose's `deriv_box` / `speed_sup` / `transverse_readings`. So iso slacks shrink and floors rise.
  - **Degree ceilings:** curves meter to degree 28, surface sups and per-axis floors to 28 per axis, and the area floor to 18. Above those the bound refuses.
  - **Filed:** `work/ssiedge/a-tighter-strip-box-refuses-a-side-the-roots-decided.md` and `work/nurbs/binomial-bernstein-product-and-blossom-have-two-homes-each.md`.
  - **Next:** NURBS unit 2 moves `patch_bound` `rational_cells` (first and second order), mesh chords and `offset_meters` onto the same form. — (NURBS orchestrator)
- 2026-10-13 — `cone-section-interval-mint-escalates-on-a-beta-enclosure-of-u-over-k-levered-near-a-parabola` closed on `pcert/cone-section-interval-beta`: the cone arm reads `√(1 − ecc²)` as `|rb|/|ra|`. Every escalating row of its table certifies at `Interval`; at `f64`, 2351 of 3582 cone mints in the six crates change `β` bits (the patch-memo keys move with them) and none misses by more than 1.2× before. The parent (GERM) and the flat-cone row carry notes. `pcurve-cache-endpoint-reads-outgrew-the-certified-endpoint-roster` and `pcurve-cache-gained-three-endpoint-reads-the-census-does-not-count` closed (fixed by PR 4953).
- 2026-10-13 — **Seam (ENCL):** PR 4976 merged. Every geom-brep Zero-arm payload (`CertifyError`, `TorusConvention::refused`, `MeterError` NormalFloor/CurvatureHeadroom) now reads "is not shown … (margin {:e} m)". Whether a reading shows no size is left to the ending (`MarginDiag::shows`). A row PR 4976 filed on this slate asks for the same idiom on this ground. (ENCL orchestrator)
- 2026-10-14 — closed `focal-section-envelope-bounds-the-whole-ellipse-not-the-span` (overtaken by PRs 4960 and 5004: no pose refuses at the envelope; across 3,120 certificates a span bound could gain at most 2.2×) and `torus-villarceau-arm-reads-its-vertex-direction-off-a-small-vector` (premise wrong: `c − center` is one correctly rounded subtraction, and the miss tracks the carrier's own `u·L`, not the item's `R·u·|c|/r`), on `pcert/close-overtaken-cone-items`.
- 2026-10-14 — closed `nurbs-stretch-bounds-round-to-nearest`: f64 `stretch_sup` is an estimate by D4 ¶2 and the Interval lane is the enclosure. In the census (geom, geom-brep, topo, half of sweep, all of step-import) the f64 value always sat inside the Interval enclosure, about one ulp below its upper end at the median. Filed `exch/surface-promotion-claims-a-certificate-from-a-round-to-nearest-bound`.
- 2026-10-14 — closed `projected-check4-incidence-refuses-on-fixed-parts-without-halving`: the fixed 8-part incidence read is within about 1.2% of the true sup on honest carriers; no suite read lies in [0.1ε, 10ε]; no envelope exceeds the band.
- 2026-10-14 — closed `projected-row-reads-its-chart-edge-through-the-box-not-its-eval`: a spline's interval `eval` over a whole edge is the entire line at `Trv` (the denominator `Σ Nⱼ·wⱼ` encloses zero once the sums lose the partition of unity), and several times too wide inside, while `chart_box` is the exact hull over a whole net and reads whole pieces over a sub-range. The reason now sits at `chart_edge`, guarded by `trim_3_chart_bound::t9b_…` and `t9c_…`. Filed `knot/nurbs-curve-interval-eval-loses-the-partition-of-unity-over-a-wide-t`.
- 2026-10-14 — **Seam (NURBS):** the sweep digest row `tilted_cut_upper` has been red on main since 5f0d1cf5, the merge of #4952 (`pcert/envelope-radial-pair-whole`). A clean bisect with a fresh worktree per commit and `-p sweep` passes at every commit before it. NURBS build A is re-cutting the table in a separate PR that cites #4952 as the cause. If #4952 meant the envelope to stay put on that pose, say so on that PR. (NURBS orchestrator)
- 2026-10-14 — **Seam (ENCL):** PR 5023 merged. At a build, a refused reading past the band now ends like its in-band sibling (D4 ¶1 (iv)): the lever plus the decision's `no_size` note. geom-core `MarginDiag::shows` gains `Shows::Refused`. `pcurve_cache.rs` ParamSpan follows through `certify::recourse`. (ENCL orchestrator)
- 2026-10-14 — `[ev]` band-sized lever (chart-winding + band-scale torus): the cone lane's step 0 found it a fork (census of 1,781,930 classifier calls: EZE ×2 on the apex split row, ZZZ ×2 on a cone rim returning `Neg` for a stored 0, DDD ×1 by design; the winding read by no certifier on a cone/cylinder; a ladder fix escalates next at `pcurve_azimuth_period`, margin 9.41e-9). Two designers (round 0 disagreed on a stored type vs an exact compare, convention placement and in-band class gates; round 1 converged on every point). Fork-log row 142.
- 2026-10-14 — **Seam (ENCL):** the import-validation design question Ev asked for is filed as exch `can-an-import-reach-a-state-only-a-kernel-bug-otherwise-reaches`. Ev named your `[ev]` PR 5113 (decision 2) as the same question as ENCL's `[ev]` PR 5134, and both are listed on that row as waiting on it, so it gets settled once. (ENCL orchestrator)
- 2026-10-14 — **Seam (ENCL):** PR 5138 merged. `CertCheck::ParamSpan`'s lever now reads "move the geometry so this edge runs forward over a clear length", one word longer. A pcurve span routes to the same lever; the quote in your `pcurve-interval-meter-refuses-under-the-span-decision` row is updated. (ENCL orchestrator)
- 2026-10-14 — `[ev]` 5113 decision 3: Ev asked for a general principle (booleans vs the pcurve class gates, the lint a separate stage). Two designers converged in two rounds on lane choice vs structural act, stated in D3 with a D10 sentence; `Grazer` goes; incidence ends by carrier authority; one sub-point split (a lane choice with no lane yet: sized where the definite arm exists, or always lever-only). Fork-log row 144.
- 2026-10-14 — design fork (row 143): how many whole periods an image may sit off what it is compared with. PR 5126's reviews found `MAX_BRANCH_PERIODS` refusing valid bodies downstream of the walk (check 4 via `chart_boundary` and the attach path; the torus point-in-solid walk) and PR 5137's review found its per-side seam images against C4's one-image seam. Two designers converged in two rounds: no cap, check 4 modulo the deck, the walk from the gap's estimate, the harmonic image placed over the span, a v deck on nets closed in v. `[ev]` PR opened; 5126 and 5137 held.
- 2026-10-14 — **Seam (ENCL):** the import-validation design question Ev asked for is filed as exch `can-an-import-reach-a-state-only-a-kernel-bug-otherwise-reaches`. Ev named your `[ev]` PR 5113 (decision 2) as the same question as ENCL's `[ev]` PR 5134, and both are listed on that row as waiting on it, so it gets settled once. (ENCL orchestrator)
- 2026-10-14 — **Seam (ENCL):** PR 5138 merged. `CertCheck::ParamSpan`'s lever now reads "move the geometry so this edge runs forward over a clear length", one word longer. A pcurve span routes to the same lever; the quote in your `pcurve-interval-meter-refuses-under-the-span-decision` row is updated. (ENCL orchestrator)
- 2026-10-14 — `[ev]` 5142: Ev asked for an angle type that compares modulo the period. Two designers converged in three rounds (round 2 a crossover on the estimate): `Period` per channel owned by the chart, `Azimuth` exit from `atan2` with an anchored `near` read, radians, no wrap; check 4 needs no integer (form-identical placement), a lifted image is the (row, deck) pair, the walk reads the marks from zero with no estimate. C4 text updated; fork-log row 145. Filed `chart-boundary-recertifies-a-lifted-iso-line-off-the-knot-domain`.
- 2026-10-15 — `[ev]` 5142 answered: Ev took the recommendation ("i think this sounds good"). Fork rows 143 and 145 filled; needs_ev dropped. 5126 and 5137 unheld for one consolidated fix pass against it.
- 2026-10-15 — `[ev]` 5113 answered: decision 1 accepted; the special-case principle accepted ("good for now"; Ev may later prefer a sharp cutoff at ε with in-band cases as a lint); edge join records class Zeros; lane choice with no lane: lever-only "not yet" (B). Decision 2 split out to `a-torus-faces-convention-is-decided-where-the-face-is-admitted` (needs_ev, waits on exch's import-validation question). Fork rows 142 and 144 filled.
- 2026-10-15 — `[ev]` question: Ev asked whether the cylinder's meridian exception (D3: "or a closed form whose own envelope carries the miss") is consistent. It exists only because a line has no projected lane. Recommended: in band always takes the general lane, every class gets one (a projected line lane on a cylinder), the clause goes. Filed `a-line-on-a-cylinder-has-no-general-lane` (needs_ev).
- 2026-10-15 — `[ev]` 5186 answered ("sounds good!"): D3 has no closed-form exception; the line lane joins unit E.

- 2026-10-15 — **No live row is at or above this track's own band (P0).** The 35 rows left are 28 at P3, 3 at P4, 4 with no band. Consider wrapping up: land the work already in flight and close the program by its exit walk. Failing that, at least re-home the low-priority rows to a program whose band matches them (by `git mv`, ids unchanged, per the closing rules), and do not start any more low-priority work here.
  Band the 4 unbanded rows first: if one of them turns out to be at or above P0, this note does not apply.
  (tracker sweep)

- 2026-10-17 — B1's frame lane is decided across the window's cut (two designers concurred independently; (a) rotated read and (j) per-frame names withdrawn).

- 2026-10-20 — B2b (PR 5343) merged: check 4 decides no whole period on any lane (a lifted copy refuses as its fidelity term), S1 through `Period::times`. The B2 item closes; B1, B2a and B2b all landed.

- 2026-10-20 — Unit C (PR 5444) merged: a spline net closed in `v` has its own `v` period, so a seam keeps one image; both net gates read reach in metres; an undecided `v` gate refuses typed. PR 5137 closed in its favour. Next: E1 (lane choices end as lanes) → E1b (EdgeAuthority in topo endings) → E2 (cylinder line lane, D3 exception out) → E3 (edge_join coincidence).
- 2026-10-25 — **Seam (PRED):** branch `pred/period-turn-rule` (unit 1 of `period-headroom-margin-has-no-shared-home`) touches `geom-brep/src/pcurve_cache.rs`: `Period`, `WalkStop` and `BranchMiss` move to `geom_core::period` and are re-exported from `pcurve_cache` unchanged (no caller re-spelled); `Period` gains `turn`, the shared whole-turn rule. (PRED orchestrator)
- 2026-10-25 — **Seam (PRED):** PR 5710 (`pred/period-turn-sites`, unit 2 of `period-headroom-margin-has-no-shared-home`) edits `pcurve_cache/projected.rs` check 2 and adds `pcurve_cache::azimuth_lever_inf` beside `azimuth_lever`. Check 2 reads each channel's sweep against one turn through `geom_core::period::Period::turn`: the cylinder's azimuth and the torus's tube at `Exact`, the sphere's, torus's and cone's azimuth at `Bounded(azimuth_lever_inf, chart_sup_arms_at)` (new straddle row `pcurve_azimuth_period_straddle`). An overshoot only the sup reads past the band now escalates instead of refusing `AzimuthPeriodExceeded`. No hunk overlaps PR 5500's. (PRED orchestrator)
- 2026-10-26 — E1 (PR 5500) merged: lane choices end as lanes. A class reading at derivation is `ChartClass` and the general lane's own mint refusal `GeneralLane`, both lever-only "not yet"; a class gate is reached only behind the chart's incidence test (`incidence::Incident`); `projected::has_lane` is the one home of the pairs with a general lane (a circle on a sphere, a spline on an analytic chart); `Grazer` gone. Concurrent pair (byte 149), fix passes P1–P12 and Q1–Q12, two main merges. Filed: E1c (plane and cylinder mints read incidence), E4's every-class invariant, E5 (mint-side class pick). Next: E1b (EdgeAuthority in topo endings) → E1c → E2 → E3 → E4.
- 2026-10-27 — E1b (PR 5797) merged: incidence ends by `EdgeAuthority` on sphere/cone/torus. `IncidenceRead` (Off / AtPose / Undecided) is the one classifier and `IncidenceRead::ending(authority, at)` the one map, read at build (`render_by`; `render` is the derived kind), at rest (`validate::off_face_ending`) and by the edge certificate's declared arms (`CertCheck::DeclaredIncidence` → `Ending::Incidence`, `certify::recourse` now `pub(crate)`); `pcurves::certify_refused` stamps the edge's `AuthorityKind`. Concurrent pair (byte 32), fix passes R1–R9, S1–S8 and T1, one delta verifier, two main merges. Filed: `who-placed-it-is-read-off-the-description-not-the-door` (S9–S14: builder declarations, at rest, `set_edge_curve`, MappedSource, the on-pair rim, edge_join, the boolean walk, the import door, who stamps), which a designer pair weighs before E1c; main reds met on the way filed under `work/flux`, `work/tcost` and `work/issues/main-reds-without-an-item-met-by-pcert-e1b.md` (P1: a wrong spline-operand volume at 1e-6). Next: the designer pair → E1c → E2 → E3 → E4.
- 2026-10-27 — **Design fork `who-placed-it`** (`who-placed-it-is-read-off-the-description-not-the-door`), weighed by a designer pair through two reconciliation rounds; fork-log row 201; `[ev]` PR open. Agreed: incidence ends by the door that reads it (D4 ¶1 (iv)); a read in band ends in the lever at every door; `EdgeAuthority` is provenance only; the README D3 sentence (from PR 5113's agent-written body) is replaced. Split for Ev: which reading a build defaults to (the caller's, with kernel crates opting into a kernel door, or the kernel's, with a caller's spec opting into `ReadAt::Input`), and a definite margined miss at import (the damaged-file ending, or `residual_in_file`'s stopgap). Evidence the designers verified against 86589d04:
  - E1b regressed import: an imported curve off its face now reads "move the declared curve onto the face it bounds", not the file's ending. Fixed with the ruling.
  - A derived carrier at a pose or undecided ends `Unsized::Defect` today, against [verdicts.consumers] and DESIGN's at-rest sentence; fixed with the ruling. The same at-rest undecided-defect tension holds for `ChartWinding`, `ImageIdentity` and `Sector`: its own item, to file.
  - The item's S12 path is not real as stated (`pcurves::not_owed` exempts only `FittedLaneUnsupported` with no fitted lane).
  - Sweep hands topo a curve spec at about 45 sites (`mef`, `mev`, `mev_line`, `set_edge_curve`); its `DOORS` census lists 3.
  - `pncad` re-exports `topo` whole, so `Body::set_edge_curve` is public API.
E1c waits on the answer.
- 2026-10-27 — **`who-placed-it`, round 3** (prompted by Ev on the `[ev]` PR, "how can we just make it not happen?"). Both designers independently answered: make topo's construction tier (the Euler operators, `Body::set_edge_curve`, the description setters and their spec types) the kernel's internal API, and have pncad stop re-exporting it. A build's miss is then the kernel's by construction, with no default, lint or `ReadAt::Input`. B added a further step, which A accepted as a later unit with its gate: a refusal's `Display` carries only its payload, and its ending exists only through `render(at)`. Five `Display` impls spell `render(Reading::Build)` today. Before cutting that unit, check whether the refusal-shape guard and the step-import fixtures read `Display`. Q2 (the margined miss at import) is still Ev's. Follow-ups once ruled: `policy_lane::Origin` collapses (span refusals); and check whether pncad-py or sweep's public API exposes topo spec types.
- 2026-10-27 — **Ev ruled `who-placed-it` (`[ev]` PR 5838).**
  - Q1: the designers' round-3 answer. Topo's construction tier becomes kernel-internal (pncad stops re-exporting it). A build's miss is the kernel's defect. A read in band ends in the lever at every door. No ending reads `EdgeAuthority`.
  - Q2: "i don't care what it says while this functionality doesn't exist". An incidence refusal at import takes the damaged-file ending, without ε_in, until EXCH's healing lands. The healing and the later ε_in wording are tracked on `work/exch/adoption-rebuilds-caches-at-eps-from-eps-in-interpretation.md`.
  - New units: E1d (the ruling) and E1e (payload-only `Display`), ahead of E1c.
- 2026-10-27 — **E1d (`pcert/door-endings`), the endings of Ev's PR 5838 ruling.**
  - E1b's authority routing is out. `IncidenceRead::ending(at)` is the one map.
  - A definite miss ends by the door: the kernel's defect at a build, the stored culprit at rest, the damaged file at import with no ε_in.
  - A read in band ends in the geometry's lever at every door.
  - The boolean loop walk reads an at-pose read as unread, and the edge join ends an incidence refusal by its read.
  - Sealing the construction tier is split out to E1d2 (the orchestrator's call: inherent methods travel with `Body`, so re-export curation alone cannot seal them).
  - Filed `policy-lane-origin-callers-outlives-the-sealed-construction-tier` and `stored-contradiction-at-a-build-still-tells-the-lever-story`.
- 2026-10-27 — **Sequencing: E1d is split.** The implementer found that topo's Euler operators and setters are inherent methods on `Body`, so curating pncad's re-export cannot hide them. `pncad::profile`'s doc names the same limit. E1d lands the endings, and a new unit E1d2 seals the tier: a pub trait `topo::construction::Construct`, with the inherent methods made `pub(crate)` (the `profile` precedent). Until E1d2 lands, an external `set_edge_curve` miss reads as a kernel defect; no product code calls it. The alternative, sealing inside E1d, was rejected: the churn (about 160 files importing the trait) would bury the endings E1c waits on.
- 2026-10-27 — **E1d2 (`pcert/seal-construction`): topo's construction tier is sealed.**
  - The doors are behind `topo::construction::Construct` (inherent `pub(crate)`), and their vocabulary lives in `topo::construction`.
  - `pncad::topo` is the glob minus that tier. Kernel crates import the trait.
  - The source walk counts the sealed doors (`construction_tier`), so the tier-1 and pcurve-posture censuses still range over them.
  - Seam logged on `work/lib/log.md`.
- 2026-10-27 — E1d (PR 5844) merged: an incidence refusal ends by the door that reads it (Ev's ruling, PR 5838). E1b's authority routing is out, and `EdgeAuthority` is provenance only. `IncidenceRead::of` is the one classifier, `ending(at)` the one map and `is_definite` the one policy. Their endings:
  - a definite miss is the kernel's defect at a build, the stored culprit at rest, and the damaged file at import (no ε_in);
  - a read in band ends in its lever at every door;
  - a poisoned margin is a defect.
  Every holder ends a refusal by its own ending, and only a genuine incidence reading produces one. The review was a concurrent pair (byte 26, tallied 2), then fix passes G1–G10 and H1–H8 and one delta verifier. Handed on:
  - E1c is widened to the sphere/cone/torus class arms and the sphere/torus ellipse kind gates;
  - E1d2 seals topo's construction tier.
  Next: E1d2 → E1c → E1e → E2 → E3 → E4.
- 2026-10-28 — **E1c (`pcert/mints-read-incidence`): every mint arm reads incidence before it images.**
  - `derive_class` opens the plane, cylinder, cone, sphere and torus arms with the chart's incidence test. The general lane takes the witness (`general_lane(shown, undecided)`). The sphere and torus kind gates send an ellipse not shown off to the general lane.
  - The spiric's plane cap and torus wall read it too (a sweep hit). `topo::chart_region`'s plane site ends an incidence refusal by the read (`TrimOffFace`).
  - A carrier off its face ends by E1d's door rule, never `ChartResidual`.
  - Moves: E1b's `set_edge_curve` and Euler-chord rows now refuse `CarrierOffChart`; `m10_bulge` pins `pcurve_cylinder_chart_incident` 16/4 and `pcurve_line_chart_incident` 18/4.
  - The read is taken in the chart's own frame: the anchor at the origin and the axis unit. A read where the carrier sits misread an exact rim 1e7 m out as off at 1e-12 (a kernel defect), and a ruling high up an axis long by δ (inside the at-rest margin) as off at 1e-6. E1d's sphere, cone and torus reads had the same flaw.
  - Closed `plane-and-cylinder-mints-meter-incidence-as-a-residual`. The cylinder meridian arm's `pcurve_chart_radial_moving` tie-break, which E2 re-reads, now sits behind the read.
- 2026-10-28 — E1d2 (PR 5881) merged: topo's construction tier is sealed.
  - Its 46 doors are crate-private inherent methods of `Body`, public only through `topo::construction::Construct`, which is where they are documented.
  - `pncad::topo` is the topo glob, with the tier's modules shadowed. The shadow set is derived from `construction.rs`.
  - Pins that go red when a door is made public: the three `compile_fail` doctests; the façade row, which scans topo's root and every unshadowed public module for a `pub use` that carries a tier name or globs through a shadow; and `every_construction_door_is_a_crate_private_body_method`.
  - `DOORS_MEASURED` is 72.
  - The spec types stay nameable because `EdgeCurve::{mint, certify, …}` take and return them.
  - Review tier: single FULL review (the move is mechanical, but whether the seal is complete had to be probed from outside). 0 MAJOR; fix list K1–K5 (the pins could not see a door made public; the tier's docs had dropped out of rustdoc). The orchestrator read the fixes.
- 2026-10-28 — E1c (PR 5889) dispatched at review tier dual, class H (the incidence read across five charts). The concurrent pair reads frozen head 6c29a7e439, scope c591ab53c7..6c29a7e439.
- 2026-10-28 — **E1c fix pass L1–L10** (PR 5889).
  - Every at-rest reader of a stored carrier now reads incidence first: the stated chart image (`run_checks`), the stored pcurve rows (`PcurveCache::certify`/`recertify`) and the planar tier's boundary (through `IncidenceRead`). This closes `select/a-stated-chart-image-meters-its-carriers-incidence-as-a-residual`.
  - The pair description's two-surface residual is filed on select: `a-two-surface-residual-meters-a-moved-face-as-the-descriptions-miss`.
  - The read is decided less its rounding, as a multiple of the distance, so the symbolic tier's zero theorem survives the charge (the box replay of `m10_10`).
  - The envelope sweeps draw on the face and count what they drop.
  - The sphere parallel reads the radius its image turns at near the pole (`polar_read_radius`).
  - Filed closed: `a-lifted-copy-of-a-circle-whose-turn-is-within-the-band-certifies` (check 2 refuses it).
  - Three rows at the 1 s line move to the slow set; base and head measure alike.
