# ATREST-2 — what the sense bit means on an arc-capped loft

**Binds one implementer lane.** Deleted at merge (`docs/DOC-LEDGER.md`);
`work/atrest/ATREST-2.md` is the record that survives.

Read `docs/prompts/implementer-discipline.md` in full first. It is
binding alongside this spec.

Branch `atrest/2-sense-measure`. Program ATREST, prefix `atrest/`.

## This unit measures. It does not fix.

Read `work/atrest/sense-inversion-is-invisible-to-tier-3-on-arc-capped-lofts.md`
in full. A PERF-6 reviewer found that
`Body::flipped_face_sense_for_tests` applied to EVERY face of the
three-station `arc_section` loft leaves tier 3 **green**, with the
body's volume enclosure unchanged and positive — while the same
whole-body inversion on `square_prism` or on step-export's
`loft_prism` refuses `LoopRoleInverted`.

The row is filed `H` because nobody knows what it is yet, and
`memories/refusal-text-is-not-cause.md` is the reason this unit exists
in the shape it does: **a refusal's text is not evidence of its
cause**, measure-first is a mandatory checkpoint, and the payload and
the raising site are the instrument. The row's own body already rules
out the obvious wrong answer — this is **not** a check-7 bug, because
the quadrature's Green form is winding-derived end to end and no sense
bit enters it, so flipping the bit cannot move the flux.

**Do not design a fix.** If you find yourself writing a new check,
stop: that is the orchestrator's call and it depends on what you
measure.

## The three questions, which are your deliverable

1. **Which pass produces `LoopRoleInverted` on the polygonal lofts,
   and what about the arc-capped body's loops makes it silent?**
   Name the raising site and the predicate that goes the other way.
   The row lists three candidate causes — a rim role derived rather
   than stored, an arc cap whose role is not computed, a pass gated on
   something the arc body fails earlier. **Those are the dispatcher's
   hypotheses, not findings** (`docs/prompts/reviewer-style-lane.md`
   §1): check them against the tree and report a correction as a
   result in its own right if none of the three is it.
2. **Does any at-rest check on an arc-capped body read
   `face.sense_sign()` at all?** Enumerate the reads; say for each
   whether an arc-capped body reaches it, and if not, what stops it.
3. **Is a body built inverted — as opposed to flipped after the fact
   through a `_for_tests` door — reachable through the PUBLIC API?**
   This is the question that decides whether the finding is a real gap
   or an artefact of a test door, and therefore what the next unit is.
   Answer it with a constructed body or with a stated, evidenced
   reason no public door can produce one.

## What you owe

1. **The answers, written into
   `work/atrest/sense-inversion-is-invisible-to-tier-3-on-arc-capped-lofts.md`**
   under a `## Measured (2026-09-20)` heading, replacing its "What to
   measure before anything moves" list with what you found. Cite by
   **name**; a number may ride along and may go stale, a bare
   `file.rs:NNN` is not a citation.
2. **A row per answer that can go RED if the answer changes.** This is
   the point of the unit: a measurement with no guard is a sentence
   someone will have to re-take. The natural home is
   `crates/topo/src/tier3_tests.rs`, unless what you measured lives
   elsewhere — say why if so. Ask of each row *can this go red*, not
   *does it pass*: an assertion monotone in the wrong direction, or a
   premise that excludes the failing mode, is not a guard. **Write
   assertions a bug could break** — name the runtime value that would
   make each false.
3. **The sweep, per discipline §5.** The class is not "arc-capped
   lofts": it is *at-rest checks that read a stored bit on a body
   class that never reaches them*. Grep for the shape, put the hit
   list and its disposition in the PR body one line per hit, and say
   **what the pattern could not match**.
4. **File what you find outside the fence** (discipline §6), on the
   owning program's slate, in this PR — `python3 scripts/work.py
   territory --files -` says who owns a path. Grep that program's
   directory first so you add evidence to an existing row rather than
   opening a second.
5. `python3 scripts/work.py lint` green.

## What you do NOT owe

A fix, a new check, a new `ValidationError` variant, or a
recommendation dressed as one. **A recommendation as a recommendation
is welcome** and belongs in your report to the orchestrator, clearly
marked — what is out of scope is landing it.

## Verification

Hosted CI is the verification of record: twelve `test (…)` jobs and
five `k-lint (gate, …)` jobs on a code-tier run. Read the **run's**
jobs API at the step level, never job-name green — a green job NAME
can sit over a skipped step.

Private paths, per `memories/agent-lane-operations.md`: your own
`CARGO_TARGET_DIR` **outside** the worktree, and a private scratch
directory — never the session scratchpad, which every lane of this
session shares.

## Review

Outside protocol v7 — opus implementer, opus reviewer. Style lane
(`docs/prompts/reviewer-style-lane.md`) plus **one** correctness
claim: *that the measurements say what the report says they say, and
that each pinned row can go red.* No A/B draw, no ordinal, no row.
