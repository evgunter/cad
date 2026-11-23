---
id: a-one-band-balls-seam-certificate-is-not-the-fresh-runs
kind: issue
title: A one-band ball's seam meridian carries a certificate a fresh re-certification does not reproduce
status: open
opened: 2026-11-23
priority: P3
cost: E
refs: [a-full-revolves-curved-wall-carries-two-seam-meridians-nobody-wrote]
---


Found by the LATHE one-site lane (2026-11-23), measured on its branch
`lathe/one-site-wrap-edges` (stacked on `lathe/full-revolve-one-band`).

## What

The bare one-band ball `revolved_about_y([(0,-1),1.0),((0,1),0.0)], Full)`
(`sweep`'s snowman `ball(1, 0)`): re-certify its seam meridian (edge
`1v1`) against its own ends and surfaces through
`sweep/tests/common/certificates.rs` `assert_certificates_fresh` at
`Tol::witness()`: it stores `Certificate { samples: 9, max_residual:
1.5700924586837752e-16 }`, a fresh run mints `1.2246467991473532e-16`.
So `snowman::the_fallback_assembly_carries_the_kept_operands_certificates`
fails its "slab ∪ ball" row (`fresh: true`) once the lens it builds first
no longer refuses.

Same shape as
`work/cleavetail/a-boolean-result-carries-a-seam-meridian-certificate-a-fresh-run-does-not-reproduce.md`,
now on a revolve's own seam rather than a boolean's.

## Ground

`crates/sweep/src/revolve/full.rs`, the one-band build's meridian mint,
or the test's `fresh` flag for the ball if the build's certificate is
the right one to keep.
