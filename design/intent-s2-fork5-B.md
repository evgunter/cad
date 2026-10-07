# FORK-5 — may a geometric slot read a measured value? (designer B)

## For Ev — round 2 (after reading the other report)

**Revised recommendation: refuse, for stage 2.** A measured value, and any definition that reads one, may be read by an assertion and by nothing else. I now agree with the other report's rule and with its one-sentence D10 revision: "a `Measure` defines an observed variable … read only by an assertion. A construction reads what was written." Confidence: **likely**.

**The strongest point, and why it moves me.** A measured scalar is geometry with its construction projected away. Every case I named has a spelling that keeps the construction, and that spelling is the one the `unproven-coincidence` lint would send the person to anyway:
- a pin in the same document: one variable read twice;
- a part in another document: AQ4's per-instance arguments, where the assembly owns `d`, passes it into the part and reads it for the bore;
- an imported body or an emergent face: a construction over the face reference, such as "up to" that face or an offset of its carrier.

So admission does not add expressiveness. It adds a second spelling, and the person is pushed toward the weaker one. It is the weaker one because the fit then holds only at the current values, and the lint flags a coincidence that is in fact true. My round-1 framing missed this: I compared a measure against copying a number by hand, when the real alternative is a construction over the face.

**The exclusions are real, and they undo my "no special rule" claim.** I checked each in the code:
1. **Mate offsets.** The mate solve runs once, before the node walk (`evaluate_at_descent` → `solve_with_env`). A measure across two instances already depends on that solve, so an offset that reads such a measure is a cycle the node schedule cannot see. *(sure)*
2. **Readers pinned to f64.** An authored frame's placement, a section's program resolution and the pinned lift all read `LaneEnv::nominal`, which is built before the walk. In the Interval or Dual lanes, such a reader of a measured value would need an f64 evaluation run inside the lane's run. This is the only place the brief's "bind per lane" cost is real. *(sure)*
3. **`min_clearance` has no f64 value.** `MinClearanceLane` returns `None` at f64, Dual, Probe and the symbolic scalar, so a body built from it never builds. *(sure)*

Admission would therefore carry three refusals that come from how evaluation is built, not from what the user means. The assertion-only rule is one refusal, stated by meaning, and it stays stable across stages.

**What still stands from round 1.**
- **One binding mechanism.** The binding is one generic change inside `evaluate`, serving assertions. No lane needs its own. Outside a run, an observed variable reads as typed "measured at evaluation", and its value is taken from the `Evaluation`. *(sure)*
- **"Observed" is derived, not stored.** It is a walk over the definition graph, like acyclicity, and it is not a second kind. *(likely)*

**What I do not take from the other report.** It argues that "a check could then break a build". That is not a reason for this rule. A measure read by a body would be an input to a construction, not a check. A rename already refuses a fillet through its select after stage 2's PR E. And under admission an assertion still gates nothing, so `a-failed-requirement-refuses-the-whole-product` stays closed either way. The rule rests on intent and on the three exclusions.

**The cost you should see.** The intensional spellings for the cross-document and imported cases do not exist yet:
- AQ4 is "not implemented";
- the "up to a face" and offset-of-a-face constructions are stage 6's.

Until they land, those cases cannot be expressed. A person will type the number, which is a stale copy with no link to its source. I accept that cost, for two reasons:
- Lifting the rule later is additive. Admitting now and narrowing later breaks documents.
- The right fix for the gap is to build those constructions sooner, not to add a measured spelling.

Some cases may have no construction spelling at all, such as a quantity used away from its own feature whose source is not a variable. Those would be the evidence for lifting the rule then.

## Round 1 (superseded where round 2 differs)

### For Ev (round 1)

**Recommendation: admit it, with no special rule.** A scalar variable is a scalar variable: any slot of its kind may read one whose definition reaches a `Measure`'s output. Two terms come with it, and neither is a refusal:

1. **One schedule binds every variable.** A definition that reads an operation's output, directly or through other definitions, is ordered with the operations and bound in the run, at the run's scalar, once its operation has run. The variable table alone gives the values of the variables that read no output, and nothing more. *(likely)*
2. **A driven value is an opaque atom to structure.** In coincidence tokens and canonical carrier forms, a measure's output is a symbol keyed by its id. Two reads of one measure are structurally one, and no other driven pairing is. The `unproven-coincidence` lint names the variable the measured geometry itself reads, where one exists. *(likely)*

