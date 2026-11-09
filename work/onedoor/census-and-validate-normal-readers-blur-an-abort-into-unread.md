---
id: census-and-validate-normal-readers-blur-an-abort-into-unread
kind: issue
title: census's five and validate's corner_normal face-normal readers fold a normal's abort (OffSurface) into Unreadable or no normal
status: open
opened: 2026-11-09
priority: P3
cost: E
---


Found by ONECOPY's `onecopy/normal-refusals` sweep (face-normal reads
whose refusal arm is a catch-all), the same shape it fixed in
`boolean::reduce::sheets_part` and `boolean::slit::spline_floor`.

## What

Each reads `crate::face_normal::face_outward_normal_at` (or `_at_foot`)
and folds every refusal into one arm, so `NormalAtError::OffSurface` (an
abort: the caller's point off the face's surface,
`NormalAtError::refusal` → `Aborts`) reads the same as a kept refusal
(a cone's apex, a degenerate torus):

- `topo` `census.rs`, five readers: the `Ok(None) | Err(_) => return
  unbuilt(errors)` arm (about line 2024), `wedge` (`Ok(None) | Err(_) =>
  WedgeRefusal::Unread(.., TouchUnread::Unreadable)`, about 2606), the
  floor reader (`=> Some(undecided(TouchUnread::Unreadable))`, about
  4476), `foot_normal` (`Err(_) => TouchUnread::Unreadable`, about 8792),
  `normal_at` (`=> InteriorsUnread::Unbuilt`, about 11808). These
  refuse, loudly, but as a limit.
- `topo` `validate.rs` `corner_normal` (check 9's pinch corners): `Ok(None)
  | Err(_) => Ok(None)`, a stated residue, so an abort is read as no
  corner normal and the check passes the corner unread.

## Owed

Class each refusal through `NormalAtError::refusal()`, and name the
abort's answer (a defect, not an unread), as `slit`'s readers now do.
