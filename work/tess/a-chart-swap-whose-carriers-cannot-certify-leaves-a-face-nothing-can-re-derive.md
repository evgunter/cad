---
id: a-chart-swap-whose-carriers-cannot-certify-leaves-a-face-nothing-can-re-derive
kind: issue
title: a chart swap onto a surface the face's boundary carriers cannot certify against leaves the face rowless and nothing can re-derive it, which patch_memo's reweighted-NURBS row now rebuilds by hand
status: open
opened: 2026-09-14
priority: P3
cost: D
refs: [tessellate-refuses-approx-face-without-caches, 2594]
---


Filed by the fix pass of TOPO's
`set-face-surface-leaves-a-complete-face-certified-against-the-chart-it-left`
(PR 2594), on both reviewers' finding, with R2's measurement.

That unit makes `topo::Body::set_face_surface` drop a face's pcurve
rows when the new surface is not the chart they were stated in. The
door's prescription for a caller that wants rows on the new chart is
`pcurves::mint_pcurves`. **There is a class of swap where that
prescription cannot be filled**: where the face's boundary CARRIERS do
not lie on the new surface, the minting pass refuses
(`PcurveMintError::Certify`), so the face stays rowless and every
reader that needs a row — the trimmed-NURBS tessellation lane, `props`,
`chart_boundary` — has nothing to read and no door to get one from.

**Measured** (R2's probe, on this suite's own fixture):
`crates/mesh/tests/patch_memo.rs`'s
`the_trimmed_nurbs_lane_misses_when_its_surface_changes` perturbs one
weight of a loft wall's net. After the swap the wall is rowless;
`mint_pcurves` on that body refuses (`MapResidual at sample 1
definitely exceeds`); and after the row's hand-restore through
`Body::attach_pcurve` of the rows saved before the swap,
`validate_pcurves` reports **two `Certify`** on the wall's half-edges.

So the row as repaired by PR 2594 measures the memo's key on a body
carrying rows certified against a chart its face is not on — the state
the unit exists to remove — rebuilt deliberately through the row-level
door. It is honest about that in a comment, and the kernel is right to
refuse: the wall's boundary carriers are iso-curves of the ORIGINAL net
and lie off the reweighted one.

**What an honest fixture needs, and why PR 2594 did not write it.**
R2's suggestion is to re-describe the carriers in the new spline space
the way `crates/sweep/tests/common/approx.rs` does (degree elevation
plus knot refinement). That module's surgery is a REPRESENTATION change
— same locus, same parameterization — and reweighting a net is not: it
moves the surface, so the wall's boundary must move with it, which is
the OFF-D face-replacement primitive's job rather than a test helper's.
The two candidate repairs are therefore (a) perturb the net in a way
that leaves the wall's four boundary curves fixed — an INTERIOR control
point or weight, which `loft_prism`'s wall does not have: three
sections at degree 2 in `v` and a straight section edge in `u` make its
net a 2x3 grid, every point of which sits on a boundary column, so this
repair needs a corpus body with more spans than the one the row uses;
or (b) drive the whole edit through a door that replaces the face and
its edges together. Either keeps what the row is about — the memo's key reading
the surface — and neither needs a hand-attached row.

The tessellation-side sibling, a rowless face the trimmed lane cannot
run on at all, is `tessellate-refuses-approx-face-without-caches`. The
kernel-side silence that lets such a face pass tier 3 is
`work/trim/validate-pcurves-cannot-tell-a-never-minted-face-from-an-emptied-one`.
