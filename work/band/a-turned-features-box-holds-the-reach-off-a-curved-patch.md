---
id: a-turned-features-box-holds-the-reach-off-a-curved-patch
kind: issue
title: The reach reads a turned face through its world box, so a feature clear of a curved patch certifies only from far clearer
status: open
opened: 2026-11-17
priority: P3
cost: M
refs: [7078]
---

Found by PR 7078 (lane `band-reach`). The reach's cell walk tiles each face's world box (`reach::face_bound`). A turned face's box reaches past the face itself. Against a curved region's bound, the cells near the box's corners cannot clear, and the face refuses `FaceClearance { bounded: true }` well short of its real clearance.

**Measurement.** The flush mixed corner's reflex void, a cube of side 0.04 near the fillet's torus patch (r = 0.1), turned 0.7 rad about `x` (`common::poses::torax_pose`):
- 3% of the size clear (0.003): refuses, margin −1.6e-3;
- 0.01 clear: refuses, margin −5.5e-4;
- 0.02 clear: builds.

In place, it builds 100ε clear. The chamfer's planar region builds at 3% turned. `band_mixed_corner_reach::a_turned_and_moved_mixed_corner_reads_as_it_does_in_place` pins the 3% case as "builds or refuses as a bound" and the 20% case as builds.

What a fix needs: cells read in the face's own frame (its plane's chart, a turned box), or the face's surface-distance clearing read over the trim rather than the box. The meter's "a clear face can be refused uncertified, never the reverse" holds either way.
