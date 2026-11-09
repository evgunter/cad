---
id: glue-disagreements-and-held-contradictions-after-stage-4
kind: unit
title: after D10 stage 4: type the glue door's disagreements and whatever declared-pair contradiction survives (mints C20-C22, C24, and the 35 held arms)
status: open
opened: 2026-10-10
priority: P1
cost: M
design: true
parent: topo-mints-indeterminates-outside-the-funnel
refs: [decided-coincidence-carries-a-synthetic-invalid-margin, contact-gate-readers-drop-the-arm-verdict-or-mint-invalid, 4641]
---

Unit 5 of `topo-mints-indeterminates-outside-the-funnel`'s re-scope
(2026-10-10, main `98a3817a1d`). This unit is the residue of the
design's step 3.

## Why it is parked

Of the row's 24 class-(b) arms, 23 are D10-held. Stage 4 PR F
(`declared-pairs-retire`) and `mates-declare-no-contact` delete the
variants that carry them, so there is nothing yet to type.

## Sites

**The 35 held arms.** They are listed in the parent's Re-scoped section,
and they sit in these files:
- `contact_verify.rs` ×7
- `carrier_eq.rs` `definite()` ×3 and `unsettled` ×2
- `boolean/mod.rs` labels ×14
- `vtxfac.rs:549`
- `rim_wedge.rs` ×5
- `merge_faces.rs:2344`, `:2381`
- `flush.rs:315`

Re-census them once stage 4 is built. Each arm that survives takes one
of two paths:
- **A contradiction that still carries evidence:** carry
  `(predicate, geom_core::Decided)` beside the band its error already
  holds. Do not add `geom_core::Definite`, which would be a third
  spelling beside `Decided` and `geom_brep::recourse::Classified`. End
  it through `RefusedArm::SignCertain(Some(margin))` and D4 ¶1 (iv)'s
  defect ending.
- **A structural fault (no margin read):** give it a typed arm, not an
  `Indeterminate`.

**Plus the non-held sites on the glue door's ground, which stage 4
reshapes:**
- **C20, `carrier_eq.rs:168` (`CoincidenceMeasure::decide`'s
  `Unreadable`).** This is honest poison. It needs a door after the seal:
  `gate_measured`, or a payload that is not an `Indeterminate`.
- **C21, `carrier_eq.rs:836` (`coincident_as_declared`).** The sum
  decided nonzero where every datum decided zero.
- **C22, `boolean/mod.rs:1238` (`unglued_coincidence`).** A corner
  decided Zero that the pair door did not glue.
  - C21 and C22 are two `Decided` readings of different extents that
    disagree. D10's glue door is "one verdict per carrier pair", which
    may settle them.
  - Otherwise each becomes a typed finding carrying both readings
    (design item 7).
