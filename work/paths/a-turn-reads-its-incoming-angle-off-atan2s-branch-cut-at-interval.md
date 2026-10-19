---
id: a-turn-reads-its-incoming-angle-off-atan2s-branch-cut-at-interval
kind: issue
title: A .turn(δ) after a chord along -x reads its incoming angle off atan2's branch cut at Interval, so the turned ray encloses the whole circle
status: open
opened: 2026-10-19
---


Found by the chord-director fix
(`the-declared-tangency-bracket-refuses-its-closing-junction-at-interval`),
whose sweep of `atan2` reads in `profile` left these as not that unit's.

A chord's director is now its components (`path::chord_director`), so
its RAY is narrow at `Interval` wherever it heads. Its ANGLE is not:
`Dir::from_unit` (`crates/profile/src/path.rs`) derives `ang` as
`u.y.atan2(u.x)`, and a ray along −x whose `y` straddles zero sits on
`atan2`'s branch cut, where inari answers `[−π, π]`. Two production sites
build a direction from that angle:

- `.turn(δ)` (`path.rs`, the turn kernel: `Dir::from_angle(inc.ang.ang + delta)`).
  Measured at `Interval`: `Open.at((1, 0.1)).angle(π/2).line(2.9)
  .line_to((0, 3)).turn(1.0)` refuses `Escalated` on `path_junction_turn`
  with a margin `[−1.0000000000000038, 1.0000000000000038]` — the turned
  ray spans the circle and the turn check reads the whole lever.
- The `Sweep` arc extension's end tangent
  (`crates/profile/src/path/verbs.rs`, `Dir::from_angle(dp.dir.ang + signed)`),
  the same shape by reading; a `.tangent().arc_to(Sweep { .. })` probe
  off the same chord did NOT reproduce, so it is unconfirmed.

The repair the chord took — turn the RAY (`path::turned(u, δ)`), never
the angle — fits both, but `.turn` is the VQ4 contract's named reader of
the angle ("derives θ by atan2 only for the angle arithmetic (`.turn(δ)`,
arc end tangents) that genuinely needs a number", `Dir`'s doc), and an
angle-authored chain's turned ray moves its last bits under it, so it is
a contract question as well as a fix.

The same cut hazard sits, by reading only, in the angle-about-a-centre
reads that difference two angles into a swept length:
`sugar::bulge_from_center` (`phi_a`, `phi_b`), the fillet leg shapes'
`corner_angle` / `far_angle` and `angle` (`crates/profile/src/sugar.rs`),
and `path::arc_fillet::about`. A point due −x of its centre with a
computed `y` reads the whole circle there; none was probed.
