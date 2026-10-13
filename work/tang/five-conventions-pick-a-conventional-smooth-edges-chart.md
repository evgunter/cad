---
id: five-conventions-pick-a-conventional-smooth-edges-chart
kind: issue
title: five conventions decide which adjacent chart a conventional smooth edge rests in
status: open
opened: 2026-10-12
priority: P1
cost: M
---


## Finding

Raised by the review of the PR that routed `Body::split_edge`'s tangent
children through the must-carry rule.

A smooth edge the rule calls under-determined stores a chart image,
and the rule leaves "which adjacent chart" to the caller: it is the
`chart` argument of `geom_brep::must_carry_description`, handed back
in `MustCarryDescription::Conventional`. Each caller decides differently:

| caller | chart |
|---|---|
| `chord_join::chord_spec` (split's tangent ruling) | the section plane |
| `splitting::finish::describe_section_boundary` | the section face's surface |
| `sweep::extrude`, cap rim | the wall |
| `sweep::extrude`, strut | the previous segment's wall (`k_prev`) |
| `sweep::revolve::upgrade` | `s1` |
| `sweep::blend::surgery`, contact edge | `he_plus`'s face |
| `boolean::ops`, a stale curved seam | `s1` |
| `split::Body::tangent_piece` | `he_plus`'s face |

Each is argued sound (the locus lies exactly in both charts), and the
battery holds each one up. The cost is that one geometric seam is
stored two ways depending on the door that made it. Observed on
`wedge_end_doors.rs`'s shoulder: the seam a thin shoulder's cut mints
stores `Chart(Plane)` through the chord, while the same ruling cut
short by `split_edge` stores `Chart(Cylinder)`. A reader that compares
descriptions (naming, adjacency coherence, a later restatement) sees
two answers for one locus.

## The question

Weighed by a designer pair. What the chart choice is evidence of: a
recorded value that records nothing. On a derived two-surface seam the
carrier fixes the locus, each face's pcurve is its own derived cache,
and the chart key only names one of the two faces, chosen by
construction history. The readers that act on it are the spline pcurve
mint (`topo::pcurves::nurbs_iso_derive`, the P0
`nurbs-iso-derive-line-cap-arm-guesses-wrong-channel`) and
`boolean::edge_join`, which never re-joins a two-surface conventional
seam. Naming, tier-3 adjacency and `topo::iso` do not read it. A probe
(branch `tang/probe-smooth-seam-chart-offset`) found single-face offset
indifferent to it.

Ev took both decisions (PR 4907, 2026-10-13), stated in D2:

1. Such a seam stores `OnPair { pair }`: no chart, an incidence-only
   certificate, admitted where tier 3's must-carry predicate reads the
   join under-determined. `Chart` stays for loci one chart defines. The
   nine doors (eight minting doors and STEP adoption) drop their chart
   choice.
2. A mixed edge (some stations definitely transverse, others definitely
   smooth) brackets an unsampled in-band point and refuses typed. Today
   tier 3 exempts it (`topo::validate`, `TransverseNotIntrinsic`'s doc,
   entered 2026-07-19 as "not enforced at M2") and the boolean's
   stale-seam arm stores it as `Chart(s1)`.

Ev on the second: make mixed edges illegal now, and if one comes up
later, investigate then how to handle the case better and what recourse
the user is offered. No construction reaches one today (the boolean's
mixed branch had 0 hits over the battery; Steinmetz and Viviani refuse
upstream), so the refusal is a fence, not a regression.

## The build

- `EdgeDescription` and its spec gain `OnPair { pair }`, certified by
  incidence on both surfaces (each surface's per-sample residual, as
  `Intersection` meters it), no witness.
- `MustCarryDescription::Conventional` carries nothing;
  `must_carry_description` drops `chart` and `foreign_chart`; the nine
  doors (the table above and STEP adoption's conventional rung) drop
  their chart choice.
- Tier 3: adjacency reads `pair == {fs_plus, fs_minus}`; the mixed
  exemption becomes a refusal. `MustCarryVerdict::Transverse` splits
  into all-transverse and mixed, the mixed arm escalating typed with the
  two bracketing stations; the boolean's stale-seam arm refuses there.
- `boolean::edge_join` gains a same-pair arm; `replace_face` and
  `offset_restate` re-section a moved `OnPair` edge against the moved
  pair and re-ask the rule; `nurbs_iso_derive` derives both faces' rows
  through one arm (closing the P0
  `nurbs-iso-derive-line-cap-arm-guesses-wrong-channel` if it is still
  open).
- Tests asserting a door's chart re-baseline to the pair.
