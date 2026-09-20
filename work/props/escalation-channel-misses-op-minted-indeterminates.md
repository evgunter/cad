---
id: escalation-channel-misses-op-minted-indeterminates
kind: issue
title: k_stats: the escalation channel misses op-minted Indeterminates (eight sites), two raw sign_within calls, and the unbracketed mate solve
status: review
pr: 2928
branch: props/escalation-channel
opened: 2026-09-05
refs: [k-stats-escalation-channel-and-redo, mate-lane-escalations-reach-no-nodes-log, topo-mints-indeterminates-outside-the-funnel, should-classify-replays-error-enum-arms-be-deleted, the-gating-corpus-reaches-no-collapsed-arm-gate, sector-shape-mints-indeterminates-through-an-invalid-helper, 1969]
---

## What

`k_stats::Bracket`'s escalation channel (PR #1969) records the
`Indeterminate`s the FUNNEL produces — `classify`'s own `Err` — and
nothing else. Three families of escalation reach a consumer without
ever passing through the log:

1. **Op-minted `Indeterminate`s after a definite verdict.** Eight
   shipped sites ask the funnel, receive a DEFINITE sign, and then mint
   an `Indeterminate` of their own (every one with
   `MarginDiag::Invalid`):
   `crates/geom-brep/src/enters.rs:205`, `:266`;
   `crates/geom-brep/src/dihedral.rs:167`, `:401`;
   `crates/geom-brep/src/pcurve_cache.rs:2527`;
   `crates/geom-brep/src/certify.rs:1555`;
   `crates/geom-brep/src/edge_nurbs.rs:345`;
   `crates/geom-brep/src/ssi/march.rs:371`
   (`grep -rn 'Indeterminate *{' crates/*/src` and keep the sites
   preceded by a `decide(...)` whose `Ok` arm returns the error).
   Executed (R1, `enters_material` with a collapsed lever arm): the
   caller receives `Err(Indeterminate { predicate: "enters_material_arm" })`
   while the frame holds `verdicts = [enters_material_arm: Zero]`,
   `escalations = []`. Pinned by
   `crates/geom-brep/tests/kstats_escalation_channel.rs`, which goes
   red when this item lands.
2. **Raw `sign_within` calls outside the funnel** — two, not one:
   `crates/topo/src/seqgen.rs:641` (a test-support candidate filter,
   documented as such) and `crates/editor-core/src/expr.rs:1214`
   (`refuse_non_finite`, a synthetic band over `value * 0`, never
   surfacing an `Indeterminate`).
