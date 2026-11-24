---
id: the-two-parameter-plates-volume-refuses-on-a-junction-margin-that-is-not-a-number
kind: issue
title: The two-parameter plate's volume refuses at props_chart_junction_closed on a margin that is not a number, in 21 of 30 replays, since PR 7370
status: open
opened: 2026-11-24
priority: P1
cost: M
---


Found by M12 (the probe lane, 2026-11-24; `probe/M12-REPORT.md` on
`boxfinish/probe-switch-off` at `0a6d17f556`), on main with PRs 7370
and 7480, the switch's box arm off.

`editor-core` `m10_3`'s
`the_two_parameter_drive_certifies_after_bisection_and_accounts_for_all_of_it`
is still red with the box arm off (31 replays, certified 0, refused
16, splits 15). At M10 all 30 volume refusals were
`props_meridian_on_surface`; PR 7370 re-posed that reading. Now:

- the root replay refuses as before on `ring_outer_locus_gap`
  (`[0, 1.0367]` on face 1v1, `[0, 0.9860]` on face 2v1);
- 21 replays: `VolumeUncomputable { face 7v1, ... "props_chart_junction_closed" }`
  with `MarginDiag(Invalid)`, at the inventory check: new, and the
  margin is not a number;
- 9 replays: `props_meridian_on_surface`, `[0, 32.302]`.

**Not measured:** what makes the junction's margin invalid (a quotient
whose divisor's enclosure holds zero is the suspect: `radius_gap`'s
precondition over a wide box), and why the ring's locus gap reads as
wide as the ring.

If the invalid margin is `radius_gap`'s, it is a later escape of PR
7370 and is fixed with
`the-radius-gap-reads-the-squares-rounding-at-far-scales`. It holds
the switch's deletion: the plate certifies nothing over two
parameters. (BOXFINISH orchestrator)
