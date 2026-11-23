## For Ev

**Recommendation (likely).** A conic answers where it is in three ways, and hands out no frame:
1. **Its point** at a parameter, through `point_at` and the jets, as now.
2. **Its point as a harmonic**, one new value named in `geom`, inside the conic's module tree: `stored + U·ξ(t) + V·η(t)`. Here `stored` is the one point the conic holds, `U` and `V` are two vectors, and `ξ`, `η` are the placement's own two functions of the parameter (`cos t`, `sin t` about a centre; `cos t − cos t₀`, `sin t − sin t₀`, formed without cancellation, about a point of the arc). The placement and its phase are private.
3. **A point read against it**, through `azimuth`, `param_in` and unit coordinates, each read from the stored point.

**What the harmonic does, and where `geom-brep` takes over.**
- The harmonic is closed under what readers do to a position: seen from a point, dotted with a direction, sent through a chart's linear map, multiplied by another harmonic.
- It answers, at every scalar: value with its rounding bound, range over a window, enclosure, and the first-degree extremes and roots. Each is posed about the stored point.
- `geom-brep` keeps what is about certification, not about the conic: the half-angle ladder, the subdivision, the noise meters and the band decisions. These read the harmonic's evaluations, jets and magnitudes, never Fourier coefficients about a centre.
- Readers cannot expand the harmonic back into `cos`/`sin` about the centre, because the type does not expose that.
- Why the posing belongs with the value: a root posed the readers' way misses even when its constant is right. Measured in a scratch probe at f64 against 80-digit arithmetic (not kernel code): an ellipse held on its arc with semi-axes 1e6 and 3e5 m, a plane crossing within 3 m of the stored point. The root misses by 1.7e-10 m posed the readers' way, and by 5e-16 m posed about the stored point. A sphere's residual along the same arc reads 1.8e-4 m off as Fourier coefficients about the centre, and 7e-16 m about the stored point. (sure that the basis and the posing are where the digits go)

**`Conic::frame` goes.** So do `Circle::frame`, `Ellipse::frame`, the conic arms of `Curve3::frame`, `Curve3::circle_at(frame, …)`, the public `Cover::arc(frame, …)`, `centred_at` (only tests call it) and the centre-relative default of `unit_in_plane`. PR 7510's `extent_along` and `cover_over` become one line each over the harmonic. Once `frame` is deleted, the clippy list, the `#[expect]`s and the gate's `.counts` retire.

**The centre is a datum: `centre() -> Centre<T>` on both kinds.**
- It is stored on a conic held at its centre. On one held on its arc it is derived, rounded at the semi-axes' scale on f64 and enclosed that wide on `Interval`, and the type's docs say so.
- `Centre` has no `Add<Vec3>`, so it cannot become a term of a point. It gives `axis_line()` and one explicit door to its coordinates.
- No panic. Your "kernel bug if we choose the wrong one" is about a construction picking its placement. A reader that asks a correctly built conic for its datum has chosen nothing. A panic there would make a reader's correctness depend on which construction made its conic: the cutoff you ruled out, moved to the reader.
- It is also unbuildable today. `step-export`'s `edge_curve` writes `ELLIPSE` with its centre placement, and the cone section already mints arc-held ellipses. An ellipse needs a datum door now.

**A conic made from a conic is a map, done by the conic's own doors.**
- Already there: `placed`/`mapped` (to be one name), `reversed`, the re-seams and `map_scalar`.
- Owed: a homothety (`replace_face::homothety`), a concentric offset, and a circle re-seamed at a point.
- `Circle::new` and `Ellipse::new` stay for mints handed a centre. With `frame` gone, nothing can feed them another conic's centre.

