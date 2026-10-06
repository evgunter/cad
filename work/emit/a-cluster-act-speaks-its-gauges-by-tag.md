---
id: a-cluster-act-speaks-its-gauges-by-tag
kind: issue
title: A cluster act's sentence (ClusterMaintenance) names its gauges by bare tag
status: review
pr: 4107
branch: emit/cluster-gauges-spoken
opened: 2026-10-01
priority: P3
cost: M
parent: node-labels-are-document-data
---


Found by the sweep on `kernel-door-refusals-beyond-edit-speak-the-node`; it was on no row.

`ClusterMaintenance`'s `Display` (`mate/solve.rs`, `impl Display for ClusterMaintenance`) says "the cluster gauged by node <tag> …" in all four arms (`Join`, `Split`, `GaugeRewrite`, `Drop`). `Maintenance::Cluster`'s `Display` (`edit.rs`) delegates to it, so the edit door's report names its gauges by bare tag, beside the strand and orphan rows, which now speak their nodes with labels.

`ClusterMaintenance` is logged and replayed (`Applied::cluster_rows`, `LoggedEdit`), so it must keep its ids. The fix is the shape `SpokenName` took for `StableName`: `Maintenance::Cluster` carries the gauges spoken at the edit door beside the act, by `mate::solve::gauge_spoken`'s rule (the document the edit leaves, or the one it found for a dropped gauge). Python's `Maintenance` exposes no sentence and the viewer does not word a cluster act (`frame::maintenance_notice`), so today the sentence is read through `Display` alone; the cost is mostly the type change.

## Overtaken

The type this row names is gone. PR #3676 (`edit/placement-gauges`, commit `1441b5154d`) deleted `ClusterMaintenance`, `Maintenance::Cluster`, `Applied::cluster_rows` and `mate::solve::gauge_spoken` with the placement registry. This row was filed at 19:26 the same day, from a branch that had not merged that change, so on main there is no cluster act to speak. The one placement row an edit reports now is `Maintenance::OffsetCleared`, and it speaks its instance as a `SpokenNode`.

What was left were comments citing the deleted rule. The closing PR removes them:

- `written` and `spoken_before_else_after` in `edit.rs` named a "cluster gauge" exception through `mate::solve::gauge_spoken`.
- `Doc::name_carriers` in `doc.rs` and the header of `tests/rv_dm7_probes.rs` cited `an_appearance_strand_precedes_the_cluster_acts_of_the_same_delete`. That test is now `a_delete_reports_its_strands_alone_and_only_a_mate_insert_clears_an_offset`, and its own doc comment told the history of the rename. The PR cuts that history and keeps the invariant. The closing PR re-measured the doc's claim: with `Carrier::ALL` reversed, exactly the three cited rows fail.

Sweep: every arm of `Maintenance` (`OffsetCleared`, `Strand`, `StrandedAppearance`, `AnonymousVarRemoved`, `LabelDropped`) holds a `SpokenNode`, `SpokenName` or `SpokenVar`, so none of them names a node by bare tag.