- **C24, `sweep/src/blend/battery.rs:128` (`measured`'s NaN →
  `INVALID`, on BAND and CARVE's ground).** A reporting projection that
  needs a public poison reading once `INVALID` is crate-private.

## Re-census (2026-11-09, main `b3363f462e`)

Stage 4 PR F (`declared-pairs-retire`, PR 4641) and
`mates-declare-no-contact` (PR 5637) have landed. A read-only census
re-read every site against main; paths are under `crates/topo/src/`.

- **The deleted variants are gone:** `BooleanCoincidence`,
  `ContinuationContradicted`, `SeamContradicted`, `FIT_DEFERRAL`,
  `CarrierEqError::{Contradicted, Unsettled}`, `fn definite`.
- **Contradiction types that survive and are reused:**
  `ContactRefusal::Contradicted { diag }`; `BooleanError::ContactContradicted`
  (now only for a glued tangency); `ValidationError::ContactContradicted`
  at rest. Door 1 lives on as `rest_pair_verdict`, read by the at-rest
  census and the glue door's tangency.
- **22 of the 35 held arms are retired:** carrier_eq `definite()` ×3 and
  `unsettled` ×2; mod.rs `sense_contradiction` ×3, the seam-departure
  label, `seam_senses_aligned`, `tangent_rim_refusal` ×7; vtxfac
  `bool_sector_coplanar` (no hand mint left in vtxfac.rs); rim_wedge's
  spline-chart exemption, corner/tangency mix and `Split` arm; flush.rs
  (the file is gone).

**The 13 survivors:**

| arm | now | mints | path |
|---|---|---|---|
| I1 `contact_rest_senses_opposed` | contact_verify.rs `rest` closure | `Contradicted{INVALID}` | (b) for curved kinds; planes read sense through `decide_nonzero`, no margin: unclear |
| I2 `contact_rest_ladder_invariant` → `contact_rest_one_carrier` | contact_verify.rs | `Contradicted{INVALID}` on `Distinct` | unclear: `Distinct` from kinds differing is (b); from a datum decided apart it is (a) only if geom-brep `IdentityLadder::Apart` carries its margin |
| I3 `contact_tangent_on/off_N` | contact_verify.rs `on_surface` | the decided margin (`decide_reported`) | (a), the margin is in hand |
| I4 `contact_tangent_opposed` | contact_verify.rs | `Contradicted{INVALID}` | (a): `decide` drops the margin |
| I5 `contact_tangent_independent` | contact_verify.rs | `Contradicted{INVALID}` | (a) |
| I6 `contact_tangent_parallel` | contact_verify.rs | `Contradicted{INVALID}` | (a) |
| I7 `contact_tangent_second_order` | contact_verify.rs (`residue.err().unwrap_or(INVALID)`) | `Escalated{INVALID}` | a gate shape: `decide_positive` carries it |
| `tangency_conformal` | mod.rs `verify_tangency` | `TangencyFailure::Contradicted{INVALID}` | (a): `carrier_pair_decided` has the margin, `carrier_pair_relation` drops it |
| `NotTangent` | mod.rs `verify_tangency` | same | (a), but geom-brep `locus::NotTangent` drops the margin |
| rim_wedge `seam_cover_unread` | rim_wedge.rs | INVALID | (b) |
| rim_wedge `seam_traversal_unread` | rim_wedge.rs | INVALID on a decided Zero | a gate shape (`decide_nonzero`) |
| merge `merge_pair_extent` ×2 | merge_faces.rs (pair_reach, carriers_continue) | `MergeCoplanarError::Escalated{INVALID}` | (b); precedent `unreadable_extent` → `InvalidInput` |

**The non-held sites:**
- **C20** (`CoincidenceMeasure::decide`): no hand mint left. The funnel's
  `ladder::decided` files a non-finite margin as `Blocked`, then `Unreadable`.
- **C21** (`coincident_as_declared`, now `coincident_over_extent`):
  **not settled by the glue door.** `pair_door_reading` decides every
  datum Zero, then the sum over the extent decides nonzero. It is wrapped
  `Undecided` with the decided margin, kept by `Glue::in_band` and quoted at
  the corner by `door_refusal`. `carrier_eq`'s `Undecided` doc now admits
  "past it where the bound stands off". It does not arise in a fold step
  (`decided_glue` returns early).
- **C22** (`unglued_coincidence`): **not settled.** The door keeps no
  `Distinct` and no unread pair. A corner re-reads such a pair over its own
  arm (recl.rs, vtxfac.rs), and a Zero there with no `door_refusal` ends
  here with the decided Zero margin. The two readings are of different
  extents, not a contradiction. README C4 keeps the corner reading. No
  `Verdicts::Given` guard was found in those readers, so a fold step's
  corner may reach it (not measured).
- **C24** (sweep `battery.rs` `measured`): unchanged; honest poison,
  waiting on the seal's public poison reading.

**New sites of the same shapes on the glue door's ground** (not in the
2026-10-10 census):
- carrier_pair.rs `torus_pair_witnessed`: ring apart against witness
  Zero, giving `Undecided` with a decided margin. This is C21's shape.
  CONTACTHOLD's `the-torus-witness-reads-only-vertices-two-tori-share` is
  on the same site.
- carrier_eq.rs `off_ring`: a torus convention decided Zero or Negative,
  filed `Unreadable` with the decided margin. It ends `PoisonedCarrierDatum`,
  a decided reading under a poison label.
- mod.rs `tangent_pair_at_rest`: `NotTangent` → `Contradicted{INVALID}`.
- contact_verify.rs `contact_tangent_overlap`, `tangency_sides_mixed`
  (the samples' decided senses disagree) and `contact_tangent_gap`.
- **The glue door decides a tangency twice:** `tangency_reading` throws
  every `Contradicted` payload away (glue.rs, `Ok(false)`), and
  `verify_glued_contacts` re-reads the same pair on the same inputs.

**Consumers.** I1 and I2 reach users only through the at-rest census. I4
and `overlap` reach them only through `tangent_table_at_rest`, whose census
dispatches on predicate strings (census.rs), so re-typing them re-pins that
dispatch.

**The two ref rows:**
- `work/twice/decided-coincidence-carries-a-synthetic-invalid-margin` (P3):
  its original reach is gone. Its surviving overlap is C22 and I1–I7.
- `work/sector/contact-gate-readers-drop-the-arm-verdict-or-mint-invalid`
  (P3): its seven are I1–I7. Its census-dihedral half was not checked.

**Split for dispatch:**
- **Mechanical:**
  - carry the margins at I3–I6, `overlap`, `gap` and `tangency_conformal`;
  - route I7 and `seam_traversal_unread` through their gates;
  - type the structural arms (merge_faces ×2, `seam_cover_unread`);
  - decide the glue door's tangency once.
- **The design question:** what a reading carries when two decided
  readings disagree, or when a decided reading breaks a convention. That
  covers C21, `torus_pair_witnessed`, C22, `off_ring`, `sides_mixed`, the
  margins `IdentityLadder::Apart` and `NotTangent` drop, and I1 for planes.

## Overlap

TANG's `decided-coincidence-carries-a-synthetic-invalid-margin` is
C22's payload question together with the held contradictions. Claim it
(by `git mv`) when this unit is dispatched, or close it into this one.
SECTOR's `contact-gate-readers-drop-the-arm-verdict-or-mint-invalid`
holds `contact_verify`'s seven, and it releases them with stage 4.

`design: true`: what a surviving contradiction or glue disagreement
carries is a choice to weigh once stage 4 has said what survives.

## Slate triage

2026-11-01: moved from `work/cleave/` by CLEAVE's split on its semantic seams (Ev, in chat: CLEAVE measured 114 budget points against 30). SEAL holds the Indeterminate seal and the declared-contact rows in its way; id and body unchanged. (CLEAVE orchestrator)
