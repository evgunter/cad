---
id: a-flush-declared-reflex-union-ships-the-wrong-volume
kind: issue
title: A flush-declared union on the reflex-corner probe returns volume 16 against the closed form 15.979, sound at every tier
status: open
opened: 2026-10-02
priority: P0
cost: M
---


(JOIN-1 dual review, PR 3790: R1's reflex battery. Pre-existing: the
same on main at 0abf909cb and on JOIN-1's fixed head.)

## What

`a` is the 315° reflex prism `prism_z` over `(0,0) (2,2) (-2,2)
(-2,-2) (2,-2) (2,0)`, z ∈ [0, 1] (material everywhere but the 45° wedge
`0 ≤ y ≤ x`). `b` is `prism_ops` over the unit square `(0,0) (1,0) (1,1)
(0,1)`, z ∈ (1, 3), described with `describe_as_intersections` and
sheared `z' = z + sx·x + sy·y` with `(sx, sy) = (−0.5, 0.25)`, so its
bottom cap passes through `a`'s reflex corner `(0, 0, 1)`. The fixture is
`crates/sweep/tests/join1_r1_probes.rs` `join1_r1_reflex_battery`
(profile `sqQ1`).

`union_with(&a, &b, &flush_declarations(&a, &b))` returns a body that
passes tiers 2 and 3′ and the at-rest certificate, of volume **16.0**.
The closed form is `14 + 2 − v∩ = 15.979166…`: the overlap is the part
of `b`'s sheared floor below `z = 1` over `a`'s material, `y > x` and
`−0.5x + 0.25y < 0` in the unit square, `v∩ = ∫ (0.5x − 0.25y) dA =
1/192 + 1/64 = 1/48` (checked by hand: `x ∈ [0, ½]`, `y ∈ [x, 2x]` gives
`1/192`; `x ∈ [½, 1]`, `y ∈ [x, 1]` gives `1/64`). The union kept the
overlap twice — a wrong body that every gate passes.

The op is flush-declared, so the declared-REST zip takes over when the
join refuses; whether this body is the zip's or the join's is not
measured. On main the same pose's ∩ and ∖ refuse
`Join(UnpairedLooseEnds)`; on JOIN-1's fixed head they build sound at the
closed form (∩ = 1/48 exactly, ∖ = 14 − 1/48), which corroborates the
overlap. JOIN-1's fix pass leaves this union unchanged (still 16.0) and
turns four other poses of
the battery that were wrong on main (`sqQ1` and `dRight` unions at
`sx ∈ {−0.5, −0.25}`, volume 15 or 16) into `Euler(FanStartMismatch)`
refusals.

## Next

Trace which lane builds it; pin the pose as a row asserting the
closed form; the backstop that refuses an implausible volume does not
see a 0.13 % excess.

## Traced (JOIN reflex-corner lane, 2026-10-02)

The REST zip builds it. `ops::through_the_join`: the join refuses
`JoinDesync` "B senses agree at a matched pair", and
`rest::try_rest_union` answers the declared union from the saved
reduction. The first step that goes wrong is upstream of both: the
corner's vertex pair keeps four germs and `insert` runs one of B's null
edges the long way round
(`work/join/four-germ-vertex-pairs-run-b-in-a-order`). The second is
the zip admitting a union that is not a pure REST contact: the 45° wall
crosses `b`'s cap transversally at the same corner.

With JOIN's run-order fix applied (that row), the join still refuses
here, and the zip answers two more unions of the probe with the same
shape of wrong volume, `va + vb` with the overlap ignored: `eBot` at
`(sx, sy) = (−0.5, −0.25)` (16 against 15.880) and `(−0.25, −0.5)` (16
against 15.768). So the JOIN fix waits on this row.

## Wider than one pose (PR 3900's review, 2026-10-03)

The review's widened battery (`join_rc_probes` `rc_wide_battery`:
eleven shears per axis, the profiles turned as well) found the class
wider. Unturned `sqQ1` ∪ gives `v = 16` at 10 poses: sx ∈ {−0.75, −0.5,
−0.3, −0.25} with sy > 0. It is the same before and after PR 3900's
strut-order change. The bar is that `rc_wide_battery` reports no
wrong body.

