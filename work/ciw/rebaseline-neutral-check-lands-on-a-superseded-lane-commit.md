---
id: rebaseline-neutral-check-lands-on-a-superseded-lane-commit
kind: issue
title: a lane's neutral re-baseline check lands on its own commit, which a later lane's commit supersedes as the PR head
status: open
opened: 2026-10-02
---

Each lane of `render.yml` commits its own cells and posts its
`render re-baselined (<lane>)` neutral check against the commit it just
pushed (`.github/actions/rebaseline-lane`, its "THE NEUTRAL CHECK" comment).
When two lanes drift, the first lane's commit is no longer the branch tip
once the second lands, so a PR shows only the LAST lane's neutral check;
the earlier ones sit on an intermediate sha the PR page does not list.

Measured on PR 3791, render run 36961370249: `render re-baselined
(freecad)` is on `ffb52205c` and `render re-baselined (gui)` on
`3cdb9f64d`, the head; the head's check-runs list carries only the gui
one. The ask the neutral check exists to make ("look at these images")
is therefore lost for every lane but one.

A fix would post every lane's check against the final tip (e.g. a
closing job that reads which lanes committed, as `render.yml`'s
`ci-on-new-head` already reads the tip), or have each lane re-post on
the tip it last saw.
