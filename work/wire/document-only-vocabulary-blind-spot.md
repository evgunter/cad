---
id: document-only-vocabulary-blind-spot
kind: issue
title: The construct-hop censuses are anchored only on the kernel vocabulary: a document-only variant that launders into a kernel form is invisible
status: open
opened: 2026-09-12
---

## Finding

`editor-core`'s two vocabulary censuses at the profile construct hop
(`crates/editor-core/tests/switch_program_vocabulary.rs`) are both
anchored on the KERNEL vocabulary and only on it:
`every_arc_mode_is_a_document_program` walks `profile::ArcMode::ALL`
through `mode_witness`, and `every_target_form_is_a_document_program`
walks `profile::TargetKind::ALL` through `target_witness` (D364). That
is the right anchor for the failure those censuses exist to catch — a
kernel variant the document vocabulary never learns — and it is
deliberate.

The direction it does not cover is the mirror one. `ProgramArcData`
(`crates/editor-core/src/program.rs`, near `ProgramTarget`) and
`ProgramTarget` are document-layer enums in their own right. A variant
added to either must be discharged by the exhaustive matches that
consume it — `res_spec` / `res_target` (the construct hop), the wire
conversions, `spec_lit`, `spec_slots`, the content-key hashers — so it
cannot ship un-noticed. But every one of those arms may legally
resolve the new document variant into an EXISTING kernel variant, and
when it does:

- both censuses stay green, because `ArcMode::ALL` and
  `TargetKind::ALL` are still fully witnessed;
- the corpus clauses stay green, for the same reason;
- and the document form silently authors something the author did not
  write — the same laundering failure the censuses catch in the other
  direction.

The two witness functions are matches on the kernel tag, so nothing
forces a document-only variant to acquire a witness at all.

## Why it is not D364's diff

D364 gave `profile::Target` its tag and re-anchored the target census
on it. Closing THIS direction needs a document-side tag
(`ProgramTargetKind` / `ProgramArcMode`) with its own `ALL` — a third
spelling of each vocabulary in a crate that already spells them twice
by G1 layering, which is a design call about where the anchor belongs
rather than a test change. D364 states the blind spot in the census's
own doc comment and cites this file.

## Shape of a fix, if it is wanted

Either a document-side tag projected from the document declaration the
same way (two more macro invocations, two more `ALL`s), or a census
clause that pairs each kernel form with the SET of document variants
that resolve to it and asserts that set is exactly witnessed — which
still needs a way to enumerate the document variants.
