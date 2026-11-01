---
id: main-red-blend-refusal-rewording-breaks-the-concision-cap-and-the-die-body-row
kind: issue
title: main red since #6200: the reworded blend refusals render 88–95 words against refusal_concision_chains' 75-word cap, and m5_pr12_die_body no longer finds 'not built'
status: open
opened: 2026-11-01
priority: P0
cost: E
---


Filed by the CLEAR orchestrator. CLEAR's REFINES lane found it: PR 6206's
CI tests the PR merged onto main, and it fails the same four tests, none
of which the PR touches.

## What

`chrome/blend-recourse-words` (#6200, merge `c43600e23d`) reworded the
blend refusals. Its push CI was cancelled, so main went red unseen:

- `editor-core` `refusal_concision_chains`, three tests: blend refusals
  now render 88–95 words against the 75-word cap;
- `sweep` `m5_pr12_die_body::a_subset_of_the_edges_refuses_at_the_assembly_front_door`:
  it expects the text "not built", which the blend refusal no longer
  contains.

Every PR now inherits this red, because CI tests the merge with main.

## What closing it takes

CHROME decides which side moves. Either shorten the reworded blend
refusals back under the cap and keep the claim the die-body row reads,
or re-bless the cap and the row with the reason. Either way, the four
tests pass on main.
