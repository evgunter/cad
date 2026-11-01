---
id: r2-drum-reverted-cavity-row-asserts-what-every-reverted-body-satisfies
kind: issue
title: shell9_r2_probes::r2_drum_reverted_cavity_alone_is_the_reason asserts only is_err on a reverted body's tier 3, which NegativeVolume satisfies on every reverted body
status: open
opened: 2026-09-14
refs: [void-insertion-refuses-a-cavity-with-a-same-surface-latitude-seam]
priority: P3
cost: E
---

Found by the TOPO lane that closed
`revert-does-not-mirror-plane-chart-images` (2026-09-14), sweeping
`crates/sweep/tests` for rows that pin the drum's void-insertion
refusal. `crates/sweep/tests/shell9_r2_probes.rs`,
`r2_drum_reverted_cavity_alone_is_the_reason` ("Claim 3 — the drum's
refusal reason, by execution") builds the plain `drum(1.0, 2.0)`, not
the collinear-cap drum the claim is about, reverts its door-built
cavity and asserts `validate_geometric(&reverted).is_err()`. A
reverted body bounds the complement, so its tier 3 reports
`NegativeVolume` by `revert`'s ratified posture
(`crates/topo/src/revert.rs`, "Validity class"); the assertion is
therefore true of every reverted body and says nothing about the
drum's refusal — it stayed green while the refusal it names was
fixed. `docs/prompts/implementer-discipline.md` §2: name the runtime
value that would make an assertion false; here there is none. The
row's neighbours in `shell9_probe.rs` and `revert_plane_charts.rs`
pin the actual claim (`Err(vec![NegativeVolume])` exactly, and the
graft's meter edge for edge); this row should either assert the
exact verdict on the collinear-cap drum or be deleted as subsumed.
