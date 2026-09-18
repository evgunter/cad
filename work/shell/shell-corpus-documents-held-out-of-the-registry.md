---
id: shell-corpus-documents-held-out-of-the-registry
kind: issue
title: cup and vessel sit beside the corpus registry: the Dual64 row and the shell door's certification rights
status: open
opened: 2026-09-06
---

`corpus/cup.rs` and `corpus/vessel.rs` are corpus documents that sit
BESIDE `corpus::documents()` rather than in it (the `die_composed`
disposition, `crates/editor-core/tests/corpus/mod.rs`), and `Shell` is
listed on `m4_pr8_corpus.rs`'s `FRONTIER_UNCOVERED` beside `Sweep`.

Why: registry membership runs every document at `Dual64` and requires
it green (`m10_di_dual_corpus.rs::every_document_evaluates_at_dual64_with_the_f64_value_channel`,
DL2 + DL3 end to end), and a dual has no shell door. The kernel verb's
last act is the certified at-rest validator, so its signature demands
certification rights (`crates/topo/src/shell.rs:713`,
`T: Decide + PropsQuadLane + CertifiedBounds`) that a dual
structurally lacks (DL3; `geom_core::Dual` implements no
`CertifiedEnclosure`). The lowering therefore has a per-scalar lane
door (`crates/editor-core/src/verbs/shell.rs`, `ShellLane`) that
answers `None` at a dual, and the node refuses TYPED
(`NodeErrorKind::ShellLaneUnsupported { lane: "Dual" }`), pinned by
name in `lib_g17_shell_node.rs::a_dual_evaluation_refuses_the_shell_typed`.

What the hold-out costs: the registry's rows — every ε row, the
Interval lane, persistence, latency, the corpus name digests, the
`m10_p_fence` digests — do not run over the two documents by
membership. `lib_g17_shell_node.rs` runs the ones it can by hand (both
lanes, persistence, the bump); the digests and the latency manifest do
not see them.

Resolutions, each someone else's call: DUAL's universal row learns to
name, by document, a lowering with no dual lane (a witness set pinned
by name, the shape the gather row already has), after which the two
documents register and `Shell` leaves the frontier; or the shell door
gains a non-validating form at a dual (SHELL's, and against the DL3
pairing obligation). Until one lands the registry's universal claim
stands and the two documents stay beside it.

## Re-homed (2026-09-08, LIB orchestrator)

Moved from `work/lib/` to `work/shell/`: the hold is the shell verb's certification rights under a dual lane (`crates/topo/src/shell.rs:713`, `crates/editor-core/src/verbs/shell.rs`) — SHELL's or DUAL's call, and SHELL owns the verb; the registry rows are TCOST's file and are one line each once decided. Id, body and header
are unchanged; the directory is the claim (`work/README.md`). LIB's
half — the Python/façade rows that move when this closes — is named in
the body and stays LIB's to execute once the kernel side lands.
