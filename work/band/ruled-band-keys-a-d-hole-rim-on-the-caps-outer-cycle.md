---
id: ruled-band-keys-a-d-hole-rim-on-the-caps-outer-cycle
kind: issue
title: sweep: a ruled crease whose ends sit in a cap RING (a D-shaped through-hole) now carves; it refused BodyNotIntact
status: closed
opened: 2026-09-13
priority: P0
cost: H
closed: 2026-09-25
pr: 3243
branch: band/ruled-d-hole-ring-crease
---



## Finding

A block with a D-shaped THROUGH-hole — a square outer loop plus a D ring
(chord at `x = flat`, major arc of radius `R`), through the extrude door,
tier-3 valid — has a concave cylinder–plane crease along the ruling whose
two ends sit in the caps' RINGS, not in their outer cycles. Requesting
that crease from `fillet_edges` refuses

    BlendError::BodyNotIntact { detail: "a face's outer cycle does not walk,
    or does not carry the half-edge the carve keys on" }

from `chord_site` (`crates/sweep/src/blend/surgery.rs`, the `not_intact`
after `flank(body, outer, …)`), reached from the ruled carve's per-end cap
cut-off (`crates/sweep/src/blend/open/ruled.rs`, `ruled_phase`'s
`chord_site(body, end.cap, …)` before the "cap cut-off mef"): `chord_site`
reads `get_face(face).outer` and the crease's two rim edges are in the
cap's ring, so the flank walk cannot find them.

The variant is the wrong one. `BodyNotIntact`'s doc says the INPUT does
not hold together (D2 addendum row 1, no recourse), and this input holds
together — it is a body a user makes in one extrude. What it is is a
frontier of the ruled door (row 2): a ruled crease terminating in a ring
of its cap, which `RuledPlan::plan` does not classify — its ring gate
checks the two SUPPORTS for rings (the pocket-in-the-flat row,
`review_fillet_h7_r1_probes::a_support_carrying_a_ring_refuses_at_the_ruled_plan`)
and a cap carrying a ring is admitted (the bored D-rod row) because THAT
crease sits in the cap's outer cycle. The honest answer is an
`UnsupportedChain` (or `UnsupportedRunOut`, whichever the cap-incidence
classifier's vocabulary already carries) at the plan, with the recourse
that is true of it, before any `mef` runs.

Measured on BLEND-14's Phase 1 (the concave near-osculating family, a D
bore at `R/r = 2` and `1.5`, `r = 0.1`): both refuse as above; at
`R/r ≤ 1.1` the clearance screen refuses first, so the ring case is what
keeps the concave difference-branch family out of the door.

## Disposition

Not this unit: BLEND-14 routes `attach_contact` through the must-carry
rule and touches neither the ruled plan nor `chord_site`. The fixture is
three loops in a test (`block` square, D ring, `extrude`); the row is the
refusal's VARIANT and site, not its existence.

## Re-homed at BLEND's exit (2026-09-17)

Filed by BLEND unit 14's implementation pass and merged with that unit on 2026-09-17, after the cut branch was drawn; moved here at BLEND's exit walk. The ruled band's walk is `crates/sweep/src/blend/open/ruled.rs`, CARVE's ground.

## Built, not refused (2026-09-25, `band/ruled-d-hole-ring-crease`)

The finding's diagnosis held — `chord_site` read `get_face(cap).outer`
and the crease's rims are in the cap's ring — but its disposition did
not: the ruled carve BUILDS this crease once the chord is hung in the
cycle that carries it. `mef(Chords)` keeps the split cycle's outer/ring
designation on the old face and moves only the run from one foot through
the old vertex to the other onto the sliver, so the ring case is the
outer-cycle case's combinatorics exactly; the supports' strips, the
crease `kef`, the sliver `kef` and the spur `kev` touch no cap cycle but
the one cut, and every mef-minted face is ring-free. `chord_site` now
walks the face's outer cycle and its rings and refuses (Row 1) unless
exactly one of them carries the keyed half-edge.

Pinned by `band_ruled_d_hole` (the D hole at flats 0.3 and −0.2 and at
`R/r = 2`: two bands, `(v+4, e+6, f+2)`, tier 3, naming totality, each
cap's ring = two surviving middles + two cut-off arcs of radius `r`,
`ΔV = +2·A·L` against `rod_section_cut`), and by the re-baselined
`review_contact_edge_must_carry_r1_probes::r1_the_d_bore_crease_carves_in_its_caps_ring`
(BLEND-14's `R/r = 2` bore, one crease requested alone).
