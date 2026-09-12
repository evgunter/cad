---
id: rust-cache-never-restores-across-branches
kind: issue
title: Swatinem/rust-cache restored nothing on five of seven build jobs: a branch's first build inherits no cache, and F3 means main never saves one
status: closed
opened: 2026-09-04
refs: [1648, 853]
closed: 2026-09-12
---



Filed by the CIW orchestrator, 2026-09-04, from TCOST-C4's own PR
(1648, merged) — finding (d) there, recorded under F4 of
`docs/CI-MINUTES-2026-08.md` and explicitly not fixed by that unit.
Filed here rather than in `work/ciw/` because caches are a CI build
knob, which CIW's `keep_out` gives to this program.

## The measurement, from C4's seven build jobs plus its control

`Swatinem/rust-cache` reported `No cache found` on **five of the seven**
build jobs C4 ran, and on the control run `33719350040` as well — each
time compiling all ~300 units. Only two jobs restored anything, and
both restored a save this same branch had made minutes earlier. Every
miss row's JSON under `docs/perf-data/sccache-trial/` names the key it
missed.

## Why, and it is structural rather than a budget accident

GitHub scopes an Actions cache entry to **the branch that saved it plus
the repository's default branch**. A branch therefore inherits an entry
only from `main` — and **F3 means `main` never runs the build job at
all** (`docs/CI-MINUTES-2026-08.md` §F3: a `push: main` run is reduced
to `filter` + `rebuild-latency` + `renders`). So no PR can inherit a
build cache from anywhere, and the only entry a branch can restore is
one it saved itself on an earlier push.

Two independent evictions compound it, both measured by C4:

- the repository's 10 GB Actions cache budget churns entries out inside
  the hour — the ~205 MB per-lane sccache object entry restored at 9
  and 17 minutes and missed at 38, 60 and 88;
- `rust-cache` hashes `RUST*`-prefixed environment variables into its
  key, so any env change on the build job buys one cold rebuild.

## Why it is worth a unit now rather than a note

C4's own framing: *"That is where the next minute lives, and it is not
sccache."* The build jobs are the gate's critical path (F4:
`build + archive (interval)` alone is ~88% of a 13.75-minute critical
path, measured at 2 vCPU / 7 GB), and a cold ~300-unit compile is what
most PRs are paying on their first push. The repository going public on
2026-09-03 removes the *billed-minute* half of the argument and leaves
the *latency* half untouched — it is now the whole argument.

The obvious lever is priming: something on `main` saves an entry PRs can
restore. That interacts with F3 directly, so it is not independent of
`work/ciw/f3-recosting-on-a-public-repo`, which is measuring what a
restored full `main` run costs in wall clock. Whichever of the two moves
first should say so to the other; neither should assume the other's
answer. Sizing, the arm to prime, and whether an entry survives the
budget long enough to be worth saving are all this unit's measurement
to take.

Also unmeasured and cheap to establish: whether the 4 vCPU / 16 GB
public runner changes the cold-compile figure enough to move the
priority.

## Closed (2026-09-12): TCOST-B3 landed the lever, and it works

The after-reading this row owed and never took. Its premise — *"restored
nothing on five of seven build jobs"* — is false on today's tree, and the
fix is the one this row proposed: **TCOST-B3 (PR 1684) landed the
`push: main` primer jobs that save under the shared keys the build jobs
restore**, plus a parity checker. This row was never updated and has sat
open describing a tree that has moved.

**Measured**, over the 32 most recent completed pull-request runs of
`ci.yml`, every `build + archive` job that executed:

| | default lane | interval lane |
|---|--:|--:|
| job duration, median | **251 s** | **279 s** |
| range | up to 362 s | up to 440 s |
| jobs with a readable `rust-cache` restore step | 16 | 16 |
| of those, miss-shaped (<= 5 s) | **0** | **0** |
| restore step, median | 13 s | 13 s |

Against this row's own cold figure of **820 / 840 s** and a restore step
that on a miss is a couple of seconds, every sampled job restored.

**Grounded rather than inferred.** The step duration is a proxy, so one
job's log was read directly to check it: run 34668412928,
`build + archive (interval)` (job 103484994105), post-step —
**`Cache up-to-date.`**, which is what `Swatinem/rust-cache` prints when
it restored a full match and has nothing new to save.

**One gap, named because the sample cannot close it.** These 32 runs are
a mix of first pushes to a branch and later ones, and nothing here
separates them — the original finding was specifically about *a branch's
first build*. With 0 of 16 miss-shaped in each lane it is unlikely the
first-push case is systematically different, but a reading that isolates
it was not taken. If anyone sees `No cache found` on a build job again,
this row reopens with that run named.

Closed rather than re-scoped: the residual question in the section above
— whether the 4 vCPU public runner changes the cold-compile figure — is
moot while nothing compiles cold.

**One half of this row did NOT close with it and has its own file.** The
primer works by REFRESHING a shared key on every main push, so an
eviction costs one push's staleness; that says nothing about an entry
written ONCE under a hash key, which is what TCOST-C4 measured churning
out inside the hour and what `work/ciw/cache-rendered-cells-on-input-hash`
is parked on. Split to
`work/tcost/actions-cache-budget-under-a-hash-key` in this same commit,
and that CIW row re-parked onto it — a closing row may not un-park
another program's item by leaving its blocker dangling
(`work/README.md`).
