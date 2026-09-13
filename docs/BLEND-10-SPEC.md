# BLEND-10 — the path fillet door never mints a joint the validator refuses (spec)

**Program:** BLEND (`work/blend/plan.md`, unit 10). **Item:**
`work/blend/path-fillet-door-validator-tangency-disagree.md`.
**Track:** kernel change — the standard v6 unit (binding spec, drawn implementer
arm, cross-model dual review, union fix pass, record-at-merge; §Review).
**Pre-draw fields, logged before the draw:** difficulty **M**, task-class
**NUMERIC**.

- **M** — one door and one classifier in one crate, but the unit is paid for
  a measurement that decides WHICH side moves, and the rows have to sweep
  four decades of turn angle at three ε rows.
- **NUMERIC** — the question is conditioning: what a stored arc of length
  `r·θ ≤ 2e-5` can carry of the tangency the door computed.

**Territory note.** `crates/profile/*` is S-BOOL's glob; the profile fillet
door is edited by announced seam (`work/bool/log.md`, at dispatch). Nothing
outside `crates/profile/src/{path.rs, sugar.rs, seg.rs, validate.rs}` and
`crates/profile/tests/**` moves.

## The claim

**The PATHS `.fillet(r)` door and `Profile::validate` disagree about one
joint across four decades of turn angle, and the validator's recourse names
the door as the way to make the joint exact.** The witness is the line × line
bend of the item (`crates/profile/tests/review_fillet_e2_probes.rs`,
`small_bends_build_at_the_path_door_and_refuse_at_validate_as_transversal`):
at `θ ∈ {1e-7, 1e-6, 1e-5, 1e-4}` the door builds a four-vertex loop and the
validator refuses it `TangencyContradicted`; at `1e-9` the door escalates
`path_corner_turn` in band; at `1e-3` and up both agree.

**The hypothesis the lane measures, not assumes.** The door computes the
fillet's centre exactly from its trims (`path.rs::fillet_arc_carrier`,
`centre = t2 + n̂·σr`), but what the profile STORES is the arc's endpoints and
its bulge `tan(θ/4)` (`ProfileVertex.bulge`); the validator's
`seg::joint_tangency` reads the carrier back from that form
(`seg::ArcGeom`, "closed forms from the crate docs' bulge") and classifies
the line/arc joint on `r − |h|` (`line_circle_joint`). For a chord of length
`2r·sin(θ/2) ≈ 2e-8` at `θ = 1e-7` and a bulge `≈ 2.5e-8`, the reconstructed
radius and centre carry a relative error of order `ε_mach / θ`, i.e. an
absolute `r − |h|` of order `1e-9`…`1e-6` across the four decades — inside
or past the band — while at `θ = 1e-3` it is `1e-13`. If that is what the
measurement shows, **the door mints an arc whose stored form cannot carry the
tangency it computed**, and the door is the side that moves. If the
measurement shows the stored form is fine and the validator's expression is
what loses the tangency, the validator is the side that moves, with the same
shape of fix. Either way: **the door never mints a joint the validator
refuses**, and neither side's sentence names the other as the remedy for a
joint the kernel itself produced.

**Ratified and not re-litigated:** the declared-tangency discipline (#101:
declarations are verified, never trusted); `seg::joint_tangency`'s predicate
reuse (`carrier_line_circle` on the same margin expression as `line_arc`;
same funnel, same bands, no new ε); the stored form (endpoints + bulge); the
`path_corner_turn` gate at the door.

## Phase 1 — measure before touching anything

For the item's bend at `θ = c·10^k`, `c ∈ {1, 2, 5}`, `k ∈ {−9 … −2}`, at the
three ε rows CI gates (default, `1e-6`, `1e-12`), one table with: `θ`; the
arc length `r·θ`; the bulge; the door's exact centre and radius; the centre
and radius `seg::ArcGeom` reconstructs from the stored vertices; `|Δcentre|`
and `|Δradius|`; the validator's `r − |h|` margin at BOTH joints of the
fillet; the door's verdict; the validator's verdict. Then the same table for
a line × arc corner and an arc × arc corner (the `sugar::arc_fillet_trims`
door) at the same decades. State from the table which side computes the
tangency the stored form cannot carry, and at which `θ` the margin crosses
the band on each ε row. Pushed as a PR-body section BEFORE Phase 2 begins.

**The stop clause.** If the table shows the two sides agree everywhere at
some ε row and disagree at another, or the disagreement is not monotone in
`θ`, stop at the report: the orchestrator re-scopes.

