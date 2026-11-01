---
id: reporting-margin-gate-misses-comparisons-of-two-read-readings
kind: issue
title: The reporting-margin gate counts MarginDiag mint sites, so a production comparison of two classifier-minted readings (Indeterminate == Indeterminate) decides on the reporting number unseen
status: open
opened: 2026-10-18
priority: P2
cost: E
---

## What

`scripts/gates/reporting-margin-door.sh` covers equality with this
premise: *"EQUALITY IS THE OTHER ROUTE, and it is counted too … A
comparand has to be minted with `MarginDiag::value` or
`MarginDiag::enclosure`, so the production MINT sites are a second
list"*. That premise does not hold. A production site can compare two
readings that the classifier minted itself, with no mint of its own,
and so decide on the reporting number unseen.

The instance: RESTREAD's PR 5261 first head, `crates/topo/src/census.rs`
`pair_edge_face`, read
`let alike = matches!(&read, [Err(a), Err(b)] if a == b)` to decide
that two undecided residuals were one value. Equal `Indeterminate`s are
equal reporting margins (Ev's PR 3402 ruling, quoted in the gate).
Two `Invalid` readings compare equal whatever their values. At
`Interval`, two equal enclosures need not hold one true value. Review
caught it; the gate passed it. The site is gone in the fix pass, which
decides the residuals' difference through `decide` instead.

## Sweep obligation

Grep production code for `==` / `!=` / `matches!(… if a == b)` /
`assert_eq!` outside `#[cfg(test)]` whose operands are
`Indeterminate`, `MarginDiag`, `Decided` or a value carrying one
(`ValidationError::CensusEscalated`, `CarrierRefusal`, `MeetFault`, the
error enums that derive `PartialEq` over a reading). Give each hit a
disposition. Then make the gate count those comparisons, not only the
mint sites. A type-level route would be to drop `PartialEq` from the
reading and give tests a separate pinning comparator. State what the
grep cannot match: a comparison through a generic `T: PartialEq` bound,
or a derive on a wrapper.

## Moved to BLIND (2026-10-26, GUARD's split)

GUARD measured 43 points against 30 and split three ways. This row is a
spelling a kernel-discipline gate's matcher misses (or refuses wrongly),
which is BLIND's charter (`work/blind/plan.md`). Id and body unchanged.
