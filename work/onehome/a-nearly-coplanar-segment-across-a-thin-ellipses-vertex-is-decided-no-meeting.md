---
id: a-nearly-coplanar-segment-across-a-thin-ellipses-vertex-is-decided-no-meeting
kind: issue
title: A nearly coplanar segment across a thin ellipse's vertex, its branches within the zero band, is decided no meeting
status: open
opened: 2026-11-19
priority: P1
cost: M
---


Filed by ONEHOME during fork 275 step 3c (PR 7137), from its delta
review's probes (O1). **It is red on main**, so it is not 3c's; the
priority is soundness, a wrong decided answer.

**The answer.** A segment nearly coplanar with a thin ellipse, crossing
it near its major vertex, where the ellipse's two branches lie within
the zero band of each other, is answered with a decided **no meeting**
(`Ok(Meetings { points: [] })`) where the true distance is below the
zero band. The full review found both operand orders answering so on
main; on PR 7137's head the segment-first order answers so (the row
below panics on it first).

**The path** (from the review's trace): the pose routes the pair as
coplanar (`Route::Level`): `curve_pair_pose_tilt` and
`curve_pair_pose_offset` read Zero, then `curve_pair_pose_root_gap` and
`curve_pair_pose_touch_spread` read Zero; then the ends read
`curve_pair_span_end_side` and `curve_pair_span_foot_gap`, and the
answer is empty. Where in that chain the meeting is lost is the
investigation: a touch the coplanar subdivision places by its in-band
stretch (`pose`'s module docs, "A touch in the subdivision") on one
branch while the segment meets the other branch, within the zero of the
first, is the likely shape.

**Not the (P1) item.** It does not overlap
`the-pose-basin-reads-the-second-carriers-reach-as-an-arc-where-only-a-chord-bound-is-proved`:
that is the across-planes closest pair's basin (`Polish::closest`); this
is the coplanar route, which reads no window bound.

**Repro** at the band `(10⁻⁹, 1.05·10⁻⁹)`, the ellipse at the origin in
`z = 0` with `u = x̂`, semi-axes `(a, b)`, its span `span_b`; the segment
through `(a cos tp, b sin tp, h)` along `normalize(cos ψ, sin ψ, h/len)`
over `span_a`:

| case | `a` | `b` | `span_b` | `tp` | `h` | `ψ` | `len` | `span_a` | true distance |
|---|---|---|---|---|---|---|---|---|---|
| 1410 | `1.2371058136272136e-4` | `1.2717406232803228e-8` | `(-0.3647375452541261, 0.11383261046515519)` | `0.025188254926288012` | `6.568256938995103e-11` | `4.330477594017474` | `1.7188559396661998e-6` | `(-1.0064985655082134e-6, 6.843763647487737e-7)` | `6.57e-11` |
| 1021 | `3.253973551187656e-4` | `3.44536105796992e-8` | `(-0.7356462272814125, 0.5285247516525128)` | `0.009417137441319778` | `2.9450535891984885e-10` | `2.232695454957603` | `2.7689621417904413e-4` | `(-0.00011522557916231402, 0.00042197410794256643)` | `2.95e-10` |

The failing row, for `crates/geom-brep/src/curve_pair/meet_rows.rs`
(not in the suite: it is red; it goes in with the fix). Checked red on
PR 7137's head:

```rust
#[test]
fn o1_a_nearly_coplanar_segment_across_a_thin_ellipses_vertex() {
    let band = Band::new(1e-9, 1.05e-9).unwrap();
    for (a, b, span_b, tp, h, psi, len, span_a) in [
        (
            1.237_105_813_627_213_6e-4_f64,
            1.271_740_623_280_322_8e-8,
            (-0.364_737_545_254_126_1, 0.113_832_612_465_155_19),
            0.025_188_254_926_288_012_f64,
            6.568_256_938_995_103e-11,
            4.330_477_594_017_474_f64,
            1.718_855_939_666_199_8e-6,
            (-1.006_498_565_508_213_4e-6, 6.843_763_647_487_737e-7),
        ),
        (
            3.253_973_551_187_656e-4,
            3.445_361_057_969_92e-8,
            (-0.735_646_227_281_412_5, 0.528_524_751_652_512_8),
            0.009_417_137_441_319_778,
            2.945_053_589_198_488_5e-10,
            2.232_695_454_957_603,
            2.768_962_141_790_441_3e-4,
            (-0.000_115_225_579_162_314_02, 0.000_421_974_107_942_566_43),
        ),
    ] {
        let thin = ellipse([0.0; 3], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0], a, b);
        let through = [a * tp.cos(), b * tp.sin(), h];
        let dir = [psi.cos(), psi.sin(), h / len];
        let seg = line(through, dir);
        let (sa, sb) = (Span::new(&seg, span_a), Span::new(&thin, span_b));
        for (order, got) in [("segment first", meet(&sa, &sb, band)), ("ellipse first", meet(&sb, &sa, band))] {
            assert!(
                !matches!(&got, Ok(m) if m.points.is_empty()),
                "a = {a:e}, {order}: no meeting read: {got:?}"
            );
        }
    }
}
```

**Scheduled** beside step 3d, after PR 7137.
