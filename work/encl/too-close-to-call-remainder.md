---
id: too-close-to-call-remainder
kind: issue
title: the 'too close to call' sentences outside the folded family still spell the undecided verdict their own way
status: closed
branch: encl/undecided-sweep
opened: 2026-10-09
closed: 2026-10-13
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
`{payload} — {reading}; {ending}` (the bare `Indeterminate` Display is
`under(COINCIDENCE_RECOURSE)`), and not the `Indeterminate::undecided`
shape, `{payload}. {ending}`. Both end in `Indeterminate::ending`,
per-kind levers included, so they differ only in the joint and the
reading clause.

**Done: the geom-brep slice, in PR 4612.** Its seven sentences spell the
verdict through `geom_core::undecided!`. Four Displays carry the payload
only (`SsiError`'s two arms, `MeterError::Escalated`,
`PlaneNurbsRefusal::TransversalityEscalated`), and their ending is the
caller's, per `Reading`. Three render the whole escalation
(`NewellError::Escalated` through the bare `Indeterminate` Display,
`IsoRowError::Escalated` and `OffsetError::Escalated` through
`under(NO_DECLARATION_RECOURSE)`), so their tail is `IndeterminateUnder`'s
` — {advice}; {ending}`, per-kind levers included.

**Done: the topo slice, in PR 4745** (fourteen sentences).

- Skipped, contact ground under D10: `chart_region.rs` `Escalated`,
  `validate.rs` `ContactRefusal::Escalated`.
