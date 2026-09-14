---
id: loop-reparenting-euler-ops-leave-rows-certified-against-the-wrong-chart
kind: issue
title: kfmrh and ring_move re-parent a loop onto a face with a different chart; its pcurve rows keep their keys and lose their meaning, and tier 3 is silent whenever the target is planar
status: open
opened: 2026-09-14
pr: 2549
branch: topo/loop-reparenting-rows
---

Found by PR 2531's reviewers (R2 by execution on `kfmrh`, R1 by reading
`ring_move`) and re-measured by that PR's fix pass, which owed the
class receipt a row per loop-RE-PARENTING op.

**The mechanism.** A pcurve row is a curve stated in a FACE's chart, and
it is keyed on a half-edge. An op that moves a LOOP from one face to
another changes which chart every row on that loop is about, without
touching a single key. Two doors do it:

- `Body::kfmrh` (`crates/topo/src/euler_kill.rs`) — the second face's
  outer loop becomes a ring of the first;
- `Body::ring_move` (`crates/topo/src/euler_ring.rs`) — a ring moves
  between two faces of one shell.

**Measured** on the minted cylinder-wall sheet (the fixture of
`crates/topo/tests/split_edge_pcurve_rows.rs`, with the `mef` face put
on a fresh PLANE), `validate_pcurves` at `Band::linear(Tol::witness())`:

| op | what moves | after | tier 3 |
|---|---|---|---|
| `kfmrh(plane, cyl)` | the minted cylinder loop becomes a ring of a PLANAR face | 4 rows, all certified against the cylinder chart, on a face whose surface is a plane | `[]` — **silent** |
| `kfmrh(cyl, plane)` | the plane's rowless loop becomes a ring of the minted cylinder face | 4 rows + 4 rowless half-edges on one curved face | 4 × `MissingCache` — loud |
| `ring_move(ring of cylinder rows, PLANE face)` | the same rows again, through the other door | 4 cylinder rows on a planar face | `[]` — **silent** |
| `ring_move(rowless ring, CYLINDER face)` | a planar-origin ring onto a minted curved face | the curved face reads incomplete | `MissingCache` — loud |

So the two directions are not symmetric, and only one of them is
caught. Onto a CURVED face the target reads incomplete and tier 3 says
so. Onto a PLANAR face nothing is reported at all —
`validate_pcurves` skips a face whose surface does not `chart_mints`,
so rows that now describe a curve in a chart the face is not on are
accepted unmeasured. They are not stale by key and no walk will drop
them: `mint_pcurves` clears and re-derives, but a body that is never
re-minted keeps them, and every reader that trusts a stored row
(`props`, the tessellator, `chart_boundary`) reads a row about the
wrong surface.

**What the declared postures say today.** `pcurves.rs`'s
`staleness_posture::DECLARED` files both doors under `Neither`, with
`ring_move`'s note reading "ring surgery: re-parents a ring, mints no
half-edge" — true as written, and the reason the case was missed is
that no entry claims anything about MEANING. What a fix decides is
whether these doors should refuse, clear the moved loop's rows, or
re-certify them against the target chart (the third is
`split_edge`'s answer, and it is available here only where the door
has the bound).

Related: `work/trim/validate-pcurves-never-recertifies-a-face-it-finds-incomplete.md`
(the same pass, skipping passes 2 and 3 on an incomplete face) and
`work/topo/half-edge-minting-euler-ops-leave-a-minted-curved-face-incomplete.md`
(the minting half of the same surface).
