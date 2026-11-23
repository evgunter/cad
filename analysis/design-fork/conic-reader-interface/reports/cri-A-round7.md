## For Ev

**The decision asked.** A conic hands every reader one statement of where it is, about the point it actually stores, and stops handing out its centre and frame for positional reads. Readers build one named polynomial from that statement, and every decision about the arc is posed in that polynomial. The README clause changes to say this. A centre-built conic's curve and evaluator do not change, but several of its readings do, mostly from a refusal to a certified answer; details below.

**Terms.**
- **Placement.** Where the conic's one stored point stands: at its centre ("centre-held", every circle today and most ellipses), or at a point of its arc ("arc-held", today only `plane_cone_section`'s ellipse). The construction fixes it; no size rule picks it. Ratified on PR 7258.
- **Position.** The conic's point stated about its stored point:
  `P(θ) = S + U·κ(θ) + V·σ(θ)`.
  - `S` is the stored point; `U = a·û` and `V = b·v̂` are the semi-axis vectors.
  - `κ`, `σ` are two functions of the parameter that belong to the placement: `cos θ`, `sin θ` about a centre; `cos θ − cos θ₀`, `sin θ − sin θ₀` about an arc point at eccentric anomaly θ₀, the first formed as `−2·sin²(θ/2)` at the seam's vertex so nothing cancels.
  - It is opaque: no reader sees `S`, the phase or which placement it has.
  - `point_at` is already exactly this sum on both placements, so the evaluator's bits do not move.
- **The polynomial along a conic.** Built by a reader from the position: a height above a plane, the part across an axis, a squared length, products. So a plane gives degree 1, a sphere, wall or cone degree 2, a torus degree 4. It is one type in `geom`, written in `κ`, `σ` about the same stored point. It replaces about ten private, centre-shaped harmonic structs (`ConicHarmonics`, `ConicTorusHarmonics`, `CircleSphereHarmonic`, `half_angle::TrigPoly`, …).
- **The cut between `geom` and `geom-brep`.**
  - `geom` states numbers: values and derivatives with a rounding bound read at the point; ranges over a window; the magnitude its rounding is metered against; its half-angle coefficients about an anchor the ladder names (formed there, never a re-based copy); and, at degree 1, its extremes and roots, posed by the placement's own rule (below).
  - `geom-brep` decides: which polynomial a surface asks for, and every band, sign, meter, subdivision, ladder and refusal.
