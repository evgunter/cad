---
id: cosurface-disjoint-curved-walls-refuse
kind: issue
title: Two stacked parts cannot both have a rounded outline - cosurface-and-disjoint walls glue as planes and refuse as cylinders
status: open
opened: 2026-08-31
github: 1352
refs: [1351]
---

## From GitHub issue 1352

opened 2026-08-31, 0 comments.

Two solids stacked on a shared footprint have COSURFACE outer walls with
DISJOINT extents: same carrier, same outward sense, meeting only along the
mating plane. While those walls are **planes** the boolean glues them with
no declaration at all. Round the outline's corners so four of them become
**cylinders**, and the same configuration refuses
`CurvedPierceUnsupported` — declared or not.

The practical consequence: **two stacked parts cannot both have a rounded
outline.** A bolted plate pair, a housing and its cover, any two parts that
sit on each other and share a profile — the moment that profile has a
corner fillet, the mate stops building.

Met in `demos/tour/src/twopeg.rs` doing Ev's montage-v3 ask to fillet the
plates' extruded profile.

## The controlled pair

Plate P spans `z ∈ [0, 1]`, plate Q spans `[1, 2]`, same 6×4 footprint, two
peg-in-hole fits between them. Everything but the outline held constant.

**Sharp outline** — six cylindrical faces per plate, all peg/bore:

```
two-peg mate WITH the three contacts declared: GLUED — volume 48 exactly
```

**Filleted outline** — four PATHS corner fillets, r = 0.5, top and bottom
edges left sharp so both mating faces stay the same rounded rectangle. Ten
cylindrical faces per plate, and every shared-carrier pair declared, the
four corner pairs included:

```
declared: 1 planar Rest + 22 cylindrical Rests (P has 10 cylinder faces,
Q has 10 — matched by shared carrier, so cross-peg pairs never arise)

panicked: declared two-peg mate failed: CurvedPierceUnsupported
  { operand: A, face: FaceKey(3v5), edge: EdgeKey(9v1),
    band: Band { zero: 1e-9, escalate: 1e-8 } }
```

— byte for byte the payload the **undeclared** mate gets. The only
difference between the two runs is the four corner cylinders.

## Why declaring does not fix it

`ContactClass` has two members: `Rest` — *"Conformal contact: same carrier,
**opposed senses**, gap ≡ 0"* — and `Tangent`. Neither describes this
contact. P's north wall and Q's north wall lie on one carrier and face the
**same** way: they are a cosurface CONTINUATION, not two surfaces resting
against each other. Declaring them `Rest` is arguably a false statement, and
the kernel does not accept it.

I did not determine which of these it is, and the distinction shapes the fix:

1. the reduction's curved arm refuses before any declaration is consulted
   for these faces, or
2. it consults the declaration, finds `Rest` does not hold (senses agree
   rather than oppose), and refuses correctly.

If (2), the missing piece is a **vocabulary** one — and `ContactClass`'s own
docs already anticipate growth: the `ALL` slice exists precisely so a new
variant cannot be silently omitted downstream, and the `content_tag` docs
discuss *"inserting `Fit` between the two"*.

## What the planar path does instead

Nothing is declared for the flat walls in the working configuration and it
glues, so the planar reduction tolerates cosurface-and-disjoint outright.
`demos/tour/src/booleans.rs::flush_declarations` also declares same-sense
flush PLANE pairs as `rest` (it computes `sigma = ±1` and accepts both
signs), and `bool_bodies::table` glues its corner-aligned legs that way. So
same-sense cosurface planes are accepted both undeclared and
declared-as-`Rest`; the curved arm accepts neither.

That asymmetry is the finding. However it is resolved — a curved arm for
cosurface-disjoint, or a class that names a continuation — the two should
agree, because a modeller who rounds a corner has not changed what the
contact IS.

## Not asserted

Which face `FaceKey(3v5)` is. It is on operand A, and both plates carry ten
cylinders in the filleted run; the controlled pair isolates the corner
cylinders without needing the identity, so I did not spend a build
confirming it.

## Meanwhile

The demo ships the plates SHARP (#1351), with the wall and this controlled
pair stated at `outline`. Per `memories/demo-purpose.md` the scene is not
contorted around it — no mismatched radii between the two plates, no
rounding one and not the other — because a shape arranged to dodge a
refusal stops measuring what using the library is like.

## Home

S-BOOL: the refusal is `CurvedPierceUnsupported` out of the boolean reduction in `crates/topo/src/boolean/*`, S-BOOL's territory, and the charter's containment doors that refuse legal inputs; the `ContactClass` vocabulary half is coordinated with S-MATE's declared-contact ground.