- Left: hand-spelled `is undecided` in `boolean/edge_join.rs`
  (`JoinUndecided`, `JoinRefusal`), `validate.rs`
  (`JoinUndecidedAtRest`) and `chord_join.rs` (`Section`'s carrier
  arm); the decision-specific verdicts in `splitting/mod.rs`
  `SectionGermsEscalated` ("too nearly in the split plane", "too close
  to order"); `splitting/section.rs` `UnplacedHole`'s "a contact too
  close to call", a cause in a list that still trips
  `DECISION_PHRASES`; the payload joint spelled by hand in geom-brep
  (`edge_nurbs.rs`, `offset_meters.rs`, `ssi.rs` ×2), which
  `geom_core::undecided!(subject, payload)` now spells.
- `SliverSector` shares the joint through that macro but not
  `UndecidedRefusal`, which needs an `Indeterminate`: two of
  `SectorReading`'s arms are decided disagreements carrying margins.

**Done: the profile slice, in PR 4780** (four sentences: `path.rs`'s
junction arm and both fall-through arms, `validate.rs`'s
`ProfileError::Escalated`, now "{site}, {subject} is undecided: …").
Both doors end a name by its row in `validate::DECISION_ENDINGS`
(geom-core's `EndingKind::Sized` or `Lever(lever)`), one row per decided
name, held both ways against `src`; a name with no row names its hole at
both. The ending is `Indeterminate::ending_by` (PR 5003).

- Skipped: nothing on D10 ground in the slice.
- Left: the hand-spelled decision-specific verdicts in `path.rs`'s
  `Escalated` (`path_continuation_target_offset` and the seam arrival,
  "neither … nor definitely off it"; `path_leg_length`, "could not be
  told from zero"), each with a hand-written ending;
  `FILLET_ENCLOSING_RECOURSE` restating the verdict ("too close to … to
  tell whether") after "the fillet at this corner is undecided"; the
  fillet and stored-form arms' `lever_recourse(.., None)`, which drops
  the unreadable-margin note on poison; `structure.rs`'s
  `StructureRefusal` bare `{source}` (owned by
  `work/profilerest/paths-refusals-short-of-the-shape-guard.md`); the seam
  arrival and continuation arms' "widen the input tolerance (K·ε)"
  against their rows' kinds.

**Done: the editor-core slice, in PR 4811.** `NodeErrorKind::Escalated`,
`NamingError::Escalated` and `SelectRefusal::InBand` render through
`Indeterminate::undecided`. Their ending comes from `decision::Decision`,
a closed enum declared from one table of name and kind. The evaluation's
escalations are built from a `Decision`, and a name with no row ends
sized plus `geom_core::MissingRecourse`. Lever: `eval_direction_norm` (a
gauge), the angles, and `name_frag_order_along` (a curve parameter).
Sized: the pattern spacing, naming's three distances and
`sel_datum_distance`. The ending is `Indeterminate::ending_by` over the
decision's `geom_core::EndingKind` (PR 5003).

- Skipped, held under D10. Each keeps its old sentence through a held
  arm at its door, pinned through that door:
  - measure: `NodeErrorKind::MeasureEscalated`, for distance and gap on
    `bool_plane_parallel` and `carrier_cyl_axis_parallel`; also
    `measure.rs`'s assertion verdict;
  - placement: `NodeErrorKind::PlacementAxisEscalated`, which
    `EditError::PlacementAxis` carries;
  - pose and datum: `NodeErrorKind::PoseDirectionEscalated`, covering
    `DirectionRefusal::node_error` (datum axes, normals and frames, and
    the sketch `FrameDirection` that carries it) and `wire/pose.rs`'s
    `degenerate`. Until the hold lifts, `datum_unit_norm` and
    `eval_direction_norm`, the same arithmetic, end two ways: held at
    the pose ground, the lever alone at the evaluation;
  - `mate.rs`: `MateError::Indeterminate`, and `MateCheck`'s "could not
    be decided".
- Left:
  - `checks.rs` `CheckEvidence::Escalated` ("the component count is
    unknowable: {shell payload}"), a consequence ahead of the question;
  - the placement route compares the user-facing role word
    (`role == PLACEMENT_AXIS_ROLE` in `eval/wire.rs` `unit`), and becomes
    a typed route from `placement.rs` when D10 lifts;
  - the naming door is fed `&str` through `emit_topo`'s `Segment`
    methods, so its census reads the naming word table: a new unworded
    naming predicate renders the missing-recourse note, but nothing goes
    red;
  - `placement.rs`'s `# Errors` doc for `Frame::rotate_then_translate`
    still names `NodeErrorKind::Escalated`, in a held file;
  - `DECISION_PHRASES` keeps "too close to call" for the held sites.

**Done: the sweep slice, in PR 4766** (eleven sentences: the ten
above and `CapPlaneError::Orientation`, which keeps its defect
ending). Each renders through `Indeterminate::undecided`, with the
lever `COINCIDENCE_RECOURSE`.

- `ending(..)`: extrude's normal and obliquity, loft's stacking, the
  clearance's arc apex and arc clearance, the tube's wall, bore and gap.
- `Indeterminate::lever_ending`: the revolve angle and headroom, the axis
  direction's length (a gauge), both cosurface decisions, and the
  clearance's line runs, arc centre and arc span.
- `RevolveError::SliverAxisClearance` and `TubeError::Escalated` carry
  closed decisions (`ClearanceDecision`, `TubeDecision`) that name
  their predicate and their kind.
- Skipped, carvetail's (`carve-refusals-short-of-the-shape-guard`):
  `swept::sliver_text`'s `Lever` arm, and `RevolveError::SliverRadius`,
  an escalation rendered as "sits a sliver away from the revolve axis:
  {source}".
- Left: a sized straddle reads as a subdividable enclosure through
  `ending(..)`, as on the cleave row.

## Repair shape

One crate per PR, re-baselining the texts each moves; drop
`"too close to call"` from `DECISION_PHRASES` with the last one. The
rule for each site:

- **A payload-only Display** (the sentence carries `payload()` and the
  ending is someone else's) writes its subject through
  `geom_core::undecided!`.
- **A whole-escalation render** goes through `Indeterminate::undecided`
  with `ending(<recourse>)`, the recourse picked from the door's
  callers — or with the ending its decision's kind warrants: a
  margin that is no size the user chose ends through `LeverOnly`, and
  an escalation that carries its decision ends as that decision does
  at that door. A sized decision another crate also renders keeps
  `ending(..)`, so it has one ending (D4 ¶1).

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
