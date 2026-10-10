---
id: too-close-to-call-remainder
kind: issue
title: the 'too close to call' sentences outside the folded family still spell the undecided verdict their own way
status: open
opened: 2026-10-09
---


(ENCL implementer, from `not-yet-note-separator-and-too-close-family`.)

## What

That row folded the "too close to call at this tolerance" family in
`geom_brep::props` (`PropsError::Escalated`), `topo::validate` and
`topo::census` into the one verdict word, `undecided`:
`Indeterminate::undecided` where the sentence carries the payload,
`geom_core::undecided!` where it does not. `git log -S` found no
ratification of either word (neither is in `docs/DESIGN.md` nor a
crate README design page), and `validate.rs`'s `certify_undecided`
already used "is undecided" for one arm as a word-budget synonym.

The same verdict is still spelled "is too close to call" by hand at
these sources (at `origin/main` 9409cb5e43; doc comments omitted):

- `crates/geom-brep/src/`: `edge_nurbs.rs` (plane × NURBS interior
  sample), `newell.rs` (vertex on the loop's plane), `nurbs_iso.rs`
  (edge on the fitted surface's boundary), `offset.rs` (valid radius),
  `offset_meters.rs` (`MeterError::Escalated`), `ssi.rs` (two
  `SsiError` arms, and the in-file test at `ends_with("too close to
  call: …")`).
- `crates/topo/src/`: `boolean/solid_contain.rs`, `chart_region.rs`,
  `chord_join.rs` (three), `props.rs` (sign of a shell's volume),
  `replace_face.rs`, `shell.rs`, `splitting/finish.rs` (three, with the
  payload in parentheses), `splitting/mod.rs`, `splitting/section.rs`;
  and `validate.rs`'s `ContactRefusal::Escalated` reason, left out
  because contact ground is held under D10.
- `crates/sweep/src/`: `extrude.rs` (two), `loft.rs` (two),
  `revolve/mod.rs` (four), `revolve/tube.rs` (two).
- `crates/profile/src/`: `path.rs` (three), `validate.rs`
  (`validation escalated {site}: …`).
- `crates/editor-core/src/`: `eval/mod.rs`, `names/emit.rs`; and
  `mate.rs` `MateError::Indeterminate` ("a case split could not be
  decided — {}. Recourse: {}"), which joins the family once the D10
  hold on placement ground lifts.
- Tests asserting the words: `test-utils/src/refusal.rs` (fixtures and
  `DECISION_PHRASES`, which must keep `"too close to call"` until the
  last site goes), `editor-core/tests/{display_contract,
  refusal_concision_chains, pattern_spacing_and_step}.rs`,
  `profile/tests/recourse_roster.rs`,
  `sweep/tests/review_recourse_roster_r2_probes.rs`,
  `topo/tests/{review_s6_probe, review_ssiflat_r1_probes}.rs`.

Most `"{what} is too close to call: {source}"` sites render an
`Indeterminate` whole, which is `{payload}. {ending}`: the
`Indeterminate::undecided` shape with the escalation's own ending.

## Repair shape

Route each through `Indeterminate::undecided` (payload in the
sentence) or `geom_core::undecided!` (static clause), one crate per
PR, re-baselining the texts each moves; drop `"too close to call"`
from `DECISION_PHRASES` with the last one.

## Also in this family (from the review of PR 4443)

- **Decision-specific "at this tolerance" verdicts.** `topo::validate`
  `certify_undecided`'s `ParamSpan` ("its length is too close to zero
  to decide at this tolerance"), `Transversality` ("its faces meet too
  nearly tangentially to decide at this tolerance") and
  `TangentSecondOrder | TangentTube` ("… curve apart too little to
  decide where it runs at this tolerance"), and `topo::census`
  `Undecided::TouchPieceInBand` ("… too nearly in line to trace at this
  tolerance"); and `topo::validate`'s `"{} is undecided at this
  tolerance"` (`close_to_boundary`), the folded word still carrying
  the qualifier. Same argument as the fold: D4 ¶1 (i) puts the tolerance
  offer in the ending, conditional and valued where a smaller ε decides
  the margin, and on a poisoned margin "at this tolerance" is false
  where "undecided" is true.
- **`topo::census` `Undecided`'s qualified not-yet endings.** Its
  Display spells `NOT_YET_ENDING` by hand, qualified, at about eight
  arms: "There is no way through yet for this shape", "… for a designed
  resting contact", "… for this declared contact", "… for a designed
  resting fit". The arms are `&'static str`, which a `const` cannot be
  `concat!`ed into, so `NOT_YET_ENDING` needs a hidden literal macro
  beside `kernel_defect_ending!` (the constant defined through it). The
  tail "for a designed resting contact; otherwise move them until their
  bounding boxes no longer overlap" is written four times there and
  wants one spelling too.
