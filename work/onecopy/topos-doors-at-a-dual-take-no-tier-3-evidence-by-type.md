---
id: topos-doors-at-a-dual-take-no-tier-3-evidence-by-type
kind: issue
title: topo's Boolean, split, blend and shell doors at a dual take a Replay, which carries no tier-3 evidence; the premise is held by the evaluation service's contract, not by type
status: parked
opened: 2026-11-10
priority: P4
cost: M
refs: [a-tier-3-premise-a-dual-operand-skips]
blocked_on: [should-topos-dual-doors-take-tier-3-evidence-by-type]
---


Filed by `onecopy/dual-tier3` (`a-tier-3-premise-a-dual-operand-skips`), on the orchestrator's ruling (B).

**What that unit settled.** A dual evaluation now takes its build of record by type: `editor_core::evaluate_recorded` is a dual's one door, and it refuses every node the record refused (`NodeErrorKind::RecordRefused`). `evaluate` is bounded on `OwnRecord`, which no dual meets. So every dual operand the evaluation service hands topo is the same bits a certifying finish validated, and topo's readers that lean on a tier-3 premise cite one anchor: `topo::Replay`'s "A tier-3 premise at a dual".

**What it left.** topo's own public doors at a dual take `Replay<Dual<_>>`, tier 2 alone, and nothing in the type says tier 3 ever read the body: the Boolean, the split, the blend operands and the shell. A caller of topo at a dual directly, with no record, can hand them a horn torus, an edge off its carrier or a ring with `R ≤ r`. There, the premise sites read wrong rather than refuse:
- `torus_region.rs` and `ruled_region.rs` take an edge as a plane section unchecked;
- `solid_contain.rs` assumes the group's sense agreement and `R > r`;
- `contain.rs` assumes a vertex is on its face's carrier;
- `validate::corner_normal` folds a convention refusal into `ConeCornerUnread`.

**The question.** Should those doors take tier-3 evidence by type, so the premise holds without the contract? For example, a premise-bearing reader taking `&AtRestBody<T>` or a `Tier3Held` marker that `Replay` lacks.

That forces the dual path to either run tier 3's structural decisions at a dual, which is a DL3 revision, or to answer each premise site by its own row at every scalar. Fork row 196 (`docs/DESIGN-FORK-LOG.md`) records Ev's pushback on the first: "it was supposed to be impossible at the type level for a dual to enter into certification". So this is an `[ev]` question if it is taken up, and it is not built here.


## Re-scoped (`onecopy/topo-dual-tier3-evidence`): held by a gate; the type-level form is option B

**Measured.**
- `Replay::of` is `pub(crate)`. The one public way to mint a dual's finished body is `AtRestPolicy::finish` at a dual, and the sweep doors' leaf at a dual is that same `Replay`.
- In production code, only editor-core's evaluation instantiates topo's or sweep's doors at a dual. Since PR 6735 that path is bound to its build of record.
- No other crate (sweep's `src`, profile, mesh, step-import, pncad, pncad-py, viewer) names a concrete dual outside docs and tests.
- The direct, unbound dual callers that exist are all tests: dual rows in 12 topo `src` files (test modules and `*_rows.rs`), 14 topo test files and 16 sweep test files.

**Held by a gate**, not by type: `scripts/gates/dual-doors-only-through-the-record.sh`. Production code names a concrete dual (an alias, `Dual<X>`, a struct literal, an associated path or a rename) only in its four homes:
- geom-core's `dual.rs` and `lib.rs` (the type);
- editor-core's `stackup.rs` (the E4/E5 driver, through `evaluate_recorded` only);
- editor-core's `analysis.rs` (the lane environment's tangent binding).

The gate states what it does not cover: test code, a macro-generated instantiation, a type alias defined in a home under another name, a generic caller instantiated through its parameter, and `include!`.

**The type-level form is option B**, as the ruling it parks on states it: a `RecordWitness<B>` handed to `finish` at a dual, a `Tier3Held` bound on the premise-bearing readers, the ripple through every finishing door, cost L. That is Ev's, because fork row 196 asks for "impossible at the type level", which the gate is not. So this row stays open, parked on that ruling.
