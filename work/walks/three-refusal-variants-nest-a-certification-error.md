---
id: three-refusal-variants-nest-a-certification-error
kind: issue
title: three EulerOpError variants and one BlendError nest a certification error with four spellings of the same refusal
status: open
opened: 2026-09-14
refs: [S93]
priority: P1
cost: E
---

## What

Both S93 reviewers raised this, independently, as a style class rather
than a defect in the unit: `EulerOpError` now carries THREE variants
whose whole payload is "a certification failed, plus which entity it
was about", and the composite doors add a fourth spelling.

| variant | where | payload |
|---|---|---|
| `Certification { error }` | `crates/topo/src/euler.rs` | `geom_brep::CertifyError` |
| `RebasedCarrier { edge, error }` | `crates/topo/src/euler.rs` (S93, PR 2562) | `EdgeKey` + `CertifyError` |
| `PcurveSplit { edge, half_edge, error }` | `crates/topo/src/euler.rs` | `EdgeKey` + `HalfEdgeKey` + `geom_brep::PcurveCertifyError` |
| `BlendError::Certify { site, source }` | `crates/sweep/src/blend/mod.rs` | `&'static str` + `topo::PcurveMintError` |

Four shapes for one sentence — *this certification did not hold, and
here is what it was about*. Three name an entity, one names a static
site string; two nest `CertifyError`, two nest a pcurve error. A reader
matching on "the operation refused for a geometric reason" matches four
patterns, and `reports_tier1_corruption`, `merge_faces`' `OpPlacement`
and every `every_*_error_once` table list them one by one.

## Why it is a row and not a fix

It is a refactor of a public error enum with callers in three crates
(`topo`, `sweep`, and whatever reads `EulerOpError` in `boolean`), and
the S93 unit that surfaced it had no mandate to restructure error types
— the reviewers said so explicitly and this row records that.
`docs/prompts/reviewer-style-lane.md`'s class rule is what files it.

## Shapes

- **A carried site**: one variant `Certification { about: CertifiedAbout,
  error: CertifyError }` where `CertifiedAbout` names the entity (none,
  an edge, an edge-and-half-edge, a caller string). One arm everywhere,
  one `Display`, and `PcurveSplit`'s distinct error type is the obstacle
  — the pcurve certification is a different taxonomy and would have to
  be nested as a second arm of the payload, not collapsed into it.
- **Leave the variants and unify the READERS**: a
  `EulerOpError::certification_failure(&self) -> Option<…>` accessor
  that the three classifiers call instead of listing arms. Cheaper, and
  it does not touch a public shape.
- **Do nothing, and say so once**: the four are genuinely four
  operations and the cost is only that a reader lists them. Then the
  thing to write is the list, in one place.

The second is the smallest and buys the most of what the class costs
today (the arm-listing). Deciding is a later unit's.
