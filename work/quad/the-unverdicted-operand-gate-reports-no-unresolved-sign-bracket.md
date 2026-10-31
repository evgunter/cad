---
id: the-unverdicted-operand-gate-reports-no-unresolved-sign-bracket
kind: issue
title: An unresolved sign met at the unverdicted operand gate carries no bracket until its doors may spell Bounds
status: open
opened: 2026-10-26
priority: P2
cost: M
---


Check 7's `ValidationError::VolumeSignUnresolved` carries the volume
bracket the sign walk stopped on (`volume: Option<(f64, f64)>`, measure
door unit C, PR 5741), read outward by `outward_volume!` in
`crates/topo/src/validate.rs`. The doors that already spell
`geom_core::Bounds` hand check 7 that reader: `validate_geometric_certified`
and `tier3_local_checks_marked`. One door does not:
`wound_negative`, which is `AtRestBody::gate_unverdicted`
(`crates/topo/src/validate.rs`), the gate an operand that carries no
verdict (a dual) meets at the split, Boolean and blend doors. There the
refusal's `volume` is `None`.

Reading the bracket there means writing `Decide + Bounds` on
`gate_unverdicted`, `wound_negative` and their callers in
`crates/topo/src/boolean/reduce.rs`, `crates/topo/src/splitting/mod.rs`,
`crates/topo/src/splitting/section.rs`, `crates/topo/src/lib.rs` and
`crates/sweep/src/blend/build.rs` — compound bounds in files
`scripts/gates/bounds-allowlist.sh` does not ratify. So the fix waits on
a ratification under `geom-core/src/real.rs`'s `Bounds` scope rule: an
entry naming those doors as reporting a bracket end and deciding
nothing with it (the rule's "a bracket may … be reported"), with the
necessity argument the rule asks for. Every scalar the gate runs at is
already `Bounds` (`f64`, `Interval`, `Dual`, `Sym`, `Probe`), so the
bound costs no caller.

Fix: ratify the entry, then pass `outward_volume!()` from
`wound_negative` as the other two doors do, and pin a dual operand's
unresolved sign reporting `volume: Some(..)`.

## Re-checked on main `9267d3ae` (QUAD tracker sweep, 2026-10-27)

Still holds. `wound_negative` (`crates/topo/src/validate.rs:5891`)
passes `&|_| None` to `plus_v_by_sign` (`:5790`). The two doors that
report pass `outward_volume!()` (`:5739`, `:7933`). `gate_unverdicted`
is at `:10026`.