Confidence in the recommendation: **likely**. The other defensible answer, refusing it for now, is laid out below, with the one argument for it that is real.

### Premise check

- **"Every analysis lane would have to bind it per lane" is not right.** *(sure)* Box, dual, symbolic, Monte Carlo and stackup are not separate evaluators. They are one generic `evaluate<T>` run at a different scalar: Interval, Dual64, the symbolic scalar, f64 per draw, and Dual64 per pass. ERROR-DESIGN E3 already evaluates a `Measure` "at every T like all nodes", which is how an assertion's margin gets its enclosure and derivative today. A measured value already exists partway through every lane's run. Letting geometry read it is one change, at the binding inside `evaluate`, and no lane needs its own.
- **The real cost is elsewhere** *(likely)*: code that reads a variable's *value* without running the document. `Doc::var_env` is called before or outside evaluation by the profile program's insert-time check (VQ9, `edit.rs`), the named-piece check that profile edits run, the mate solve's frame (`mate/solve.rs`), and the viewer's sketch, profile pane and properties. Under admission, each of these meets a variable it cannot know without a run. It must then say "known at evaluation", and the check moves to the run, where the profile lift already re-verifies every decision at the run's scalar (PP1–PP6).
- **What the fork is evidence of** *(likely)*: stage 2 would have two dependency mechanisms. One is the variable table, bound before the run by `bind_definitions`. The other is the schedule over operations. D10 says "reading is the only dependency", and the spec's own `Doc::upstream` already expands reads through definitions to their defining operations. So admission falls out of the dependency relation stage 2 is building anyway. Refusal is the rule that has to be added on top of it.
- **"A hole sized to a mating part" mostly does not need a measure.** *(sure)* When the pin is modelled in the same document, its diameter is a variable. D10's spelling is "one Length variable read twice", the same way coaxiality is one `Axis` read twice, and that spelling is structural. A measure is the only way to say it when **no variable exists**:
  - an imported body (D7), which has no parameters;
  - an instanced part in another pinned document, whose variables the assembly cannot read;
  - an emergent quantity, such as a gap left after booleans, a fillet or a pattern, that no single variable states.
  Those are the cases this fork decides.

### Ratified text this changes

- **ERROR-DESIGN E3** calls `Measure` a "recipe *sink* node … failures poison descendants only (… sinks have none)". Under D10 nothing is a sink. Admission retires the word and lets a measure's failure poison what reads it. The doc's header records the round as Ev's pass ("broadly looks good"), so this is approved agent text, not Ev's own wording. The shallow clone could not confirm the commit that wrote it.
- **D10 itself is unchanged.** Admission is what its "every slot holds a variable whose type suits the slot" already says.

### Answer A: admit (recommended)

What it makes true:
- One dependency graph. The schedule orders operations and definitions together, and acyclicity is checked over reads. "Depth of B reads a measure of B" is the ordinary cycle refusal.
- No variable has a hidden class. A slot's admissible readers are exactly the variables of its kind.
- Every lane gets the right semantics for free:
  - interval: the downstream geometry is built from the measure's enclosure;
  - Dual and stackup: derivatives chain through the measure;
  - Monte Carlo: the hole comoves with the pin in every draw;
  - symbolic: the measure's symbolic value flows on.
  A driven variable is defined, so it is never an axis and is never seeded, and VR3 already says so.
- A measure that fails, such as a vanished face, poisons its readers with its typed error, as a failed `Select` already does after stage 2's PR E. Making name resolution part of the geometry's dependency is not new.

What it leaves possible that you might not want:
- **A value that a person cannot read off the table.** The panel shows a driven variable's value from the last evaluation, as it shows a measure's today.
- **Comovement is the meaning, and it should be shown as such.** A hole driven by the pin's measured diameter is *match-made*: in Monte Carlo it varies exactly with the pin. "Designed to the pin's nominal, made independently" is a different statement: a second variable at an equal value, which is the VR offer declined. Neither answer to this fork can express "nominal from there, tolerance of its own". *(sure; see the orchestrator section)*
- **Driven pairings are never structural, except one measure read twice.** A fit built that way is linted on every box. That is honest, because it really holds only at the measured values, and the lint points to the structural spelling.
- **Wide enclosures.** An interval measure feeding geometry can widen the downstream enclosure and escalate more often. That is the real variation, not an artefact.

