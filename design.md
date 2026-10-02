# Boolean door and tier 3
## For Ev

**Recommendation (likely).** Every public door that returns a body
guarantees tier 3, or tier 3′ when the body carries contacts. It also
requires that of its operands. Carry both in the type that already
exists for this: `topo::AtRestBody`, a body kept together with its
tier-3 verdict. The boolean takes `&AtRestBody` operands and returns
its result through `T::gate_at_rest_kept`, so each body pays for tier 3
once, at the door that built it. A body made directly with Euler
operators reaches a boolean only by passing `AtRestBody::validate`
first. Retire the backstop's positivity arm (`encloses_material`),
because it re-reads tier 3's +V check. Keep the volume inequalities,
which check what tier 3 cannot: that the solid is plausibly *this op's*
answer.

**Premise correction (sure).** The "PR 3 description gap" is closed
history, not a pending decision. It was M3 PR 3's: seam edges then carried chord-line stand-in
descriptions, so no boolean result could pass tier 3, and the gate's
doc (and `m3_pr5_boolean_ops.rs`'s module doc) called the honest
re-description "a PR 6 obligation". PR 6a stage C (`82ffba91f1`) met
it: `describe_minted_edges` describes every seam and merge-kept edge at
mint ("tier-3 passes directly on split and boolean results"). The docs
were never updated, so the door stops at tier 2 because a stale comment
says so. The open question is a different one:

**The real defect (likely): "at rest" means two things.** D1 says a
body handed out of a door "is a plain value" at rest. Tier 3 is the
at-rest validity. But the doors guarantee only tier 2, and say "the
caller re-validates at rest per the workspace convention" (`extrude`,
`revolve`, `loft`). That convention is written down nowhere. The only
callers that do re-validate are the product aggregate (editor-core
`product.rs`), assembly and import. Doors disagree with each other:
`shell` gates tier 3 at its own door, the boolean, split and sweeps
gate tier 2, and blend gates tier 2 only in debug builds. What follows:
- **Defects surface late, or never.** A wrong mid-recipe boolean result
  becomes the next verb's operand; the product's refusal names the root
  it ended in, not the node that made it, and a later union covering
  the bad edge can hide it, leaving a valid solid of the wrong shape.
- **Operands below tier 3 make the door look broken.** An operand with
  edges off its own surfaces ends every boolean on a kernel invariant
  (`work/hone/a-stranded-operand-reaches-the-classification-invariant`)
  instead of a typed refusal of the input: classification and SSI
  assume certified operand geometry, a precondition nothing states.

**Ratified text to change (D1, tier 2).** "Finished bodies must pass
tier 2; tier-1-only states are visible solely inside operation
sequences, never across an API boundary at rest." The sentence was
written in the M1 ratification sweep (`ea81facfc0`), while tier 3 was
"named now, not implemented". It set the bar when tier 2 was the
highest tier there was, so it was not a choice against tier 3. I could
not tell from the repo whether the wording is Ev's. Proposed:
"A body a public door returns passes tier 3 (tier 3′ when it carries
contacts). Tier-2-only states are Euler-construction scaffolding, and
reach a door only through the at-rest gate (`AtRestBody`)."

### The answers, as final states

**A. Tier 3 in and tier 3 out, in the type (recommended).**
- *What it makes true.* No door returns a body below tier 3. No door
  reads an operand below tier 3. Any refusal names the door that made
  the body, or the gate the operand failed. The verdict is checked once
  per body, because `AtRestBody` cannot be carried to another body.
  Duals keep `NotRunAtThisScalar`, as they do now.
- *What it leaves possible.* A tier-3-valid body with the wrong shape
  still passes. That is why the backstop stays.
- *Consequences.* The 49 shipped results with tier-3-failing operands
  come from test fixtures (`review_m3_pr55`, `surgery::tests`, the
  far-origin `offer_rows` rows): each is described honestly, or its row
  expects the operand refusal. The 3 slivers from valid operands refuse
  at the door, typed; their cause is a filed P3 door defect that should
  move to P1 (below). The 42 sweep results the backstop refuses as
  `VolumeUnmeasured` refuse at tier 3 first, so their error changes.
  About 15 % of op time is added (measured on both corpora); tier 3′'s
  census cost on results with contacts is not measured.
