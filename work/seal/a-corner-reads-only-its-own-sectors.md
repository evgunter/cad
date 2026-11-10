---
id: a-corner-reads-only-its-own-sectors
kind: unit
title: a corner reads only a glued pair as one surface; an unglued pair that reaches its coplanar branch refuses on the corner's own reading (CornerCrossing at a vertex pair's arm, CornerBound on a bound's tie)
status: closed
opened: 2026-11-09
priority: P1
cost: M
closed: 2026-11-10
pr: 6693
parent: glue-disagreements-and-held-contradictions-after-stage-4
---


Step 4 of the parent's design (fork `glue-disagreements-carry`, 2026-11-09). Step 1 gave `bounds_off` its `Decided` (`Result<Decided, Indeterminate>`), and this row was opened when step 1 closed.

## What

A corner that meets a pair the glue door did not glue re-reads the carrier pair over its own arm (`carrier_eq_reading` in `recl.rs` and `vtxfac.rs`), and `unglued_tilt` decides `bool_plane_parallel` a third time. All of it serves only to build a refusal, and C22 (`unglued_coincidence`) is that refusal.

- **Carrier identity is the glue door's alone.** A corner reads a glued pair's record. An unglued Zero never opens the coplanar branch; it is a refusal.
- **Retired:**
  - the carrier re-read in `recl` and `vtxfac`;
  - C22, `unglued_tilt` and `ReadOn`;
  - the "definitely-distinct" and "departs definitely" invariants, which become unrepresentable;
  - DR-245's three-step precedence, and with it the door's in-band arm (`door_refusal`, `Glue::in_band`, `InBandPair`).
