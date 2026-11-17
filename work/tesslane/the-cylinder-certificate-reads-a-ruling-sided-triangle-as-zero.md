---
id: the-cylinder-certificate-reads-a-ruling-sided-triangle-as-zero
kind: issue
title: cert_cylinder and cert_sphere clamp a NaN to 0: a cylinder triangle with a side on a ruling certifies at 0 in some corner orders
status: closed
opened: 2026-11-16
priority: P1
cost: M
branch: tesslane/cylinder-cert
closed: 2026-11-17
pr: 7044
---


## Finding

`cert::cert_cylinder` is `(r − dist_line_triangle(axis, T)).max(0.0)`
(`crates/mesh/src/cert.rs`), and `f64::max` drops a NaN. One arises on a
sound triangle. A triangle with a side along a ruling projects, across the
axis, onto a segment: `dist_line_triangle` hands `dist_point_triangle` a
triangle of no area, whose interior branch divides by zero for some corner
orders and answers NaN. The certificate is then 0.

Measured: the cylinder of radius 1 about `y`, the triangle
`(1, 1, 0)`, `(1, 0, 0)`, `(cos π/6, 0, −sin π/6)` (a half-cell of a column
a twelfth of a turn wide, as the curved lane's `nu == 1` branch emits it for
`tools/tess-meter`'s `mesh5_r2probe::r2_wedge_bytes`, the frustum wedge at
δ `0.25`): `r − dist` is NaN and the certificate 0; the triangle deviates by
the chord's sagitta, `1 − cos(π/12) = 0.0341`, which its sibling half-cell
`(1, 1, 0)`, `(cos π/6, 0, −sin π/6)`, `(cos π/6, 1, −sin π/6)` certifies at.
`cert::tests::the_cylinder_and_sphere_certificates_read_a_nan_as_zero`
pins both, and a corner that is NaN, which reads 0 the same way on the
cylinder and on `cert_sphere`.

## Why it matters

An under-reporting certificate passes a mesh that is over δ. On the curved
lane a cell's two halves share the chord, so the sibling reads the cell's
deviation and the face's verdict is right. Nothing says the same of the
trimmed cylinder arm's triangles, where a triangle with one side on a
ruling need not have such a sibling. Not measured: whether any accepted
mesh holds a triangle read as 0 that is over δ.

## Not fixed where found

