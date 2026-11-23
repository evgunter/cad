---
id: main-red-geom-breps-recourse-roster-misses-three-ssi-enclose-cone-sites
kind: issue
title: Main red: geom-brep's recourse roster does not list three decide sites in ssi/enclose.rs's cone arms
status: open
opened: 2026-11-22
priority: P0
cost: E
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
