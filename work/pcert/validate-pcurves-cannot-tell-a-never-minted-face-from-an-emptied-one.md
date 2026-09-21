---
id: validate-pcurves-cannot-tell-a-never-minted-face-from-an-emptied-one
kind: issue
title: validate_pcurves reads a face a door emptied exactly as it reads one never minted, so a drop that re-charters a whole loop is indistinguishable from a body the pass has not run on
status: open
opened: 2026-09-14
refs: [validate-pcurves-never-recertifies-a-face-it-finds-incomplete, S331, loop-reparenting-euler-ops-leave-rows-certified-against-the-wrong-chart]
priority: P0
cost: D
---

Found by both reviewers of PR 2549
(`loop-reparenting-euler-ops-leave-rows-certified-against-the-wrong-chart`),
each by execution, and filed by that PR's fix pass.

`validate_pcurves` (`crates/topo/src/pcurves.rs`) skips a face whose
boundary stores NO row: "a body that never ran the minting pass has
none, and the pass says nothing about it — absence is never a claim".
That reading was written for a body the pass has not run on. It is now
also the reading of a face a DOOR emptied: `Body::kfmrh`, `Body::mfkrh`
and `Body::ring_move` drop a moved loop's rows when the loop changes
chart, and where the destination's whole boundary is that loop the
face is left storing nothing at all. The two states are the same state
to this pass, and they are not the same fact: one is a body nobody has
minted, the other is a body a door has just re-chartered.

**Measured** on the suite of
`crates/topo/tests/loop_reparenting_pcurve_rows.rs`, `validate_pcurves`
at `Band::linear(Tol::witness())`, at PR 2549's head. Every row below
is a CURVED destination, so the pass is not being skipped for
`chart_mints`; it is being skipped for emptiness:

| door | destination | after | findings |
|---|---|---|---|
| `kfmrh` | a curved face carrying no rows of its own | `(0, 10)` | `[]` |
| `ring_move` | the same | `(0, 10)` | `[]` |
| `mfkrh` | a face promoted onto a fresh curved chart | `(0, 4)` | `[]` |

On the merge base each of those three left the rows in place and the
pass refused them, one `PcurveMintError::Certify` per row. The drop is
the right answer — the body no longer HOLDS a row about another
surface — and what it costs is the reading, every time.

**Why it is this program's.** The doors are TOPO's and are now honest:
they leave no row stated in a chart its face is not on. What is left
is a question about the PASS — that its silence has two meanings and a
caller cannot tell which — and the pass is TRIM's. It is the same
shape as
`validate-pcurves-never-recertifies-a-face-it-finds-incomplete` and
`S331` one step further out: that row is about a face the pass
measures PARTLY, this one about a face it does not measure at all.
An S331-shaped vacuous green, which both reviewers named.

**What would close it.** A reading that separates the two: a face on a
minting chart that stores no row is either a body the pass has not
run on, which is a fact about the BODY rather than the face, or a face
whose rows a door removed. Nothing in the body records which today.
The cheapest honest shape both reviewers reached for is a whole-body
premise — once ANY face of a body stores a row, a rowless face on a
minting chart is a finding — which needs no new state and would have
caught every row in the table above (the fixture's other curved panel
keeps its four rows through all three). Whether that premise is the
right one is this program's call.
