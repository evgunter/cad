---
id: contact-verdict-is-a-rung-under-contact-contradicted
kind: issue
title: pncad: topo::ContactVerdict, ContactContradicted's verdict, is a payload rung on no curated list and has no Python word
status: open
opened: 2026-11-09
priority: P4
cost: E
---



(SEAL implementer, `seal/tangent-verdict`, `work/seal/a-tangent-reading-returns-a-verdict-and-only-a-record-is-contradicted`.) `scripts/payload-rung-sweep.py --check` found the new rung.

## What

`topo::ValidationError::ContactContradicted { record, verdict }` (`crates/topo/src/validate.rs`) carries `topo::ContactVerdict` (`crates/topo/src/contact.rs`): the reading that rules out what a face–face record claims, `Rest { row: RestRow, .. }`, `NotTangent { apart, .. }`, `Ruled { row: TangentRow, .. }` or `SidesMixed`. The prelude carries `ValidationError` and `ContactPair` (`crates/pncad/src/prelude.rs`, section 5) but not `ContactVerdict`, `RestRow` or `TangentRow`, and Python's `ValidationFinding` has no word for them. The sweep's `DISPOSITIONS` files the rung here, with the non-carriage argued beside the declaration.

## Why it is not carried yet

Every verdict ends the same way (the defect ending, no lever), and the record's two faces ride in `record`, which the prelude carries. A caller can tell the finding and the faces apart without the verdict.

**The falsifier is a caller who must act on which reading ruled the record out**: a viewer that draws the tangent line for a tangent ruling and both faces for a `Rest` one, or a Python caller that reads `verdict.fact()` rather than the sentence.

## Repair shape

Carry `ContactVerdict`, `RestRow` and `TangentRow` in the prelude's section 5 beside `ValidationError`, and give `ValidationFinding` a word for the verdict's kind (one attribute, `None` on every other arm, as `stale_kind` is). Then delete the disposition row and the pointer in `ContactVerdict`'s doc.
