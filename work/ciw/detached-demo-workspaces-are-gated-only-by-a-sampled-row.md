---
id: detached-demo-workspaces-are-gated-only-by-a-sampled-row
kind: issue
title: demos/tour and demos/wild are detached workspaces, so the clippy a lane runs before pushing cannot see them and CI is the first thing that tells them
status: closed
opened: 2026-09-04
branch: ciw/unreachable-roots
pr: 2263
closed: 2026-09-09
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

## Re-homed (2026-09-04)

Moved from `work/issues/` to `work/ciw/` in the tracker-wide
re-home sweep of 2026-09-04 (Ev's direction, in-chat), which read every
open `work/issues/` file against every open program's `paths` and
against the code-quality K–X fences. Id, body and header are unchanged;
the directory is the claim (`work/README.md`). Any `## Home` section
above naming `work/issues/` is superseded by this line and is kept as
the record of why the file was parked there.

## Re-read against the tree (2026-09-06, CIW orchestrator)

**Half 2 is discharged.** `demos tour fmt + clippy` and `demos wild fmt +
clippy` are still `if: matrix.row == 'dev-default'`
(`.github/workflows/ci.yml:4008`, `:4011`), but since PR 1850 the k-lint row
is not drawn: `ci.yml:460` fans all five unifications on every code-tier run,
so the `dev-default` leg — and with it both demo steps — executes every time.
A green `k-lint` over a skipped demos step is no longer a state this workflow
can reach. The title is stale from here on and the item is about half 1
alone.

**Half 1 stands, unchanged and untouched by any of this.** `demos/tour` and
`demos/wild` are still detached workspaces, so `cargo clippy --workspace
--all-targets` — the check a careful lane runs before pushing — still cannot
see them, and `clippy-all-features` (`ci.yml:2109`) is `--workspace` too and
does not reach them either. The hole is now one-sided rather than two: the
gate catches it, the lane's own local check does not, and the lane finds out
from CI instead of from its own run. That is a smaller defect than the one
filed, and a real one.

Its sibling `gui-wasm-build-is-not-gated-at-all` is NOT improved by 1850 —
that one is an `--exclude`, not a draw.

## Disposition (2026-09-09, PR 2263): nothing built, and why

**No new row, no new mechanism.** The question this unit was told to
answer first was whether the local gate already covers these roots. It
does: `local-scripts/ci-local.sh`'s `demos_hygiene` runs
`cargo fmt --check && cargo clippy --all-targets -- -D warnings` in
`demos/tour` and again in `demos/wild`, wired as
`run_row_if "$RUN_K_LINT" "demos tour (fmt + clippy)"`. Hosted runs the
same two steps on every code-tier run since 1850 (the 2026-09-06 re-read
above). Both gates are right; a third would have been a gate over work
two gates already do, and the k-lint rows would still be the ones that
fired first.

**So the gap is a habit, and it gets a sentence.** A lane's own check is
`cargo clippy --workspace --all-targets`, and `--workspace` reaches
nothing under the roots `Cargo.toml:22` excludes. That is not something
a careful lane would think to check, which is the argument this item's
sibling makes for calling it infrastructure rather than discipline —
except that here the infrastructure is already correct and only the
lane's model of it is wrong. §2 of
`docs/prompts/implementer-discipline.md` now carries a bullet saying so,
naming `demos/tour` and `demos/wild` as ordinary users of the public
API, and giving the two-line version of the check.

**The bullet's own first draft was the defect one level up**, and the
fix pass caught it. It said "the hosted gate and `ci-local.sh` both
cover every root", which is false for two of them: `benches` gets
rustfmt in the PR gate and its only clippy is `nightly.yml`'s, with no
local mirror at all, and `interval-transcendentals`' clippy runs only
when the filter buys `interval-backend`. It also said "five" roots where
`scripts/doc-gate.sh --print-roots` derives seven. **The bullet now
carries no count and tells the reader to run that derivation** — and the
same staleness turned out to sit in five places inside CIW's own fence
(`ci.yml`'s cache-scope paragraph, `ci-local.sh`'s rustdoc note, and
three lines of `scripts/doc-gate.sh`, one of them a selftest failure
message), every one of which had said "six" since before
`tools/tess-meter` landed. All five are fixed in this PR, by deletion of
the number rather than by correction of it. A sentence written to fix a
lane's model of coverage, which is itself wrong about coverage, is worse
than no sentence.

**Disposition (3) of the three listed above, then**, and (1) and (2) are
moot: (2) landed with 1850 and (1) has nothing left to disambiguate,
because the rows are no longer sampled.

**Announced to META**, whose fence `docs/prompts/*` is. The edit is a
§2 run-fact — what a local command does and does not compile — which is
the standing clause CIW writes under; it is one bullet in the
"When you do run locally" list and changes no rule.


## Closed 2026-09-09 — nothing built, and that is the finding

PR 2263. Both gates were already right: `ci-local.sh`'s `demos_hygiene`
runs fmt and `clippy --all-targets -- -D warnings` in both roots, and the
hosted `demos tour/wild fmt + clippy` steps have run on every code-tier
run since PR 1850 un-sampled the k-lint row. So there was no row to add.

What survived was a **habit**, not a mechanism: a lane's own pre-push
check is `cargo clippy --workspace --all-targets`, which reaches none of
the excluded cargo roots. The fix is one bullet in
`docs/prompts/implementer-discipline.md` §2 naming the roots, the two
demo consumers among them, the instance that cost two lanes an hour, and
the two-line version of the check.

The first draft of that bullet claimed both halves "cover every root",
which is false — `benches`' clippy is nightly-only with no local mirror,
and `interval-transcendentals`' is filter-gated. A sentence written to
correct a lane's model of coverage, itself wrong about coverage, is the
defect one level up; it now states coverage per-root and names
`scripts/doc-gate.sh --print-roots` as the derivation instead of counting.

That correction swept five more stale counts of the same roots inside
CIW's fence (`ci.yml`'s cache-scope paragraph, `ci-local.sh`'s rustdoc
note, three lines of `scripts/doc-gate.sh` including a `--selftest`
failure message) — all now cite the derivation. The prose said five, six
and seven in different places while `tools/tess-meter` had landed and
moved none of them.
