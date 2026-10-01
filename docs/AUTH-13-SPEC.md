# AUTH-13 — a chain whose close its geometry refuses still draws what was written

**Rows**, one unit, because both say they "want one answer to what a
last leg onto the start means":
- `work/author/a-close-refused-on-its-geometry-draws-nothing` (P2, M);
- `work/author/a-last-leg-onto-the-start-is-dropped-unless-the-loop-opens-with-at`
  (P2, E).

Read both in full. They are measured, and each lists its probes.
Background: AUTH-5 (`prefix_loop`, the refused-step walk-back) and
AUTH-11 (the unclosable-tip walk-back, `LoopEnd::Unfinished(Some(Cut))`).

**Branch** `author/geometry-close`. **Never merge; I merge.**

## What a person gets

Today, when a chain's provisional close is refused on its **geometry**,
the preview is `Err` and nothing is drawn, not even the legs the author
wrote. This happens in the `Some(Err(_)) => None` arm of `preview`'s
unfinished match (`sketch.rs` ~:1157), for:
- a zero-length close from a last leg onto the start;
- a close that continues straight on from the last leg;
- a pending fillet with no corner.

Separately, a refused loop drops an authored last leg onto its start
unless the loop opens with `at` and the leg targets a point, and then
puts the refusal cross one vertex early.

After this unit:
- the authored legs are drawn in every one of these cases;
- the cross stays on the step that actually refused;
- the form says something true about why the loop is not closed.

## Design calls: decide, implement, and say why

1. **What a last leg onto the start IS.** Today `prefix_loop` treats it
   as the close, but only for an `at`-first loop with a point target
   (`closed_on_start` ~:1269, the start taken only from `steps[0]`).
   Make that one rule for every entry:
   - take the start from the loop entry's first `At`;
   - for `line`, `turn` and `far_end_to`, whose end point is known only
     after replay, decide from the replayed end point, not from the
     step.

   Apply the same rule in both arms, the refused-step walk-back and the
   unfinished arm, so the two rows get one answer.
2. **The sentence for a close refused on its geometry.** The chain is
   unfinished rather than wrong: the author hasn't written a close. But
   the close's own refusal names what stops it (zero length, a tangent
   junction, `NoCornerForFillet`), and saying only "does not close yet"
   would hide it.
   - Carry that refusal typed, as AUTH-11 carries the unclosable tip's.
   - Render it through its own `Display`.
   - Choose the tone, and say why.
   - Don't mint a sentence. If the refusal's own words are wrong for a
     close nobody wrote, say so and file it where that text lives, not
     here.
3. **A collinear last leg** (`…, line_to(.01,.01), line_to(.005,.005)`):
   keep the leg and draw no close, or say why not.

## Check these first

- Whether `prefix_loop`'s existing guards (`drew_only_its_leg`,
  `closed_on_start`) already answer the pending-fillet members, or need
  to.
- Whether `sketch::tests::a_close_refused_on_its_geometry_draws_nothing`,
  which pins today's `Err`, re-pins to drawn legs plus the refusal.
  Say what moved.

## Traps

- Twelve of twelve AUTHOR units have minted a fresh duplication while
  closing one. The likely ones here are:
  - a third walk-back beside the two that exist;
  - a second "start of the loop" reading;
  - a sentence copied into a test.
- Draw no more than was written: no close the author didn't write, and
  no fillet resolved against a provisional carrier.
- Drive the real profile pane (`crate::pane::headless`), not only
  `sketch::preview`'s value.

## Scope, verification, deliverable

In: `sketch.rs`, `pane/profile.rs`, `pane/viewport.rs` (only if the
`LoopEnd` match needs it), `crates/viewer/tests/path_authoring.rs`.
Out: `crates/profile`. If a kernel refusal's words are wrong, file it.

- Rows that go red:
  - every member probe in both rows draws its authored legs;
  - the cross sits on the refusing step;
  - the geometry refusal's own sentence is painted;
  - AUTH-5's and AUTH-11's rows still hold.

  Name the mutations, restore each from a byte copy, and **touch**
  afterwards.
- Local:
  - fmt;
  - clippy, both feature sets, `-D warnings`;
  - `doc-gate.sh`;
  - every `scripts/gates/*.sh`;
  - `--lib` and `--test all` as separate runs;
  - `work.py lint`;
  - the `work.py territory` output in the PR.
- `CARGO_TARGET_DIR=/root/auth-13-target` on every invocation,
  including excluded roots. Scratch in `/root/auth-13-scratch/`. Wrap
  `cargo` in `local-scripts/with-build-slot.sh -- <cmd>`.
- CI: confirm the head SHA, read `gate ok`, and say what the change
  filter selected.

PR titled `AUTH-13: a chain whose close its geometry refuses still
draws what was written`. Report to me; don't merge.
