---
id: purchasable-area-tightness-valve
kind: issue
title: Purchasable area tightness - a demand-triggered valve on the patch lanes' area enclosure (caller-requested target, typed refusal)
status: open
opened: 2026-08-31
github: 1367
refs: [870, 873, 472, 1315, 1366, S26, S230]
---

## From GitHub issue 1367

opened 2026-08-31, 0 comments.

(S-CERT orchestrator) Filed by CERT-6 (issue 870, PR 1366) as the door Q1 left open — **a design sketch, not a work order. No consumer asks for this today, and it should not be built until one does.**

**The ruling this sits under.** S-CERT **Q1** (Ev, in-chat 2026-08-29) declined always-on area metering: any realized geometry everywhere within ε of correct is valid, so the wide-but-sound default bracket stands and no funnel target is built — independently supported by the cost arithmetic, since an ε-scale area target is a ~10³–10⁴× piece-count multiplier under an O(h) rule. Purchasable tightness was explicitly deferred to a **demand-triggered valve, filed not built**. This is that filing.

**What the default gives.** Both patch lanes fix `QUAD2_AREA_PIECES = 64` *before* the refinement rounds and no round recomputes it, so the area bracket is resolution-driven, not tolerance-driven: bit-identical at every ε and on `Interval`. Measured post-CERT-5 on shape (iii)'s loft: `area_pad` = 0.189612 m² on a 25.3214 m² surface (7.5e-3 relative) beside a `volume_pad` of 1.07e-13 m³ — eleven orders apart, because one is metered and the other is a denominator at a frozen resolution.

**The sketch.**
1. An optional caller-supplied area target on the patch-lane entry points, defaulting to absent — absent must keep today's numbers bit-identical (D9), so the valve cannot be a change of default behaviour.
2. `QUAD2_AREA_PIECES` recomputed per round when a target is present. It is currently pre-refinement and therefore *cannot* respond; this is the structural change, and it is why the valve is not a small patch.
3. A typed area-side refusal when the target is unreachable in budget — the analogue of `PropsError::QuadratureBudget` on the flux side. **Minting it changes which faces certify**, so it needs its own D2-addendum classification and a row-by-row argument over the shipped refusal set, exactly as `QUAD2_RATIONAL_MAX_ROUNDS` does (issue 1315).
4. Re-cost the round budget against an O(h) rule: 2× the cells per halving of the width, per axis.

**What lands red when it lands.** `review_m6_3_chart_probes.rs`'s deliberate lower bound (`out.area.width() > 1e-3`, currently clearing by 2634×) reds *for the right reason* under an area-refining funnel — it must be re-derived, not deleted. `m6_loft_body.rs`'s and `m5_pr11_quad_props.rs`'s ceilings tighten. `props/quad.rs`'s A2 gauge (CERT-6) is unaffected: it is an outer tripwire 52× above the corpus, and a funnel only moves values away from it.

**Related:** issue 870 (the measurement), issue 873 (the acceptance rows and the flux/area coupling), issue 472 (deferred the metering in writing), issue 1315 (the round-budget dial), S26/S230 in `docs/SMELL-SCAN-2026-08.md`.

## Home

S-CERT: the valve sits on the patch lanes' area enclosure in `crates/geom-brep/src/props/*`, S-CERT territory, and it is the door S-CERT's own Q1 ruling left open — enclosure quality and metering is the program's charter.
