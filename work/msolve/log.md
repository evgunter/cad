# MSOLVE log

Newest entries at the bottom; the tail is the program's live status.
Plan: `work/msolve/plan.md`.

## Opened (2026-09-04)

Opened by the FIX orchestrator on Ev's steer (in-chat, PR 1731 thread:
DOCM "feels somewhat different — it's ok to open a successor unit to
S-MATE if that's what makes sense").

The five items below were measured or ruled during FIX's run and have
no owner: S-MATE closed while they were in flight, and DOCM inherited
the FILES rather than this class of question. Re-homed here by header
edit and `git mv`, ids unchanged.

Two things this program starts with that most do not:

- **A live defect with characterization rows already on main.** PR 1773
  pins the transform-blind solve as a known-wrong answer, with a header
  saying the fix DELETES the rows rather than updating them. They go red
  when item (1) is fixed; that is the signal, not a regression.
- **A ruling already made.** Ev ruled item (2) in on PR 1731, and the
  sequencing — the gate first — with it.

The territory overlap with DOCM is announced on their orchestrator PR,
not assumed. Nothing has been taken from them.

## Ruled and cut (2026-09-05)

Orchestrator's first session. Ev's rulings, in chat:

- **The gate's fix shape.** The plan's "`derived_offset` sibling that
  walks the input chain" is well-defined only for a pattern head; a
  bare transform is invisible in a mate's data (N1: no segment, so a
  mate through a transform is byte-identical to a mate on the
  instance). Three shapes weighed: (a) a transform mints a segment —
  rejected, it changes what a name IS and renames every entity
  downstream of every transform, in persisted documents too, against
  what `emit_union` and the measure door were built on; (b) the mate
  stores the node each side is read at, the measurement reference's
  shape — **ruled in**; (c) refuse a mate whose instance has a placing
  consumer until (b) lands — not needed once (b) is the unit. Ev's
  framing that decided it: a transform is represented only as a DAG
  parent of the thing it transforms; the operand IS that, provided the
  edge is A12's reading kind, not consuming (A10's roots).
- **Territory:** touch whatever, resolve conflicts as they come.
- **The first `[ev]` question** was already answered by
  `ASSEMBLY.md`'s A11 (5); withdrawn, no PR.
- The `DanglingHead` catch-all ruled in by this program (S-MATE's
  successor) as `MSOLVE-3`; AQ8's SKIP half stays Ev's, a short `[ev]`
  PR to come.

Cut: `MSOLVE-1` (spec `docs/MSOLVE-1-SPEC.md`), `MSOLVE-2` parked on
it, `MSOLVE-3` open; the three issues they answer parked on them. Two
items re-homed here by the 2026-09-04 sweep read and placed on the
slate (the memo key: a unit after 1; the lever's extent: an `[ev]`
question). Next: dispatch MSOLVE-1 on `msolve/1-mate-operand`.

## AQ8's SKIP half homed (2026-09-05)

Not an `[ev]` after all: PR 592's addendum comment carries Ev's 👍
ratifying option (b). The clause joins the weld half in `ASSEMBLY.md`;
`aq8-skip-half-is-cited-as-ratified-and-is-not` closed. Orchestrator
PR 1913 (spec and cut) merged.

## MSOLVE-1 dispatched, landed, in review (2026-09-05)

Dispatched on `msolve/1-mate-operand` against `docs/MSOLVE-1-SPEC.md`.
The lane took the two item-7 measurements first: the blindness was
never class-dependent (a residual tree edge refuses `Under` before any
pose exists, with or without the transform; only a DETERMINING fold
showed it), and it covers rotation (an x-π/6 transform tilts the placed
block, `Opposed` broken in the product, nothing refused). PR 1929
green on the full matrix at `03d812228`; the reference type is
`SitedRef` (`EntityRef` was taken by N4's arena key); one row's
expectation changed by the PR 1731 ruling (pattern-of-transform now
places). Style review and correctness arm dispatched on that head.
Findings reported outside the fence, placed at state-sync: `Node::Part
{ Instance(i) }` is a third identity-transparent node the walk refuses
(MSOLVE-2's decision); `Frame::rotate_then_translate` normalizes with a
bare `.normalize()` (filed here).

## MSOLVE-1 reviews adjudicated, fix pass dispatched (2026-09-05)

Style review: twenty findings, Q1–Q8 all exercised. Correctness arm:
APPROVE-WITH-FIXES, C1–C6 confirmed on documents of its own (non-axis
rotations, a gauge-side chain, a placed gauge frame with transforms on
both sides, the viewer end to end). One MAJOR, the suite's own: the
acceptance fixture's seat was an interpenetration (`b`'s axis on the
wrong side of the cap), so the gate refused the control too and A5's
"inconsistent pair refuses" was vacuous; the claim itself is true on a
physical seat. Ruled in the fix pass: the split door refuses TYPED, in
both directions, when a mate and its operand land on opposite sides of
a cut (the reading-edge twin of D-2's `SeveredEdge`; the kept-mate
direction was silently keeping a stale operand, the cut-mate direction
was refusing in the input's vocabulary); `DanglingHead.head` names the
node the walk stopped at, not a live instance; the product oracle pins
frame origins (rotation about the seat normal was invisible); a
non-commuting chain row; the viewer row seats a moved instance after
commit; doc rot from the rename swept. Carried to MSOLVE-2: the
`Placer`/`copy` duplication, the triple walk, splitting
`mate/member.rs`. Filed: the gate's `Vanished` on a mate read below a
pattern (here); an instance under two placing roots refusing `Naming`
at the gather, and `Transform` refusing a pattern's `Instances`
(`work/issues/`, no obvious owner).

## MSOLVE-1 MERGED (2026-09-05, PR 1929)

Fix pass green on the full matrix at `550a9f2`; merged without a
fresh run on the state-sync commit (docs and tracker only). What the
fix pass added beyond the reviews' letter: `SplitError::
OperandSeveredFromMate` runs after the cluster precondition and exempts
the interface crossing (a kept at-mint mate whose name lies wholly in
the cut re-anchors through the minted instance); a non-exhaustive
Python `split_err` match the new variant exposed. Item closed, spec
deleted into the ledger, `mate-solve-is-transform-blind` closed;
MSOLVE-2 and MSOLVE-4 un-parked. Next: dispatch MSOLVE-4 (spec on
main), write MSOLVE-2's spec against the walk as landed.

## MSOLVE-4 landed, in review (2026-09-05)

PR 1960 green on the full matrix at `b4764ea`. The spec's premise
verified on the tree: the memo has one reuse site and it matches only
`NodeResult::Ok`, so the fault's content does not feed the key. One
CHROME row's PREMISE was rewritten (it asserted the memo hazard as
its precondition and said so); the guard in `tree.rs` is gone with no
row needing it. Style review and correctness arm dispatched on that
head.
