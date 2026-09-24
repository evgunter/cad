---
id: section-of-re-derives-the-whole-f64-precompute-the-profile-node-already-made
kind: issue
title: wire::section_of redoes a section profile's entire f64 precompute (resolve, replay, validate, naming) though the profile node already computed ProfilePre, which NodeValue does not carry
status: open
opened: 2026-09-12
refs: [2435]
priority: P1
cost: H
---


## Finding

Found by the `frame-f64-placement-once` lane (PR 2435) in its sweep,
one level up from its own unit and outside it. Reported rather than
widened, which was the right call — filed here by the WIRE orchestrator.

`wire::section_of` re-derives a section profile's **whole f64
precompute** — resolve, replay, validate and naming — although the
profile NODE already computed exactly that as `ProfilePre`. It is not
carried: `NodeValue` holds only `ProfileValue { validated, naming }`.

**This is `frame-f64-placement-is-re-evaluated-per-profile`'s shape one
level up, and far more expensive.** That row was nine slots re-evaluated
per profile; this is an entire program resolve-and-replay-and-validate
re-run per section.

## Why it is its own unit and not a rider

It reaches **PP1/PP2's structure record** (`crates/editor-core/README.md`,
ratified with a recorded hedge, #1151) — *"structure f64-once as the
witness"* is precisely what `ProfilePre` is, so widening `NodeValue` to
carry it is a change to what the witness is and where it lives, not a
threading change. PR 2435 established the cheap version of this
argument for the frame placement and found it was an **instance** of
PP1; a taker here should not assume the same answer, because carrying
the structure record is closer to PP1's subject than carrying an input
to it.

Read PR 2435's report before starting: it settles the method (carry on
the value, read at the consumer, prove bit-identity by construction
rather than by measurement, and check content keys directly rather than
through tests) and its outcome on keys was **negative** — worth knowing
whether that survives at this size.

## Two neighbours the same sweep found, dispositioned and not filed

Recorded so the next sweep does not re-derive them:

- **`wire::lane_profile` re-resolves the program at `T`** though
  `eval_node` resolved it in the same environment (`LaneEnv { params:
  &env }`). Already **declined in writing in-tree**, above
  `lane_program`, with a D9 argument and the cost stated — so it is a
  disposition, not an oversight.
- **`mate::member::node_slots`** is not fixable by a carry at all: the
  solve produces `SolvedPoses`, an *input* to `OpEnv`, before any node
  has a result. Structural, not an oversight.

## Where else to look

The sweep that found this keyed on `eval_slots(`, `.resolve(`,
`doc.node(` and every `ParamEnv`-taking function in
`crates/editor-core/src`. Its stated blind spots are this row's too: a
re-derivation through a **kernel crate's** own functions (a face pose, a
frame normal recomputed off a body); a duplicated derivation two calls
down where no call site spells the idiom; and anything outside
`editor-core`, which was checked only for readers of `Evaluation`.

## Three consumers now wait on it (EDIT, 2026-09-16, PR 2759)

Evidence added rather than a second row opened: this row's subject —
`ProfilePre` not being on `NodeValue` — is now what blocks a built
door, not only what makes `section_of` expensive.

`ProfileProgram::profile_edges_of` (DM8,
`crates/editor-core/src/program.rs`) takes a
`profile::ProfileStructure` and an `eval::ProfileNaming` and answers
which `ProfileEdgeRef`s one authored step became. `ProfileValue`
carries the naming; the structure is on `ProfilePre`, which is
`pub(crate)`. So:

- the door **has no caller outside its own tests**, and its acceptance
  rows must REBUILD the structure through the public replay doors and
  pair the rebuild with the evaluation's real naming — a third copy of
  the walk this row is about, in
  `crates/editor-core/tests/edit_step_segments.rs`;
- `work/view/focus-marking-is-per-node-not-per-segment` (VIEW) cannot
  consume the door from the viewer;
- `work/lib/python-has-no-step-to-profile-edge-door` (LIB) cannot bind
  it from Python.

None of the three is a reason to decide this row's PP1/PP2 question one
way rather than the other — they are three more readers of whatever it
decides.
