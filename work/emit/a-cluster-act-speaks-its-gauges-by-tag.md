---
id: a-cluster-act-speaks-its-gauges-by-tag
kind: issue
title: A cluster act's sentence (ClusterMaintenance) names its gauges by bare tag
status: open
opened: 2026-10-01
priority: P3
cost: M
parent: node-labels-are-document-data
---


Found by the sweep on `kernel-door-refusals-beyond-edit-speak-the-node`; it was on no row.

`ClusterMaintenance`'s `Display` (`mate/solve.rs`, `impl Display for ClusterMaintenance`) says "the cluster gauged by node <tag> …" in all four arms (`Join`, `Split`, `GaugeRewrite`, `Drop`). `Maintenance::Cluster`'s `Display` (`edit.rs`) delegates to it, so the edit door's report names its gauges by bare tag, beside the strand and orphan rows, which now speak their nodes with labels.

`ClusterMaintenance` is logged and replayed (`Applied::cluster_rows`, `LoggedEdit`), so it must keep its ids. The fix is the shape `SpokenName` took for `StableName`: `Maintenance::Cluster` carries the gauges spoken at the edit door beside the act, by `mate::solve::gauge_spoken`'s rule (the document the edit leaves, or the one it found for a dropped gauge). Python's `Maintenance` exposes no sentence and the viewer does not word a cluster act (`frame::maintenance_notice`), so today the sentence is read through `Display` alone; the cost is mostly the type change.
