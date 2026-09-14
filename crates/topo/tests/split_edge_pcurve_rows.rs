//! **`split_edge` carries its parent half-edges' pcurve rows across the
//! split.** The forcing fixture is a minted cylinder-wall sheet: a
//! curved chart, so the face stores rows, and both a CIRCLE rim and a
//! LINE meridian to split.
//!
//! The claim under every row is one sentence: after the op, every
//! half-edge of every face it touched carries the row it should, those
//! rows are the ones `mint_pcurves` derives, and a body that stored no
//! rows still stores none.

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

/// Every half-edge of `face`'s outer loop, with the parameter interval
/// of its stored row (`None` where no row is stored).
fn rows_of(body: &Body<f64>, face: FaceKey) -> Vec<(topo::HalfEdgeKey, Option<(f64, f64)>)> {
    let f = body.get_face(face).unwrap();
    let topo::LoopBoundary::Cycle { first } = body.get_loop(f.outer).unwrap().boundary else {
        panic!("the wall face lost its cycle")
    };
    body.loop_cycle(first)
        .unwrap()
        .into_iter()
        .map(|he| (he, body.pcurve(he).map(topo::PcurveCache::params)))
        .collect()
}

/// The edge of `body` whose carrier is a circle (`rim`) or a line
/// (`meridian`), with its certified parameter interval.
fn pick(body: &Body<f64>, which: &str) -> (EdgeKey, (f64, f64)) {
    let edge = body
        .edges()
        .find(|(_, d)| {
            let c = body
                .get_curve_geom(d.curve)
                .and_then(topo::CurveGeom::certified)
                .unwrap();
            matches!(
                (which, c.carrier()),
                ("rim", Curve3::Circle { .. }) | ("meridian", Curve3::Line { .. })
            )
        })
        .map(|(e, _)| e)
        .unwrap();
    let params = body
        .get_curve_geom(body.get_edge(edge).unwrap().curve)
        .and_then(topo::CurveGeom::certified)
        .unwrap()
        .params();
    (edge, params)
}

/// A whole-body textual snapshot: every arena, every field a reader
/// can see through the public API, and every stored row with BOTH its
/// image and its certificate. What the mint-identity row compares — a
/// re-derived (rather than restricted) image, a shifted branch, a
/// widened interval or a certificate assembled from a different window
/// all move a line of it.
fn deep(body: &Body<f64>) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for (k, v) in body.vertices() {
        out.push(format!("V {k:?} {:?} {:?}", v.point, v.emanating));
    }
    for (k, e) in body.edges() {
        out.push(format!(
            "E {k:?} {:?} {:?} {:?}",
            e.he_plus, e.he_minus, e.curve
        ));
    }
    for (k, h) in body.half_edges() {
        out.push(format!(
            "H {k:?} start={:?} edge={:?} loop={:?} next={:?} prev={:?}",
            h.start, h.edge, h.parent_loop, h.next, h.prev
        ));
    }
    for (k, l) in body.loops() {
        out.push(format!("L {k:?} face={:?} b={:?}", l.face, l.boundary));
    }
    for (k, f) in body.faces() {
        out.push(format!(
            "F {k:?} {:?} {:?} {:?}",
            f.outer, f.rings, f.surface
        ));
    }
    for (k, s) in body.shells() {
        out.push(format!("S {k:?} {s:?}"));
    }
    for (k, s) in body.solids() {
        out.push(format!("O {k:?} {s:?}"));
    }
    for (k, pt) in body.points() {
        out.push(format!("P {k:?} {pt:?}"));
    }
    for (k, c) in body.curves() {
        out.push(format!("C {k:?} {c:?}"));
    }
    for (k, u) in body.surfaces() {
        out.push(format!("U {k:?} {u:?}"));
    }
    for (k, c) in body.pcurves() {
        out.push(format!(
            "R {k:?} {:?} {:?} {:?}",
            c.params(),
            c.pcurve(),
            c.certificate()
        ));
    }
    out.sort();
    out
}

