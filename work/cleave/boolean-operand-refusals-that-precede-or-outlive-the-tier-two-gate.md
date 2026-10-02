---
id: boolean-operand-refusals-that-precede-or-outlive-the-tier-two-gate
kind: issue
title: Boolean refusals that run before the tier-2 operand gate, and operand-blaming raises whose premise the gate may now rule out
status: open
opened: 2026-10-02
---


Found by the style review of PR 3860
(`a-strut-bearing-operand-passes-the-boolean-gates-and-refuses-at-the-join`),
which gates every Boolean operand in `reduce::gate_operand_pairs` on the
validator's own verdict: tier 1 → `CorruptOperand { corruption:
Structure }`, tier 2 → `ScaffoldingOperand`. Two halves, both
**unmeasured**:

**1. Refusals that reach an operand before that gate.** A slit or torn
operand can refuse as something other than what it is when a check that
reads its geometry runs first:

- `crates/topo/src/boolean/ops.rs` `boolean_op_with` (≈:481): the ∖/∩
  front door's `reduce::first_unsupported_pair` (`RevertRoster` →
  `CurvedPairUnsupported`) runs on the raw operands before
  `gate_operand_pairs`.
- `crates/topo/src/boolean/mod.rs` `boolean_reduce_declared_strategy`
  (≈:3029-3031): `validate_declarations`, `verify_declared_contacts` and
  `DeclaredPairs::build` read carriers and face extents of undeclared-
  valid operands before the gate.

Neither ordering is pinned. Whether a real input (a slit operand with a
torus face, or one carrying a declaration) lands there is not measured.
The repair candidate is to run the tier gate first at both doors.

**2. Operand-blaming raises whose premise the gate may now rule out.**
With tier 1 and tier 2 enforced at the gate, these may no longer be
reachable on an operand, or be reachable only on a body the pipeline
itself modified (after null-edge insertion), in which case blaming the
operand is wrong:

- `CorruptOperand` (`Corruption::Vertex`) raises:
  `crates/topo/src/boolean/insert.rs` (≈:480, :483, the strut-site
  successor walk), `crates/topo/src/boolean/sectors.rs` `corrupt`
  (≈:144), `crates/topo/src/boolean/vtxfac.rs` (≈:118, :121, :568,
  :571, :672, :675).
- `crates/topo/src/boolean/reduce.rs` `ContainError::Corrupt` mapping
  (≈:3182): raises `CorruptOperand` with `VertexKey::default()`, a
  vertex the refusal does not have.
- `crates/topo/src/boolean/reduce.rs` `gate_maximal_faces` (≈:586):
  `let Ok(sides) = edge_sides(..) else { continue; }` silently skips an
  edge whose sides do not read; on a tier-1-valid operand that may now
  be impossible and should say so.

Each needs tracing: which body it reads (a gated operand, or a
pipeline-modified one), then either an invariant or a refusal that names
the real cause.
