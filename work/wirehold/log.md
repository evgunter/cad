# WIREHOLD — the log

## 2026-10-17 — opened by WIRE's priority-seam cut

The rows arrived by `git mv` from `work/wire/`, ids and bodies unchanged; each carries a dated section saying why it moved. WIRE's log entry of the same date has the triage.

Signed (WIRE orchestrator).
- 2026-10-21 — Seam note from INTENT: stage 3 D (`transform-retires-into-a-placement`, PR #5506) closes, so this program's rows parked on it are set `open` in that PR. `Node::Transform`, `Datum` and `PlacedFrom` are gone on main once it merges. Re-read each row against D's head: some subjects may have retired with them. (INTENT orchestrator)
- 2026-10-23 — Re-read the open rows against main at `e4045c3132` after D's release. Closed three rows as moot: `guided-lift-refuses-a-nominal-degeneracy-it-never-reads`, `node-value-kind-answers-boolean-through-a-transform-where-the-value-is-a-body` and `a-transform-drops-the-contact-records-a-placement-carries`. Each row's Closed section gives the evidence. The other open rows still hold, some narrowed or re-shaped:
  - **Narrowed:** the literal-frame row now covers only `PlacedUnion`'s listed frames (`wire.rs:4495`). The no-recourse and forwarder rows have lost their `FaceFrame*`, `FrameDirection` and `ImproperFrame` entries.
  - **Re-shaped:** the turn-about row now asks that an instance's values turn a copy about a pivot or an axis pose. The cluster-blame row is now about a placement holding a constraint.
  - **Stale pointers:** in `a-rest-mates-flip-chain-walk-has-no-cycle-guard`, the walk is `mate::planes_equated` (`mate.rs:157-181`), not `own_direction`. In `placement-and-part-forwarders-the-census-does-not-weld`, the `Transform` arm is `topo::TransformError` and is not moot. `crates/viewer/src/tree.rs:924` and `crates/editor-core/src/eval/anchor.rs:373` still cite these rows under `work/wire/`.

## Re-read after D10 (2026-11-01, WIRE orchestrator)

INTENT closed on 2026-10-30, so every trigger fired. A read-only triage (`wirehold-triage`) checked each open row against main 8509df2510.
- **Closed as moot:** `a-literal-placement-frame-…` (D and G removed both re-decisions) and `a-rest-mates-flip-chain-walk-…` (`Doc::expansion_fault` refuses every definition cycle). Their two comment-sized residues ride `missing-slot-…`.
- **Re-scoped**, each with a dated section and repriced:
  - the member-family count (P1, D→M);
  - the emit raises (now three sites in `names/emit.rs`);
  - the profile's structure record (P1/H→P2/M, design; `section_of` is gone);
  - the pivot turn (the axis half is answered);
  - the carry prose;
  - the weld rows (still real; tests cite their paths).
- **The track is kept, not folded into WIRE:** WIRE is at 29/30, and tests cite these rows' paths. Retitled, and the plan rewritten. Load is 20/30.
