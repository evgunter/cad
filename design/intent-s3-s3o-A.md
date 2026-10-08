# FORK-S3O — overconstraint, and the structural proof of a mate-placed face (designer A)

## For Ev

**Recommendation (likely).** Decide a placement's fold over **structure, not values**. A bundle of mates is a set of equations between pose forms. The coset algebra counts how many of a new mate's equations the bundle already fixes. Call these its **excess equations**. The coincidence door (stage 4) then decides whether each excess equation holds structurally.

- A mate is admitted when it fixes something new and every one of its excess equations is a structural identity.
- Otherwise it refuses `Overconstrained`, and nothing is measured.
- An admitted bundle is then a sound set of rewrite rules for the door. A face a mate places, and any carrier built from it, reduces to its partner's canonical form.

That one rule answers both halves of the fork.

**Terms.**
- **Equation of a mate**: a mate of kind K (Plane, Axis, Frame) says "the copy's pose A equals the partner's pose B modulo K's symmetry". That is 3 scalar equations for Plane, 4 for Axis and 6 for Frame.
- **Excess**: codim(held) + codim(added) − codim(result), where codim = 6 − dim of the residual subgroup. This is the number of the added mate's equations that the held fold already fixes. Each excess equation says that an *invariant* of the copy-side poses (an angle or a distance between two of them) equals the same invariant on the partner side.
- **Structural identity**: the door proves the two sides equal in canonical form (rung 2 linear forms, or rung 3 polynomial identity), so the equation holds for every value of the variables.

### 1. Premise check

1. **"Redundant" is not only "lowers no dimension".** The spec's provisional rule refuses a mate that leaves the fold's dimension where it was, and admits any mate that lowers it. But mates that do lower the dimension can also carry excess equations, and the fold checks those by value today:
   - **Two pegs.** Plane, then peg 1 coaxial (Revolute), then peg 2 coaxial gives Trivial. The excess is 3: two angles and the peg spacing.
   - **Oblique plane + axis.** The excess is 1: the axis-to-plane angle.
   - **A box in a corner.** Its three plane mates carry 3 angle equations in all.

   Under the provisional rule, such a mate still refuses `Contradictory` when an edit makes its excess equation false. That is a constraint falling back to an assertion, inside the fold. *sure*
2. **The table's case splits are value-decided coincidences.** `coset::table` branches on `parallel` and `perpendicular` margins. A plane plus an axis that happens to be parallel to its normal becomes `Revolute` because two numbers met. D10 says "every coincidence the kernel infers from values … is recorded at the one door", and none of these is recorded. "Decided by subgroup algebra without measuring" is true today only for the pinned case. *sure*
3. **The two halves are one question.** If a mate's excess equation holds only by value and the mate is still used as a rewrite, the door "proves" a falsehood by transitivity. Example: two pegs at spacing `s` in holes at spacing `s′`, typed separately as equal values. Each peg's contact rewrites to its hole, so the door would conclude `s ≡ s′`. The lint would then be silent on a coincidence that holds only at today's values. *sure*

   So overconstraint is what makes stage 4 H sound, and H cannot be specified until it is settled.

### 2. Options, as final states

**S — structural fold (recommended).**

- **Branches.** Each branch of the table is chosen by the door. If the two sides' directions are structurally parallel, the degenerate row is taken, and it is exact by construction. Otherwise the generic row is taken, and a value that is degenerate there refuses `Degenerate`. This is FORK-1b's "a combination refuses its degenerate case", applied to a bundle.
- **Excess equations.** Each must be a structural identity, or the mate refuses `Overconstrained` with the door's residual as its recourse ("make the hole spacing read the peg spacing"), or "say it with an `Assert`".
- **Outcomes.** `Contradictory` retires: an admitted bundle's excess holds at every value, so its cosets always meet. A11 (1)'s outcome list becomes **DETERMINED / UNDER / OVERCONSTRAINED / DEGENERATE**. This mirrors the banked sketch rule ("Sketch DOF diagnosis is two named layers"): structure diagnoses over- and under-constraint, and a degenerate configuration has its own word.

