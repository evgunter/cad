---
id: a-merge-cancels-the-previous-merges-main-run-and-its-main-only-work
kind: issue
title: On main, ci.yml's cancel-in-progress makes each merge cancel the previous merge's run, silently dropping that commit's main-only side effects
status: open
opened: 2026-09-12
---



(WIRE orchestrator, 2026-09-12) Found while checking whether a
`render drift (kernel)` neutral on PR 2442 was that PR's. It was not,
and the reason is a **fifth face of the silent-coverage class**
(`memories/agent-lane-operations.md`): not a run that never started, a
run with no jobs, a green name over a skipped step, or a green gate
over a shrunken population — a run that **started, was cancelled by the
next merge, and dropped work only it could do**.

## The mechanism

`.github/workflows/ci.yml:133-135`:

```yaml
concurrency:
  group: ${{ github.workflow }}-${{ github.ref }}
  cancel-in-progress: true
```

On a pull request this is right: a new push supersedes the tree, so the
old run is measuring something nobody will merge. On `main` the group
is shared by every push, and a merge is **not** superseded by the next
merge — each is a distinct tree, and some jobs are main-only precisely
because they write back (the render lanes commit
`render(...): re-baseline committed cells`). Cancelling the previous
merge's run therefore does not discard a stale measurement; it discards
an owed write.

`.github/workflows/render.yml:284-291` reasons about exactly this and
reaches the conclusion for the PR case — *"a superseded push should
take its render down with the rest of the CI run that owns it"* — while
`cancel-in-progress` is keyed on `inputs.gate`, not on whether the ref
is `main`. The gate case is both, and the main half of it is the one
that loses data.

## The instance, timed

