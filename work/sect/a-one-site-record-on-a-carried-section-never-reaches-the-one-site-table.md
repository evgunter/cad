---
id: a-one-site-record-on-a-carried-section-never-reaches-the-one-site-table
kind: issue
title: A one-site record whose pair rides a carried section is skipped before closes_at_its_site is read
status: open
opened: 2026-11-20
priority: P3
cost: M
---

Found by PR 7177's sweep (`sect/closed-edge-on-torus`), 2026-11-20.

## What

`crates/topo/src/boolean/join.rs` `one_site_segments` reads a one-site
record only where `one_site` hands it a frame:
`let Some((g0, Some(frame))) = one_site(..) else { continue }`. A record
whose pair rides a carried section (`germ_rides_carried`) gets
`(g0, None)`, so it is skipped before `closes_at_its_site` is read, and
`bool_connect` refuses it `SingleSiteSectionLoop` whatever the table
would say. The frame is read only for the desync check that the
record's two germs turn opposite ways (`rotational_sense` about
`frame.axis`).

## Reach

Not reached by any pose PR 7177 measured: every pair in its rows is
closed-form. PR 7177's review found none either (a puck with two carried
candidates still reached `closes_at_its_site`). The RimSide unit's
predicted pose, a tilted one-seam tube through a ball, does reach it
(`sect/rimside-retires`).
