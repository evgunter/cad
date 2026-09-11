---
id: render-hosted-knows-four-lanes-and-there-are-six
kind: issue
title: render-hosted.sh knows four lanes and the repo has six
status: review
opened: 2026-09-11
branch: ciw/render-hosted-lanes
pr: 2325
---



`local-scripts/render-hosted.sh` is the helper for taking a hosted
render pass from a developer box: push, dispatch, poll, install the
result. It knows the lanes `kernel|freecad|uv|wild`, and it has never
known `mc`.

```
$ grep -c mc local-scripts/render-hosted.sh
0
```

The `mc` density lane landed at #2284 and `render-mc.sh` has been a
committed lane since, so `--lane mc` has been unreachable through this
helper for as long as that lane has existed. The GUI lane (#2317 /
#2318) makes it two.

**The header says the stale number out loud, three times** — "CI now
RE-BASELINES all four lanes itself" (:12), "ALL FOUR LANES RE-BASELINE,
uv included" (:18), "branch renders all four lanes" (:31). Those
sentences were true when written. A count in prose goes stale silently,
which is the same defect `render.yml`'s own header carried until #2318
replaced its counts with a roster and a sentence saying why a count
does not belong there. The same fix applies here.

**Found rather than fixed, and the reason not to fix it in passing is
worth keeping.** This surfaced while wiring the GUI lane. Adding `gui`
to the lane map while `mc` stayed missing would have made the file
wrong in a NEW way — a helper that knows five of six lanes reads as
complete in a way that one knowing four of six does not, because the
gap stops looking like drift and starts looking like a decision. So
neither was added.

**What closing it looks like:** teach the lane map and the install list
both missing lanes, and replace the three prose counts with a roster
that names the lanes — or better, with a reading of the lanes
`render.yml` actually declares, so the next lane cannot leave this file
behind without something saying so.

**What is verified here:** the grep above, the three header lines, and
that `render.yml` declares lanes this file does not. **What is not:**
whether `--lane mc` or `--lane gui` fail cleanly or confusingly, which
wants running the helper and was not done.

## The reading the filing lane did not take (2026-09-11)

Run on this tree before the fix:

```
$ bash local-scripts/render-hosted.sh --lane mc --no-install
render-hosted: --lane must be one of kernel, freecad, uv, wild, all (got 'mc')
$ echo $?
1
```

`--lane gui` is identical. So both fail CLEANLY: the validation `case`
sits above the `gh` checks and everything else, and the refusal names
the set it accepts. The defect is narrower than a half-run — and it is
also wider than the two flags, because the failure that is NOT clean is
the default:

**`--lane all` takes four of six lanes and says nothing.** `lanes_of all`
echoed `kernel freecad uv wild`, so the `mc` and `gui` artifacts were
never requested and no "no artifact" note was printed for them; the
closing `git status` named four directories, and the install reported
success. A caller who asked for everything got four sixths of it with a
green exit.

And the refusal is reachable from CI's own output: a drifting lane's
neutral check is posted by `.github/actions/rebaseline-lane`, whose
summary names `local-scripts/render-hosted.sh --run <id> --lane ${LANE}`
as the fix (`action.yml:122`). For `mc` and for `gui` that is a command
that dies.
