# FORK-S4F — how a construction's output carriers are known to the door (designer A)

## For Ev

**Recommendation: no construction describes its outputs a second time. The door learns
each output carrier by running the construction's own code with every variable as a
symbol, and reads the carrier off the result.** The kernel is already generic over its
scalar, and a document already evaluates at the symbolic scalar `Sym` (ERROR-DESIGN E12:
`evaluate<T>`, with `Sym`'s symbols keyed by `VarId`). So an extrude's cap built at `Sym`
*is* the formula `plane(frame, d, depth)`, computed by the same code that builds the f64
cap. Nothing is stated by hand, so nothing can drift. Both rungs read one symbolic
evaluation:

- **Rung 2, carrier identity.** Are the two cells on one carrier? Each cell's carrier,
  taken from the symbolic evaluation, is reduced to its kind's canonical form, modulo the
  kind's symmetry: a plane becomes its unit normal and its offset along that normal; an
  axis becomes its direction and its foot point. Two carriers are equal when every
  coordinate has the same exact-rational polynomial normal form. This covers flush,
  continuation, on-carrier, coaxial and equal radii.
- **Rung 3, margin identity.** Is the deciding margin itself identically zero at `Sym`?
  This covers relations between two *different* carriers: the mitre's equal angles, a
  tangent junction, a pinch at a computed point.

Under this design the spec's hand-written per-verb `CarrierFlow`, its `LinForm`/`PoseForm`
builders and its corpus witness are never built. `verbs::flow::ParamFlow` and the
`param_source` attach doors are today's hand-written note of where a radius lands; they
retire with `RadiusEvidence`. Confidence: **likely**.

### Premise check
1. **The fork asks how a description is kept true to its code. That description need not
   exist.** The formula Ev's mitre answer wants ("a face's carrier defined as a formula
   over the node's inputs") is what the generic kernel already computes at `Sym`. A
   per-verb form table would be the second description that answer says can drift. A
   form claims less than a theorem does, but it is still a copy of the code. (sure)
2. **Rungs 2 and 3 ask different questions; they need no different machinery.** Rung 2
   was linear only because Ev would "build only (2) to begin with, and extend it to (3)
   later if necessary, if that change wouldn't be too invasive" (#3990, part 2). What Ev
   cared about there was transitivity: the loop of blocks and the brick on two. Equality
   of polynomial normal forms is still an equivalence relation, so the loop and the brick
   close. A linear form is the degree-1 case, so this proves everything linear forms
   proved, and more (a boss of radius `w/2` centred at `w/2`). The mitre ruling already
   brings `Sym` into stage 4, so reading rung 2's forms from it builds nothing new.
   (likely)
3. **D10's "Each construction states which of its inputs each output carrier is a
   function of" is agent-written**: designer text in #3990, approved with D10. Ev's own
   words there concern transitivity and staging, not operations listing anything.
   (likely: the transcript gives the assistant's turns only in summary)

### What "states" then means
A construction states what its outputs depend on by being generic code over the scalar.
"Which of its inputs" is the set of symbols in the output's form: something observed,
never declared. D10's two examples become tests: an extrude's cap form contains no profile
symbol, and the walls of two stacked extrudes over one profile reduce equal, because the
base offset `h·d` lies in the wall's plane and the plane's symmetry forgets it.

### How it stays true
The form is the code, so no verb needs its own guard. Two hazards remain:

- **Value laundering would prove something false.** If a construction turns a computed
  value back into a constant (`T::from_f64` of a value it computed), two separately typed
  `5 mm` slots both become the literal 5 and compare equal. E12's zero verdicts already
  rely on the kernel never doing this, so the contract is inherited, not new. One test
  guards every verb at once:
  1. re-value every variable of each corpus document at a random point;
  2. evaluate there the forms derived at the nominal point;
  3. check they equal the f64 build of the re-valued document.
  A laundered constant does not move with its variables, so the check fails. (likely)
- **Numerical algorithms** (surface–surface intersection, fitting, the mate solve) give
  opaque forms and prove nothing, the sound direction (like the spec's `Opaque`). Their
  outputs must enter as an opaque atom per call, never as a literal. (likely)

The design can miss proofs but cannot make false ones. A kernel branch that picks its
formula by value can leave two equal carriers in two shapes, so that pair goes unproven.
Proofs are polynomial identities, so a refactor that keeps the mathematics keeps every
verdict.

### Cost at the door
- **One `Sym` evaluation per document**, run up to the last deciding node and only when
  the lint is asked for. It is memoized by content key like any lane, so after an edit
  only the edited cone replays. The f64 build pays nothing.
- Per row: two canonicalizations for rung 2; rung 3 reads a margin already computed.
- Measured on the corpus and tour when it lands (the spec already measures box mitres).
  If too slow, the fallbacks in order are the memo, then the per-node theorem cache Ev
  allowed. (unsure: I found no whole-corpus timing of `Sym` through booleans)

### Worked example
| Case | Rung 2 | Rung 3 |
|---|---|---|
| Blocks `h` and `h/2 + h/2`, stacked | both offsets normalize to `h`: proven | — |
| Blocks `h` and a typed `10 mm`, with `h = 10 mm` | residual `h − v`: unproven; recourse "make the slot reading `v` read `h`" | not identically zero |
| Box mitre | two carriers: no proof | `dot(d, cross(d,t))/‖…‖ ≡ 0`: proven |

### The options as final states
- **A (recommended): forms derived from the code at `Sym`.**
  - One source of truth and no per-verb tables. `ParamFlow` and its census test retire.
  - Projections reduce for free: a face-frame datum is computed from its face's carrier.
  - Transforms built from slots compose as formulas; a solved mate pose stays opaque
    until stage 3 defines a placed copy's frame from its mates.
  - Still possible: laundering, caught by the one test. Reversible: a cache can be added
    later without changing a verdict.
- **B: the spec's hand-written `CarrierFlow`, plus a witness.**
  - Cheap, needs no replay, and matches D10's "states" to the letter.
  - The witness checks the corpus's points, but a form claims every value. A form that
    misses a dependency the corpus never varies passes, and the lint, whose kind is
    `Certified`, then reports a false "proven" without a word.
  - Needs a builder per role for every carrier-minting verb, kept in step with the kernel
    by hand: exactly what Ev's answer rules out for theorems. I lean against it.
- **C: A's derived forms, cut down to linear forms** (nonlinear terms as atoms). This keeps
  D10's "linear forms" wording, but it proves strictly less than A and saves nothing.
  Rejected.

### Ratified text that changes (`docs/DESIGN.md` §D10, Coincidence)

- "Each construction states which of its inputs each output carrier is a function of (an
  extrude's end cap is `plane(frame, direction, depth)`, independent of the profile; a
  side wall is independent of where along the direction it sits)" → "Each output carrier
  is the function of its construction's inputs that the construction's own code computes,
  read by evaluating that code with every variable a symbol; no operation describes its
  outputs a second time (an extrude's end cap comes out as `plane(frame, direction,
  depth)`, with no profile variable in it)".
- "with offsets summed as linear forms over the variables with exact rational
  coefficients" → "with each coordinate a polynomial normal form over the variables with
  exact rational coefficients".
- "today it is the canonical-form equality above, and a later rung — such as the symbolic
  tier's identities — is one addition there that may only prove more" → "it is the
  canonical-form equality above, then the identity of the deciding margin itself; a
  further rung may only prove more".
- `crates/verbs/README.md` P1/P2: the parameter-identity channel goes. D10's list already
  names `ParamSource`.

## For the orchestrator

- **Spec deltas if A is taken.**
  - §1 and §4 lose `CarrierFlow`, the hand-built `LinForm`/`PoseForm` and the witness.
  - Test 11 becomes the re-valuation test, keeping a mutant that launders `depth` through
    `from_f64`. C and D share one replay, so they are better as one unit.
  - C's needs on stage 2 A/B/E probably shrink: whatever the evaluation reads is a symbol
    once its slot is a `VarId`. Unchecked against the frame reads in `wire.rs`.
  - Before stage 3, a slot-built `Transform` composes symbolically, so C's opaque
    placement atom may be unnecessary. H/FORK-S4-5 are unchanged in substance.
- **Unchecked:** whole-document `Sym` cost through booleans (only the `m10_*` tests run
  it); whether the kernel's `Plane`/axis fields can be read at `Sym` without new
  accessors; the lane (`Sym` decides Zero only over a `CertifiedEnclosure`, so the lint
  runs `Sym<Interval>` at point boxes); whether any carrier routine launders today (an
  audit of `from_f64(` on computed values in `sweep` and `topo` is cheap).
- **Provenance:** the 2026-10-03 transcripts are not in this shallow clone; part 2 was read
  through the GitHub contents API at `3d70e5de72`. I filed no defects; I found none off
  the question.

## Round 2

**1. One rung or two: I move to B's single rung, with one condition.** B's argument decides it.
The ladders' margins already read each kind modulo its symmetry. The plane ladder, for
example, decides `‖n₁ × n₂‖·arm` and then `d₁ − σ·d₂`. My rung 2 would state that
quotient a second time at the door: a per-kind copy of the predicates, which is the kind
of second description my round-1 report argued against. My argument for keeping it is only
half answered. Some margins read computed points: the plane ladder reads its offsets at
the extent's centre, and a vertex is a quotient of determinants. The `Sym` lane holds
`Div` as `Mul(a, Inv(b))`, so such a margin may freeze where a carrier comparison would
prove. That loses proofs; it is never unsound.
- **The condition:** where the measurement shows a carrier-relation row freezing, the fix
  is to phrase that predicate's margin over carriers, in the kernel where the decision is
  made, not to add a door-side rung 2.
- **The other difference is row matching.** B matches a row across lanes by (node, site,
  ordinal, cell names), which assumes D9's decision order holds at `Sym<Interval>`. Mine
  looked carriers up by `StableName` and needed no match. Both are sound when a row is
  unmatched, because an unmatched row stays unproven.
- **Is it a choice for Ev?** No; it is a measurement. Reversing later is cheap either way:
  rung 2 is "one more rung that may only prove more" at the same door.

**2. Soundness: I hold, and laundering breaks B's "never a false proven" as worded.** E12's
theorem covers the expression the lane saw. If a construction launders a computed value
through `T::from_f64`, that expression is no longer the construction's dependence on its
inputs. Two typed `5 mm` slots both become `Lit(5)`, and their difference is the zero
polynomial. Both designs read the same replay, so both need the guard. One test covers
every verb: re-value each corpus document's variables at random, evaluate the derived
forms there, and compare them with the f64 build. B's claim survives with a contract
attached: no construction turns a value it computed back into a constant.
- **A second, milder limit applies to both designs.** A kernel branch taken by value fixes
  the form to that branch. An `abs` written as `if x < 0 { -x } else { x }` yields `x`. So
  "proven" means proven on the region where the construction makes the same decisions.
  Outside that region the build differs and the row is re-decided, so this is honest, but
  D10's "across the family" should say "for every value" only with this scope.

**3. Units: I move to B on C, D and H, and agree on the discharge kind.**
- C and D merge, and C needs nothing from stage 2: slots and placement steps have been
  `VarId`s since stage 1, so the replay sees symbols.
- H becomes a measurement after stage 3: does a mate-placed face's contact discharge as a
  theorem? It becomes a build only if not. The opaque placement atom goes.
- Unit B's record needs the discharge. `Decided` has no theorem arm, so the `Sym` replay's
  rows carry `Discharge::Theorem` versus numeric, as a field on the row or an arm on
  `MarginDiag`.

**D10 text I now propose.** Take B's two replacements as written. Add one sentence after "a
theorem for every value (E12)": "A construction never turns a value it computed back into
a constant, which is what keeps its code the definition; and the theorem holds wherever
the construction takes the decisions it took here."
