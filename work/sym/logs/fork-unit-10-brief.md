# SYM-10 implementer brief — the decision door and the floor: the folds the sign-hull frame needs

You are the implementer lane for unit SYM-10 of program SYM (the E12
symbolic identity tier, `geom_core::sym`). **The binding text is
`docs/SYM-10-SPEC.md` in your worktree — read it in full first.** Then
`docs/prompts/implementer-discipline.md` in full; then the spec's
"Read first" list in its order — the item whole
(`work/sym/the-decision-door-is-opaque-to-the-tier.md`, both PROPS
measurements and SYM's reading), the SYM log's 2026-09-19 answer to
PROPS, `sym.rs`'s header and `combine`'s `Min | Max | Copysign | Atan2`
arm and the `Select` arm as they stand on your MERGED tree,
`sym/signed.rs` whole, `sym/manifest.rs` (rule F — on
`origin/sym/8-manifest-sign` if #2616 has not landed; merge that branch
into yours too if it has not, and say so), `sym/quotient.rs`, the
sign-hull construction in `linalg/vec.rs` and `real.rs`'s
`select_le_zero`, the three rows the spec names, and PR #2468's body
(`gh pr view 2468 --json body --jq .body`). `CLAUDE.md` and
`work/README.md` apply as to every lane.

## Your lane

- Worktree `/home/evan/cad-lanes/sym-10`, branch `sym/10-decision-door`,
  at `origin/main`. **Your first commit is a merge of
  `origin/props/sign-hull` into your branch** (a merge commit, never a
  rebase; resolve any conflict keeping both sides and say what you
  resolved), so every measurement is on `main` + the construction.
  Every command: `cd /home/evan/cad-lanes/sym-10 && …`.
- **Private build directory, always:**
  `CARGO_TARGET_DIR=/home/evan/cad-lanes/sym-10-target CARGO_INCREMENTAL=0 nice -n 10 cargo …`
  on one line with the `cd`. A warm build is being made in it as you
  start: **wait until `/home/evan/cad-lanes/sym-10-target/SEEDED`
  exists** before your first cargo command (a foreground `ls` poll;
  the seed is queued behind two review lanes' seeds and this box is
  slow today — read everything first). Private scratch:
  `/home/evan/cad-lanes/sym-10-tmp/`.
- **This is a SHARED machine**: 8 cores, ~5 GB of RAM free, four
  orchestrators' lanes, load ~30. `nice -n 10` every cargo; one crate
  at a time; ONE heavy row at a time, detached and polled; the pad's
  shape report OOMs on a bigger box than this — do not run it.
- Do not read, fetch or build any other lane's worktree, target or
  scratch (`/home/evan/cad-lanes/sym-8*`, anything else under
  `/home/evan/cad-lanes` or `/home/evan/.mngr/worktrees`). `pgrep -f`
  with your own paths only; never `pgrep -af`, `ps` or `git worktree
  list` unfiltered.

## Foreground rule (both halves)

Never arm waiters or background chains for your own builds and tests.
A job that could outlive a 600 s foreground call runs `setsid`-detached
with output under `/home/evan/cad-lanes/sym-10-tmp/` and its PID
recorded, then is polled in the FOREGROUND (an until-loop inside one
Bash call; re-issue on timeout after killing your own previous poller).
Never end your turn with a detached job still running.

## What you deliver

Exactly the spec's Phase 1, Phase 2, Scope and Acceptance. **Phase 1
before anything else** — the three rows red, reproduced; the chains
rendered with the atoms counted by op and the fold each needs; each of
the three pieces hand-planted (reverted) and the table of which
together turn each row green — in the PR body before any rule is
written. **If the three together do not, STOP after Phase 1 and report
the fourth piece**; the fork returns to Ev. Then the folds behind a
dial in the spec's homes (manifest order and the manifest bound beside
rule F; the `Select` read beside rule C's `sqrt`/`abs` reads, with
factor stripping argued at the enclosability test), the theorem and
negative rows, the three rows green WITHOUT edits to their assertions,
the ledger re-baselined with its reason, every other split and ceiling
unmoved, the cost on both instruments, the `sign_gated` row.

Commit early and often; push after each coherent step
(`git push -u origin sym/10-decision-door`) — this machine has lost
lanes before and only pushed work survives for certain. **No
`Co-Authored-By` trailer in your commits** — if one lands in a pushed
commit, note it in the PR body and carry on; never rewrite history.

Before the PR: `cargo fmt --all`; clippy `-D warnings` on `geom-core`
(default, `interval`, `interval,sym-profile-testing`, all targets) and
`editor-core` (`--features interval`, all targets); `scripts/doc-gate.sh`;
`python3 scripts/work.py territory --base origin/main` listed in the PR
body (the `crates/editor-core/tests/m10_*` overlap with S-TCOST and
S-TINT is the announced tests-family one; `linalg/vec.rs` and `real.rs`
are NOT touched — the sign-hull diff in your tree is PROPS's, merged,
not yours); `python3 scripts/work.py lint`. Set `status: review` on
`work/sym/SYM-10.md` in your last commit before the PR. **Open the PR
with `gh pr create --base props/sign-hull --head sym/10-decision-door`**
(the spec says why: hosted CI runs only on PRs to `main`; #2468's next
run after the merge is the hosted gate and the PR body names it so),
title `SYM-10: the decision door and the floor — …`, NOT draft, body
per the spec; scan the body for `fixes|closes|resolves #N` shapes and
break any; end the body with the line
`🤖 Generated with [Claude Code](https://claude.com/claude-code)`. Do
not merge.

## Your report (≤120 lines)

PR number and head SHA; the Phase 1 tables (atoms by op per red row;
the pieces each needs; the hand-plant table); the two predicates as
stated before the code; the mechanism and the dial(s); the three rows'
state with their assertions untouched; the `sign_gated` and
`symbolic_zero` columns before/after; splits and ceilings on the six
documents; the cost on both instruments; every deviation from the spec
and whether it is an improvement or owes a follow-up; rows filed
outside your fence and where; the local full checks' output; anything
you could not do and why.
