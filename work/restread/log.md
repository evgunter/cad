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
- 2026-10-10 — Seam note from PIPE S350 (PR 4482): `census::face_reach_in`'s `FaceBoxRule::ControlNet` arm now matches `NurbsSurface::net_state()` and answers `None` for `Poisoned` as well as `Placeholder` (a described net poisoned in one channel no longer folds to a box finite on the other axes). Arm 1 refuses such a face's pairs as `NoSoundReach`; arm 2 never takes its solid as the container. Test-module change: `n2r2_class7_…` is replaced by `a_net_poisoned_in_one_channel_has_no_reach_and_clears_no_pair`. (PIPE S350 lane)
