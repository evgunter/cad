---
id: producer-closing-mint-is-a-convention-with-thirteen-copies
kind: issue
title: every producer's closing pcurve mint is a prose convention spelled thirteen times, unenforced, and it launders a stale operand row
status: closed
opened: 2026-09-08
priority: P1
cost: M
branch: walks/finish-completes
pr: 7497
closed: 2026-11-25
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

**Amended by TOPO** (2026-09-29, PR 3160, `half-edge-minting-euler-ops-leave-a-minted-curved-face-incomplete`).
One premise the convention used to rest on is retired: `mev`, `mef` and
`mekr` no longer leave a complete face half-minted — they re-mint the
loops they rewire at the mint site, clear a face the closed-form lane
cannot mint, or refuse on a spline chart. All thirteen calls stay, and
this item's finding is unchanged by it: an operator never mints onto an
unminted face, so a producer's final pass is still what mints the faces
it builds, and it still re-derives what the producer's other doors
staled, dropped or left incomplete (`kfmrh`, `ring_move`, `kef`'s
same-chart merge, the chart-change drops, `mev_null` —
`PcurveMintError::MissingCache`'s list). The laundering in (1) and the
drop in (2) are the pass's, not the operators', and stand as measured.

## Staleness audit (2026-11-22, WALKS, origin/main 7709b0ea)

Partly overtaken. Now about 21 closing call sites in 15 files (new: mirror.rs, boolean/edge_join.rs, sweep extrude.rs; transform.rs no longer mints), each with its own `Pcurve` arm. A computed guard exists (`every_mutation_door_declares_its_pcurve_posture`, pcurves/staleness_posture.rs), but it covers only `&mut Body` doors in topo; value-returning producers are outside it. Consequence (2) is fixed (`carry_rows`, pcurves/mint.rs). Consequence (1) is answered at the public verb doors, which take `F: FinishedBody`, but not at construction-state doors on a raw `Body`. The stale prose list in pcurves.rs is corrected by WALKS's E-bundle. Weighed as fork F4.
## The question to Ev (2026-11-22, fork F4)

Two designers weighed it and converged after one round: the obligation moves off the producers into the doors that construct a rest type (`finish`/`leaf` at every scalar, and `FinishedBody::moved_by`'s reflection arm), and those doors COMPLETE the rows (drop dead, keep stored, derive missing) rather than re-mint them, so a stale construction-state row fails loudly at the finish. Producers lose their closing mints and their `Pcurve` arms; `a-joined-finisher-mints-the-whole-body-twice` closes with it. The binding sentence is the new `[one-way.places]` row in `docs/DESIGN.md` on the `[ev]` PR; the implementation waits for it.

## Decided (Ev, 2026-11-22, PR 7420)

"sounds great! the DESIGN.md edit could possibly be folded into the one about `finish` in general, but either way is fine." Completion approved; the binding sentence is folded into `[one-way.places]`'s at-rest validity row rather than a row of its own.

## Implemented (WALKS, `walks/finish-completes`)

The finish completes a body's rows (`crates/topo/src/pcurves/complete.rs`,
`complete_pcurves`), in one preamble every `AtRestPolicy::finish` and
`::leaf` runs: the required methods take a `Completed<T>` only that
preamble constructs. It asks tiers 1 and 2 first, drops rows on dead
half-edges, keeps every stored row, derives a rowless face whole
(`derive_face`) and fills a half-minted loop by the site mint's rule
(`walk_cycle` with the stored images and elements kept). A face that
refuses is left as found and its refusal reported beside the verdict's
findings. The reflection arm of `FinishedBody::moved_by` completes the
mirror's rowless image. Every closing mint and its `Pcurve` arm is
gone; two readers that read stored rows mid-pipeline (the result sort,
`crate::pieces`, and the shell's cavity questions) read them through
the same completion, strict, since neither has a closed-form route to a
spline face's rows. The PR body carries the sweep and the reds it
measured.

## Closed

2026-11-25: PR 7497 merged — producers stop minting to close; the finish (and the mid-pipeline feeds, through one helper) COMPLETES the rows: keeps every stored row and element, derives the missing, so a stale row is refused rather than replaced (DESIGN `[one-way.places]`, Ev on PR 7420); `mint_pcurves` is test-support only; a source row refuses an `AtRestPolicy` impl overriding `finish`/`leaf`.
