---
id: rest-zip-seam-chord-on-cylinder-wall
kind: issue
title: The declared-REST zip leaves a straight seam chord where a cap rim cuts a bore wall mid-height; the merge door's refusal hid it
status: open
opened: 2026-09-07
---


Found by CURVED-MERGEDOOR's row 0 / STOP 2 (spec
`docs/CURVED-MERGEDOOR-SPEC.md`), measured on `curved/merge-door` with
the door's non-planar declared pairs recorded instead of refused.

## The finding

Scenes A (floating peg: `collar()` ∪ `peg_at(0, 1.5, 1.0)`) and B
(mid-bore: `collar_at(0)` ∪ `peg_at(0, 0.5, 1.0)`) of `mate2_common`,
nine wall `Rest`s, exit the declared-REST zip (`crates/topo/src/boolean/rest.rs:315`)
carrying an edge between the peg's CAP plane (z = 1.5) and the BORE
wall (`Surface::Cylinder`, r = 0.5, sense in) whose stored curve is a
LINE chord:

    EdgeKey(22v3): description Scaffold(ExtrudedPoint { point: (0,0),
      place: translation(-0.25, -0.433, 1.5), vec: (0, 0.866, 0) }),
      carrier Line { origin: (-0.25, -0.433, 1.5), dir: +y }, params [0, 0.866]
    faces: plane z=1.5 (peg cap) / cylinder r=0.5 axis -z (bore, SurfaceKey(4v1))

The locus of plane ∩ cylinder is a circle; the edge is the 120° chord
between two rim vertices. On main the merge door refuses these scenes
first (`Merge(InvalidDeclaration)`), so nothing downstream ever
walked this edge. With the door recording instead, `describe_minted_edges`
(`crates/topo/src/boolean/ops.rs:868`) walks the recorded faces'
boundaries, classifies the dihedral `Transverse`, re-describes the edge
as `Intersection` on the SAME line carrier (`curved == false` because
the existing carrier is a `Line`), and `set_edge_curve`'s certification
refuses it — `Certification { error: ResidualExceeded { check:
Surface2Residual, sample: 1 } }`, the line's samples are off the
cylinder — which `ops.rs:978-981` maps to
`JoinDesync { what: "minted-edge description failed certification" }`,
DROPPING that payload (`map_err(|_| …)`; measured with a lane-private
print).

So the zip ships a body whose boundary carries a chord where the
adjacency demands an arc, and the pre-MERGEDOOR refusal was the only
thing standing between that body and a consumer. Tier 3 is never
reached on A/B today (the boolean refuses first), so whether tier 3
would catch the chord is unmeasured.

Scenes C (proud one end) and D (partial engagement, `r1_probes_m9_3`)
pass the same walk and are tier-3/3′ green, so the shape is specific
to a cap rim cutting the bore wall MID-HEIGHT with the bore remainder
surviving on both sides of nothing — A's bore remainder z ∈ [1, 1.5]
under the peg's bottom cap; B's z ∈ [1.5, 2] over the peg's top cap.
Which zip step mints the chord (the seam segment, or an operand rim
edge re-homed onto the bore) is not measured here; the edge's slot
version (`v3`) says it was re-minted twice.

## Recourse

The zip (CURVED ground, `rest.rs`) must mint the cap-rim seam on the
cylinder as a `Circle` arc with a certified description, or refuse
typed before the output stages. Until then, opening the merge door for
non-planar declared pairs lands A and B on `JoinDesync` — the
MERGEDOOR unit's STOP 2 — and its rows 1, 2 and 6 cannot go green.
Separately (S-BOOL's ops.rs): `describe_minted_edges`' `JoinDesync`
arm should carry the certification error it discards.
