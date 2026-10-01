---
id: the-plane-nurbs-seam-refusal-cell-is-below-every-gated-eps-row
kind: issue
title: The plane-NURBS seam's refusal cell moved below every gated eps row, so six rows now pin only their first-class arm
status: open
opened: 2026-09-30
---


Filed by PROPS' convex-insertion unit (PR 3524), which closed site 5 of
`props/f64-refinement-inside-an-enclosure-has-five-more-sites` by moving
`geom_core::spline::compose`'s `insert_once_ring` from the lerp form to
the convex form `c_{i−1}·β + c_i·α`. Owners by territory: `tint` and
`tcost` for the two `sweep` suites, `exch` as well for the `step-import`
one.

## What moved

The plane-NURBS declare-and-check seam's certified between-samples sup
is assembled over a Bézier decomposition, so it carried the lerp form's
compounding insertion width. It fell about two orders:

| row | sup before | sup after | boundary before | after |
| --- | --- | --- | --- | --- |
| `sweep` `m8_4_intersection_iso::seam_at_eps` | ~6.22e-12 m | **5.4680709999176037e-14 m** | attaches at ε_in ≥ 1e-9 | ε_in ≥ 1e-13 |
| `sweep` `review_probes_m8_4::probe_e_reversed_chart_takes_the_backward_candidate` | ~6.22e-12 m | **5.472048160373179e-14 m** | ε_in ≥ 1e-9 | ε_in ≥ 1e-13 |
| `step-import` `recognize_pins::the_integral_mixed_body_imports_first_class_with_a_charted_seam` | ~6.2e-12 m | **3.5528237131349995e-14 m** | ε_in ≥ 1e-9 | ε_in ≥ 1e-13 |
| `step-import` `recognize_pins::the_mixed_arc_prism_imports_first_class_over_the_intersection_pcurve_arm` | ~6.3e-12 m (6.3156e-12 in the payload) | **3.5528237131349995e-14 m** | ε_in ≥ 1e-9 | ε_in ≥ 1e-13 |

Measured by driving each row at `CAD_TOLERANCE_EPS` 1e-12, 1e-13 and
1e-14 in release. All six tests (the three `m8_4_intersection_iso` rows
share `seam_at_eps`) take the first-class arm at 1e-13 and coarser and
the typed-refusal arm at 1e-14, where the refusal still carries a number
strictly above ε. PR 3524 re-baselined the guards and the quoted sups;
this row is the part it did not fix.

## The residue

**`nightly.yml`'s eps matrix is `default`, `1e-6`, `1e-12`.** Every one
of those is now on the first-class side of the boundary, so the refusal
cell — "a bound too loose at ε refuses with its number, never through a
widened gate", which is the whole point of pinning both cells — is
exercised by nothing the gate or the nightly runs. Before this change
the 1e-12 row reached it.

That is a silent coverage loss of exactly the kind these rows exist to
prevent: the `Err` arms still compile and still assert, and they will go
on passing vacuously because no run enters them. The rows' own prose
says "Both cells are pinned; neither is widened" — which is no longer
true of any gated configuration.

## Options

1. **Add a finer eps row to the nightly matrix** (1e-14, or 1e-15 if the
   suite is clean there — PROPS measured `m8_4_intersection_iso` failing
   at 1e-15 and 1e-16 for an unrelated reason, so 1e-14 is the one
   known-clean candidate). One line in `nightly.yml`, and it restores
   the cell for every row of this shape at once, not just these four.
2. **Drive the refusal arm from a fixture rather than from the run's ε**:
   give each row a seam whose sup is above the run's ε at every eps the
   matrix runs, so the refusal cell is reachable at the default row. This
   is the more durable answer — the rows stop depending on where the
   matrix happens to sit — but it needs a seam per row, and the sup is a
   derived quantity, so the fixture has to be constructed to miss rather
   than chosen to miss.
3. Accept it and say so in the rows' prose. Cheapest, and it is what PR
   3524 did as a stopgap (each row now records that its refusal cell sits
   below every gated row), but it leaves four `Err` arms that cannot fail.

Option 1 is the one PROPS would take; the choice belongs to the owners.

## The recommendation, and why it is filed rather than done

PROPS recommends **option 1**: one line in `nightly.yml`'s eps matrix
(`for eps in default 1e-6 1e-12` gains `1e-14`). `1e-14` is the
known-clean candidate — PROPS drove `m8_4_intersection_iso` at 1e-12,
1e-13, 1e-14, 1e-15 and 1e-16, and only the last two fail, for a reason
unrelated to this change and not investigated.

It is filed rather than done because **adding a nightly ε row spends
this program's CI budget on this program's gate**, and that is the
owners' call, not a visiting lane's. A `work/` file is this project's
schedule; the row is on the slate with its options costed, which is what
a visiting lane owes.
