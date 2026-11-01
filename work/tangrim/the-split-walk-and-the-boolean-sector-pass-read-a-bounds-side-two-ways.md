---
id: the-split-walk-and-the-boolean-sector-pass-read-a-bounds-side-two-ways
kind: issue
title: entry_side and sectors::side_code read one bound's side of a plane two ways, and disagree at every first-order tie
status: dispatched
opened: 2026-10-13
priority: P1
cost: H
branch: tangrim/one-bound-side-reading
---


Found by the review of PR 4858 (branch `cleave/vertex-against-section`).

## What

Two readers decide which side of a plane one bound of a vertex's
sector leaves on:

- `splitting::entry_side`: the split's neighbourhood walk, which
  `vertex_section` also reads at both doors. A conic reads
  `split_conic_departure` and, on a first-order tie, falls through to
  `geom_brep::conic_bend_side` (second order).
- `boolean::sectors::side_code`: the Boolean's own sector pass. It
  levers the bound at its `Reach` with a sagitta charge, and a tie stays
  On or refuses.

So at every first-order tie they disagree by construction. On the poses
PR 4858 measured, the disagreement changed no outcome. But the Boolean
decides which faces a section enters at a vertex
(`vertex_section`) by one reader, and builds its sectors by the other.

## Owed

One reading of a bound's side, read by both: either `side_code` takes
`entry_side`'s second-order fall-through, or the reverse, decided on
which one certifies what it claims.


## Duplication triage (INTENT, 2026-10-26)

Classified on Ev's ask, via the CLEAVE note on INTENT's log, against main `e92fb951` and DESIGN `[one-way.places]` / `[one-way.readings]`. The four classes:
- **design signal:** DESIGN names no single home, so a design conversation is owed;
- **mechanical:** DESIGN names the home; delete the off-home copy and repoint its users;
- **D10 debris:** a D10 stage retires one copy;
- **not duplication.**

**Design signal.** **Cluster A, the vertex-neighbourhood reading against a surface.** Split and boolean each own run enumeration, the sector walk, the bound side, struts and site choice. The only shared ground is the Euler types and the loose `vertex_section.rs`/`sector_*` modules, and the boolean already reaches into `splitting`. `neighborhood.rs::entry_side` and `sectors.rs::side_code` disagree at ties. The canonical copy is hosted inside one consumer (`vertex_section.rs` imports `splitting::entry_side`). `[one-way.readings]` names only the crate (topo), not a module.

## Slate triage (2026-10-26, CLEAVE)

- Re-banded P3 → P1: two readers of one bound's side: two implementations of one underlying logic, P1 by `work/README.md` Priority.
- Moved to ONEHOME (`work/onehome/`) by `git mv`, id and body unchanged: the P1 track opened for CLEAVE's duplication rows; INTENT is sorting each into design signal or a mechanical deletion (`work/intent/log.md`, 2026-10-26), and the sort goes on the row. CLEAVE measured 270.5 budget points against 30 and split on its priority seam (Ev, in chat, 2026-10-26).

## Design (decided 2026-11-01, TANGRIM)

Weighed by two designers for `a-union-across-a-tangent-crescent-refuses-unbuilt`'s step 2, reconciled in two rounds to one recommendation, and decided by the TANGRIM orchestrator: it elaborates `[one-way.readings]` and CURVED-DESIGN C7's principle ("where first order ties exactly, classification descends one order") and changes no ratified decision (`docs/DESIGN-FORK-LOG.md`, the tangent-crescent row). Claimed from ONEHOME by TANGRIM; this row is the carrying unit.

**The flaw.** At least five readers answer "which side of this face does this bound leave on", and they disagree at first-order ties: `sectors::pierce_side` (vertex-on-face: first order, a curvature charge, one order down for a conic against a plane only); `sectors::side_code` through `pair_codes`, `recl::flank_key`, `wedge_classes`, `cone_side` (first order against the tangent plane, no curvature charge — `NO_CURVATURE` — never one order down); `splitting::entry_side` (first order, then `conic_bend_side`); the declared-tangency lump (`tangent_relative_side`, surface against surface); and in-plane bound against bound (`runs_into`, `crescent`, `slit::walls_inside`, `fate::corner_holds`). And `On` means two things: "the bound rides the face" to the coincidence rules and Program 15.10's lumps, but "ties at first order" in the sector passes — which is why a bound tangent to a curved partner face kills the crossing records the crescent's union needs.

