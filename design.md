# Boolean door and tier 3

## For Ev

**Recommendation (likely).** Make the finished body a type:
`topo::AtRestBody`, a body kept with its tier-3 verdict.
- **Every verb door takes finished bodies and returns one** (boolean,
  split, extrude, revolve, loft, blend, shell, import). It guarantees
  tier 3, or tier 3′ when the result carries contacts. Each body pays
  for that once, at the door that built it.
- **Euler operators work on construction state** (`Body`). A body they
  build becomes finished only by passing `AtRestBody::validate`.
- **For the boolean:**
  - operands are `&AtRestBody`;
  - the gate is `T::gate_at_rest_kept`, then the census over the
    result's contact records;
  - `BooleanBody.body` is an `AtRestBody`.
- **D1's tier-2 sentence is changed**, not supplemented.
- **The backstop keeps its volume inequalities and retires its
  positivity arm** (`encloses_material`).
- **One fix lands before or with the gate.** Tier 3's volume-sign check
  (check 7) must take the arm's interval re-derivation. Today check 7
  refuses a valid sliver that the door ships.

**Terms.**
- *Tier 3* (`validate_geometric`): carriers certified, planar residuals,
  dihedrals, no scaffold at rest, positive-volume sign (check 7).
- *Tier 3′*: tier 3 plus the coincidence census, checked both ways
  against declared contacts.
- *Currency*: the tier a result's wrapper claims.
- *Scaffold*: the stand-in description an edge carries before its faces
  exist.
- *Construction state*: a body mid-build, tier 1 or tier 2.

**Premise check (sure).** The "PR 3 description gap" in `gate`'s doc is
closed history.
- In M3 PR 3, seam edges carried chord-line scaffolds.
- PR 6a stage C (`82ffba91f1`) gave them honest descriptions: "tier-3
  passes directly on split and boolean results".
- The door stops at tier 2 only because that comment was never updated.
- The 52 shipped tier-3 failures are 49 test fixtures handed in below
  tier 3, and 3 slivers from a filed door defect. Neither is a question
  about descriptions.

**The question under it: which doors owe tier 3.** It is decided by the
door's role, not door by door:
- Euler operators legitimately hand back scaffolding.
- Verbs and import hand back solids someone will use: an operand, a
  product root, an export.

Today one type (`Body`) means both, so the boundary lives in a
"workspace convention" recorded nowhere, and the doors disagree. Shell
gates tier 3. The boolean, split and the sweeps gate tier 2. Blend
gates tier 2 in debug builds only.

Once the finished body is a type, typed operands are the same rule read
from the input side: a verb consumes what verbs produce. That makes four
things true:
- a sub-tier-3 operand cannot be represented;
- the stranded-operand kernel invariant
  (`work/hone/a-stranded-operand-reaches-the-classification-invariant`)
  becomes a typed refusal at `validate`, naming the operand's entities;
- no refusal names the wrong party;
- nothing is gated twice, because a kept verdict cannot be carried to
  another body.

**Final state, boolean.**
- **Refusals** are tier 3's typed findings under `ResultInvalid`. The
  dual review's arc-in-a-plane body refuses `PlanarBoundaryResidual`.
  The three slivers refuse `ScaffoldAtRest` (and `LaminaWedge` on the
  ∩) until the seam-lever item lands with the fix to
  `describe_minted_edges`' `Scaffold(_) => false` arm, which leaves a
  smooth seam's scaffold in place.
  (Item: `work/contact/seam-description-reads-a-dihedral-at-the-seams-own-length`.)
- **The backstop** catches what tier 3 cannot: a valid solid of the
  wrong shape (∩ ≤ each operand, ∪ ≤ A + B, ∖ ≥ A − B).
- **At duals both are absent**: the dual `AtRestPolicy` runs neither,
  and its outcome says so.

**Measured: check 7 against the valid sliver (sure).** The probe is
`contact9_side_codes`' corner pose, ∩ at ε = 1e-12.
- The exact volume is 5.83e-19 m³. `mass_properties` reads −2.96e-16 m³,
  and `validate_geometric` refuses it as `NegativeVolume`.
- The door ships it today. `encloses_material`'s interval re-derivation
  straddles zero, so the arm passes.
