## EMBED run-probe report: measurements on embed/run-probe

**What this is:** 4 kinds of finished closed solids, built and run through tier 3, the census and the embedding question. Each has a narrow control at a = 1e-3.
**Probe commits:** `76c60cd198` (planar bodies) and `95f6f188d0` (B3), branch `embed/run-probe`, pushed. The commits carry no attribution trailers, as you asked.

**Code:** `crates/topo/src/run_probe_embed.rs`, a `#[cfg(test)]` module declared at the end of `injective.rs`. No production code was changed.
- Run with `cargo nextest run -p topo --lib --no-capture run_probe_`. It prints and asserts nothing; total runtime is under 8 s.
- **How the bodies were built:** a small greedy builder makes each body by Euler ops (`mvfs`/`mev`/`mef`) from named corners and outward-CCW face cycles.
  - Planar bodies: `plane_every_face` + `describe_as_intersections`.
  - B3: cone, cylinder and plane surfaces set explicitly. Every edge (line or arc) is restated as the `Intersection` of its two faces, then `mint_pcurves`.
  - No Boolean was used, so no build door was involved.
- **Doors run on each body:**
  - `validate_closed`
  - `topo::geometric` (tier 3 local battery; it supplies the facts the question reads)
  - `validate_geometric` (tier 3)
  - `finish` → `AtRestBody::validate_pseudomanifold` with empty records (the census, tier 3′)
  - `clearance::embedded`, on all faces and on face subsets {X, Y}, which reads that pair plus each face against itself.
- ε = 1e-9, K = 10, sine 5e-5, budgets depth 40 / 65,536 pairs.

**Bodies:**
- **B1** (as specified): S is L-shaped. It is narrow (a across) for x in [0,1] and wide to y = 1 for x in [1,2]. The narrow arm ends on the x = 0 end wall.
- **B1n**: the narrow arm sits in the middle (x in [0.5, 1.5]), with wide arms on both sides and two step walls. No vertex hangs in the band on an end wall.
- **B1t**: the narrow far edge tapers from P0, on the shared edge, up to (1, a, a·t). T holds that tapering edge and meets P at the vertex P0.
- **B3**: B1n revolved, (x, y) → (θ = −0.6x, r = 1 + y).
  - P is a planar annular sector; S is the flat cone z = (r − 1)·tan φ.
  - T is a 45° cone and the back wall is the cylinder r = 2; the walls are planes through the axis.
  - The in-band far boundary is the arc S|T (r = 1 + a, z = a·t, 5e-12 above P). It is a curved edge of two curved faces.

| body | tier 1–3 | census (tier 3′) | edge S\|P: class, run(S), run(P) | `embedded{S,P}` | pair owning the near-touch | `embedded{ALL}` |
|---|---|---|---|---|---|---|
| B1 a=1e-7 | Ok | Err: `VertexOnEdge{S5, end0\|P edge}` (+5e-12), `VertexOnFace{S4,P}`, `EdgeFaceOverlap{S\|T edge, P}`, `EdgeEdgeOverlap{end0\|S, end0\|P}`, 2× `BoundaryCrossesItselfUndecided` | Crease(Convex), 1.00000000125, 1.0 | **Embedded** (certified 4) | {T,P} `Crosses` d=7.07e-12; {step,P} `Crosses` d=5.0e-12; {end0,P} **Embedded** (end0 run 0.9999999) | `Crosses` P/step d=5.0e-12 |
| B1c a=1e-3 | Ok | Ok | same, 1.0 / 1.0 | Embedded | {T,P}, {step,P}, {end0,P} Embedded | **Embedded** (22, 72, 0, 0, 0, 50, 33) |
| B1n a=1e-7 | Ok | Err: `VertexOnFace{N1,P}`, `VertexOnFace{N0,P}`, `EdgeFaceOverlap{N0N1, P}` (−5e-12), 2× `BoundaryCrossesItselfUndecided{Degenerate}` | Crease(Convex), 1.00000000125, 1.0 | **Embedded** | {T,P} `Crosses` d=7.07e-12; {stepL,P}, {stepR,P} `Crosses` d=5.0e-12 | `Crosses` P/stepR |
| B1n a=1e-3 | Ok | Ok | same | Embedded | all Embedded | **Embedded** |
| B1t a=1e-7 | Ok | Err: `VertexOnFace{S4,P}`, `EdgeFaceOverlap{S\|T edge, P}` margin 0.0, `BoundaryCrossesItselfUndecided` | same | **Embedded** | {T,P} `Crosses` d=1.16e-10; {step,P} `Crosses` d=5.0e-12 | `Crosses` P/step |
| B1t a=1e-3 | Ok | **Ok** | same | Embedded | {T,P} **`Refused(Budget{Depth 40, a: T, b: P})`** | **`Refused(Budget Depth 40)`** at P/T |
| B3 a=1e-7 | Ok | Err: `VertexOnFace{N0,P}`, `VertexOnFace{N1,P}` (−5e-12), 2× `BoundaryCrossesItselfUndecided{Curved(Unreadable)}`. **No `EdgeFaceOverlap` for the arc S\|T over P** (B1n reports it for the straight version) | Crease(Convex), 1.00000000125, 1.0 | **Embedded** (certified 9) | {T,P} `Crosses` d=7.07e-12 (4,599 splits); {stepL,P} **`Refused(Budget{Depth 40, stepL, P})`**; {stepR,P} `Crosses` d=1.15e-11 | `Crosses` P/stepR d=1.15e-11 |
| B3c a=1e-3 | Ok | Ok | same | Embedded | all Embedded | **Embedded** (28, 340, 0, 0, 0, 312, 78) |