## Phase 2 — the change

1. **The door asks the validator's question of its own stored form before
   it emits.** After the trims are resolved and before the arc is emitted,
   the door reconstructs the joint the profile will store (the same
   `seg::ArcGeom` path the validator reads — one spelling, not a copy) and
   runs `seg::joint_tangency` on both joints of the fillet. `Tangent` →
   emit as today. `Transversal` (definite) → a typed refusal, new arm
   `PathError::FilletArcCannotCarryTangency { turn, radius, arc_length,
   margin }` (or the equivalent `CornerReason` arm if the envelope is the
   right home — decide by where the corner's other refusals live and say
   why), with a recourse that names what to change (a larger turn or a
   smaller radius keeps the stored arc conditioned; dropping the fillet
   leaves a sharp corner the validator accepts). In-band → escalate typed
   through `PathError::Escalated` with the predicate name the validator
   reads — **no new predicate name**; the door reuses `carrier_line_circle`
   / `carrier_circles_*` through `joint_tangency` itself.
2. **If Phase 1 says the validator is the side that moves**, the fix is the
   same shape at the other site: the classifier's margin expression is
   corrected to what the stored form can carry, with the door unchanged and
   every fixture that validates today validating bit-identically.
3. **The validator's sentence.** `TangencyContradicted`'s recourse ("make the
   tangency exact (the PATHS .fillet(r) door computes it)") stays true for
   hand-authored joints and must not name the door for a joint the door
   produced; if the door now refuses those, the sentence needs no change —
   say so, with the row that proves the door's output always validates.
4. **Rows**, in `crates/profile/tests/fillet_stored_tangency.rs`
   (aggregated; subject-named): the witness's four decades refusing typed
   at the door with the predicate name and margin read off the error, at
   the three ε rows; `θ = 1e-3` and above building and validating
   unchanged; the line × arc and arc × arc corners at the same decades;
   the recourse followed (the recourse's own larger turn / smaller radius
   builds and validates); every fillet that builds today validating —
   over every fillet fixture in `crates/profile/tests/**` (`Profile::validate`
   on every door output, which is what the item says the pair cannot
   both be). The e2 probe row flips to the new contract, renamed to its
   claim.
5. **The mutant**, in the PR body: skip the door's stored-form check and
   show exactly the door-refusal rows red.

## Constraints, binding

- **Every fillet that builds AND validates today is bit-identical to the
  merge base**: dump every fillet fixture's stored loop (vertices and
  bulges, to the bit) in the profile suites at both SHAs and diff.
- **No new predicate name.** The door reuses the validator's classifier;
  the K-stream gains the two joint classifications per fillet and the PR
  states the count; k-lint green or re-derived per the runbook.
- **The stored form does not change.** Storing the centre exactly is a
  representation change and is out of scope; if Phase 1 says only that
  would make the door's tiny arcs valid, the door refuses them typed and
  the item that wants the representation is filed.
- **Comments state the invariant**; the disagreement is the PR body's story.

## Acceptance

- The Phase 1 table (three corner kinds, three ε rows, the crossing `θ`).
- The door refusing typed where the validator would refuse, never building
  what it refuses; the recourse followed; the door's every output
  validating; the differential clean; the mutant table; the K count.
- Hosted CI green (full matrix).

## Out of scope

The stored form; `path_corner_turn`'s band; the arc-leg candidate machinery
(unit 11); `EscalationSite::Fillet`'s producer (unit 12).

## Review

v6 dual on the frozen head; claims to falsify (verbatim to both reviewers,
with `docs/prompts/reviewer-style-lane.md` by path):

- **C1** Every fillet that built and validated at the merge base is
  bit-identical at the head (re-run the differential).
- **C2** The Phase 1 table is right about WHICH side loses the tangency:
  re-derive the reconstructed carrier's error from the stored form yourself
  (the row's arithmetic is the implementer's, not the oracle) and decide
  whether the door or the validator moved for the right reason.
- **C3** The door never emits a joint `Profile::validate` refuses: sweep
  every fillet fixture in the profile suites AND construct your own —
  arc × arc at tiny turns, a fillet at the seam, a fused-incoming fillet —
  and validate each door output.
- **C4** The door's check is the validator's classifier (one spelling): no
  second margin expression, no new predicate name; the K count per fillet
  matches the PR's claim.
- **C5** The refusal's recourse is followable at every decade it fires:
  follow it (the larger turn, the smaller radius) and show the result
  builds and validates.
