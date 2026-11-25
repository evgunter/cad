---
id: main-red-halfcap-eps6-pole-margin-moved-one-ulp
kind: issue
title: main red: step-import tier_gate's halfcap_eps6.step pole-membership margin prints one ulp off its pin
status: open
opened: 2026-11-25
priority: P1
cost: E
---


## The finding

(Filed by the WALKS orchestrator, 2026-11-25, from the WALKS finish-completes lane, which reproduced it on a clean origin/main 6003877a worktree.) `step-import` `tier_gate::every_corpus_import_passes_the_shared_gate` panics at `crates/step-import/tests/tier_gate.rs:1211`: `halfcap_eps6.step` at band 1e-9 refuses with the pinned pole-membership message, but its margin prints `9.99999999978924e-9` against the pinned `9.999999999789241e-9` — one ulp. It is the import's first pole reading, before any pcurve row, so it is not a completion-at-the-finish effect.

## What a fix is

Find the main change that moved the margin by an ulp (bisect main since the pin last held). If the new value is the right reading, re-pin and say what moved (`docs/prompts/implementer-discipline.md` §3: a golden is not a target); if not, fix the reading. Consider pinning the margin by its band position rather than its printed digits.