- **Vertex pairs** (the old `ReadOn::Faces`): the corner refuses at `fan::meeting`'s own reading, the tilt at its arm, under a new Positive-only `Coincide` row.
- **A corner lying on a plane face by its bounds** (the old `ReadOn::FirstBounds`, and `vtxfac`'s flat sector): the corner refuses on the reading that put it there, carried from where it was read.
- **README C4:** the sentence "An unglued pair is read at a corner by the ladder over the corner's own arm, where every verdict but `Distinct` refuses" is replaced by the design's. It is agent text (`c7571919b0`, PR 6079), with no ratification found.

## Design (fork `unglued-corner-ending`, 2026-11-10)

The review of PR 6693 falsified the orchestrator's ruling (b), a gate rejection of the far bound under `SizedPass::Zero`. The corner does not pass Zero: at an unglued corner a Zero refuses too, so that tag was chosen only to get past `gate_rejection`'s `unreachable!`, and a mutant to `SizedPass::Negative` survived every row. It also re-gated a margin `bounds_off`'s NonZero gate had admitted, and it quoted the far bound with no offer, although tightening is measured to build.

Two designers (Fable A, Opus B) converged in round 2 on B's position, with no question for Ev:

1. **Carrier identity is the glue door's alone.** A corner reads a glued pair's record. For planes, `Meeting::Coincident` opens only from a glued pair's record. An unglued Zero never authorizes a coplanar lump: the lump speaks for the full spans (DESIGN `[tolerance.epsilon]`), while an unglued Zero is a refusal acting over no extent. No `Off(Decided)` on the far bound.
2. **The vertex-pair path.**
   - The corner's question is whether these faces cross resolvably within this corner, read on `fan::meeting`'s `bool_faces_parallel` tilt at the corner's arm. The arm lever stays.
   - Only Positive passes. An unglued Zero ends as `k_stats::gate_rejection("bool_faces_parallel", <that Decided Zero>, band, SizedPass::Positive)`, under a new `Coincide` row that passes only Positive.
   - It carries the reading as read (θ·arm), offers "tighten below θ·arm/K" beside the lever, and is the same in both operand orders.
3. **The bounds path, and `vtxfac`.**
   - The refusal is the reading that put the sector on the coplanar branch: the steeper bound's tie, carried from where it was read. It is never re-read through `bounds_off`.
   - A rejected Zero passes on the same rule. A tie read under a cone's slack ends as that in-band reading.
   - `vtxfac`'s tilt at the span goes too.
4. **The door's in-band arm never binds.** Whenever the corner reads a decided Zero, the tolerance it offers also decides the door's in-band margin apart, since Kε′ < θ·arm ≤ ε < the door's margin. DR-245's three steps collapse to the one rule.
5. **The pose still refuses at the default ε.** Making it answer is SECTOR's P3 `a-vertex-pair-near-coincidence-refuses-where-its-long-edges-decide`.

## To probe before filing anything

- On the `Faces` path, a tolerance that `bounds_off` offers above `bool_faces_parallel`'s margin at the short arm may still read Coincident and refuse again.
- Whether a fold step's corner reaches this arm.

## Done when

No corner reads a carrier pair. Both `ReadOn` arms end on the sector's own reading, pinned by a scene of the 1 mm / 10 m shape.

## Dispatch (2026-11-09)

Branch `seal/corner-sectors`. Review tier: **single, FULL**. The fork settled the shape: no corner reads a carrier pair, and the precedence stays as it is. What is at risk is correctness on the `ReadOn::Faces` arm, where two readings of one sector over two extents disagree, and whether the retired invariants were ever reachable. A FULL review's claims target that, so the step does not take the dual tier.

Reworked on 2026-11-10 to fork `unglued-corner-ending` after the first review falsified the orchestrator's ruling (b). The rebuild had a fresh single FULL review on `23595a252d`, then a delta review of its fix pass on `9aedca00ce`. The last fix pass merged on the orchestrator's delta read.

## Closed (2026-11-10)

Built to the fork `unglued-corner-ending` design (PR 6693). The PR body has the per-site table, each ending before and after.

**Carrier identity is the glue door's alone.** No corner reads a carrier pair: `recl::require_same` and `vtxfac::unglued_flat_sector` no longer call `carrier_eq_reading`.

**Vertex pairs** (`fan::meeting`):
- Two plane faces parallel at the corner's arm open the coplanar branch only for a pair the glue door glued.
- Unglued, the corner refuses on that reading: `Coincide::CornerCrossing`, the `bool_faces_parallel` Zero rejected by `gate_rejection(.., SizedPass::Positive)`. It carries θ·arm and offers θ·arm/K, the same in both operand orders.
- It refuses only where the two sectors overlap (`fan::Meeting::Parallel`, `fan::unglued_parallel`); sectors that only touch at the vertex are no record, as on main. The fix pass found this through sweep's slow `short_arcs_at_a_crescents_tip`, whose `one side` union refused `CornerCrossing` at `23595a252d`.
- A curved face keeps C8's frontier refusal.

**A corner lying on a plane face by its bounds** (recl's flanks and line cones, vtxfac's flat sector):
- It refuses on the reading that put it there, carried from where it was read: the flank's tie (`recl::FlankKey::On`), the line cone's representative's own reading (`sectors::SheetRead`), and the steeper of the flat sector's two ties (`vtxfac::EntryTie`).
- The refusal is `Coincide::CornerBound` (`boolean::unglued_corner`, which either definite side passes). A Zero tie is the nonzero gate's rejection. A slack's tie is the slack's own reading: the moved reading in band, or the distance moved by the slack either way, `[d − s, d + s]`, straddling the band (`bool_corner_side_slack_straddle`).
- `bounds_off` is deleted.
- A record the meeting read as one coincidence carries how (`fan::Coincident`: the glue door's relation, or the frontier's curved face), so `recl_sectors` never re-reads the glue record. Where a line cone reads its representative `On` but the representative's own reading decided it off the face, the pass ends `LineUnread::Contradicted`.

**Fix pass (2026-11-10):** the in-band crossing is pinned (`fan::rows::only_a_glued_pair_opens_the_coplanar_branch` at 1.2·zero, `offer_rows`' `corner_tilted_in_band_at_its_arm`). The slack straddle pairs `d − s` with `d + s`. `bool_sector_coplanar`'s two rows are closed (HONEPAIR, HONEONE).

**Retired:**
- C22 (`unglued_coincidence`), `unglued_tilt`, `ReadOn`, and the three "definitely-distinct" and two "departs definitely" invariants;
- DR-245's precedence, with the glue door's in-band arm (`door_refusal`, `Glue::in_band`, `InBandPair`);
- the cross-operand plane door (`PlaneDoor`) and what only it minted: `Coincide::OnPlanes`, `Coincide::Carriers`, `BooleanDecision::PlaneOrientation`, `undecided_coincidence`, `plane_eq::senses` and `GlueClass::of_senses`.

**Probes:**
- **The 1 mm / 10 m scene** reaches the vertex-pair corner through the public union. On main it ended C22 with margin `0.0`. It now ends `CornerCrossing` on θ·1 mm.
- **The bounds' in-band offer on the `Faces` path:** on chord bounds the vertex-on-face sweep refuses the far vertex first, on the same margin, so no second corner refusal follows. Curved and bisector far bounds were not probed.
- **The fold step:** an n-ary union of the scene and a far brick, in three member orders, refuses at the pairwise judgement before any step runs.
- **The edge-edge path:** two reads of one bound disagree. `recl`'s `a_sheets_side_decided_within_the_cones_tie_is_the_corners` shows it: the line cone's slack reads the flank On while the band decides it. The row now pins the slack's straddle.

**The door's in-band arm never binds.**
- With `door_refusal` gone, the in-band wedge rows still refuse at the corner, and their offers execute (`offer_rows`' `thin_wedge_in_band_over_its_faces`, `the_in_band_wedge_is_in_band_at_the_glue_door`).
- At ε 1e-12, where the corner's offer falls below the floor, the corner says so; the door offers none either.

**Following the offer:** the designers' generic pose builds in all six runs just under the offer, and the hinged pose meets `bool_contact_edge` there (`the_corners_offer_executes`).

**Closed into this row:** `decided-coincidence-carries-a-synthetic-invalid-margin` (from TWICE) and `the-corners-senses-decide-the-plane-orientation-a-second-time`.

**Filed (HONEONE):** `a-crossing-germ-re-decides-the-faces-parallelism-the-fan-decided` and `the-covered-arc-re-reads-a-carrier-pair-the-glue-door-decided`.

**Evidence added:** to SECTOR's P3 row (the span lever's measurement) and to FLUSHTAIL's door-offer row.
