---
id: a-widened-shared-vertex-escalates-the-pair-pass
kind: issue
title: A widened profile vertex escalates the pair pass on its own adjacency
status: open
opened: 2026-10-18
priority: P2
cost: M
---

The profile validator's pair pass decides, for two adjacent segments, the side of their shared endpoint (`crates/profile/src/seg.rs:1129-1152`, `line_line` → `chord_side`). The margin `perp_dot(û, q − a)` is exactly zero at f64, since `q` is an endpoint of the other segment. At `Interval`, when that vertex reads a widened parameter, the arithmetic loses the dependency between `q` and the segment through it. The enclosure is then about ±2.15× the box's half-width around zero, and the pass escalates as soon as that exceeds the 1e-9 zero band. Measured at boxes of ±1e-9, ±1e-6, ±0.01 and ±0.25: every one escalates, on a square and on a triangle alike.

Before INTENT stage 3 D this was not reached: a widened parameter moved a profile's datum frame, and the profile's own coordinates stayed exact. In D (D10) a profile is 2-D and reads its parameters itself, so any profile vertex a toleranced variable drives fails lane validation at `Interval`. `eval9_nominal_in_the_key::over_a_param_box` scales its box by 2⁻³¹ to stay under the band. The profile crate is unchanged from main.

**Done when** the pair pass takes a shared endpoint's side by adjacency (`Sign::Zero` by construction, not decided), or reads the margin with the dependency kept. A square whose corner a ±0.01 box widens then validates at `Interval`, and eval9's scaling comes out.

## Re-homed at PATHS's close (2026-10-21)

Moved from `work/paths/` to PROFILE (`work/profile/`) when PATHS closed. PATHS's live rows went to four programs opened at this close and cut along the priority seam; PROFILE holds the goal (`docs/doc-ledger/paths-leaves-the-tracker.md`). The item's id, content and history are unchanged.