3. **The whole-document mate solve** (`crates/editor-core/src/eval/mod.rs:2058`
   calls `mate::solve_document` before any node's bracket opens): its
   funnel escalations (`crates/editor-core/src/mate/solve.rs:462`,
   `:750`, `:908`) sit on no node's log — visible only to a caller's
   outer frame — and reach `drive::classify_replay` as
   `NodeErrorKind::Mate(MateFault::Indeterminate)`. Pinned by
   `asm_r2a_mate_solve::row7e_a_mate_solve_escalation_is_on_no_nodes_log_but_visible_in_an_outer_frame`.

## Why it is priced right today, and why it still needs closing

Family 1 carries `MarginDiag::Invalid`, and `drive::sliver`
(`crates/editor-core/src/drive.rs`) requires an `Enclosure`, so every
such escalation lands `Bisect` through the retained error-enum arms —
the same verdict the channel would give. Family 3 is right only because
`solve_document` runs at `f64`. The cost is structural: the two enum
arms in `classify_replay` are LOAD-BEARING (the code says so now) and
cannot be deleted by the sweep `indeterminate-error-arms-sweep`
schedules until this item lands; and `NodeValue::escalations` answers
"did any predicate escalate" for funnel predicates only (the k-stats
item's `## Closed` says so).

## Shape

Route family 1 through the funnel — a `decide` door that can return
the op's own indeterminacy as a classified outcome, or a
`k_stats::escalate(name, Indeterminate)` that records without deciding
— then bracket the mate solve (a document-level frame, or the solve
recorded per mate node), and delete the two arms. The spec of the
k-stats unit named this as a different unit; it is.

## Note (folded from `k_stats`' module doc)

Paths that never open a bracket still pay the `RefCell` borrow and an
empty-stack check per decision; gating that on a `Cell<bool>` is a
live optimization orthogonal to any feature. Unscheduled; recorded
here rather than in the module doc.


## Landed (PR 2928): family 1 closed at its eight sites, families 2 and 3 measured

**Family 1's eight sites no longer mint, because the mint was made
impossible THERE rather than by asking each site to record.**
`geom_core::k_stats` gained three gate doors — `decide_positive`,
`decide_nonzero` and `gate_measured` — which classify (or, for the last,
check that a value is a measurement at all) AND apply the caller's
requirement, so the rejection is minted and recorded by
`record_escalation`, the module's one MINT of the escalation channel. A
caller never holds a definite sign long enough to reject it in private,
because the call that would have handed it the sign hands it the
refusal instead. A gated rejection records BOTH channels — the definite
verdict the classifier reached and the gate's escalation beside it — so
the verdict channel is unchanged across the seam.

**The structural claim, stated at the width it actually holds.** What
ships is a spelling census over ONE crate:
`shipped_geom_brep_code_spells_no_indeterminate_literal` fails on any
`Indeterminate {` struct literal under `crates/geom-brep/src` outside a
`#[cfg(test)]` body, and there is none. It does **not** make the defect
unreintroducible. Three evasions are executed, not imagined:

- `margin.sign_within(band)` + `.with_predicate(name)` mints the exact
  payload with no `Indeterminate` token in the file. Both review arms
  of PR 2928 compiled and ran this and got an empty log.
- `k_stats::decide`, match `Positive`, build the refusal from a payload
  assembled anywhere but the call site.
- **A local helper, which is not hypothetical**: `topo` holds four
  separate `fn invalid(band, predicate) -> Indeterminate`-shaped
  helpers, minting after a definite sign at eight shipped sites, and the
  census would see none of them. Filed as
  `work/curved/topo-mints-indeterminates-outside-the-funnel.md` and
  `work/props/sector-shape-mints-indeterminates-through-an-invalid-helper.md`.

The census's own doc comment carries this list; the guard is worth
having and it is a guard on a spelling, not a proof about the kernel.

**Family 2 has no residue, measured rather than assumed.**
`topo/src/seqgen.rs`'s raw `sign_within` is a test-support candidate
filter over `Body<f64>` only, never instantiated at the recording
scalar, and nothing escalates out of it.
`editor-core/src/expr.rs`'s `refuse_non_finite` no longer calls
`sign_within` at all — it goes through `k_stats::check_unlogged` under
the name `expr_non_finite`, deliberately outside the verdict log, and
refuses as `EvalError::NonFiniteResult`, never as an `Indeterminate`.

**Family 3 is open and re-filed.** Closing family 1 does not close it:
the whole-document solve runs before any node's bracket opens, so no
funnel door helps.
`work/msolve/mate-lane-escalations-reach-no-nodes-log.md` carries it,
with `mate::coset::parallel`'s own hand-built mint — a two-line
`gate_measured` fit — on the same slate.

**What the gates are not covered by, with digits.** No committed
k-report baseline has ever taken one of these gates: five of the eight
predicates have ZERO rows across all twelve baselines, and the three
that appear (`dihedral_arm` 606,900; `enters_material_arm` 47,151;
`nurbs_span_meter` 12) have only `positive` rows. Every shipped caller
gates the same quantity before the door does, so reaching a gate needs
geometry a constructor refuses to build. Four unit rows in
`crates/geom-brep/tests/kstats_escalation_channel.rs` carry the entire
verification, and
`work/props/the-gating-corpus-reaches-no-collapsed-arm-gate.md` is what
closing that costs.

**Also filed**: `work/props/should-classify-replays-error-enum-arms-be-deleted.md`
(the precondition is discharged; the measurement is not) and
`work/props/nurbs-span-meter-cannot-tell-a-reversed-domain-from-a-collapsed-one.md`.

**The `RefCell`-cost note above is untouched** by this unit and stays
recorded here.
