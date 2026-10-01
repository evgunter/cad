# PROPS-KLINT-BASELINE-SPEC.md

PROPS k-lint baseline — the instrument fired after going dark (PR #3530).

Deleted at the unit's merge, 2026-09-30.
Recover with `git show 8d9cb980b:docs/PROPS-KLINT-BASELINE-SPEC.md`.

**Spec note worth keeping: the clause that produced the right answer was
the one that pre-authorised refusing the task.** The spec's step 4 said
to choose a recourse from k-lint's own list, and then added that if the
measurement showed real geometry had regressed, *neither recourse
applies — stop and report*. The unit measured, found the flags were
neither a stale baseline nor a demotable row, and invoked that clause.
Without it the lane's two sanctioned options were both wrong, and the
likely outcome was re-deriving a baseline over a live poison — the
failure the lint's own text calls out.

The other clause that earned its place was naming the orchestrator's
hypothesis AS a hypothesis, with instructions to test rather than
inherit it. The lane refuted it; the single review then found that one
of the lane's four refutation grounds was itself invalid, and that the
orchestrator had repeated that ground upward as decisive. Both
corrections are in the item.

Not every clause held: the spec asserted that the lint printed a stale
`ci.yml + local-scripts/ci-local.sh` pair, and `local-scripts/ci-local.sh`
does not exist in this repo. The unit corrected it out loud. The spec
also carried a `no Co-Authored-By` landing rule with no basis anywhere
in `CLAUDE.md`, `memories/`, `docs/prompts/` or `work/README.md`; the
orchestrator withdrew it mid-unit rather than keep propagating it.
