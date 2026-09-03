---
id: rim-only-sphere-cap-panics-at-census
kind: issue
title: "mesh: tessellate panics at the issue-897 census on a rim-only sphere cap the shape door admits (even at f = 0)"
status: parked
blocked_on: [MESH-12]
opened: 2026-09-03
github: 1615
refs: [897, 1571, rim-continuation-witness-fixture-needed]
---

## From GitHub issue 1615

Opened 2026-09-03; 0 comments.

**Found by:** MESH-12's measurement pass (issue 1588's rim-continuation witness), filed by the S-MESH orchestrator as a forward observation. MESH-12 does NOT touch the walk; this is out of its fence.

**What was measured.** A sphere face bounded by a single rim-only loop (a latitude circle, no meridian edges, no pole crossing) passes the shape door — `require_iso_rectangle` admits it, and `require_one_chart_branch` has nothing to refuse since no traversed arc crosses a pole — and then `tessellate` panics in the cross-face identification census (`crates/mesh/src/tessellate.rs`, the issue-897 census) for every such cap tried, including the trivial one at f = 0 (the rim at the equator). The panic is the census doing its job: the emission for a rim-only cap is not a manifold patch by the census's count. So this is the same class as MESH-11's half-cap and bow-tie findings (a debug build panicked at the census at δ = 0.5), except that here the door does NOT refuse the input, so the walk reaches a shape it cannot emit.

**Pre-existing.** MESH-12's branch changes nothing on this path; the measurement was taken to locate the rim-continuation witness (issue 1588) and found that the only on-surface construction reaching `RimContinuation` is the Euler-door two-level rim, which constructs and which `examine_chart_coherence` reports on at 1.5ε / 1.9ε. The import route to the same state is dead at all ε (`pcurve_loop_continuity`, `crates/topo/src/pcurves.rs`, refuses at R·Δv ≥ ε before props decides).

**What the unit that takes this has to decide (design surface, not a fix-in-place):**
1. Is a rim-only sphere cap a shape the mesh lane admits at all? If yes, the walk needs a cap emission (the pole is interior to the face and no meridian edge names it) and the census must pass on it. If no, the shape door must cite a predicate that refuses it structurally — never by inspecting values — and the refusal must be a `NotIsoRectangle`-style typed door, not the census panic.
2. Whichever way, CERT-1's rows and `mass_properties` are untouched: this is emission, not measurement.
3. A debug-only census panic reaching a door-admitted shape is a fail-loud gap in the release build too: the release build emits the non-manifold patch silently. The unit should measure what the release build emits for the f = 0 cap and record it.

**Fixture.** MESH-12's PR (branch `mesh/12-saturated-span`) discloses the exact construction it used; the unit taking this should lift the fixture from there rather than rebuild it.

Band: S-MESH (1200–1299). Not scheduled; parked behind MESH-12.

## Home

`work/mesh/` — the panic is in `crates/mesh/src/tessellate.rs`, an S-MESH territory glob, and the issue names the S-MESH band and parks itself behind MESH-12.
