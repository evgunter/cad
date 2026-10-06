---
id: a-hole-filleted-at-both-rims-in-one-fillet-panics-in-blend-surgery
kind: issue
title: A through-hole filleted at both rims in one Fillet panics in blend surgery: a birth row names an edge the source body does not carry
status: open
opened: 2026-10-05
priority: P1
cost: M
---


Found by RECIPE's third review of PR 3886 (2026-10-05), building a document to reach `RoleSeg::BandCross`/`BandSlit` pairs that differ only in `band` (two rims at the two ends of one meridian segment, `names/role.rs` ~1328–1358).

A through-hole whose two rims are both rounded by one `Fillet` panics during evaluation at `crates/sweep/src/blend/surgery.rs:924`: "a birth row names EdgeKey(20v1) as its source, which the source body does not carry (kernel bug)". A panic on a valid authored document is a kernel defect; the evaluation should build or refuse typed. The reviewer's probe is in `~/.local/share/cad-work/rev3886c/rev3886c.rs` (lane-private; the construction: a box, a cylinder subtracted through it, one fillet over both of the hole's rim edges).
