---
id: main-red-since-pr-7052-the-recourse-roster-and-a-close-rings-pin
kind: issue
title: Main red since PR 7052: geom-brep's recourse_roster lists two certify.rs sites that went and misses two that came, and close_rings_carry_counted's 1e-6 pin names a chain of 206 boxes where the periodic fit has 208
status: review
opened: 2026-11-17
priority: P1
cost: E
branch: sectarms/main-red-1e-6
pr: 7057
---

Found by the sliver lane (`sectarms/sliver-loop-tube`, 2026-11-17) and replayed by the orchestrator on main at `f50b58e2f3`, in a clean worktree with its own target directory.

## What

- **`crates/geom-brep/tests/recourse_roster.rs`, `every_decided_name_is_read_and_every_site_is_listed`, at the default ε:** red on main, green at PR 7052's first parent `fa4e9ba921`. The lane read the failure as: `certify.rs`'s indirect sites `of` and `turn` are not listed, and `mint_checks` and `run_checks` are listed but no longer exist. PR 7052's fix pass kept check 2's turn decided on the stored loop (`ReadSpec::of`, `EdgeCurve::turn`) and moved `run_checks` / `mint_checks`. The row is in the slow set, so the PR gate skipped it.
- **`pair_section::wiring::close_rings_carry_counted` at ε = 1e-6** (`crates/geom-brep/src/pair_section.rs:4313`): the pin names `δ 3e-4, counted: Refused(March(TubeStraddles { …, boxes: 206 }))`, and the periodic fit's chain has 208. The stop is the same and the box count moved with the fit.

## Owed

List the roster's sites as they are now, with what each forwards, and re-pin `close_rings_carry_counted`'s 1e-6 stop to the periodic chain. Run both rows, at the default ε and at 1e-6. `pair_section::one_loop_keeps_its_order_near_the_node` at 1e-6 is also red since PR 7052, but it is the sliver pose and belongs to `a-sliver-loop-that-carried-as-a-clamped-fit-refuses-its-tube-as-a-periodic-one-at-a-coarse-band`.
