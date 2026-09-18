---
id: escalation-channel-misses-op-minted-indeterminates
kind: issue
title: k_stats: the escalation channel misses op-minted Indeterminates (eight sites), two raw sign_within calls, and the unbracketed mate solve
status: open
opened: 2026-09-05
refs: [k-stats-escalation-channel-and-redo, 1969]
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
