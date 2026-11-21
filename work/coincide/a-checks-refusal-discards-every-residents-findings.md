---
id: a-checks-refusal-discards-every-residents-findings
kind: issue
title: A check's refusal discards every other resident's findings
status: open
opened: 2026-10-30
---


Found by the review of #6081 (the `unused` check), which pins today's behaviour as `intent_unused_check::a_product_refusal_discards_the_unused_findings`.

`run_checks_at_rest_on` (`crates/editor-core/src/checks.rs`) returns `Err` for the whole report as soon as one enabled resident cannot run:

- `connectedness` and `chart_coherence` each read `ev.usable(placement)` and refuse `ChecksError::Placement` on the first placement that did not evaluate;
- `unproven-coincidence` and `at-rest` refuse `ChecksError::Product` when the product does not gather.

Every finding the other residents already produced goes with it. The `unused` check reads only the recipe and has an answer on every document, yet under the default configuration a document whose product refuses (a mate naming no face) reports nothing of its unused named parameter; with the subject-reading residents `Off` it does. The registry's ordering (`CheckId::ALL`) decides only which refusal is reported, not whether findings survive.

The question is the report's shape: whether a resident's refusal becomes a per-resident entry beside the others' findings (a report that says "connectedness could not run: …" and still lists what `unused` found), or whether the all-or-nothing `Err` is the intended posture (`ChecksError` documents itself as "the checks could not RUN"). The viewer's Checks window shows the refusal alone today.

- 2026-11-21 (TESSLIE, `tesslie/retire-coherence`) — **the chart-coherence examination is retired** (Ev's ruling on PR 7224): `crates/topo/src/coherence.rs`, `topo::examine_chart_coherence` and its report types (`CoherenceReport`, `CoherenceFinding`, `CoherenceCondition`, `Unexamined`, `Unexaminable`, `StructureRead`), `CheckId::ChartCoherence` and `ChecksConfig::chart_coherence`, `ImportOptions::examine_chart_coherence` and `StepImport::Solid::coherence`, and `topo::chart_iso::classify_kind` are gone. `chart_coherence` is no longer one of the residents that refuse `ChecksError::Placement`; `connectedness` still does.
