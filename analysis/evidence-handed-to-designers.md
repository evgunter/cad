# Evidence for the born-form question (ONEROUND, 2026-11-20)

Everything here was measured by the lane building `oneround/anchored-conic` (local runs; evidence, not committed numbers). Its raw lists are on branch `origin/analysis/oneround/conic-vertex-probes`, under `analysis/oneround/conic-vertex/` (`phase1-report.md`, and `phase2-one-chain/`). The build as it stands is `origin/oneround/anchored-conic` (stacked on `origin/onehome/carrier-frames`, PR 7131); the variant in which every conic is summed from its zero point is `origin/oneround/anchored-conic-one-chain`.

## What "one chain" means, and what it moved

One chain: every conic point is summed from its zero point `P0` (the vertex on `u_ref`), `P(θ) = P0 + u·a·(−2 sin²(θ/2)) + v·b·sin θ`. A conic born anchored stores `P0`. A conic born with a centre has `P0` read as `c + u_ref·a` inside the chain, a plain rounded sum.

Rows moved, by variant (ci profile):
- one chain alone, first form, geom + geom-brep + topo: 12 of 6,969;
- one chain with a cone mint, a cylinder mint and a re-fitted constant, eight crates: 29 of 10,866;
- the same with the offsets tightened and the cylinder mint out: 22;
- the chain following the born form (a centre-born conic evaluated in its centre form, bit for bit as on the base; only an anchored-born conic from its anchor): 2, both from the cone mint.

The 22, by kind:
- bit pins, 3 (for example a sphere-support reading 0.49999999999999994 where 0.5 is asserted);
- mesh bytes and digests, 6 (19 of 48 body/δ pairs in `d9_mesh_goldens`);
- a fixed point, 1: `freecad::cross_dialect_fixed_point` writes `CARTESIAN_POINT (0.0006249999999999999, 0.00025, 0.0)` against `(0.000625, 0.00025, 0.0)`, a 1-ulp two-cycle where import snaps a vertex to its carrier;
- a carried certificate no longer equal to a fresh recertification, 2 (8.040613248383182e-17 against 6.123233995736766e-17; 2.22e-16 against 1.22e-16);
- counts of queries, 2 (36 for 35, 32 for 33);
- bounds against evaluated points, 2 ("sup 4 is short of D 4.000000000000001"; a cover missing the true arc by 4e-10);
- VERDICTS, 4: a root a row expects refused is certified (`ellipse_torus`, the slack meter); a `Spread` refusal becomes a meeting (`touch_spread_rows`); a refusal changes kind (`SingleColumnCurvedFace` where `PinchWedge` is asserted); a `KernelLimit` certification refusal where a build is expected (attributed by the lane to the constant going 32 → 40, not to the chain). The first two were not checked against an oracle; the analogous pair in phase 1 were true certifications.

Why a centre-born conic's bits move: `c + r` is rounded into the zero point and `−r` does not undo it, where the centre form gives `c + r·cos(π/2) = c` to the bit. So a centre-born circle no longer reproduces its quarter-turn points.

## What each form reads on a far-centred conic (ρ = 1500 near-gap families, u·ρ = 1.67e-13)

| | meter's error bound | certified of 40 |
|---|---|---|
| centre form (main; and a centre-born conic under "the chain follows the born form") | 3.8 u·ρ | 7, 7, 7, 7, 1 |
| zero point as a plain sum (one chain, centre-born) | 0.77 u·ρ | 40 in every family |
| zero point as an exact sum | 0.004 u·ρ | 40 in every family |
| anchor stored (anchored-born) | u·\|P0\| plus u times the offset | 40 in every family |

An exact-sum zero point needs a product-with-residual door on the scalar trait (`two_product` + `two_sum` at f64; `Dual`'s value channel must match f64 bit for bit; at `Interval` it gives nothing, the plain enclosure is u·far wide). It does not give back the quarter-turn bits. Under PR 7131's reading of a stored frame as its polar factor, the exact sum of the stored entries is not the true vertex at the far scale.

Phase 1 (on main) also found that readings which compare a far-centred conic's point with something else in the box (a cone's apex, a clearance, a line, an endpoint pin, `row_interval`) read right only with the vertex exact; a plain `c + u·a` rounds at u·far like the centre form.

## Who is born how, in the build as it stands

- Born anchored: `plane_cone_section`'s ellipse, at its near vertex (in-box points within 7e-16 m of the exact-rational oracle at a = 1e3 to 1e6, against 7e-13 to 7e-10 on the base).
- Born with a centre: everything else. `plane_cylinder_section` (a fixed vertex measured up to 2× worse than the centre form on off-axis far cuts, whose in-box arc is 0.42 to 0.50 rad from any principal vertex); every circle (no circle mint is converted; `tangent_circle`'s ρ = 1500 fixtures are far-centre circles); an imported ellipse; an authored arc.
- Whether any import produces a far-centre ellipse was not surveyed.
