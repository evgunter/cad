# SYM-17 — a widened rotation angle refuses on the plain interval lane: read the column norm, then carry the rotation or say it is the tier's (spec)

**Program:** SYM (`work/sym/plan.md`). **Item:**
`work/sym/a-widened-rotation-angle-refuses-on-the-plain-interval-lane.md`
(P1; on the chain Ev asked for). **Unit:** `work/sym/SYM-17.md`.
**Branch:** `sym/17-rotation-readout`, cut from `main`.
**Implementer:** Opus. **Review tier:** single FULL review (§Review).

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

## Phase 2 — the answer Phase 1 justifies (when no stop rule applies)

- The change.
- The chain's plain-lane row in `chaintol.rs` re-baselined, and said.
- A minimal row outside the chain for the shape.
- Ev's standing words: "never skip out on a change that would make the
  code better because it would require rebaselining".

## Scope

- **Files:**
  - `crates/editor-core/src/eval/wire.rs`, if the answer lives there;
  - the minting site Phase 1 names, within the stop rules;
  - `demos/tour/src/chaintol.rs` for the pin;
  - tests; the unit and item files.
- **Territory:** run `python3 scripts/work.py territory --base origin/main`
  and announce every seam.
- **Not in scope:** the symbolic tier's own handling, which already
  certifies; the chain's construction.

## Review

**Single FULL review**, recorded in `work/sym/log.md` at spec time.
Claims to falsify:
1. the readout (the enclosure and the cause);
2. each candidate's cost and reach;
3. the recommendation;
4. any re-baseline;
5. plus `docs/prompts/reviewer-style-lane.md` in full.

## Landing

- When the PR opens, the unit's status becomes `review`.
- At merge the orchestrator closes the unit, deletes this spec (with a
  note under `docs/doc-ledger/`), and closes the item or re-homes it
  with what is left.
