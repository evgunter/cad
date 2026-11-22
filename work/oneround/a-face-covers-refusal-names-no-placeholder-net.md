---
id: a-face-covers-refusal-names-no-placeholder-net
kind: issue
title: A face cover's refusal cannot tell a placeholder net from a poisoned one, nor name which datum is not a finite number
status: open
opened: 2026-11-22
priority: P2
cost: M
---


Filed by PR 7225 (`oneround/box-region`) on the ruling for its F3, which took the other option for now.

**What the refusal cannot name.** `geom::cover::Unknown` (`crates/geom/src/cover.rs:171`) has three causes: `Unbounded`, `PoisonedNet` and `NonFinite`.
- A spline carrier's net reads `PoisonedNet` wherever any channel of it carries poison (`crates/geom/src/cover/readings.rs:342`). A placeholder net (`NetState::Placeholder`, `crates/geom/src/surfaces/nurbs.rs:537`) is a net every channel of which is poison: a seed face whose surface is not described yet, a legitimate mid-construction state. It reads `PoisonedNet` just as a net with one corrupt value does. The refusal cannot tell them apart.
- `NonFinite` names the surface kind and nothing else. It does not say which datum (a torus's tube radius, a cone's apex, a window end) describes no locus. Check 1 names that datum.

**First reader: the session box.** `ValidationError::SessionBoxUncovered` refuses at the gate before check 1 runs, and carries the cover's refusal. Its sentence and recourse are chosen per cause class (`uncovered_face`, `crates/topo/src/validate.rs:3357`). The net class says "its surface may not be described yet" with the placeholder recourse, true of a seed face and of a corrupt net alike. The datum class names only the surface kind.

**Second reader: the turn lever.** Of the faces whose cover refuses at `WedgeReach::reach`, nearly all are NURBS faces in `NetState::Placeholder` mid-construction. `euler::certify_edge_spec` alone has 1,436,200 of 6,789,246 reads at 1e-9 (`analysis/oneround/turn-lever/m-a.md` on `oneround/turn-lever`, commit f0302665). A typed "no geometry yet" is wanted there too, so such a read is not counted with a corrupt net.

**The shape of a fix**: split `Unknown::PoisonedNet` into a placeholder cause, read from `NetState`, and a poisoned cause. Give `NonFinite` the datum it found (a `&'static str` naming the field). Every reader on `scripts/gates/cover-kind-match.readers` then says what each new cause does. The session box's net sentence then splits in two: the placeholder's ("no surface described yet") and the defect's.
