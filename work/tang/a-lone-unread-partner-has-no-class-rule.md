---
id: a-lone-unread-partner-has-no-class-rule
kind: issue
title: A vertex whose lone partner reads nothing has no class rule, so naming refuses a crossing there
status: open
priority: P2
opened: 2026-10-15
---


Filed by `a-lone-unread-partner-leaves-its-vertex-edges-unclassed`.

## What

`crates/topo/src/boolean/vtxfac.rs` `pair_classes`: a vertex in one
pair whose partner reads nothing (`sectors::wedge_read`'s
`UnreadNeighbourhood`) gets no edge class. The boolean builds on and
lists it in `BooleanNaming::unread_partners`, and naming refuses
`NamingError::PartnerUnread` at a crossing there
(`crates/editor-core/src/names/emit_topo.rs` `senses_at`,
`emit_union.rs` `Flush::sense_against`), citing this item.

No rule reads such a vertex's edges against the partner's
neighbourhood:
- `OneFace`: the partner lies inside one smooth face;
- `NoBisector`: two faces, neither holding the subdivision bisector
  `wedge_read` reads a wedge by;
- `ConeUnread`: a polygon cone some edge of which no reference decides
  (`sectors::cone_side`'s `None`).

## Where it is reached

`NoBisector` is reached by real bodies: the shaft's vertices where a
shaft of the bore's radius meets a full-turn collar's rim
(`crates/sweep/tests/full_turn_bore_mate.rs`
`the_mate_records_its_unread_partners`;
`crates/editor-core/tests/a_mate_on_a_full_turn_bore_names.rs`, 26 of
32 poses, off the seam too wherever a shaft cap meets a rim). Naming
names those cleanly today, since a listed vertex is an operand's own
and takes that identity before any senses are read, so the refusal is
pinned only on constructed rows. Off the seam is no lever: an
off-seam cap at a rim lists too.

## The shape to give

A reading of the vertex's edges against the partner's neighbourhood
for each cause: a direction strictly inside the wedge where neither
face holds a bisector (one off the faces' shared edge, as the cone
reading's references are), and whatever decides the cone where its
references do not. Then `pair_classes` keeps rows, the list empties,
and the refusal has nothing to cite.

**Another reaching scene** (INTENT stage 6 PR B, 2026-10-29, while measuring). Start from `coincidence_door::a_stated_axis_proves_a_wall_pair_the_fold_does_not`'s document, then change the housing copy's spins from `X 0, Y 0.5` to `X 0.2, Y 0.3`. The copy is now tilted. The union of the housing copy and the mated shaft copy then refuses at `f64` with `Naming(PartnerUnread { neighbourhood: OneFace, operand: B, .. })`, beside a `volume_backstop_violation` escalation. The shaft is mated onto the tilted copy's bore wall axis, so the shaft and the bore are coaxial. The untilted scene builds.

**2026-11-01 (ONEREAD, PR 6247):** `OneFace` and `NoBisector` are gone with `sectors::wedge_read`. Every partner now reads through one gathered `link::Cone` (`boolean::vtxfac::cone_classes`), and the bore rims class (`full_turn_bore_mate::the_mate_classes_the_shafts_edges_at_the_rims`). A lone partner still reads nothing only where an own edge reads within the band of its faces. `UnreadNeighbourhood` is collapsed, and `NamingError::PartnerUnread` now renders as a precision limit, not a missing rule, and cites no row. What remains here is a band answer, so this row can close.
