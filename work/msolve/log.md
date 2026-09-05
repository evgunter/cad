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