Found by PR 6965's fix pass, which made the two certificates answer NaN
where `r − dist` is not finite: that refused the frustum wedge above
(`CertificateExceeded` with the sibling's bound, `0.034` against δ `0.25`),
a face of the rectangle lane, which that unit may not move. The cure is the
distance, not the clamp: `dist_point_triangle` total on a triangle of no
area (the nearest of its sides), after which the clamp can poison on a NaN.
It moves cylinder certificates from 0 to their true value, so its PR owes
the sweep that shows which verdicts move.

## Unit (2026-11-16, TESSLANE)

**Letter M. Tier: dual review.** The cylinder and sphere certificates stand behind every face of those two surfaces on both curved lanes, so a change to what they answer is as broad as the mesh; the change itself is small and its hazard is in what it moves. An under-reporting certificate passes a mesh over δ silently, and a cure that over-reports refuses faces that are sound.

Branch `tesslane/cylinder-cert`, cut from `tesslane/cone-near-apex` (PR 6965, reviewed and closing): merge main after it lands. The clearance unit (`tesslane/lattice-clearance`) and unit 4 (`tesslane/route-by-domain`) are in `trimmed.rs` and `tessellate.rs`; this unit is `cert.rs` and what reads it.

**Owed by the unit:**

- **Measure before building.** Instrument the two certificates and run every mesh the `mesh` suite, `sweep`'s meshing rows, the tour and the meter build: how many triangles' distance is not finite today, on which surface, lane and corner order, and for each the triangle's true deviation (sampled on the triangle against the surface's closed form, and its sibling's certificate where it has one) against the face's δ. **Say whether any mesh `tessellate` returns today holds a triangle over δ that the certificate read as 0.** That number is the unit's first result, whatever it is, and it goes to the top of the PR body. Say how the sphere's clamp is reached, if a body reaches it.
- **The cure is the distance.** `dist_point_triangle` (and whatever `dist_line_triangle` hands it) answers the true distance on a triangle of no area, the nearest of its sides, in every corner order: the answer must not depend on the order the corners come in. Then the clamp has nothing finite to hide, and an input that is not finite is poisoned and refuses, as `cert_cone`, `cert_torus` and `cert_spindle` do (`cert::worse` keeps it). Find every other degenerate projection the two distances can be handed (a triangle seen edge-on, a point on the axis, a triangle through the sphere's centre) and say what each answers.
- **What moves, argued.** A certificate that read 0 and now reads its true value can change a verdict. For every face whose verdict or bytes move: its body, the old and new bound, its true deviation, and why the new answer is right. A face that was over δ and now refines or refuses is main having been wrong, and is reported as that. **A face whose every triangle is within δ and that now refuses is a stop**: report it, do not tune. D9 goldens, `docs/tess-budget-data`, the meter's bytes (`tools/tess-meter`'s `r2_wedge_bytes` is the body that found this) and the tour pins: name each that moves.
- **Rows.** The pin `cert::tests::the_cylinder_and_sphere_certificates_read_a_nan_as_zero` becomes the true values in every corner order; a row that the certificate of a triangle with a side on a ruling equals its sibling's; a fuzz row of the two certificates against sampled true distance over triangles built to be degenerate in projection, in the manner of `cert_cone_bounds_the_true_deviation_on_triangles_built_to_break_it`; a non-finite input poisons. Mutants: the old clamp back; the degenerate branch answering one fixed side; say what catches each.
- **Sweep the class** (implementer-discipline §5): a `.max(…)`, `.min(…)` or comparison in `mesh` that reads a NaN as a number where it decides or reports a bound; a distance or projection helper that divides by an area. One disposition per hit.
- **Done when** no certificate in `cert.rs` reads a non-finite distance as a bound; the measurement above is in the PR body; every moved face is argued; this row closes.
- **Run what CI runs before every push:** `bash scripts/run-gates.sh`; `scripts/doc-gate.sh --workspace-pass --skip-viewer-toolkit`; fmt; clippy for mesh, geom-brep, `demos/tour` and the viewer job (`cargo clippy -p viewer --features app --all-targets -- -D warnings`); `cargo nextest run -p mesh` whole and the viewer's tests; the steps of the `mesh budget meter + certificate falsifier` job (it runs `-p mesh --lib` in release) and of the `tools` job from `.github/workflows/ci.yml`; sweep's meshing rows; the rows you add or change at `CAD_TOLERANCE_EPS` 1e-6 and 1e-12 as well, each stating what it expects at each. A test costing 0.5 s or more goes in the slow set with its measured time.

## Closed (2026-11-17, PR 7044)

`cert::dist_point_triangle` answers the distance on every triangle, the same to the bit in every order of its corners, and the five certificates answer NaN on any input that is not finite (`cert::all_finite`). No verdict and no mesh byte moves.

What the sections above had wrong, from the unit's measurement and its review:

- **The sibling is no safeguard.** Both halves of a lattice cell have a side on a ruling, so both have a shadow of no area; which half read NaN was its corner order (two of the six). On the rectangle lane the triangles main read as 0 were within δ by that lane's theorem. On the chart lane's cylinder arm, which has no theorem and names the certificate as its guarantee, they were passed on no guarantee: 0.4 to 0.5 of δ in bulk on the mesh suite, with a tail to 0.652 of δ.
- **The misreading was not only a NaN.** On a cylinder whose axis is off the coordinate axes the shadow of a ruling is a sliver with a rounding-sized area, and 127 emitted triangles read a finite distance too long, a certificate up to 11 % under the true deviation. A cure at "exactly zero area" would have left them.
- **How many.** 689,547 emitted cylinder triangles over the mesh suite, sweep's meshing rows, the meter and the tour were certified at 0 or short. None is over its δ at its true value (the largest is 0.99980 δ), which is a measurement of that corpus and not a property of the lanes. No sphere triangle is misread by any body.

What the review added: the distance was checked against exact arithmetic on about 480,000 triangles (worst 0.31 of a rounding allowance on triangles with their corners on the surface). It found a direction of length 10¹⁵⁵ answering a wrong finite distance and coordinates under 10⁻⁷⁷ underflowing; the function now reads its inputs at their own power-of-two scale, with no range guard. It found the choice of sides for the plane's normal pinned by no row; that choice is gone, and a row holds what does carry the accuracy on a needle, the corners' one order. The same reviewer then re-reviewed the rewrite (approve with fixes, no MAJOR): the oracle unchanged, the fixed pair of sides held on 135,000 needles built against it, and every mesh byte-identical again. It found that no row pinned which coordinate sets the scale, which a row with coordinates 200 decades apart now holds, and that the torus and spindle certificates answered `+∞` where finite inputs overflow, which now answer NaN.

Left for others, each filed: the NURBS certificates' folds that drop a NaN chart corner (`work/chord/nurbs-cell-bounds-cert-drops-a-nan-vertex-and-lands-a-nan-end-on-the-first-cell.md`); the sizing folds that drop a NaN ahead of their own guard (`sizing-folds-drop-a-nan-before-their-own-guard`); a standing sample of the mesher's own cylinder triangles against the certificate (`work/chord/S236.md`).
