//! **The planar lane on `cross`** (GERM U4c): a line's and a conic's
//! crossings of a plane face are `cross`'s answer read through the face's
//! trim, as the curved lane's are (`reduce::span_verdict`).
//!
//! `A`'s direction of the sweep runs directly
//! (`planar_lane_carrier_rows::sweep_a`), so each row reads where the
//! sweep put a vertex of `A` on the face (`on_face`), or which refusal
//! it raised. The oracles are computed from the stored data in their own
//! arithmetic (a double-double [`Dd`]), never by the kernel's pose.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use super::planar_lane_carrier_rows::{on_face, sweep_a};
use crate::Body;
use crate::test_support_fixtures::prism_z;
use geom_core::{Point3, Tol, Vec3};

/// `a + b` and its rounding error, exactly (Knuth's two-sum).
fn two_sum(a: f64, b: f64) -> (f64, f64) {
    let s = a + b;
    let bb = s - a;
    (s, (a - (s - bb)) + (b - bb))
}

/// A double-double: `hi + lo`, about 106 bits.
#[derive(Clone, Copy)]
struct Dd(f64, f64);

impl Dd {
    fn of(x: f64) -> Self {
        Self(x, 0.0)
    }

    fn add(self, o: Self) -> Self {
        let (s, e) = two_sum(self.0, o.0);
        let (hi, lo) = two_sum(s, e + self.1 + o.1);
        Self(hi, lo)
    }

    fn sub(self, o: Self) -> Self {
        self.add(Self(-o.0, -o.1))
    }

    fn mul(self, o: Self) -> Self {
        let p = self.0 * o.0;
        let e = self.0.mul_add(o.0, -p) + self.0 * o.1 + self.1 * o.0;
        let (hi, lo) = two_sum(p, e);
        Self(hi, lo)
    }

    fn div(self, o: Self) -> Self {
        let q = self.0 / o.0;
        let r = self.sub(o.mul(Self::of(q)));
        let (hi, lo) = two_sum(q, r.0 / o.0);
        Self(hi, lo)
    }

    /// `a · b` over three components.
    fn dot(a: [Self; 3], b: [Self; 3]) -> Self {
        a[0].mul(b[0]).add(a[1].mul(b[1])).add(a[2].mul(b[2]))
    }
}

/// A point's or a vector's components, as double-doubles.
fn dd3(x: f64, y: f64, z: f64) -> [Dd; 3] {
    [Dd::of(x), Dd::of(y), Dd::of(z)]
}

/// The run's band: the sweep's own (`sweep_a`).
fn band() -> geom_core::Band {
    geom_core::Band::linear(Tol::witness()).unwrap()
}

