# CONTACT-10: the containment refusals carry their decision

**Binds one implementer lane.** Deleted at merge;
`work/contact/CONTACT-10.md` survives. Read
`docs/prompts/implementer-discipline.md` in full first.

Branch `contact/10-contain-endings`, from `main`. Read both rows in
full:
- `work/contact/contact-near-boundary-endings-say-lower-the-tolerance`;
- `work/contact/contain-escalation-carries-no-decision`.

The rule is D4 ¶1 (i) in `docs/DESIGN.md`: a refusal's recourse follows
from its decision and verdict, and the decision is a closed type at its
site. The shared ending table is `geom_brep::recourse::SizedDecision`,
with the two-sided `SizedPass::NonZero`. ENCL's PR 3398 already moved
`census.rs`'s `WitnessTooClose` ending. Read how 3398 and PR 3390 did
their sites and follow the same shape; do not invent a second one.

## The work

1. **The grazed-parity refusal** in `crates/topo/src/boolean/contain.rs`
   (~:107) ends "move the point off the boundary or lower the tolerance",
   with no `Recourse:` marker and no value. It should end the D4 ¶1 way:
   the lever, plus the conditional, valued tighten where the decision
   passes on a nonzero sign and its margin is carried.
2. **`ContainError::RayExhausted`'s `Display`** ends the same way; fix it
   to match.
3. **`ContainError::Escalated`** carries the escalating decision as a
   closed type, from `PointInLoopError::Escalated` through to
   `topo::validate::classify_contain` and through
   `census::Undecided::of_point_in_solid`. Its ending is then rendered by
   `SizedDecision` at `Reading::AtRest`, with the valued tighten. If a
   decision cannot be carried as far as the renderer, say where the
   chain breaks and end on the lever alone there. Do not fabricate a
   value.

## Rows

- Pin each changed ending's text, valued and unvalued, the way 3398's
  rows do.
- `grep` for "lower the tolerance" in `crates/topo/src` after the change.
  Every hit left is either a site that D4 ¶1 permits (say which clause)
  or a row you file.

## Discipline

- Work in your own clone, with `CARGO_TARGET_DIR=/home/user/contact-10-target`,
  `CARGO_INCREMENTAL=0` and `CARGO_PROFILE_DEV_DEBUG=line-tables-only`.
- Wrap every cargo command in `with-build-slot.sh`, and run `df` first.
- No process listings, and kill only PIDs you recorded.
- Never run git in `/home/user/cad`.
- Push the branch only; no PR.

Before hand-back, run all of the following:
- `topo` + `sweep` at three eps;
- ALL of `editor-core`;
- `test-utils`;
- the Python suite (maturin wheel, `unittest discover` under
  `crates/pncad-py/tests`);
- clippy, `cargo fmt --all --check`, gates, lint.

Refusal texts are asserted in Python and `editor-core` rows, so expect
those to be the ones that move. Re-baseline each one and say what moved.

Write `docs/CONTACT-10-PR.md`, the PR body, and hand back.

## Territory

`crates/topo/src/validate.rs` is RESTFRONT's territory (`work.py
territory`). Keep the edit to `classify_contain`'s rendering, and leave a
one-line seam note in `work/restfront/log.md` naming the change and your
branch.
