# PCURVE-P2-SPEC.md

PCURVE P-2, the interior-column `Intersection` carriers (#498, #1177), read by TRIM as its charter input.

Deleted 2026-10-01 at TRIM's close, superseded by the three TRIM specs that delivered its residue: TRIM-1 (the de Boor collapse extractor, #2095), TRIM-2 (the trimmed-region quadrature and tessellation, #2564, #2863) and TRIM-3 (the chart-boundary description and the clearance seam, #1911, #2554), each ledgered at its own merge.
Recover with `git show 2310b98bb:docs/PCURVE-P2-SPEC.md`.

Its "producers already exist" note said `edge_nurbs.rs` derives the chart image and "then THROWS IT AWAY"; that was false by 2026-09-11 (the image is the certificate's input, and `chart_image` is the one shared producer it asked for) — `work/trim/pcurve-p2-spec-says-edge-nurbs-throws-the-image-away.md`, closed with this deletion.