**Other runs tier 3 kept** (a = 1e-7, then a = 1e-3):

| edge | class | runs at a = 1e-7 | runs at a = 1e-3 |
|---|---|---|---|
| S\|T | Crease(Concave) | S 0.99999990, T 1.41421 | S 0.999, T 1.4128 |
| step\|S | Crease(Concave) | step 0.99995, S 1.5 (B1n) or 1.567 (B3) | step 0.99895 |
| end0\|P (B1) | Crease(Convex) | end0 0.9999999, P 2.0 | not shown |

S's run from S|T is its far arm, not its 1e-7 narrow width. The question still sets the lever at the lesser face's run, but at S|T both runs are about a metre.

**Did any door refuse each body, and where?**
- **B1:** refused at the census (vertex-on-edge on end0, vertex/edge-on-P) and by the question at the owning pairs (T–P, step–P, `Crosses`). Tier 3 passes. The pair holding the shared edge (S,P) certifies, and so does end0–P, whose edge has S5 5e-12 off it.
- **B1c:** nothing refuses; every door passes.
- **B1n:** refused at the census (vertex-on-face, edge-on-face) and by the question at the owning pairs (T–P, stepL–P, stepR–P, `Crosses`). Tier 3 passes; S,P certifies.
- **B1t:** refused at the census and by the question at the owning pairs (T–P, step–P). Tier 3 passes; S,P certifies.
- **B1tc (a valid body):** tier 3 and the census pass, but the question refuses it at T–P with `Refused(Budget Depth 40)`. T and P share only the vertex P0, and T's tapering edge leaves P at 5e-8 rad, so it stays within ε for about 20 mm. This is the vertex analogue of the sector sliver, refused rather than certified.
- **B3 (curved):** refused at the census, but only through the two in-band vertices; the arc edge over P is not reported. The question refuses at the owning pairs: T–P and stepR–P `Crosses`, stepL–P `Refused(Budget)`. Tier 3 passes; S,P certifies.
- **B3c:** nothing refuses.

**Notes:**
1. **Claim 1 held in every finished body I built:** the shared-edge pair S,P always certifies at run ≈ 1, and the in-band near-touch is always refused elsewhere.
   - By the census: vertex-on-face, vertex-on-edge, edge-on-face.
   - By the question at the owning pair: mostly `Crosses` (Violated), with distances 5e-12 to 1.2e-10 that are inside the band. In B3 one owning pair is `Refused(Budget)`.
   - **Tier 3 refused none of them,** including B1's end wall with a vertex 5e-12 off its own edge. The end0–P pair also certifies by its far arm.
2. **Claim 2 held for B3, but the census caught it only through vertices.** The in-band arc S|T over P produced no `EdgeFaceOverlap` (the straight B1n twin does). The census refused B3 only through the arc's end vertices.
   - I did not build a curved body with an in-band curved edge and no in-band vertex. With a constant-height arc on a cone I could not see how to taper the arc onto the shared edge.
   - So "the census catches it" is unmeasured for a curved edge whose ends are out of band.
3. **B1t's control is a valid body the question refuses** (budget, T–P at their shared vertex), while tier 3 and the census pass it.
4. **B2 was not built as specified.** My builder makes no ring loops, so the through-pocket was not built.
   - B1n is the notch form: S is U-shaped, and its notch rim N0–N1 lies in band over P.
   - It reads as B1 does: S,P certifies, and the census and the owning pairs refuse.
5. Every `embedded` call ran on the facts of `topo::geometric`, which tier 3 granted on all bodies. So I never needed `Geometric::fixture`, though `measure` falls back to it if tier 3 refuses.

**Still untried:**
- B2 with a true through-hole (needs a ring loop, `kemr`).
- A curved in-band edge with no in-band vertex.
- Pairs the census does not flag at all, such as a curved face over a curved neighbour, as listed in `census.rs`'s module docs.
