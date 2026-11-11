---
id: main-red-a-needle-cavity-12eps-channel-escalates-at-bool-contact-vertex
kind: issue
title: Main red (slow set): the 12ε needle channel escalates at bool_contact_vertex where it must refuse on the sector question
status: closed
opened: 2026-11-01
closed: 2026-11-11
priority: P1
cost: M
refs: [a-needle-cavity-refuses-at-the-joins-section-loop-before-any-read]
pr: 6812
branch: flush/needle-12eps
---


Found by ONEREAD's PR 6247 dual review (R1, NOTE 3; slow set, default ε 1e-9). Fails identically at that PR's merge base `4d92c824` and its head, so it is main's red. Filed by the ONEREAD orchestrator on FLUSH, whose closed `a-needle-cavity-refuses-at-the-joins-section-loop-before-any-read` (PR 5694) made this family build.

`topo::all shell_role_reader::a_needle_cavity_builds_at_every_definite_width_and_holds_it_as_a_void` panics at `crates/topo/tests/shell_role_reader.rs` (~645):

> brick ∖ slab ∪ fill leaving a 12ε channel, tilt 0, Turned, at ε 1e-9: refuses on the sector question, got Err(Escalated { decision: Containment { decision: Some(Loop(Boundary)), escalation: Margin }, diag: Indeterminate { margin: 6.000000011551611e-9, band { zero: 1e-9, escalate: 1e-8 }, predicate: Some("bool_contact_vertex") } })

Reproduce: `cargo nextest run -p topo --profile ci --ignore-default-filter -E 'test(=a_needle_cavity_builds_at_every_definite_width_and_holds_it_as_a_void)'`. Related: `work/germ/a-sector-direction-read-off-a-needles-short-run-carries-its-rounding-over-1-w.md`. Owed: bisect the commit that turned it red, then the fix.

## Still red, same reading (ONECOPY, `onecopy/main-reds-triage`, 2026-11-09)

Reported again by a ONECOPY lane at `f8419853`, with the message above unchanged (the test fn is `crates/topo/tests/shell_role_reader.rs:708`; the assertion that fires is at :648). PRs 6622, 6629 and 6576, which class boolean refusals, all merged on 2026-11-09, after this row recorded the same escalation at `4d92c824` (2026-11-01). So none of them changed the expected refusal: the red is this row's, not a stale expectation. `work/flush/a-needle-cavity-test-red-on-main.md` was a same-day copy and is closed into this row.

## Same reading on SEAL PR 6693 (2026-11-10)

At `seal/corner-sectors` the test still fails at `shell_role_reader.rs:648` with the same `Containment` at `bool_contact_vertex`, margin `6.000000011551611e-9`. That PR deletes `bool_sector_coplanar`, so the expected arm now reads `bool_sector_within` or `Coincide::CornerBound` (a flat sector's bound tie). It was not set to the `Containment` this head yields, because that is this row's red.

## Closed

Closed by PR 6812 (2026-11-11). The kernel was wrong; the expectation's arm was a guess SEAL could not observe.

- **Bisect.** The first bad first-parent commit is `0709e658`, the merge of PR 6225. That PR moved "does this face hold this piece of an edge lying in its plane" from the carry into the reduction (`reduce::settle_in_plane`), ahead of the corner question. The good end was `712dadf2`, PR 6053's merge, where the five runs refused `bool_sector_coplanar`.
- **The margin is a probe's, not a pair's.** The settle read the midpoint of a 12ε piece of the slab's edge, lying along the fill's end face's boundary. `6.0e-9` is `w/2`, the distance from that midpoint to the piece's own end, the face's corner `(1, 0.8, 0.5 + w)`. No cell of either operand is 6ε from another, so the refusal judged thinness by something other than the pairs (`[coincidence.booleans]`).
- **Why only turned 12ε.** The pieces are read in-plane only because the glue door reads the two end faces' carrier pair in band (`bool_plane_parallel`, `1.19e-9` and `1.06e-9` against zero `1e-9`) and leaves it unglued. At 10.5 and 15ε turned, and in every aligned and moved pose, the pair glues. The carriers are fitted across `w`: GERM's rounding over `1/w`. GERM's P2 row has the measurement.
- **The fix.** A piece whose midpoint lies in band of its own ends now reads at an end tied to one corner of the face (`reduce::held_at_corners`). The germ is read against the corner's two boundary lines, levered at the piece. With that, the five runs reach their corner and refuse `Coincidence(CornerCrossing)` at `bool_faces_parallel`, a Zero rejected. That is SEAL's reading of an unglued plane pair at a shared corner, and the test now pins it (`Want::Corner`).
- **Residue.** The cases the corner read leaves to the midpoint are filed on PARTING: `work/parting/the-in-plane-settle-reads-a-short-piece-by-its-midpoint-where-no-end-reads-at-a-corner.md`.
