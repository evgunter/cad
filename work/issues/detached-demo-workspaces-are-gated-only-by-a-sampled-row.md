---
id: detached-demo-workspaces-are-gated-only-by-a-sampled-row
kind: issue
title: demos/tour and demos/wild are detached workspaces, invisible to workspace-wide clippy and gated only by a sampled k-lint row
status: open
opened: 2026-09-04
---


Named by FIX's `no-parametric-loop-constructor` lane after a second
independent lane tripped the same trap within an hour. Filed by the FIX
orchestrator; CIW is the natural claimant. Sibling of
`gui-wasm-build-is-not-gated-at-all` — same shape, different consumer.

## The combination

`demos/tour` and `demos/wild` are **detached workspaces** (excluded
roots; `cargo fetch` treats them separately). Two consequences compose
into a hole:

1. **A `--workspace` check cannot see them.** So a lane that changes a
   crate signature and dutifully re-spells every caller under
   `cargo clippy --workspace --all-targets` has done the natural,
   correct thing and still missed two consumers.
2. **Their only gate is a SAMPLED row.** `demos tour fmt + clippy`
   runs when `klint_row` draws `dev-default` or `all` (`ci.yml:3600`).
   A run that draws otherwise reports `k-lint (gate)` **success** over
   a skipped step.

So the breakage is invisible to the check a careful lane runs, and the
gate that would catch it fires only sometimes. Neither half is a
mistake on its own; together they are a hole that lands on whoever
happens to draw the row next.

## The instance, measured

PR #1756 (`shell/1-naming`, `6caaa7d2b`) changed `topo::shell` to return
`Shelled<T>`. Two statements in `demos/tour/tests/verbs_teapot.rs` became
bare field accesses (`….body;`) — `clippy::unnecessary_operation`,
denied. #1756's own `k-lint (gate)` concluded **success** with the demos
step **skipped**.

Main was red from that merge until FIX's PR 1775 (a two-line fix by an
orchestrator who did not own the code). In between:

- one FIX lane hit the red on an unrelated diff, reproduced it on a
  detached worktree at `origin/main`, and traced it correctly;
- a second FIX lane hit it, diagnosed it correctly, and **also** noted
  that a green `k-lint` on its own head proved nothing because the row
  had not drawn;
- a third lane concluded from a green `k-lint` that main had been
  fixed, when it had not — the same signal, read the other way.

Three lanes, one defect, and the third drew a false conclusion from the
identical evidence. That is the cost this issue is about: not the two
lines, but that the signal is ambiguous by construction.

## Also true of `demos/tour/tests/` specifically

The `render lanes` job **does** execute the tour end to end (`demo tour
(STL + STEP + UV SVGs + scenes.json)`), which covers `demos/tour/src`.
It does **not** compile `demos/tour/tests/`. So a lane reasoning "the
render lane ran the tour, so the tour is covered" is wrong for exactly
the directory where this red lived.

## Dispositions worth weighing

1. **Make the ambiguous signal unambiguous.** The cheapest real fix, and
   the same one `gui-wasm-build-is-not-gated-at-all` argues for: a
   `k-lint (gate)` that skipped its demos rows should say so in a way a
   reader sees without the jobs API. A job name that means two different
   things is the whole defect.
2. **Unsample the demo rows.** They are compile-and-lint, not test
   execution. If the cost is small, drawing them always removes the
   class rather than labelling it.
3. **Give signature changes a demo check.** Narrower: a lane changing a
   public kernel signature is told, by a doc rather than by CI, that
   `--workspace` does not reach the demo roots.

(1) and (2) are not exclusive and (2) subsumes the instance. Not decided
here.

## Counter-example worth keeping

The same lane noted `reader_census` caught its new source-reading row
immediately and correctly — **and that census is not sampled.** So the
defect is not "censuses are unreliable"; it is specifically the sampled
axis over a consumer no other check reaches.

## Home

`work/issues/` — `.github/workflows/ci.yml` is CI ground and CIW is the
open program there. Re-home by header edit.