What S makes true:
- **A mate never checks.** No value edit can make an admitted mate refuse. Only a structural edit can, such as re-pointing the hole spacing to a new variable, and that refusal is the solve's, as today's split between door and solve already says.
- **Overconstraint is decided without measuring**, pinned or not.
- **The door's rewrites are sound and confluent**, so H needs no special rung.

What it costs:
- **Value-only redundant mates refuse.** A two-peg mate over separately typed equal spacings refuses, and so does mating an imported part (whose numbers prove nothing). The idiomatic recourse is one `Frame` mate:

  `Through{peg-1 axis, peg-2 centre} = Through{hole-1 axis, hole-2 centre}`

  This pins the copy with no excess. Peg 2's clearance in hole 2 is then an `Assert`, as D10 intends.
- **The door's reach becomes the limit of what can be mated.** A right angle the door cannot prove (an angle identity through trig at rung 3; *unsure* how far Sym reaches) refuses a mate that would have built.

**V — value-consistent excess admitted and recorded.** The fold glues an excess equation decided Zero, as a boolean glues a flush face. It records the equation at the door, and the lint reports it unless it is structural. A definite excess refuses `Contradictory`, and the sliver band escalates.

What V makes true:
- It is as permissive as mainstream CAD, so two pegs and imported parts just work.
- Your "slick" case is admitted.

What it leaves possible:
- **A mate that checks.** An edit to a value turns an admitted mate into a `Contradictory` refusal. That is against D10's "a mate places and never checks" and your rejection of constraints that fall back to assertions.
- **H must split each mate** into its placing equations (which are rewrites) and its excess equations (which are records). That is two descriptions of one mate.

D10 changes in substance.

**D — the provisional rule (refuse only a non-lowering mate; fold by value otherwise).** Reject. It leaves premise items 1–3 standing. Under it, H is either unsound or must refuse to rewrite through any bundle with excess.

