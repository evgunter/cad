//! **Review rows, R2 of PR 4704: a quarter round meeting a slab along its
//! rim, read against an analytic oracle.** Each pose builds a radius-1
//! quarter round (the one `an_arc_tangent_at_a_boolean_vertex` builds:
//! bottom `z = 1` on `x ∈ [0.5, 2]`, wall centred `(2, ·, 2)` leaving the
//! rim `x = 2, z = 1` along `+x`, `y ∈ [0, 4]`), optionally mirrored in
//! `x ↦ 4 − x` and turned by `β` about the rim line, against a box slab.
//! Every op's result is checked three ways that share nothing with the
//! Boolean: its volume against the closed form, its tier-3 validity, and
//! `point_in_solid` at sample points against the operands' analytic
//! membership.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::approx::band;
use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use sweep::test_support::finished;
use sweep::{ExtrudeSide, Extrusion, extrude};
use topo::{AtRestBody, BooleanResult, SolidContainment, mass_properties};

/// The profile is laid in `(u, v) ↦ (x = v, z = u)` before the turn: the
/// plane's `u` axis is `R ẑ`, its `v` axis `R x̂`, `R` the turn by `beta`
/// about `+y` through the rim line, so `(u, v) = (1, 2)` is the rim.
fn frame(beta: f64) -> (Vec3<f64>, Vec3<f64>) {
    let (s, c) = beta.sin_cos();
    (Vec3::new(s, 0.0, c), Vec3::new(c, 0.0, -s))
}

fn extrude_in(
    vertices: Vec<(Point2<f64>, f64)>,
    cu: Vec3<f64>,
    cv: Vec3<f64>,
    origin: Vec3<f64>,
    y0: f64,
    depth: f64,
) -> AtRestBody<f64> {
    let plane = SketchPlane::from_frame(geom_core::test_support::rigid(
        &Affine3::from_parts(
            geom_core::Mat3::from_cols(cu, cv, Vec3::unit_y()),
            origin + Vec3::new(0.0, y0, 0.0),
        ),
        geom_core::Tol::witness(),
    ));
    let profile = Profile::new(plane, vec![bulge_loop(vertices)])
        .validate(Tol::witness())
        .unwrap();
    let body = extrude(
        &profile,
        Extrusion::Distance {
            depth,
            side: ExtrudeSide::Along,
        },
        Tol::witness(),
    )
    .unwrap()
    .body;
    finished("review 4704 operand", body, Tol::witness())
}

/// The quarter round, mirrored in `x ↦ 4 − x` when `mirror`, turned by
/// `beta` about the rim line (`β > 0` lifts its bottom off `z = 1`).
fn round(beta: f64, mirror: bool) -> AtRestBody<f64> {
    let b90 = (core::f64::consts::PI / 8.0).tan();
    let (cu, cv) = frame(beta);
    let rim = Vec3::new(2.0, 0.0, 1.0);
    let origin = rim - cu - cv * 2.0;
    let verts = if mirror {
        vec![
            (Point2::new(3.0, 3.5), 0.0),
            (Point2::new(3.0, 1.0), 0.0),
            (Point2::new(2.0, 1.0), -b90),
            (Point2::new(1.0, 2.0), 0.0),
            (Point2::new(1.0, 3.5), 0.0),
        ]
    } else {
        vec![
            (Point2::new(1.0, 0.5), 0.0),
            (Point2::new(1.0, 2.0), -b90),
            (Point2::new(2.0, 3.0), 0.0),
            (Point2::new(3.0, 3.0), 0.0),
            (Point2::new(3.0, 0.5), 0.0),
        ]
    };
    extrude_in(verts, cu, cv, origin, 0.0, 4.0)
}

