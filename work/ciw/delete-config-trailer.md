---
id: delete-config-trailer
kind: unit
title: Delete the CI-Config commit-trailer configuration path
status: review
opened: 2026-09-04
refs: [reinstate-full-configuration-runs, klint-row-still-sampled, fillet-specs-require-a-narrowing-ci-config, 1850, 1855, 1823]
branch: ciw/delete-config-trailer
---

## The ask, and who authorised it

**Ev, in chat, 2026-09-04:** *"i see in 1855 it's still talking about the ci
config trailer; that code should be deleted since it's no longer live"*.

## Why it is dead rather than merely unused

Three merges landed on 2026-09-04 and the third of them killed the trailer
without anyone saying so:

* PR 1823 un-sampled the lane and the eps row, and made the trailer
  **additive-only** — `parse_config`'s `additive_only` arm refused any value
  that would gate less than no trailer at all.
* PR 1850 un-sampled the k-lint row, which put `klint` into
  `WHOLE_BY_DEFAULT` alongside `lane` and `eps`.

With all three dimensions in that table, the additive-only guard covered every
dimension the trailer could name. So a `CI-Config:` trailer had exactly two
possible effects: name the whole-dimension value — `lane=both`, `eps=all`,
`klint=all` — which is already the default and changes nothing; or name
anything else and red the classify step. **No input made it useful.** The
file's own comment said as much: the legal values are ones *"every one of
which changes nothing. It may not narrow."*

A mechanism whose entire legal vocabulary is a no-op is not a feature with a
narrow use; it is dead code that still has to be read, taught and swept every
time the configuration story changes — which it did three times in three days.

## What goes

* `scripts/ci-filter.py`: `config_from_message`, the `CONFIG_TRAILER` regex,
  `parse_config`'s `additive_only` parameter and the `WHOLE_BY_DEFAULT`
  narrowing guard inside it, the `WHOLE_BY_DEFAULT` table itself (nothing else
  read it), the `--config-from-message` flag and its call site, and every
  selftest case that exercised the trailer — together with the `--selftest`
  prose that claimed the coverage.
* `.github/workflows/ci.yml`: the `HEAD_COMMIT` env, the
  `git log -1 --format=%B > commit-message.txt` line and the
  `--config-from-message` argument, plus every comment that taught the
  spelling or matched `commit-trailer` in `CONFIG_SOURCE`.
* `CONFIG_SOURCE`'s vocabulary loses `commit-trailer`. It is now
  `unsampled` (the whole dimension runs) or `requested` (a dispatch input),
  and both prose sites say those are the only two words there are.
* The `CI-Config:` teaching in `memories/agent-lane-operations.md` (Ev's
  message above is the sign-off `CLAUDE.md` requires),
  `docs/prompts/implementer-discipline.md`, `docs/K-REPORT.md`, five live
  specs, `local-scripts/ci-local.sh` and this program's merge-queue runbook.

## What stays, deliberately

`parse_config` and the `workflow_dispatch` path. That is the live way to
narrow a run and Ev is not removing it; a narrowing is an act someone takes
deliberately at the moment they take it, which is what a dispatch is and what
a copied trailer never was. **No row's default changes**: `LANE=both`,
`EPS=all`, `KLINT_ROW=all` before and after.

## Fence

`scripts/ci-filter.py` is **S-TCOST's**. This is an announced cross-fence
change; the PR body names every owner `work.py territory` reports, says which
selftest cases went and why, and invites S-TCOST to own the result.