## Measured on JOIN-2 (PR 3880, 2026-10-03)

The zip now reads the join's own segments (`join::section_segments`)
instead of pairing germs by its own facing test. At every wrong-volume
pose the join's matching itself refuses `JoinDesync` "B senses agree at
a matched pair", so the zip refuses with it rather than zipping
`vol a + vol b`. JOIN-1's reflex battery and `rc_wide_battery`, main
at `66bbdaa6b` against PR 3880's head: the 27 wrong-volume unions there
(3 in the reflex battery, 24 in `rc_wide_battery`) refuse typed, 11
`RestZipUnsupported` refusals become the join's `JoinDesync`, and
nothing else moves. `join_rc_probes`
`flush_declared_reflex_unions_never_ship_the_overlap_twice` pins four of
them as refusing or sound (red on main). The second cause, the zip
admitting a union that is not a pure REST contact, is unchanged: it is
no longer reached at these poses because the matching refuses first.

## Measured on main (zip-reflex, 2026-10-06)

Main at `3f1e3b0d03`, release, with an env-gated trace of the REST zip's
entry and exits (`zip/rest-admission`, the instrumentation commit).

- **The row's own bar is met.** `join1_r1_reflex_battery`: every ∪, ∩
  and `a ∖ b` is `SOUND` or `EMPTY ok` (`b ∖ a` is a harness gap,
  `work/join/the-reflex-probes-run-b-minus-a-under-declarations-keyed-for-a-b`).
  `rc_wide_battery`: 10 080 unions, 9 911 `SOUND`, 169 `Escalated` (all at
  the 0.003° turn), no `BAD`. `flush_declared_reflex_unions_never_ship_the_overlap_twice`
  passes, its four poses `SOUND` at the closed form. No union of either
  battery enters the REST zip: the join builds them all.
- **The second cause is live on main.** `try_rest_union` admits once
  the join's matching is complete, a `Rest` pair exists, and some
  seam-bounded region on each solid lies wholly on `Rest` surfaces with
  congruent cycles. It never checks that the seam is exactly that
  region's boundary, so a transverse segment beside the patch is minted
  and ignored and B is grafted whole. `zip_rest_admission_probes`
  `zip_rest_admission_reflex_lever` reaches it: the reflex pose's `b`,
  bridged high over `a` to a post resting on `a`'s top whose rounded
  footprint's west wall is flush with `a`'s at `x = −2`. The join refuses
  the post's tangent site (`Join(SectionInvariant { what: "tangent
  plane×cylinder germ pair …" })` in `a ∪ b′`, `"tangent section chord
  endpoints coincide along the ruling"` in `b′ ∪ a`), and the zip ships
  `vol a + vol b′` at all 16 lever runs (8 poses × 2 orders; the excess
  is `v∩` to 1e-9 in each). `sqQ1` at `(−0.5, 0.25)` and `(−0.3, 0.25)`
  pass tiers 2 and 3′, the certificate and the operand check; the other
  12 fail tier 3′, which the boolean's gate does not run. Each admitted
  run has 4 to 6 segments whose seam edge has a patch face on neither
  side (the cap across `a`'s top and its 45° wall). The controls build
  `SOUND`: the same `b′` with the post at `x = −1.9`, through the join;
  and the lever with `sqQ1 (0.25, 0.25)`, where `v∩ = 0`, through the
  zip.
- Across the sweep suite the zip is entered 95 times, from four join
  refusals (`RingHomingAmbiguous`, `NotSameFace`, the two tangent
  `SectionInvariant`s). It builds 91, and every one of them has no
  segment off the patch.
- The criterion "every segment bounds a patch on both solids" has a
  blind spot: `zip_rest_admission_tangent_lever` "inside-top" (a box
  dipping into the plate's top, inside the contact) has no segment at
  the dip at all. The zip identifies it and the result gate refuses
  `ResultInvalid { RingOutsideOuter }`. That run is caught, but by the
  gate, not by the admission.
