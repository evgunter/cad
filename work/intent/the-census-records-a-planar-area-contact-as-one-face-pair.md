---
id: the-census-records-a-planar-area-contact-as-one-face-pair
kind: unit
title: the at-rest census records a planar area contact between arrivals as one face pair, a flush pair as its own row, and a touch of its faces' boundaries as the pair's only inside its overlap's closure
status: closed
opened: 2026-10-22
closed: 2026-10-22
priority: P1
cost: H
branch: intent/census-planar-face-pair
pr: 5605
---


## What

`[assertions]` as ratified on PR 5555, and confined by location on
PR 5605 (Ev, 2026-10-23):

- **An area contact is one row.** Between two arrivals, the census
  decides each pair once, at the highest-dimensional cells that touch.
  Two faces meeting over an area are one pair. A touch of a cell
  bounding one of them against a cell bounding the other is part of
  that pair where it lies in the closure of their overlap; elsewhere it
  is a pair of its own.
- **A flush pair is its own row.** Two faces on one carrier, facing the
  same way, whose boundaries meet while their interiors do not, are a
  flush pair. It is a finding unless it is established.

Today the census falls short in four ways:

- **No planar face pair.** `crates/topo/src/census.rs`
  `sweep_conformal_patches` is curved-only, so a planar contact between
  arrivals shows up only as its edge and vertex events: about 30
  findings for one seat.
- **No flush pair.** Nothing records it.
- **Backing reaches outside the overlap.** `vv_face_backed` and
  `ee_face_backed` back an event anywhere on a declared face. Only
  `vf_face_backed_confined` (via `pair_verified`) checks that the event
  is in the verified overlap.
- **A near-flush mated seat refuses the product.** `collinear_overlap`
  escalates an in-band near-flush edge before any backing is consulted,
  and `checks::at_rest::partition` sends the pair-less escalation to the
  gate.

## Owed

- **A planar face-pair arm.** Door 1 checks one carrier with opposed
  normals; Door 2 checks an overlap of positive area.
- **A flush arm.** Door 1 checks one carrier, aligned. Door 2 finds the
  overlap empty, and the shared boundary segment is the witness.
- **Confinement.** Every backing check goes through `pair_verified`.
- **No refusal at rest.** An in-band event inside a confirmed pair's
  boundary is asked nothing. Elsewhere, an in-band pair is that pair's
  "could not decide" finding, not a product refusal.
- **Quieting.** A flush finding is quieted by a measure that reads the
  signed offset between the two carriers along their shared normal.
- **Rows:**
  - a flush block on a block, unmated, flush on 1, 2 and 3 sides;
  - an overhang;
  - a near-flush pair in band;
  - two coplanar faces that meet only along a line (no face pair);
  - perf12's census goldens, re-blessed with their stated causes.
- **On stage 4 I's path:** `mates-declare-no-contact` waits on this.

## Closed (2026-10-22)

- **The planar face-pair arm** is `census.rs` `planar_face_pairs`, run
  before the sweeps over planar faces of two solids whose boxes meet.
  Door 1 is the undeclared carrier reading (`SameOpposite`, a Zero
  margin); Door 2 is the declared-pair overlap door on Door 1's verdict,
  in either frame order, `PositiveArea`. An unrecorded pair is one
  `UnrecordedContact` (`ConformalPatch`) carrying Door 1's margin, which
  `census_contact_row` records as one `SameOpposite` face row. Door 2's
  interior-witness rung now also runs on an escalated region walk
  (`chart_region.rs` `declared_pair_overlap`), so a near-flush seat's
  in-band boundary no longer keeps its area from certifying.
- **The flush arm** is `AtRestPolicy::flush_at_rest`
  (`census.rs` `flush_at_rest`), between groups only, called by the
  gather (`product.rs`) and the STEP import's gate. Door 1 reads
  `SameOriented`; the witness is an edge of each face overlapping
  along a positive length; Door 2 reads no certified area both
  interiors hold (`Empty`, or `TouchingBoundary` with the witness rung
  declining, which is the empty overlap at ε for a pair whose trims
  share a boundary). The row is `SameOriented` on the two faces, cited
  by no record. An in-band pair is a `CensusUndecidable` naming the two
  faces.
- **Confinement, by location.** Every face-pair rung reads
  `Declared::confirmed`: each record that passes `pair_verified` or the
  confirm pass's own reading of it (a point touch, a tangent line), and
  each pair the planar arm found, recorded or not. A touch of the two
  faces' boundaries is the pair's only where it lies in the closure of
  their overlap (`Declared::holds`, `in_overlap_closure`: both closed
  regions hold the point, and where both hold it on their boundary their
  corners share material, `corners_meet`); `Declared::confine` lets the
  lane read the touch first, and drops what it found, an in-band reading
  included, only where a pair holds it. Elsewhere the touch is decided
  as a pair of its own (`m9_c1_r1_probes::the_lemma_probe_declared`'s
  tangent vertex), and a place the census cannot read (a curved pair, a
  pinch or an arc corner) is a `CensusUndecidable` naming the two cells.
  A vertex inside the other face, and an edge's run through it, lie in
  the closure by incidence alone (`vf_face_backed`, the edge-face lane).
  `vf_face_backed_confined` and `ee_face_backed` are gone. A record
  that does not confirm answers for nothing, and
  arm 2 no longer refuses on one (`RecordedFacePair` is gone). The
  crossing rung still reads the side of every recorded pair, so
  `SameSide` is still named.
- **No refusal at rest**, elsewhere: not built. An in-band face pair's
  own boundary cells still escalate, a pair-naming verdict still blocks
  the arrivals' records, and the non-refusing vessel is stage 5's:
  `an-in-band-face-pair-at-rest-is-not-one-finding`.
- **Quieting.** `Gap` between two parallel planes is
  `(o_inner − o_outer) · n_outer` (`eval/measure.rs` `gap`), the signed
  offset along the outer face's outward normal: for an aligned pair the
  role swap negates it and `= 0` reads either way, so it is the measure
  the item names. The `unproven-coincidence` check reads no assertion
  yet (`ChecksConfig::unproven_coincidence`), which is stage 5's.
- **Rows:** `census_records_what_it_decides` (flush on 1, 2 and 3
  sides, the overhang, the near-flush side, coplanar faces meeting
  along a line); `census::tests::a_confirmed_pair_elsewhere_backs_no_crossing`;
  `mate6_gather_mints` (a mated seat still proven by rung 1, beside its
  four flush sides); perf12's census goldens re-blessed (rows only);
  the tour's bench reports its two flush post ends.

