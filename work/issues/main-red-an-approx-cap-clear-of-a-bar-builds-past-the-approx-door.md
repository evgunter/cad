---
id: main-red-an-approx-cap-clear-of-a-bar-builds-past-the-approx-door
kind: issue
title: main red: sweep's an_approx_cap_clear_of_a_bar_reads_alike_at_every_pose builds the union its pin expects refused at the classification's Approx door
status: open
opened: 2026-10-27
priority: P1
cost: E
---

Filed by the SPLICE interior-arm lane (`splice/interior-arm`), which
split its slow-set reds against `origin/main` at `6117d399` in a
separate worktree.

## What

`crates/sweep/tests/operand_gate_pose.rs`,
`an_approx_cap_clear_of_a_bar_reads_alike_at_every_pose`, is red on
main with the same message as on the branch:

    ε 1e-9, identity, ∪: the cap clears the bar, so the op reaches the
    classification's Approx door: Ok(true)

The row pins `Err(Containment(KindUnsupported { kind: Approx }))`
("`Approx` stays off the boolean roster by decision"); main builds the
union. Whether the build is right (its volume) or the door was lost is
the question; the row asserts the refusal only.