/// **A far line crossing a plane at a small tilt is split where the plane
/// crosses its carrier**, not where its vertex points' offsets
/// interpolate (U4c: the line formula goes).
///
/// A brick's face lies in the vertical plane through `C = (X, ½X, 0)`,
/// `X = 10⁶` m (at ε `10⁻¹²`, `10³` m: a frame as far as that band builds)
/// along `û = (cos 0.6, sin 0.6, 0)`; a triangular prism's long edges run
/// through `C` along `û` turned `10⁻⁶` rad, from `C − 100ê` to `C + 100ê`,
/// at `z = ±½`. Its ends stand `∓10⁻⁴` m off the plane, past the band at
/// every ε, so they straddle it. The `C − 100ê` vertex points are moved
/// half the zero band off their carrier, towards the face's normal: an
/// edge whose vertex points stand within the band of its carrier, as a
/// built edge's may. The oracle is the stored carrier's crossing of the
/// stored plane, `s = ((o − O)·n̂)/(d̂·n̂)` along the carrier from its
/// origin `O`, in double-double. The sweep's split lies on the plane
/// within the zero band, and along the edge within the crossing's own
/// conditioning of the oracle: at a tilt `θ` an offset read with rounding
/// `δ` moves the crossing `δ/θ` along the edge, and the rounding of an
/// offset at the edge's length `L` is a few ulps of `L`, so the bound is
/// `64·ε_f64·L/θ`, about `3·10⁻⁶` m. No reading of the crossing in `f64`
/// holds it to the zero band along the edge at this tilt, the old
/// interpolation's included: its offsets round at the same scale.
///
/// Red at every ε where the crossing is interpolated between the vertex
/// points' offsets from the face's plane (`t₀ + (t₁ − t₀)·d₁/(d₁ − d₂)`,
/// the planar lane's old line formula): the half band at the moved end
/// moves the interpolated crossing about `½ζ/10⁻⁶` along the edge,
/// `2.5·10⁻⁴` m at the default ε, past the bound.
#[test]
fn a_far_line_crossing_a_plane_at_a_small_tilt_is_split_where_its_carrier_crosses() {
    let zero = band().zero();
    let a = 0.6f64;
    let u = Vec3::new(a.cos(), a.sin(), 0.0);
    // The face's outward normal: `û` turned a quarter clockwise.
    let n_out = Vec3::new(a.sin(), -a.cos(), 0.0);
    // `10⁶` m out where the band holds a frame built there, and at a
    // tighter band as far as its coordinates' spacing stands to the band as
    // it does at the default ε (`10³` m at ε `10⁻¹²`).
    let far = 1e6 * (zero / 1e-9).min(1.0);
    let c = Point3::new(far, 0.5 * far, 0.0);
    let e = Vec3::new((a + 1e-6).cos(), (a + 1e-6).sin(), 0.0);
    let xy = |p: Point3<f64>| (p.x, p.y);
    // The brick: the face `Q₀ → Q₁` first, counter-clockwise.
    let (q0, q1) = (c - u * 0.5, c + u * 0.5);
    let b = prism_z::<f64>(
        &[xy(q0), xy(q1), xy(q1 - n_out), xy(q0 - n_out)],
        -1.0,
        1.0,
        Tol::witness(),
    );
    let face = b.side_faces[0];
    // The prism: its long edge `P₁ → P₀` first, counter-clockwise.
    let (p0, p1, p2) = (c - e * 100.0, c + e * 100.0, c + n_out * 3.0);
    let mut prism = prism_z::<f64>(&[xy(p1), xy(p0), xy(p2)], -0.5, 0.5, Tol::witness());
    for v in [prism.bottom[1], prism.top[1]] {
        let at = prism.body.get_vertex(v).expect("the vertex").point;
        let p = prism.body.points[at];
        prism.body.points[at] = p + n_out * (0.5 * zero);
    }
    let a_body = prism.body;
    // The oracle, from the stored carrier and the stored plane.
    let geom::Surface::Plane { frame } = *b.body.proven_face(face).expect_described() else {
        panic!("the brick's side face is a plane");
    };
    let (o, n) = (frame.origin(), frame.axis());
    let crossing = |line: geom::Line<f64>| {
        let dir = line.dir.get();
        let (oo, d) = (line.origin, dd3(dir.x, dir.y, dir.z));
        let rel = [
            Dd::of(o.x).sub(Dd::of(oo.x)),
            Dd::of(o.y).sub(Dd::of(oo.y)),
            Dd::of(o.z).sub(Dd::of(oo.z)),
        ];
        let nn = dd3(n.x, n.y, n.z);
        let s = Dd::dot(rel, nn).div(Dd::dot(d, nn));
        let at = |k: usize, x: f64| Dd::of(x).add(d[k].mul(s)).0;
        Point3::new(at(0, oo.x), at(1, oo.y), at(2, oo.z))
    };
    let want: Vec<Point3<f64>> = a_body
        .edges()
        .filter_map(|(k, _)| {
            let curve = a_body.proven_edge(k).certified()?;
            let line = curve.carrier().as_line()?;
            // The two long edges: along `ê`.
            (line.dir.get().dot(e).abs() > 1.0 - 1e-9).then(|| crossing(line))
        })
        .collect();
    assert_eq!(want.len(), 2, "the prism's two long edges");
    let got = on_face(sweep_a(&a_body, &b.body), face);
    assert_eq!(got.len(), 2, "one split on each long edge: {got:?}");
    let off_plane = |p: Point3<f64>| {
        let rel = [
            Dd::of(p.x).sub(Dd::of(o.x)),
            Dd::of(p.y).sub(Dd::of(o.y)),
            Dd::of(p.z).sub(Dd::of(o.z)),
        ];
        Dd::dot(rel, dd3(n.x, n.y, n.z)).0.abs()
    };
    let along = 64.0 * f64::EPSILON * 200.0 / 1e-6;
    for w in &want {
        let near = got
            .iter()
            .map(|g| (*g - *w).norm())
            .fold(f64::INFINITY, f64::min);
        assert!(
            near <= along,
            "the split stands within the crossing's conditioning ({along:e}) of the carrier's \
             crossing {w:?}: {near:e} off, {got:?}"
        );
    }
    for g in &got {
        assert!(
            off_plane(*g) <= zero,
            "the split lies on the face's plane within the zero band ({zero:e}): {:e} off",
            off_plane(*g)
        );
    }
}

