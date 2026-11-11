---
id: a-src-change-that-moves-a-decide-site-skips-its-crates-recourse-roster
kind: issue
title: A source change that moves or adds a decide site skips its crate's recourse_roster census on the PR gate, so it reds main
status: open
opened: 2026-11-11
priority: P4
cost: E
refs: [6809]
---

Filed by PCERT's orchestrator (two main reds in one day).

## What

Each crate's `recourse_roster::every_decided_name_is_read_and_every_site_is_listed` (geom-core, geom-brep, editor-core; topo's `every_decided_name_has_its_door_and_every_site_is_read`) is in the slow set (`.config/nextest.toml`, the `ci` filter). The per-PR gate runs a slow test only when the PR adds or edits it, or names it in an added line. The census reads the crate's `src/`, though, so a PR that moves, adds or renames a decide site, and never touches the roster file, passes its gate and reds main at the next nightly.

On 2026-11-10/11 this happened three times:
- geom-brep, after PR 6755's forwarder rule: PCERT's PR 6531 (`spline_incidence_as` forwarding to a new helper) and sectarms' PR 6518 (a pose test's `decide(x, …)`). Fixed by PR 6809.
- geom-core: onehome's 6717 (the rigidity loop moved from `check_rigid` into `admitted_for_a_suite`). Fixed by PCERT's geom-core roster PR.

## Fix

Have `scripts/ci-filter.py` select a crate's roster row whenever the diff touches that crate's `src/`. Each roster takes 2–8 s, so this costs one test per touched crate. The alternative is to move the rosters into the fast set if they can be made to fit the bar.
