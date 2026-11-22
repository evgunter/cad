**1. A's strongest point, against the tree.** A says that under the accessor, a numeric scalar still implements something that exists only for `Sym`'s sake. It does not move me, because A's form has the same property.
- A's per-scalar item ("this scalar's unit direction of a vector") is implemented at `f64`, `Interval`, `Dual` and `Probe` as the general spelling. It exists only because `Sym` spells the direction differently.
- Both forms give each numeric scalar a true sentence. Under the accessor it is a fact about the type ("carries no expression"); under A's form it is a fact about geometry ("the quotient"). Neither is a run-time check.
- On "`if let Some(e)` reads as a check": it is a match on `Option<NoExpression>`, whose `Some` arm the compiler proves empty. That is the `NoLane` pattern exactly: `dihedral_side` is called with `None::<geom_core::NoLane>`, and `NurbsLaneOf for NoLane` is `match self {}`.
- DL5.3 ratifies that pattern ("A scalar's lanes are an associated type with no values at a dual"). The one difference: there the caller passes the `Option` in, while here the scalar hands it out. That changes nothing about what is reachable.

**What A's per-scalar item costs:**
- **At the mint:** a trait in `profile` or `geom-core`, for example `DirectionSpelling: Decide`, with a default body (the quotient). That is five impls: `f64`, `Interval`, `Dual<T>`, `Probe` (feature-gated), and `Sym<T>`, which overrides to call the `Symbolic` closed spelling.
- **Nested scalars:** `Sym<Dual<…>>` falls under `Sym<T>` and takes the closed spelling, which is right, since a proven zero has zero derivative. `Dual<Sym<…>>` takes `Dual`'s default, which is safe.
- **At `arc_rim_normal_weight`:** a second trait in `geom-brep`, with the same five impls.
- **Every further site** (`one_value`, `UnitVec3::is_exactly`, the torus hub, if the audit shows they are needed) adds one trait and five impls each.
- **The bound spreads:** every function on the path to a site gains that trait's bound. For the mint that means the profile's `T: Decide` signatures up through `editor-core`'s evaluator, and `ProfileLane` would gain a supertrait.
- **Layering:** collecting the items into one trait in `geom-core` would put profile- and brep-specific spellings into the scalar crate.
- So A's form is the accessor written out once per site. The accessor is the same per-scalar fact, stated once, for every site.

**2. A form that is both bounded and one body? No.** On stable Rust, without specialization, a generic body over `T: Decide` cannot reach a `T: Symbolic` method without some item every `T` implements: the accessor or a per-site trait.

**3. Final position: the accessor.** If it goes to Ev as a choice, the two sides in two lines each:
- **Accessor:** one associated type with no values at every numeric scalar (`NoLane`/`Carries` precedent, DL5.3). The question and future symbolic doors are methods of the expression, the mint is one body, and each new site costs nothing at the scalars.
- **Bounded per site (A):** nothing named "expression" exists at a numeric scalar. Each spelling site has its own per-scalar trait, whose sentences are about its result. Cost: one trait and five impls per site, and its bound on every caller path.

I would pick the accessor. It is the precedent DL5.3 already ratifies, and it keeps [one-way]: one per-scalar fact, not one per site.

**Agreed with A and B:** `Symbolic`, implemented by `Sym` alone, is the right bound for code whose whole meaning is symbolic. Today that is `register_equal` and `free_identity`, moved later. Under the accessor, `Symbolic` can be "`Real` whose `Expression` is inhabited", which is `Sym`'s `expression()` made total.