fn round_has(p: Point3<f64>, beta: f64, mirror: bool) -> bool {
    let (cu, cv) = frame(beta);
    let q = Vec3::new(p.x - 2.0, p.y, p.z - 1.0);
    let (mut x, z) = (2.0 + q.dot(cv), 1.0 + q.dot(cu));
    if mirror {
        x = 4.0 - x;
    }
    (0.0..=4.0).contains(&p.y)
        && (1.0..=3.0).contains(&z)
        && (0.5..=3.0).contains(&x)
        && (x <= 2.0 || z >= 2.0 || (x - 2.0).powi(2) + (z - 2.0).powi(2) <= 1.0)
}

#[derive(Clone, Copy, Debug)]
struct Slab {
    x: (f64, f64),
    z: (f64, f64),
    y: (f64, f64),
}

impl Slab {
    fn body(self) -> AtRestBody<f64> {
        let ((x0, x1), (z0, z1)) = (self.x, self.z);
        extrude_in(
            vec![
                (Point2::new(z0, x0), 0.0),
                (Point2::new(z1, x0), 0.0),
                (Point2::new(z1, x1), 0.0),
                (Point2::new(z0, x1), 0.0),
            ],
            Vec3::unit_z(),
            Vec3::unit_x(),
            Vec3::new(0.0, 0.0, 0.0),
            self.y.0,
            self.y.1 - self.y.0,
        )
    }
    fn has(self, p: Point3<f64>) -> bool {
        (self.x.0..=self.x.1).contains(&p.x)
            && (self.z.0..=self.z.1).contains(&p.z)
            && (self.y.0..=self.y.1).contains(&p.y)
    }
}

#[derive(Clone, Copy, Debug)]
enum Op {
    Union,
    SlabMinus,
    RoundMinus,
    Meet,
}

/// Deterministic sample points over the poses' common box, kept only
/// where every operand's membership is the same a step `1e-4` away in
/// each axis (off every boundary).
fn samples(slab: Slab, beta: f64, mirror: bool) -> Vec<(Point3<f64>, bool, bool)> {
    let mut s: u64 = 0x9e37_79b9_7f4a_7c15;
    let mut next = || {
        s ^= s << 13;
        s ^= s >> 7;
        s ^= s << 17;
        (s >> 11) as f64 / (1u64 << 53) as f64
    };
    let mut out = Vec::new();
    while out.len() < 120 {
        let p = Point3::new(
            -0.5 + 5.0 * next(),
            -1.5 + 7.0 * next(),
            -0.5 + 4.5 * next(),
        );
        let a = slab.has(p);
        let b = round_has(p, beta, mirror);
        let d = 1e-4;
        let steady = [
            Vec3::new(d, 0.0, 0.0),
            Vec3::new(-d, 0.0, 0.0),
            Vec3::new(0.0, d, 0.0),
            Vec3::new(0.0, -d, 0.0),
            Vec3::new(0.0, 0.0, d),
            Vec3::new(0.0, 0.0, -d),
        ]
        .iter()
        .all(|&o| slab.has(p + o) == a && round_has(p + o, beta, mirror) == b);
        if steady {
            out.push((p, a, b));
        }
    }
    out
}

