---
id: cache-rendered-cells-on-input-hash
kind: issue
title: Cache rendered cells on their input hash, to close the window where main's committed renders are stale
status: open
opened: 2026-08-18
github: 603
refs: [598]
---

## From GitHub issue 603

Opened 2026-08-18; 0 comments.

## The window

#598 made the render lanes re-baseline themselves, with the rule **PRs report, `main` commits** (a bot commit on a PR branch becomes the PR head, and a `GITHUB_TOKEN` push triggers no run, so every other check strands on the parent commit — the recursion guard and that blank slate are the same fact).

The accepted cost of that rule is a **staleness window**: a PR with a render-affecting change merges with cells that do not match its code, and `main` carries them until its own run finishes rendering and commits. That is a few minutes today, and it is the one part of the design that is uncomfortable in a repo that built a gate specifically against committed renders rotting.

The window exists because `main`'s run **re-renders from scratch** what the PR run already rendered minutes earlier. If those pixels were already available, `main` could commit almost immediately and the window would shrink to a cache fetch.

## Why not "just download the PR run's artifact"

That was the first idea and it is not safe. The PR run renders `refs/pull/N/merge` — the merge preview — not `main`'s post-merge tip. Those are the same tree only if the base did not move between the PR's last run and the merge, and on this repo `main` moves often. When they diverge, reusing the artifact commits frames drawn from the wrong scenes, and they look entirely plausible. `local-scripts/render-hosted.sh` has a *hard refusal* for exactly this failure mode; a cache must not reintroduce it through the back door.

## The shape that works: key on the inputs, not on the run

Cache the rendered cells under a key derived from everything a render reads. The repo already computes almost exactly that key for the tour output, in both `ci.yml` and `render.yml`:

```
tour-out-v1-${{ hashFiles('crates/**', 'demos/tour/**', 'Cargo.toml', 'Cargo.lock', 'rust-toolchain.toml') }}
```

A hit means the inputs are identical, so the bytes are identical — the repo measured a repeat hosted render of one commit as byte-identical across all 55 cells of both PNG lanes. Keying on inputs rather than on "the previous run" makes the merge-preview-vs-tip question disappear: either the inputs match and the bytes are reusable, or they do not and it renders.

It also helps in cases the artifact idea does not — repeat pushes to a PR, re-runs, and any two branches that happen to share render inputs.

## The sharp edge: mesa is an input and is not in that key

The runner image's mesa version is part of what determines the pixels — the lanes re-baseline roughly monthly when it bumps, and that is the gate working. But mesa is not in the hash above. A naive cache would therefore **hit across a mesa bump and commit stale pixels**, which is precisely the rot this whole lane exists to prevent.

So the key must include the runner image version (`ImageVersion` from the runner environment, or an equivalent pin), and busting it must be part of the monthly re-baseline rather than something anyone has to remember.

## Sketch

- Add a cache step to each lane keyed on `render-cells-v1-<lane>-<source hash>-<runner image>`.
- On a hit, skip the render and use the cached cells; the drift comparison and the re-baseline commit are unchanged downstream.
- On a miss, render as today and populate the cache.
- The `uv` lane is renderer-free and cheap; it may not be worth including.

## Out of scope / non-goals

- **Not a runner-minutes optimization.** The lanes are already off the critical path — `ci.yml` calls them "free in wall clock" at ~150 s against a ~460 s critical path — and the double render only happens on the minority of merges that actually change renders. If this only saved minutes it would not be worth the correctness surface. The point is the staleness window.
- Not a route back to PR-branch commits; that trade was settled in #598.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

https://claude.ai/code/session_013rfD4xGEJuD41qXBW8JR2r

## Home

`work/issues/`: `.github/workflows/render.yml` and `ci.yml` are S-QA's territory and S-QA is closed; no open program's charter names the render lanes.
