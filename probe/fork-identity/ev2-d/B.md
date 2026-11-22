**1. The other side's strongest point, checked against the tree. It does not move me.**
- **The accessor is DL5.3's exact shape, which the README states as the rule.** `geom_core::NoLane` is `pub enum NoLane {}` (`real.rs:2083`). Its doc reads: "a pass that runs at every scalar takes its lane as a type that is the lane at a scalar that certifies and this at a dual, so the arm that enters certification is not reachable there by type." `geom-brep` implements `NurbsLaneOf`, `SplineFootOf` and `FittedLaneOf` for `NoLane` (`certify.rs:3149`, `3201`, `fitted_lane.rs:376`), and passes are called with `None::<NoLane>` (`dihedral_side.rs:1114`, `intersect.rs:565`). DL5.3 says: "A scalar's lanes are an associated type with no values at a dual, never an `Option` a dual answers `None` to." That rules out an impl that *chooses* `None`, and prescribes `Option` of a type with no values. `Real::Expression` = an empty type at every numeric scalar is that sentence with "expression" for "lane".
- **On A's point that a numeric scalar still implements something only for `Sym`'s sake.** True. But under A's form it also implements something: the per-site item, at every numeric scalar. What differs is what the item says. The accessor says one fact about the type ("carries no expression"), once, on a pattern the tree already ratifies. A's items each restate the general spelling per site.
- **A's cost, concretely.**
  - Each site that has a closed spelling gets a trait with impls for `f64`, `Interval`, `Dual<T>`, `Probe` (feature-gated) and `Sym<T>`. That is five impls per site, or four empty ones inheriting a default plus `Sym`'s override.
  - Today that is two sites, the mint (trait in `profile`) and `arc_rim_normal_weight` (trait in `geom-brep`): two traits and about ten impls, in two crates.
  - `UnitVec3::is_exactly` and `one_value`, if any of them stays a spelling choice, add a trait each. Being multi-value, they fit a per-site item no better than a combinator.
  - Nesting: `Sym<Dual<…>>` falls under `Sym<T>`'s impl and takes the closed spelling. That is right, because an identically zero value has zero derivative. `Dual<Sym<…>>` falls under `Dual<T>`'s and takes the general spelling, as under the accessor.
  - A default method covers the numeric arm with empty impls. Those empty impls are then the "for `Sym`'s sake" impl A objects to, written per site.
- **On "`if let Some(e) = x.expression()` reads as a check."** The `None::<NoLane>` passes read the same way. DL5.3 calls that unreachable by type, not a run-time check.

**2. Can both hold, everything symbolic bounded `Symbolic` and the mint one body?**
- No. Without specialization, a generic body can reach a `Symbolic`-only method for some `T` only through an item every `T` implements: an accessor (`Real::Expression`) or a per-site item. Rust cannot express it otherwise.
- The two compose, though. Put the question and every symbolic method on a `Symbolic` trait that only `Sym`'s expression type implements, so it is fully bounded. `Real::Expression` is then just the bridge, with "no values" at every numeric scalar. That is my position: the bound is on the expression, and the accessor is the bridge.

**3. Final position: the accessor, with the question and later `register_equal` / `free_identity` bounded by the expression's `Symbolic` trait.** (likely)
- If the orchestrator holds this as Ev's choice:
  - *Accessor:* one bridge on DL5.3's ratified `NoLane` pattern. Each numeric scalar carries one fact about its type, and every site stays one body.
  - *Bounded per site:* no numeric scalar has an expression slot. Each site that spells by scalar has a trait, five impls and a geometric sentence per scalar.
- I would pick the accessor.
- Under it, the merged text's "by a type with no values" can cite DL5.3 beside `[unrepresentable]`: "…by a type with no values, as a dual's lane is (`crates/geom-core/README.md` DL5.3)".