**The families.**
- **The certified lane** reads the harmonic at degree 1 (plane), 2 (wall, sphere, cone) or 4 (torus).
- **The cone chart image** is the harmonic through the cone's frame. Its azimuth channel is already stated about the vertex.
- **The symbolic scalar** is `point_at`.
- **`mesh/sizing.rs`** is a first-degree harmonic's range.
- **Two stated conics compared** is `Curve3::same_content`, which already compares the placement. A pair held differently reads as "not one conic". That is conservative and reachable only between different constructions. Whether two conics are one locus over a window is a different question that nothing asks yet.
- **The circle datum reads (79)** move to `Centre` in this unit, which is mechanical. Today every circle is held at its centre, so the datum they read is the stored, exact one. The reads that decide something about the arc (lies on this surface, is a parallel of that one) are owed a re-posing as the harmonic's range against the surface. That re-posing is the acceptance of the unit that first mints an arc-held circle (the STEP import's), not of this one.

**Ratified text (needs your sign-off).** The README clause "Every reading of where a conic is goes through the conic" was agent-written and approved in passing on PR 7258. Replace it with:

> A conic's point is stated about the point it stores, as that point plus two vectors times two functions of its parameter, and every reading of where the conic is (its point, its range, its roots, its residual against a surface) is a harmonic in that form. Its centre is a datum, stored or derived, and never a term of a point.

The `conic.rs` module doc changes to match. It currently says "a position is read as the centre (`Conic::center`, the origin of `Conic::frame`)", which contradicts the README today.

**Confidence.**
- The harmonic, with the first-degree posing inside it: likely.
- The certification machinery stays in `geom-brep`, reading the harmonic: likely, with the ladder unbuilt (unsure).
- `Centre` with no panic: likely.
- Re-posing the circle datum reads as a later unit's acceptance: likely.

## For the orchestrator

1. **How much the value does.** C's probe moved me. My report gave `Trig` evaluation, range, jets and re-basing, and left root posing to the solvers. I now put the first-degree roots and extremes in the value, as C does. A's `Stated` view leaves the 1.7e-10 m failure representable, with nothing to turn it red, so it is not the final state. Where C has the harmonic answer roots of every degree, I still hold that the ladder, the subdivision and the noise meters are `geom-brep`'s certification, reading the value's evaluations, jets and magnitudes. C's own "not weighed" leaves that open. My magnitude-ledger idea (in place of hand-written `terms`) is worth keeping as a direction, but it is unbuilt.
2. **The centre as a datum.** I keep my `Centre` type, the same as C's lean. A's panic is answered by C's argument, and it is refuted by a live caller I checked: `crates/step-export/src/writer.rs` `edge_curve` writes `ELLIPSE('', #placement, a, b)` from `conic_k.frame()`, so A's "the ellipse gets no centre door" breaks STEP export of a cone section today.
3. **The circle datum reads.** My predicate framing ("coaxial" means the distance harmonic is constant) is the same thing as C's "harmonic's range against the surface". I sign C's split, with the staging in For Ev. C's count of 79 is right; the brief's "about 70" is not.
4. **Two conics compared.** `same_content`. C's own reading of this is unsure and nothing needs it yet.
5. **The README sentence.** The wording above is what I sign. It states what a conic's point is, not what to avoid. A's "never as harmonics about a centre" alludes to an error, against your PR 7410 answer.
6. **Corrections I take from the others, and one I add.**
   - From A: the homothety and concentric offset I missed; the `mesh/sizing.rs` correction; `whole_conic_principal` reading `u_ref` through `Curve3::frame`.
   - From C: `unit_in_plane`'s default reads the derived centre on arc-held ellipses (callers `pose/window.rs` `foot_seed`, `section_loops` `conics_clear`, `end_face`), hidden by the module-wide `allow`; and the gate holes `Curve3::circle_at` and `Cover::arc(frame, …)`.
   - I add the export caller in item 2.
   - My own report's "phase (0,0) unifies the centre-held placement" holds in ℝ only; the evaluator's bits differ, so the `Held` enum stays.
7. **What to measure in the kernel before Ev is asked.**
   - (a) A's probe: an arc-held section with `a ≥ 1e5` against a sphere beside its anchor, through `conic_quadric_roots`. Expect a noise refusal today and a certified verdict with the quadric formed about the stored point.
   - (b) Re-basing the half-angle ladder's 32 anchors from a stored-point polynomial, and whether its escalations stay sound.

   If (b) loses digits at the anchors that matter, the posing of degree-2 and higher roots must also move into the value, which is C's position.
