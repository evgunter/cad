---
id: the-meeting-axes-pose-escalates-the-joins-endpoint-end-at-1e-12
kind: issue
title: verbs_germarms2_interval's meeting-axes pose refuses at 1e-12 on main: the join's certification escalates EndpointEnd in band
status: open
opened: 2026-10-21
priority: P2
cost: M
---


Found by the review of PR 5479. It is red on clean `origin/main`
(`7615abd608`) and independent of that PR. `work/flux/log.md` noted it
as a main red, but no row carried it.

## What

`crates/sweep/tests/verbs_germarms2_interval.rs`,
`the_meeting_axes_pose_builds_at_the_certified_scalar`, at
`CAD_TOLERANCE_EPS=1e-12`:

    the meeting-axes pose builds: Err(Join(BooleanJoin(Euler(Certification {
      error: Escalated { check: EndpointEnd, sample: 8, cause: Indeterminate {
        margin: MarginDiag(Enclosure { lo: 0.0, hi: 1.19288120443679e-12, .. }),
        band: Band { zero: 1e-12, escalate: 1e-11 },
        predicate: Some("carrier_endpoint_end") } } }))))

The union's join certifies a minted edge's end against its vertex
(`carrier_endpoint_end`), and the enclosure `[0, 1.19e-12]` straddles
the zero band, so it escalates. At the certified scalar the end's own
width may be the rounding of the meeting point; it looks like the family of the
closed `a-section-curves-end-read-in-band-of-its-vertex-refuses-as-a-kernel-defect-at-the-join`,
here at interval.

## Fix

Read the end at the vertex the join meets, as that row did for f64, or
type the in-band reading with the parts' recourse. Then pin the pose
building at 1e-12.

## Slate triage (2026-10-26, CLEAVE)

- Moved to HONEPOSE (`work/honepose/`) by `git mv`, id and body unchanged: the P2 track opened for in-band refusals at measured poses: a reading that lands in band where the pose is not near a decision. CLEAVE measured 270.5 budget points against 30 and split on its priority seam (Ev, in chat, 2026-10-26).