- **A point against the conic.** Three readings, each from the stored point: its parameter (`azimuth`, `param_in`), its height over the conic's plane, and its level (inside or outside, the radial defect formed without cancellation). Plus the parameter of a direction, for "the conic's point toward that".
- **`Centre`.** The centre as a datum, stored or derived (rounded at the semi-axes' scale on an arc-held conic), that cannot be added to a vector. It has two doors, `axis_line()` and one coordinate door, and both are gated with a reason per caller. Its callers are a closed list:
  - **reporters:** the STEP writer, a sketch arc written back to the user;
  - **mints of a new entity's datum:** a blend surface about a rim's axis, a section loop's turning axis.

  No decision reads it, and no restatement of the conic's own edge does. There is no panic: a reader does not choose the placement, so it may not fail by it.

**What the kernel measured** (f64, ε 1e-9 and 1e-12, K = 10; each judged against the curve the stored numbers state, in 60-digit arithmetic):
1. **The loss is the centre-shaped form, not just the centre read.** A plane root near the anchor of a real cone section, semi-axis 1e6 m:
   - 4e-9 m off as main reads it;
   - 1e-9 m with the right constant taken from the stored point but solved as main solves (crest ± half-chord);
   - 2e-15 m posed about the stored point.

   Up to 1e9 m the third stays at 2e-15 to 1e-14 while the first reaches 5e-6 m. Far from the anchor all three sit at the angle's own last bit. So posing belongs with the polynomial, and constant-only is not enough.
2. **A sphere beside the anchor**, with the residual formed in the stored-point form and fed to the existing subdivision: 456 of 480 poses certify at ε 1e-9, against 261 today. No wrong answer in 2,880 runs. Today's refusals come from the centre harmonics' own noise, 3.6e-3 m at a = 1e6, r = 1.
3. **The quartic ladder's anchor coefficients.** Formed about the anchor from two point jets, `F(anchor)` is right to 4e-16 m in the median. Re-basing the centre form's constant leaves it up to 64 m wrong. The ladder still escalates spuriously at 14 of 270 clean crossings even with exact coefficients; that is its own filed metric defect, not this design.
4. **Main answers wrongly today, at the public API** (found on the way): a line through a point of an arc-held section is answered "no meeting"; the ellipse's own plane is answered `Miss`; a point in the band's gap is answered `Off`. Each is a decision made from the derived centre with no charge for its rounding. The same verdicts stand before PR 7203. A reading pass marks 22 of the 63 derived-centre readers suspect.

   Through `split` no body is built at the sizes that go wrong, because the edge certificate refuses the carrier first. A fix is being built now in this design's direction.

**Not measured.**
- `Interval` for any arc-held reading.
- A wall or a cone in place of the sphere.
- The torus doors.
- The moved f64 bits of centre-held conics against a truth.
- The first-degree roots' callers past `split`.
- A touch (double root) on an arc-held conic at `Interval`.

**What changes for a centre-built conic.** Its stored numbers and `point_at` are unchanged to the bit, so "nothing about a centre-built conic changes" still holds of the curve. Its readers' arithmetic changes. Measured with every conic read through the polynomial, against the current suite:
- 61 of 72,428 door calls change verdict. 49 of those are a sphere or wall refusal becoming an answer the suite's exact oracle accepts; 4 move the other way.
- Last bits of roots move by up to 8e-11 rad (median 4e-15).
- 5 of 6,514 tests fail:
  - two are the first-degree solve rewritten as a quadratic in `tan(θ/2)`, which at a touch on `Interval` merges the two roots into one wide enclosure where crest ± half-chord does not;
  - one is a terminal-sliver escalation at `Interval`;
  - two are pinned query counts.

So the final state is not "convert everything to one form". The posing follows the placement, inside the polynomial, where no reader sees it:
- A centre-held conic keeps today's first-degree posing (crest ± half-chord), which is right at its own scale and right at a touch.
- An arc-held one is posed about its anchor.
- Both get point-local bounds at degree 2 and above, which is where the 49 verdicts that now answer come from.

This is not a cutoff: the construction fixed the placement, and the polynomial reads it.

**Also in the final state.**
- `Conic::frame`, `Conic::center`, `along`, `semi_axis_vectors`, `unit_in_plane` and `centred_at` (only tests call it) leave the public surface. `unit_in_plane` reads the derived centre on arc-held ellipses today.
- `u_ref` and `v_ref` stay for now. The general-anchor row removes or renames them (it may not re-document them under the same name) when parameter zero moves to the anchor.
- A conic made from a conic is a map on the conic: isometries, `reversed` and re-seams exist; a homothety, a concentric offset and a re-seam at a point are owed. Seven records that copy out centre, axis and radius (`Rim`, `MeetSegment::Arc`, `SphereCutIn`, …) hold the `Circle` itself.
- Two conics compared is three questions:
  - the same stored value: `same_content`;
  - one locus exactly, either sense: a door beside it;
  - apart, nested, tangent, crossing or one locus within the band: each conic's level along the other's position, by sign pattern.
- Circle decisions (39 of the 79 circle-reason functions, 12 more as a second use) become tests on the position. The order of that work is sequencing; the gate on `Centre`'s doors catches a forgotten read whichever order is chosen.

**The README sentence**, replacing "Every reading of where a conic is goes through the conic." (agent-written, approved in passing on PR 7258):
> A conic's position is the point it stores plus two vectors times two functions of its parameter, and every reading of where the conic is (at a parameter, along it, or of a point against it) is read from that position. Its centre is a datum of the whole conic, stored or derived.

The `conic.rs` module doc's "a position is read as the centre" goes with it.

**Where I expect pushback.**
1. **Is a polynomial type with its own arithmetic too much?** My answer: the kernel already has about ten centre-shaped copies of it, and the three wrong answers on main all live in their gaps.
2. **The STEP writer.** STEP's `ELLIPSE` is placed at its centre. For an arc-held ellipse the file states the curve through the derived centre, about `u·a` off the kernel's: 2e-10 m at a = 1e6, which is past ε at 1e-12. A file cannot carry the arc-held placement. Re-importing it gives a centre-held ellipse that is a different curve by that rounding. I would accept that as the format's limit, documented at the writer, not designed around.

**Confidence.**
- Position plus one polynomial, with the `geom`/`geom-brep` cut: likely.
- Posing follows the placement, with centre-held keeping crest ± half-chord: likely, because M4's `Interval` loss came only from rewriting the centre-held solve.
- An arc-held touch at `Interval`: unsure, not measured.
- `Centre` and its closed caller list: likely.

## For the orchestrator

1. **Where first-degree roots live now.** In the polynomial, posed per placement:
   - centre-held: crest ± half-chord, as main does, on both lanes;
   - arc-held: about the anchor (M1's (iii), `h − 2P·sin²(φ/2) + Q·sin φ`, φ from the anchor) on f64.

   For the arc-held case on `Interval`, the touch case is unmeasured. If it merges roots as M4's tan form did, the polynomial must offer the crest-about-the-anchor form at a touch. That stays inside the type and changes nothing else.

   "No first-degree door certifies its root" does not change the design. It is a defect of its own: the plane door's callers split edges at uncertified roots and are kept right only by `KernelLimit` (sweep §1). It belongs on a row, with the root's height off the plane as the quantity to certify.

   Nothing else I signed moves.
2. **The windowed speed (M2).** It belongs to the conic, and the reading exists: `Conic::speed_over(span)`. Derivatives read no position, so it is placement-free. The root-slack, monotone and width meters in `half_angle.rs` choosing `speed_hi` is `geom-brep`'s choice of a whole-turn number. The fix is to read the windowed one, so the cut does not change.

   Caveat: M2's "passes at the span's own speed" re-reads logged numbers; it was not a run with the meter changed.
3. **Bits.** Final state as in For Ev: evaluator bits kept; posing per placement inside the type; point-local bounds for both placements. Ev must be told that a centre-built conic's readings move (61 verdicts, 49 toward an answer, 5 tests). The text above does that, and does not claim "unchanged".

   Of M4's failures:
   - two are the tan-form rewrite my final state does not make;
   - two `Interval` losses are the scratch jet double-charging;
   - three are untraced (the sliver escalation and the two pinned query counts). Those three want a trace before the PR claims they are benign.
4. **The wrong answers.** Urgency and order change; the design does not. The class, an unmetered decision from a derived datum, is exactly what the position removes. The fix unit should NOT build:
   - (a) any public door exposing the stored point or phase (the probe's `Conic::probe_stored` shape);
   - (b) readings that return a centre-based `(c, p, q)` triple or Fourier coefficients;
   - (c) a charge for the derived centre's rounding in place of re-posing ("re-posing, never charging");
   - (d) more per-question `Conic` methods than the three point-against readings and a first-degree height.

   That height should be a type that becomes the polynomial's degree-1 case, not a tuple, so it is not undone later.
5. **Measured or judged against the wrong thing.**
   - M4's "toward" is the suite's oracle passing, not a truth. The 13 `Tangency → certified roots` cells did not judge the roots.
   - M4's 2.5e-2 move in `a_segment_across_a_thin_ellipses_vertex_meets_it_at_a_root` is large for "last bits" and was not explained. Trace it.
   - Sweep2 row 61: the double-double oracle (`conic_oracle/exact.rs::truth`) states an arc-held ellipse through `frame()`, so any test judging arc-held answers by it is judged against the wrong curve. File it.
   - Sweep2 row 60 (the writer) is in For Ev.
   - M1's door B shows the polish erasing the posing. That is right as measured, but it means door B's miss counts say nothing about the posing.
   - Row 22 (`one_stated_conic`'s exact compare of two derived centres) is a third-door case for the compare work.
