---
id: a-split-through-a-bore-makes-a-square-and-a-disc-not-an-annulus
kind: issue
title: split through a bored body makes each half's section a full face with no ring plus a coplanar disc face over the bore, not one annular face with a ring
status: closed
opened: 2026-09-28
priority: P1
cost: M
branch: cleave/section-rings
pr: 3658
closed: 2026-10-01
---


Found by CONTACT-6's reviewer (2026-09-28) and filed by the CONTACT
orchestrator. It is pre-existing at `main` before CONTACT-6.

**Repro.** `brick([-2,2]²×[0,2.5]) − rod(r=1, z∈[-0.5,3])` (through
`topo::subtract`), then `split` through `(0,0,1.25)` with normal
`(0,0,1)`. It also shows at normal `(sin .3, 0, cos .3)`, with an
off-centre rod, and on `bored_cylinder(0.3, 0.2, 0.37)` at tilts 0
and 0.3.

**Observed.** Each half's section is two coplanar faces: a full 4×4
face with no ring, and a two-edge disc (a circle or an ellipse) over
the bore. They overlap, and they cancel by orientation, which is why
point-in-solid and volume come out right. A section with a hole should
be one annular face whose ring is the bore's section. The encoding
also depends on the disc's sense bit, which `split_finish` inherits
from the bore wall; CONTACT-6 now reads that bit from the loop winding.
At base one half fails `validate_geometric` (`LoopRoleInverted` on the
disc). There is also `RingMeetsOuter` at an off-centre rod at tilt 1.4
with the normal flipped.

**Where.** The section assembly in `crates/topo/src/splitting/`
(`finish.rs` and the section's loop homing: `RingHomingAmbiguous` and
`TornComponent` also show on neighbouring poses). The fix is the ring
homing, so that the bore's section loop becomes a ring of the outer
section face.
