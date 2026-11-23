## For Ev

**Recommendation (likely).** Three parts.

1. **One statement of where a conic is, its position**: `stored + U·κ(θ) + V·σ(θ)`. `stored` is the one point the conic holds, `U` and `V` are two vectors, and `κ`, `σ` are the parameter's two offsets from that point (`cos θ`, `sin θ` about a centre; `cos θ − cos θ₀`, `sin θ − sin θ₀`, formed without cancellation, about a point of the arc). It is opaque: no reader sees the stored point, the offsets or which placement it is.
2. **One named polynomial along a conic**, in `geom`, opaque, written in `κ`, `σ` about the same stored point. A reader builds it from the position (height above a plane, part across an axis, squared length, products). It replaces `half_angle::TrigPoly` and the doors' private harmonic structs.
3. **`geom` states numbers; `geom-brep` decides.** The polynomial owns everything whose arithmetic depends on the placement:
   - sums and products;
   - its value and derivatives at a parameter, with a running rounding bound;
   - its range, and its derivatives' ranges, over a window;
   - the magnitude its rounding is metered against;
   - its tangent half-angle coefficients about an anchor;
   - for the first degree only, its extremes and its two roots, posed about the stored point.

   `geom-brep` keeps which polynomial a surface asks for, every band and sign decision, the noise and root-slack meters, the subdivision walk, the quartic ladder and every refusal.

**Why first-degree roots sit with the polynomial** (measured; a scratch probe at f64 against 80-digit arithmetic, not kernel code). An ellipse held on its arc, semi-axes 1e6 and 3e5 m, and a plane crossing it within 3 m of the stored point. The root's miss along the arc:
- 3.4e-10 m read from the derived centre;
- 1.7e-10 m with the right constant but solved as the doors solve today (crest ± half-chord, a difference of two angles of order 1);
- 5e-16 m posed about the stored point.

`harmonic_roots_as` meters that cancellation (`speed × rounding_charge(τ)`), so at that door the middle case is a refusal of valid geometry, not a wrong root. Either way the arc-held form pays nothing there until the solve moves. Degrees two to four need no such door: the certified answer is the subdivision, which reads evaluations and Taylor bounds.

