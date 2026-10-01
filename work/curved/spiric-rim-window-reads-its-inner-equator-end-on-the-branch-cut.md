---
id: spiric-rim-window-reads-its-inner-equator-end-on-the-branch-cut
kind: issue
title: The spiric rim's window reads its inner-equator end on atan2's branch cut, so half the klein elbow's rims run backwards by pi
status: closed
closed: 2026-10-01
pr: 3626
branch: curved/equator-seam
opened: 2026-09-30
refs: [equator-seam-reauthor-refuses-the-hollowed-elbow, spiric-carrier-ruling]
priority: P0
cost: M
---

## What

Measured on `curved/equator-seam` once the elbow's equator seams
re-author: the klein elbow's hollow now passes every decision of
`offset_charts_together` and refuses at the attach layer, on a RIM:

`ShellError::Face { face: FaceKey(1v1), error: Op { edge: None,
error: RechartFalsifies { edge: EdgeKey(2v1), error:
IntervalNotForward { verdict: Negative { margin: -0.7068583470577036
} } } } }` — the
margin is `−π·r′` (`r′ = 0.225`) to the last digit; the partial
two-arc torus (`shell7_seam_corner`, `r′ = 0.45`) reads
`−1.413716694115407` on `EdgeKey(1v1)`, again `−π·r′`.

**Cause, measured** (a probe in `offset_axial.rs`'s `param_on`, the
`(Spiric, Circle)` arm): each rim is the half meridian circle between
the two equators, so one endpoint is the INNER equator, where the
anchor `v_q = atan2(h, ρ − R)` has `h` exactly `±0.0` and
`ρ − R < 0`. `atan2(±0, −x)` is `±π`, so the sign of a zero picks the
branch. Elbow: `EdgeKey(1v1)` reads `t: 0 → π` (forward),
`EdgeKey(2v1)` reads `t: π → 0` (backward), `5v1` forward, `6v1`
backward; the two-arc torus reads `h = −0.0` on `1v1`/`2v1`, so `1v1`
runs `0 → −π`. The window is read endpoint by endpoint with no
orientation between them — the circle arm carries the old window's
turn (`t_old + atan2(…)`), the spiric arm does not (by the spec's
§4: "the old window's turn is not carried across a kind change, its
SENSE is"). The sense predicate decides the carrier's direction; nothing
decides the window's.

## Fix shape (candidate)

Read the rim's end parameter FORWARD of its start in the carrier's
sense (the mint's `offset_axial_rim_sense` already made the spiric
run the old circle's way), rather than each end within a half turn of
an anchor that sits on the branch cut — a decided turn, not a
`±0.0`. Then the elbow rows (`torax_axial`, `verbs_shell` open and
sealed, `shell7_seam_corner`, `spiric_rim`, `torax_interval`) flip
again, predicted to check 7's props door, where the sectioned vessel
already stands.

## Home

CURVED — the spiric lane (`offset_axial.rs:param_on`).

## Fixed on `curved/equator-seam` (PR 3626)

`offset_axial.rs:forward_window` decides the read span `t₁ − t₀`
levered at the moved tube's radius (`offset_axial_rim_window`):
- `Positive` keeps the read end;
- `Negative` takes the same end a period on;
- `Zero` refuses typed.

The sealed elbow, the two-arc torus and the interval row now reach
check 7's props door. The opened elbow stops at the lift
(`work/shell/shell-open-lift-takes-the-per-chart-door-on-the-klein-elbow.md`).

## Closed (2026-10-01)

Merged in PR 3626 (single full Opus review, MERGEABLE; outside the suspended A/B protocol, no row).
