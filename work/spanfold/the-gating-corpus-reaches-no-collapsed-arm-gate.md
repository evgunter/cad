---
id: the-gating-corpus-reaches-no-collapsed-arm-gate
kind: issue
title: The gating corpus reaches no collapsed-arm gate: five of the eight predicates have zero rows, three have only positive
status: open
opened: 2026-09-20
priority: P3
cost: M
---


## The digits

Across every committed k-report baseline
(`docs/k-report-data/eps-1e-{6,9,12}.csv` and the `m4`/`m5`/`m7`
archives, 12 files), counting rows by predicate and outcome:

| predicate | rows | outcomes |
|---|---:|---|
| `dihedral_arm` | 606,900 | `positive` only |
| `enters_material_arm` | 47,151 | `positive` only |
| `nurbs_span_meter` | 12 | `positive` only |
| `material_wedge_side` | 0 | — |
| `pcurve_interval_meter` | 0 | — |
| `plane_nurbs_transversality_reported` | 0 | — |
| `ssi_transversality_arm` | 0 | — |
| `tangent_sector_order2_arm` | 0 | — |

**Five of the eight have no row anywhere; the three that appear have
only definitely-positive ones.** So no corpus document has ever taken
one of these gates, before or after PR 2928 routed them through the
funnel, and the "no population moved" receipt in that PR is a fact
about the corpus rather than about the gates.

## Why, and why that is not simply a corpus gap

Each gate is a defense-in-depth arm, and at every shipped caller the
caller has ALREADY gated the same quantity:

- `topo::splitting::rules` decides `split_sector_extent` positive before
  it calls `enters_material` with that extent.
- `topo::census`'s pairing lane runs the dihedral gate first, so a pair
  reaching `classify_material_pairing` reads smooth: the wedge
  `sin θ · arm` is within ε. That does not keep the pairing margin
  `cos θ · arm` out of the band, though. It is bounded below only by
  `√(arm² − ε²)`, so an arm near `K·ε` can land it in band. At `K < √2`
  it can even be decided Zero, on sound geometry (ENCL PR 4474, whose
  split-finish row reaches it at `K = 1.2`). Such a refusal is the arm,
  too short to read a side over, and it ends as
  `geom_brep::MATERIAL_PAIRING`.
- `nurbs_span_meter` and `pcurve_interval_meter` gate a knot domain's
  extent; a zero or reversed domain is a malformed carrier the mint
  side refuses.

So reaching a gate needs geometry a constructor refuses to build. That
is the right posture for the arms and it is also why they are the least
exercised code in the funnel: their only executed evidence today is
four unit rows in
`crates/geom-brep/tests/kstats_escalation_channel.rs`, which call the
predicates directly.

## What closing this looks like

Either a fixture that builds the refused geometry behind the
constructor (a `Body` assembled by hand, the way `topo`'s own
failure-injection doors do it) and drives one gate end-to-end through a
node evaluation, so the escalation is observed on `NodeValue::
escalations` and in `drive`'s classification — or a ratified statement
that these arms are unreachable from any well-formed input and that
direct unit rows are the coverage they get, recorded where a reader of
the K report will find it. The first is worth more: it would also pin
the plumbing from a gate through the node's frame to the driver, which
nothing currently executes.

## Re-homed from FLUX to FLUXTAIL (2026-10-10)

(FLUX orchestrator) FLUX measured 125.5 budget points against 30 and was cut on its priority seam: FLUX kept the curved closed-form arms. FLUXTAIL collects the curved arms' numeric honesty rows: rim levels, chord lengths, span folds and bounds that cancel, drop a refusal or depend on the frame, under FLUX's closed forms. The id and the body above are unchanged; the move may have set `priority`, `cost` or `status` in the header.
