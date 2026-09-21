---
id: recourse-chain-stops-at-pcurve-certify-error
kind: issue
title: PcurveCertifyError's stopping arms are per-SITE, not per-arm: the recourse chain's last second-hop carrier, cut from the second-hop unit
status: open
opened: 2026-09-21
---


(FIX implementer, the second-hop unit) Cut by carrier from
`recourse-chain-stops-at-the-second-hop-carriers`, whose other four
carriers landed together. The method is that row's and the parent's:
per carrier, give the arms a repair **grounded in the module's or the
variant's own docs**, add `every_<carrier>_arm_names_a_recourse`, prove
it red by mutation, and assert a delegating arm TRANSITIVELY only where
the carrier below has an enforcement row of its own.

## The re-derived reading (at 161e610)

`PcurveCertifyError` (`crates/geom-brep/src/pcurve_cache.rs`) is
**fifteen variants**, as the parent row counted. Three delegate
soundly: `FittedEscalated` and `Escalated` carry
`COINCIDENCE_RECOURSE`, `Band` carries `BandError`, which has its own
row. `ChartRow` delegates to `SplineError`, which HAS a row as of this
unit — that arm is now transitively sound and wants only the assertion.
Four arms already name a repair and rewriting them would be a loss:
`FittedLaneUnsupported` (three scalars to replay at), `FittedMateMissing`
(supply the mate face's surface), `AzimuthPeriodExceeded` (split the
edge first), and — by a reading, not by the verb match —
`UnsupportedChart`, which routes the caller to the fitted lane.

**Seven arms stop at the condition**: `UnsupportedCarrier`,
`IsoUnsupported`, `FittedCertificate`, `IntervalNotForward`,
`ChartWindingUnsupported`, `ResidualExceeded`, `TrimEscape`.

## Why it is its own unit — the shape is not the others'

Two of the seven do not render prose of their own at all. Their whole
content is a `&'static str` payload minted at the refusal SITE:

- `IsoUnsupported { what }` renders `"the iso-line lane refuses this
  class — {what}"`, and `what` is one of **fourteen distinct literals**
  minted across `pcurve_cache.rs` (the iso lane) and `topo::pcurves`.
  Each names a different excluded class — an mvfs placeholder chart, a
  LINE seam on a rational column, an iso line leaving the chart domain,
  a seam carrier whose spline structure is not the traversed column's,
  and so on. A repair belongs at each site; a single clause at the arm
  would be exactly the blanket tail PR 2354 removed, and would read as
  a complete chain while proving nothing about any of the fourteen.
- `FittedCertificate { limb, what, magnitude }` is the same shape over
  the SSI certificate's own refusals, flattened to three parts at
  translation time.

So this carrier's unit is a per-SITE pass over two other modules'
refusal vocabularies, plus five ordinary arms — a different job from
the four that landed, and one that wants a real read of the SSI and
iso-lane domains before a repair is written. Writing one without that
read is how a wrong repair ships in confident prose.

## Fence

`crates/geom-brep/src/pcurve_cache.rs` is **CHART's**; the iso `what`
literals are also minted in `crates/topo/src/pcurves.rs` (**TRIM's**).

## What pins these messages today

Nothing discriminates any of the fifteen renderings; the enforcement
row this carrier is owed is also the first pin it will have had. The
missing pin is part of the defect, not a baseline to preserve.

## When this lands

`every_pcurve_mint_error_arm_names_a_recourse`
(`crates/topo/src/pcurves.rs`) asserts its `Certify` arm as a
DELEGATION only, which is honest exactly while this row is open. It
becomes transitive — and the chain from `ValidationError::Pcurve` is
proved end to end — in the PR that closes this.

## Re-homed to PCERT, 2026-09-21

(FIX orchestrator) Filed on FIX's slate by the lane that cut it, which
read `crates/geom-brep/src/pcurve_cache.rs` as CHART's at its own merge
base. **`scripts/work.py territory` says `pcert`** — this program opened
while that lane was running. Routing from the instrument rather than
from a report, which is the correction this dispatching seat has now
made eight times.

It leaves FIX because FIX takes only rows whose fix is already written,
and this row says plainly that its own is not: the repairs belong at
fourteen distinct `&'static str` minting sites across two crates, and
writing them needs a real read of the SSI and iso-lane domains —
*"writing one without that read is how a wrong repair ships in confident
prose."*

It lands on PCERT because this is the certifier's own refusal
vocabulary, and your slate already carries
`pcurve-fit-refusal-drops-the-domain-doors-reason` — the same class on
the same family.

**One thing became true while this row was being written**, and it
shortens the job: `SplineError` now carries
`every_spline_error_arm_names_a_recourse`, so `PcurveCertifyError::ChartRow`
is transitively sound and wants only the assertion, not a repair.

The second fence is `crates/topo/src/pcurves.rs` (**TRIM's**), where the
iso lane's `what` literals are also minted.
