---
id: producer-closing-mint-is-a-convention-with-thirteen-copies
kind: issue
title: every producer's closing pcurve mint is a prose convention spelled thirteen times, unenforced, and it launders a stale operand row
status: open
opened: 2026-09-08
---



Measured by SHELL-9's R2 reviewer (PR #2223, 2026-09-08; probe rows on
`shell/9-r2-probes`, merged into the unit) and placed here by the
SHELL orchestrator. The pcurve posture table
(`crates/topo/src/pcurves.rs`, `staleness_posture`) says in prose
that every producer's final mint pass re-derives every row of the
body it returns, and nothing enforces it: the call is spelled
thirteen times — `topo/src/{replace_face.rs, offset_together.rs,
offset_axial.rs, merge_faces.rs, transform.rs, splitting/mod.rs,
boolean/ops.rs, shell.rs}`, `sweep/src/{revolve/mod.rs,
revolve/tube.rs, loft.rs, blend/surgery.rs}`,
`step-import/src/assemble.rs` — each error enum grows its own
`Pcurve { source: PcurveMintError }` arm with its own `Display`, and
`shell` went eight units without the call (found by a diagnosis
probe, not by CI; SHELL-9 added it). Two consequences the table does
not state. (1) A producer that re-mints launders its OPERAND's rows:
`mint_pcurves` clears the map before re-deriving, so an operand whose
map is wrong — a row attached to the wrong half-edge, a row missing —
fails tier 3 on its own and yet passes through the producer to a
tier-3-valid result (measured on `shell`: a vessel with one face's
row on another face's half-edge, two `LoopDiscontinuity` findings on
the operand, `shell` returns `Ok`; SHELL's own item
`shell-launders-a-stale-operand-row` pins the rows). Whether a
producer should gate its operand's rows, or whether "rows are the
producer's to re-derive" is the ruled stance, is a posture-table
decision — TOPO's — and every producer in the list inherits it. (2)
`mint_pcurves` drops a row it cannot re-derive
(`Certify(UnsupportedCarrier)` clears that face's caches) rather than
refusing, so a producer that TRANSFERRED such rows before its mint
was added now destroys them; reachability through the public doors
is not measured. Fix shape the reviewer proposed: a
`staleness_posture` row that is COMPUTED rather than described — a
door's posture asserted by a test that walks the producers and finds
the call — would have found the fourteenth. Where else to look: every
public door that hands back a body (`graft_disjoint`, `Body::revert`,
`split_edge`, `insert_voids` itself). Signed (SHELL orchestrator).

**Amended by SHELL-10** (2026-09-08, a doc-only edit from the SHELL
lane; the item is TOPO's and the finding above is unchanged). Two of
the thirteen have left the population: `topo/src/offset_together.rs`
and `topo/src/offset_axial.rs` now close with
`pcurves::mint_pcurves_of` over the solids their move set names, not
`mint_pcurves` over the body. Eleven copies, and the sentence "every
producer's final mint re-derives every row of the body it returns" is
now false of those two by design — each re-derives the rows of the
faces it wrote and asserts nothing about the rest, which is the
strongest claim a partial producer can make. The subset pass carries
its own weaker at-rest guarantee (it cannot reach a row whose
half-edge is dead, where the whole-body pass clears one); the
contradiction that matters for THIS item is that a fourteenth
spelling now exists, and a single enforced door would have to cover
both.
