## For Ev

**The decision asked.** Code outside the conic stops reading a conic's centre or frame to find out where the conic is.

**Terms.**
- **Placement**: one of the two ways a conic stores itself: about its centre, or about a point of the arc it serves (the *stored point*, at eccentric anomaly θ₀).
- **Position**: the conic's point written about the point it stores,
  `P(θ) = S + U·ξ(θ) + V·η(θ)`.
  - `S` is the stored point.
  - `U = a·u_ref` and `V = b·v_ref` are the semi-axis vectors.
  - `ξ`, `η` are the placement's own two functions of the parameter: `cos θ`, `sin θ` about a centre; `cos θ − cos θ₀`, `sin θ − sin θ₀` about a point of the arc, the first formed as `−(2 sin²(θ/2) − (1 − cos θ₀))` so nothing cancels near the stored point.
  - It is opaque: no reader sees `S`, θ₀ or which placement it is.
- **Polynomial along a conic**: a trigonometric polynomial in θ, made by composing a polynomial on space with the position. A height over a plane is degree 1. A sphere's, a cylinder wall's or a cone's quadric is degree 2. A torus's implicit is degree 4.

**What the conic offers code outside `conic.rs`, and only this:**
1. Its point at a parameter (`point_at` and the jets), as now.
2. Its position, and one named polynomial along a conic, in `geom`. Readers build the polynomial from the position. It replaces the ten or so private harmonic structs built about the centre today.
   - The polynomial **states numbers**: its value and derivatives with a rounding bound; its range over a window; the magnitude its rounding is metered against; its half-angle coefficients about an anchor, on request; and for degree 1, its extremes and roots, posed in whichever form is well conditioned for its placement.
   - `geom-brep` **decides**: which polynomial a surface asks for, and every band, meter, subdivision, ladder and refusal.
3. A point read against it: the point's parameter, its height over the conic's plane, and its level (inside or outside, the radial defect formed without cancellation). Each is read from the stored point.

**What goes, and what replaces it.**
- **Removed:** `Conic::frame`, `Circle::frame`, `Ellipse::frame`, and the conic arms of `Curve3::frame`. With them go `along`, `semi_axis_vectors`, `unit_in_plane`, `centred_at`, the public `Cover::arc(frame, …)` and `Curve3::circle_at(frame, …)`.
- **`u_ref` and `v_ref`** stay until the general-anchor unit renames or removes them. That unit owes it: it must not re-document them under the same name.
- **The centre** is a datum type, `Centre`: stored or derived, and it cannot be added to a vector. It has two doors, `axis_line()` and one door to its coordinates, both on the gated list with a reason at each caller. Its callers are reporters (the STEP writer, a sketch written back, a description shown to a person) and mints of a *new* entity's datum (a surface built about a circle's axis).
  - An edge's own description is not a new entity. The revolved-point scaffold is built from the axis that the file or the surface states.
  - A derived centre is rounded at the semi-axes' scale. So a STEP file, whose `ELLIPSE` must carry a centre, states an arc-held ellipse about `u·a` off the kernel's curve. That is the format's limit; the kernel does not decide anything on it.
- **A conic made from a conic** is a map on the conic: isometry, reversal, re-seam and scalar map (all existing), plus a homothety, a concentric offset and a re-seam at a point (owed).
- **Records** that carry a circle's centre, axis and radius hold the `Circle` itself instead.
- **Two conics compared** is three questions:
  - the same stored value (`same_content`);
  - the same locus exactly, in either sense (a door beside it);
  - how one conic lies against the other: the range of one conic's level along the other's position, whose sign pattern reads apart, nested, tangent or crossing, with "one locus within the band" its zero case.
- **Circle decisions** (coaxial, a parallel of a surface, on a surface, the same circle) become tests on the position. On a circle held at its centre, which is every circle today, that is the same arithmetic as now.

