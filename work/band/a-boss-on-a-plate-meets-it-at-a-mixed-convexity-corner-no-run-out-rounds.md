---
id: a-boss-on-a-plate-meets-it-at-a-mixed-convexity-corner-no-run-out-rounds
kind: issue
title: A boss on a plate meets it at a mixed-convexity corner that no run-out rounds, so the bracket cannot be filleted all round
status: open
opened: 2026-11-02
priority: P0
cost: H
design: true
pr: 6507
---

Filed by the REFS orchestrator (2026-11-02), from the designers' measurement on `work/refs/select-all-edges-picks-edges-a-fillet-refuses.md` (Ev: P0, "an ordinary part cannot be filleted all round in one click").

Once the bore's smooth struts are out of the request, a plate with an upright boss (flush or inset) refuses every edge at each vertex where the boss meets the plate: `UnsupportedCorner::MixedConvexity` ("2 of 3 edges convex … only a run-out policy would handle: feather the radius out"; 1 of 3 for an inset boss). This is the parked feather-out run-out (`docs/KERNEL-VERBS.md`, "fillet run-out": "no consumer has ever wanted either"), and the bracket is its consumer. The two-step workaround fails too: filleting the concave base rim first, then the convex edges, makes the convex band end on the base band's cylinder, a curved end face, which refuses typed.

Witness, committed: `crates/sweep/tests/pins_of_todays_blend_refusals.rs::pin_today_the_bracket_filleted_all_round_refuses_a_mixed_convexity_corner` (a 6 × 4 × 0.5 plate, a 0.5 × 3 boss flush with one side standing 2 tall, a two-arc bore; every edge requested refuses `UnsupportedCorner::MixedConvexity`). It pins today's refusal; this row's PR flips it to a build. The GUI replay script is in the REFS item.

Band P0 under work/README's definition ("a normal verb broken on normal geometry"), on Ev's own example; Ev was asked to confirm on the REFS `[ev]` PR. Priced H; it may want `X` once its shape is designed.

- 2026-11-02 — moved from BANDRULE to BAND by `git mv` when BANDRULE merged into BAND, id and body unchanged (the track budget now counts only P0, P1 and unbanded rows, and BANDRULE had fallen far under it).

## The rule

At a trivalent vertex whose three edges do not all have one convexity, the **odd edge** is the one whose convexity differs, the **pair** is the other two, and the pair's **shared face** `E` is the face the odd edge does not lie in. What a fillet or chamfer builds there depends on which of the three edges the request names. Each vertex is decided separately, inside one call, before any mutation.

- **All three.** The odd edge's band is cut off in `E`, by the ratified cut-off. The pair's bands do not end at the vertex. They run on as one tangent-continuous chain through the curve the cut-off leaves in `E`. That curve becomes a link of the pair's chain, resting on `E` and on the odd band itself. A fillet's link is the coaxial `cylinder×plane(⊥)` arm's torus, major radius `2r`, so the chain is cylinder, torus, cylinder. A chamfer's is a strip, mitred onto the pair's strips at both ends. Where `E` is oblique to the odd edge, the fillet's link is a canal surface and refuses `SpineUnsupported`.
- **The odd edge alone, or one pair edge alone.** The cut-off, whatever its convexity. A foot that lands inside a neighbouring face refuses on the existing door.
- **The pair alone.** The mitre. It is a mitre join: each band's surface continues past its edge's end to their intersection. So at a reflex corner of `E` the mitre removes, or fills, the offset corner's wedge (`d³/3` for a chamfer), and `E`'s outline becomes the mitred offset of the old one.
- **The odd edge and one pair edge.** The chamfer builds: the pair strip is cut off in the odd strip's plane. The fillet refuses: the pair band would have to stop where its edge runs smoothly into the cut-off arc.

The vocabulary after it:
- A band's end is one of `Patch | CutOff | Mitre | Overrun`. The pair's chain through a mixed corner is a link, not an end.
- One typed refusal, `UnbuiltEnd`, replaces `UnsupportedCorner`'s run-out policy and `UnsupportedRunOut`'s free-text detail. Its tags are valence, dependent normals, seam, indeterminate, curved end face, foot inside a face, feet crossing and run-out. Its payload is the vertex's convex count, its odd edge and the requested subset.
- `MixedConvexity`, `RunOutPolicy` and `CornerConfig::policy` retire. `Patch` carries no convexity.

Build order:
1. The gate. Uniform convexity is a precondition of the patch only, and a mixed vertex refuses only a request naming the odd edge with another edge (`band/mixed-vertex-gate`).
2. The junction carry-through: `a-rounded-outline-is-a-closed-chain-of-mixed-links-the-blend-refuses`, then `an-open-tangent-chain-of-mixed-links-refuses-at-the-open-carve`.
3. This row: the link resting on another band, the all-three rule, the odd-plus-one chamfer, and the vocabulary.
