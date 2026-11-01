---
id: sweep-places-vanishing-tangent-is-a-bare-f64-compare
kind: issue
title: sweep_places refuses a vanishing path tangent by a bare n > 0.0 in unit_tangent, while every other direction-length question is decided under the band
status: closed
closed: 2026-10-26
opened: 2026-10-06
priority: P3
cost: M
design: true
---


Found by `carve/one-door-for-coincident-sections`'s sweep (2026-10-06).

`crates/sweep/src/skin.rs` `unit_tangent` refuses a station's path
derivative as `SkinError::PathTangentReversal` when `!(n > 0.0)`: an
exactly-zero (or poisoned) derivative. A derivative of any nonzero
length is normalised and used, so a station at or beside a cusp of the
path builds its frame from whatever direction the float residue
points. Every other "does this direction have any length" question in
the kernel is decided under the band (`geom_core`'s
`DIRECTION_LENGTH_SUBJECT`, `UnitVec3Error::Escalated`), and the
sweep's own start frame comes from `path_start_frame`, which decides.

Not measured: no fixture yet puts a station at a near-cusp. The
derivative is length per unit parameter, not a length, so the band
does not apply to `n` as it stands; the row is to decide what the
banded question is (the direction's stability across the station, or
the chord to the neighbouring station) and whether a near-cusp station
is reachable through `sweep_body` at all. The exact-anti-parallel arm
beside it is `a-half-turn-spine-sweeps-only-off-its-exact-tangents`.

## Closed (2026-10-26, tracker sweep)

Fixed on main. `unit_tangent` and its `!(n > 0.0) || !n.is_finite()`
test are gone: `47d913107f` (the PATHS sketch-plane-is-its-frame
branch, merged) deleted them and decides each station's path
derivative under the band through `UnitVec3::new(d, SWEEP_PATH_TANGENT,
band)` (`crates/sweep/src/skin.rs:1730`, the name at `:1654`). An
escalation refuses `SkinError::PathTangentUndecided`, and a degenerate,
non-finite or underflowed length refuses `PathTangentVanishes`
(`skin.rs:1730-1741`). The row's design question (what the banded
question is) was answered by that change. It reads the derivative's
direction length as `UnitVec3`'s ladder does everywhere else, and the
opposed arm beside it now goes through `RigidMotion::turned_between`
(`:1748`).
