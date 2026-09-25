---
id: cluster-maintenance-acts-reach-the-outcome-but-not-the-line
kind: issue
title: A cluster act rides OpOutcome::maintenance but the status line words none of them
status: open
opened: 2026-09-24
priority: P4
cost: E
---


`OpOutcome::maintenance` carries every row an action's edits reported,
the A11 cluster acts (`Maintenance::Cluster`: a join, split, gauge
rewrite or drop of the mate graph's placement registry) among them.
`frame::maintenance_notice` (`crates/viewer/src/frame.rs`) words the
four DM7 arms through `Display for Maintenance` and answers `None` for
the cluster arm, so a mate edit that joins two clusters, or a delete
that drops one, shows nothing on the status line.

That was a choice made to keep the fix to the DM7 row, not a ruling:
the cluster acts were silent before and stay silent. The reason given
at the function is that a gauge and its frame are bookkeeping the
chrome names nowhere and the parts' placement is what the picture
draws. The case against is `Maintenance`'s own doc — "what the record
adds is VISIBILITY, at the door where the consequence happened" — and
`ClusterMaintenance`'s `Display`, which already words each act for a
reader. Deciding it is a chrome question (what a GUI user should be
told about a mate edit), and the change is one match arm and one row.

Found while building the maintenance report
(`the-viewer-drops-every-dm7-rename-report`).
