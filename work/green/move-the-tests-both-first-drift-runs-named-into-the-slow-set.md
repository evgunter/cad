---
id: move-the-tests-both-first-drift-runs-named-into-the-slow-set
kind: unit
title: Move the ~95 tests both of the first drift runs named over 0.25 s alone into the slow set
status: open
opened: 2026-11-23
priority: P1
cost: E
refs: [7448, a-test-timed-alone-still-reads-the-runners-speed-class]
---


Filed by the GREEN `green/fast-set-bar` lane from PR 7448's review (finding 6), 2026-11-23.

## What to do

Add to `.config/nextest.toml`'s `ci` filter the fast-set tests that both of the first two drift dispatches named at 0.25 s or more alone:
- run 45084526868, which named 134;
- run 45093266519, which named 242.

The lane's count of the tests both named is 95, and the reviewer's is 97. Re-derive the list from those two runs' logs (the `(binary_id(..) & test(=..)) |` lines), or from a scheduled run once PR 7448 is merged. Take the intersection, not the union: one run's list swings with its runner's speed class (`a-test-timed-alone-still-reads-the-runners-speed-class`).

## Why it is P1

Until these tests move, a PR that edits one of their files goes red on the 0.25 s bar. The reviewer replayed PR 7448's token code over the last 40 merges on main: 3 of the 40 (PRs 7362, 7369 and 7516) would have gone red on one of these tests.
