---
id: blend-reach-skips-every-face-at-a-chain-vertex
kind: issue
title: blend: the reach meter skips every face at a chain vertex, so an end face that folds back into the band away from the vertex is never metered
status: open
opened: 2026-10-06
priority: P2
cost: M
---


Found by the lane that built predicate 2's reach
(`blend-material-is-never-checked-against-faces-that-are-not-its-supports`).

`band_reach` in `crates/sweep/src/blend/reach.rs` (`excluded`) skips, for
each chain, its supports and every face meeting one of its vertices. A
face at a chain end is the face the band runs into (a cut-off's end face,
a mitre's other support, a ruled cap), and it touches the band's region
at the vertex by construction, so metering it whole would refuse every
open chain. But the skip is by face, not by neighbourhood: an end face
that is not convex — an L-shaped end plate whose other arm overhangs the
band further along, or a curved end face that comes back — can meet the
band's material away from the vertex, and nothing meters that.

The fix is to meter such a face against the reach with the vertex's own
neighbourhood (the end section, which the end face's plane already bounds
in the reach — `straight_reach`'s caps) taken out, rather than not at all.
No body on main is known to reach it; a witness should be built first.

## Findings (2026-10-07)

Measured against the tree, the skip is wider than the row says, and the
hole it describes is not where the row puts it:

- `excluded` skipped, for every link of a chain, the faces at EVERY
  vertex of that chain, not only at the link's own two.
- At a link's own vertex the skip is sound. The battery refuses a curved
  end face (`END_FACE_CURVED`, both open bands; corner supports and turn
  supports are planes), and `straight_reach` caps the window with the
  plane of every non-support face at its own vertex. A plane end face,
  however non-convex (an L-shaped or stepped plate), lies on that cap's
  zero set, the reach's boundary, so it never meets the reach's inside.
  The overhanging arm of an L-shaped end plate is a different face, at
  no vertex of the chain, and was always metered. The end face's ring
  meter refuses it first at the door
  (`blend_band_reach_chain_ends::an_l_shaped_end_plates_arm_over_the_band_is_metered_and_refused`).
- The real hole was a cut-off end face at ANOTHER link's end of the
  same chain. Where it lives in production: the battery breaks a chain
  at every turn before the reach reads `verdict.chains`, so a
  multi-link chain spans valence-2 joints only, and the hole was a
  jointed chain's far end face cutting the near link's band. Witness,
  read at the meter (`band_reach_for_tests`, which now builds the
  verdict's chains, turns broken): a station on the rim of a prism
  whose end face leans back over the edge. Main's meter passes it; the
  door's joint-foot screen refuses it first
  (`…::a_jointed_chains_far_end_face_is_metered_against_the_near_link`).
  No door-level body is known to reach it.
- A thin trapezoid whose back face ends two links and cuts the band of
  the link between them was first read as the witness, through a test
  door that walked chains without breaking them at turns. In production
  the three links are three chains, and the back face was metered
  against the middle one all along; it stays as a pin
  (`…::a_cut_off_face_at_another_links_end_is_metered_against_the_link`).

Fix: a link's reach skips its chain's supports and the faces its own
window ends in, which `straight_reach` records as it caps the window
(a plane end face, or a corner patch's support); a curved face there
fails loud, and a circular reach, which has no caps, skips none. The
"meter with the vertex's neighbourhood taken out" the row proposed is
not needed for an own-vertex face, because the cap already takes that
neighbourhood out. Siblings filed:
`blend-reach-never-meters-the-bands-of-one-chain-against-each-other`,
`support-screen-skips-adjacent-boundary-features-wherever-they-approach`.