**Why.**
- The readers write what they read as a Fourier sum about the centre. Its constant term is the reading's mean over a turn, which is its value at the centre. For an arc far from its centre that sum cancels at the semi-axes' scale, however exact the centre is.
- Handing readers a better centre does not help. Posing the reading about the stored point does.
- **How the measurements are judged.** All runs are in the kernel, at f64, with ε = 1e-9 and 1e-12. Each is judged against the curve the stored numbers state, computed in 60-digit arithmetic.
  - The carriers are real `plane_cone_section` ellipses, semi-major axis `a` = 1e4 to 1e9 m.
- **First-degree roots** (a plane crossing the arc 0.3 to 3 m from the stored point), worst miss along the arc:
  - at `a` = 1e6: 4e-9 m today; 1e-9 m with only the constant taken from the stored point; 2e-15 m posed about the stored point (the parameter's own last bit is 9e-16);
  - at `a` = 1e9: 5e-6, 4e-7 and 2e-15 m.
  - Far from the stored point the three are alike (the angle's own last bit), as they should be.
- **A sphere beside the stored point**, with the residual formed about the stored point and fed to the existing certified subdivision: 456 of 480 poses certify at ε = 1e-9, against 261 today, with no wrong answer in 2,880 runs. The refusals left are a meter that reads the carrier's top speed `a` where the window's own speed is meant. The conic already answers that (`speed_over`).
- **The half-angle ladder's coefficients**, formed about its anchor from two evaluations of the position: `F(anchor)` is right to 4e-16 m (median), against 9e-5 m from centre harmonics. Re-basing only the constant fails, up to 64 m off. The ladder still escalates spuriously at 14 of 270 clean crossings even with exact coefficients; that is its own metric, already filed.
- **The class is live on main.** A sweep found three wrong answers at the public API, all of one kind: a decision made from an arc-held ellipse's derived centre, with no charge for its rounding.
  - `curve_pair::meet` answers "no meeting" for a line or circle through a point of the arc.
  - The plane door answers `Miss` for the ellipse's own plane.
  - `Curve3::on_carrier` answers `Off` inside the band's gap.
  - These verdicts predate the arc-held placement: the tree before PR 7203 gives the same ones, miss for miss.
  - No wrong body is built, because the edge certificate's size limit refuses such a carrier first.
  - A fix unit is building plain readings from the stored point now, in this direction.

**What changes for a centre-built conic.**
- Its stored numbers, its curve and `point_at` keep their bits.
- What readers compute from it may change. The measurement converted three readers for every conic, centre-held included, and ran the suite. Of 72,428 door calls at ε = 1e-9, 641 answered with different bits and 61 with a different verdict:
  - 49 refusals became answers that the suite's exact oracle accepts;
  - 4 went the other way: two are the probe's own double charge, and two are a half-angle quadratic that encloses a touch's two roots as one wide interval on the `Interval` lane, where today's crest ± half-chord keeps them apart;
  - 4 swapped one refusal for another.
- 5 tests failed: the two touch rows, one sliver escalation, and two pinned query counts that were not traced.
- So what you were told on PR 7258, that nothing changes for a centre-built conic, holds for the curve but not for every verdict read from it.
- **The final state.** Each question is posed in the form that is well conditioned for the placement, privately. A centre-held conic keeps crest ± half-chord for its first-degree roots, because a half-angle about θ = 0 has no reason to be preferred where no stored point lies on the arc. The verdicts that move toward answering are kept, each judged against an oracle when its unit lands.

**Not measured.**
- The `Interval` lane for arc-held conics.
- A wall or a cone in place of the sphere beside the stored point.
- The moved f64 bits judged against a truth.
- The cone and torus readers, converted.
- Whether a public build reaches the uncertified plane roots on a far section: `split` refuses such bodies past `a` ≈ 6e5 at ε = 1e-9.

**The README.** It replaces the clause "Every reading of where a conic is goes through the conic." That clause was written by agents and approved by you in passing on PR 7258. The new text:
> A conic's position is the point it stores plus two vectors times two functions of its parameter, and every reading of where the conic is (at a parameter, along it, or of a point against it) is read from that position. Its centre is a datum of the whole conic, stored or derived.

The `conic.rs` module doc changes to match. It currently says "a position is read as the centre", which contradicts the README today.

**Where I expect pushback.**
1. **Size against benefit.** This is 142 functions, and the certified lane's harmonic builders are rewritten, for a loss only far-centred sections show. The answer: that class gives wrong answers on main today, and the type replaces about ten private harmonic structs, each built about the centre.
2. **Verdicts on centre-built conics move**, mostly toward answering.

**Confidence.**
- Posing about the stored point is what recovers the digits: sure (measured).
- The shape and the `geom`/`geom-brep` cut: likely.
- The polynomial's per-placement forms (crest ± half-chord at a centre): likely; not measured.
- Circle decisions as tests on the position: likely.

## For the orchestrator

1. **Does any measurement change what I signed? No; it sharpens where first-degree roots live.**
   - They belong to the polynomial, posed per placement:
     - **arc-held, f64:** `h − 2P·sin²(φ/2) + Q·sin φ` with `φ` measured from the stored point, solved as a quadratic in `tan(φ/2)` with the stable quotient for the near root (M1's form iii);
     - **arc-held, `Interval`:** the same roots held as a middle ± half-gap in `tan(φ/2)`, so a touch's gap encloses from zero symmetrically. This is unsure and unmeasured;
     - **centre-held:** crest ± half-chord, as today.
   - M4's `Interval` loss came from forcing the `tan(θ/2)` form onto centre-held conics. That form is not the design's for that placement.
   - "No first-degree door certifies" does not move the cut. Certifying is `geom-brep`'s. The plane door and `ring_path::plane_roots` stay uncertified closed forms, now posed right.
   - `curve_pair`'s polish erasing the posing is fine: it re-solves on the evaluators.
2. **M2's windowed speed belongs to `geom-brep`'s meter**, reading `Conic::speed_over(window)`, which already exists (it sits on the trait today), in place of `speed_hi`. The same applies to the monotone and width rows. The cut does not move, and the Taylor bound may stay whole-turn, as M2 shows.
3. **Centre-held bits.** The final state is as in For Ev. Ev must be told plainly that verdicts move, with the counts. I keep "the evaluator's bits kept, the readers' not a goal". I now take C's point that the opaque type may hold a placement-specific form (Fourier, crest ± half-chord) for a centre-held conic where that form is better on a question. M4 shows exactly such a question.
4. **The three wrong answers change urgency and order, not the design.** They are the defect class the design removes: no door hands out the centre as a position. What the fix unit should **not** build, because it would have to be undone:
   - a public accessor for the stored point or its phase (the probe's `probe_stored`);
   - a first-harmonic struct with public `(c, p, q)` about the centre or the stored point; its height-along-a-direction should return an opaque value with range and roots, so the later polynomial subsumes it;
   - a charge for the derived centre's rounding as the fix. That is charging where the README rule is to re-pose.
   - Its three readings (a point's height, a foreign point's parameter and level, a height as a first harmonic) are exactly the agreed point-against-conic family and the degree-1 polynomial, so they carry straight into the final state.
5. **Measured wrongly, or judged against the wrong thing?**
   - M2's re-read of the slack meter uses the speed at the span's far end. That is a bound over the span only where the speed is monotone across it. `speed_over` is the bound to judge against. I expect the same conclusion.
   - M4's 641 moved f64 bits are compared only branch against probe, so neither direction is known. The note says so.
   - Sweep row 61: the double-double oracle `conic_oracle/exact.rs` takes `ellipse.frame()` as its datum, so for an arc-held ellipse it states a different curve from the evaluator. That is a test-side defect worth a row.
   - Sweep row 60: the STEP export of an arc-held ellipse is `u·a` off. That is inherent to the format; it should be documented, not fixed.
   - The 3 untraced M4 failures (one sliver escalation, two pinned query counts) should be traced before the unit lands.
