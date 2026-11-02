---
id: the-clearance-tool-copies-the-seat-machinery
kind: issue
title: The clearance tool re-implements Seats' pick rule and survival and the mirror tool's face survival and event
status: open
opened: 2026-11-02
priority: P3
cost: M
---

Found by the style review of PR 6357 (finding 13).

`viewer::combine::ClearanceTool` holds its two seats outside `Seats`
but declares their roles in its vocabulary (`Seat::ClearedFirst/Second`,
`FaceRole::ClearedFirst/Second`), and re-implements:

- the pick rule (`next`, "the seats' pick rule");
- the body survival step (`reconcile`, cf. `Seats::reconcile`) and
  respeak;
- the mirror tool's face survival (`reconcile_faces`, a two-slot
  `MirrorTool::reconcile_plane`) and its event (`ClearanceEvent::FaceLost`
  is `MirrorEvent::PlaneLost` plus a role, `Say` and `Display` copied).

Why the body seat is not a `Seats` is argued on `ClearancePick` (a
construction copied several times names no one copy, so the tool
refuses the pick; `Seats` would seat the construction). That reason
holds for the body seat's pick, not for its survival or the face seat's.

## Repair shape

A seat value that can hold a body or a face, with one pick rule and one
survival step, shared by the clearance and mirror tools; `Seats` keeps
its copy-to-body rule as a policy the clearance tool opts out of.