**The final state.**

1. **One parting primitive in `geom_brep`**, beside `enters_material_order2` and `tied_germ_turn`: two bends along a shared tangent toward a side, `(κ₁ − κ₂)·û·l²/2`, with its lever and its witness. `runs_into`, `crescent`, `slit::walls_inside`, `fate::corner_holds`, `tangent_relative_side` and `tied_germ_turn` call it (closes `the-crescents-second-order-gap-is-spelled-three-times`, which counts three of these).
2. **One bound-side reading in `topo`**, called by `boolean::sectors` (vertex-pair and vertex-on-face passes, recl's flank readings) and `splitting::entry_side` alike. It takes the bound's reach (direction, curvature vector: a conic's, zero for a line) and a face datum at the vertex (outward normal and the surface, replacing `vtxfac::PierceDatum`'s `FaceShape` and lever) and returns `Off(side) | Tangent(side) | Rides | refusal`:
   - first order, `d̂·n̂` at the bound's reach, charged for the face's curvature as the vertex-on-face pass does today (so the vertex-pair pass gets the MAJ-1 charge too; closes `vertex-vertex-side-codes-take-no-curvature-charge-on-curved-sector-faces`): decided → `Off(side)`;
   - on a first-order tie, one order down: the sign of `(κ_b·n̂ − II_F(d̂, d̂))·l²/2` at the shorter of the bound's reach and the face's lever → `Tangent(side)`. A bisector has no bend; where its sector's face is tangent to the partner, that face's normal curvature stands in (today's `tangent_relative_side` arm, now this rule);
   - tied at that order too → `Rides` (the bound rides the face to the orders read); `InFace`, the coincidence, is minted from it at the carrier ladder, the one coincidence door, never by the sector reading;
   - an in-band second-order tie, a spline edge or face (no certified jet), or no witness → typed refusal.
3. **Certification of a `Tangent` verdict (M2).** The sign is the leading term's, decided off zero; the verdict stands only where a witness point of the bound at its reach (the edge's extent from below, or the capped reach) stands more than the band off the face's carrier on that side. Reason: a side verdict decides topology and must agree with every finite-reach reader of the same edge against the same carrier — the reduction's crossing layer (`reduce.rs` `interior`, `carrier_cross`, `carrier_touch::off_face`, the conic × quadric root door) and the result census (`[coincidence.booleans]`: a result holds no in-band pair). MAJ-1 already ruled this one order up (a first-order sign charged at the reach). Where a root door covers the edge and carrier kinds, the witness's offset is that door's residual, not a new distance reader; a conic against a plane keeps `conic_bend_side`'s certified lift as that arm, under the one rule; the shipped leading-term sites adopt the witness.
4. **Consumers.** The on-edge gateways (`recl`'s on-edge event, `sectors::along`, `insert::across_tangent`, the edge-on-sector and edge-on-edge rules) key germ attribution on `Tangent` and coincidence on `Rides`/`InFace`, not on `On`. `SideCode` survives for consumers that want only a side, read off `Off`/`Tangent`. CONTACTHOLD measured (2026-10-10) that descending `side_code` alone moved 15 sweep rows from building to refusing because these gateways keyed on the first-order tie: re-keying them is part of this unit, and the outcome diff runs over every tangent pose in the suite.
5. **Text.** CURVED-DESIGN C7's list of descents in `crates/geom-brep/README.md` (agent-written, `b6b347fc12`) is re-worded with the code to "a bound against any face through the one bound-side reading; an in-band second-order tie escalates". CONTACT-DESIGN C7 (the glued tangent lump) is unchanged; a declaration keeps its job and does not enable a bound's reading. `pierce_side`'s "no reading of the bound alone gives" clause is false for an edge and goes.

Declared tangency was weighed as the home and rejected: it certifies a face-pair tangency along a locus, while a bound's tie is a fact at a point, and the face-pair form reads the wrong object (worked: boss wall × plate top, the hole's rim reads `In` by its own bend, `Out` by the faces').
