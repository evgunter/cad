---
id: a-corner-pair-with-an-edge-in-the-partners-face-plane-builds-with-undeclared-contacts
kind: issue
title: A corner pair with an edge in the partner's face plane builds a body tier 3' rejects (UndeclaredContact, StaleContactDeclaration)
status: parked
opened: 2026-10-05
priority: P0
cost: M
blocked_on: [intent-stage4-is-built]
---


## What

Found by PR 4061's review (NOTE 3). It is pre-existing: the lines are
identical on main.

In `corner_pairs_battery`, 287 runs build a body with the right volume
that `validate_pseudomanifold` rejects. All 287 are at exact-tie turns:
98 at ψ = 0, 34 at π/2 and 155 at π. By pair: w343-w330 93, w300-w270
107, w350-w200 87.

**The failures are definite, not census-escalated.** No error carries
an `Indeterminate`. The kinds, read per run, are:
- `StaleContactDeclaration` alone: 128;
- two `UndeclaredContact` (a `VertexOnFace` and an `EdgeFaceOverlap`):
  91;
- both of the above: 68.

**Each pose holds an exact coincidence: an edge in the partner's face
plane**, not a face in a face plane. At ψ = 0 or π the frame's `u` is
horizontal, and at ψ = π/2 its `w` is. So B's corner edge along that
axis lies exactly in A's top-face plane z = 1, and the witnesses read
z = 1.0 exactly. Example, `w343-w330 i=0 j=1 psi=0 ab U`:
- `UndeclaredContact { VertexOnFace, witness (-0.385, 1.963, 1.0) }`;
- `UndeclaredContact { EdgeFaceOverlap, witness (-0.193, 0.981, 1.0) }`.

The other two ops at that pose pass tier 3′. This is undeclared-
coincidence ground, so the row waits on D10.

## Repro

`cargo test -p sweep --release --test all corner_pairs_battery --
--ignored --nocapture | grep t3p=false`. Read each body's
`topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol)` errors
for the kinds.

## Re-pointed from the D10 hold (2026-10-08)

Waits on `intent-stage4-is-built`, not on the whole program: the volume is right; the fault is tier 3′ UndeclaredContact/StaleContactDeclaration on the contact records, which stage 4 replaces with one-door records and unproven-coincidence findings. (INTENT's re-homing of the parked rows, `work/intent/log.md`.)

## Also in `join1_delta_arc_battery` (2026-10-08)

Measured on main `047d10d5` and on `join/battery-hygiene`, unmoved
between them: 6 lines `ARC tri {half,shallow,tri} d=(0.0, 0.5)
zb=(0.5, 1.5) decl={false,true} U` build two solids at the volume,
with tier 2, the certificate and a legal operand. They fail tier 3′
on one `UndeclaredContact { EdgeFaceOverlap }`. The triangle's apex
edge `x = 0, y = 0.5` lies in the partner's face plane `y = 0.5`
(witness `(0.0, 0.5, 1.0)`).
