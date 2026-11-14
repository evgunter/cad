use super::turn_element;
use crate::construction::{MevSite, NewVertexSide};
use crate::joint::{Deck, JointElement};
use crate::test_support_fixtures::{CylFrame, cyl_wall_sheet};
use crate::{Body, LoopBoundary};
use geom::Surface;
use geom_core::{Band, Tol};

/// **A turn on a sub-band lever decides the lift its gap meets, in
/// both orientations.** A null strut on a minted cylinder wall,
/// described as the wall's own circle once round, is a closed
/// carrier: its turn is a period, `−τ` read along `he_plus` and `+τ`
/// along `he_minus`. On a cylinder whose radius `r` keeps the gap's
/// nearest mark, `π·r` off, inside the band's escalate edge (under
/// `escalate/π`, about `3.2ε` at this band), the marks read Zero or
/// in band, and the near-pole read ([`super::lever_periods`]) takes
/// the orbit point the gap meets within ε at that lever: the image
/// one period over (`u = ∓1`), or under `ε/τ`, where every orbit
/// point it tries is within ε, the image itself (`u = 0`). Past
/// `escalate/π` the marks decide `u = ∓1` themselves, the control. Each is a lift of the one
/// point, so the turn decides rather than missing as a
/// discontinuity. Through the doors no body reaches either, since
/// every joint at that vertex was decided at the same lever and
/// margin when its rows were minted, so the row shrinks the
/// carrier's surface under the stored rows.
#[test]
fn a_sub_band_turn_decides_the_lift_its_gap_meets() {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    let mut body = Body::<f64>::new();
    let frame = CylFrame::canonical(1.0);
    let face = cyl_wall_sheet(&mut body, frame, (0.2, 1.4), (0.0, 1.0), tol);
    let outer = body.get_face(face).unwrap().outer;
    let LoopBoundary::Cycle { first } = body.get_loop(outer).unwrap().boundary else {
        panic!("the wall is bounded by a cycle")
    };
    let v = body.get_half_edge(first).unwrap().start;
    let p = *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
    let null = body
        .mev_null(
            MevSite::Fan {
                he1: first,
                he2: first,
            },
            NewVertexSide::Above,
        )
        .unwrap();
    let t0 = p.y.atan2(p.x);
    let circle = geom::Curve3::circle(
        frame.origin + frame.axis * p.z,
        frame.axis,
        frame.radius,
        frame.u_ref,
    );
    let spec =
        geom_brep::EdgeCurveSpec::arc_of_circle(circle, t0, t0 + core::f64::consts::TAU).unwrap();
    body.set_edge_curve(null.edge, spec, tol).unwrap();
    let key = body.get_face(face).unwrap().surface;
    let he_minus = body.get_edge(null.edge).unwrap().he_minus;
    let at = |he, r: f64| {
        let mut shrunk = body.clone();
        let Some(Surface::Cylinder { radius, .. }) = shrunk.surfaces.get_mut(key) else {
            panic!("the wall is a cylinder")
        };
        *radius = r;
        turn_element(&shrunk, he, band)
    };
    let lift = |u| {
        Some(JointElement(Deck {
            u,
            v: 0,
            twin: false,
        }))
    };
    let read = |out: Result<Option<JointElement>, Option<geom_core::Indeterminate>>| {
        out.map_err(|cause| cause.is_some())
    };
    assert_eq!(
        read(at(null.he_plus, frame.radius)),
        Ok(lift(-1)),
        "the control: at the wall's own radius the turn is a period"
    );
    let eps = band.zero();
    for (he, turn, side) in [(null.he_plus, -1, "he_plus"), (he_minus, 1, "he_minus")] {
        for r in [0.01, 0.1, 0.15] {
            assert_eq!(
                read(at(he, r * eps)),
                Ok(lift(0)),
                "{side}, r = {r}·ε: under ε/τ every orbit point is within ε, the image itself"
            );
        }
        for r in [0.17, 0.2, 0.25, 0.3, 0.5, 1.0, 3.0] {
            assert_eq!(
                read(at(he, r * eps)),
                Ok(lift(turn)),
                "{side}, r = {r}·ε: the near-pole read takes the image a period over"
            );
        }
        assert_eq!(
            read(at(he, 10.0 * eps)),
            Ok(lift(turn)),
            "{side}, r = 10·ε: the control, where the marks decide the period"
        );
    }
}
