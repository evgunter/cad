INTERIM, PR 7526 delta (a79bf304d5). No MAJOR so far; leaning APPROVE-WITH-FIXES.

Held so far:
- `mesh` whole at ε 1e-9: 562/562.
- Correction-off differential: digest-identical over 59,575 patches; only `two_holes_…` goes red.
- My two-hole class probe, about 3,300 cells (three holes, staggered heights, a hole beside a notch, phases 9.0/9.25/9.5, gap 1.0–3.0, h/r 0.1–20, r 1 and 1e3, δ to 1e-4, ε 1e-9 and 1e-12): no refusal; the identity, `check_mesh` and `geometry` hold on every cell. With the correction off, 12 cells refuse. The correction converges in one round everywhere.

New MINOR findings:
1. DEMONSTRATED. "No rows" is no longer a fact of `lattice_rows`, despite what the PR body and the docs say. `tessellate_trimmed` now bypasses `lattice_rows` on a cylinder by kind. A mutant that lays rows in `lattice_rows`' cylinder arm reddens only the `chart_table` unit test; no lane row goes red. The same mutant in `trimmed.rs` reddens rowless rows 1–3, `d9_mesh_goldens` and `route_rows`.
2. BY-INSPECTION. `common::rowless::two_holes` maps a `mesh::tessellate` refusal of its one-hole probe pipe to `Err`. So off the default ε a mesher refusal prints SKIPPED instead of going red. Under a metric mutant that probe pipe does refuse (CertificateExceeded 0.0166 against δ 1e-3).
3. Metric table partly reproduced. With the old rows' metric, the δ 1e-3 cells refuse even on the one-hole probe, but the δ 1e-2 two-hole cells mesh with 60–139 points instead of refusing. So "180 of 180 refuse" does not hold on my cell set.

Still running: `mesh` at ε 1e-6 and 1e-12, then merged main (probe, digest, viewer pins) and the `sweep` differential.
