---
id: k-names-that-record-another-quantity-or-band-since-the-readings-unit
kind: issue
title: Eight K names record another quantity or in another band since the readings-not-levers unit, and one is gone: the k-lint baseline has to re-take them
status: open
opened: 2026-11-23
priority: P2
cost: E
refs: [7442]
---



Filed by `oneround/readings-not-levers` (PR 7442) from its review's finding n4. That unit converts ten readings from `Lever::unsourced` to what each is, and with them what the K recorder samples under these names changes. The PR's CI did not run the k-lint tools job (the change filter skips it), so the first measurement is the nightly's.

## What changed, per name

| name | recorded before | recorded now |
|---|---|---|
| `bool_fate_corner` (the samples from `fate::tip_wraps`' count; its corner samples are unchanged) | two readings per call, `count + h` and `count − h` in half-turns at a unit arm: one near zero, one near `±2h` | the turning's gap from each half-turn mark the walk asks (`Period::decide`), as a share of a quarter turn among the `h` half-turn corners, levered at the least of those corners' levers: about `L/h` in size at a true count, one to three per call |
| `bool_join_arc_ahead`, `bool_join_arc_travel` | the signed travel margin `m` | the far row of `decide_across`: the magnitude `|m| + 2ε·cot ψ`, never negative |
| `bool_join_arc_clear` | `|m| − 2ε·cot ψ`, gated positive, asked unless the tie stood | `m` shrunk toward zero by `2ε·cot ψ`, signed, gated nonzero, asked only where the far row is positive |
| `bool_join_arc_slack` | `2ε·cot ψ`, asked at a tie | gone: no decision is made under it |
| `dihedral_wedge_at_arm` | `sin θ · Kε`, in the run's band | the wedge `sin θ · arm`, in the band at the tolerance the arm's refusal offers (not the run's band) |
| `material_pairing_offer_wedge` | `tan θ · Kε`, in the run's band | `sin θ · arm`, in the band at the tolerance the pairing's refusal offers |
| `section_pin_resolution`, `vertex_section_resolution` | `K·ρ`, in the run's band | `ρ`, in the band one K-step finer, `(ε/K, ε)` |

`pn_apex_tangent_reach`, `pn_apex_point_reach` and `pn_apex_pair_reach` move in last bits only (a quotient where a product was).

## What a re-baseline has to re-take

- The four names that now record in a band other than the run's (`dihedral_wedge_at_arm`, `material_pairing_offer_wedge`, the two resolution names): a rule that places a sample against the run's `(ε, Kε)` misplaces these. Whether k-lint reads the band a sample was decided in, or assumes the run's, decides whether they need an exemption or only new counts.
- `bool_fate_corner`'s boundary population: the old count readings sat at zero by construction (a decided tie); the new ones sit a quarter turn's share from zero.
- The join's three names: the caller's names lose their negative half.

## Done when

The nightly k-lint rows have run on a main that holds PR 7442 and their baseline is re-derived by the K-REPORT runbook where they fire, or it is shown that none of the eight moves a count.
