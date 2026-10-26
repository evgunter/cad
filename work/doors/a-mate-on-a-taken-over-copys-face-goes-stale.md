---
id: a-mate-on-a-taken-over-copys-face-goes-stale
kind: issue
title: A mate another placement seats on a face of a body's copy goes stale when a fillet or mirror takes that copy over: the mated part is no longer placed
status: parked
opened: 2026-10-25
priority: P0
cost: M
blocked_on: [5636]
---


Found by the third review of PR 5683 (the mirror tool; sure, probed). A small block is mated onto a base's top face, and then the base is mirrored. The small block's `Place` row fails: "constraint #… side … no longer resolves … the recorded reference disagrees with the recipe … (Place <base copy>'s payload differs). Recourse: rebind it." The small block is no longer placed.

A fillet of the base fails identically (the reviewer's comparator), so this is the takeover's own rule, not the mirror's. `repoint` changes the copy's payload, and every recorded reference that reads the copy's faces goes stale. The refusal is typed and carries a recourse, but the everyday case is exactly this: a symmetric or filleted base with parts seated on it. `session::select`'s `Renamed::through` does not cover it; it serves the transform's one-placement-deeper case.

**To decide when dispatched.** Should the takeover re-seat recorded references whose face survives in the result through the result's names (a fillet and a mirror both keep the source's names; the image carries them verbatim), or does the reference record the wrong thing? Check whether the recorded payload should name the copy's body rather than its source's operation. The tests should mate onto the body that is then taken over, the case the mirror suite's mated row excludes.

## Decided (2026-10-25, designer pair, converged in round 1; AUTHTAIL orchestrator)

**Premise corrections (both, sure):**
- The row's "a fillet and a mirror both keep the source's names" is wrong for what is taken over. A fillet re-mints survivors as `From { read: B, of: n }`. The mirror gesture takes the copy over with `B ∪ image`, and a union keys its members by read (the −x-wall mirror's top comes out merged).
- The staleness check is right to fire. Resolution is a lookup (N4), and following descent automatically is the opt-in policy `[identity]` excludes (Ev's text, `e56146c06b`).
- The reference records the right thing.

**The design.** The takeover becomes one authored action:
1. it re-points the copy (as now);
2. it writes the existing `DocEdit::Rebind` for each name read in the copy whose carrier in the result's names is unique: `From { read: B, of: n }`, or a `Merged` set containing it.
3. A fragment, a borders-qualified row or a tie counts as no carrier and is left stale, with the result's own diagnosis.

It is one undo step. The door is editor-core's, beside `refactor::inline` / `sketch_on_face` (e.g. `refactor::take_over`), and the carrier query lives in `names/`. The viewer's `feature_over`, `add_mirror`, `sketch_on_face` and `add_transform` call it, and the Rust and Python façades get the same semantics. The same rebind map re-seats the viewer's held picks, replacing `Renamed::through`.

**Open for the implementer:**
- `Rebind { body: Some(_) }` rewrites top-level selections of that body only. It must also reach a name read in the copy wherever it is embedded downstream, e.g. a face of `C ∪ D` spelled `From { read: C, of: face }` (one designer, likely).
- Merged rows may need the staged result evaluated to read its table (one designer); check whether the carry is structural.

**Sequencing: after D′3 (PR 5636).** D′3 makes copies carry names verbatim, which deletes the `Placed { of }` handling a fix now would need, and it rewrites `repoint`. D′3 also fixes the transform takeover's staleness incidentally. Questions D′3 must settle are noted on INTENT's log.

**Tests owed:**
- P mated on C's face, then each of fillet, mirror and sketch-on-face: P stays seated.
- A selection on `C ∪ D` through the same takeover.
- A face the feature consumes stays stale with its diagnosis.
- One undo restores everything.

## Moved (2026-10-26)

AUTHTAIL folded into DOORS (Ev, in chat, 2026-10-26): both were AUTHOR's
successors on the viewer authoring files. Id and body unchanged.
