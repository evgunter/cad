use super::*;
use crate::joint::{Deck, JointElement};
use core::f64::consts::TAU;
use geom::Surface;
use geom_brep::Pcurve;
use geom_core::{Band, Point3, Tol, Vec3};

/// **A whole turn of a Villarceau row is one period on each channel**
/// (the reviewers' probe, PR 4227): on either family and traversal,
/// from four centres and two starts, the joint carrying the row's
/// exit onto its own entry decides
/// `Shift(Deck { u: sense, v: vl, twin: false })` — the `(±1, ±1)`
/// winding the loop invariant admits. The same holds on the row's
/// lift by `k` azimuth periods and `−k` tube periods, which is the row
/// with that deck element (`Deck::apply`), never a row of its own: as
/// a row it refuses at its azimuth's fidelity term (C4).
#[test]
fn a_whole_turn_decides_one_period_on_each_channel() {
    let band = Band::linear(Tol::witness()).unwrap();
    let (big, r) = (2.0_f64, 0.7_f64);
    let axis = Vec3::new(0.1, 0.2, 1.0).normalize();
    let u_ref = (Vec3::unit_x() - axis * axis.x).normalize();
    let center = Point3::new(0.3, -0.2, 0.5);
    let surface = Surface::Torus {
        center,
        axis,
        major_radius: big,
        minor_radius: r,
        u_ref,
    };
    let chart = DescribedChart::of(&surface).unwrap();
    let tilt = (r / big).asin();
    let mut cases = 0;
    for phi in [0.0_f64, 1.0, 2.5, -2.0] {
        let d = u_ref * phi.cos() + axis.cross(u_ref) * phi.sin();
        for family in [1.0_f64, -1.0] {
            let lean = d.cross(axis) * tilt.cos() + axis * (family * tilt.sin());
            for traversal in [1.0_f64, -1.0] {
                for psi in [0.0_f64, 2.0] {
                    let carrier = geom::Curve3::circle(
                        center + d * r,
                        d.cross(lean) * traversal,
                        big,
                        d * psi.cos() + lean * psi.sin(),
                    );
                    let (t0, t1) = (0.3, 0.3 + TAU);
                    let row = geom_brep::PcurveCache::certify(
                        geom_brep::chart_pcurve_over(&carrier, t0, t1, &surface, band).unwrap(),
                        t0,
                        t1,
                        &carrier,
                        &surface,
                        band,
                    )
                    .unwrap();
                    let Pcurve::FocalSection(focal) = *row.pcurve() else {
                        panic!("a Villarceau row is a focal section")
                    };
                    for k in [-1, 0, 2] {
                        let lift = Deck {
                            u: k,
                            v: -k,
                            twin: false,
                        };
                        let lifted = lift
                            .apply(
                                row.pcurve(),
                                &surface,
                                super::ChartPeriods::of(&surface, band),
                            )
                            .unwrap();
                        if k != 0 {
                            assert!(
                                matches!(
                                    geom_brep::PcurveCache::certify(
                                        lifted.clone(),
                                        t0,
                                        t1,
                                        &carrier,
                                        &surface,
                                        band,
                                    ),
                                    Err(geom_brep::PcurveCertifyError::ResidualExceeded {
                                        check: geom_brep::PcurveCheck::EnvelopeTerm(
                                            geom_brep::EnvelopeTerm::FidelityU
                                        ),
                                        ..
                                    })
                                ),
                                "phi {phi}, family {family}, traversal {traversal}, \
                                     psi {psi}, k {k}: the lifted copy is no row"
                            );
                        }
                        let joint = decide_plain_joint(
                            chart,
                            &lifted,
                            t0,
                            lifted.eval(t1),
                            carrier.eval(t0),
                            super::ChartPeriods::given_u(
                                Some(geom_brep::Period::new(TAU)),
                                chart.surface(),
                                band,
                            ),
                            band,
                        );
                        let periods = (focal.sense, focal.vl);
                        assert!(
                            matches!(
                                joint,
                                Ok(JointElement(Deck { u, v, twin: false }))
                                    if (f64::from(u), f64::from(v)) == periods
                            ),
                            "phi {phi}, family {family}, traversal {traversal}, psi {psi}, \
                                 k {k}: {:?}, not a shift by {periods:?} periods",
                            joint.as_ref().ok()
                        );
                        cases += 1;
                    }
                }
            }
        }
    }
    assert_eq!(cases, 96);
}
