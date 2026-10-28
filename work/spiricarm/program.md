---
id: spiricarm
kind: program
title: SPIRICARM — the spiric and torus lanes: bands and rings with no lane, mirror tori, near-tangent ends, and the widths that grow with distance
status: ready
opened: 2026-10-27
group: kernel/props
prefix: spiricarm/
tag: (SPIRICARM orchestrator)
ab_band: 15400-15499
paths: [crates/geom-brep/src/props/spiric.rs, crates/geom-brep/src/props/spiric/*, crates/topo/src/props/quad_lane.rs]
keep_out: [opened 2026-10-27 when FLUX (51 budget points) and FLUXTAIL (44.5) were joined and re-cut by logical thread rather than by priority so that the successors can run in parallel (Ev in chat 2026-10-27) - the rows moved by git mv with ids and bodies unchanged and both directories left the tracker (docs/doc-ledger/flux-leaves-the-tracker.md), crates/geom-brep/src/props/spiric.rs and crates/topo/src/props/quad_lane.rs are shared with QUAD (the measure door and its lanes) - run scripts/work.py territory on your branch and announce the seam in the PR where the changes are coupled]
---

**The torus faces the kernel builds but cannot yet measure well.** A spiric-bounded torus band has no volume lane, the spiric wall reads only its own torus and not the mirror, and its ends near tangency can read a closed junction open at f64. A long plane face cut by a torus reads a pad 1e3× wide about a point off its plane, a major-1300 tube refuses at tier 3 though the door converges on it, a far elbow and a pinched corner-bar cap read wide, and a torus face whose chart closes both ways may read its side off the disc.

Slate, order and posture: `work/spiricarm/plan.md`; narrative in
`work/spiricarm/log.md`.
