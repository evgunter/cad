---
id: an-aborting-touch-waits-in-the-backstop-like-an-undecided-one
kind: issue
title: The cross-solid backstop waits on a touch whose reading aborts (Unreadable, OffInterior) as on an undecided one
status: closed
opened: 2026-11-09
closed: 2026-11-10
---


Found while carrying the touch's reason into the backstop (`onecopy/carrier-standing-2`, row `the-typed-refusal-reason-leaves-an-unused-second-spelling-and-unpinned-reads` item 11). The orchestrator ruled: file it, don't decide it there.

`link::TouchUnread::refusal` names two of its reasons as aborts, refusals of the whole query:
- `Unreadable`, a germ no later face rescues;
- `OffInterior`, two readings of the touch that disagree, a kernel defect.

The census's cross-solid backstop (`census.rs`, the `blocks` reading over `Undecided::is_undecided_touch`) still waits on both as undecided touches. A decided reason found later takes the finding's place, and the abort is reported only where nothing else is. `Corrupt`, the third abort, does not wait: it goes to `Undecided::CorruptInstance`.

So the backstop and the ladder disagree on what an abort is. To the ladder it stops the reading now. To the backstop it is one more undecided touch, and a decided touch can hide it, a kernel defect included.

**The question.** Should a touch whose reading aborts stop waiting, reported at once as the backstop's own refusal (as `Corrupt` is), or wait as it does?
- `Unreadable` is the common case: a curved face or edge at the touch.
- Changing it moves what the backstop reports wherever a curved touch and a decided one meet on one pair.

This is cross-cutting: it touches every abort the backstop meets, not one reason. It needs a ruling before code.

## Closed (2026-11-10, `onecopy/backstop-abort`): ruled "at once"

The orchestrator ruled one meaning of an abort, the ladder's: the first touch whose reading aborts ends the pair's reading and is reported as the backstop's finding. A later decided crossing does not hide it. "As `Corrupt` is" was the other candidate, and it would leave that hole open for a defect read (`OffInterior`, a contradicted `EdgeTangent`) reaching the backstop.

**The change.**
- `census.rs`'s `blocks` now hands its readings, in scan order and read lazily, to one pure function, `blocked`, through `pair_blocked`: the pair's findings first (each scanned by `between_scan`), then its recorded and shared-key sites.
- `blocked` returns the first crossing or the first abort, whichever is scanned first, at once. Failing both, it returns the first other reason ahead of the first waiting touch.
- **The standing** is read off the touch's cause, `TouchUnread::refusal()`, before `TouchUnread::undecided` names it as the backstop's reason (`Unrest::Aborts`, `Scan::Aborts`). So `Corrupt`, named `CorruptInstance`, aborts too, and a future abort cause joins without a list. A crossing read with no point, a kernel defect, aborts as well.
- **Stand-ins wait** (the orchestrator's F1 ruling in PR 6757's review). A curved touch the curved arm found (`Said::CurvedTouch`) and an unconfirmed face pair (`Said::FacePair`) stand for a reading no analysis here makes; they are not failed reads. They used to map to `Touch(Curved(Unreadable))`, which aborts. That was a mislabel: an abort is a reading no later one rescues, and a decided crossing at another site of the pair does rescue these. They now read `Undecided::CurvedTouch`, which waits like an undecided touch and renders the curved reading's sentence. The real faults (`OffInterior`, `NormalFault`, a contradicted `EdgeTangent`, `Corrupt`) still abort at once.

**Measured before building** (the topo, sweep and editor-core `ci` sets, both ε):
- 123 backstop readings per ε met a touch then read as aborting, all `Curved(Unreadable)`.
- None ended in a crossing or `Stands`. Each lost to another undecided reason (`Unexamined` 121, `LoneLoop` 2).

**Measured after F1** with a probe in `blocked` over the same sets at default ε and 1e-12, which gave the same counts at both:
- 124 readings end at an abort: topo 58, sweep 55, editor-core 11. These are the 123 above plus this PR's own rows. All are site-read touches; no suite reading reaches `blocked` with a stand-in finding.
- None has a crossing scanned after it, so no verdict moves. The only aborts ahead of a crossing, the only `CorruptInstance` abort and every stand-in are in `census::blocked_rows`, which plant exactly those cases.
- The sweep rows behind `common/touch_union.rs`'s `only_curved_within_reach` each see the curved touch as the solid pair's finding, never `Unexamined`. A probe of all 5 call sites, the three in `bisector_nodes.rs` included, showed this at both ε. So the helper now requires that reason exactly, and a precedence revert reds all 8 rows (`two_discs_kissing_externally_touch_at_every_op`, `externally_tangent_walls_touch_at_every_op`, `a_same_side_tangent_touch_answers_every_op`, `r2_every_same_side_touch_pose_answers_as_a_touch`, and bisector_nodes' `a_rod_against_a_bore_builds_every_operation`, `equal_cylinders_with_meeting_axes_build_every_operation_at_the_closed_forms`, `the_family_re_posed_builds_the_same`, `slanted_bores_build_every_operation_one_difference_pinched_to_itself`). It was measured red on all 8 with the reason check reverted to `Unexamined`.

**Pinned** in `census::blocked_rows`:
- `an_aborting_touch_is_reported_ahead_of_a_later_crossing`: an abort scanned before a decided crossing reports the abort, and it is red under the old rule.
- `a_touch_aborts_by_its_causes_standing`: every cause, by its own `refusal()`, with `Corrupt` aborting under `CorruptInstance`.
- `a_curved_stand_in_waits_for_a_crossing_at_another_site`: either stand-in as a finding ahead of a crossing at a site reads the crossing, which `blocks`' caller reports as `TouchInterference`.
- `a_pairs_findings_are_scanned_before_its_sites`: the scan order `pair_blocked` fixes.
