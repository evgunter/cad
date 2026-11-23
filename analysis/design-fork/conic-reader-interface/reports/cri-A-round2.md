## For Ev

**Decision (likely).** A conic states where it is in one way: its **position**, `stored + U·κ(θ) + V·σ(θ)`.
- `stored` is the one point the conic holds; `U` and `V` are two vectors.
- `κ` and `σ` are the parameter's two offsets from that point. About a centre they are `cos θ` and `sin θ`. About a point of the arc they are `cos θ − cos θ₀` and `sin θ − sin θ₀`, formed without cancellation.
- The position is opaque: no reader sees which placement it is.

Readers build **one named polynomial along a conic** (in `geom`, in `κ` and `σ`) from that position: a height, a part across an axis, a squared length, products. It replaces the private harmonic structs.
- `geom` states numbers: values with rounding bounds, ranges over a window, jets, half-angle coefficients about a requested point, and the first-degree roots and extremes.
- `geom-brep` decides: every band, meter, subdivision, ladder and refusal.

**Why.** The readers lose digits by writing the residual as cosines about the centre. A scratch probe at f64 (not kernel code), on an arc a megametre from its centre, read 1.8e-4 m off that way and 7e-16 m about the stored point. A kernel measurement is running now.

**The rest.**
- `Conic::frame`, `along`, `semi_axis_vectors` and `unit_in_plane` go.
- `axis` stays, and `u_ref` and `v_ref` stay as principal directions.
- A conic made from a conic is a map on the conic. A homothety and a concentric offset are owed.
- **The centre is `Centre`:** a datum, stored or derived, that cannot be added to a vector. It never panics.
- **Who reads `Centre`:** only the file writer and constructions that mint a new datum (a surface about a circle's axis).
- **Circle checks** (coaxial, parallel of a surface, on a surface, same circle within tolerance) are decided on the position. That is the same arithmetic on every circle stored at its centre, which is every circle today. They are converted in one unit, before any construction stores a circle on its arc.
- **Two conics compared is three questions:**
  - same stored value: `same_content`;
  - same locus exactly, either direction: a new door beside it;
  - same locus within tolerance over the part in use: one conic's level along the other's position.
- **A centre-built conic keeps main's bits** where it is the curve: its stored numbers and its evaluator. The derived polynomials' bits are not a goal.

**README** (replacing the agent-written "Every reading of where a conic is goes through the conic"):
> A conic's position is the point it stores and two vectors times its parameter's two offsets from that point, and every reading of where the conic is (at a parameter, along it, or of a point against it) is stated in that position. Its centre is a datum of the conic, stored or derived.

**Pushback I expect:** whether a new polynomial type and a rewrite of the certified lane are worth it beyond far-centred sections. The kernel measurement answers that. If nothing moves except those sections, the type still replaces about ten ad hoc harmonic structs.

## For the orchestrator

Your summary of the agreement is right.

1. **Circle datum reads: I sign (iii).**
   - C's argument is decisive against (ii): a read that is never converted is right on every circle that exists today, so nothing turns red when the first arc-held circle is minted. The conversion therefore cannot be the acceptance test of a later unit.
   - Legitimate `Centre` callers: the STEP writer (circle and ellipse placements), and constructions that mint a new datum about a conic's axis or centre.
   - Concentricity of whole circles is a decision, so it goes on the position.
   - Anything the 79-function classification finds outside that list is a defect, or a reason to revisit.
2. **Two conics compared: three questions.**
   - C checked `injective::one_conic`, which admits an opposed axis and an opposed `u_ref`, and `same_content`'s doc disclaims answering whether two statements describe one locus.
   - So the exact-locus door is a real, separate question.
   - The banded comparison sites C lists (`loop_circle`, `same_circle`, `misses_in_plane`, `one_curve_gap`) are the third question.
3. **What stays public on `Conic`.**
   - Fold `along` and `semi_axis_vectors` into the position (B and C).
   - `unit_in_plane` goes; `azimuth` and `param_in` serve its callers.
   - `u_ref` and `v_ref`: "no reader pairs them with the parameter" is a convention, not enforced. It stays correct only while the parameter is the eccentric anomaly from `u_ref`. If the general-anchor unit moves parameter zero to the anchor, as the README's "parameter zero there" reads, they must leave or be renamed. That decision belongs to that row; I flag it.
   - With `along` gone, pairing them with the parameter takes hand-written `semi_axes · cos`, which is visible in review.
4. **The ladder's anchors: I sign C.** The polynomial answers "your half-angle coefficients about `θa`" from its own stored terms, each coefficient with its rounding bound. It never hands out a re-based polynomial value: that value's constant is a rounded `P(θa)`, a rounded copy, against "re-posing, never charging". This matches what I proposed in my first report.
5. **Bits.** What is right: the curve is its stored numbers and its evaluator. Those do not change (`point_at` is already `stored + u·(a·κ) + v·(b·σ)` on both placements), so what Ev was told on PR 7258 stays true.
   - The polynomial is a derived reading, and its bits are not a contract.
   - Whether a centre-held conic's polynomial holds Fourier coefficients internally is the type's private choice, and measurement (c) should make it.
6. **The sentence: I sign (c) as written.**
   - (a), mine, is weaker: "harmonic" is an undefined word, and it omits a point read against the conic.
   - (b) says every reading "is a harmonic", which is false for `azimuth`. Its "never a term of a point" alludes to an error, against Ev's answer on PR 7410.
   - (c) stays true wherever the `geom`/`geom-brep` boundary lands.
7. **What Ev is asked.** The first two lines of For Ev, and the README sentence for sign-off. I expect pushback on cost against benefit. If measurement (a) shows the middle case (the right constant, the doors' own solve) already lands at the arc's scale, the first-degree roots leave `geom` and the type shrinks. I would report that as a narrowing, not a reversal.
