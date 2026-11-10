---
id: main-red-a-needle-cavity-12eps-channel-escalates-at-bool-contact-vertex
kind: issue
title: Main red (slow set): the 12ε needle channel escalates at bool_contact_vertex where it must refuse on the sector question
status: open
opened: 2026-11-01
priority: P1
cost: M
refs: [a-needle-cavity-refuses-at-the-joins-section-loop-before-any-read]
---


Found by ONEREAD's PR 6247 dual review (R1, NOTE 3; slow set, default ε 1e-9). Fails identically at that PR's merge base `4d92c824` and its head, so it is main's red. Filed by the ONEREAD orchestrator on FLUSH, whose closed `a-needle-cavity-refuses-at-the-joins-section-loop-before-any-read` (PR 5694) made this family build.

`topo::all shell_role_reader::a_needle_cavity_builds_at_every_definite_width_and_holds_it_as_a_void` panics at `crates/topo/tests/shell_role_reader.rs` (~645):

> brick ∖ slab ∪ fill leaving a 12ε channel, tilt 0, Turned, at ε 1e-9: refuses on the sector question, got Err(Escalated { decision: Containment { decision: Some(Loop(Boundary)), escalation: Margin }, diag: Indeterminate { margin: 6.000000011551611e-9, band { zero: 1e-9, escalate: 1e-8 }, predicate: Some("bool_contact_vertex") } })

Reproduce: `cargo nextest run -p topo --profile ci --ignore-default-filter -E 'test(=a_needle_cavity_builds_at_every_definite_width_and_holds_it_as_a_void)'`. Related: `work/germ/a-sector-direction-read-off-a-needles-short-run-carries-its-rounding-over-1-w.md`. Owed: bisect the commit that turned it red, then the fix.

## Still red, same reading (ONECOPY, `onecopy/main-reds-triage`, 2026-11-09)

Reported again by a ONECOPY lane at `f8419853`, with the message above unchanged (the test fn is `crates/topo/tests/shell_role_reader.rs:708`; the assertion that fires is at :648). PRs 6622, 6629 and 6576, which class boolean refusals, all merged on 2026-11-09, after this row recorded the same escalation at `4d92c824` (2026-11-01). So none of them changed the expected refusal: the red is this row's, not a stale expectation. `work/flush/a-needle-cavity-test-red-on-main.md` was a same-day copy and is closed into this row.
