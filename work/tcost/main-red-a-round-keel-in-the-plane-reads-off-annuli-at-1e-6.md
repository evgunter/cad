---
id: main-red-a-round-keel-in-the-plane-reads-off-annuli-at-1e-6
kind: issue
title: main red (slow set, eps 1e-6): a_round_keel_in_the_plane_reads_two_annuli_today reads regions off the annuli by about 1.5e-6, since 11-02..11-03
status: open
opened: 2026-11-09
priority: P0
---

Nightly run 42567046151 (sha `836426e07e`), job `full suite (eps 1E-06, slow set)`: `crates/sweep/tests/a_curved_keel_in_the_plane.rs` `a_round_keel_in_the_plane_reads_two_annuli_today` reads `the V's tip 0.6 ε off the plane, the plane's normal -1 y: the section's regions hold [11.780973958926847, 15.707964775914], not the annuli [11.780972450961723, 15.707963267948966]`. The regions are about 1.5e-6 off the annuli.

Bisected by BOXES (sweep built with `--all-features`, row run at `CAD_TOLERANCE_EPS=1e-6`):
- green at `f1867c8b71^1` and at `f1867c8b71` (PR 6336);
- red at `06c73b5d51^1` and at `06c73b5d51` (PR 6383), and at `c036d7ae88^1`.

So it arrived between PR 6336 and PR 6383's first parent (`98aa78e74f`). The only BOXES merge there is PR 6334, which touches docs and `work/` only. Not BOXES'. The semantic question this row pins is `work/onehome/a-closed-keel-in-the-section-plane-divides-the-section-into-two-regions.md`; this one is the 1e-6 drift only.

(BOXES nightly-reds triage, 2026-11-09)