/// **A conic grazing a plane face inside its trim is split at the touch
/// point**, as the graze net did (U4 §2.3: the planar lane's touch
/// treatment). The unit cylinder wall's sheet over `u ∈ [−1, 1]`,
/// `v ∈ [0, 2]` has its bottom rim on the unit circle at `z = 0`, which
/// touches the brick's face `x = 1` at `(1, 0, 0)` and nowhere else: the
/// plane is the circle's tangent there. The brick's face holds that
/// point (`y ∈ [−½, ½]`, `z ∈ [−½, ½]`), and the top rim at `z = 2`
/// touches the plane outside it. `cross` answers the bottom rim's touch,
/// the face reading finds its arc on the face, and the planar lane places
/// the touch point and splits there: one vertex of the sheet on the face,
/// at `(1, 0, 0)`, the circle's own tangent point.
///
/// Red where the planar touch takes the curved face's typed frontier
/// (`FaceMeeting::TouchOnFace` read as `SpanVerdict::Unsettled` on a
/// plane too): the sweep refuses the pair.
#[test]
fn a_conic_grazing_a_plane_face_inside_its_trim_is_split_at_the_touch_point() {
    let mut sheet = Body::<f64>::new();
    crate::test_support_fixtures::cyl_wall_sheet(
        &mut sheet,
        crate::test_support_fixtures::CylFrame::canonical(1.0),
        (-1.0, 1.0),
        (0.0, 2.0),
        Tol::witness(),
    );
    let b: Body<f64> =
        crate::test_support_fixtures::brick((1.0, 2.0), (-0.5, 0.5), (-0.5, 0.5), Tol::witness());
    let face = b
        .faces()
        .map(|(k, _)| k)
        .find(|&f| {
            crate::face_normal::face_plane(&b, f)
                .is_some_and(|p| p.normal.dot(Vec3::unit_x()) < -0.5)
        })
        .expect("the brick's face x = 1");
    let got = on_face(sweep_a(&sheet, &b), face);
    let touch = Point3::new(1.0, 0.0, 0.0);
    assert!(
        got.len() == 1 && (got[0] - touch).norm() <= band().zero(),
        "one vertex of the sheet on the face, at the touch point {touch:?}: {got:?}"
    );
}

