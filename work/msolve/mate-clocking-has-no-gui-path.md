---
id: mate-clocking-has-no-gui-path
kind: issue
title: Mate clocking through the GUI: the tool admits a rider the coset table statically refuses, and no affordance can turn a mate's roll
status: closed
opened: 2026-09-01
github: 1461
blocked_on: [MSOLVE-10]
closed: 2026-09-20
pr: 2913
---

## From GitHub issue 1461

Opened 2026-09-01; 0 comments.

Found by the `story_assembly` integration lane landing two windmill sails crossed. Two halves of one UX hole:

**1. The tool commits an edit the solve is statically certain to refuse.** Repro: two instances, two mate-tool picks, `proposal(..., MateChoice { Rest, FrameCoincidence, Opposed, clocking: Some(FRAC_PI_2) })` → proposal Ok, `perform(proposal.op())` commits with no refusal — and the *next evaluation* fails with "the mate solve refused: … predicate `mate_clocking_redundant`". The coset table decides FrameCoincidence + nonzero clocking contradictory **statically** (`solve.rs:487–504`, no geometry consulted), so the tool or the `AddMate` door could refuse typed at authoring time; instead a poisoned edit enters the history and the user meets the failure as a tree badge one step later.

**2. There is no working way to clock.** With the rider refused for frame coincidence, turning a mate's roll means hand-deriving the alignment's roll reference: `face_frame` roll references across a box's opposite walls carry no documented relation, so the lane had to measure the first blade's solved direction and choose between the derived reference and its in-plane quarter turn. That is guesswork where "rotate this mate 90°" is the everyday intent. Either a rotate-mate affordance or documented reference conventions would close it.

The suite's workaround (turning the alignment's roll reference and committing through `AddMate` directly) is recorded in `crates/viewer/tests/story_assembly.rs`.

(story-suites orchestrator)

## Home

`work/mate/` — the static coset-table refusal is `crates/editor-core/src/mate/solve.rs`, inside S-MATE's territory glob `crates/editor-core/src/mate/*`, and mate authoring is the program's charter (the viewer half rides along).

## Re-homed from FIX to DOCM, 2026-09-12 (FIX orchestrator)

**The `## Home` section above names `work/mate/`, a directory that no
longer exists.** S-MATE left the tracker on 2026-09-04
(`docs/DOC-LEDGER.md` sweep 6). Its territory was not freed but
**inherited**: `work/docm/program.md` and `work/msolve/program.md`
both carry `crates/editor-core/src/mate/*` in `paths`.

`work/fix/plan.md` has held **half (1)** — refuse a nonzero clocking
rider on `FrameCoincidence` at `AddMate` — on FIX's slate since the
program opened, on a fence that has since been claimed twice, while
`work/docm/plan.md:94-96` already carries **half (2)** (a rotate-mate
affordance or documented roll conventions) as DOCM's. One item cannot
live on two slates, and splitting it would need two files for one
finding.

**DOCM takes both halves**, on the charters as written: half (1) is an
`AddMate` door refusing at authoring time, and the `DocEdit` set is
DOCM's charter explicitly; half (2) was already DOCM's. DOCM's own
charter is to open one `[ev]` PR per question and **hand each build to
FIX, CHROME or VIEW when ruled** — so if the ruling on half (1) is
"refuse typed at the door", handing that build back to FIX is the
expected flow and FIX will take it. The sibling row
`levered-clash-margins-hide-their-arm` went to MSOLVE by the same
reading, since it is solve semantics rather than document custody.

FIX took nothing here and changed no code; this is a routing move
only. `work/fix/plan.md`'s half-(1) line is corrected in the same PR.

## Re-homed (2026-09-13)

Moved from `work/docm/` to `work/msolve/` at DOCM's exit sweep (`docs/DOC-LEDGER.md`,
sweep 14): mate authoring is MSOLVE's (`mate.rs`, `mate/*`); the viewer half rides along as CHROME's announced seam. Id, body and header are unchanged; the directory is the
claim (`work/README.md`). Any `## Home` section above is superseded by
this line and is kept as the record of why the file was where it was.

## Closed (2026-09-20, PR 2913)

MSOLVE-10 (`docs/MSOLVE-10-SPEC.md`).

**Half (1) — built.** The edit door asks the solve's own per-mate
admission (`admit_mate`, `crates/editor-core/src/mate/solve.rs`) of a
mate being inserted and refuses `EditError::MateRefused` carrying the
solve's fault unaltered, so a mate the coset table refuses on its own
datum is refused at the insert door: the walk's head, one member twice,
the class, each frame, the table's static gaps, the clocking rider on
a coincidence decided over the mate's own lever through the reach the
door holds. The viewer tool refuses the static gaps before any
geometry (`MateToolError::TableRefused`); the decided rider is met at
`perform`, typed. The doors decide edits and the solve decides
states: a mate that comes to carry such a fault after insert (a
stranded head, a re-pointed `Part`, a loaded snapshot) is the
solve's at evaluation. The rows: `crates/editor-core/tests/
msolve10_door_admission.rs` (A1, A2 over a corpus, A4's counting
reach, the history, replay), `crates/viewer/tests/story_assembly.rs`
stage 11a and `mate_tool_flow.rs`, and
`crates/pncad-py/tests/test_assembly_author.py`'s
`test_a_rider_the_table_decides_against_is_refused_at_the_door`.
The item's measurement — `proposal(…, FrameCoincidence, clocking:
Some(π/2))` → `perform` commits and the next evaluation fails — now
reads: `perform` refuses typed, nothing commits, nothing fails.

**Half (2) — recorded, not built.** The spelling that exists for
turning a mate's roll is a coaxial mate with a clocking rider (the
table's coaxial+clocking row), which `MateChoice` already offers; a
roll-reference convention for authored frames is MSOLVE-9's
`reference` rule; a rotate-mate affordance is CHROME's viewer seam,
handed there when MSOLVE-9's convention is ratified. The story
suite's recovery (turning the roll reference and committing through
`AddMate`, stage 10) stays the record of that cost.
