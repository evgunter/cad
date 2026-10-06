---
id: shell-clearance-gate-skips-planar-pairs-tilted-off-antiparallel
kind: issue
title: wall_clearance reads only antiparallel planar pairs, so two planar walls meeting at an angle across less than 2t shell silently
status: open
opened: 2026-10-06
priority: P1
cost: M
---


Found by the `shell/planar-gate-misses` lane while levering
`shell_walls_antiparallel`, and **measured**.

`wall_clearance` (`crates/topo/src/shell.rs`) gates a planar pair only
when its outward normals are antiparallel to the band across the pair's
extent; a pair whose drift is definite (`shell_walls_antiparallel`
Positive) is skipped. Two non-adjacent planar walls that face each
other at an angle, closer than `2t` over part of their overlap, are
therefore never read, and their offsets cross. The module docs name the
window ("a planar pair that is not antiparallel"); nothing schedules it.

Witness (run on the lane's head; the probe was not kept): the prism of
`(0,0),(3,0),(3,1),(2.5,1),(2.8,0.5),(0,0.5)` extruded 1
(`sweep::test_support::{prism, corners}`), shelled sealed at `t = 0.15`.
The arm between the right wall `x = 3` and the tilted notch wall
`(2.5,1)–(2.8,0.5)` is 0.5 wide at its top and 0.2 at its foot, so the
two walls' offsets cross below `y ≈ 0.75`. `topo::shell` returns `Ok`;
`mass_properties` reads volume `1.3197249948958765`, a cavity of
`0.3553 = 0.7 × 0.5075`, and `0.5075` is exactly the signed shoelace
area of the self-crossing mitre polygon
`(0.15,0.15),(2.85,0.15),(2.85,0.85),(2.7649,0.85),(3.0649,0.35),(0.15,0.35)`
— the crossed lobe counted negative. The true erosion contains the
rectangle `[0.15,2.85]×[0.15,0.35]` (area 0.54) alone, so the cavity is
under-counted and the result is silently wrong.

Before the lever the facing test read `−(n̂_a·n̂_b) − 1` against the
band, which admitted tilts up to about `√(2ε)` and then measured them
with the parallel-plane gap; this pair (tilt ≈ 0.54 rad) was outside
that window too, so the miss is not new.

A closing gate reads, for a facing non-antiparallel pair, the least
distance between the two faces' offset regions (or refuses any facing
pair whose dihedral is below a threshold and whose footprints overlap,
the #571 direction), and is decided per pair like `shell_wall_clearance`.