/// One op's outcome as a line: `OK`, or what disagreed with the oracle.
fn judge(
    op: Op,
    r: &Result<BooleanResult<f64>, topo::BooleanError>,
    want_vol: f64,
    pts: &[(Point3<f64>, bool, bool)],
) -> String {
    let want_in = |a: bool, b: bool| match op {
        Op::Union => a || b,
        Op::SlabMinus => a && !b,
        Op::RoundMinus => b && !a,
        Op::Meet => a && b,
    };
    match r {
        Err(e) => format!("REFUSED {e:?}"),
        Ok(BooleanResult::Empty) => {
            if want_vol > 1e-6 {
                format!("WRONG Empty, want volume {want_vol}")
            } else {
                "OK Empty".into()
            }
        }
        Ok(BooleanResult::Body(bb)) => {
            let mut bad = Vec::new();
            let v = mass_properties(
                &bb.body,
                Tol::witness(),
                topo::test_support::pinned_resolution(Tol::witness()),
            )
            .unwrap()
            .met()
            .expect("the pinned resolution is met")
            .volume;
            if (v - want_vol).abs() > 1e-9_f64.max(1e-9 * want_vol) {
                bad.push(format!("volume {v} vs {want_vol}"));
            }
            if let Err(e) = topo::validate_geometric(&bb.body, Tol::witness()) {
                bad.push(format!("tier3 {e:?}"));
            }
            let mut miss = 0;
            for &(p, a, b) in pts {
                let got = topo::point_in_solid(&bb.body, p, band(), Tol::witness());
                let ok = matches!(
                    (&got, want_in(a, b)),
                    (Ok(SolidContainment::In), true) | (Ok(SolidContainment::Out), false)
                );
                if !ok {
                    miss += 1;
                    if miss <= 2 {
                        bad.push(format!("pis {p:?}: {got:?} want in={}", want_in(a, b)));
                    }
                }
            }
            if miss > 0 {
                bad.push(format!("{miss} of {} samples disagree", pts.len()));
            }
            let shells = bb.body.shells().count();
            if bad.is_empty() {
                format!("OK body v={v:.12} shells={shells} kind={:?}", bb.kind)
            } else {
                format!(
                    "WRONG shells={shells} kind={:?}: {}",
                    bb.kind,
                    bad.join("; ")
                )
            }
        }
    }
}

/// All four ops of one pose, judged; returns the lines.
fn pose(slab: Slab, beta: f64, mirror: bool, meet: f64) -> Vec<String> {
    let s = slab.body();
    let r = round(beta, mirror);
    let vol = |b: &AtRestBody<f64>| {
        mass_properties(
            b,
            Tol::witness(),
            topo::test_support::pinned_resolution(Tol::witness()),
        )
        .unwrap()
        .met()
        .expect("the pinned resolution is met")
        .volume
    };
    let (vs, vr) = (vol(&s), vol(&r));
    let pts = samples(slab, beta, mirror);
    [
        (
            Op::Union,
            topo::union(&s, &r, Tol::witness()),
            vs + vr - meet,
        ),
        (
            Op::SlabMinus,
            topo::subtract(&s, &r, Tol::witness()),
            vs - meet,
        ),
        (
            Op::RoundMinus,
            topo::subtract(&r, &s, Tol::witness()),
            vr - meet,
        ),
        (Op::Meet, topo::intersect(&s, &r, Tol::witness()), meet),
    ]
    .into_iter()
    .map(|(op, got, want)| format!("{op:?}: {}", judge(op, &got, want, &pts)))
    .collect()
}

const FLUSH: Slab = Slab {
    x: (0.0, 2.0),
    z: (0.0, 1.0),
    y: (-1.0, 5.0),
};
const CORNER: Slab = Slab {
    x: (0.0, 2.0),
    z: (0.0, 1.0),
    y: (0.0, 4.0),
};

/// The poses, `(name, slab, β, mirror, closed-form meet volume)`.
fn poses() -> Vec<(String, Slab, f64, bool, f64)> {
    let mut v = vec![
        // The PR's postures (opposite-side tangent pair, half-turn wedge).
        ("flush".to_string(), FLUSH, 0.0, false, 0.0),
        ("corner".into(), CORNER, 0.0, false, 0.0),
        // Their mirror image: the round's bottom on a slab to its +x.
        (
            "mirror-flush".into(),
            Slab {
                x: (2.0, 5.0),
                ..FLUSH
            },
            0.0,
            true,
            0.0,
        ),
        (
            "mirror-corner".into(),
            Slab {
                x: (2.0, 5.0),
                ..CORNER
            },
            0.0,
            true,
            0.0,
        ),
        // A slab ABOVE z = 1 sharing the round's bottom plane with the same
        // orientation: the half-turn's lumped flank reads In for ∩ and ∪.
        (
            "inner-flush".into(),
            Slab {
                z: (1.0, 4.0),
                ..FLUSH
            },
            0.0,
            false,
            12.0,
        ),
        (
            "inner-corner".into(),
            Slab {
                z: (1.0, 4.0),
                ..CORNER
            },
            0.0,
            false,
            12.0,
        ),
        // Same-side tangent pair: the mirrored round's wall rides back over
        // the flush slab's top; its bottom overhangs the slab's edge. They
        // touch along the rim line only.
        ("sameside-flush".into(), FLUSH, 0.0, true, 0.0),
        ("sameside-corner".into(), CORNER, 0.0, true, 0.0),
    ];
    for beta in [1e-3f64, 1e-7, 1e-11] {
        for sign in [1.0, -1.0] {
            let b = sign * beta;
            // β < 0 dips the bottom into the flush slab: a wedge 4.5 tan|β|.
            let meet = if b < 0.0 { 4.5 * b.abs().tan() } else { 0.0 };
            v.push((format!("tilt β={b:e} flush"), FLUSH, b, false, meet));
        }
    }
    v
}

