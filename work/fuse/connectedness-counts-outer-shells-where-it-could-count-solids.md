---
id: connectedness-counts-outer-shells-where-it-could-count-solids
kind: issue
title: The Connectedness check counts decided Outer shells by a sign walk, where a solid is now one piece and it could count solids
status: open
opened: 2026-10-03
priority: P3
cost: M
---

Left by PR 3891 (the piece rule's sort), from the FUSE orchestrator's
rework list: "`Connectedness` counts solids instead of probing".

`crates/editor-core/src/checks.rs`, `connectedness`: the component
count is `topo::classify_shells` over each root output, counting
decided `Outer` shells, with `CheckEvidence::Escalated` /
`Unsupported` findings where a shell's role does not decide. Under Ev's
ruling (PR 3901) a solid is one piece of material, every verb sorts
its result that way, and tier 3's check 10 refuses two `Outer` shells
under one solid. So on the bodies the editor builds the count of
decided `Outer` shells equals `body.solids().count()` — except a solid
with one decided `Outer` and a shell whose role stays undecided, which
the sort leaves alone and which may really be a second piece
(`work/fuse/one-home-for-where-a-shell-stands.md`, NOTE-1): there the
classifier escalates where a solid count would say 1. Counting solids
would need no sign walk and no escalation.

Not done in PR 3891 because it retires the check's two escalation
arms, whose bindings run through `crates/pncad-py` (`tags.rs`,
`check_payload.rs`, `tests.rs`), the façade's doc in
`crates/pncad/src/document.rs`, and two `dsc_checks.rs` rows
(`thin_sheet_…`, `in_band_void_shell_escalates_with_its_valued_ending`)
that exist to pin those escalations. Until then the check pays a sign
walk per shell, and answers differently from a solid count only on the
undecided-shell case above.
