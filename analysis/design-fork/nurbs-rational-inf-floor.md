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

**B (02:58Z).**
- Answer: one coefficient-ratio bound on dyadic Bézier pieces for every rational derivative bound, lower and upper, curve and surface. The quantity is a quotient of Bernstein polynomials of one degree, bounded by the min or max of `a_k/b_k`. It has no weight range, no sign split and no hull.
- Argument:
  - The surface pair-term floor is a strictly weaker reading of the curve's polynomial.
  - Measured: the ratio at 1 piece reads 90–98% of the true inf, and at 2 pieces 99%, where the pair term reads 0 past 2×.
  - The sup is exact on the written nets, against up to 3.9× for the pair term.
- Confidence: sure on the inf half; likely on carrying the sup.
- Rejected the framing: yes, in part.
  - There are three inf recipes; `offset_meters` is the third.
  - The two recipes are one polynomial read at two resolutions.
  - The loss is the grouping and weight division, not mainly the hull.
- Proposed changing ratified text: none governs this. It says the 2026-10-12 sup ruling is the orchestrator's and needs no Ev.

## Comparison

- **Convergent** on the recommended final state: the one Bernstein-quotient/ratio statement for both sides, curves and surfaces, read on pieces. Both reject knot refinement and "option one over w_max³", and both say no Ev is needed.
- **Differences (details, not the final state):**
  - Per-piece direction (a norm floor): A calls it load-bearing; B leans to it.
  - RationalPairTerm: A would route SSI enclose to the quotient form; B would keep it as the enclosure's carrier.
  - Surface pieces: A says 1 or 2×2 per cell; B says a D9 constant ≥ 2 per direction, measured.
  - offset_meters: A says it becomes a reader in time; B says it goes with the second-order row.
- **No reconciliation round.** The orchestrator ruled at about 03:00Z. The ruling is in `work/nurbs/log.md`.
