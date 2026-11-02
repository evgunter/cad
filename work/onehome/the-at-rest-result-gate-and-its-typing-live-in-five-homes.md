---
id: the-at-rest-result-gate-and-its-typing-live-in-five-homes
kind: issue
title: Five doors gate a result at rest and type the refusal, each in its own words: only the Boolean and the split share the typing
status: dispatched
opened: 2026-10-14
priority: P1
cost: M
branch: onehome/finished-body
---


Found by PR 5100's review (`cleave/seam-split-invalid-ok`). That PR lifted
the Boolean's finished-body typing out as `topo::boolean::at_rest_refusal`
(`boolean/ops.rs`; in-band shells → `BooleanDecision::ShellRole` escalation, the kernel's
findings → every finding carried) and `topo::validate::write_result_not_at_rest` (one
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


## Duplication triage (INTENT, 2026-10-26)

Classified on Ev's ask, via the CLEAVE note on INTENT's log, against main `e92fb951` and DESIGN `[one-way.places]` / `[one-way.readings]`. The four classes:
- **design signal:** DESIGN names no single home, so a design conversation is owed;
- **mechanical:** DESIGN names the home; delete the off-home copy and repoint its users;
- **D10 debris:** a D10 stage retires one copy;
- **not duplication.**

**Mechanical, plus a small gap DESIGN leaves open** (cluster E). **Cluster E, where a refusal's typing lives.** The places table names the at-rest gate and "one recourse per decision", but not where the gate's refusal typing lives, so it lands in whichever consumer wrote it first. There is one gate (`AtRestPolicy::gate_at_rest*`). The typing `at_rest_refusal` lives in `boolean/ops.rs`, inside a consumer. `shell.rs` goes around the gate (`validate_geometric` → `ShellError::NotValid`), and editor-core types findings its own way. `product.rs`'s census is D10 stage 5 C debris (ATREST). It recurs the closed row `boolean-and-split-result-gates-are-one-gate-with-two-texts`. Unclear: where a gate's refusal typing lives.

## Slate triage (2026-10-26, CLEAVE)

- Re-banded P3 → P1: one gate's typing in five homes: two implementations of one underlying logic, P1 by `work/README.md` Priority.
- Moved to ONEHOME (`work/onehome/`) by `git mv`, id and body unchanged: the P1 track opened for CLEAVE's duplication rows; INTENT is sorting each into design signal or a mechanical deletion (`work/intent/log.md`, 2026-10-26), and the sort goes on the row. CLEAVE measured 270.5 budget points against 30 and split on its priority seam (Ev, in chat, 2026-10-26).

## Decided (Ev, PR 6326, 2026-11-02)

The design is in DESIGN `[topology.tiers]`, `[scalars]` and the at-rest places row. In short:
- The scalar's `finish` is the one constructor of a finished body. Its refusal is the non-empty **list** of findings, none folded into another, each read through its **family's** one text and recourse. `at_rest_refusal`, `finding_arm` and the two-arm `AtRestRefusal` go, as does the "binding reading, counting the others" fold. Grouping `ValidationError`'s variants into families is the real work.
- Every door that returns a body returns the scalar's finished type, and `editor-core`'s `Framed` stores it. Every consumer re-gate goes: `finished_operand`/`UnfinishedOperand`, the `wire.rs` member read, `distribute::gated`, the `checks.rs` placement gate, `product.rs` `attribute_at_rest`, export's re-gate, and `gate_unverdicted`. The sweep doors finish at their own end (closes `work/carve/the-sweep-doors-return-bodies-tier-3-refuses.md`). Import goes through the same gate, naming the file.
- A dual's finished type is a `Replay` that has passed tier 2 alone (`validate_closed`), carries no verdict, and is read by nothing that reads a verdict. `AtRestOutcome`, `NotRunAtThisScalar`, `not_run`, `structural_gate`, the at-rest check's `NotRun` and door_meter's "not-run" go. Dual-only test rows expecting an orientation or join refusal move to the base scalar.
- The editor offers each finding's own lever, never one picked on the user's behalf.
- Separately sequenced after this: `certified-body-is-a-second-finished-body-type`.
