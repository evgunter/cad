---
id: shell-clearance-gate-skips-planar-pairs-tilted-off-antiparallel
kind: issue
title: wall_clearance reads only antiparallel planar pairs, so two planar walls meeting at an angle across less than 2t shell silently
status: closed
opened: 2026-10-06
priority: P1
cost: M
pr: 4311
branch: shell/tilted-walls
closed: 2026-10-08
---


Found by the `shell/planar-gate-misses` lane while levering
`shell_walls_antiparallel`, and **measured**.

`wall_clearance` (`crates/topo/src/shell.rs`) reads a planar pair only
inside one of two windows: its planes drift apart by at most one wall
`t` across the pair (`shell_walls_antiparallel` not Positive), or its
normals' cosine is antiparallel to the band
(`shell_walls_antiparallel_cosine` Zero, a tilt up to about `√(2ε)`).
Two non-adjacent planar walls that face each other at a steeper angle,
closer than `2t` over part of their overlap, are never read, and their
offsets cross. The module docs name the window ("a planar pair tilted
further than that"); nothing else schedules it.

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

The pair (tilt ≈ 0.54 rad) is outside both windows, and was outside the
cosine window the gate read before PR 4115, so the miss is not new. The
review of PR 4115 measured another instance on the base and the head
alike: an arm 100 long at `t = 0.001` returns `Ok`, volume `0.20918`.

A closing gate reads, for a facing non-antiparallel pair, the least
distance between the two faces' offset regions (or refuses any facing
pair whose dihedral is below a threshold and whose footprints overlap,
the #571 direction), and is decided per pair like `shell_wall_clearance`.

## Closed

2026-10-08, PR 4311. `moved_walls_cross` reads every transversal,
non-adjacent planar pair on the cavity the offset doors built, cutting
both faces by the line their moved planes share (lines, circles and
ellipses exactly), and refuses `ShellError::OffsetsCross` where the cuts
overlap. Both witnesses (the notched prism, the long thin arm) refuse
typed; the thick wedge at the same lean shells to its closed form; a
merge-base differential moved no outcome outside the PR's rows. Residues
filed: `tilted-read-accepts-a-zero-touch-on-any-vertex-sharing-pair`,
`tilted-read-takes-a-spline-or-spiric-edge-as-its-carrier-ball`,
`tilted-read-skips-edge-adjacent-pairs-that-cross-away-from-their-edge`.