**The centre.** `centre()` returns a `Centre`: a datum, stored or derived, that cannot be added to a vector. No panic.
- **Decisions do not read it.** "Coaxial with this axis", "a parallel of that surface", "on this surface" and "one locus within the band" are predicates on the position. On a centre-held circle they are today's arithmetic; on an arc-held one they are decided at the arc's scale.
- **`Centre` is read only to write the file and to mint a new datum** (a surface about a circle's axis). The STEP writer needs it for the one arc-held conic that exists today, the cone section's ellipse.
- **Your two sentences on PR 7258.** "Kernel bug if we choose the wrong one" is about a construction choosing which form to hold: the kernel chooses there, and a wrong choice is its bug. A reader does not choose; it is handed a conic whose placement is private. A centre door that panics on one placement makes the reader work or die by which construction made its conic. That is the cutoff on which form is used, moved to the reader.

**The rest.**
- `Conic::frame` is deleted. `along`, `semi_axis_vectors` and `unit_in_plane` fold into the position and the point-against-conic readings (`azimuth`, `param_in`). `u_ref` and `v_ref` stay as principal directions, which no reader pairs with the parameter.
- A conic made from a conic is a map on the conic: the existing ones, plus a homothety and a concentric offset.
- Two conics compared is three questions:
  - one stated value: `same_content`;
  - one stated locus, exact, either sense (what `injective::one_conic` asks): a content door beside it in `conic.rs`;
  - one locus within the band: one conic's level along the other's position.
- The symbolic scalar is `point_at`. `cover_over` and `extent_along` are sugar.

**README, replacing the last clause** (agent-written, approved in passing on PR 7258):
> A conic's position is the point it stores and two vectors times its parameter's two offsets from that point, and every reading of where the conic is (at a parameter, along it, or of a point against it) is stated in that position. Its centre is a datum of the conic, stored or derived.

**Unsure, and worth knowing before you rule.** Whether a centre-held conic's numbers stay main's to the bit. PR 7258 told you nothing about a centre-built conic changes. An opaque polynomial can hold Fourier coefficients for a centre-held conic and offsets for an arc-held one, and so keep that; a published view cannot.

**Size, kept out of the ranking.** 142 functions in 57 files. The datum re-posing is one unit, and it lands before any construction mints an arc-held circle: after that an unconverted reader compiles and reads a derived centre with nothing red.

**Confidence.** The shape and the boundary: likely. First-degree posing inside the polynomial: likely, and it falls to the first measurement below. No panic: likely. Every circle decision re-poses on the position: likely to unsure (twelve of 79 read).

## For the orchestrator

Your summary of the agreement is right, with one addition: all three reports also agree that `mesh/sizing.rs` is a first-degree range and not a by-bit comparison.

1. **How much the value does. Moved.** I sign the opaque polynomial (`Trig` over `position()`), with first-degree posing inside it. Three things moved me:
   - `half_angle.rs`'s module doc: the certified answer for degrees two to four is `certified_subdivision` on evaluations and Taylor bounds, and the ladder's answers are dropped. My "it answers roots" over-reached above degree one.
   - `half_angle::TrigPoly` exists. My first report's "the kernel has no type" was wrong: it has one, Fourier-shaped, in `geom-brep`.
   - `harmonic_roots_as` charges `speed × rounding_charge(τ)`, so my "nothing red" was wrong for that door: it refuses.

   Not moved to the public `Stated` view. Its author says only review catches re-expansion, and the middle number is what a careful reader gets from it. It can be `pub(crate)` under the polynomial.
2. **The centre. Not moved on the panic; moved on scope.** Narrowing `Centre` to "building a new datum", with decisions re-posed, is stricter than my first report (which let coaxiality and concentricity verdicts read it) and better: the wrapper's `.point()` callers are then the whole list of legitimate uses. The panic report's own text concedes a typed refusal if a user file can reach it, and that datum reads must be posed through the arc once the import mints one; that final state is the predicates. Its "the ellipse gets no centre door" is wrong: `step-export/src/writer.rs` `edge_curve` writes an ellipse's centre placement.
3. **Circle datum reads.** Final state: predicates on the position, uniformly. They are the same numbers on a centre-held circle, so no reader branches and nothing waits on an arc-held circle existing. One unit (splittable by crate), ordered before `a-construction-whose-centre-runs-off…` mints the import's circle. It cannot be a property each later unit owes, because a forgotten read is correct on every circle that exists when it is written.
4. **Two conics compared.** Both reports saying `same_content` looked at `injective::one_conic`; mine looked at `contain.rs` `loop_circle`, `rim_wedge.rs` `same_circle`, `ring_path.rs` `misses_in_plane` and `pose.rs` `one_curve_gap`, which decide in a band. They are different questions. And `one_conic` is not `same_content`: it admits an opposed axis and an opposed `u_ref` ("the two parametrizations may differ"), where `same_content`'s doc says it never answers whether two statements describe one locus. Hence three doors. My "unsure" becomes likely.
5. **README.** As above. It drops my "nothing takes a centre or a frame…" and the others' "never a sum about the centre" and "never as harmonics about a centre": each alludes to the error.
6. **Wrong elsewhere, or to weigh.**
   - Both other reports keep `Conic::along`, one calling it direction-only. It returns `(a·û·n, b·v̂·n)`, amplitudes its caller pairs with `cos`, `sin` of the parameter; under a general anchor that pairing reads wrong. It belongs inside `position().height`. The same holds for any `u_ref`/`v_ref` read paired with the parameter.
   - `about(θa)` "with its rounding charged" hands out a polynomial whose constant is a rounded `P(θa)`. That is the local form the other report rejects as a rounded copy, and the README says re-posing, never charging. I would offer the ladder its half-angle coefficients about an anchor as a question, not a re-based value.
   - Both worked examples by hand agree with my measured sphere residual: 1.8e-4 m Fourier, 7e-16 m in offsets, at `a = 1e6`.
7. **Measure in the kernel first.**
   - (a) A real `plane_cone_section` ellipse (flat cone, semi-axis ≥ 1e4) against a plane, through `curve_pair`'s first-harmonic path and `ring_path::plane_roots`, against an exact oracle: as on the branch; with only the constant from the stored point; posed about the stored point. `pose.rs` has a Gauss–Newton polish (`Polish::closest`); I did not trace whether first-harmonic roots pass through it. If the middle case already lands at the arc's scale, or refuses nowhere the third certifies, first-degree roots need not be the polynomial's and evaluations suffice.
   - (b) The sphere-beside-the-anchor row through `conic_quadric_roots` with `ConicHarmonics` in offsets. If the subdivision still refuses, because its Taylor and noise bounds read whole-turn amplitudes, the windowed bounds must be the polynomial's too and the boundary moves further into `geom`.
   - (c) The suite with centre-held polynomials in the offset basis against Fourier. Any moved row means the polynomial keeps Fourier coefficients when centre-held.
   - (d) Not a measurement: classify all 79 circle functions as decision, construction or writer. More than a handful of constructions reading a centre off a conic would make the derived `Centre`'s meaning the main question, not a footnote.

Read this round: `half_angle.rs`'s module doc and `TrigPoly`, `injective::one_conic` and `one_stated_conic`, `same_content`'s doc, the ellipse arms in `blend/surgery.rs`, the writer's `edge_curve` header. Nothing was built.