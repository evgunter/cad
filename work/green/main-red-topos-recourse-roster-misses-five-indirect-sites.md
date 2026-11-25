---
id: main-red-topos-recourse-roster-misses-five-indirect-sites
kind: issue
title: Main red: topo's recourse roster reports five indirect sites not listed (boolean/join, section_cert, census, sheet_apex_rows)
status: open
opened: 2026-11-24
priority: P1
cost: E
---


Filed by the SCALAR orchestrator, 2026-11-24, met while running the slow censuses for PR 7651 on its head `b803670e2c` (main merged in that day). `cargo nextest run -p topo -E 'test(recourse_roster)' --ignore-default-filter`: `topo::all recourse_roster::every_decided_name_has_its_door_and_every_site_is_read` fails at `crates/topo/tests/recourse_roster.rs:1024` with

- `an indirect site not listed: boolean/join.rs: conic_section_frame_at: sa`
- `an indirect site not listed: boolean/section_cert.rs: cone_torus: f`
- `an indirect site not listed: census.rs: meeting_point: a.p0 + a.dir.get() * s`
- `an indirect site not listed: census.rs: meeting_point: e.p0 + e.dir.get() * ((f.origin - e.p0).dot(f.normal) / along)`
- `an indirect site not listed: pcurves/sheet_apex_rows.rs: joint: DescribedChart::of(surface).unwrap()`

PR 7651's diff touches `topo` only in `replace_face.rs` and `offset_derive.rs`, so none of the five is its; not re-run on `origin/main` alone. The roster is a slow-set row no per-PR gate runs, the same shape as `work/ciw/main-red-the-geom-brep-roster-trails-every-merge-that-adds-a-name-site`. Asks: list or route each site, and say which merges added them.
