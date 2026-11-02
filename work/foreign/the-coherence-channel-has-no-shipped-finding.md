---
id: the-coherence-channel-has-no-shipped-finding
kind: issue
title: the import coherence channel's handling of a finding is read through no shipped body
status: open
opened: 2026-10-29
priority: P3
cost: M
---


Found by `import-normalizes-the-rim-only-cap`'s lane (FOREIGN implementer, 2026-10-29).

The import's chart-coherence channel (`ImportOptions::examine_chart_coherence`,
`StepImport::Solid::coherence`) is pinned by
`crates/step-import/tests/coherence_channel.rs` to report
`topo::examine_chart_coherence` verbatim on the body that ships. Its one
committed witness carrying a FINDING was `halfcap.step` (and its twins):
a meridian arc across the sphere's pole, read as a half-turn
(`MeridianClosure` / `MeridianContinuation`). A chart singularity in a
face is a vertex of it, so import now splits that arc at the pole
(`NormalizationKind::ChartSingularityVertex`), and every committed
import fixture ships with an empty findings list at every band.

So the channel row `the_channel_reports_the_kernel_door_verbatim` no
longer kills a mutant that drops, rewrites or permutes a finding: the
channel only ever sees bodies the door ships, and none carries one. The
door's own order is still pinned on a hand-built body
(`the_half_cap_findings_arrive_in_the_doors_order`,
`common::halfcap_with_its_arc_through_the_pole`), but that reads the
door, not the channel.

What would close it: a committed file the import ships whose body the
door reports a finding on (a condition other than the pole half-turn,
which import now removes: `RimContinuation` across a chart column, for
one), or a decision that the channel's finding path needs no import
witness because it is a pass-through. Measure what the door reports on
the wild corpus at each band first; the corpus-quiet row says it is
nothing today.
