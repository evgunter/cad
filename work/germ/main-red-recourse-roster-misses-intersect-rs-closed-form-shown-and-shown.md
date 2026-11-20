---
id: main-red-recourse-roster-misses-intersect-rs-closed-form-shown-and-shown
kind: issue
title: Main red: geom-brep's recourse_roster finds intersect.rs's closed_form_shown and shown as indirect sites it does not list, since PR 7171
status: open
opened: 2026-11-20
priority: P1
cost: E
refs: [7171]
---


Found by an EXCH reviewer at main `46e89240` and bisected by EXCH's triage lane (2026-11-20). The roster row is in the slow set, so the PR gate did not run it and the nightly is the first to see it.

## What

`geom-brep::all recourse_roster::every_decided_name_is_read_and_every_site_is_listed` panics at `crates/geom-brep/tests/recourse_roster.rs:726`:

```
an indirect site not listed: intersect.rs: closed_form_shown: name
an indirect site not listed: intersect.rs: shown: name
```

The two sites are `fn closed_form_shown` (`crates/geom-brep/src/intersect.rs:1027`) and `fn shown` (`crates/geom-brep/src/intersect.rs:2894`). The roster's `intersect.rs` lines (`crates/geom-brep/tests/recourse_roster.rs:388-394`) name neither. It is still red on main `bd8b2c9bb5`.

## Cause

PR 7171 (`germ/section-class-lane-choices`, merge `71bdbfd537`). `closed_form_shown` arrived in `b2c4346902` and `shown` in the fix pass `2cc97f1ce7`. The row passes on the merge's first parent and fails on the merge.

## Owed

List both sites in the roster with their disposition, or route the name through a site the roster already admits. The lane that caused the red owes the fix.

## Reproduce

    cargo nextest run -p geom-brep --ignore-default-filter -E 'test(recourse_roster::every_decided_name_is_read)'

Signed: (EXCH orchestrator)
