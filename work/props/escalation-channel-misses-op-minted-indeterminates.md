---
id: escalation-channel-misses-op-minted-indeterminates
kind: issue
title: k_stats: the escalation channel misses op-minted Indeterminates (eight sites), two raw sign_within calls, and the unbracketed mate solve
status: review
pr: 2928
branch: props/escalation-channel
opened: 2026-09-05
refs: [k-stats-escalation-channel-and-redo, mate-lane-escalations-reach-no-nodes-log, contact-verify-mints-indeterminates-outside-the-funnel, 1969]
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

## Landed (PR 2928): family 1 closed, families 2 and 3 measured

**Family 1 is closed, by making the mint impossible rather than by
asking each site to record.** `geom_core::k_stats` gained three gate
doors — `decide_positive`, `decide_nonzero` and `gate_measured` — which
classify (or, for the last, check that a value is a measurement at all)
AND apply the caller's requirement, so the rejection is minted and
recorded by `record_escalation`, the module's one write to the
escalation channel. A caller never holds a definite sign long enough to
reject it in private. All eight sites go through a door; none builds an
`Indeterminate`, and `kstats_escalation_channel.rs` carries both halves:
four rows driving gates through the public API, and a source census
asserting that shipped `geom-brep` code builds no `Indeterminate` at
all — so a ninth site cannot reintroduce the shape silently.

A gated rejection records BOTH channels: the classifier's definite
verdict (it really did decide) and the gate's escalation beside it. The
verdict channel is therefore byte-identical across this seam.

**Family 2 has no residue, measured rather than assumed.**
`topo/src/seqgen.rs`'s raw `sign_within` is a test-support candidate
filter over `Body<f64>` only, never instantiated at the recording
scalar, and its own comment argues the bypass; nothing escalates out of
it. `editor-core/src/expr.rs`'s `refuse_non_finite` no longer calls
`sign_within` at all — it goes through `k_stats::check_unlogged` under
the name `expr_non_finite`, deliberately outside the verdict log
(logging it refused every M10-6 min-clearance box on a vector mismatch),
and its refusal reaches the caller as `EvalError::NonFiniteResult`, never
as an `Indeterminate`. Neither is a hole in the channel.

**Family 3 is open and re-filed**, because closing family 1 does not
close it: the whole-document mate solve runs before any node's bracket
opens, so the doors above change nothing for it.
`work/msolve/mate-lane-escalations-reach-no-nodes-log.md` carries it,
on the program whose ground it lands on, together with
`mate/coset.rs`'s own hand-built mint — the same shape as the eight,
one crate over.

**A fifth sibling family, filed not widened**:
`topo/src/boolean/contact_verify.rs` holds four more mint-after-a-
definite-sign sites, two of which are mis-typed (a definite contradiction
wearing `MarginDiag::Invalid`).
`work/curved/contact-verify-mints-indeterminates-outside-the-funnel.md`.

**The `RefCell`-cost note above is untouched** by this unit and stays
recorded here.
