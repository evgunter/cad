---
id: too-close-to-call-remainder
kind: issue
title: the 'too close to call' sentences outside the folded family still spell the undecided verdict their own way
status: dispatched
branch: encl/undecided-geom-brep
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
  `splitting/containment.rs` (`PointInLoopError::RayExhausted` and
  `Uncrossable`) and `chord_join.rs` (`RingHoming`'s `RayExhausted`
  arm), which already say "is undecided" but spell it by hand as
  `"… is undecided: {}"` rather than through `geom_core::undecided!`;
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
`Indeterminate` whole. That render is `IndeterminateUnder`'s,
`{payload} — {advice}; {ending}` (the bare `Indeterminate` Display is
`under(COINCIDENCE_RECOURSE)`, or `under(NO_DECLARATION_RECOURSE)` on a
margin decided past the band), and not the `Indeterminate::undecided`
shape, `{payload}. {ending}`. The two differ in more than the joint:
on an enclosure that straddles the band `under` puts "subdivide the
parameter box for a tighter enclosure" in front of the door's levers,
and on an unreadable margin "check the operation's inputs upstream";
`Indeterminate::ending` carries neither.

**Done: the geom-brep slice, in PR 4612.** Its seven sentences spell the
verdict through `geom_core::undecided!`. Four Displays carry the payload
only (`SsiError`'s two arms, `MeterError::Escalated`,
`PlaneNurbsRefusal::TransversalityEscalated`), and their ending is the
caller's, per `Reading`. Three render the whole escalation
(`NewellError::Escalated` through the bare `Indeterminate` Display,
`IsoRowError::Escalated` and `OffsetError::Escalated` through
`under(NO_DECLARATION_RECOURSE)`), so their tail is `IndeterminateUnder`'s
` — {advice}; {ending}`, per-kind levers included.

**Done: the topo slice, in PR 4745.** Its fourteen sentences spell
the verdict through the rule: `props.rs`, `splitting/mod.rs`
(`SliverSector`), `splitting/containment.rs` (two) and `chord_join.rs`
(`RayExhausted`) through `geom_core::undecided!`; `splitting/finish.rs`
(three), `splitting/section.rs`, `shell.rs`, `replace_face.rs` and
`chord_join.rs` (`OrderEscalated`, `Escalated`, `RingHoming`'s
`Escalated`) through `Indeterminate::undecided` with `ending(..)`.
Left on contact ground under D10: `chart_region.rs`'s `Escalated` arm
(the face–face record's overlap predicate) and `validate.rs`'s
`ContactRefusal::Escalated`. `boolean/solid_contain.rs` holds only a
doc comment. Still hand-spelled in topo, as `"{subject} is undecided
({payload}). {ending}"` or `"… is undecided: {diag}"`:
`boolean/edge_join.rs` (`JoinUndecided`, which renders the bare
`Indeterminate` Display, and `JoinRefusal`'s `undecided` closure),
`validate.rs` (`JoinUndecidedAtRest`), `chord_join.rs` (`Section`'s
carrier escalation, "… is undecided for the section through a curved
face"); and "too close to order" (`splitting/mod.rs`
`SectionGermsEscalated`), a decision-specific verdict like those below.

## Repair shape

One crate per PR, re-baselining the texts each moves; drop
`"too close to call"` from `DECISION_PHRASES` with the last one. The
rule for each site:

- **A payload-only Display** (the sentence carries `payload()` and the
  ending is someone else's) writes its subject through
  `geom_core::undecided!`.
- **A whole-escalation render** goes through `Indeterminate::undecided`
  with `ending(<recourse>)`, the recourse picked from the door's
  callers (`NO_DECLARATION_RECOURSE` where none takes a declaration).
  Where the site renders through `under(..)` today, `ending(..)` drops
  the per-kind levers above, so it moves only with an ending that
  carries them.

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
