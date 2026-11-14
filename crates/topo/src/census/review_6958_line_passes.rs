//! **Reviewer probe: what moved at `f64` when a ray's span keeps the
//! direction it is given** ([`line_passes`]). The span was a segment
//! read off its two ends ([`segment`]), its direction the normalised
//! chord; it is now the caller's direction as given. Over random unit
//! directions the two closest reads ([`ee_lines_closest`]) are compared:
//! how far each enclosure end moves, in units of the reading's scale,
//! and whether any sign a ray decides off them (the gap against the
//! band, the advance's sign, the edge's reaches) differs.

#![allow(clippy::expect_used, clippy::panic)]

use super::*;

fn given<T: Real>(from: Point3<T>, dir: Vec3<T>, top: T) -> EdgeGeo<T> {
    EdgeGeo {
        key: EdgeKey::default(),
        v0: VertexKey::default(),
        v1: VertexKey::default(),
        p0: from,
        p1: from + dir * top,
        points: [None; 2],
        dir,
        len: top,
        f_plus: FaceKey::default(),
        f_minus: FaceKey::default(),
    }
}

#[test]
fn a_span_given_its_direction_reads_as_the_span_off_its_ends_to_rounding() {
    let mut rng = test_utils::fuzz::start("review_6958_line_passes");
    let replay = test_utils::fuzz::replay();
    let tol = geom_core::Tol::witness();
    let band = Band::linear(tol).expect("the witness band");
    let n = test_utils::fuzz::scaled(20_000);
    let point = |rng: &mut test_utils::fuzz::Rng, r: f64| {
        Point3::new(rng.range(-r, r), rng.range(-r, r), rng.range(-r, r))
    };
    let (mut worst, mut bits_moved, mut signs_moved) = (0.0_f64, 0_usize, 0_usize);
    let mut near = 0_usize;
    for k in 0..n {
        let q = point(&mut rng, 3.0);
        let d = (point(&mut rng, 1.0) - Point3::origin()).normalize();
        let top = rng.range(0.5, 8.0);
        let (a, b) = (point(&mut rng, 2.0), point(&mut rng, 2.0));
        // A third of the edges pass through a point of the ray, so the
        // gap is rounding's and the band decisions are exercised.
        let (a, b) = if k % 3 == 0 {
            let on = q + d * rng.range(0.0, top);
            let off = b - a;
            (on - off * rng.unit(), on + off * rng.unit())
        } else {
            (a, b)
        };
        let e = segment((a, b));
        let old = ee_lines_closest(&segment((q, q + d * top)), &e);
        let new = ee_lines_closest(&given(q, d, top), &e);
        let scale = 10.0;
        let pairs = [
            (old.gap, new.gap),
            (old.reach[0][0], new.reach[0][0]),
            (old.reach[1][0], new.reach[1][0]),
            (old.reach[1][1], new.reach[1][1]),
        ];
        let sine = d.cross(e.dir).norm();
        if sine < 1e-3 {
            continue;
        }
        for (o, w) in pairs {
            for i in 0..2 {
                if o[i].to_bits() != w[i].to_bits() {
                    bits_moved += 1;
                }
                worst = worst.max((o[i] - w[i]).abs() * sine / scale);
            }
        }
        let signs = |c: &LinesClosest<f64>| {
            let [lo, hi] = c.reach[0][0];
            (
                geom_core::k_stats::decide_magnitude(
                    "review_6958_gap",
                    geom_core::Margin::of(c.gap[0]),
                    band,
                )
                .ok(),
                geom_core::k_stats::decide(
                    "review_6958_along",
                    geom_core::Margin::of(geom::mid_param(lo, hi)),
                    band,
                )
                .ok(),
                c.reach[1].map(|[lo, hi]| {
                    (
                        geom_core::k_stats::decide(
                            "review_6958_span",
                            geom_core::Margin::of(lo),
                            band,
                        )
                        .ok(),
                        geom_core::k_stats::decide(
                            "review_6958_span",
                            geom_core::Margin::of(hi),
                            band,
                        )
                        .ok(),
                    )
                }),
            )
        };
        let (so, sn) = (signs(&old), signs(&new));
        if so.0 != Some(geom_core::k_stats::Magnitude::Positive) {
            near += 1;
        }
        if so != sn {
            signs_moved += 1;
            println!("[review 6958] signs moved: {so:?} -> {sn:?} ({replay})");
        }
    }
    println!(
        "[review 6958] line_passes at f64, ε {:e}: {n} draws, {near} in band of the edge's \
         line; enclosure ends whose bits moved: {bits_moved}; worst move·sine/scale: \
         {worst:e}; draws whose decided signs moved: {signs_moved}",
        tol.eps()
    );
    assert!(worst < 1e-13, "a move past rounding: {worst:e} ({replay})");
}
