**Three designers weighed this and agree; their texts are below, unsigned, as A, B and C. Three things are yours to decide.**

**1. The README's conic clause.** "Every reading of where a conic is goes through the conic." (agent-written; you approved it in passing on PR 7258) does not say what "through" means, and that is what stopped the unit built on it. This PR replaces it with:

> A conic's position is the point it stores plus two vectors times two functions of its parameter, and every reading of where the conic is (at a parameter, along it, or of a point against it) is read from that position. Its centre is a datum of the whole conic, stored or derived.

What it commits the kernel to: a conic hands readers one opaque statement of where it is, about the point it actually stores (its centre, or a point of its arc), and one named polynomial along a conic is built from that statement; no code outside `conic.rs` takes a conic's centre or frame to find a position. *Rejected:* keeping the clause and adding one reading to `Conic` per reader (each still poses its arithmetic about the centre, or matches on the placement); and publishing the stored point with its phase (measured: a reader handed the right constant that solves the usual way still misses by 1e-9 m where the same root posed about the stored point misses by 2e-15 m, at a semi-axis of 1e6 m).

**2. The centre is a datum, never a position, and asking for it never panics.** It is a wrapped value that cannot be added to a vector, stored on a centre-held conic and derived (rounded at the semi-axes' scale) on an arc-held one. Its callers are a closed list: reporters into a format that is centre-form by its own definition (the STEP writer, a sketch arc written back) and mints of a new entity's datum (a blend surface about a rim's axis). No decision reads it. *Rejected:* a panic when an arc-held conic is asked for its centre, which one designer first proposed from your "kernel bug if we choose the wrong one" and withdrew: that sentence is about a construction choosing its form, and a panic at the reader would make a reader work or fail by which construction made its conic, the cutoff you ruled out, moved to the reader.

**3. Verdicts read from centre-built conics move.** On PR 7258 I told you nothing about a centre-built conic changes. That stays true of its stored numbers and its evaluator, to the bit. It is not true of what readers of degree two and up compute from it: they would read a rounding bound at the point where today they read one whole-turn noise. Measured by forcing every conic through a scratch polynomial over two crates' suites: 61 of 72,428 door calls change verdict, 49 of them a refusal becoming an answer the suite's own exact oracle accepts, 4 the other way; 5 of 6,514 tests fail, 3 of them untraced. *The alternative:* the polynomial keeps the centre-shaped numbers for centre-held conics, which keeps today's bits and today's refusals. The designers recommend letting the verdicts move, each judged against an oracle when its unit lands.

**Not asked of you** (sequencing, mine): the order in which the 79 circle readers are converted; which unit removes `u_ref` and `v_ref` from the trait (the end state all three sign is that they leave).

**Why now.** A sweep made on the way found three wrong answers on main at the public curve-level doors, all of one kind, a decision made from an arc-held ellipse's derived centre with no account of its rounding (`curve_pair::meet` says no meeting for a line through a point of the arc; the plane door says `Miss` for the ellipse's own plane; `on_carrier` says `Off` inside the band's gap). They are older than PR 7203, and through `split` no body is built on a section large enough to go wrong, because the edge certificate's size limit refuses the carrier first. A fix for those three is building now as plain readings from the stored point; it does not pre-empt this decision.

**Where the three texts differ** (detail, not the decision): who forms the quartic ladder's coefficients from the polynomial's jets (the polynomial on request, or `geom-brep` itself); the first-degree root form for an arc-held conic on `Interval`, which nobody has measured; two of the three flag that a STEP file cannot carry the arc-held placement, so an exported arc-held ellipse stands off the kernel's by the centre's rounding.

**Not measured:** `Interval` for any arc-held reading; a wall, a cone or a torus in place of the sphere; whether any moved f64 bit is nearer the truth.

The measurements, the class sweep, the defects filed and the seven rounds are in `work/oneround/log.md`, the entries of 2026-11-23 from 04:07Z; the measurement notes themselves are on `oneround/conic-harmonic-probe` under `analysis/oneround/conic-harmonic/`.

---

## Designer A

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

---

## Designer B

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

---

## Designer C

**Decision asked.** Three things, and one sentence in the README.
1. Code outside `conic.rs` stops taking a conic's centre or frame. A conic states where it is as one opaque **position**.
2. The kernel names one **polynomial along a conic**, in `geom`, built from the position. `geom` states its numbers; `geom-brep` makes every decision on them.
3. The centre becomes a wrapped datum, `Centre`, which no decision reads.

**Terms.**
- *Placement*: which point a conic stores, its centre or a point of its arc (PR 7258). Private to `conic.rs`.
- *Position*: `P(θ) = stored + U·ξ(θ) + V·η(θ)`. `stored` is the one stored point; `U`, `V` are two vectors. At a centre, `ξ, η` are `cos θ, sin θ`. On the arc they are `cos θ − cos θ₀, sin θ − sin θ₀`, formed without cancellation, with `θ₀` the stored point's parameter. No reader sees which.
- *Polynomial along a conic*: a reading composed with the position. Height over a plane is degree 1; the residual against a sphere, wall or cone is degree 2; against a torus, degree 4. It is a polynomial in `ξ, η` whose constant is the reading at the stored point.
- *Datum*: a number about the whole conic (its centre, its axis line), not a point of it.

**What was measured.** Scratch branch, f64. The carriers are cone sections minted by the real `plane_cone_section`, semi-axis `a` from 1e4 to 1e9 m, nothing axis-aligned. The truth is the curve the stored numbers state, in 60-digit arithmetic, with no kernel code in it.
- **A first-degree root through the plane door**, within 3 m of the stored point, `a` = 1e6:
  - 4e-9 m off as the tree stands;
  - 1e-9 m with the constant taken from the stored point and the solve unchanged;
  - 2e-15 m posed about the stored point (at most 1e-14 at every size).

  So the right constant is not enough; the solve belongs with the value. Far from the stored point (θ = 0.5 and 2) the three agree, at the parameter's last bit times the speed. That is the angle parameter's own limit, which this decision does not touch.
- **A sphere beside the stored point, the existing subdivision unchanged.** 480 poses, each a true double crossing. At ε 1e-9, 261 certify today and 456 with the residual formed about the stored point; at ε 1e-12, 125 and 216. No wrong answer in 2,880 runs. Today's refusals are digits lost in harmonics about the centre.
- **Wrong on main today.** Three public curve-level doors answer wrongly for a section held on its arc, each deciding on its derived centre with no account of its rounding:
  - `curve_pair::meet` certifies no meeting for a line through a point of the arc (from `a` ≈ 1e8 at ε 1e-9, from 1e5 at ε 1e-12);
  - the plane door answers Miss for the ellipse's own plane;
  - `Curve3::on_carrier` answers Off inside the band's gap.

  The tree before PR 7203 gives the same verdicts. No body was found built wrongly: `split` builds such a section only to `a` ≈ 6e5 at ε 1e-9, because the edge certificate's size limit refuses the carrier first, and every vertex it built stood within 0.06 ε of the truth. A fix for these three is being built now, in this direction.
- **By reading, not run:** 22 of the 63 remaining ellipse readers decide on the derived centre unmetered.

**Not measured.**
- `Interval`, except where the suite runs it (below).
- A wall, a cone or a torus in place of the sphere.
- Whether any moved f64 bit is nearer the truth.
- How many pieces the subdivision walks (its cost).
- The 22 suspects beyond the three that were run.

**Who does what.**
- **`geom`, the polynomial:**
  - sums and products;
  - its value and derivatives at a parameter, each with a rounding bound read at that parameter (in the probe that bound held at every point checked, worst error 0.36 of it, where today's one whole-turn noise is what refuses);
  - a whole-turn bound on its top derivative (enough in the sphere run);
  - its range over a window;
  - at degree 1, its extremes and roots.
- **`geom`, the conic:** its speed over a window (`speed_over`, which exists). Re-reading the logs, every root-slack refusal left at ε 1e-9 passes at the span's own speed; the meter reads the whole turn's top speed today.
- **`geom-brep`:** which polynomial a surface asks for; every band, sign and meter; the subdivision; the quartic ladder; every refusal. The ladder forms its five coefficients from two of the polynomial's jets (measured: right to 4e-16 m in the median, against 9e-5 m from centre harmonics). It asks the polynomial nothing else.

**First-degree roots: one closed form per placement.**
- **At a centre:** the crest and half-chord main uses, unchanged.
- **On the arc:** measured from the stored point, a quadratic in the tangent of half the angle from it.
- **Why not one form:** the suite run that forced the arc's form onto centre-held conics made two `Interval` rows that built escalate (a touch's two roots enclosed as one wide interval).
- **On `Interval`:** a conic on its arc may state both forms and keep the tighter, since each is a sound enclosure. That is by argument, not measured.
- **No first-degree door certifies its root today.** The plane door returns a closed-form root unchecked and its callers split the edge there. That is `geom-brep`'s to fix; this decision gives it a root good to the parameter's last bit to certify.

**The centre.** `centre()` returns a `Centre`: stored or derived, never panics, and cannot be added to a vector.
- **Two doors only:** `axis_line()` and one coordinate door, both on a gated list with a reason per caller.
- **Callers, a closed list:** reporters into a format that is centre-form by its own definition (the STEP writer, a sketch arc written back) and mints of a new entity's datum (a blend surface about a rim's axis).
- **A circle's own edge description is not on the list.** It is built from the conic, or from the axis of the surface the edge lies on.
- **Decisions read the position.** Of the 79 circle readers, 39 decide something (coaxial, a parallel of, on this surface, one circle). Each is a test that a polynomial is constant or zero along the conic, which is today's arithmetic on a centre-held circle.

**Also in the final state.**
- **Off `Conic`:** `frame`, `along`, `semi_axis_vectors`, `unit_in_plane`, `u_ref`, `v_ref`. `axis` stays. The principal frame is asked as `major_frame()` / `section_frame()`, which state the parameter at their vertex.
- **A point against a conic** is three readings from the stored point: its parameter, its height over the conic's plane, its level (inside or outside).
- **A conic made from a conic** is a map on the conic: the existing ones, plus a homothety, a concentric offset and a re-seam at a point. Seven records that carry a circle as centre, axis and radius hold the `Circle`.
- **Two conics compared** is three doors: one stored value (`same_content`); one stated locus exactly, either sense; one conic's level along the other's position, whose sign pattern is apart, nested, tangent or crossing.

**What changes for a centre-built conic.** PR 7258 told you nothing about it changes.
- **Still true of the curve:** its stored numbers and its evaluator are untouched, by construction.
- **Still true of first-degree readers:** they keep main's form.
- **Not true of readers of degree 2 and up.** They read a bound at the point where they read one whole-turn noise, and verdicts move. Measured by forcing every conic through the scratch polynomial over the `geom-brep` and `topo` suites (6,514 tests, one fuzz seed), 57 of 4,252 sphere and wall door calls change verdict:
  - 49 from a refusal to an answer the suite's own exact oracle accepts;
  - 4 the other way (2 of them the scratch's double charge on `Interval`);
  - 4 one refusal for another.
- **Three more failing rows are untraced:** one `Interval` escalation on a sliver, and two pinned query counts.
- None of the moved numbers was judged against an independent truth.

**README, replacing "Every reading of where a conic is goes through the conic."** (agent-written, approved in passing on PR 7258):
> A conic's position is the point it stores plus two vectors times two functions of its parameter, and every reading of where the conic is (at a parameter, along it, or of a point against it) is read from that position. Its centre is a datum of the whole conic, stored or derived.

**Order.** The fix under way builds plain readings on `Conic`, each from the stored point; under this decision they are sugar over the position. The remaining readers are converted before the edge certificate's size limit is lifted for a conic held on its arc; otherwise the 22 suspects become reachable through `split`.

**Pushback I expect.**
1. **The machinery against the need:** only sections far from their centre gain, and none that large is built today. My answer: three wrong answers at public doors; 261 against 456; and the type replaces about eleven private harmonic structs.
2. **Centre-built conics' verdicts move**, where PR 7258 said nothing changes.

**Confidence.** The position, the polynomial and `Centre`: likely. The cut: likely for a sphere, unsure for a cone or torus. First-degree form per placement: likely at f64, unsure on `Interval`.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

https://claude.ai/code/session_01EdpJs4dRmhgaKbdqQzfNmw
