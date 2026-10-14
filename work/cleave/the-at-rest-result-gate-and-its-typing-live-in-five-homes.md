---
id: the-at-rest-result-gate-and-its-typing-live-in-five-homes
kind: issue
title: Five doors gate a result at rest and type the refusal, each in its own words: only the Boolean and the split share the typing
status: open
opened: 2026-10-14
priority: P3
cost: M
---


Found by PR 5100's review (`cleave/seam-split-invalid-ok`). That PR lifted
the Boolean's finished-body typing into `topo::validate::at_rest_refusal`
(in-band shells → `BooleanDecision::ShellRole` escalation, the kernel's
findings → every finding carried) and `write_result_not_at_rest` (one
spelling). Since then the Boolean's `gate` (`boolean/ops.rs`) and the split
(`splitting::split`, `SplitFinishError::ResultNotAtRest` /
`ResultEscalated`) share the typing.

Three more homes gate a result or an operand at rest and type what they
find on their own:

- `editor-core` `eval/wire.rs` `finished_operand`: refuses every finding as
  `NodeErrorKind::UnfinishedOperand { errors }`, with no in-band arm;
- `editor-core` `product.rs`: `gate_at_rest_kept` on the aggregate,
  `gate_at_rest_declared`, and `gate_at_rest` per body, each typed there;
- `topo::shell` (`shell.rs`, `validate_geometric` → `ShellError::NotValid`),
  which calls `validate_geometric` directly rather than the policy.

So an in-band shell found by the shell door or the editor seat is not the
`Escalated` with a lever that Ev's PR 4335 rule gives it at the Boolean
and the split.

**What closes it:** route each through the policy and `at_rest_refusal`, or
say per door why its typing differs. The sweep doors' missing gate is
`work/carve/the-sweep-doors-return-bodies-tier-3-refuses.md`.
