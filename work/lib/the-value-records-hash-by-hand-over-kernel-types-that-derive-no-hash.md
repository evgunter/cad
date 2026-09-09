---
id: the-value-records-hash-by-hand-over-kernel-types-that-derive-no-hash
kind: issue
title: pncad-py: nine value records hash by hand over kernel types that derive no Hash
status: open
opened: 2026-09-09
---



Found by LIB-HASH-2 in the same walk that filed
`the-tag-mirrors-hash-over-kernel-enums-that-derive-no-hash`: every
class the compiled module exposes, measured against the rule Ev's (B')
ruling states — **the Python class mirrors the Rust type's derives**.
The tags are that row; these are the records with fields, and they are
a different question because a hand-written hash over a struct has to
decide what it folds.

## The nine

Each carries a hand-written `__eq__` and `__hash__` over a Rust type
that derives no `Hash`:

| Python class | `__eq__` / `__hash__` | Rust type | its derives |
| --- | --- | --- | --- |
| `Denotation` | `crates/pncad-py/src/py/readback.rs:183`, `:193` | `editor_core::Denotation` (`crates/editor-core/src/names/interrogate.rs:53`) | `PartialEq, Eq` |
| `Distribution` | `crates/pncad-py/src/py/analysis.rs:471`, `:480` | `editor_core::Distribution` (`crates/editor-core/src/distribution.rs:52`) plus the dimension the offsets were written in | `PartialEq` |
| `DocParamValue` | `crates/pncad-py/src/py/doc.rs:3043`, `:3048` | `editor_core::DocParamValue` (`crates/editor-core/src/doc.rs:103`) | `PartialEq` |
| `FaceCensus` | `crates/pncad-py/src/py/value.rs:1574`, `:1580` | `step_import::FaceCensus` (`crates/step-import/src/lib.rs:205`) | `PartialEq, Eq` |
| `Frame` | `crates/pncad-py/src/py/place.rs:352`, `:358` | `editor_core::Frame` (`crates/editor-core/src/placement.rs:74`) | `PartialEq` |
| `McAssertion` | `crates/pncad-py/src/py/analysis.rs:994`, `:1000` | `editor_core::McAssertion` (`crates/editor-core/src/mc.rs:168`) | `PartialEq` |
| `McConfig` | `crates/pncad-py/src/py/analysis.rs:843`, `:847` | `editor_core::McConfig` (`crates/editor-core/src/mc.rs:77`) | `PartialEq, Eq` |
| `McMeasure` | `crates/pncad-py/src/py/analysis.rs:922`, `:932` | `editor_core::McMeasure` (`crates/editor-core/src/mc.rs:147`) | `PartialEq` |
| `ValidationFinding` | `crates/pncad-py/src/py/value.rs:503`, `:509` | the binding's OWN `validation::Finding` (`crates/pncad-py/src/validation.rs:40`) | `PartialEq, Eq` |

## Why this is not one question with the tags

- **A record's hash is a choice, not a discriminant.** Five of the
  nine bottom out in `f64` (`Distribution`, `DocParamValue`, `Frame`,
  `McAssertion`, `McMeasure`), so each already had to answer the float
  question `Expr` answered — and each did, through the kernel's own
  fold where one exists (`Distribution::fold_signed_zeros`,
  `crates/pncad-py/src/py/analysis.rs:475-486`). Those folds are the
  work that would be thrown away by removing the hashes, and the
  reason the `-0.0` inconsistency (B') dissolved for `Length` does not
  apply here.
- **`ValidationFinding` has no kernel type to mirror at all.** Its
  Rust side is a projection the BINDING authors
  (`crates/pncad-py/src/validation.rs`), so "mirror the Rust type's
  derives" reduces to "mirror a derive list this crate wrote itself",
  which is not a constraint. It is listed here because its `__hash__`
  is the same shape, not because the rule obviously speaks to it.
- **`Frame` and `DocParamValue` are the ones that read most like
  `Length`.** A frame is a pose and a param value is a magnitude with
  a dimension; if the rule reaches anything on this list, it reaches
  these two first.

The repair, whichever way it goes, is one decision per direction:
either these types gain `Eq, Hash` in the kernel (the reading
`the-unit-classes-hash-over-a-partialeq-only-newtype` names, and the
one that costs Python nothing), or the nine classes lose their
`__hash__` and join `UNHASHABLE` with the nine LIB-HASH-2 wrote
reasons for. Nothing here is being decided by a lane.

## Question for Ev (2026-09-09, LIB orchestrator; `[ev]` PR)

Asked as one question with three siblings — the full text with options and the recommendation is on `the-tag-mirrors-hash-over-kernel-enums-that-derive-no-hash`. For this item under the recommended (A): the float-free records (`Denotation`, `FaceCensus`, `McConfig`) get `Eq, Hash` upward in the kernel and Python moves nothing; the five that bottom out in `f64` keep their hand-written hashes as the (B′) carve-out generalised (recipe data folding the zero through the kernel's own fold); `ValidationFinding` mirrors the binding's own projection and stays as it is. Under (B): all nine lose `__hash__`.

## Ruled (2026-09-09, Ev on `[ev]` PR #2265): (A), case by case

`McConfig` and `Denotation` gain `Eq, Hash` upward in the kernel
(exact fields; a memo key and a set), Python unchanged. `Frame`,
`DocParamValue`, `Distribution`, `McMeasure`, `McAssertion` (the
float-bearing five), `FaceCensus` and `ValidationFinding` lose their
Python `__hash__` and join `UNHASHABLE` with the mirror reason, Rust
unchanged. Mechanical unit LIB-MIRROR.
