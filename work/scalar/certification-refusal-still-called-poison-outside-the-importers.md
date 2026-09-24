---
id: certification-refusal-still-called-poison-outside-the-importers
kind: issue
title: outside the certification importers, holders and tests still call the certification refusal 'poison'
status: open
opened: 2026-09-24
priority: P4
cost: D
---

## Finding

RING-5 (#3174) renamed the certification constructor `poison` →
`Certification::refused`, and its fix pass swept the word out of the
fifteen `CERT_IMPORTERS` files for the certification refusal (comments,
docs, locals, private helpers, test names and messages, user-facing
refusal notes; `TensorNet::poisoned` → `TensorNet::refused`).
Evaluation's poison keeps the word (`Real::is_poison`, `f64` NaN, a lane
`T`'s poison, the generic `CurvePlan::apply_points` fallback).

Outside that fence the certification refusal is still called poison.
Case-insensitive `poison` hits at the fix pass's head, per file, NOT
dispositioned hit by hit (many are evaluation's word and correct):

| file | hits | role |
| --- | --- | --- |
| `crates/geom/src/curves/nurbs.rs` | 69 | holder (`rational_span_bound`) and lane code |
| `crates/geom-core/tests/m5_pr1_poison_conservation.rs` | 27 | certification-refusal suite, named for the old word |
| `crates/geom/src/surfaces/nurbs.rs` | 20 | holder |
| `crates/geom-core/tests/certified_door.rs` | 19 | the door rows (its corpus label moved to `refused`) |
| `crates/geom/src/net.rs` | 14 | holder (`ring_coords`) |
| `crates/mesh/src/nurbs_cert.rs` | 8 | holder; its user-facing note "…hull is unbounded/poisoned —" is the twin of `chords.rs`'s, now "unbounded/refused" |
| `crates/geom-brep/src/ssi.rs` | 7 | holder (`pcurve_windows`) |
| `crates/step-import/src/recognize_curve.rs` | 5 | holder |

Also: `work/ssi/plane-nurbs-ssi-misblames-control-net.md` quotes the
old `ssi/exhaust.rs` note ("…control-net enclosure poisoned over a
cell"), which is now "…refused over a cell".

## What would close it

The same sweep over the holders and the certification test files:
each hit read, the certification refusal re-worded, evaluation's poison
left. Naming hygiene (P4); no number moves unless a pinned message does.
