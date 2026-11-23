//! Review probe (PR 7294 delta): does a crossing whose sine separates the
//! arc's reach from the meeting from its diameter reach
//! `split_ring_path_spline_across`?
use super::*;
use geom_core::spline::KnotVector;

#[test]
fn probe_a_near_tangent_crossing_at_a_half_turns_middle() {
    let band = Band::new(1e-9, 1.3e-9).unwrap();
    for s in [0.60e-9_f64, 0.68e-9, 0.70e-9] {
        let circle = geom::Circle::new(
            geom::Frame::new(Point3::origin(), Vec3::unit_z(), Vec3::unit_x()),
            1.0,
        );
        let piece = Piece::Arc(CircleArc { circle, span: (0.0, core::f64::consts::PI) });
        let m = Point3::new(0.0, 1.0, 0.0);
        let d = Vec3::new(-s.cos(), 0.0, s.sin());
        let net = geom::NurbsCurve3::new(
            KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(),
            vec![m - d * 0.5, m + d * 0.5],
            vec![1.0, 1.0],
        )
        .unwrap();
        let carrier = geom::Curve3::Nurbs(std::sync::Arc::new(net));
        let arc = SplineArc::on_by_construction(&carrier, (0.0, 1.0)).unwrap();
        let cc = geom::Curve3::Circle(circle);
        let on_piece = geom_brep::curve_pair::Span::new(&cc, (0.0, core::f64::consts::PI));
        let on_arc = geom_brep::curve_pair::Span::new(&arc.carrier, arc.span);
        let met = geom_brep::curve_pair::meet(&on_piece, &on_arc, band);
        let got = spline_crossings(&piece, &arc, false, 0.0, band);
        println!("PROBE s={s:e} sqrt2*s={:e} 2s={:e} meet={met:?}", 2f64.sqrt() * s, 2.0 * s);
        println!("PROBE s={s:e} spline_crossings={got:?}");
    }
}
