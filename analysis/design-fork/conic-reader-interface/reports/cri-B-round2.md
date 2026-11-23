## For Ev

**Decision (likely).** Code outside `conic.rs` reads where a conic is in only one way: its **position**, `stored + U·κ(θ) + V·σ(θ)`.
- `stored` is the one point the conic holds. `U` and `V` are two vectors.
- `κ` and `σ` are the parameter's two offsets from that point: `cos θ` and `sin θ` when the conic is held about its centre; `cos θ − cos θ₀` and `sin θ − sin θ₀`, formed without cancellation, when it is held about a point of its arc.
- The position is opaque. No reader sees the stored point, the offsets, or which placement the conic has.

**One named polynomial along a conic.** It lives in `geom`, is opaque, and is written in `κ` and `σ` about the same point. Readers build it from the position (a height, the part across an axis, a squared length, products). It replaces the private harmonic structs scattered across the code. It states numbers:
- value and derivatives with a rounding bound;
- range over a window;
- the magnitude its rounding is metered against;
- its half-angle coefficients about an anchor, as a question;
- for the first degree only, its extremes and roots, posed about the stored point.

`geom-brep` decides everything else: every band, sign, meter, the subdivision walk, the quartic ladder and every refusal.

**What goes and what changes.**
- `Conic::frame` is deleted.
- `along`, `semi_axis_vectors` and `unit_in_plane` fold into the position and the point-against-conic readings.
- `u_ref` and `v_ref` stay as principal directions.
- A conic made from a conic is a map on the conic: the existing maps, plus a homothety (a scaling about a point) and a concentric offset.

**The centre.** `centre()` returns a `Centre`: a datum, stored or derived, that cannot be added to a vector. It never panics.
- Decisions never read it. "Coaxial", "a parallel of", "on this surface" and "concentric" are predicates on the position.
- It is read only by the STEP writer and by a construction that mints a new datum from it (a surface about a circle's axis).

**Two conics compared: three questions, each with its own door.**
- **The same stored value:** `same_content`.
- **The same locus exactly, in either sense:** what `injective::one_conic` asks. It admits an opposed axis or seam, which `same_content` by its own doc does not answer.
- **The same locus within the tolerance band:** one conic's level along the other's position over the window in use.

**README.** This replaces the clause "Every reading of where a conic is goes through the conic", which agents wrote and you approved in passing on PR 7258:
> A conic's position is the point it stores and two vectors times its parameter's two offsets from that point, and every reading of where the conic is (at a parameter, along it, or of a point against it) is stated in that position. Its centre is a datum of the conic, stored or derived.

**The circle datum reads are one unit, landing before any construction holds a circle on its arc.** Today every circle is held at its centre, so a read left unconverted is correct on every circle that exists when it is written. After the first arc-held circle it would silently read a derived centre.

**Not promised: identical bits for centre-held conics.** What PR 7258 said, and what stays true, is that a centre-held conic is the curve about its stored centre. Whether its polynomial is held internally as Fourier coefficients or in offsets about the centre is the type's private choice, and the kernel measurement picks it.

**Expected pushback:** re-posing 79 circle reads for an arc-held circle that no construction mints yet.

## For the orchestrator

1. **Circle datum reads: I sign (iii).** C's argument moved me: staging the re-posing as the acceptance of a later unit cannot be enforced, because a forgotten read is correct on every circle that exists when it is written. That makes my (ii) unenforceable.
   - **Legitimate `Centre` callers:** the STEP writer (`step-export/src/writer.rs` `edge_curve`, circles and ellipses), and a mint of a new datum from a conic (a surface of revolution about a circle's axis).
   - Concentricity of two whole circles is a decision, so it is a predicate, not a caller.
   - I agree with A on keeping one disallowed-methods entry on `Centre::point()`, with a reason at each caller. Measurement (d) gives the full list.
2. **Two conics compared: I sign three questions.** I had said `same_content` alone, and I was wrong. C read `one_conic`'s docs: it admits an opposed axis and `u_ref`, and `same_content` disclaims the one-locus question. The band question is real at `contain.rs`, `rim_wedge.rs`, `ring_path.rs` and `pose.rs` (C's reading; I did not re-check them).
3. **The public surface.**
   - `along` and `semi_axis_vectors` fold in. Their amplitudes are paired with `cos`/`sin` of the parameter, which reads wrong under a general anchor. I withdraw my "direction-only".
   - `u_ref`/`v_ref` stay. "No reader pairs them with the parameter" is a convention, not enforceable: a reader can always form `theta.cos()`. With `along` gone, no door hands out amplitudes, so a violation has to be written deliberately.
   - `unit_in_plane` goes. On the branch it reads the derived centre on an arc-held ellipse (C's defect, which A confirmed). This should go on the readers row.
4. **The ladder's anchors: I withdraw `about(θa)`.** A re-based polynomial's constant is a rounded `P(θa)` held as a value: a rounded copy, and a charge where the README says re-pose. The polynomial answers its half-angle coefficients about an anchor as a question, evaluated fresh from the stored-point form. That is an evaluation, not a stored copy.
5. **Bits of a centre-held conic: a non-goal as a property.** Both bases are "about the centre", so the ratified sentence holds either way. Let measurement (c) choose. If no row moves, use one offset basis for both placements, which means one code path. If Fourier conditions better for a centre-held conic, the opaque type keeps it internally. In my round-1 report I had stated that the numbers move; withdrawn, it is the measurement's to say.
6. **The sentence: I sign (c).** It does not depend on where the geom/geom-brep boundary lands, which (b) does by naming roots. It also states what a conic's position is without alluding to an error. Of the three candidates I think it is the one all three of us can sign.
7. **The ask to Ev.** "A conic's position is stated about the point it stores, behind one opaque polynomial in `geom`; `frame` goes; its centre is a datum for the file writer and for new datums only."
   - Expected pushback: the size of the work, or doing the circle re-posing before any arc-held circle exists.
   - The answer to the second is item 1's argument.

Nothing new built or measured this round. I re-read nothing beyond the two reports and the writer check from round 1.
