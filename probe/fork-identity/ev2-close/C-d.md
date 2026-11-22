**Checked on `main`:** DESIGN `[unrepresentable]` ("A signature is a claim too"); designer.md's two new clauses; the geom-core README DL5.3; and its instances, `geom_core::NoLane` (`real.rs` ~2083, `pub enum NoLane {}`) with `NurbsLaneOf` / `SplineFootOf` / `FittedLaneOf` in `geom-brep`. A pass that runs at every scalar holds `Option<L>` with `L: NurbsLaneOf<T>`. At a dual, `L` is `NoLane`, so `Some` cannot be built, and `NoLane`'s methods are `match self {}`. DL5.3's "never an `Option` a dual answers `None` to" forbids `Option<NurbsLane<T>>` where an impl CHOOSES `None`. It does not forbid `Option` of a type with no values.

**(d) One sentence per admitted type:**
- `Real::Expression`, and `expression()`, which returns `Option<Self::Expression>`:
  - **at `Sym<T>`:** "this value's node in the document's expression graph". `Some` is always returned.
  - **at `f64`, `Interval`, `Dual`, `Probe`:** "this scalar carries no expression". The type is `NoExpression`, an empty enum, so `None` is the only value `Option<NoExpression>` has. Nothing is answered at run time. The type is NoLane's shape exactly.
- **The question** (`identically_zero`, on `Expression`):
  - at `Sym`: "proof that the value is zero as a function of the document's variables, wherever the build takes the decisions it took";
  - at `NoExpression`: not reachable. There is no receiver value, and the body is `match self {}`, as NoLane's are.

  So the question admits only `Sym`, by type.
- **The mint** (`T: Decide`, every scalar): at every scalar, "the unit direction of the chord". At `Sym` it is spelled exactly where the tier proves an axis. Its closed arm is unreachable by type at the numeric scalars.

**Does the shape pass Ev's clause? Yes (likely).**
- The accessor has a true sentence at every scalar, and the question has a sentence only at `Sym`.
- The clause asks that code with no meaning at a scalar be excluded by type. That is done here at the value level, by the precedent DL5.3 ratifies.
- A `T: Symbolic` bound (a sealed trait only `Sym` implements) is the right form for code whose whole meaning is symbolic: later, `register_equal` and `free_identity`, and any helper that needs a proof and has no general spelling.
- It is the wrong form for the mint, because the mint has a meaning at every scalar. Bounding it would split it into a `Sym` mint and a numeric mint, which is two spellings of one function ([one-way]).

**"Needed before safe," applied to the question's callers:**
- **The mint:** needed. Without it, the slab's wall turns over the box and the sweep doors refuse it.
- **`arc_rim_normal_weight`:** needed. Its doc says the closed spelling keeps a held pattern's weight a symbolic identity at `Sym`.
- **`one_value`, the torus hub and `UnitVec3::is_exactly`:** not shown. Each should either name what breaks under the general spelling at `Sym`, or take the general spelling and not ask. That belongs to the audit, not to the text.

**Cite `[unrepresentable]` in the merged text? Yes, as the reason.** Amend my closing-round text's first sentences to:

> - **What only a symbolic scalar can do is asked of its expression.** Each scalar declares the expression it carries (`Real::Expression`): `Sym` its node in the document's expression graph; every numeric scalar none, by a type with no values, as a scalar that does not certify holds no lane (`crates/geom-core/README.md`, DL5.3). A signature is a claim ([unrepresentable]): the structural question has a meaning only at `Sym`, so it is a method of the expression, and a call against a numeric scalar does not typecheck; code generic over the scalar that has a meaning at every scalar takes the general spelling where there is no expression, and code that has a meaning only at `Sym` is bounded to it.

The rest is as I returned it ([A2]'s E12 clause included).

**Wrong elsewhere:** nothing new.

**What should change in the PR text:** a shape that returns `Option<NurbsLane<T>>`-like answers chosen by each impl would fail DL5.3. The PR's text should name the empty type, not just "none".
