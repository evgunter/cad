---
id: cone-trim-reads-an-in-band-image-walk-as-a-corrupt-face
kind: issue
title: cone_trimmed_window reads the image walk's in-band escalation on a face clear of the apex as CorruptFace
status: closed
opened: 2026-10-10
closed: 2026-10-12
priority: P3
cost: E
parent: periodic-branch-floor-remains-at-three-sites
---


Found by CLEAVE's `point-in-solid-mints-outside-its-period-and-nappe-rows`
in its second sweep pass (a decided or in-band reading discarded without a
mint). Read, not measured.

## What

`crates/topo/src/boolean/solid_contain.rs` `cone_trimmed_window`, on
`ApexClosure::Clear`, reads the outer loop's images through
`crate::chord_join::face_azimuth_images(..).ok().flatten()` and answers
`PointInSolidError::CorruptFace` for any refusal. `face_azimuth_images`
documents an escalation among its refusals (and a run edge with no
closed-form image, a fitted image, a vertex off the carrier), so an in-band
reading on a healthy cone face would be told as "one of its faces is
broken". The `Closed`/`Open` arms of the same match keep the escalation
(`Err(SplitJoinError::Escalated { diag, .. }) => esc(diag)`), and the
cylinder's `wall_outline` maps the walk's other refusals to the class
statement (`unsupported()`), not to corruption.

## What would close it

Measure whether `cone_apex_closure` answering `Clear` already ran the walk
that would escalate (then the second walk cannot, and the `.ok()` is a
second read to remove); otherwise match the walk's refusals as the
`Closed` arm and `wall_outline` do.

## Closed

Claimed from INSIDE and closed by `pcert/periodic-branch-structural`,
whose structural branch pin made the walk's escalation reachable here
(`split_window_branch`). `cone_trimmed_window`'s `ApexClosure::Clear`
arm now matches the walk's refusals as the `Closed` arm does:
- an escalation stays `Escalated`;
- `Corrupt` / `UnpairedLooseEnds` / a non-cycle stay `CorruptFace`;
- any other refusal (a walk this face cannot take) is
  `PartialConeFace`.

The cylinder sibling, `cylinder_chart_trim`, carries the escalation typed
too; its other refusals still read `CorruptFace` for want of a class
variant, filed as
`work/inside/cylinder-chart-trim-reads-a-walk-it-cannot-take-as-a-corrupt-face.md`.