/// **An end read on the face owns the line's meeting there: a root of the
/// carrier just inside the span is that end's, never a split beside it**
/// (U4 §2.3, "The ends"). A brick's face lies in `x = 1`. A triangular
/// prism's edge runs from `V`, half the zero band short of the plane
/// (`x = 1 − ½ζ`, so its side reads `Zero`), into the brick at a tilt of
/// `10⁻³` rad over a metre. The carrier itself crosses `x = 1` at `½ζ/10⁻³`
/// along the edge from `V`, past the band at every ε, where `cross` would
/// place an inside root. The sweep records `V` (and its twin on the top
/// rim) on the face, and cuts no vertex beside it.
///
/// Red where a line with an end read `Zero` is still read through `cross`
/// and split at its inside root: a sliver `½ζ/10⁻³` long, and two more
/// vertices on the face.
#[test]
fn an_end_read_on_the_face_owns_a_root_of_its_carrier_just_inside_the_span() {
    let zero = band().zero();
    let v = (1.0 - 0.5 * zero, 0.0);
    let theta = 1e-3f64;
    let p = (v.0 + theta.sin(), v.1 + theta.cos());
    let r = (v.0 - 1.0, v.1 + 0.5);
    // Counter-clockwise: `V → P → R` turns left.
    let prism = prism_z::<f64>(&[v, p, r], -0.25, 0.25, Tol::witness());
    let ends: Vec<Point3<f64>> = [prism.bottom[0], prism.top[0]]
        .into_iter()
        .map(|w| {
            prism
                .body
                .vertex_points()
                .find_map(|(k, q)| (k == w).then_some(q))
                .expect("the vertex")
        })
        .collect();
    let b: Body<f64> =
        crate::test_support_fixtures::brick((1.0, 2.0), (-0.5, 0.5), (-0.5, 0.5), Tol::witness());
    let face = b
        .faces()
        .map(|(k, _)| k)
        .find(|&f| {
            crate::face_normal::face_plane(&b, f)
                .is_some_and(|q| q.normal.dot(Vec3::unit_x()) < -0.5)
        })
        .expect("the brick's face x = 1");
    let mut got = on_face(sweep_a(&prism.body, &b), face);
    got.sort_by(|a, b| a.z.total_cmp(&b.z));
    got.dedup_by(|a, b| (*a - *b).norm() == 0.0);
    assert!(
        got.len() == 2 && got.iter().zip(&ends).all(|(g, e)| (*g - *e).norm() == 0.0),
        "the face holds V and its twin, and no vertex cut beside them: {got:?}, V at {ends:?}"
    );
}

/// **Two roots of a conic within the band of each other, where the face
/// holds them, refuse** (C5: a window the face may hold refuses the face
/// reading), where the conic root lane inserted them once. An ellipse of
/// semi-axes `10` (along `x`) and `ζ/10` crosses the plane
/// `x = 10·cos ½` at `θ = ±½`: decided roots of a definite belly, whose
/// chord, `2·(ζ/10)·sin ½`, is inside the zero band, so the placement
/// cannot part them (`place`'s `Together`). A brick's face in that plane
/// holds both points. The planar lane's reading refuses, at its typed
/// frontier (`plane_refused`).
///
/// Red where the two roots are inserted as one meeting: the lane splits at
/// it.
#[test]
fn two_roots_within_the_band_of_each_other_on_the_face_refuse() {
    use super::super::BooleanError;
    let zero = band().zero();
    let x0 = 10.0 * 0.5f64.cos();
    let ellipse = geom::Curve3::Ellipse(geom::Ellipse::new(
        geom::Frame::new(Point3::origin(), Vec3::unit_z(), Vec3::unit_x()),
        10.0,
        0.1 * zero,
    ));
    let b: Body<f64> = crate::test_support_fixtures::brick(
        (x0, x0 + 1.0),
        (-0.5, 0.5),
        (-0.5, 0.5),
        Tol::witness(),
    );
    let face = b
        .faces()
        .map(|(k, _)| k)
        .find(|&f| {
            crate::face_normal::face_plane(&b, f)
                .is_some_and(|q| q.normal.dot(Vec3::unit_x()) < -0.5)
        })
        .expect("the brick's face x = x0");
    let surface = b.proven_face(face).expect_described().clone();
    let span = geom_brep::curve_pair::Span::new(&ellipse, (-1.0, 1.0));
    let answer = geom_brep::curve_face::cross(&span, &surface, band());
    let got = super::span_verdict(
        (&b, super::super::Operand::B, face),
        &surface,
        &span,
        answer,
        band(),
        (&super::door_escalates, &|why| {
            super::plane_refused(
                why,
                |diag| BooleanError::Escalated {
                    decision: super::super::BooleanDecision::Coincidence(
                        super::super::Coincide::EdgeOnPlane,
                        super::super::DeclarationRead::Moot,
                    ),
                    diag,
                },
                || BooleanError::TangencyUnsupported,
            )
        }),
    );
    assert!(
        matches!(got, Err(BooleanError::TangencyUnsupported)),
        "two roots the band cannot part, on the face, refuse at the frontier: {got:?}"
    );
}
