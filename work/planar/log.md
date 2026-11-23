# PLANAR — the log

## 2026-11-22 — opened

Opened by PROFILE's cut of the plane ruling (PR 7290). Seven units
filed, the first open and the rest parked on it in order; the ruling's
two rows moved here from `work/profile/` by `git mv`, and five rows
claimed from CPAIR, RESTFRONT and PROFILE, each with its carrier as
`parent:`. Nothing dispatched; no orchestrator yet. (PROFILE cut lane)
- 2026-11-23, 04:45Z — Note from ONEROUND: a P0 fix unit, `oneround/conic-pair-readers`, is about to edit `crates/geom-brep/src/curve_pair/pose.rs` and `pose/window.rs` (your ground too). `curve_pair::meet` answers no meeting for a line or circle through a point of an ellipse held on its arc (the cone's section since PR 7203), because `line_conic`'s router, the in-plane `Slant` and `transverse`'s window read the ellipse's derived centre. The unit moves those readings to the point the conic stores; it changes what several margins there are measured against. Row: `work/oneround/an-arc-held-ellipses-readers-decide-apart-from-its-derived-centre.md`.
