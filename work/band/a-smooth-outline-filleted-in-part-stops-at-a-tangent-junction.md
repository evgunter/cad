---
id: a-smooth-outline-filleted-in-part-stops-at-a-tangent-junction
kind: issue
title: A smooth outline filleted in part stops at a tangent junction into an unrequested link, which no end builds
status: open
opened: 2026-11-10
priority: P3
cost: M
design: true
refs: [an-open-tangent-chain-of-mixed-links-refuses-at-the-open-carve, a-boss-on-a-plate-meets-it-at-a-mixed-convexity-corner-no-run-out-rounds]
---


Filed by the band-openmix lane (2026-11-10), measured while building the open mixed chain.

A request that names part of a tangent-continuous outline, and stops where its last edge runs on smoothly into an edge it does not name, refuses at the battery: `UnsupportedRunOut` with `PARTIAL_TANGENT_RUN` (`crates/sweep/src/blend/battery.rs`, `corner_at`, read before any cut-off or corner reading, whatever the arm of the link that stops there). The witnesses are `crates/sweep/tests/band_open_mixed_chain.rs::a_smooth_outline_filleted_in_part_refuses_naming_the_whole_run`: the rounded plate's top run of line, arc and line requested alone, and the L-plate's line alone, line and arc, and arc alone. Before, the straight link's end read `END_FACE_CURVED` and the torus link's `UnsupportedCorner { Indeterminate }`. At such an end the face the band would stop against is tangent to the band's own support there. No cut-off builds against a face tangent to its support, so this is not the curved-end-face case of an ordinary cut-off.

The refusal's recourse (`FILLET3_CORNER_RECOURSE`) asks for every edge the request runs on into smoothly. Requesting the whole outline builds as the closed mixed chain.

What the band should do there has several viable answers, and this row decides none:
- refuse, as today;
- stop flush in the junction's section plane, leaving a step where the unrequested link goes on sharp;
- a ball-cap stop, the ball at rest at the junction capping the band with a sphere patch (`crates/sweep/README.md` A3-3's presumptive first pick for a mid-curve run-out);
- a feather-out, the radius tapering to zero along the unrequested link (variable-radius machinery).

Ev, on PR 6507 about the sibling case (a fillet that would have to stop where its edge runs smoothly into an unrequested one): "i'm not sure if the fillet must always refuse in that case but we certainly don't need to build that case now". The orchestrator ruled it stays refused for the open mixed chain (2026-11-10).