**Sub-choice inside S: your "slick" case** (a mate fixing nothing new whose equations all hold structurally, such as both faces of a block mated into a slot whose width reads the block's width).

- **S-strict (lean).** It refuses. The refusal says "already holds structurally; the contact is proven". Nothing is lost: the second wall's contact is proven through the first mate's rewrite plus the identity.
  - It keeps your direction's words, "a relation added to a pinned placement is an overconstraint", and D10's sentence verbatim.
  - Every mate in a bundle pins something.
  - Relaxing it later is additive.
- **S-slick.** It is admitted as a no-op. This is the general form: one rule ("every excess equation is a structural identity") with no dimension clause. It costs a document holding mates that do nothing, and it changes D10's sentence.

*unsure* between the two. I lean strict for reversibility.

### 3. How the door proves a mate-placed face (S)

1. A copy's frame `F` is a pose **defined by its bundle**, never a solved value. The solve computes its value, but the door reads the definition.
2. Each admitted mate is a rewrite rule `F·A ≡ B` modulo its kind's symmetry, oriented from the copy to the partner. Read order is acyclic, so the rewriting terminates.
3. Canonicalising a copy's carrier pushes `F` through every construction, because a rigid motion commutes with them: `F·Meet(P,Q) = Meet(F·P, F·Q)`, a fillet's flank, an offset, a hole drilled on a face. Any leaf `F·X` with `X ≡ A` modulo the mate's symmetry is then rewritten to `B`, keeping the folded normal offset.
4. **Worked example.** A plate's cap is mated to a base's top, and the plate is then filleted.
   - The flank's carrier is the cap's plane by `CarrierFlow`. It is `F·plane(cap)`, which rewrites to `plane(base top)`, and the contact is proven.
   - A face parallel to the cap at thickness `t` reduces to base top `+ t`, so a second plate built on the base at `t` is proven too.
   - Two copies placed by bundles of the same forms have equal `F` atoms, so all their faces compare equal.

What this requires of stage 3:
- the placed frame is a definition over the bundle;
- `OfCopy` is a composite form, not a value;
- mates are oriented equations between pose forms (stage 3 B and C's shapes already are);
- the fold calls stage 4 C's door for its case splits and its excess. So the structural classifier must land before or with H, and F's refusal stays after I.

### 4. Ratified text that changes (quoted)

- **D10, Spaces.** Today: "a mate added to a pinned copy refuses as an overconstraint, decided by subgroup algebra (A11 (1)) without measuring". It extends to: "…, and so does a mate any of whose equations the bundle already fixes unless that equation holds structurally; subgroup algebra counts the equations and the coincidence door decides them, without measuring". This is Ev-ratified text (#3990/#4002, your direction item 1). It extends the rule to unpinned copies, so it waits for you.
- **D10, Coincidence.** Add one sentence: "a placement's admitted mates are equations of canonical forms, so a pose a mate equates reduces to its partner's, and so does every carrier built on it". That states the mechanism behind "(a mate-placed face is)", which is currently stated without one.
- **A11 (1).** Today: "several mates on one pair fold by exact coset intersection … to DETERMINED, UNDER or CONTRADICTORY, the last refusing with the added mate's measured clash". It becomes the S outcome list. "Exact" and "measured clash" go.

Confidence:
- *sure* on the premise;
- *likely* on S over V;
- *unsure* on strict versus slick;
- *likely* that rungs 2 and 3 cover the common cases: box angles, a hole drilled on a mated face, shared spacing.

## For the orchestrator

- **Provenance.** The clone is shallow, so `git log -S` stops at the graft. "Measured clash" in A11 (1) shows only at the graft (#4003), so its origin is unknown. The D10 sentence's provenance is taken from stage 3 spec §0. The transcript commits (`5f7a1c71e3` and so on) are absent, so Ev's "slick" and "fall back" quotes come from the brief and from `work/intent/mate-offset-verified-…`: "constraints and assertions are either one thing or two with no cleverness".
- **Ordering.** S makes stage 3 F's fold depend on stage 4 C, and on D for the angle identities.
  - Stage 4 H must rewrite only with the mates the structural classifier admits. Otherwise test 22's twin, two pegs with unshared spacing, proves a value coincidence.
  - Suggested order: stage 3 C → classifier, in or before stage 4 H → stage 4 I → stage 3 F's refusal and the migration.
  - C's interim `pins` bit becomes the classifier's output.
- **Spec tests to restate.**
  - Stage 3 test 19 stands under S-strict.
  - Test 20 (`Contradictory` on skew axes) becomes `Overconstrained` (the distance excess is not structural) or `Degenerate`.
  - Migration F drops more corpus mates under S: every value-only excess, not only the non-lowering ones. Measure the count before dispatch.
- **Defect off the question, not filed.** `coset::table`'s parallel and perpendicular branches are value-decided coincidences that nothing records, against D10 Coincidence. This holds today, whatever this fork decides. It belongs with stage 4 B's emission list, or with stage 3 F. File it if neither spec picks it up.
- **Unchecked.** I have not checked whether `Sym` proves trig angle identities: `cos(turn/4)` exactly, or the dot product of an extrude's cap and wall being 0 at rung 3. Stage 4 D's mitre measurement is the place to check.

## Round 2

Having read B's report: I move on the table splits (3) and on what "redundant" means (4), and I hold on the mate with excess equations (1), so `Contradictory` (2) still retires.

**1. A mate that lowers the fold and also carries excess equations (the two-peg plate): hold.**

- **Does B's rewrite system escape the transitivity problem? Only with a rule it does not state.**
  - Peg 2's axis has two forms. It is a pose of its own, which mate 2 rewrites to hole 2. It is also peg 1's axis `+ s`, which mate 1 rewrites to hole 1 `+ s`.
  - B's claim that "no bundle holds two rules for one carried pose" is true of named poses, not of forms. The door can therefore reach one form two ways, and once it equates them it has proved `t ≡ s`.
  - B is sound only if every proof routed through a mate inherits that mate's unproven `MateFold` row, so that the proof is conditional on the row. That is a second soundness mechanism, layered on the rewrite.
- **Is B's state the shape Ev rejected? Yes, in my reading.**
  - Mate 2 places a rotation and checks a spacing.
  - A value edit to `s` turns it into a `Contradictory` refusal, so it is a constraint that falls back to an assertion for part of its content.
  - A boolean never does this: a value edit there changes topology, and nothing refuses.
- **The two-peg plate still assembles under A.**
  - With one variable for both spacings, the spacing equation is a structural identity, so mate 2 is admitted.
  - With two separately typed spacings, mate 2 refuses. The recourse is one `Frame` mate between `Through{peg-1 axis, peg-2 centre}` and `Through{hole-1 axis, hole-2 centre}`, plus an `Assert` on peg 2's clearance.
- **This is a choice for Ev.**
  - Option A (refuse unless the spacing is structural): a mate never checks, and the door needs no conditional proofs. The cost is that value-only and imported two-peg plates must be placed through one `Frame` mate.
  - Option B (admit, decide the spacing by value, record it): mainstream permissiveness. The cost is a mate that checks, plus proofs conditional on `MateFold` rows.
- **Cost of reversing later.**
  - From A to B is a relaxation: every admitted document stays valid.
  - From B to A drops mates on migration.
  - So A is the cheaper one to start from.

**2. `Contradictory`: it retires under A.** If every admitted excess equation is an identity, two mates' cosets always meet. Under B it stays. The point stands or falls with point 1.

**3. The table's case splits: move to B.**

- Keep the splits as today's decided predicates. Under A's rule, nothing else is needed:
  - **Value-parallel but not structurally parallel:** the degenerate row's directional excess is not proven, so the mate refuses `Overconstrained`.
  - **Definite split:** the row's excess must be structural, as for any mate.
  - **Sliver band:** the decision escalates `Indeterminate`, as every decision does.
- So `Degenerate` is no longer needed as an outcome.
- No split needs recording either: a split whose consequent equations hold structurally is proven, and one whose equations do not is refused.
- **On layering:** B says that A's solve reading the door inverts the layering. It does not cross a crate boundary. The door reads canonical forms of the recipe (`editor-core`'s coincidence module), and an excess equation is an invariant within each part, which rung 2 decides from definitions without any evaluation result.

**4. A fully redundant mate: converged. It refuses, including Ev's "slick" case.**

- I adopt B's leave-one-out test: a mate is redundant when its subgroup contains the fold of the bundle's other mates. Mine ("fixes something new" when added) depended on the order the mates were added.
- I adopt B's spelling of the copy's frame: there is no `Frame` port, and the frame is known only through the bundle (`Carried`). It is the same claim as my "defined by its bundle".
- Reversibility: admitting slick mates later is additive.

**The D10 sentence I now propose.**

> "A mate that pins nothing of its copy (its subgroup contains the fold of the bundle's other mates) refuses as an overconstraint, and so does one carrying an equation the other mates already fix that the coincidence door does not prove structural; subgroup algebra counts the equations and the door decides them, so a mate places and never checks. A placed copy's frame is known only through its bundle, which the door reads as equations."

**The A11 (1) outcome list I now propose.**

> "a placement's mates fold by decided coset intersection to DETERMINED (`Trivial`) or UNDER (the residual named), or refuse OVERCONSTRAINED: a mate whose subgroup contains the fold of the rest, nothing measured, or an excess equation the door does not prove, its residual quoted as the recourse. A case split in the sliver band escalates `Indeterminate`."

"Exact" and "measured clash" go, and CONTRADICTORY retires.
