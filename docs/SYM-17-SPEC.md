# SYM-17 — a widened rotation angle refuses on the plain interval lane: read the column norm, then carry the rotation or say it is the tier's (spec)

**Program:** SYM (`work/sym/plan.md`). **Item:**
`work/sym/a-widened-rotation-angle-refuses-on-the-plain-interval-lane.md`
(P1; on the chain Ev asked for). **Unit:** `work/sym/SYM-17.md`.
**Branch:** `sym/17-rotation-readout`, cut from `main`.
**Implementer:** Opus. **Review tier:** Phase 1 single FULL; Phase 2 dual, concurrent arm (§Review).

**Read first, in full:**
- `docs/prompts/implementer-discipline.md`;
- the item, whole, and `memories/refusal-text-is-not-cause.md`;
- `crates/editor-core/src/eval/wire.rs`: `transform_map` (around line
  4189) and how it builds the rotation;
- `crates/geom-core/src/linalg/affine.rs`: `rotation_about_axis`;
- `crates/topo/src/transform.rs`: the rigid-map checks, including
  `transform_rigid_col0_unit` (around line 315, `c0·c0 − 1`), what band
  they are asked against, and who calls them;
- `demos/tour/src/chaintol.rs`'s
  `the_certified_table_says_what_the_header_says`, and `chain.rs`.

## The claim

On the plain `Interval` lane, a chain of one link whose joint carries
σ = 0.01 rad refuses at its first `Node::Transform`:
"transform_rigid_col0_unit refused, definitely or in-band". The same
happens at two, three and four links.

The same one-link document certifies whole at `Sym<Interval>`
(`symbolic_zero` 1760).

The item reads the refusal as follows: `cos(angle)` and `sin(angle)`
are two independent brackets, so a column's `c·c` encloses an interval
around 1 rather than 1. **Nobody has read that enclosure**, so this is
a hypothesis.

## Phase 1 — read the norm, and price the answers

1. **The readout.** At the refusal (one link, plain `Interval`, the
   whole declared box), read `c0·c0 − 1`'s enclosure and its parts: the
   angle's enclosure, `cos` and `sin`, and the column. Read also the
   band it is asked against.
   - Is the item's reading the cause, or does something else widen it?
   - Give the enclosure's width as a function of σ at two more σ.
   - Find the largest σ that passes.
2. **The candidates, each measured on the chain at one and four links:**
   - **(a) An interval-lane rotation that keeps the identity by
     construction.** For example, a parametrization whose columns are
     unit by construction, or a rotation whose rigid-map certificate is
     carried from how it was built rather than re-derived from its
     entries.
     - Say where it would live: `affine.rs` is PROPS' and
       `transform.rs` is SHELL's (`work.py territory`).
     - Say what it changes for every other caller of
       `rotation_about_axis` or of the rigid checks.
   - **(b) The rigid checks asked with a tolerance that a widened angle's
     honest enclosure fits.** Say whether that is sound, or whether it
     loosens what the checks guard.
   - **(c) The statement, where `transform_map` is written, that a
     widened `SlotId::RotationAngle` is a symbolic-tier-only
     construction.** The plain lane's refusal is then the designed
     answer, and the refusal text says so (the cause, not the branch).
3. **Recommend one, with its cost.**

**Stop rules.**
- If (a) or (b) lands in another program's code (PROPS' `affine.rs`,
  SHELL's `transform.rs`) and changes what those checks decide for
  other callers, stop after Phase 1 and report. The orchestrator takes
  it to the owner, or to Ev if it is a design fork.
- If (c) is the answer, it states a scope of the plain lane. That is
  Ev's to rule: stop after Phase 1 and report.

## Phase 2 — the rigid-motion witness (Ev's answer, 2026-10-12)

Phase 1 stopped on both stop rules. Two designers weighed the question
and converged, and Ev answered on the `[ev]` PR. The final state is in
the item's section "Question, and Ev's answer"; read it whole. In
short:

1. **One type, `RigidMotion<T>`**, in `geom-core`'s linalg: a proper
   rigid motion (orthonormal linear part, determinant +1, finite).
   - It replaces `OrthoFrame`. A frame is the rigid motion that places
     the world frame, so `OrthoFrame`'s accessors (`origin`, and
     `u`/`v`/`w` as `UnitVec3`), its decided mints (`gram_schmidt`,
     aim-and-reference, axis-and-reference) and its exact world frames
     carry over unchanged in meaning.
   - The name `OrthoFrame` retires; every caller follows, demos
     included. Nothing is released, so compatibility is not a reason
     to keep it.
2. **Mints by construction.** Each of these decides only finiteness,
   and names what was not finite:
   - rotation about a `UnitVec3` by an angle;
   - rotation about an axis through a point;
   - translation;
   - the exact half-turns;
   - compose;
   - the inverse as the transpose.
