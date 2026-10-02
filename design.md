# Boolean door and tier 3

## For Ev

**Recommendation (likely).** The boolean door ships a result only after
the at-rest gate of the currency its result claims. That is tier 3, or
tier 3′ over the result's own contact records when it carries any. The
verdict rides the result: `BooleanBody.body` becomes a
`topo::AtRestBody`, a body kept with its tier-3 verdict, whose only
`Validated` constructor is the gate.
- **Operands** stay plain bodies for now. Typing them as `&AtRestBody`
  is the right end state of a general door rule, not of this decision
  (below).
- **The volume backstop** keeps its inequalities. Its positivity arm
  (`encloses_material`) is retired: it reads tier 3's +V check a second
  time, at a different band.
- **DESIGN.md** gains one sentence beside D1's tier-2 sentence, and the
  tier-2 sentence is not reworded.

**Terms.**
- **Tier 3** (`validate_geometric`) is the geometric battery: carriers
  certified, planar residuals, dihedrals, no scaffold at rest, and the
  positive-volume sign.
- **Tier 3′** (`validate_pseudomanifold`) is tier 3 plus the
  coincidence census, checked both ways against the declared contacts.
- **Currency** is the tier a result's wrapper claims. A `BooleanBody`
  with contacts is 3′-grade; without them it is tier 3.
- **A scaffold** is the stand-in description an edge carries before its
  faces exist.
- **A door** is a public construction entry.

**Premise check (sure).** The "PR 3 description gap" in `gate`'s doc is
closed history, not a decision anyone still has to take.
- In M3 PR 3, boolean seam edges carried chord-line scaffolds, so no
  result could pass tier 3, and honest descriptions were "a PR 6
  obligation".
- PR 6a stage C (commit `82ffba91f1`) delivered them. Its message reads
  "tier-3 passes directly on split and boolean results", and the
  acceptance suite asserts tier 3 directly.
- The door stops at tier 2 only because the comment was never updated.

The 52 shipped results that fail tier 3 are three unrelated things:
- **49 test fixtures** handed to the door below tier 3.
- **3 slivers**, from a seam-description defect in the door itself,
  already filed.
- **The off-carrier body** from the dual review, which only tier 3 can
  see.

None of them is a question about descriptions.

**Final state.**
- **The gate.** `gate` runs `T::gate_at_rest_kept`, then
  `T::gate_at_rest_declared` over the result's contacts. Both go through
  `AtRestPolicy`, like the backstop: present at every certifying scalar,
  absent at duals, whose outcome says so. `validate_geometric` runs
  tiers 1 and 2 first, so this is one gate with no repeated tier 1.
- **Refusals** are tier 3's own typed findings under `ResultInvalid`.
  The arc-in-a-plane body refuses `PlanarBoundaryResidual`. The three
  slivers refuse `ScaffoldAtRest`, and on the ∩ `LaminaWedge`.
- **The three slivers' cause.** `describe_minted_edges` leaves a
  `Scaffold` in place on a seam it reads as smooth (its
  `Scaffold(_) => false` arm). The seam's angle is read over the seam's
  own length (`work/contact/seam-description-reads-a-dihedral-at-the-seams-own-length`).
  That filed item is P3 today. It lands before or with the gate, or
  those rows pin the refusal until it does.
- **What tier 3 cannot see.** A valid solid of the wrong shape still
  passes. That is the backstop's job: ∩ ≤ each operand, ∪ ≤ A + B,
  ∖ ≥ A − B. The 4 short positive ∩ bodies are caught only if each is
  also off its carriers. Only one is known to be.
- **Cost.** Tier 3 measured at about 15 % of op time on both corpora.
  The census at the door is not measured.
  - *Fallback if the census proves comparable to the op:* tier 3 at the
    door, and the census at the document's assembly gate. I lean
    against it, because the door would then ship a 3′ claim it did not
    check.
- **Reversible.** Yes: one gate call and one wrapper field.

**Operands: why not in the type now (likely).**
- **The end state I lean towards.** Every verb door returns an
  `AtRestBody` and takes `&AtRestBody` operands. The verdict is then
  paid once per body: an operand that already carries its verdict is
  not re-gated. A sub-tier-3 operand becomes unrepresentable, so no
  refusal can name the wrong party. It also ends
  `work/hone/a-stranded-operand-reaches-the-classification-invariant`,
  where an operand whose edges lie off its own surfaces ends every
  boolean on a kernel-invariant error.
- **Why it waits.** That end state only holds once every producer
  returns `AtRestBody`. Today extrude, revolve, loft, split and blend
  return plain bodies, and so do the editor's node outputs and Python's
  handle. Typed operands now would re-gate each of those at the boolean
  seam. That is the triple pay the rule exists to avoid, and the refusal
  would belong to the producing door.
