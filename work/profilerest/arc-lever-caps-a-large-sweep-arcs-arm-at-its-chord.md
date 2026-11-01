---
id: arc-lever-caps-a-large-sweep-arcs-arm-at-its-chord
kind: issue
title: seg::arc_lever caps a large-sweep arc's arm at its chord, so a joint the arc's own reach parts reads tangent
status: open
opened: 2026-10-19
priority: P3
cost: M
design: true
---


`seg::arc_lever` (`crates/profile/src/seg.rs:1163`) is `min(radius, chord)`, and `Seg::arm` (`seg.rs:1170`) reads it for every arc stored by its chord. For an arc swept past `5π/3` the chord falls below the radius again and shrinks to zero as the sweep nears a full turn, though the arc still reaches `2r` from the joint and its tangent ray parts from it over about `r`. A full turn's arm is its radius (`ArcSpan::FullTurn`), so the arm drops from `r` to the chord at the last ulp short of a full turn. Found by #5377's review (finding F8), which asked whether the shorter-leg arm the joint pass now reads (`seg::joint_tangency`, `path_junction_turn`) is still right for such an arc.

Measured at the default ε (#5377's fix pass, a throwaway probe): a unit arc ending at the origin heading `+x`, a line out of it turned `φ`, a line back up and a line home tangent into the arc's start (the only simple loop a tangent leg longer than the chord fits, since the arc's own start blocks the other side of the pinch).

| sweep | `arc_lever` | φ·arm | φ·r | profile | extrude, depth 1 |
|---|---|---|---|---|---|
| 6.2 rad | 0.083 | 0.5ε | 6.0ε | joint 1 tangent | `SliverJoin` at joint 1, margin 6.0ε |
| 5.9 rad | 0.38 | 0.5ε | 1.3ε | joint 1 tangent | `SliverJoin` at joint 1, margin 1.3ε |
| 6.27, 6.2, 5.9 rad | | 5ε | 379ε, 60ε, 13ε | pair (0, 1) escalates at margin 5ε | |
| 6.27, 6.2, 5.9 rad | | 20ε | | a corner | builds, tier 3 passes |

So under the chord cap the profile records as tangent a joint whose legs part by `6ε` over the arc's reach, and the pair pass reads the same short lever (its escalation margin is `φ·chord`, not `φ·r`). Nothing builds silently: the extrude reads the dihedral at the faces' reach and refuses the join. At 6.27 rad the loop with the turn at 0.5ε is refused `NonSimple` before the joint is read.

The fix is an arm that follows how far the arc runs beside its tangent ray, `min(r, r·|sweep|)` or `r` past a half turn, rather than its chord, in the one `arc_lever` every reader shares; it moves the pair pass and the joint pass together, and every pinned margin that reads an arc's arm needs re-baselining with it.

**The fix moves a ruled verdict.** The 95ε witness of #5354
(`rejections.rs`, `a_cusp_on_crossing_carriers_95_eps_apart_validates`)
joins a 6.27 rad arc of radius 2.71 to a 1.41 rad arc of the same
radius. Its shorter arm is the first arc's chord, 0.0285 m, where its
turn reads 0.0106ε and the cusp validates, as Ev ruled. Levered at the
radius, 2.71 m, the same turn reads 1.006ε and escalates. So an arm
that follows the arc's reach is a change to what #5354 ratified, and
goes to Ev as one.

## Re-homed at PATHS's close (2026-10-21)

Moved from `work/paths/` to PROFILEREST (`work/profilerest/`) when PATHS closed. PATHS's live rows went to four programs opened at this close and cut along the priority seam (PROFILE, PROFILEREST, PROFILETAIL, PROFILEPROSE; `docs/doc-ledger/paths-leaves-the-tracker.md`). The item's id, content and history are unchanged.
