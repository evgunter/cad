fork: a-wall-seam-between-two-fits-has-no-section (what is a shelled loft's wall-wall seam, and what is owed so a lofted body shells?)
dispatched: 2026-10-10
byte: 242
mapping: Opus = A, Fable = B
protocol: 26db1af89e

## First reports (2026-10-10, before reconciliation)

- **A**:
  - Answer: `shell` moves every face of a solid at once through one general simultaneous door.
    - Every edge is the section of its two moved surfaces, through C5.
    - The crease seam is `Intersection{fit_i, fit_j}`: a seeded NURBS × NURBS section that runs corner to corner from the old seam. It is certified by plane × NURBS-style limbs on both operands.
    - Corners are the common roots of the surfaces meeting there; Cramer and the axial solve stay as fast paths.
    - The iso-row arm is kept only for same-chart and hold cases.
  - Argument: moving one face at a time puts the intermediate section on the fit's window edge (offset by t·cot φ), so the result depends on face order. Moving them together puts every convex seam inside both windows.
  - Exhaustiveness is NOT owed for this consumer. C5 splits each arm's service into seeded vs complete; the boolean asks for complete.
  - D2: no change.
  - Confidence: likely. Framing rejected: partly (exhaustiveness; the seam row is an at-rest fact).
  - Ratified text: C5 (service split, agent text from 3aacd6d28a, wants Ev's sign-off), O4 (agent prose), C3.
  - Units: junction rule → C5 split + seeded NURBS × NURBS + certify rung → general door → fit budget, quadrature, vase.
- **B**:
  - Answer: the same final state.
    - The seam is `Intersection{Approx_j, Approx_j+1}`, a rung-3 C2 section.
    - `shell` moves all charts together through one general door; the planar and axial doors are its closed forms; `replace_faces_offset` stays the single-face verb.
    - The iso-row arm fires only for self-shared images or where the neighbour holds the move, and `holds_the_move` gains NURBS-mover arms.
  - Argument: the same sequencing algebra (per face d·cot φ, which changes sign at 90°).
  - Exhaustiveness IS built into the arm: a product-domain boundary pass, a two-chart tube, and a hull-excluding subdivision.
  - D2: the NURBS-adjacent exemption should state its reason (the loft seam is a shared domain side, stated exactly by the chart image).
  - Confidence: likely. Framing rejected: partly (the description asymmetry picks the refusal; the iso-row arm sits outside the door's discipline).
  - Ratified text: C5 (add NURBS × NURBS), C2/C3 (two-chart tube and boundary pass), O4 (agent text), D2 (the exemption's reason).
  - Units: arm (H) ∥ general door (H; can land first, with the seams refusing until the arm lands) ∥ narrow the iso-row arm (M, first if possible) → fit budget.
- Agreement: yes on the final state: crease seam = fit × fit section, all charts moved together, iso-row arm narrowed, fit budget a separate gate.
- They differ on:
  1. whether the NURBS × NURBS arm owes exhaustiveness (A: a seeded service, no; B: yes, product-domain);
  2. whether D2's exemption text changes (B yes, A no).
- Arithmetic slip: B's together-offset is d·tan(φ/2); A's is d·cot(φ/2). cot is right (→0 as φ→π). Not decision-bearing.

## Reconciliation

- **Round 1.** Each designer was shown the other's first For Ev section and asked about exhaustiveness and D2.
  - A moved to complete with no split, plus a D2 carve-out.
  - B moved to seeded with the split, and called D2 optional.
  - B corrected tan to cot.
  - That is a CROSSOVER on both points.
- **Round 2.** Each designer was shown the other's round-1 revision and asked what moved it, whether its own first argument was answered, and what the question underneath is.
  - Both returned to their first positions, each naming the question underneath.
  - A: completeness is evidence for the body's claims (boolean, clearance), not the edge's. So the arm is seeded and C5 rows state seeded or complete. No D2 change now.
  - B: C5 is one total table whose answer is the section, whoever asks. Completeness is the arm's definition and selection is the consumer's. A per-consumer service axis is the contract-per-consumer shape that "no runtime fallback" forbids. D2 gets the shared-row carve-out.
  - Each read the other's crossed round-1 report and wrongly believed the two now agree.
- **Round 3.** Each designer was shown the other's real round-2 position and asked to check C5, C3 and D2 against the text.
  - Both agreed that C5's "no runtime fallback" is about rung dispatch, and that D2 needs no change now.
  - The split held on exhaustiveness: A leaned seeded (likely), B leaned complete (likely).
  - It went to Ev as PR 4515.

## Ev

- **Ev's follow-up (2026-10-10):** "i am compelled by B. is the final state it describes compatible with not doing extra computation that we know we will just throw away later?"
- **Both designers answered yes.**
  - A (Opus): a lazy complete arm, where the proof always runs and certification happens on demand. A leaned B after this.
  - B (Fable): a `Section` handle with `branch_at(seed)` and `all()`, where `shell` never runs the subdivision.
- **Ev's ruling:** "ok sweet, B then!" Recorded in fork-log row 106. Match: B.
