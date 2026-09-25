---
id: canonical-segment-type-in-profile
kind: unit
title: Lower to the canonical segment form (verbatim vertices + Line | Arc{centre, radius, Δθ}) inside profile; byte-identical
status: dispatched
opened: 2026-09-25
priority: P1
cost: H
parent: lower-profiles-to-carrier-and-interval-not-vertex-and-bulge
branch: claude/clever-bardeen-4itqb3
---


Unit 1 of the #3218 lowering (A2, ratified by Ev). Every profile-side reader of the bulge form moves to the canonical segment, except geom-brep (unit 2). Carriers are derived exactly as `seg::arc_carrier` derives them today, and Δθ = 4·atan b, so output is byte-identical. Survey: parent row, "Survey" §1a–1c, §3(a)(b)(d).

## Spec (PATHS orchestrator, 2026-09-25)

**Goal.** `ProfileLoop` and `ValidatedLoop` store verbatim vertices,
with one canonical segment per edge:
`Line` | `Arc { centre, radius, sweep: Δθ (signed) }`. This is A2 in
D1 "Profile format" as merged on #3218. **Output is byte-identical**:
every existing test, golden, census and demo frame is unchanged.
**No full turn yet**: |Δθ| < 2π is still enforced, and unit 3 admits
n = 1.

**Where each value comes from in this unit.** At lowering, both the
emission layer and `RawLoop`'s bulge input compute the arc's carrier
exactly as `seg::arc_carrier` does today from (chord, bulge), and set
Δθ = 4·atan(b). That is what makes the unit byte-identical. Storing the
carriers the constructions actually build is unit 5.

**Readers that move** (survey §1a–1c, §1e):
- `seg::build_seg` and every predicate that currently reads the bulge
  or `Seg::bulge`;
- `ValidatedSegment` (its `bulge` field and the "one sanctioned
  re-inspection" doc paragraph), `SegmentKind`, `canonicalize_loop`
  (reversal negates Δθ);
- `loop_orientation` and area sums;
- `lift.rs` (`carrier_form`, `chain_form`);
- sweep's `swept.rs`, `extrude.rs`, `revolve/*`, `skin.rs` and
  `loft.rs`. Note that `skin::vertex_segment`'s `bulge == 0.0` test
  must read the kind;
- editor-core's `anchor.rs`, `stackup.rs` digest, `program.rs`,
  `names/emit_sweep.rs` and `eval/wire.rs`.

geom-brep's `SketchSegment` is unit 2's. Here, sweep keeps handing it a
bulge that it derives from Δθ at that one boundary (`tan(Δθ/4)`),
which must reproduce the stored bulge bit-for-bit. **Prove that for
every arc**, or keep the original bulge beside Δθ until unit 2, and say
which you did. The `RawLoop` fixture door and `ProfileVertex` keep
their current bulge input in this unit; their migration is
`fixture-door-takes-canonical-segments`.

**The certified-scalar lift.** `ValidatedSegment::lift` REBUILDS the
arc's carrier at `U` from the embedded endpoints and bulge, and that
derivation is what mints the interval enclosure. Keep that behaviour
exactly: the lift still rebuilds the carrier at `U` from the stored
data (endpoints plus Δθ, or the retained bulge). **Do not simply
embed a stored f64 centre.** That would change what the interval lane
certifies, which is a design question and not this unit's. If you find
you cannot keep it, stop and report.

**Hard constraints.**
- Endpoints are never recomputed from angles.
- No new ε and no new predicate name in the K stream.
- Every existing refusal keeps its variant and payload.
- The seven hand copies of bulge→carrier are unit 5's, so leave them
  alone unless a copy's input disappears. In that case route it
  through one shared helper and say so.
- D9: the lily and the tour stay byte-identical.

**Review tier: dual** (two independent Opus reviewers, same frozen
head, `docs/DUAL-REVIEW-PROTOCOL.md`). The reason is that this is an
architectural change with broad reach, and every later lowering unit
stands on its byte-identity claim.

**Claims the reviewers will falsify:**
- (C1) byte identity, across goldens, censuses and demo frames;
- (C2) the certified lift is unchanged;
- (C3) no reader still re-inspects a bulge except the one geom-brep
  boundary;
- (C4) reversal in `canonicalize_loop` negates Δθ and keeps the carrier;
- (C5) the survey's reader census is complete for `profile`, `sweep`
  and `editor-core`, with its blind spot re-swept.
