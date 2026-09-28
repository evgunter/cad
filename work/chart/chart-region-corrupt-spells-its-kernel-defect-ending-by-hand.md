---
id: chart-region-corrupt-spells-its-kernel-defect-ending-by-hand
kind: issue
title: topo: ChartRegionError::Corrupt spells its kernel-defect ending by hand, without the marker
status: open
opened: 2026-09-28
---


(ENCL implementer, from the sweep of
`work/encl/kernel-defect-endings-and-repair-labels-have-no-shared-home.md`.)

## What

`topo::chart_region::ChartRegionError::Corrupt`'s `Display`
(`crates/topo/src/chart_region.rs:434`) ends "a kernel invariant, not an
input a caller can repair. Rebuild the body through the Euler
operators, and report this: …": a hand-spelled kernel-defect ending
with no `There is no way through` marker (zero to
`test_utils::refusal::recourse_markers`), behind a `chart-region:`
stage prefix. The ending has one home now,
`geom_core::KERNEL_OR_FILE_DEFECT_ENDING` for a body that may have
been read (`KERNEL_DEFECT_ENDING` otherwise).

## Repair shape

End the sentence in the shared constant; the prefix and the rest of
the prose are `chart-refusal-prose-outgrows-the-viewer`'s.
