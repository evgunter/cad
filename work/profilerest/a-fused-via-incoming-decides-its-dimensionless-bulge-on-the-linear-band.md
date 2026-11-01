---
id: a-fused-via-incoming-decides-its-dimensionless-bulge-on-the-linear-band
kind: issue
title: paths: a fused Via incoming decides its computed bulge against zero on the linear band (bulge_carrier's path_arc_bulge), so a sound shallow Via whose plain leg builds refuses as a fused incoming
status: open
opened: 2026-10-20
priority: P2
cost: M
---


Found by the re-review of PR #5449 (pre-existing, off that PR's delta).

**The gap.** `bulge_carrier` (`crates/profile/src/path/family.rs`)
decides `path_arc_bulge` with `Margin::of(b)`: the bulge is the
quarter-tangent tan(θ/4), dimensionless, classified against the linear
band (ε = 1e-9 m) as if it were a length. A fused `Via` incoming
computes its bulge (`bulge_from_via`) and passes it through that gate.
The door has already decided the authored through-point's offset from
the chord (`path_arc_via_offset`, metres) definitely off it, so the arc
exists. But a shallow arc's bulge, about 2·offset/chord, can still read
Zero or in band.

**Measured** (default ε). Authored points: a 100 m chord (0,0)→(100,0),
`q` at (50, off), home (50, −30).

| off | sharp `Via` leg | fused `ArcFillet { Via, r = 1 }` |
|---|---|---|
| 2e-8 m | builds | refuses `DegenerateArcSpec`, "this arc spec's authored datum (-4e-10) names no arc" |
| 1e-7 m | builds | escalates on `path_arc_bulge` (margin −2e-9) |

The reviewer drew 199 980 random decided offsets: 20 069 read the
bulge Zero and 5 105 read it in band, about 13% between them. The
refusal is loud, not silent: P2.

**Shape.** The fused `Via` incoming takes its winding from the decided
offset already (`PointIncoming for Via`, since #5449). It needs no
bulge gate of its own, or one on a length (the sagitta, as
`segment_straightness` reads it) rather than on tan(θ/4). A fused
`Bulge` incoming, whose bulge is authored, keeps its gate, but that
gate should also be levered by the chord to be a length.

## Re-homed at PATHS's close (2026-10-21)

Moved from `work/paths/` to PROFILEREST (`work/profilerest/`) when PATHS closed. PATHS's live rows went to four programs opened at this close and cut along the priority seam (PROFILE, PROFILEREST, PROFILETAIL, PROFILEPROSE; `docs/doc-ledger/paths-leaves-the-tracker.md`). The item's id, content and history are unchanged.
