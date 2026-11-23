---
id: cover-kind-match-cannot-tell-a-size-read-from-a-region-read
kind: issue
title: cover-kind-match counts a conic's semi_axes() read as a region read, and its no-raise-above-base rule has no way to re-baseline a field read that became an accessor
status: open
opened: 2026-11-20
priority: P4
---


Filed by GERM's `germ/ellipse-speed-magnitudes` lane (PR 7183).

`scripts/gates/cover_kind_match.py` counts an identifier in
`REGION_IDENTS`, which includes `semi_axes`, inside a `Curve3` or
`Surface` kind arm as a region read. It counts the identifier, not
what is done with it. PR 7183 made `geom::Ellipse`'s sizes private. A
field read `e.semi_u` was invisible to the gate. The same read through
`e.semi_axes()` is counted.

Fourteen such reads appeared in eleven files (`scalar_lift.rs`,
`curves.rs` `same_content`, `signed_zero.rs`, `step-export` `writer.rs`,
`loop_area.rs`, `injective.rs`, `replace_face.rs`, `stands.rs`,
`conic_oracle/exact.rs`, `props/curved.rs`, `pcurve_cache.rs`). Each
copies or prints a size and builds no box, support or reach. Each pin
rose by the number of its file's reads.

The gate's `RAISED` rule (a pin above the merge base's reds, with
`GATE_BASE` set in CI) gives no way to record that re-baseline. The
pins could not move. The PR read those sites through per-axis
accessors `Ellipse::semi_u()`/`semi_v()` instead. That keeps the
gate's sight where it was on main, where these were uncounted field
reads.

Two limits:

1. **The lexical rule cannot tell a size read from a region read.** A
   reader that builds a box or support from the semi-axes is a region
   read and should be counted. A reader that copies or prints them is
   not.
2. **The ratchet has no way to acknowledge a re-baseline** when a field
   becomes an accessor the vocabulary names, for example a cited
   reason on the pin line.

- 2026-11-22 — **Re-banded unbanded → P4 by CIW triage.** Gate friction, so its payoff is CI reds and workarounds avoided, which is P4 by `work/README.md:226-228`.
One incident: PR 7183 (2026-11-20) added `semi_u()`/`semi_v()` "only to keep cover-kind-match's lexical count".
GERM's review called that API shaped to a gate. GERM kept the accessors (`work/germ/log.md`, 7183 merge entry), so no pending change will hit this again.
The band-log 7078 entry is the gate being met by re-spelling, not friction.