- *Reversible:* yes. Dropping the type returns to answer B.

**B. Gate the result at the door, and leave operands as plain bodies.**
This catches what the door itself mints. An operand below tier 3 then
surfaces as `ResultInvalid` on the result. That names the wrong party,
and it still hits the stranded-operand kernel invariant before the gate
is even reached. Every operand that came out of a door is valid
already, so re-gating operands as plain `&Body` would pay for tier 3
three times per op. This is A without the precondition. It is defensible
only as a step towards A.

**C. Gate only the off-carrier checks (residuals: check 3, the planar
vertex residuals, and check 5, planar boundary containment).** This
catches the measured defect (an arc edge in a plane face) at a lower
cost. It also creates a fourth validity level that nobody has named,
and the door stops enforcing prefer-intrinsic and `ScaffoldAtRest`. The
door would hold the operands to one standard and its result to another.
Not recommended.

**D. Refuse only the findings the door introduced** (tier 3 on the
result, minus whatever the operands already failed). Findings would
have to be attributed to entities across the zip and the merge, which
is a second account kept in step by hand. It also lets an invalid
operand pass straight through. Rejected.

**E. Gate in the evaluator** (editor-core gates each node's output).
Right attribution in recipes, but direct topo and Python callers stay
unguarded and the kernel's guarantee moves into its client, reversing
API-first. Rejected.

### Load-bearing claims

- The door's own fault, ending at a typed refusal: `describe_minted_edges`
  keeps a `Scaffold` description on a seam that reads Smooth but whose
  jet is not determinate (its `Scaffold(_) => false` arm). The
  seam-length lever makes a seam between faces about 5e-8 rad apart read
  as Smooth (`work/contact/seam-description-reads-a-dihedral-at-the-seams-own-length`,
  P3). Under A, these results refuse at the door instead of shipping.
  Treating `Scaffold` as stale there (re-describing it in the chart),
  together with the filed lever fix, removes the cause. On the ∩ result
  `LaminaWedge` can remain. That is tier 3's honest verdict on a wedge
  end that nobody declared, and the op that minted it owns the refusal
  (D1, tier 3). *Likely*: from reading the code and the filed witness;
  I did not re-run it.
- Tier 3 does not certify that the op was right. A face dropped where
  every glued edge still lies on its carriers passes tier 3. The
  backstop's inequalities and the 4 short positive ∩ bodies stay a
  separate question (the residue item). *Sure.*
- Once A is in place, `encloses_material` checks the same predicate as
  tier 3's check 7, with the same exemption. *Likely.*

## For the orchestrator

- **Brief errors.** The "description gap" is closed history
  (`82ffba91f1`); the gate's doc and `m3_pr5_boolean_ops.rs`'s module
  doc are stale (the latter still cites `describe_as_intersections`,
  though it asserts `validate_geometric` directly). Re-title
  `boolean-door-tier-3-waits-on-the-description-gap`: it waits on the
  door contract (answer A), not on a description decision.
- **Unchecked.** Tier 3′ census cost at the door; whether all 4 short
  positive ∩ bodies from #3627 fail tier 3 (only the cited example is
  known to); which findings the 42 sweep results carry; whether D1's
  tier-2 sentence is Ev's wording (its PR conversation was not read).
- **Scope.** A is a door contract across `topo`, `sweep` and `verbs`
  (split, extrude, revolve, loft, blend, shell, boolean), larger than
  REACH. `verbs::run` and the evaluator carry `AtRestBody` between
  nodes; `shell`'s own `validate_geometric` becomes the shared gate.
- **Off-question defects, unfiled** (the door-contract work if A is
  ratified, else SWEEP and BLEND): the "workspace convention" that
  callers re-validate at rest is cited by `extrude.rs`, `revolve/mod.rs`
  and `loft.rs` but recorded nowhere; `Extruded::body`'s doc says a
  smooth cap rim refuses `SliverDihedral` with "nothing to do at the
  door", yet the door ships that body; blend's tier-2 postcondition is a
  `debug_assert_eq!` only (`sweep/src/blend/surgery.rs`).
- Full history fetched for provenance; no other design-fork branch read.
