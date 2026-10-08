---
id: ruled-cut-off-builds-a-bore-wholly-inside-the-removed-sliver
kind: issue
title: "blend: a bore wholly inside a ruled cut-off's sliver refuses RingClearance; may a blend delete an authored through-feature?"
status: open
opened: 2026-09-25
priority: P2
cost: H
---


## The question

A convex ruled crease's cut-off removes a sliver `S` from each
transverse cap. A bore whose walls lie wholly inside `S` refuses
`RingClearance` at the cap today (`ring_clearance_pass` arm (c),
`crates/sweep/src/blend/surgery.rs`; `CapSliver`,
`crates/sweep/src/blend/open/end_face.rs`). Rows:
`band_ruled_cap_ring::a_bore_inside_the_d_rods_removed_sliver_refuses_ring_clearance`
and
`review_band_ruled_ring_probes::a_bore_in_a_keyhole_creases_removed_sliver_refuses_ring_clearance`.

The alternative is to build it: the bore dies with the sliver, its two
rings, the walls between them and their names gone, at
`ΔV = −2·A·L + V_bore`. That is a blend deleting an authored
through-feature, which no blend does today. Whether one may is the
open question, and four things turn on it:

1. **Intent (D10).** The author placed the bore. A fillet that erases
   it without a word is the silent outcome fail-loud exists to
   prevent, unless something reports it. D10 routes known-answer
   complaints to lints, so a third answer is to carve and report a
   finding ("the blend consumed the bore").
2. **Naming.** N5 already resolves a gone name to `Vanished`, so
   nothing dangles silently. But it is a new way for a name to
   vanish: a later step reading the bore's wall would break on an
   upstream radius edit, not on an edit to the bore.
3. **Predicate 2.** The reach meter meters the band's volume against
   every non-support face, and the bore's walls lie inside that
   volume. It would have to learn which faces die with the sliver,
   which changes what predicate 2 certifies.
4. **Exact membership.** Deleting the bore needs an exact "wholly
   inside `S`" test, a Q1 trilean with its own margin, and the
   refusal split into "straddles the arc" (a frontier: the band would
   be trimmed by the bore) and "wholly inside". The enclosure `Ω` the
   meter reads cannot serve: it contains `S`, so a ring inside `Ω` may
   still cross kept material. Its surgery is likely `kfmrh`/`kef` over
   the bore's walls.

`crates/sweep/README.md`'s ruled-band clause ("every other edge of
the cap stays where it was … an edge not definitely clear of the
region refuses `RingClearance`") came from a BAND fix pass, not from
Ev (`git log -S`), so it binds neither answer; a carve would re-word
it.

**Recommendation (PR 4271's lane, both reviewers concurring):** keep
the refusal. An author who wants the bore gone deletes it in one edit,
and the refusal's sentence already says the edge lies in the material
the blend removes. If the carve is wanted, it comes with the finding
in (1), predicate 2 exempting the dying faces (3), and the exact
membership test (4).

## Done

- **An edge that leaves `Ω` through different faces** (PR 4271). A
  straight cap edge is read point by point against `Ω`
  (`CapSliver::line_clearance`), at `f64` and at `Interval`. Three
  rectangular holes in the D-rod and the keyhole at `r = BR` and
  `1.1·BR`, all refused before, carve at their closed forms. Curved
  edges are still read term by term:
  `cap-sliver-meter-reads-a-curved-edge-term-by-term`.
- **The `V − c` half-plane's wedge near each foot** does not bite: the
  floors on the section's own axes (PR 4173) close it.
  `band_ruled_cap_ring::a_bore_on_the_arc_just_past_a_foot_carves_at_the_closed_form`
  pins it.
- **The keyhole's slot end edge** (the rocker's wall 2) carves past
  `r* = 0.3097` since PR 4173.
- A D-rod bore and five straight-edged holes clear of `S` still refuse
  the reach meter's `FaceClearance`:
  `blend-reach-refuses-a-bore-clear-of-a-ruled-cut-offs-sliver`.
