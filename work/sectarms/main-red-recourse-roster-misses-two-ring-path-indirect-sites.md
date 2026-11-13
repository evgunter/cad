---
id: main-red-recourse-roster-misses-two-ring-path-indirect-sites
kind: issue
title: Main red: recourse_roster finds two unlisted indirect decide sites, ring_path.rs spline_crossings and ring_path/spline_wall.rs paths (since PR 6891)
status: open
opened: 2026-11-13
priority: P1
cost: E
---

Found by ONEHOME's `onehome/star-cut` lane (PR 6871) running the topo suite on its merge with main.

## What

`crates/topo/tests/recourse_roster.rs`, `every_decided_name_has_its_door_and_every_site_is_read`, is red on main at `63226c502447e07099c3986bca2ed786d56b1742`, reproduced in a clean worktree with its own target directory:

```
an indirect site not listed: ring_path.rs: spline_crossings:
an indirect site not listed: ring_path/spline_wall.rs: paths:
```

`git log` on `crates/topo/src/ring_path.rs` and `crates/topo/src/ring_path/spline_wall.rs` puts both sites in SECTARMS' ring lane (`a1db4fc2`, `fcedde73`, `efd340e9`), which landed through PR 6891 (`sectarms/spline-ring-winding`). The row is in the slow set, so the PR gate skipped it; the nightly runs it.

## Owed

Register the two sites in the roster's `INDIRECT` list with what each forwards, or give the names they decide a carrier the census reads. Then run the row.

On main at `5e5fec6d0a` a third site joins them: `an indirect site not listed: ring_path/spline_wall.rs: reach:`.
