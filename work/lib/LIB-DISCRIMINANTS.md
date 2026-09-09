---
id: LIB-DISCRIMINANTS
kind: unit
title: ValidationFinding carries stale_kind and ring_contact_kind
status: review
branch: lib/discriminants
opened: 2026-09-09
refs: [two-validation-payload-discriminants-still-uncrossed]
---

Ev's ruling (A) on `two-validation-payload-discriminants-still-uncrossed`,
executed: every payload DISCRIMINANT of a projected refusal crosses as
an attribute of its own, named per type, `None` on every other arm. The
two that were prose-only — `StaleDeclaration` (4 arms) and
`RingContact` (3 arms) — now cross as `ValidationFinding.stale_kind`
and `.ring_contact_kind`. Arena-key FIELDS do not cross, so the
projection stops at the discriminant.

## Delivered

- **Two tag maps**, exhaustive with no wildcard, beside the two the
  ruling's first pair minted: `stale_declaration_tag` and
  `ring_contact_tag` (`crates/pncad-py/src/tags.rs`). Deleting one arm
  from each fails the build with `E0004` at both sites (run, reverted).
- **Two attributes** on `validation::Finding` and on the
  `ValidationFinding` pyclass, present on every finding and `None` off
  their arm, populated by one extractor per payload
  (`crates/pncad-py/src/validation.rs`). `__repr__` and `__hash__`
  carry six words now, and equality was already the whole value.
- **The extractors use the EXTRACT-licence wildcard**, matching
  `census_contact`'s spelling in the same file rather than naming
  `ValidationError`'s other seventy arms twice. The brief's
  parenthesis said "`None` BY NAME" and also said to match
  `census_contact`; the two disagree, and matching the worked example
  is what was done. The licence is stated at that site already: the
  classifying map (`validation_error_tag`) is exhaustive, so a kernel
  arm added with a payload stops this crate compiling there, in front
  of the person who then decides what it projects.
- **The words.** `stale_kind` is `vertex_vertex`, `vertex_on_face`,
  `curve_locus`, `patch` — the arms' own names, which are the record
  GRANULARITY the kernel's `Display` names too. `ring_contact_kind` is
  `vertex_vertex`, `vertex_on_edge`, `edge_along_edge` — Ev's own
  spelling of the three shapes in the ruling, and the census
  vocabulary's words where the shape is the same one, rather than the
  arms' bare `Vertex` / `Edge`, which would put two words on this
  surface that name a contact by only one of its two sides.
- **Inventory, stub, docstring, census, ty fixture**: `TAG_INVENTORY`
  rows for both maps (`src/tests.rs`); `pncad.pyi` and the class
  docstring extended in the same voice; both census rows moved from
  `INTERIOR` to `BOUND_AS` with the measurement stated and the note
  above `CensusContact` rewritten to say the rule rather than the
  pair; a fixture line each in `tests/ty_fixtures/legal.py`.
- **The prelude's prose** (`crates/pncad/src/prelude.rs`) said "two of
  the four now cross to Python … `StaleDeclaration` and `RingContact`
  are still Rust-side only". It says all four now.
- **Reachability: no Python scene reaches either payload**, and the
  reason is per type and stated at three sites (the Rust pins, the
  census note, `tests/test_validate.py`'s unreachable-arms row).
  A stale record needs a declaration parted from its witness: `product`
  bodies are plain, `assemble` bodies are gated by the very census that
  would refuse, `Value.body` off a boolean carries only records the
  result still witnesses, and `ContactRecords` has no Python spelling.
  A ring on its own outer loop is built by raw Euler surgery (the shell
  verb's suites use `kfmrh`) and the binding exposes no Euler operator.
  So all seven arms are pinned in Rust by construction, one row per arm
  (`every_stale_declaration_arm_projects_the_payload_it_carries`,
  `every_ring_contact_arm_projects_the_payload_it_carries`), and one
  Python row drives the nearest scene that exists — a declared glue
  through the boolean that welds it — to show it is not one.
- **`witness-bifurcation-arm-has-no-inner-word` stays open** with a
  `## Progress` line: the rule now applies to it and it waits on the
  M6 solver constructing the arm.
