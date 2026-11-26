---
id: main-red-the-finish-completes-merge-reds-the-spiric-reimport-and-the-seam-split-census
kind: issue
title: main red since PR 7497: spiric_roundtrip's native cavity reads Pcurve Unminted on its spiric faces, and the seam-split census escalates its pcurve envelope
status: open
opened: 2026-11-26
priority: P1
cost: E
---



## Finding

Filed by INSIDE's measurement lane `inside-attribute-reds` (2026-11-26).
Two slow-set rows are red on origin/main `649a6026b` (release, default
ε). Both were green at nightly 45553333820 (`4dc697d543`), and no row
named either test.

1. `step-import` `spiric_roundtrip::a_spiric_rim_exports_as_a_spline_and_reimports_at_its_volume`,
   at `crates/step-import/tests/spiric_roundtrip.rs:157:5`, on the
   *native* cavity before any export:

       the native cavity passes tier 3, its spiric faces measured
         left: Err([Pcurve { finding: Unminted { face: FaceKey(4v1) } }, Pcurve { finding: Unminted { face: FaceKey(6v1) } }, Pcurve { finding: Unminted { face: FaceKey(8v1) } }])
        right: Ok(())

   The raising site is `crates/topo/src/pcurves/tier3.rs:373`. The face
   stores no pcurve rows (`!stored.stores`), its loops are closed,
   `derive_face` succeeds, and `stored.gaps` is non-empty. So tier 3
   reads the face as one the finish should have minted and did not.

2. `sweep` `an_aimed_ray_from_far_reads_a_split_side_right::the_seam_split_census_reads_no_side_inside_out`,
   at `crates/sweep/tests/an_aimed_ray_from_far_reads_a_split_side_right.rs:159:13`:

       φ -2.8304347826086955 ψ 2e-7 L 0 sign 1: a side refuses on the ellipse's own rounding alone: NotFinished { findings: [Pcurve { finding: Certify { half_edge: HalfEdgeKey(31v1), error: Escalated { check: EnvelopeTerm(FidelityV), sample: 0, cause: Indeterminate { margin: MarginDiag(Value(1.8626460093257343e-9, None, ZeroWord(0))), terminal_sliver: false, band: Band { zero: 1e-9, escalate: 1e-8 }, predicate: Some("pcurve_envelope") } } } }] }

   The finish now certifies a pcurve row on that half-edge. Its
   `FidelityV` envelope term reads 1.86e-9, which lies inside the
   (1e-9, 1e-8) band.

## Bisect

Release-profile runs over origin/main's first-parent merges from
`4dc697d543`. Both tests pass at `04be3b9bb` (#7765) and fail at
`414ff4b8e`, **PR 7497 (walks/finish-completes)**, the next merge. The
payloads at `414ff4b8e` match main's byte for byte. PR 7497 rewrites
`crates/topo/src/pcurves/mint.rs`, `staleness_posture.rs` and
`splitting/mod.rs`: the finish now completes a body's pcurve rows.

Related: `work/green/main-red-a-spline-capped-boxs-chart-move-refuses-pcurve-unminted.md`
(the same `Unminted` finding on another slow-set row, filed 2026-11-25
and not bisected). It is probably the same merge, but this lane did not
run that test.

Reproduce:

    cargo nextest run --release -p step-import -p sweep --test all -E 'test(a_spiric_rim_exports_as_a_spline_and_reimports_at_its_volume) | test(the_seam_split_census_reads_no_side_inside_out)'

## What a fix is

Read each raising site against PR 7497's change. Either the finish
mints rows for the spiric faces it now leaves gapped, or the rows'
expectation is re-read and re-pinned with the reason. Do the same for
the seam-split side's escalated envelope term. P1 because a red slow
test is a red nightly.
