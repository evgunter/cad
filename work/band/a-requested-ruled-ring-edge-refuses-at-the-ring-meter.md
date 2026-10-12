---
id: a-requested-ruled-ring-edge-refuses-at-the-ring-meter
kind: issue
title: blend: a ruled band's requested edge in a planar support's ring refuses at the ring meter, though its strip may clear
status: open
opened: 2026-10-07
priority: P3
cost: M
---



## Finding

Arm (a) of `surgery::ring_clearance_pass` meters every ring of an open
link's support against the link's UNBOUNDED trimline. For a PLANAR
link whose requested edge lies in a ring, that ring is now arm (d)'s
(`RequestedBoundary::rings`, `surgery::strip_clearance`), metered against
the strip between the edge's stations. A RULED link keeps arm (a) for
every ring, because arm (d) meters planar strips alone: so a ruled
band's requested edge lying in a ring of its planar support (a blind
cylindrical groove's mouth line, read at its own trimline through
`co_requested_trim`, sits at margin zero) refuses `RingClearance`
however small the band, by the same mechanism the planar row
`a-requested-ring-edge-refuses-at-the-ring-meter` closed.

Not measured: no fixture builds the blind groove yet, so the first
step is a witness row (both verbs) confirming the refusal and that
nothing earlier — `open/ruled.rs`'s plan gates, predicate 6 at the
groove's end walls — stops it first. A blind half-cylinder groove
with its axis at the face's level (ends at `x = 1` and `x = 5` of a
`6 × 5 × 2` block) stops at plan with `UnsupportedRunOut` ("trimline
meets the end face's plane past its rim"), so the witness needs
another shape: its end walls must take the cut-offs. Only the fillet
can witness it: a chamfer of a cylinder–plane edge refuses
`ChamferArmUnsupported`.

**The carve-out arm (a) keeps for ruled links is pinned**
(`band_planar_cut_off_meters::a_ruled_links_strip_on_a_planar_support_refuses_a_feature_inside_it`).
A plane can support a ruled link and carry a planar chord in a ring.
A through half-cylinder groove along `x`, axis at `y = 4.6`, puts the
mouth line `y = 4.4` on the top face that carries the tabbed pocket's
ring. A spike or arc of that ring reaching `y = 4.35` lies inside the
fillet's strip (setback `√0.08 − 0.2`) and between the mouth's
samples. Filleting the mouth and the tip together, arm (a) refuses it
exactly, `RingClearance` at `0.05 − setback = −0.0328`.

With the ruled branch dropped, it still refuses, typed, but not
exactly:
- the spike's vertex at the reach (`reach::screened`,
  `FaceClearanceUncertified`);
- the arc, with no vertex in the strip, at the reach's cell meter
  (`FaceClearance`, bounded, on the arc's wall).

So any change to arm (a) for this item keeps a closed-form meter on
that ring.

Conservative, not unsound: nothing is built.

## What would close it

A witness first. Then meter that ring against the ruled band's strip
on the plane — bounded by its two caps, as `RuledPlan` holds them —
the way arm (d) meters a planar strip, and leave arm (a)'s unbounded
trimline for rings that carry no requested edge.
