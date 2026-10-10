# RESTREAD — the log

## 2026-10-09 — opened

Cut out of CONTACT by CONTACT's orchestrator when it closed (Ev, in
chat 2026-09-29: close CONTACT and move all remaining rows to 2-3
successor programs). CONTACT measured 99.5 budget points against 30;
its units CONTACT-10, -11 and -12 merged (PRs 4363, 4368, 4372) and
CONTACT-13 parked on the D10 hold. Every row here moved by `git mv`
with its id, body and history unchanged. Rows dispatchable: 11, for 29.5 points.
— (CONTACT orchestrator)
- 2026-10-09 — Seam note from ENCL (PR 4443, merged at `070cec41da`): the "X is too close to call at this tolerance" sentences in `topo::validate`, `topo::census` and `geom_brep::props::PropsError::Escalated` now read "X is undecided" (one composer: `Indeterminate::undecided` / `geom_core::undecided!`). Tolerance advice lives in the ending, per D4 ¶1 (i). Not-yet endings join a note with `"; "` via `geom_core::noted`, and the `Recourse:` label is `geom_core::Recourse`. A new refusal on your ground should use these instead of hand-spelling. The remaining "too close to call" sites are listed in ENCL's `too-close-to-call-remainder`. (ENCL orchestrator)
- 2026-10-10 — Seam note from PIPE (PR 4486, `pipe/census-arms-and-witness-caps`): `boolean::boxes::edge_box_rule` now takes the certified `Option<&geom_brep::EdgeCurve<T>>`, and `EdgeBoxRule::ConicAmplitude` carries `params: (T, T)`. `AxialCarrier::Conic::params` is `(T, T)`, not `Option`, and the full-turn `None` arms (unreachable by construction) are gone. No box moved. (PIPE implementer, D291 lane)
- 2026-10-10 — Seam note from PIPE (PR 4486, `pipe/census-arms-and-witness-caps`): `ChartRegionError::WitnessBudgetExhausted { segments, cells }` is split into `WitnessSegmentCapExceeded { segments }` and `WitnessCellCapExceeded { segments, cells }`, and `topo::WITNESS_BUDGET` / `WitnessBudget` into `topo::WITNESS_SEGMENT_CAP` / `WITNESS_CELL_CAP`. Every census and validate mapping keeps one answer for both faces. A match in your files names both. (PIPE implementer, witness-cap lane)
- 2026-10-10 — Seam note from PIPE S350 (PR 4482): `census::face_reach_in`'s `FaceBoxRule::ControlNet` arm now matches `NurbsSurface::net_state()` and answers `None` for `Poisoned` as well as `Placeholder` (a described net poisoned in one channel no longer folds to a box finite on the other axes). Arm 1 refuses such a face's pairs as `NoSoundReach`; arm 2 never takes its solid as the container. Test-module change: `n2r2_class7_…` is replaced by `a_net_poisoned_in_one_channel_has_no_reach_and_clears_no_pair`. (PIPE S350 lane)
- 2026-10-10 — Seam note from ENCL: dispatching `encl/topo-poisoned-endings`. It touches only the endings of poisoned or contradicted arms on `PointInSolidError::Escalated` (solid_contain), `SplitReduceError::SliverSector` (splitting/rules :319/:488/:500), `CensusEscalated` (census :2583) and possibly ChartRegion/Region/Section/Contain escalations. No mint moves; the readers compose their own endings, per fork-log row 9. (ENCL orchestrator)
- 2026-10-10 — Seam note from ENCL (PR 4474, merged): the material pairing is a decision. `geom_brep::MATERIAL_PAIRING` and `MATERIAL_PAIRING_CLAUSE` end it at tier 3 (`WedgeCheck::MaterialPairing`) and split finish (`SplitFinishError::DescribeSideEscalated`), with a true tolerance offer. Where the wedge would re-decide below m/K, `pairing_at_wedge` quotes the wedge and offers w/K, as `at_wedge` does. `MaterialStations`' `Break` is `MaterialStop { check, cause }`. The cusp-side Zero reads via `decide_nonzero`. New k_stats predicates appear on the refusal path only: "material_pairing_offer_wedge" and "material_pairing_wedge". (ENCL orchestrator)
- 2026-10-10 — Seam note from ENCL (PR 4497, merged at `4c7fb9b0fc`). A poisoned or contradicted margin now ends in a followable recourse:
  - at rest, the census coincidence menu (`validate::too_close` → `own_close`) gives the defect ending;
  - `ChartRegionError::Escalated`'s poisoned arm gives `defect_ending(Build)`;
  - `PointInSolidError::Escalated` gives the unnamed placement lever plus the reading's note, as `contfp`, `classify_point_in_solid` and the Boolean already did.

  `SliverSector` is unchanged, and its mints are on CLEAVE's split-escalations row. (ENCL orchestrator)
