---
id: repaired-pole-rim-serves-no-closed-door
kind: issue
title: fillet: a boolean-REPAIRED pole-touching rim is served by neither closed-rim door — one plane face, several mate half-bands
status: open
opened: 2026-08-29
github: 1245
refs: [1222]
---

## From GitHub issue 1245

opened 2026-08-29, 0 comments.

Surfaced by the BLEND-1 review (PR #1222), measured on the body a consumer actually holds.

**The shape.** A raw pole-touching revolve refuses `NonMaximalFaces` at every boolean door, so any consumer who booleans — the tour's own lily flow does — repairs first with `merge_coplanar_faces`, which merges each axis-touching cap's two half-disks back into ONE face. After that repair a latitude rim on a plane is still **two arcs** (the curved side is still seam-split — its seam meridians are not coplanar and do not merge), but their planar support is now a **single face**.

That shape is served by neither closed-rim door:

- `resolve_rim`'s host-side discriminant sees one planar face hosting every link and routes it to the **LADDER**, whose ring gate then refuses — `"a closed chain is not a ring of its plane support"` — because the rim sits in the plane's own outer boundary cycle, not as a ring of it;
- the seam-split **ANNULUS** arm is never reached, and would refuse anyway: its resolution requires each support to be a half-band carrying exactly one arc, and the repaired plane carries both.

So the multi-link seam-split band BLEND-1 shipped serves the UNREPAIRED shape; a plane-involving rim loses that door at exactly the repair the boolean lane requires. A rim between two CURVED walls is unaffected (nothing merges).

**Measured** at `crates/sweep/tests/blend1_r1_probes.rs::p4_the_repaired_lantern_neck_rim_is_outside_both_closed_rim_doors`: the repaired lantern's neck rim refuses on the ring gate, and one arc of it no longer registers a `SeamVertex` at all (the cap's seam is gone, so that end is trivalent) — so at least no recourse names a door that cannot serve it.

**What would close it**: a third resolution, or a widening of one of the two. The natural reading is that the ladder's gate is the wrong one for this shape rather than that a third band exists — the rim is a boundary cycle of the plane and a ring of nothing, while its mate side is half-bands — so this is most likely the annulus arm learning that ONE host face may carry several of the rim's arcs, with the host trim then minted as several arcs of one circle in one face's loop.

**Consumers**: the tour's lily lantern after its authored repair, and any revolve-then-boolean flow that fillets afterwards — which is the ordinary modelling order.

## Home

`work/issues/` — the closed-rim doors in `crates/sweep/src/fillet/surgery.rs` were S-BLEND's ground and that program is closed.
