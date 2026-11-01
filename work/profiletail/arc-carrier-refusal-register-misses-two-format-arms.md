---
id: arc-carrier-refusal-register-misses-two-format-arms
kind: issue
title: PATHS-DESIGN's typed-runtime-refusal register omits both format arms of the 2-D director doors
status: open
opened: 2026-09-12
refs: [2359, 2415]
priority: P4
cost: E
design: true
---

Disclosed by the `fix/arc-fillet-underflow` lane (PR 2415) while
fitting the underflow gate at `arc_fillet::carrier_tangent`, and
filed at the moment of disclosure rather than left in that PR's body.

`docs/PATHS-DESIGN.md`'s **Refusals** paragraph (the "Typed runtime
errors, from geometry" register, around `docs/PATHS-DESIGN.md:903-940`)
enumerates the `PathError` arms the §2a/§2c surface can raise —
`NonpositiveCircleRadius`, `ZeroDirection`, `ArcViaCollinear`,
`DegenerateArcChord`, `DegenerateArcSpec`, `ArcCenterNotEquidistant`,
`DegenerateArcCenter`, `FarEndAnchorWithoutFillet`,
`CircleSplitCount`, the two `ArcContinue*`, the two `SeamArrival*` —
and names neither of the two arms that refuse a length the FORMAT
lost:

- **`PathError::NonFiniteDirection`**, landed by PR 2359, which the
  register has never named;
- **`PathError::UnderflowedDirection`**, landed by PR 2415.

Both are raised from inside this document's own surface — the
components director (`PartialPath::toward`) and the arc carrier's
tangent — so the omission is not a scope question. The register is
prose with no check under it, which is exactly why two arms could
land without it noticing: nothing in CI compares that list against
`PathErrorKind`.

**Two things a taker decides**, and they are separable:

1. The cheap half: add the two arms to the register with the
   sentence that distinguishes them from `ZeroDirection` (not a
   coincidence at any tolerance; the recourse is scale, not ε).
2. The half worth arguing: whether that register should be checked at
   all. `PathErrorKind` is a fieldless mirror of `PathError` and the
   `pncad-py` `TAG_INVENTORY` already pins the full arm set at the
   Python door, so a register that goes stale silently while a
   neighbouring list cannot is a shape worth naming — or the register
   is prose by design and should say so, so the next lane does not
   read its completeness as a claim.

**Fence.** `docs/PATHS-DESIGN.md` is not in any open program's
`paths` globs. It is filed here because FIX took the two units that
moved the arms it omits, not because FIX's charter claims the
document; a taker should say which way it read that.

## Re-homed to PATHS, 2026-09-20

(FIX orchestrator) Ev, in chat, 2026-09-20: *"can you kick all the design decisions back to
the track they actually belong to, leaving fix design-free?"* FIX is the
program for rows whose fix is already written; a row whose blocking
question is a DESIGN decision belongs to the track that owns the surface
the decision is about.

**The decision this row is blocked on:** whether `docs/PATHS-DESIGN.md`'s typed-refusal register should be checked
at all, or say in prose that it is not a claim.

**Why PATHS.** `python3 scripts/work.py territory --files -` says `docs/PATHS-DESIGN.md` is
**owned by paths**. This row was filed on FIX's slate because FIX took the two
units that moved the arms the register omits, and its own Fence section says
so — *"not because FIX's charter claims the document"*.

The cheap half (add `NonFiniteDirection` and `UnderflowedDirection` to the
register with the sentence distinguishing them from `ZeroDirection`) does not
wait on the design half and could ride any PATHS unit that touches the
document.

Nothing about the finding is changed by the move: same id, same
evidence, still `open`, and no part of its question is answered for
you except where this note says Ev answered it.

## Reference note (FIX's sweep, 2026-09-21)

`underflow-gate-owed-at-five-more-doors` was dropped from this row's `refs` because the row closed with **FIX**, which left the tracker at sweep 18 — `work/fix/` is deleted and `docs/DOC-LEDGER.md` is its done-state of record. The finding is unchanged and still readable: `git show 6f0e04ce1534:work/fix/underflow-gate-owed-at-five-more-doors.md`.

## Re-homed at PATHS's close (2026-10-21)

Moved from `work/paths/` to PROFILEPROSE (`work/profileprose/`) when PATHS closed. PATHS's live rows went to four programs opened at this close and cut along the priority seam (PROFILE, PROFILEREST, PROFILETAIL, PROFILEPROSE; `docs/doc-ledger/paths-leaves-the-tracker.md`). The item's id, content and history are unchanged.

**Its design half waits on a ratification, not on an item.** Whether `docs/PATHS-DESIGN.md`'s typed-refusal register is checked at all, or says in prose that it is not a claim, is a change to a ratified page and goes to Ev on an `[ev]` PR. The cheap half (the two arms and the sentence telling them from `ZeroDirection`) does not wait on it.

## The cheap half done (2026-10-26, tracker sweep)

The register (`docs/PATHS-DESIGN.md`, §3's Refusals paragraph, ~:1064
now) names `NonFiniteDirection` and `UnderflowedDirection` after
`ZeroDirection`, with the sentence that tells them from it. What is
left is the design half: whether the register is checked against
`PathErrorKind` or says it is prose. It is not complete today in other
ways too: `ArcMissesAuthoredPoint` and `ArcMissBelowSceneResolution`
(`crates/profile/src/path.rs`, `PathErrorKind`) are not in it either.
The Fence above is stale: `docs/PATHS-DESIGN.md` is in this program's
`paths`.
