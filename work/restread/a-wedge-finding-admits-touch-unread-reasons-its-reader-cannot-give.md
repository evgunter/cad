---
id: a-wedge-finding-admits-touch-unread-reasons-its-reader-cannot-give
kind: issue
title: FaceThroughEdgeWedgeUndecided admits every TouchUnread reason, and its samples render reasons its reader may not give
status: open
opened: 2026-10-25
priority: P4
cost: E
refs: [census-passes-a-face-inside-an-edges-material-wedge]
---


Found by review 1 of PR 5529 (branch `restread/coincident-wedges`);
filed in its fix pass, which fixed the same shape on its own finding.

## What

`ValidationError::FaceThroughEdgeWedgeUndecided.why` is a
`census::TouchUnread`, which has seven reasons, and
`test_support_samples.rs` samples the finding once per reason
(`TouchUnread::iter()`). Its one producer, `census.rs`
`edge_in_own_face`, takes the reason from `Link::on_edge`,
`Link::in_face` and `rest::judge`, and from `read_undecided`'s branch
carry (`InBand` only). Which of the seven those can give has not been
read: `Corrupt` is a kernel defect on a body tier 1 admits, and
`ChordScale` is raised by the link builders' chord scaling, which an
edge's own link may never reach. A sample of a reason no run can give
renders a sentence no user sees, and pins nothing.

PR 5529's `CoincidentWedgesOverlapUndecided` had the same shape. Its
fix samples only what its reader names: `InBand`, and
`census::WEDGE_UNREAD`, the reasons `WedgeRefusal::Unread` carries.

## Owed

Read which reasons `edge_in_own_face` can return, name them beside it
as `WEDGE_UNREAD` is named, and sample only those; or a row per
reason that no run reaches, saying why it stays.