3. **One decided door: today's `check_rigid`, moved onto the type.** It
   admits a raw `Affine3` (a literal `Frame`, a STEP placement, a
   library caller's matrix).
   - It keeps its predicate names and its ledger-row F10 flag.
   - It returns an undecided margin as undecided, not folded into
     "not rigid".
4. **Closed under the action:**
   - `RigidMotion × UnitVec3 → UnitVec3`, `RigidMotion × RigidMotion
     → RigidMotion`, and `RigidMotion × Point3`.
   - The revolve (`sweep/revolve/partial.rs`'s rotated normal), the
     sweep (`swept.rs`) and the patterns (`eval/wire/stepped.rs`) build
     `RigidMotion`s where they build rigid maps today.
5. **Callers.**
   - `topo::transform_rigid` takes `&RigidMotion<T>` and asks nothing
     about rigidity. `TransformError::NotRigid` leaves that door.
   - Placements mint by construction: `eval::wire`'s `transform_map`
     and `Step::Rigid`, and `placement.rs`'s `chain_motion` and
     `Motion<T>`.
   - A literal `Frame` goes through the decided door.
   - `editor-core`'s persisted `Frame` stays a general linear part, so
     an improper map stays representable and refusable.
   - The mate solve's representatives (`candidate_rotation` /
     `candidate_translation`, `opposed`'s half-turns, a face base) are
     typed as the witness. Every solve-side inverse (`mate_coset`,
     `invert`, `clocking_about`) is the transpose. Leave `member_of`
     and `check_offsets` themselves to INTENT stage 3, which retires
     them.
6. **Products of admitted frames are not decided again.**
7. **Not in this unit:**
   - SHELF's `transform-rigid-recertifies-images-enclosed-apart` (its
     own question; correct its "one design, land together" sentence:
     the witness is its prerequisite, not a co-lander);
   - the box-replay recourse ending (`work/issues/an-undecided-margin-over-an-interval-box-ends-as-a-kernel-defect.md`);
   - any mirror or scale type.

**Rows:**
- A minimal row outside the chain: an `Interval` rotation by a widened
  angle passes `transform_rigid`. A widened axis passes too.
- The decided door refuses a raw shear or scale (as today's
  `transform_rigid` rows do), and returns `Indeterminate` on a
  straddle.
- The transpose inverse is exact (`m · m⁻¹` is the identity at `f64`
  bit for bit on the half-turns, and encloses it at `Interval`).
- Re-baseline, saying what moved:
  - the chain's plain-lane row in `chaintol.rs` (`assert_row`'s
    `Interval` arm, and the header's "does not carry a widened
    rotation angle at all");
  - the symbolic lane's counts on placed documents (seven margins per
    transform stop being asked);
  - the K roster's rigidity rows (now asked only at the decided door);
  - every refusal text that named `transform_rigid_*` from a
    constructed map.
- Ev's standing words: "never skip out on a change that would make the
  code better because it would require rebaselining".

**Measure, and put in the PR body** (on `main` before, and after):
- the chain's plain-lane reach at one and four links (the largest σ
  that certifies, by the Phase 1 probe's bisection), and its new first
  refusal;
- the symbolic lane's chain row (counts and time);
- MSOLVE's clocking item's widening
  (`work/msolve/a-box-over-a-solved-clocking-widens-thirty-thousandfold.md`)
  if the transpose inverse reaches it. Add the measurement there.

**Docs:**
- `transform.rs`'s module doc ("checked at the door");
- the "one home of the rule" sentences on `check_rigid` and `Frame`;
- ASSEMBLY.md A2's parenthetical "(rigidity re-decided, every carrier
  re-certified)", which becomes "(rigidity carried by the witness or
  decided at the door, every carrier re-certified)". These are
  agent-written, not Ev's; the clause's decision does not change.

**Stop rules.**
- If merging `OrthoFrame` loses a guarantee a caller relies on, stop
  and report.
- If a stage 3 branch of INTENT is rewriting `mate/coset.rs` or
  `mate/solve.rs` when you reach them, do not touch those files.
  Report, and file the mate-side typing on INTENT's slate.

## Scope

- **Files (Phase 2):** `geom-core`'s linalg (`ortho_frame.rs` becomes
  the witness's home, plus `mat.rs` and `affine.rs` where they mint
  rotations); `topo/src/transform.rs`; `editor-core`'s `eval/wire.rs`,
  `eval/wire/stepped.rs`, `placement.rs`, `mate/solve.rs` and
  `mate/coset.rs` (within the stop rule); `sweep`'s revolve and
  `swept.rs`; `step-import`; every caller the compiler names, demos and
  `pncad-py` included; `docs/ASSEMBLY.md`; `demos/tour/src/chaintol.rs`;
  tests; the unit and item files, and SHELF's item.
- **Territory:** run `python3 scripts/work.py territory --base origin/main`
  and announce every seam.
- **Not in scope:** the symbolic tier's own handling, which already
  certifies; the chain's construction.

## Review

**Dual review, concurrent arm (difficulty H)**, recorded in
`work/sym/log.md` at spec time, per `docs/DUAL-REVIEW-PROTOCOL.md`. It
is a new core type that replaces another and changes a door's
signature across many crates. Claims to falsify:
1. **Soundness of each mint by construction at every scalar.**
   - `f64`: rigid up to rounding.
   - `Interval`: an enclosure of one rigid map per point of the box.
   - `Sym`: the same, with expressions.
   - `Dual`: tangent transport.
   - Can any path build a `RigidMotion` that is not one?
2. **The decided door** is today's check, unchanged in what it decides,
   and returns undecided as undecided.
3. **Nothing `OrthoFrame` promised is lost.**
4. **Every re-baseline is said and right.**
5. **The measurements.**
6. Phase 1's readout still holds (it is part of this PR).
7. Plus `docs/prompts/reviewer-style-lane.md` in full.

## Landing

- When the PR opens, the unit's status becomes `review`.
- At merge the orchestrator closes the unit, deletes this spec (with a
  note under `docs/doc-ledger/`), and closes the item or re-homes it
  with what is left.
