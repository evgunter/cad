---
id: recourse-chain-stops-at-the-second-hop-carriers
kind: issue
title: five second-hop carriers stop at the condition, so the recourse chain is unproved one delegation past the ValidationError arms
status: open
opened: 2026-09-12
---



(FIX orchestrator) Disclosed by the second PR of
`validation-arms-delegate-a-recourse-their-carriers-do-not-give`, which
took `PcurveMintError` and `OffsetFitError` and could not honestly
close the chain behind them.

## The class, stated as the first PR's lane named it

The census sweep that opened the parent row matched **verbs in string
literals**, and a verb match cannot see past the first hop. The real
class is **every carrier reachable from a delegating arm**: an arm
whose `Display` renders a carried error whole contributes no recourse
of its own, so the claim *this message names a repair* is a claim about
the carrier, and about the carrier's carrier, all the way down.

The parent row's two PRs fixed six enums at the first and second hop
(`BandError`, `MassPropsError`, `PropsError`, `PcurveMintError`,
`OffsetFitError`, and `geom_core::Indeterminate` was already sound).
**Five carriers at the next hop are not fixed**, and each was read
rather than verb-matched:

| carrier | file | arms stopping at the condition |
| --- | --- | --- |
| `PcurveCertifyError` | `crates/geom-brep/src/pcurve_cache.rs` | 6 of 15 |
| `MeterError` | `crates/geom-brep/src/offset_meters.rs` | 2 of 3 |
| `PatchBoundError` | `crates/geom-brep/src/patch_bound.rs` | 5 of 7 |
| `FitError` | `crates/geom/src/curves/fit.rs` | 7 of 10 |
| `SplineError` | `crates/geom-core/src/spline/knots.rs` | 5 of 5 |

The counts are of RENDERINGS read by eye, not of verb hits, and they
are as of this file's merge base — re-derive before taking the row.

**Several arms are already right and are the reason to read rather than
sweep.** `PcurveCertifyError::FittedLaneUnsupported` names three
scalars to replay at; `FittedMateMissing` says to supply the mate
face's surface; `AzimuthPeriodExceeded` says to split the edge first;
`FittedEscalated` carries `COINCIDENCE_RECOURSE`.
`PatchBoundError::Degree1Crease` and `Crease` both say to split the
face at the crease. Rewriting any of those would be a loss.

## Why the two enforcement rows do not cover it

`every_pcurve_mint_error_arm_names_a_recourse` and
`every_offset_fit_error_arm_names_a_recourse` assert a delegating arm
TRANSITIVELY only where the carrier below has an enforcement row of its
own — which is true of `BandError` and of `Indeterminate`'s
`COINCIDENCE_RECOURSE` tail, and of no carrier in the table above. For
those five the rows assert only the DELEGATION: that the arm renders
the carrier whole rather than summarising it. That is deliberate and
stated at both rows: a row that asserted a recourse by picking the one
payload whose carrier happens to name one would read as a complete
chain and prove almost nothing.

So the shape of the fix is the same one the parent row established —
per carrier, give the arms their clause and add
`every_<carrier>_arm_names_a_recourse`, proved red by mutation — and
the last step is to **turn the five delegation assertions into
transitive ones**, at which point the chain from
`ValidationError::Pcurve` and `ValidationError::ApproxCertification`
is proved end to end and a defect three crates away reddens the row.

## Fences

Four owners, none of them FIX by glob: `pcurve_cache.rs`,
`offset_meters.rs` and `patch_bound.rs` sit beside `offset_fit.rs` in
**PROPS's** ground; `crates/geom/src/curves/fit.rs` is **PROPS's** too;
`crates/geom-core/src/spline/knots.rs` is `geom-core`, the same ground
`BandError` was taken on by announcement. Whether this is one unit or
one per carrier is the taker's call — `SplineError` is five short arms
and the cheapest place to start; `PcurveCertifyError` is the one that
wants a domain read.

## What pins these messages today

**Nothing does, for any of the five.** The same finding the parent row
recorded at three carriers: no test discriminates any of these
renderings, so the enforcement row each carrier is owed is also the
first pin it has ever had. The missing pin is part of the defect, not a
baseline to preserve.
