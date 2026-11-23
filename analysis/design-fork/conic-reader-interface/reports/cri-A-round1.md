## For Ev

**Recommendation (likely).** A conic answers "where am I" with exactly one value: its **harmonic**. This is the conic's point written about the point it stores:
`P(t) = stored + U·ξ(t) + V·η(t)`.
- `U` and `V` are vectors.
- `ξ` and `η` are the placement's own two functions of the parameter. About a centre they are `cos t` and `sin t`. About a point of the arc they are `cos t − cos t₀` and `sin t − sin t₀`, formed without cancellation.
- Which placement the conic has stays private.

The harmonic is a named type in `geom`, and readers cannot see inside it. It is closed under what readers do to a position: a point subtracted, a dot with a direction, the part across an axis, a sum, a product. So a wall, sphere or cone residual is a second-degree harmonic and a torus residual a fourth-degree one, with no reader writing out cosines.

It answers:
- its value at `t`, with a running bound on its rounding;
- its range over a window;
- its jets;
- its **posed form for root finding**: the polynomial in a local half-angle variable about the stored point, with the map from a root back to `t`.

`Conic::frame` is deleted. `Conic::center` becomes a **datum**: a `Centre` type that cannot be added to a vector, stored or derived. A conic made from a conic is made by the conic's own maps; a homothety and a concentric offset are owed.

**Why (sure).** The readers lose digits by writing what they read as cosines about the centre. Whatever constant they are handed, a far-centred arc cancels at the semi-axes' squared scale.
- Worked example: a cone section with `a = 1e6 m`, against a 1 m sphere beside its anchor. The constant term today is about 5e11, which rounds to about 1e-4 m in a residual that is order 1 m near the arc. About the stored point, each term rounds at its own size.
- A scratch probe (f64 against 80-digit arithmetic, not kernel code) measured the same thing:
  - the sphere's residual read 1.8e-4 m off as cosines and 7e-16 m about the stored point;
  - a plane root solved the readers' way from the right constant still missed by 1.7e-10 m along the arc, against 5e-16 m when solved about the stored point.

So the way a root is posed differs by placement, and it belongs to the harmonic, not to each reader.

**Where `geom` ends and `geom-brep` begins.**
- `geom` owns the harmonic: its basis, its arithmetic, its evaluation and rounding bound, its window range, and its posed root form.
- `geom-brep` owns each surface's algebra composed into it, and every tolerance decision on it: the noise meter, the tangency band, the half-angle ladder's certification.
- The ladder takes the posed form; it never re-expands into cosines.

**The centre as a datum (likely).** `centre() -> Centre<T>` on both kinds. On a conic held at its centre it is the stored centre, to the bit. On one held on its arc it is derived, rounded at the semi-axes' scale at f64, or an enclosure that wide at `Interval`. That is as good a datum as the centre the same construction would have stored.
- It is for readers that need the number: the STEP writer (circles and ellipses), a surface of revolution made about a circle's axis, concentricity of two whole circles.
- I first proposed that asking an arc-held circle for its centre should panic. I withdraw that. Whether a reader works would then depend on which construction made the conic, which is the form cutoff you ruled out, moved to the reader. Your "a panic if we choose the wrong one" is about a construction choosing the wrong form, and it stays there.

**Circle datum reads (79 sites).**
- Most decide something about the arc: it lies on this surface, it is a parallel of that one, it is the same circle as another. These become the harmonic's range or a constant-ness test against the surface.
- The rest take `Centre`.
- This is one unit. It lands before the first construction that holds a circle on its arc (the STEP import's circle), so no reader meets an arc-held circle on the old path.

**Two conics compared.** Two different questions:
- "Is this the same stored carrier?" is `same_content`, which already compares the placement.
- "Do these two stated curves coincide over the part in use?" is a tolerance verdict: the level of one along the other's harmonic over that window.
- Each comparison site says which question it asks.

**README.** The clause "Every reading of where a conic is goes through the conic" was agent-written and approved in passing on PR 7258. I would replace it with:
> A conic's position is a harmonic of its parameter about the point it stores, and every reading of where the conic is, is read from that harmonic or from its point at a parameter. Its centre is a datum of the whole conic, stored or derived.

The `conic.rs` module doc changes to match: the placement's first bullet, and the paragraph "a position is read as the centre".

**Confidence.**
- Opaque harmonic with the root posing inside it, over a view or a polynomial-only type: likely.
- Cosines about the centre lose the digits: sure.
- `Centre` wrapped, with no panic: likely.
- The ladder can consume the posed form without new loss: unsure.

## For the orchestrator

1. **How much the named value does.** I moved from my view (`stated()`, with readers posing their own polynomials) to an opaque type.
   - C's measured 1.7e-10 m answers my argument: a reader handed the right basis can still pose the solve wrongly.
   - B's opacity closes the hole I named myself (a reader expanding back into cos and sin by hand).
   - B's ledger survives as the type's running bound on its evaluation; I am not convinced it needs to be a separate magnitude record.
   - The cut I sign: tolerance decisions stay in `geom-brep`. C's "answers roots" I read as posing roots, not certifying them.
2. **Centre.** I withdraw the panic on C's argument, and because I had missed that the STEP writer needs an ellipse's centre. I sign B's and C's wrapped `Centre`.
3. **Circle datum reads.** I sign B's and C's split. It is one unit, ordered before the first arc-held circle, not a property each later unit owes. While every circle is centre-held, no digit moves for them, so the ordering is the only constraint.
4. **Two conics compared.** I sign both answers, each for its own question. The by-bit sites (`injective::one_stated_conic`, `mesh/src/curved.rs` per C) are `same_content`.
5. **README.** Above. It avoids "never a sum about the centre" (B) and "never a term of one of its points" (C), following the PR 7410 ruling that ratified text should say what a thing is, not allude to an error.
6. **Disagreements and corrections:**
   - B keeps `Conic::along` public as "direction-only". It returns `a·(u·n)` and `b·(v·n)`, which are a harmonic's amplitudes (C's point). With the harmonic in place, `along` and `semi_axis_vectors` (whose doc reads "the conic is `center + …`") become internal.
   - B retires the gate entirely. `Centre::point()` still hands out bare coordinates, so I would keep one disallowed-methods entry on it, with a reason at each datum site. It is cheap, and it is what C's "one greppable door" means.
   - C's defect 1 is real and live: I confirmed `Ellipse`'s `Conic` impl does not override `unit_in_plane`, so it reads the derived centre. It belongs on the reader row.
   - B's "consumed extent" wording change is plausible, but it is a separate question; I would not fold it into this PR.
   - C has the general anchor moving `U` and `V` to the conjugate pair with parameter zero at the anchor; B and I keep principal `U`, `V` and a general phase. That choice is private and reader-invisible. C's version matches the README's ratified "parameter zero there", so it is the general-anchor row's to settle, not this fork's.
7. **Measure in the kernel before Ev is asked.**
   - A far-centred arc-held cone section (`a ≥ 1e5 m`) through `conic_quadric_roots`, against a sphere and a plane, using today's code and using a scratch `quadric_harmonics` built in the base form, then fed to the existing half-angle ladder.
   - I expect today's code to refuse on noise and the base form to certify at the arc's scale.
   - What would change my recommendation: if the ladder still refuses or loses digits when given the base form, then the ladder's own expansion must move into the harmonic too. That is a larger `geom` type and a thinner `geom-brep`.
   - Second, C's root probe reproduced in the kernel through `pose.rs`'s plane-conic root.
