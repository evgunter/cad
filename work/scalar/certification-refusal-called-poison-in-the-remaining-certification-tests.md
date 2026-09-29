---
id: certification-refusal-called-poison-in-the-remaining-certification-tests
kind: issue
title: certification test files outside CERT-NAMES' list still call the certification refusal 'poison'
status: closed
branch: scalar/cert-names
pr: 3448
opened: 2026-09-29
closed: 2026-09-29
priority: P4
cost: M
---

## Finding

CERT-NAMES (`certification-refusal-still-called-poison-outside-the-importers`)
swept the eight files that row named. Its shape sweep — every `.rs`
file under `crates/`, `tools/` and `demos/` that names a certification
value (`is_certified`, `Interval::refused`, `from_certified`,
`certified_coords`, `CurveCertData`, `SurfaceCertData`,
`apply_certified`, `sup_bound`) and also says `poison` — found more
outside that list. Case-insensitive `poison` occurrences per file, NOT
dispositioned hit by hit; many are evaluation's word and correct:

| file | hits | what the sample read showed |
| --- | --- | --- |
| `crates/geom-core/tests/ring2_r2_probes.rs` | 32 | probe suite; unread |
| `crates/geom-brep/tests/m5_pr7_ssi.rs` | 26 | "the sweep's poison arm" is the enclosure refusal arm (`ssi/exhaust.rs`), e.g. near `:2035`, `:2087` (an asserted message) |
| `crates/geom-core/tests/interval_exact_fuzz.rs` | 17 | unread |
| `crates/geom-core/tests/m5_pr7b_tensor_compose.rs` | 11 | `sup_bound` NaN called poison; test names `a_sign_changing_weight_extension_poisons_the_bound`, `a_beyond_budget_composition_poisons_rather_than_rounds` |
| `crates/geom-core/tests/review_m5_pr2_scratch.rs` | 10 | unread |
| `crates/geom-core/tests/spline_hull.rs` | 9 | unread |
| `crates/geom/tests/curves/review_m5_pr4_adversarial.rs` | 8 | unread |
| `crates/geom-core/tests/review_m5_pr7b_tensor.rs` | 8 | `sup_bound` NaN; test name `the_budget_boundary_54_completes_and_57_poisons` |
| `crates/geom-core/tests/r2_cert3_probes.rs` | 7 | unread |
| `crates/geom-core/examples/r2_onb_dl6.rs` | 3 | unread |
| `crates/geom-core/tests/review_m5_pr2_scratch_hull.rs` | 2 | unread |
| `crates/geom-brep/tests/cert10_r1_probes.rs` | 2 | unread |
| `crates/geom-brep/tests/approx_surface.rs` | 2 | unread |
| `crates/geom/tests/curves/review_m5_pr2_e2e.rs` | 1 | unread |
| `crates/geom-core/tests/ring0_review_probes.rs` | 1 | unread |
| `crates/geom-core/tests/coeffs_pair_identity.rs` | 1 | unread |
| `crates/geom-core/tests/certified_endpoint_census.rs` | 1 | unread |
| `crates/geom-brep/tests/review_m5_pr7_enclosure.rs` | 1 | unread |

Not listed: `geom-core/src/{real,interval,linalg/vec,interval/certification}.rs`
and `demos/tour/src/chaintol.rs`, where the word is evaluation's by
definition (`Real::is_poison`, the NaI/empty convention) and the
sample showed nothing else; `geom-brep/src/ssi/certify.rs`, read in
full (its three hits are the lane `T`'s poison and two names of
`is_poison`); and the fifteen `CERT_IMPORTERS`, which
RING-5 swept.

The sweep's blind spot: a file that talks about a certification
refusal without naming any of those markers. `crates/topo` (hundreds of
`poison` hits, almost all the lane scalar's) was not read.

## What would close it

The same pass CERT-NAMES made: each hit read against RING-5's rule
("refused" for `!is_certified()`/`Certification::refused`, "poison" for
`Real::is_poison` and evaluation's NaN/NaI), certification refusals
re-worded with any asserted message moved alongside its test, and test
names renamed with `.config/nextest.toml`'s slow set checked.

## Folded into CERT-NAMES

This row rides PR #3448: its fix pass took the whole table above, so
the unit's boundary is the class (the certification refusal's
vocabulary in certification code and tests) rather than a file list.
Occurrences at the fix pass's merge base, each read against RING-5's
rule:

| file | hits | re-worded | kept, and why |
| --- | --- | --- | --- |
| `geom-core/tests/ring2_r2_probes.rs` | 32 | 8 | 24: the retired ring's `Old::poison`/`is_poison`, ported verbatim |
| `geom-brep/tests/m5_pr7_ssi.rs` | 26 | 25 ("poison arm" → "refusal arm", two test names) | 1: `rect_box`'s midpoint `0/0` at the lane `T` |
| `geom-core/tests/interval_exact_fuzz.rs` | 17 | 17 | 0 |
| `geom-core/tests/m5_pr7b_tensor_compose.rs` | 11 | 11 (two test names) | 0 |
| `geom-core/tests/review_m5_pr2_scratch.rs` | 10 | 10 | 0 |
| `geom-core/tests/spline_hull.rs` | 9 | 9 (one test name) | 0 |
| `geom/tests/curves/review_m5_pr4_adversarial.rs` | 8 | 2 | 6: F2's NaN control point at `f64` |
| `geom-core/tests/review_m5_pr7b_tensor.rs` | 8 | 8 (one test name) | 0 |
| `geom-core/tests/r2_cert3_probes.rs` | 7 | 6 (one test name) | 1: `normalize` at `f64` |
| `geom-core/examples/r2_onb_dl6.rs` | 3 | 0 | 3: `is_poison` printed beside `is_certified`, and lane evaluation |
| `geom-core/tests/review_m5_pr2_scratch_hull.rs` | 2 | 2 | 0 |
| `geom-brep/tests/cert10_r1_probes.rs` | 2 | 2 | 0 |
| `geom-brep/tests/approx_surface.rs` | 2 | 0 | 2: `implicit_residual`'s `f64` NaN |
| `geom/tests/curves/review_m5_pr2_e2e.rs` | 1 | 1 | 0 |
| `geom-core/tests/ring0_review_probes.rs` | 1 | 1 | 0 |
| `geom-core/tests/coeffs_pair_identity.rs` | 1 | 1 | 0 |
| `geom-core/tests/certified_endpoint_census.rs` | 1 | 0 | 1: names `Real::is_poison` |
| `geom-brep/tests/review_m5_pr7_enclosure.rs` | 1 | 1 | 0 |

Totals: 142 hits, 104 re-worded, 38 kept.

The 19-versus-18 count: the sweep's own list, less the files the row
names and the three sets it excludes, is 18. The nineteenth was most
likely `geom-brep/src/ssi/certify.rs`, which the row reports reading
in full and moves to its "Not listed" paragraph.

## Closed (2026-09-29) — PR 3448 (CERT-NAMES)

Landed with the unit after a single review and its fix pass; the PR body has the classification table.
