# GREEN — the plan

Main green and the per-PR gate fast. Opened 2026-10-28 by a
priority-seam cut across the infra group (`work/green/log.md`).

## State

No orchestrator holds the track (`status: ready`). Its P0 and P1 rows
are closed (2026-11-15):

- **Main reds.** The nightlies of 11-13 and 11-15 were read whole, from
  each leg's junit, and every red was re-blessed against a named merge,
  pinned on a row on the program that owns the code, or filed (PRs 6921,
  6931, 6934, 6956, 6967, 6985). Failing tests went from 22 / 35 / 38
  (default ε / 1e-6 / 1e-12) on 11-13 to 10 / 11 / 15 on 11-15, and
  most of the 11-15 reds were then greened or are BOXFINISH's own
  (P0, its lanes). The 11-16 nightly is the first run with PR 6985 in
  it: read it first. Three fuzz rows are red on some seeds until their
  owners fix them (GERM, SPLICE, HONEROOT); a fuzz row takes no fixed
  seed, so GREEN files and pins the counterexample and cannot green
  the row. GERM's is a false certificate, P0.
- **Nightly.** PR 6932: the 1e-12 leg is three hash shards (42, 31 and
  29 minutes on 11-15, against 114 of 120), a coverage job holds every
  ε row to the build's own listing, the mutant job reads junit, the
  viewer app job runs every test, the all-features listing is its own
  step. On 11-15 all of that ran as designed. Not yet shown on a
  runner: the probe sweep finishing its dumps and the two lints
  running behind it (PR 6985 pinned the demo wall that stopped it), a
  budget kill, and a listing failure. The render lanes pin Python 3.12
  (PR 6985) after the runner image's default moved to 3.14.
- **Latency.** The alarm was red on 11-15 (median 20.8 minutes over
  128 runs). GREEN's own greening lanes were 22% of that day's
  run-minutes; without them the median is 19.6.

**Why main reddens daily**, each with its row, none fixed here:

- a PR that moves a census's site or an id-keyed digest runs none of
  the slow rows it moves (`work/ciw/tree-wide-source-censuses-in-the-slow-set-run-on-no-pr-that-moves-a-site`,
  `work/ciw/a-diff-that-moves-corpus-ids-runs-none-of-the-slow-set-id-pins`,
  `a-src-change-that-moves-a-decide-site-skips-its-crates-recourse-roster`);
- the gate's slow-set step runs a slow test only when the diff's added
  lines spell its name, and only at the default ε, so a PR that edits
  or pins one gets a green gate that did not run it
  (`work/ciw/an-edit-inside-a-slow-test-that-does-not-respell-its-name-runs-it-nowhere-on-the-pr`);
- tests just under the fast set's 1 s bar red unrelated gates on runner
  variance (`fast-set-tests-half-a-second-to-0-8-at-opt-0-on-the-slow-runner`).

Until the first two are closed, greening the nightly is a daily
sitting of about one lane.

## The slate

The board has it (`python3 scripts/work.py status --program green`):
P2 rows and below. `test-shards-rebuild-dependencies-their-cache-holds`
pays little wall clock (the rebuilds overlap the first workspace libs),
and its priming fix adds runner-minutes on every main push, so weigh it
against queueing before taking it.

## How the track runs when picked up

Each morning, read that night's `triage/nightly-<date>` PR and green
what it left red, under `program.md`'s two rules: a red is greened,
not fixed, here; and GREEN acts on the triage, not beside it. Wait for
the triage table before touching any red. No triage PR was open on
11-14 or 11-15, and the sittings read the nightly themselves: download
each leg's `full-suite-junit-*` artifact and read every failing test,
since a job's log tail shows a fraction of them.

What the 11-14 and 11-15 sittings learned about running the lanes:
one unit per kind of red and per ε row, each lane its own cloud
session; a pin PR's green gate says little (above), so each lane reads
the gate's step log for which tests ran and runs the rest locally at
the nightly's opt-level 2; a lane's "unmeasured" or "not settled" in
an owner's row is usually decided by a short probe, and reviewers
decided every one they tried; each pin PR merges on the reds it
inherits from main, named in its body, since the lanes are red on each
other's reds until all land.
