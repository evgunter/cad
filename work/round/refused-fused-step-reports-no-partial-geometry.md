---
id: refused-fused-step-reports-no-partial-geometry
kind: issue
title: design: what a refused fused step (arc_fillet_arc and kin) hands back, so the path preview can draw the carriers it did bind
status: open
opened: 2026-09-18
refs: [path-preview-draws-nothing-for-a-refused-step]
priority: P1
cost: D
---

**This is a DESIGN item, not a dispatchable unit.** Its question changes what the replay driver returns, and that is the interface the ratified PATHS lattice exposes. Settle it in an `[ev]` PR before anyone builds it.

Follow-up to `work/author/path-preview-draws-nothing-for-a-refused-step.md`, split out at Ev's request (2026-09-18). That item's first step, drawing the prefix that replays when step `k` refuses, needs nothing from the driver and is the author program's. This item is the second step. It covers the fused family (`fillet_arc`, `arc_fillet`, `arc_fillet_arc`), where one step binds several things before it can fail. For example, `arc_fillet_arc` authors an incoming arc carrier, a fillet radius and an arrival arc carrier, and resolves the fillet between the two arcs (`arc_fillet_arc_kernel` in `crates/profile/src/path/family.rs`). When the resolution refuses (the radius does not fit, the carriers do not meet, an anchor falls outside the trimmed extent), the two carriers are exactly what an author needs to see to judge the numbers. Today they are lost: the refusal is a `ReplayError { step, kind: Path(PathError) }` carrying the reason, and the driver returns no geometry.

**Why it is a design question and not a patch:**
- **What travels.** A refusal could carry the bound carriers (circles and lines in the profile frame), or the partial `ProfileLoop` up to the refusing step plus those carriers, or the driver could grow a separate "replay as far as you can" door that returns geometry and refusal side by side. Each is a different contract for `replay` and `ReplayError`.
- **Authored data only.** A carrier the step derived (a `Radius` arrival's centre, the arc-extension carrier) is not authored data. Handing it out in a refusal is new surface, and PROFILES-V2 §V1 keeps derived values out of what is recorded. Whether a refusal payload counts as "recorded" needs a ruling.
- **Refusal text is not the cause** (`memories/refusal-text-is-not-cause.md`). A payload that the preview draws becomes evidence a person reads. What it shows has to be what the raising site actually had, not a reconstruction.
- **Scope across the family.** `CornerRefusal` and `CornerReason` already carry some of the corner's geometry for the fillet recourse (`fillet_select.rs`, BLEND's ground by `work/paths/program.md`'s `keep_out`). Any answer should reuse that, not add a second payload beside it, so BLEND has a say.

**Owed:** an `[ev]` PR proposing one of the shapes above as a PATHS-DESIGN revision. Implementation follows as a unit once it is ratified. The chrome item then draws what the chosen shape hands back.

**Evidence from the prefix half (AUTH-5, `author/path-preview-prefix`).** The prefix the preview draws for a fused step is often shorter than `steps[..k]`, and what it leaves out is exactly what this item would hand back. A fused step whose arrival is completed by binders is refused AT the binder, not at itself: `line_to, line_to, fillet_arc { radius: 0.001, spec: Radius { r: 0.01 } }, at (0, 0.02), angle 3.0` is refused at step 5 (`NoCornerForFillet { reason: CarriersDoNotMeet }`), and the tips before steps 4 and 5 are radius arrivals no provisional close can leave, so the prefix drawn ends before the fused step (step 3). The incoming ray and the arrival circle the fillet could not join are therefore not on screen at all, only the two legs before them. A step refused at itself (the `arc_fillet_arc` the form offers after two legs, refused `NoCornerOfPair`, every corner behind the incoming ray) draws everything before it and nothing of its own two arcs, so the `CornerRefusal` points in its payload are the only trace of them.