- At ε = 1e-9 the same pose passes tier 3, and so does the slab pose at
  1e-12.

So a tier-3 gate at the door refuses this valid body unless check 7
takes the interval re-derivation first. That item is
`work/contact/volume-door-reads-a-tiny-valid-boolean-result-wrong`: P3
today, P1 under the gate. Once check 7 has the re-derivation, the arm
reads the same predicate the same way and retires as a duplicate.

**Ratified text: a change in substance.** D1 currently reads: "Finished
bodies must pass tier 2; tier-1-only states are visible solely inside
operation sequences, never across an API boundary at rest." It was
written in the M1 sweep (`ea81facfc0`), while tier 3 was "named now,
not implemented". The new rule raises the bar for finished bodies to
tier 3, and a second sentence beside it would set two bars on one
thing. Proposed:

> A finished body (`AtRestBody`) passes tier 3, or tier 3′ when it
> carries contacts; every door that returns or consumes one pays that
> gate once, at the door that built it. Construction state (tier 1, or
> tier 2 without geometric certification) is what Euler operators hand
> back, and becomes a finished body only through the at-rest gate.

Tier 2's definition in the tier list is untouched. The sentence binds
future doors, so it is yours to ratify.

**Consequences.**
- **Cost.** About 15 % of op time for tier 3, measured on both corpora.
  The census at the door is unmeasured. If it proves comparable to the
  op, the fallback is tier 3 at the door and the census at the assembly
  gate. I lean against that: the door would claim a 3′ currency it did
  not check.
- **Reach.** The type runs through `topo`, `sweep`, `verbs`, the
  evaluator and Python's handle.
- **Fixtures.** The 49 fixtures take the description step
  (`describe_as_intersections`), or refuse at `validate`.
- **Reversible.** Dropping the operand type returns to gating the
  result alone.

**Rejected.**
- *Gate the result, keep operands plain.* My round-1 position; answered
  below.
- *Rely on the product gate.* One door late, under the wrong node, and
  a later union can hide the defect.
- *Gate checks 3 and 5 only.* A second tier-3 roster kept in step by
  hand.
- *Refuse only the findings the door introduced.* Needs attribution
  across the zip and the merge, and a broken operand passes through.

## For the orchestrator

**What moved me this round.** The round-1 sequencing argument that moved
me (typed operands are sound only once every producer returns
`AtRestBody`) is about the cost of the change. `designer.md` §4 says to
disregard that, and d1's final state answers it by including the
producers. My first argument (each body pays once; the stranded operand
becomes a typed refusal) was never answered, so it stands and I return
to it. d1 also moved me on two points: the D1 sentence is a change, not
an addition; and tier-2-only bodies are not "Euler scaffolding", hence
"construction state" in the proposed wording.

**What I add.**
- **The question under both positions** is whether "finished body" is a
  type. If it is, typed operands are not a separate decision.
- **The check-7 dependency is on the gate, not on retiring the arm.**
  With tier 3 gated, check 7 refuses the sliver whether or not the arm
  exists. So the check-7 fix is a precondition of the gate, and the
  arm's retirement follows from the gate with no further dependency.
- **Converged:** the final state now matches d1's revised report.

**Probe.** A temporary test appended to
`crates/topo/tests/contact9_side_codes.rs` ran ∩, − and ∪ on the corner
and slab poses at `CAD_TOLERANCE_EPS` = 1e-12 and 1e-9, reading
`mass_properties` and `validate_geometric`. Only the 1e-12 corner ∩
failed (`NegativeVolume`). The probe is reverted and not committed.

**Sequencing (not Ev's).**
1. Check-7 interval re-derivation.
2. Seam lever, with the `Scaffold(_) => false` arm.
3. The boolean gate and operand type (callers wrap sweep outputs with
   `validate` meanwhile).
4. Sweep, blend and split doors.
5. Retire `encloses_material`.

**Unmeasured.** The census cost at the door; whether all 4 short
positive ∩ bodies fail tier 3; which findings the 42 sweep results
carry.

**Unfiled, for the door-contract work.**
- The unrecorded "workspace convention" (`extrude.rs`, `revolve/mod.rs`,
  `loft.rs`).
- `Extruded::body` documents a refusal for a body its door ships.
- Blend's tier-2 postcondition is debug-only.
