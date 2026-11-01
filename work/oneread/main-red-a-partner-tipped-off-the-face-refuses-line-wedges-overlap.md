---
id: main-red-a-partner-tipped-off-the-face-refuses-line-wedges-overlap
kind: issue
title: Main red (slow set): a partner tipped 0.5ε off the face refuses LineWedges Overlap where it must build or refuse typed
status: closed
branch: oneread/partner-tipped-off-the-face
pr: 6272
opened: 2026-11-01
priority: P1
cost: M
refs: [6247, a-lying-pyramid-half-an-epsilon-off-the-top-refuses-line-wedges-overlap]
closed: 2026-11-01
---


Found by PR 6247's dual review (R1, NOTE 3; slow set, default ε 1e-9). Fails identically at PR 6247's merge base `4d92c824` and at its head `80957e80`, so it is main's red, not that PR's. Main's nightly tail did not show it (only the 1e-6 and 1e-12 sections were visible).

`topo::all a_vertex_read_by_two_sector_passes::a_partner_tipped_off_the_face_builds_sound_or_refuses_typed_at_every_pose` panics at `crates/topo/tests/a_vertex_read_by_two_sector_passes.rs` (~1699):

> around the lying pyramid, the pyramid lying 0.5ε off the top, at rest, x − y: builds or refuses typed, got LineWedges { fault: Overlap { edges: [EdgeKey(7v1), EdgeKey(11v1)] }, from: "(1.75, 1.4330127018922192, 1.0)", to: "(1.5, 1.0, 1.0)" }

Reproduce: `cargo nextest run -p topo --profile ci --ignore-default-filter -E 'test(=a_partner_tipped_off_the_face_builds_sound_or_refuses_typed_at_every_pose)'`.

Filed on ONEREAD because the test is the vertex-pair pass's (the reading PR 6247 rewrote) though the fault is raised at the line wedges; re-home to CLEAVE (lines) if the cause is there. Owed: bisect the commit that turned it red, then the fix.

## 2026-11-01 (ONEREAD, `oneread/partner-tipped-off-the-face`): already green on main, a duplicate of SPLITDOOR's row

Measured on main `bc859043` (after PR 6247's merge), default ε: the row passes (25 s), and so does every test in `a_vertex_read_by_two_sector_passes.rs` at 1e-6, 1e-9 and 1e-12, and in `a_vertex_read_again_classes_every_edge.rs` at 1e-9. It is red at PR 6247's head `80957e80` and green on the main PR 6247 merged into (`80867e9b`), so the red this row records was main's at 6247's merge base, and main fixed it before 6247 landed.

- **Turned red**: the merge of PR 6212 (`367ff259`, SPLITDOOR, "a within-zero tilt decides as upright"): red there, green on its first parent `082c42d2`.
- **Turned green**: `git bisect --first-parent 4d92c824..80867e9b` names the merge of PR 6256 (`dcd012f6`, `splitdoor/lying-pyramid-line-wedges`), which closed `work/splitdoor/a-lying-pyramid-half-an-epsilon-off-the-top-refuses-line-wedges-overlap.md`, this same red filed a few hours earlier.
- **Cause** (SPLITDOOR's row, confirmed here): the refusal is honest. `boolean::ops` `coincident_wedges_disjoint` raises `LineWedges { Overlap }` where the result holds two coincident edges on one line whose four sheets it pairs otherwise than the material does, the unbuilt re-pairing of `work/topo/revert-re-pairs-the-sheets-on-a-line-where-coincident-edges-meet.md`. The exact on-top scene is pinned to refuse it (`REFUSING`: "around the lying pyramid, x − y"). Once PR 6212 read a 0.5ε tilt as upright, the tipped pose refused as the on-top one does, as the test's own contract asks ("within the zero every scene builds as on the top"); `builds_or_refuses` looked its pins up by the decorated label ("…, the pyramid lying 0.5ε off the top"), which never matches `REFUSING`. PR 6256 passes the on-top scene's label as the pins for poses within the zero, and enforces `not_pinned` there too.

Nothing remains to fix on ONEREAD's ground: the vertex-pair reading is not involved, and the kernel side is the open topo row above.

## Closed (2026-11-01, duplicate)

A duplicate of SPLITDOOR's closed `a-lying-pyramid-half-an-epsilon-off-the-top-refuses-line-wedges-overlap`, fixed on main by PR 6256 before this row was filed; the measurements above confirm it green on main. The underlying re-pairing of coincident edges is `work/topo/revert-re-pairs-the-sheets-on-a-line-where-coincident-edges-meet.md`. (ONEREAD orchestrator)