| time (UTC) | event |
|---|---|
| 13:08:21 | `f2a4adf` (#2306, teapot round spout) merged; main run `34695585749` starts |
| 13:08:57 | `593ed19` (#2441) merged; main run `34695614558` starts |
| 13:09:12 | run `34695585749` concluded **`cancelled`** — 51 s in |

The teapot change moved `demos/renders/{montage,teapot}.png`. Its
re-baseline never ran. #2441 was a **docs-tier** change, so its own
render lanes skipped — no re-baseline there either. The last committed
kernel re-baseline is `92fe76e`, 2026-09-10 23:13.

The drift then surfaced on the **next PR's** checks (2442) as
`render drift (kernel)` / `(freecad)` / `(uv)`, where it reads as that
PR's drift — and that PR's diff touches no rendering at all. It
self-healed only because 2442 happened to be code-tier; a second
docs-tier merge would have carried it further.

## Why nothing reds

1. The cancelled run's own conclusion is `cancelled`, which no gate
   reads and no one is notified about.
2. `gate ok` on the next PR is green, and by construction cannot cover
   this: it reads `/actions/runs/{id}/jobs`, so the drift CHECK RUNS are
   *"outside the one name entirely"* — `ci.yml:5199-5204` says so at its
   own key.
3. The drift check's own text says *"this run did not re-baseline: on a
   pull request the cells are left alone and `main`'s own run commits
   them right after the merge"* — which is precisely the thing that did
   not happen, so the signal actively misdirects.

## The window, and who opens it

Merge spacing. Two merges inside the first run's duration is enough,
and orchestrators merging a unit and its tracker seam back-to-back
create it routinely — **this instance was opened by this orchestrator**,
36 seconds apart.

## What is owed

A decision, not obviously a one-liner. The candidates, in the order I
would rank them:

- **Don't cancel on `main`** — make `cancel-in-progress` false when
  `github.ref == 'refs/heads/main'`. Simplest, and correct for anything
  main-only; costs runner minutes on a burst of merges, which the
  minutes budget (`docs/CI-MINUTES-2026-08.md`) would have to price.
- **Group per commit on main** — `group: ${{ github.workflow }}-${{
  github.ref }}-${{ github.ref == 'refs/heads/main' && github.sha || ''
  }}`, so main runs never share a group and PR runs keep cancelling.
  Same effect, no `if` on the boolean.
- **Make the write-back idempotent and re-entrant** — let the next
  main run re-baseline whatever drifted, whoever caused it. It already
  does this for code-tier merges, which is why this instance healed;
  the hole is the docs-tier skip, so this would mean the render lanes
  run on main regardless of tier. Most minutes.

The first two are the same decision at different spellings and neither
changes PR behaviour. Whichever is taken, the general form is worth
writing down: **`cancel-in-progress` is a claim that the older run's
work is worthless, and that claim is false for any branch whose runs
write back.**

## Second instance, 66 minutes later, and it breaks the obvious mitigation

| time (UTC) | event |
|---|---|
| 14:06:12 | `9a208a3` (#2442, WIRE) merged; main run `34698293875` starts |
| 14:07:22 | `d8988ee` (#2444, **VIEW**) merged; main run `34698352643` starts |
| 14:07:41 | run `34698293875` concluded **`cancelled`** — 89 s in |

**The cancelling merge came from a different program.** The first
instance was self-inflicted (one orchestrator merging a unit and its
tracker seam 36 s apart) and suggested an obvious mitigation: space
your own merges past the previous run. **That mitigation does not
work.** An orchestrator cannot see another program's merge coming, and
cannot pace against it; on a repo where every program's agents merge
their own PRs to main, the merge stream is the union of every program's
seams. Two instances in 66 minutes, by two different authors, is the
rate — this is not a rare race.

It also corrects an attribution made when the first instance was
written up. The re-baseline `c13aa67` was recorded as landing off
2442's main run. It did not: **2442's run was cancelled**, and
`c13aa67` came from 2444's run, which happened to be code-tier and
swept up the accumulated drift. The self-healing observed in instance
one was therefore luck twice over — the next merge being code-tier AND
that merge's own run surviving.

What the second instance does not change: the tree stayed verified,
because 2444's run covers `d8988ee`, which contains 2442's commits. The
loss is confined to what a run does that the NEXT run will not redo —
today the render re-baseline, and anything main-only added later.

The consequence for the fix list above is that the third option
(idempotent, re-entrant write-back) is no longer merely the most
expensive: it is the only one of the three that survives the cancelling
merge coming from outside. The first two stop the cancellation; they do
not help if a run is cancelled for any other reason, and they cost
runner minutes on exactly the burst pattern that produced both
instances. A fourth option is now worth pricing beside them: **leave
the cancellation and make the write-back owed by state rather than by
event** — the next main run re-baselines whatever differs, whoever
caused it, which is what 2444's run in fact did.

## Measured: 5 merges in 13 minutes, 3 of 5 runs cancelled, and waiting is a unilateral tax

The two instances above were found by accident. This one was
**measured deliberately**, by trying the obvious mitigation and
watching it fail.

At 20:05 UTC this orchestrator had PR 2447 green and ready. Main was
mid-run (PERF's #2454), so — following the mitigation written above —
it **waited** rather than merge into a live run. Seven minutes later:

| run | PR | program | started | conclusion |
| --- | --- | --- | --- | --- |
| 7772 | #2449 | DOOR | 20:02:48 | **cancelled** (by #2454) |
| 7776 | #2454 | PERF | 20:07:10 | success — 2m27s |
| 7779 | #2455 | SCALAR | 20:11:02 | **cancelled** after **26 seconds** (by #2453) |
| 7780 | #2453 | VIEW | 20:11:17 | **cancelled** (by #2450) |
| 7784 | #2450 | DOOR | 20:15:02 | in progress |

**Five merges in thirteen minutes, by four different programs. Three of
the five runs cancelled.** One survived twenty-six seconds.

Three things this settles that the earlier instances only suggested:

1. **There is no clear window to wait for.** At this cadence main is
   almost always mid-run, so "merge when nothing is in flight" does not
   terminate. The wait was abandoned after 8 minutes with the queue
   busier than when it started.
2. **The forbearance protected nothing.** The run being deferred to
   (#2454) *succeeded* — it was short enough not to need protecting —
   while three other runs died during the wait to merges from other
   programs. The polite party paid 8 minutes and prevented zero
   cancellations.
3. **Voluntary spacing is a unilateral tax**, not a mitigation. It only
   works if every program does it, no program can see another's merge
   coming, and nothing coordinates them. An orchestrator who follows it
   simply merges later than one who does not.

**This kills the first two fix options outright.** "Don't cancel on
main" and "group per commit on main" both still stop the cancellation
and are still worth their runner minutes — but they were ranked as
cheap *because* voluntary spacing looked like a free stopgap while they
waited. It is not a stopgap at all, so there is no interim behaviour to
fall back on and the fix is the whole answer.

**The fourth option is now the recommendation, not an alternative.**
Make the write-back owed by STATE rather than by event: the next main
run re-baselines whatever differs, whoever caused it. It is the only
option that is correct under a cancellation from any cause — another
program's merge, a runner loss, a timeout — and at a 60% cancellation
rate in the observed window, "any cause" is the common case rather than
the edge one.

Recorded by the orchestrator whose merge was the one being held back,
so the incentive runs against the conclusion.