/// **Every pose, every op, printed against the oracle** (R2 of PR 4704).
/// Run:
/// `cargo test -p sweep --test all review_4704_every_pose -- --ignored --nocapture`
#[test]
#[ignore = "measurement: prints each pose's outcome; run with --ignored --nocapture"]
fn review_4704_every_pose() {
    let only = std::env::var("R2_POSE").ok();
    for (name, slab, beta, mirror, meet) in poses() {
        if only.as_deref().is_some_and(|o| !name.contains(o)) {
            continue;
        }
        for line in pose(slab, beta, mirror, meet) {
            eprintln!("R2-4704 {name} | {line}");
        }
    }
}

/// **The rim's half-turn postures answer every op against the oracle, at
/// any ε** (R2 of PR 4704). `flush` and `corner` are the PR's own;
/// `mirror-*` are their mirror image, which reads the round's flanks in
/// the other order around the rim; `inner-*` put the slab above `z = 1`,
/// sharing the round's bottom plane with the same orientation, so the
/// half-turn's lumped flank reads In for ∪ and ∩ where the PR's poses
/// read it In for ∪ alone; `sameside-*` put the slab on the round's own
/// side of the rim line, `sameside-flush` running past the round's ends.
#[test]
fn the_rims_half_turn_postures_answer_against_the_oracle() {
    let mut bad = Vec::new();
    for (name, slab, beta, mirror, meet) in poses() {
        if !(name.contains("flush") || name.contains("corner")) || name.starts_with("tilt") {
            continue;
        }
        for line in pose(slab, beta, mirror, meet) {
            if !line.contains(": OK ") {
                bad.push(format!("{name} | {line}"));
            }
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

/// **A same-side tangent touch answers every op** (`sameside-corner`:
/// the mirrored round's wall rides back over the slab's top, its bottom
/// overhanging the slab's edge, the two touching along the rim line
/// `x = 2, z = 1`, `y ∈ [0, 4]` only). Each op agrees with the oracle;
/// ∖ returns each operand, ∩ is empty, and ∪ is the two solids with the
/// edge–edge record of the slab's top edge and the round's rim edge.
#[test]
fn a_same_side_tangent_touch_answers_every_op() {
    for line in pose(CORNER, 0.0, true, 0.0) {
        assert!(line.contains(": OK "), "sameside-corner | {line}");
    }
    let inside = [Point3::new(1.0, 2.0, 0.5), Point3::new(3.0, 2.0, 2.5)];
    assert!(
        CORNER.has(inside[0]) && !round_has(inside[0], 0.0, true),
        "the slab's sample point"
    );
    assert!(
        round_has(inside[1], 0.0, true) && !CORNER.has(inside[1]),
        "the round's sample point"
    );
    crate::common::touch_union::assert_touch_at_every_op(
        "sameside-corner",
        &CORNER.body(),
        &round(0.0, true),
        [Point3::new(2.0, 0.0, 1.0), Point3::new(2.0, 4.0, 1.0)],
        inside,
    );
}