/// **The unit's row.** Splitting a rim (circle carrier) or a meridian
/// (line carrier) of a minted cylinder wall leaves the tier-3 pcurve
/// pass with nothing to say: the parent halves carry the first child's
/// interval, the new halves the second child's, and no half-edge is
/// left without a row.
///
/// At this unit's merge base both new halves were rowless — the pass
/// read two `MissingCache` findings — and each parent half's row still
/// claimed the whole parent interval while its edge had been narrowed
/// to the first child's.
#[test]
fn a_split_on_a_curved_chart_leaves_every_half_edge_a_row() {
    for which in ["rim", "meridian"] {
        let (mut body, face) = wall(0.2, 1.4, 0.0, 1.0);
        let (edge, (t0, t1)) = pick(&body, which);
        let t = (t0 + t1) * 0.5;
        let created = body.split_edge(edge, t, tol()).unwrap();

        assert_eq!(
            topo::pcurves::validate_pcurves(&body, band()),
            vec![],
            "{which}: tier 3's pcurve pass after the split"
        );

        let parent = body.get_edge(edge).unwrap();
        for he in [parent.he_plus, parent.he_minus] {
            assert_eq!(
                body.pcurve(he).map(topo::PcurveCache::params),
                Some((t0, t)),
                "{which}: parent half {he:?} carries the first child's interval"
            );
        }
        for he in [created.he_plus, created.he_minus] {
            assert_eq!(
                body.pcurve(he).map(topo::PcurveCache::params),
                Some((t, t1)),
                "{which}: new half {he:?} carries the second child's interval"
            );
        }
        assert!(
            rows_of(&body, face).iter().all(|(_, p)| p.is_some()),
            "{which}: a half-edge of the wall face is left rowless: {:?}",
            rows_of(&body, face)
        );
    }
}

/// **The rows the op carries are the rows the minting pass derives,
/// BYTE FOR BYTE** — every split site, both carrier kinds: run
/// `mint_pcurves` over the split body and no line of the whole-body
/// snapshot moves, image, interval, certificate or arena. That is what
/// makes the carry a RESTRICTION of the parent's certified image
/// rather than a second answer beside it.
///
/// **The claim is about an ANALYTIC chart** — this cylinder, and the
/// sphere and torus that certify through the same closed-form door.
/// On a SPLINE chart the carry is exact too, but the mint pass refuses
/// on the split body rather than agreeing with it, so there is no
/// identity to state: `sweep`'s `split_edge_loft_charts` carries that
/// frontier and the row that pins it.
///
/// At this unit's merge base the op left 8 rows where the pass then
/// derived 10, and the parent halves' rows still read the parent's
/// whole interval against the pass's narrowed one.
#[test]
fn the_carried_rows_are_the_mint_passs_rows_byte_for_byte() {
    for which in ["rim", "meridian"] {
        for frac in [0.001_f64, 0.02, 0.5, 0.98, 0.999] {
            let (mut body, _) = wall(0.2, 1.4, 0.0, 1.0);
            let (edge, (t0, t1)) = pick(&body, which);
            body.split_edge(edge, t0 + (t1 - t0) * frac, tol()).unwrap();
            let carried = deep(&body);
            topo::mint_pcurves(&mut body, tol()).unwrap();
            let after = deep(&body);
            let moved: Vec<_> = carried
                .iter()
                .zip(after.iter())
                .filter(|(a, b)| a != b)
                .collect();
            assert_eq!(
                carried.len(),
                after.len(),
                "{which} @{frac}: the mint pass changed the row/arena count"
            );
            assert!(
                moved.is_empty(),
                "{which} @{frac}: the mint pass moved {} snapshot line(s), first {:?}",
                moved.len(),
                moved.first()
            );
        }
    }
}

/// **Absence is never a claim.** A body that never ran the minting pass
/// stores no rows, and the op mints none: it carries what is there, it
/// does not start caching a body whose producer chose not to.
#[test]
fn a_body_with_no_rows_still_has_none_after_a_split() {
    let (body, _) = wall(0.2, 1.4, 0.0, 1.0);
    let mut bare = body.clone();
    let keys: Vec<_> = bare.pcurves().map(|(he, _)| he).collect();
    for he in keys {
        bare.detach_pcurve(he);
    }
    assert_eq!(bare.pcurves().count(), 0);
    let (edge, (t0, t1)) = pick(&bare, "rim");
    bare.split_edge(edge, (t0 + t1) * 0.5, tol()).unwrap();
    assert_eq!(bare.pcurves().count(), 0);
    assert_eq!(topo::pcurves::validate_pcurves(&bare, band()), vec![]);
}
