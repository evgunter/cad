---
id: tessellate-refuses-approx-face-without-caches
kind: issue
title: mesh::tessellate refuses an Approx-faced body whose half-edges carry no stored pcurve cache
status: closed
opened: 2026-09-04
refs: [1758]
priority: P0
cost: D
closed: 2026-09-22
---


Found by a SHELL-2 reviewer's end-to-end consumer (2026-09-04): an
`Approx`-capped box built through the public doors (storage door,
`set_face_surface`) tessellates with "NURBS-face half-edge carries no
stored pcurve cache" — the same wall `mass_properties` reports. The
cache cannot be minted for the cap's straight-carrier edges (the
iso-line seam class refuses a non-spline carrier), so today the only
`Approx`-faced body that meshes is the loft, whose walls were minted
with their caches. Recorded so the mesh lane knows the class exists;
whether the fix is a cache mint for straight carriers on an `Approx`
chart or a tessellation arm that reads the description directly is
the mesh program's call.

Home: `crates/mesh` (S-MESH).

## Re-homed at S-MESH's exit (2026-09-16)

Moved from `work/mesh/` to TESS (opened at this exit as S-MESH's successor for the tessellation kernel) when S-MESH closed (`docs/S-MESH-EXIT-WALK.md`); the item's content, id and history are unchanged.

## Closed on measurement (2026-09-22) — the wall is gone

Survey lane `tess/approx-face-survey` at `d423d1b46` (probe
`crates/sweep/tests/tess_approx_survey_probe.rs`). The `Approx`-capped
box built through the public doors (`common::approx::box_with_approx_
cap`) refuses exactly as filed WITHOUT `topo::mint_pcurves` — and that
helper deliberately skips the mint. WITH it, all four cap half-edges
carry `Pcurve::IsoLine` caches, the two seam-class ones over
`Curve3::Line` carriers included, and `tessellate` = `Ok` (6 patches),
`mass_properties` = 4.000…, tier-3 check 7 = `Ok`, at six (d, ε) rows.
What closed it: the LINE-carrier limb of `PcurveCache::certify`'s seam
class (EXCH's PR 1798, merged 2026-09-17; this row was filed 09-04).
The residual `IsoUnsupported { "a seam-class iso line over a
non-spline carrier" }` stands only below that limb (conics, rational
columns, a LINE seam on a non-boundary column).

The fork this row named — a cache mint versus a mesh-side arm — needs
no choosing: the mint exists, and the mesh-side arm would have moved
the refusal from typed to silent (`topo::pcurve_of` is uncertified;
the edge description's residual is bounded only at its certification
samples, not at chord parameters).

What survives, each its own row: the fixture that pins the minted
body (`work/tint/approx-capped-box-fixture-runs-the-mint-and-pins-
the-minted-body.md`); the refusal's five spellings, wrong noun and
inconsistent producer lists (`work/tess/missing-pcurve-cache-refusal-
has-five-spellings-and-a-wrong-noun.md`); the LINE limb's partial- and
rational-column corners, unmeasured because no public door builds a
curved `Approx` chart with a straight carrier (evidence appended to
PCERT's slate). SHELL's `no-approx-faced-body-is-both-movable-and-
valid` legs 2–3 lift on this measurement; told on that row.
