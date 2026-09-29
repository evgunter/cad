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
