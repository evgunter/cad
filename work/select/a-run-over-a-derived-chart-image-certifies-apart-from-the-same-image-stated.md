---
id: a-run-over-a-derived-chart-image-certifies-apart-from-the-same-image-stated
kind: issue
title: An edge certified over a chart image its restatement derives stores a certificate a run over the same image stated does not reproduce
status: open
opened: 2026-11-23
priority: P3
cost: M
refs: [a-one-band-balls-seam-certificate-is-not-the-fresh-runs, a-boolean-result-carries-a-seam-meridian-certificate-a-fresh-run-does-not-reproduce]
---

Found by the LATHE one-band lane (2026-11-23, PR 7414) and its review.

## What

`geom_brep::EdgeCurve::certify` over a description whose chart image
it DERIVES (a `wrap` or `chart` restatement with no image) stores a
`Certificate { samples, max_residual }` that a run over the same image,
STATED (`recertify`, or a restatement from the stored edge's
`restated_spec()`), does not reproduce: same carrier, interval, ends
and surface, and the residuals differ in the last bits. Measured:

- a full revolve's ball seam meridian: `1.5700924586837752e-16` stored,
  `1.2246467991473532e-16` fresh
  (`work/lathe/a-one-band-balls-seam-certificate-is-not-the-fresh-runs.md`);
- the one-segment full-turn torus's latitude wrap edge
  (`sweep::revolve::turn`), measured by the PR's review: `8.0e-16` stored,
  `6.1e-16` fresh.

Each consumer that compares a carried certificate with a fresh run
(`sweep/tests/common/certificates.rs` `assert_certificates_fresh`) sees
a body whose certificate is not the one re-certification mints.

## Where it is worked around

`sweep::swept::describe_wrap_edge` restates the wrap edge once more from
what it stores, so every sweep wrap edge (a full revolve's meridian, a
one-segment loop's strut, extrude's and revolve's) stores the fresh
run's certificate (`one_segment_loop::every_wrap_edge_stores_the_certificate_a_fresh_run_mints`).
Other restatements that derive an image and keep the derived run's
certificate, not checked against a fresh run:

- `crates/sweep/src/extrude.rs` (the strut and smooth-join
  restatements, ~1253 and ~1533, past `describe_wrap_edge`);
- `crates/step-import/src/normalize/adopt.rs`;
- `crates/sweep/src/blend/surgery.rs` (the band's `wrap` restatement,
  ~6236);
- `crates/topo/src/boolean/chord_join.rs`;
- `crates/topo/src/attach.rs` (the `chart` restatements, ~2504, ~2641,
  ~3759).

`work/cleavetail/a-boolean-result-carries-a-seam-meridian-certificate-a-fresh-run-does-not-reproduce.md`
may be the same class at a boolean's output.

## Wanted

The cause in `geom-brep`'s certify: why the derived image's run and the
stated image's run sample to different residuals (a derived image read
at the band versus the stored one, or a different sample schedule), and
one answer for every door, so the workaround above can go.
