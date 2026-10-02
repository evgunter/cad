# Boolean door and tier 3

## For Ev

**Recommendation (likely).** The boolean door ships a result only after
the at-rest gate of the currency its result claims. That is tier 3, or
tier 3′ over the result's own contact records when it carries any. The
verdict rides the result: `BooleanBody.body` becomes a
`topo::AtRestBody`, a body kept with its tier-3 verdict, whose only
`Validated` constructor is the gate.
Operands stay plain for now: typing them `&AtRestBody` is the end state
of a general door rule, not of this decision. The backstop keeps its
inequalities and retires its positivity arm (`encloses_material`),
which re-reads tier 3's +V check at another band. DESIGN.md gains one
sentence beside D1's tier-2 sentence, which stays as written.

**Terms.** *Tier 3* (`validate_geometric`): carriers certified, planar
residuals, dihedrals, no scaffold at rest, positive-volume sign. *Tier
3′* (`validate_pseudomanifold`): tier 3 plus the coincidence census,
checked both ways against declared contacts. *Currency*: the tier a
result's wrapper claims (`BooleanBody` with contacts is 3′-grade). A
*scaffold* is the stand-in description an edge carries before its faces
exist. A *door* is a public construction entry.

**Premise check (sure).** The "PR 3 description gap" in `gate`'s doc is
closed history. In M3 PR 3 boolean seam edges carried chord-line
scaffolds and honest descriptions were "a PR 6 obligation"; PR 6a stage
C (`82ffba91f1`) delivered them ("tier-3 passes directly on split and
boolean results"). The door stops at tier 2 only because the comment
was never updated.

The 52 shipped tier-3 failures are 49 test fixtures handed in below
tier 3 and 3 slivers from a filed door defect; with the dual review's
off-carrier body, none is a question about descriptions.

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
- **Cost.** Tier 3 is about 15 % of op time on both corpora; the census
  at the door is unmeasured. If it proves comparable to the op, the
  fallback is tier 3 at the door and the census at the document's
  assembly gate (lean against: the door would ship a 3′ claim unchecked).

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
- **So it is a follow-on**: the door rule's last step, decided when the
  sweep doors adopt the rule; it changes nothing about the result gate.
- **Meanwhile** a result from a sub-tier-3 operand ships iff it passes
  tier 3; every such operand in the corpus is a test fixture.

**The backstop's +V arm (likely).** Both are absent at duals (the dual
`AtRestPolicy` runs no backstop either), so nothing is lost there. Where
both run they read one predicate, a bounded result encloses material,
at two bands: check 7 at ε with zero exempt, `encloses_material` at the
exact band after interval re-derivation. Two readings of one rule can
disagree on an in-band negative. The better reading is a fix to check 7 itself, where the item
`work/contact/volume-door-reads-a-tiny-valid-boolean-result-wrong`
already points. It is not a second copy at the door.

**Ratified text (likely).** D1's tier-2 sentence ("Finished bodies must
pass tier 2…") stays true as a floor: Euler operators are public doors,
and they legitimately return tier-2 scaffolding. It was written in the
M1 sweep (`ea81facfc0`), while tier 3 was "named now, not implemented".

An added sentence carries the rule, naming its doors (Euler operators
are doors too). Reversible: one gate call, one wrapper field. Proposed:

> A door that ships a finished body — a verb door: boolean, split, the
> sweeps, blend, shell, import — runs the at-rest gate of the currency
> its result claims, once, at the door's end, over the state the caller
> will see; the verdict rides the result. Euler operators are the doors
> that ship scaffolding.

Shell and import follow it; split, the sweeps and blend are its
follow-on. It binds future doors, so it is yours to ratify.

**Rejected.** *The product gate alone*: one door late, under the wrong
node, while the API, verb seat and Python ship the body onward, where a
later union can hide it. *Checks 3 and 5 only*: a second tier-3 roster
kept in step by hand, exempting the door's own scaffold and lamina
findings. *Only findings the door introduced*: attribution across zip
and merge, and a broken operand passes straight through.

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
- **Final state now:** the other report's, except the +V arm.

**Unverified in the other report:** "the 4 short positive
intersections … refuse at tier 3" — only the cited one is known to.

**Census cost at the door is unmeasured**: instrument
`gate_at_rest_declared` beside the `gate_at_rest` probe before choosing
the recommendation over its fallback.

**Off-question defects, unfiled** (the door-rule follow-on if
ratified, else SWEEP and BLEND): the "workspace convention" that callers
re-validate at rest is cited by `extrude.rs`, `revolve/mod.rs` and
`loft.rs` and recorded nowhere; `Extruded::body`'s doc says a smooth cap
rim refuses, yet the door ships it; blend's tier-2 postcondition is only
a `debug_assert_eq!` (`sweep/src/blend/surgery.rs`).

**Process.** Read the d1 report for this round on your instruction
(`memories/orchestration-model.md`); full history fetched; built nothing.
