---
id: a-cut-by-a-curved-seat-keeps-its-two-faces-held-apart-by-the-record
kind: issue
title: A cut whose tool's seat is curved, crossed by faces alone, keeps the seat's two faces inside one shell held apart by the record: a silent wrong body
status: open
opened: 2026-11-09
priority: P2
cost: M
design: true
refs: [a-declared-wall-is-read-only-by-a-vertex-crossing-it, a-cut-across-its-tools-seat-leaves-a-sliver-between-the-seats-two-sections]
---


Filed by SEAL's lane on PR 6613's fix pass, from R2's review (by
inspection, not executed: the lane did not build a coaxial fixture).

## What

On main, `cap − seat` (the bolt on the plate, its seat recorded, cut
from a cap that covers the whole of the bolt's foot) built a silently
wrong body. It was one solid of 11 faces, volume 0.168. Its floor was
the bolt's foot reverted and lying on the plate's top reverted, inside
one shell, held apart by the carried record, so tier 3 and 3′ passed
it. PR 6613 refuses that shape at the result gate
(`ops::Held::apart`): a shell that holds both faces of a carried area
record refuses `ToolSeatCrossed` in a cut by the record's holder, and
`TouchesItself` otherwise.

The gate reads only a record of two **planar** faces
(`crate::contact::area_record`), because a curved record may be a
tangency along a line, which tier 3′ reads inside one shell (`sweep`'s
`face_face_touch`, a rod resting on a plate, bridged). So where the
tool's seat is curved, the same body is still built:

- a shaft in a bore, the seat the shaft's wall and the bore's;
- cut from a block covering the whole seat, or one that crosses the seat
  by faces alone, as the cap does;
- with no vertex of the cut part on the seat, so the vertex pass reads no
  wall;
- the result keeps the bore's wall and the shaft's wall, both reverted,
  on each other inside one shell.

## Done when

A coaxial fixture (a shaft in a bore as a `⊔`, its seat recorded) cut
from a block that crosses the seat by faces alone either builds the
op's body or refuses typed. The gate must tell a curved area record
from a tangency without re-deciding the carriers (the carry's lineage,
or the record's own backing), and a row must pin the outcome.

## Dispatch (2026-11-10)

Branch `seal/curved-seat`. Review tier: **single, FULL**. The fix closes a silent wrong body, so correctness is at risk: a gate that refuses too much would refuse a rod resting on a plate, and one that refuses too little leaves the wrong body in place. The direction is the row's own ("the record's own backing"): the carried record's class says whether it holds two pieces apart over an area (`Rest`, one carrier) or along a tangent locus (`Tangent`, distinct carriers), with no carrier re-decided. The lane executes the coaxial fixture before anything else, since the defect was found by inspection.

## Lane findings (2026-11-10): reproduced, and the dispatch's mechanism does not exist

The lane stopped before building, as it should have. The dispatch claimed the carried record knows its class. It does not: the orchestrator had checked that `ContactPair { a, b, class }` exists, not that the gate is handed one.

**Measured on main `06840491a5`:**
- **The fixture:** a plate `bored_block_of_arcs(2, 2.0, 0.5, 0.4)` bored at (1, 1), r = 0.4; a shaft `cylinder_of_arcs_at(2, 0.4, (1, 1), z −0.5..1.0)` turned 90° about its axis, so its seams are off the bore's; the two grafted into one body of two solids. The block `[0.4, 1.6]² × [−0.2, 0.7]` covers the whole seat with no vertex on it, and is cut as `block − ⊔`.
- **The wrong body:** one solid, 16 faces, volume 0.374938, with both faces of all four records in one shell. Nothing refuses. The right body is two solids of the same volume, so a volume check cannot see it.
- **The planar control:** a square peg in a square hole refuses `ToolSeatCrossed { record: 0 }`.
- **Flush shaft (z 0..0.5):** one solid in two shells, 18 faces, volume 0.576; the right body is two solids. Each record's two faces lie in *different* shells of one solid, so a gate that checks shells misses it even once curved records are read. Filed as `a-seat-held-apart-across-two-shells-of-one-solid-passes-the-gate`.
- **Arcs aligned with the bore's seams:** refuses `CarryRefused { Unread }`, because two hand-made pairs touch only along a seam.
- The probe is kept at `/tmp/claude-0/-home-user-cad/1dc20da4-ec95-5f44-ba30-f41a85f77ec3/scratchpad/seal-seat-probe.rs` (session scratch; re-derive it from the numbers above).

**Why the class is not there:**
- In `Held::apart` (`crates/topo/src/boolean/ops.rs` ~5705), `carried` is `CarriedRecord<FfContact>`, and `FfContact` holds `{ a, b }` only (`boolean/mod.rs` ~775).
- `ContactRecords::carried()` (`records.rs` ~610) builds it from the record's cells.
- The class lives in the backing row's `relation` (`SameOpposite` / `SurfaceTangent`), which the boolean is never handed.
- The ratified `[topology.records]` (`docs/DESIGN.md` ~889; `crates/topo/README.md` ~828) says a record carries "no stored granularity". Whether a contact is a point, a curve or a patch "is read from the two cells' geometry by the census (CONTACT-DESIGN C3)".
- `area_record` has a second caller, `vtxfac::pierced_and_paired` (`vtxfac.rs` ~144).

**Reachability, and why P2:**
- No door mints a curved area record today. The at-rest census returns `CensusUndecidable` for every curved seat pair (filed as `work/atrest/the-census-cannot-record-a-curved-seat-between-two-solids.md`), and a union merges two curved faces glued at rest.
- So the wrong body is reachable only through caller-supplied `BooleanInputs`. Even then the ⊔ operand fails tier 3′ under its own records (five findings).
- It goes live when the census can record a curved seat. Settle the design before that lands.

**The design question (weigh as a fork):**
1. **Carry the class from the record's backing.** `FfContact` or `OperandRecords` gains it, and `carried()` (52 call sites, including `editor-core` `eval/wire.rs` ~3401) needs the rows, and the op history for `Carried` chains. Arguably this is stored granularity on the carried form, against `[topology.records]`.
2. **Read it from the two faces as the census does.** `contact::area_record` takes `census::confirm_face_pair`'s routing (`census.rs` ~10813–10849): two planes, or `carrier_pair_relation` reading one carrier or in band, is an area; distinct carriers is a point or line touch. The census, the gate and `vtxfac` share that one function. This matches the ratified clause, but it re-decides carriers in the gate (against this row's "without re-deciding") and needs the senses to tell `Continuation` from `Rest`.
3. **Surface kinds alone.** Ruled out: two cylinders can be one carrier or tangent.

Either way, the gate likely compares solids rather than shells (the flush finding).
