//! PHASE-1 SCRATCH (to be rewritten as the unit's rows).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{Curve3, Surface};
use geom_brep::{EdgeCurveSpec, EdgeDescriptionSpec};
use geom_core::{Band, Point3, Tol, Vec3};
use topo::{Body, EdgeKey, FaceKey, FaceSurface, MefSite, MevSite};

fn tol() -> Tol {
    Tol::witness()
}

fn band() -> Band {
    Band::linear(tol()).unwrap()
}

struct Frame {
    origin: Point3<f64>,
    axis: Vec3<f64>,
    radius: f64,
    u_ref: Vec3<f64>,
}

impl Frame {
    fn surface(&self) -> Surface<f64> {
        Surface::Cylinder {
            origin: self.origin,
            axis: self.axis,
            radius: self.radius,
            u_ref: self.u_ref,
        }
    }
    fn at(&self, u: f64, v: f64) -> Point3<f64> {
        let w = self.axis.cross(self.u_ref);
        self.origin + (self.u_ref * u.cos() + w * u.sin()) * self.radius + self.axis * v
    }
}

fn frame() -> Frame {
    Frame {
        origin: Point3::origin(),
        axis: Vec3::unit_z(),
        radius: 1.0,
        u_ref: Vec3::unit_x(),
    }
}

/// An open cylinder-wall sheet over `[u0, u1] x [v0, v1]`: two rim
/// arcs (exact circle carriers, `Intersection` descriptions) and two
/// meridian struts, with every pcurve minted.
fn wall(u0: f64, u1: f64, v0: f64, v1: f64) -> (Body<f64>, FaceKey) {
    let f = frame();
    let mut body = Body::<f64>::new();
    let (p00, p10, p11, p01) = (f.at(u0, v0), f.at(u1, v0), f.at(u1, v1), f.at(u0, v1));
    let seed = body.mvfs(p00).unwrap();
    let cyl = body
        .set_face_surface(seed.face, FaceSurface::New(f.surface()))
        .unwrap();
    let rim = |body: &mut Body<f64>, v: f64, ccw: bool| {
        let center = f.origin + f.axis * v;
        let scaffold = body.mvfs(center).unwrap();
        let plane = body
            .set_face_surface(
                scaffold.face,
                FaceSurface::New(Surface::Plane {
                    origin: center,
                    normal: f.axis,
                    u_ref: f.u_ref,
                }),
            )
            .unwrap();
        let (carrier, t0, t1) = if ccw {
            (
                Curve3::Circle {
                    center,
                    axis: f.axis,
                    radius: f.radius,
                    u_ref: f.u_ref,
                },
                u0,
                u1,
            )
        } else {
            let w = f.axis.cross(f.u_ref);
            let s = f.u_ref * u1.cos() + w * u1.sin();
            (
                Curve3::Circle {
                    center,
                    axis: -f.axis,
                    radius: f.radius,
                    u_ref: s,
                },
                0.0,
                u1 - u0,
            )
        };
        EdgeCurveSpec {
            description: EdgeDescriptionSpec::Intersection {
                s1: cyl,
                s2: plane,
                witness: f.at((u0 + u1) * 0.5, v),
            },
            carrier,
            param_start: t0,
            param_end: t1,
        }
    };
    let bottom = rim(&mut body, v0, true);
    let e_b = body
        .mev(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            p10,
            bottom,
            tol(),
        )
        .unwrap();
    let e_r = body
        .mev_line(
            MevSite::Fan {
                he1: e_b.he_minus,
                he2: e_b.he_minus,
            },
            p11,
            tol(),
        )
        .unwrap();
    let top = rim(&mut body, v1, false);
    let e_t = body
        .mev(
            MevSite::Fan {
                he1: e_r.he_minus,
                he2: e_r.he_minus,
            },
            p01,
            top,
            tol(),
        )
        .unwrap();
    let he = body
        .find_half_edge(seed.face, e_t.vertex, e_r.vertex)
        .unwrap();
    let face = body
        .mef(
            MefSite::Chords {
                he1: he,
                he2: e_b.he_plus,
            },
            EdgeCurveSpec::line_between(p01, p00),
            FaceSurface::Shared(cyl),
            tol(),
        )
        .unwrap()
        .face;
    topo::mint_pcurves(&mut body, tol()).unwrap();
    (body, face)
}

fn rows_of(body: &Body<f64>, face: FaceKey) -> Vec<(topo::HalfEdgeKey, Option<(f64, f64)>)> {
    let f = body.get_face(face).unwrap();
    let topo::LoopBoundary::Cycle { first } = body.get_loop(f.outer).unwrap().boundary else {
        panic!("no cycle")
    };
    body.loop_cycle(first)
        .unwrap()
        .into_iter()
        .map(|he| (he, body.pcurve(he).map(topo::PcurveCache::params)))
        .collect()
}

#[test]
fn scratch_measure() {
    let (body, face) = wall(0.2, 1.4, 0.0, 1.0);
    eprintln!("[m] minted rows on the wall: {:?}", rows_of(&body, face));
    assert!(topo::pcurves::validate_pcurves(&body, band()).is_empty());
    // Split the bottom rim (a curved-chart carrier).
    for pick in ["rim", "meridian"] {
        let mut b = body.clone();
        let target: EdgeKey = b
            .edges()
            .find(|(_, d)| {
                let c = b
                    .get_curve_geom(d.curve)
                    .and_then(topo::CurveGeom::certified)
                    .unwrap();
                matches!(
                    (pick, c.carrier()),
                    ("rim", Curve3::Circle { .. }) | ("meridian", Curve3::Line { .. })
                )
            })
            .map(|(e, _)| e)
            .unwrap();
        let (t0, t1) = b
            .get_curve_geom(b.get_edge(target).unwrap().curve)
            .and_then(topo::CurveGeom::certified)
            .unwrap()
            .params();
        let created = b.split_edge(target, (t0 + t1) * 0.5, tol()).unwrap();
        eprintln!("[m] {pick}: parent params were {:?}", (t0, t1));
        eprintln!("[m] {pick}: rows after split {:?}", rows_of(&b, face));
        eprintln!(
            "[m] {pick}: parent edge curve params now {:?}",
            b.get_curve_geom(b.get_edge(target).unwrap().curve)
                .and_then(topo::CurveGeom::certified)
                .unwrap()
                .params()
        );
        eprintln!(
            "[m] {pick}: new halves {:?} {:?}",
            created.he_plus, created.he_minus
        );
        eprintln!(
            "[m] {pick}: validate_pcurves {:?}",
            topo::pcurves::validate_pcurves(&b, band())
        );
    }
}
