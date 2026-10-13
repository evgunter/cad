# Design fork: the rational lower-bound statement and its reach (NURBS)

- Rows:
  - `work/nurbs/rational-inf-floor-reach-refines-before-reading` (P2, design);
  - `work/nurbs/rational-inf-speed-has-two-recipes-curve-and-surface` (P3, design).
- Protocol: `5dedb0a41a`.
- Blinding byte: 44 (even), so Opus = A and Fable = B.
- Dispatched: 2026-10-13 02:37Z, at main `03a0fec33c`, with the same statement for both.
- Sessions: A (Opus) is session_01Sf5mna2WP1aQXhUPouz77T; B (Fable) is session_017sVtLpKp2c2yUWyve5ZYmt.

## First reports, recorded as delivered, before any reconciliation

**A (02:46Z).**
- Answer: one *Bernstein quotient bound* for both the inf and the sup, for curves and surfaces. On each Bézier piece, write the quantity as numerator over weight power, raise both to one degree, and bound by the min or max of `n_k/ω_k`.
- It replaces the pair-term sup and floors and the curve meter's divisor. The floor becomes a norm floor with its direction picked per piece.
- Argument: the pair term loses through the hull relaxation and the worst-weight divisor. The quotient pairs each numerator coefficient with its own weight coefficient. Measured: 145 of 150 nets enter against 0 of 150; the sup is 1.00× the truth against 3.4×.
- Confidence: likely. Soundness is sure.
- Rejected the framing: yes, in part.
  - The direction loss lives in the consumer (one `d` per surface).
  - "Option one" and knot refinement are both worse.
  - There are three inf recipes, not two: `offset_meters` is the third.
- Proposed changing ratified text: no DESIGN/README text governs this. It recommends replacing the orchestrator's 2026-10-12 sup ruling.
