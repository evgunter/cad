---
id: main-red-geom-breps-recourse-roster-misses-three-ssi-enclose-cone-sites
kind: issue
title: Main red: geom-brep's recourse roster does not list three decide sites in ssi/enclose.rs's cone arms
status: closed
opened: 2026-11-22
priority: P0
cost: E
closed: 2026-11-23
pr: 7670
---


Reported on 2026-11-22 by the author of BOXFINISH's PR 7394 and
confirmed by one of its reviewers; filed by the BOXFINISH
orchestrator from their reports, not re-run by the filer.

`geom-brep` `recourse_roster::every_decided_name_is_read_and_every_site_is_listed`
(slow set) is red on main: three decide sites in
`crates/geom-brep/src/ssi/enclose.rs`'s cone arms are not in the
roster. They arrived with SECTCONE merges after `0a62bae4`
(`ea59b6fd…024a8fe7` by the author's reading); the row is green at
`0a62bae4`. A PR sees it only when its diff edits the roster, which
is how PR 7394 met it (through its merge ref with main). No other row
under `work/` names it.

**Owed:** SECTCONE lists the three sites, each with its recourse.

## 2026-11-23 — green on main (PR 7670 closes the row)

Fixed on main, not by PR 7670. The three sites (`ssi/enclose.rs`: `implicit_gradient_enclosure: parts`, `over: ConeParts::over(mid, apex, axis, angle)` and `over: parts`) were listed `SAME_NAMED` in the roster's `INDIRECT` table by `c0db369c8b`, "the roster declares the sites 7350/7354/7358 left unlisted", which landed with PR 7408 (`sectcone/certified-foot`). Then CIW's PR 7496 (`ciw/census-hand-kept-lists`, `a3ad83ef26`) dropped the roster's `INDIRECT` list and its stale-entry reds, so the row no longer lists sites. It is now `recourse_roster::every_decided_name_is_read`. It passed on probe run 45310731023 (`test (kernel)`, the slow-set step), at `origin/main` `82273ca89b` plus a comment line added to the roster file so the row would run. PR 7670 carries no change for it.
