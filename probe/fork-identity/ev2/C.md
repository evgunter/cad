## For Ev

**Yes (likely).** The question moves off the scalar and onto the expression that only `Sym` carries. At `f64`, `Interval` and `Dual`, asking it does not compile, and the generic mint takes the general spelling there because the compiler proves there is nothing to ask.

Terms:
- **Numeric scalars**: `f64`, `Interval`, and `Dual` (a value with its derivatives).
- **`Sym`**: a numeric value carried together with its expression (a node in the hash-consed graph of the document's variables).
- **The question**: "is this value identically zero in the document's variables?", answered by the symbolic tier with every value-reading rule switched off.
- **The mint**: the one function that makes a segment's unit direction from two points, called by the `line_to` verb and by validation.

**The trait surface.** One associated type and one accessor on `Real`, the way `Real::Carries<C>` already gives `Dual` no certificate by type (`Uncertified`):
- `type Expression: Expression<Self>`. At `Sym` it is the expression handle. At the numeric scalars it is an **uninhabited type**, one that has no values (`core::convert::Infallible`, or a sealed empty enum).
- `fn expression(self) -> Option<Self::Expression>`. At the numeric scalars the body is `None`, and the compiler knows nothing else is possible.
- The question lives on the `Expression` trait: `fn identically_zero(&self) -> Option<Identity>`. `Identity` is a proof token that only the symbolic tier can mint. At an uninhabited type the method body is `match *self {}`: there is no value to call it on, so it holds no answer.

**What fails to compile.** Any attempt to put the question to a numeric value: `x.identically_zero()` with `x: f64` (no such method on `f64`). And no impl anywhere says "no". (sure that this typechecks as described; it is the `Carries` pattern again)

**The generic mint.**
```
match (w.x.expression(), w.y.expression()) {
    (Some(e), _) if e.identically_zero().is_some() => axis(0, sign(w.y)),
    (_, Some(e)) if e.identically_zero().is_some() => axis(sign(w.x), 0),
    _ => quotient,
}
```
- At a numeric scalar both accessors return `None`, the guarded arms are statically dead, and only the quotient remains. The sign reading in the axis arm therefore never exists at those scalars.
- The choice of spelling lives at the call site, in that match, and the `None` arm is the general spelling. No scalar holds a geometry-specific hook.

**Other callers that ask at a generic scalar** (current `main`):
- *Choices of spelling*: `arc_rim_normal_weight` (`w0 − w2`), `one_value` (held weights), the torus hub (`major`), `PatchClass` on a net's control points. These take the same `expression()` match: closed spelling where `Sym` proves the identity, general spelling otherwise.
- *Structure written as a number*: an iso line's held slope (`Pcurve::held_line`, `row_of`, `PatchClass` on `pl`) and a ray's start. These become variants, so they need no question.
- *Decisions about what gets built*: chart corner and side (`spline_known`, `description`), row ends against stated ends (`pcurve_cache`), the bulge's line-or-arc choice, and the zero-chord stand-in. These become margined decisions.
- *New since the last round*: `Real::is_exact_zero_in_every_channel`, used by `UnitVec3::is_exactly` for the least turn's identity shortcut. It is a second structural question read off numbers (value and derivative at `Dual`). Under this final state it goes: that shortcut uses the `expression()` match, and the numeric scalars take the general arithmetic. (likely, not traced to every caller)
- `is_exact_zero` then has no callers and is retired.

**What the ratified text becomes.**
- **`[scalars]`**: "**The structural question** is asked of a value's expression, not of the scalar. Each scalar declares the expression it carries (`Real::Expression`): `Sym` its node in the document's expression graph; `f64`, `Interval` and `Dual` none, by an uninhabited type, so the question cannot be put to them and does not compile at them. Code generic over the scalar takes the general spelling where the scalar carries no expression. At `Sym` the question is answered by the symbolic tier with rules that read no value and a budget no session sets (ERROR-DESIGN E12). It is not a decision: it reads no sign or band, counts, records and defers nothing. Its answer may only choose between two spellings of one real value (a lane choice, [verdicts.consumers]). A branch that changes what is built is a decision; a value the kernel writes to encode structure is carried by its type."
- **E12**: "A numeric scalar is never asked" becomes "A numeric scalar carries no expression, so the question cannot be put to it (`Real::Expression`, DESIGN [scalars])."

**Is it a choice?** A small one, and I lean towards your suggestion. Both final states store the same bits in every lane.
- **(i) Type error, as above.** Asking a numeric scalar is unrepresentable ([unrepresentable]). The dead arm is the compiler's proof, not a convention. No impl holds an answer that a reader could take for "not zero".
- **(ii) A method on the scalar trait that answers "no" at the numeric scalars.** Simpler to read: no associated type, and a plain `bool`/`Option` call. But a numeric caller can write the question, get a vacuous answer, and nothing says that answer carries no information.

(ii) is what the PR described. (i) is (ii) made unrepresentable at the cost of one associated type, and `Carries` shows that cost is already accepted. (likely)

## For the orchestrator

- **Checked on current `origin/main`:**
  - `seg::axis_or`, `read_component` and `Component` still use the PR 6990 sign-reading shape;
  - `Real` already has `type Carries<C>` with `Dual = Uncertified` (`dual.rs:473`), the precedent for the uninhabited associated type;
  - `Real::is_exact_zero_in_every_channel` is new (`real.rs` ~506) and is a structural door the fork did not see;
  - the `is_exact_zero` caller set is unchanged in kind (`pcurve_cache::held_line` is the renamed iso-slope reader).
  - I did not find `READS_BY_SUPPORT` in `crates/geom-core/src` on `main` under that name. The nearest is `Real::same_value` ("the one knob of reading by support"), which is unrelated to this question.
- **Not checked:**
  - whether `Infallible` satisfies the trait bounds `Expression` would need (`Copy`/`Debug`/`Send`/`Sync`). It does for all four, but no build was run;
  - whether `Probe` (the K-statistics recording scalar) should carry its base's expression. It wraps `f64`, so I would say none;
  - who calls `UnitVec3::is_exactly` at `Dual` (an audit row).
- **Worth filing:** `is_exact_zero_in_every_channel` should be folded into this fork's audit; it lets `Dual` take a structural shortcut on a value coincidence where `W₁ − W₂` has a zero value and a zero derivative only by accident of the seed. The kernel-written-literal readers (`held_line`, `Ray::start_point`) should become variants, under fork 271's row.