Worked example: `d = distance(pin.cyl, pin.cyl')`, then `hole.diameter = d + clearance`. In Monte Carlo, each draw rebuilds the pin, measures it and cuts the hole. In the box lane, the hole's radius is the measure's enclosure plus clearance. If the pin's face name vanishes, the measure refuses `UnresolvedRead`, and the hole and everything below it refuse with that cause. Deleting the measure strands the hole's definition, typed and not re-pointed.

Reversibility: harder than A′. Once documents rely on driven slots, refusing later breaks them. Nothing is released, though, so today that is a cost of the change, not of the final state.

### Answer A′: refuse at the door, for now

A geometric slot, or any slot outside an assertion, refuses `SlotReadsMeasuredValue { var }` when its variable's definition closure reaches an output. Assertions and other measures may read it.

- **For:**
  - Every variable outside an assertion's closure stays known from the table alone, so edit-door checks and the panel need no run.
  - The coincidence door never meets an opaque atom.
  - It is the cheaper direction to reverse: admitting later only lifts a refusal.
- **Against:**
  - It makes two kinds of scalar variable that the kind does not show, decided by a taint walk through definitions, and the variable offer must filter on it.
  - It leaves the imported-body, cross-document and emergent cases with one way out: copying the number by hand into a free variable. That copy silently goes stale when the measured geometry changes, which is the hidden dependency D10 exists to remove ("nothing else carries dependency").
  - It also keeps two binding regimes in the code for good.
- I would choose A′ only if you want to defer the evaluation-time rework of the value-reading doors listed above to a later stage. The cost is real, but it is a matter of sequencing, not a better final state.

"Admit only into definitions read by assertions" is A′ under another name, because an assertion already reads any scalar.

## For the orchestrator

- **Round 2:** I converged on the other report's rule. I verified its three exclusions in the code: `solve_with_env` before the walk, the `LaneEnv::nominal` readers, and the `MinClearanceLane` impls. I also checked AQ4 in `ASSEMBLY.md`. It is per-instance arguments *into* a part, "not implemented", and it serves a part that exposes `d` as an argument; a pinned vendor part that does not expose `d` falls back to "up to" or offset constructions. The remaining disagreement is small: I drop "a check could break a build" as a ground.

- **Assumed:** every lane goes through `evaluate<T>`. I checked `mc.rs`, `stackup.rs`, `drive.rs` and the `EvalScalar` bound, and the symbolic scalar implements `Decide`. I did not check that every measure arm is generic at the symbolic scalar; if one is not, it refuses typed there today already.
- **What admission adds to D's spec:**
  - `VarEnv` needs a "pending" state, an output-dependent variable not yet bound, distinct from `refused`, so that a reader outside the run knows to defer.
  - The value-reading doors move their check to the run: VQ9's profile check at `edit.rs` insert and set-value, the named-piece check, the mate frame (stage 3), and the viewer's `sketch.rs`, `pane/profile.rs` and `props.rs`.
  - VR8's `ParamSource` expansion stops at output ids, as atoms.
  - The spec's risk "binding … per lane is new machinery" should be restated as "one in-run binding plus the pre-run readers".
- **Brief error:** "several existing requests". I found no `work/` row asking for a driven dimension, using the obvious greps. The hole-to-mating-part case should be cited from wherever it came from, or dropped.
- **Off-fork gap, not filed (it belongs with the stage that adds tolerancing to defined values):** D10 cannot say "nominal derived from X, own tolerance". A defined variable carries only its inputs' pushforward (VR3). That is the usual meaning of a mating-part fit across documents, and Ev may raise it while reading this fork.
- **GUI note:** the stage-1 offer of an equal-valued existing variable will, under admission, also offer driven variables. One click then makes a match-made dimension. The offer's wording should say so.
- **Stage 3 note:** a placement reading a measured value across copies in different spaces has no value. The measure refuses, per stage 3's spaces. Nothing in this fork needs to change for that.
