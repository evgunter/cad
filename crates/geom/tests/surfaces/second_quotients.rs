//! **The surface's second-order quotients are `S_uu`, `S_uv`, `S_vv`
//! themselves, pointwise** (`geom_core::spline::cell_second_quotients`,
//! module docs "The second order"): each piece's numerator and weight
//! power, evaluated as tensor Bernstein sums at the piece's local
//! parameters, reproduce `NurbsSurface::ders` through a knot span's
//! axes.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

test_utils::gated_to!["crates/geom/src/surfaces/", "crates/geom-core/src/spline/",];

use geom::NurbsSurface;
use geom_core::Point3;
use geom_core::spline::{CellAxis, CellBlock, KnotVector, QuotientPiece, cell_second_quotients};

/// The Bernstein value of `x` at `s`, by de Casteljau.
fn bernstein(x: &[f64], s: f64) -> f64 {
    let mut row = x.to_vec();
    for level in 1..row.len() {
        for k in 0..row.len() - level {
            row[k] = row[k] * (1.0 - s) + row[k + 1] * s;
        }
    }
    row[0]
}

/// The tensor Bernstein value of a row-major `(nu × nv)` grid at
/// `(s, t)`.
fn tensor(x: &[f64], nv: usize, (s, t): (f64, f64)) -> f64 {
    let along_v: Vec<f64> = x.chunks_exact(nv).map(|r| bernstein(r, t)).collect();
    bernstein(&along_v, s)
}

/// `q`'s quotient at `(s, t)`, coordinate `c`, over a numerator of
/// `nv` coefficients per row.
fn read(q: &QuotientPiece<f64, 3>, nv: usize, st: (f64, f64), c: usize) -> f64 {
    let m: Vec<f64> = q.numer().iter().map(|a| a[c]).collect();
    tensor(&m, nv, st) / tensor(q.denom(), nv, st)
}

/// **The span axis reproduces the jet.** A rational bicubic-by-
/// biquadratic Bézier patch with weights varying in both directions,
/// cut into `2 × 2` pieces, read at five parameters per direction
/// against `ders` to `1e-8` of each partial's scale.
#[test]
fn the_second_quotients_are_the_partials_pointwise() {
    let (p, q) = (3usize, 2usize);
    let ku = KnotVector::clamped([vec![0.0; p + 1], vec![1.0; p + 1]].concat(), p).unwrap();
    let kv = KnotVector::clamped([vec![0.0; q + 1], vec![1.0; q + 1]].concat(), q).unwrap();
    let mut control = Vec::new();
    let mut weights = Vec::new();
    for i in 0..=p {
        for j in 0..=q {
            #[allow(clippy::cast_precision_loss)]
            let (x, y) = (i as f64, j as f64);
            control.push(Point3::new(
                x + 0.3 * y,
                y - 0.2 * x * x,
                0.4 * x * y - 0.1 * y * y,
            ));
            weights.push(1.0 + 0.7 * ((i * 3 + j * 5) % 4) as f64 / 3.0);
        }
    }
    let s = NurbsSurface::new(ku.clone(), kv.clone(), control.clone(), weights.clone()).unwrap();
    let points: Vec<[f64; 3]> = control.iter().map(|c| [c.x, c.y, c.z]).collect();
    let (hu, hv) = (ku.knots().to_vec(), kv.knots().to_vec());
    let (axis_u, axis_v) = (
        CellAxis::Span(ku.span(ku.first_span()).unwrap(), &hu),
        CellAxis::Span(kv.span(kv.first_span()).unwrap(), &hv),
    );
    let ((cu, cv), pieces) = cell_second_quotients(
        (axis_u, axis_v),
        CellBlock::Points {
            points: &points,
            weights: &weights,
            origin: points[0],
        },
        (2, 2),
    )
    .unwrap();
    assert_eq!((cu, cv), (2, 2), "the pieces per axis");
    let nv = 3 * q + 1;
    for u in [0.0, 0.13, 0.5, 0.77, 0.9999] {
        for v in [0.0, 0.13, 0.5, 0.77, 0.9999] {
            #[allow(clippy::cast_precision_loss)]
            let at = |x: f64, n: usize| {
                #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                let k = ((x * n as f64).floor() as usize).min(n - 1);
                (k, x * n as f64 - k as f64)
            };
            let ((iu, su), (iv, sv)) = (at(u, cu), at(v, cv));
            let (_, second) = &pieces[iu * cv + iv];
            let jet = s.ders(u, v);
            for (name, piece, truth) in [
                ("S_uu", &second.uu, jet.duu),
                ("S_uv", &second.uv, jet.duv),
                ("S_vv", &second.vv, jet.dvv),
            ] {
                for (c, want) in [truth.x, truth.y, truth.z].into_iter().enumerate() {
                    let got = read(piece, nv, (su, sv), c);
                    assert!(
                        (got - want).abs() <= 1e-8 * (1.0 + want.abs()),
                        "{name}[{c}] at ({u}, {v}): the pieces read {got}, ders {want}"
                    );
                }
            }
        }
    }
}
