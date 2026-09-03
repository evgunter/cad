---
id: props-refusal-cannot-carry-measured-overshoot
kind: issue
title: "props/curved: a typed refusal cannot carry its measured overshoot in metres — the Bounds compound bound is off bounds-allowlist.sh"
status: open
opened: 2026-09-02
github: 1602
refs: [1599, 1571, S19]
---

## From GitHub issue 1602

Opened 2026-09-02; 0 comments.

**Filed from MESH-11 (PR [#1599](https://github.com/evgunter/cad/pull/1599)) as the schedule for a disclosed deviation; both reviews asked for one.**

MESH-11's `require_one_chart_branch` refuses with `PropsError::NotOneChartBranch { edge, what }` and no `f64` overshoot, though the spec asked for the measured overshoot in metres in the payload (the number that separates "re-author your part" from "kernel bug", per S19's postmortem lesson). Reason, verified by R1 by planting the bound: reading a definite margin back as `f64` from a `Decide`-generic lane needs a `T: Bounds` compound bound, and `scripts/gates/bounds-allowlist.sh` (~:297–305) exempts only `topo/{boolean,separation,props,chart_region,validate,shell}.rs` and `geom-brep/{pcurve_cache,ssi,ssi/certify,edge_nurbs}.rs`; `props/curved.rs` is not on it, and no `PropsError` arm in tree carries a measured `f64` today. The overshoot IS measured and recorded through the funnel; the floor rows pin the threshold from both sides against the run's own band.

**Owed (a ratification, not a unit's drive-by):** decide whether `props/curved.rs`'s refusals may carry a measured `f64` (a ratified seam on the allowlist, with the reason the allowlist exists stated against it), or whether the K-stream record is the payload of record and the refusal stays name-only. Then either add the seam and the overshoot to `NotOneChartBranch` (and to `NotIsoRectangle`, which has the same `&'static str` shape), or write the second answer at both arms.

Refs MESH-11, #1571, the `bounds-allowlist` gate's rationale, S19.

## Home

`work/cert/` — the refusal arms are in `crates/geom-brep/src/props/curved.rs`, an S-CERT territory glob, and S-MESH names 1602 a cross-program follow-on on another program's ground.