- **So it is a follow-on.** It is the last step of the general door
  rule below, taken when the sweep doors adopt that rule. It does not
  change what the boolean does to its result, and it can be decided
  then, in either direction.
- **Meanwhile.** A result built from a sub-tier-3 operand ships if it
  passes tier 3, and otherwise refuses naming the result. Every such
  operand in the corpus is a hand-built test fixture or one of two
  far-origin rows.

**The backstop's +V arm (likely).** Both arms are absent at duals: the
dual `AtRestPolicy` runs no backstop either. So nothing is lost there.
Where both run, they read one predicate (a bounded result must enclose
material) at two bands:
- tier 3's check 7 at the ε band, with zero exempt;
- `encloses_material` at the exact band, after an interval
  re-derivation.

That is one rule with two readings, which can disagree on an in-band
negative. The better reading is a fix to check 7 itself, where the item
`work/contact/volume-door-reads-a-tiny-valid-boolean-result-wrong`
already points. It is not a second copy at the door.

**Ratified text (likely).** D1's tier-2 sentence ("Finished bodies must
pass tier 2…") stays true as a floor: Euler operators are public doors,
and they legitimately return tier-2 scaffolding. It was written in the
M1 sweep (`ea81facfc0`), while tier 3 was "named now, not implemented".

An added sentence carries the rule. It should name which doors it
covers, because "at rest" alone does not separate them from the Euler
operators. Proposed:

> A door that ships a finished body — a verb door: boolean, split, the
> sweeps, blend, shell, import — runs the at-rest gate of the currency
> its result claims, once, at the door's end, over the state the caller
> will see; the verdict rides the result. Euler operators are the doors
> that ship scaffolding.

Shell and import already follow it. Split, extrude, revolve, loft and
blend are its follow-on. The sentence binds future doors, so it is
yours to ratify.

**Rejected.**
- **Rely on the product gate.** It catches the defect one door later,
  under the wrong node. Meanwhile the kernel API, the verb seat and
  Python ship the bad body onward, where a later union can hide it.
- **Gate checks 3 and 5 only.** That is a second roster of tier 3 kept
  in step by hand, and it exempts the door's own scaffold and lamina
  findings.
- **Refuse only findings the door introduced.** That means attributing
  findings across the zip and the merge, and it passes a broken operand
  straight through.

## For the orchestrator

**What moved me.**
- *Operands.* The other report's sequencing argument. `&AtRestBody`
  operands are sound only once every producer returns `AtRestBody`.
  Before that, they re-gate sweep and editor outputs at the boolean's
  seam. So it belongs to the general door rule as its last step, not to
  this decision. I still lean towards it as the end state.
  - *What I correct in their report:* "pays tier 3 three times" holds
    only for re-gating plain operands. A typed operand carries its
    verdict and pays nothing.
- *DESIGN text.* An added sentence carries the rule. It should list the
  verb doors, because Euler operators are public doors that ship tier 2
  legitimately.
- *What did not move me:* retiring `encloses_material`. The brief's
  premise is false: the backstop does not run at duals.
  `impl AtRestPolicy for Dual<T>` in `crates/topo/src/props.rs` takes
  `_op, _a, _b…` and runs nothing, exactly as `gate_at_rest` does. So
  nothing is lost at duals. Where both run, the arm is check 7's
  predicate read at a different band.
- **Final state now:** the same as the other report, except for the +V
  arm. Theirs keeps the backstop whole.

**Unverified claims in the other report.**
- "The 4 short positive intersections … refuse at tier 3." Only the
  cited example is known to be off its carriers; the other three were
  not re-measured.

**Census cost at the door is still unmeasured.** Instrument
`gate_at_rest_declared` beside the existing `gate_at_rest` probe on the
same corpus before choosing between the recommendation and its
fallback.

**Off-question defects, unfiled.** These belong to the door-rule
follow-on if it is ratified, otherwise to SWEEP and BLEND.
- The "workspace convention" that callers re-validate at rest is cited
  by `extrude.rs`, `revolve/mod.rs` and `loft.rs`, and recorded nowhere.
- `Extruded::body`'s doc says a smooth cap rim refuses, yet the door
  ships that body.
- Blend's tier-2 postcondition is only a `debug_assert_eq!`
  (`sweep/src/blend/surgery.rs`).

**Process.** I read the d1 report on your instruction for this round;
the procedure in `memories/orchestration-model.md` provides for it. I
fetched full history for provenance, and built nothing.
