---
id: main-red-a-partner-tipped-off-the-face-refuses-line-wedges-overlap
kind: issue
title: Main red (slow set): a partner tipped 0.5ε off the face refuses LineWedges Overlap where it must build or refuse typed
status: open
opened: 2026-11-01
priority: P1
cost: M
refs: [6247]
---


Found by PR 6247's dual review (R1, NOTE 3; slow set, default ε 1e-9). Fails identically at PR 6247's merge base `4d92c824` and at its head `80957e80`, so it is main's red, not that PR's. Main's nightly tail did not show it (only the 1e-6 and 1e-12 sections were visible).

`topo::all a_vertex_read_by_two_sector_passes::a_partner_tipped_off_the_face_builds_sound_or_refuses_typed_at_every_pose` panics at `crates/topo/tests/a_vertex_read_by_two_sector_passes.rs` (~1699):

> around the lying pyramid, the pyramid lying 0.5ε off the top, at rest, x − y: builds or refuses typed, got LineWedges { fault: Overlap { edges: [EdgeKey(7v1), EdgeKey(11v1)] }, from: "(1.75, 1.4330127018922192, 1.0)", to: "(1.5, 1.0, 1.0)" }

Reproduce: `cargo nextest run -p topo --profile ci --ignore-default-filter -E 'test(=a_partner_tipped_off_the_face_builds_sound_or_refuses_typed_at_every_pose)'`.

Filed on ONEREAD because the test is the vertex-pair pass's (the reading PR 6247 rewrote) though the fault is raised at the line wedges; re-home to CLEAVE (lines) if the cause is there. Owed: bisect the commit that turned it red, then the fix.
