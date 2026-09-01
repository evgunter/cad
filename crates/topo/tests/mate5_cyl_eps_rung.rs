//! **MATE-5 acceptance — issue 943's cylinder residue: the certified
//! everywhere-within-ε overlap enclosure for declared cylinder pairs**
//! (`docs/MATE-5-SPEC.md`; the sanctioned closing shape of
//! `docs/CENSUS-REST-CLOSURE-DESIGN.md` Q2 + latitude note 2).
//!
//! These rows are about the PREDICATE (the census-consumption rows
//! live with the patch certifier's own in-src rows): they call
//! [`topo::declared_pair_overlap`] directly with Door 1's verdict in
//! hand, on pairs of independently authored cylinder-wall sheets whose
//! descriptions genuinely diverge — different `u_ref`, different
//! origin station, opposed axis directions — so `same_chart` refuses
//! and the cylinder enclosure arm is the only authority in play.
//!
//! **ε posture.** As the #1063 suite (`census_g2_carrier.rs`): the
//! SEAT rows are exact at every ε the matrix runs (both descriptions
//! place the shared carrier from the same `f64` literals, so the
//! carrier residues are bit-zeros); the CARRIER-GATE rows ride the
//! band ON PURPOSE and build their disagreements ε-RELATIVE from
//! [`Tol::witness`]`.eps()`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use geom::{Curve3, Surface};
use geom_brep::{EdgeCurveSpec, EdgeDescriptionSpec};
use geom_core::{Band, Point3, Tol, Vec3};
use topo::{
    Body, ChartOverlap, ChartRegionError, ContactVerdict, FaceKey, FaceSurface, MefSite, MevSite,
    declared_pair_overlap,
};

fn band() -> Band {
    let tol = Tol::witness();
    Band::new(tol.eps(), tol.k() * tol.eps()).unwrap()
}

/// One cylinder description: the frame a sheet is authored in.
#[derive(Clone, Copy)]
struct CylFrame {
    origin: Point3<f64>,
    axis: Vec3<f64>,
    radius: f64,
    u_ref: Vec3<f64>,
}

impl CylFrame {
    fn surface(&self) -> Surface<f64> {
        Surface::Cylinder {
            origin: self.origin,
            axis: self.axis,
            radius: self.radius,
            u_ref: self.u_ref,
        }
    }

    /// The chart map `S(u, v) = o + radial(u)·r + axis·v`.
    fn at(&self, u: f64, v: f64) -> Point3<f64> {
        let w = self.axis.cross(self.u_ref);
        self.origin + (self.u_ref * u.cos() + w * u.sin()) * self.radius + self.axis * v
    }
}

/// The canonical frame: axis +z through the origin, seam at +x,
/// radius 1 — the world frame the fixtures reason in.
fn frame_a() -> CylFrame {
    CylFrame {
        origin: Point3::origin(),
        axis: Vec3::unit_z(),
        radius: 1.0,
        u_ref: Vec3::unit_x(),
    }
}

/// The divergent frame of the SAME locus: origin shifted a quarter up
/// the axis, axis direction OPPOSED, seam rotated by 0.7 rad — every
/// field a real seat's two instances disagree on, and none of it
/// moving the cylinder as a locus.
fn frame_b() -> CylFrame {
    let d = 0.7_f64;
    CylFrame {
        origin: Point3::new(0.0, 0.0, 0.25),
        axis: -Vec3::unit_z(),
        radius: 1.0,
        u_ref: Vec3::new(d.cos(), d.sin(), 0.0),
    }
}

/// An open cylinder-wall sheet over `frame`'s chart window
/// `u ∈ [u0, u1] × v ∈ [v0, v1]`, in its OWN body (its own arena, so
/// no structural chart identity can exist between two sheets): two
/// rim circle edges (exact `Curve3::Circle` carriers with
/// `Intersection` descriptions) and two meridian struts, pcurves
/// minted, and the wall's surface carrying a DISTINCT `GeomSource` per
/// sheet — the cross-instance fingerprint (`same_chart`'s
/// "distinct GeomSources" arm, the census's `:466`-arm class).
/// Returns the wall face.
fn wall_sheet(
    body: &mut Body<f64>,
    frame: CylFrame,
    src_id: u64,
    u0: f64,
    u1: f64,
    v0: f64,
    v1: f64,
) -> FaceKey {
    let (p00, p10, p11, p01) = (
        frame.at(u0, v0),
        frame.at(u1, v0),
        frame.at(u1, v1),
        frame.at(u0, v1),
    );
    let seed = body.mvfs(p00).unwrap();
    // The seed face is the wall's complement after the mef below; its
    // surface slot doubles as the cylinder key's home (the public
    // spelling of the census fixture's `add_surface`).
    let cyl = body
        .set_face_surface(seed.face, FaceSurface::New(frame.surface()))
        .unwrap();
    body.set_surface_source(cyl, topo::GeomSource::minted(src_id, 0))
        .unwrap();
    let rim = |body: &mut Body<f64>, v: f64, ccw: bool| {
        let center = frame.origin + frame.axis * v;
        // A scaffold seed carries each rim plane's description (the
        // stray lone-vertex solid is inert at the predicate door).
        let scaffold = body.mvfs(center).unwrap();
        let plane = body
            .set_face_surface(
                scaffold.face,
                FaceSurface::New(Surface::Plane {
                    origin: center,
                    normal: frame.axis,
                    u_ref: frame.u_ref,
                }),
            )
            .unwrap();
        let (carrier, t0, t1) = if ccw {
            (
                Curve3::Circle {
                    center,
                    axis: frame.axis,
                    radius: frame.radius,
                    u_ref: frame.u_ref,
                },
                u0,
                u1,
            )
        } else {
            let s = frame.at(u1, v) - center - frame.axis * ((frame.at(u1, v) - center).dot(frame.axis));
            (
                Curve3::Circle {
                    center,
                    axis: -frame.axis,
                    radius: frame.radius,
                    u_ref: s.normalize(),
                },
                0.0,
                u1 - u0,
            )
        };
        EdgeCurveSpec {
            description: EdgeDescriptionSpec::Intersection {
                s1: cyl,
                s2: plane,
                witness: frame.at((u0 + u1) * 0.5, v),
            },
            carrier,
            param_start: t0,
            param_end: t1,
        }
    };
    let bottom = rim(body, v0, true);
    let e_b = body
        .mev(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            p10,
            bottom,
            Tol::witness(),
        )
        .unwrap();
    let e_r = body
        .mev_line(
            MevSite::Fan {
                he1: e_b.he_minus,
                he2: e_b.he_minus,
            },
            p11,
            Tol::witness(),
        )
        .unwrap();
    let top = rim(body, v1, false);
    let e_t = body
        .mev(
            MevSite::Fan {
                he1: e_r.he_minus,
                he2: e_r.he_minus,
            },
            p01,
            top,
            Tol::witness(),
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
            Tol::witness(),
        )
        .unwrap()
        .face;
    topo::pcurves::mint_pcurves(body, Tol::witness()).unwrap();
    face
}

/// The A-side sheet of the seat: an arc wall in the canonical frame,
/// world azimuth `[t0, t1]`, world height `[z0, z1]`.
fn sheet_a(t0: f64, t1: f64, z0: f64, z1: f64) -> (Body<f64>, FaceKey) {
    let mut body = Body::<f64>::new();
    let f = wall_sheet(&mut body, frame_a(), 7001, t0, t1, z0, z1);
    (body, f)
}

/// The B-side sheet: the SAME world region authored in `frame_b`. The
/// transfer between the charts is `θ_world = 0.7 − u_B`,
/// `z_world = 0.25 − v_B`, so the world window `[t0, t1] × [z0, z1]`
/// is the B-chart window `[0.7 − t1, 0.7 − t0] × [0.25 − z1, 0.25 − z0]`.
fn sheet_b(t0: f64, t1: f64, z0: f64, z1: f64) -> (Body<f64>, FaceKey) {
    let mut body = Body::<f64>::new();
    let f = wall_sheet(&mut body, frame_b(), 7002, 0.7 - t1, 0.7 - t0, 0.25 - z1, 0.25 - z0);
    (body, f)
}

/// The VERDICT of one call, as the thing a caller may branch on (the
/// `census_g2_carrier` convention: outcomes and refusal ROWS, never
/// margin bits).
fn verdict_class(r: Result<ChartOverlap, ChartRegionError>) -> String {
    match r {
        Ok(ChartOverlap::PositiveArea) => "PositiveArea".into(),
        Ok(ChartOverlap::Empty) => "Empty".into(),
        Err(ChartRegionError::Escalated(d)) => {
            format!("Escalated({})", d.predicate.unwrap_or("?"))
        }
        Err(other) => format!("{other:?}")
            .split([' ', '{', '(']).next().unwrap().to_string(),
    }
}

// ---------------------------------------------------------------------
// Red-first: the class's own shape (issue 943's cylinder residue)
// ---------------------------------------------------------------------

/// INVARIANT (red-first, the MATE-5 closure): a declared
/// cylinder×cylinder pair whose two descriptions genuinely diverge —
/// no shared key, no `GeomSource`, different `u_ref`/origin/axis
/// sign — certifies its overlapping seat through the certified-ε
/// enclosure once Door 1 has verified the carrier.
///
/// On main this exact call refuses
/// `ChartDivergence { detail: "no shared SurfaceKey and no GeomSource
/// on both faces" }` — quoted in the PR body as the measured
/// refusal chain (→ `CensusUnsupported{Face}` → `Declined` →
/// `Uncertified` at the census, per the spec's situation paragraph).
#[test]
fn a_declared_cylinder_pair_with_divergent_descriptions_certifies() {
    // Overlapping arc seat: A holds azimuth [0.2, 1.6] × z [0.0, 1.0],
    // B holds [0.5, 1.3] × z [0.3, 0.7] of the same world locus.
    let (a, fa) = sheet_a(0.2, 1.6, 0.0, 1.0);
    let (b, fb) = sheet_b(0.5, 1.3, 0.3, 0.7);
    // The structural door still refuses the pair — the enclosure arm
    // is an ADDITION below that rung, never a weakening of it.
    match topo::chart_region_overlap(&a, fa, &b, fb, band()) {
        Err(ChartRegionError::ChartDivergence { .. }) => {}
        other => panic!("the structural rung must still refuse: {other:?}"),
    }
    assert_eq!(
        declared_pair_overlap(&a, fa, &b, fb, ContactVerdict::Definite, band()).unwrap(),
        ChartOverlap::PositiveArea,
    );
}

/// INVARIANT (the Refuted direction): a cylinder declaration the
/// geometry refutes — the two trims share the carrier but their
/// axial bands are definitely disjoint — answers `Empty`, which is
/// what the census turns into `StaleContactDeclaration` and the
/// assembly attribution into `Refuted` naming the mate.
#[test]
fn an_axially_disjoint_declared_cylinder_pair_is_definitely_empty() {
    let (a, fa) = sheet_a(0.2, 1.6, 0.0, 0.4);
    let (b, fb) = sheet_b(0.5, 1.3, 0.6, 1.0);
    assert_eq!(
        declared_pair_overlap(&a, fa, &b, fb, ContactVerdict::Definite, band()).unwrap(),
        ChartOverlap::Empty,
    );
}

// ---------------------------------------------------------------------
// Frame invariance (the lemma's row, run both ways)
// ---------------------------------------------------------------------

/// INVARIANT (frame invariance, the spec's binding obligation): the
/// verdict is a property of the pair, not of which description is the
/// representative — `overlap(A, B)` and `overlap(B, A)` agree in
/// verdict class on every fixture of the battery, certifying and
/// refusing alike.
#[test]
fn the_verdict_does_not_depend_on_which_description_is_representative() {
    let cases: Vec<(&str, (Body<f64>, FaceKey), (Body<f64>, FaceKey))> = vec![
        (
            "overlapping arc seat",
            sheet_a(0.2, 1.6, 0.0, 1.0),
            sheet_b(0.5, 1.3, 0.3, 0.7),
        ),
        (
            "axially disjoint",
            sheet_a(0.2, 1.6, 0.0, 0.4),
            sheet_b(0.5, 1.3, 0.6, 1.0),
        ),
        (
            "azimuthally disjoint arcs",
            sheet_a(0.2, 0.9, 0.0, 1.0),
            sheet_b(1.4, 2.0, 0.2, 0.8),
        ),
        (
            "nested arc",
            sheet_a(0.2, 2.2, 0.0, 1.0),
            sheet_b(0.8, 1.4, 0.3, 0.7),
        ),
    ];
    for (name, (a, fa), (b, fb)) in cases {
        let ab = verdict_class(declared_pair_overlap(
            &a,
            fa,
            &b,
            fb,
            ContactVerdict::Definite,
            band(),
        ));
        let ba = verdict_class(declared_pair_overlap(
            &b,
            fb,
            &a,
            fa,
            ContactVerdict::Definite,
            band(),
        ));
        assert_eq!(ab, ba, "{name}: the two orders disagree");
        println!("{name}: {ab} (both orders)");
    }
}

// ---------------------------------------------------------------------
// The carrier gates, at the pair's own extent
// ---------------------------------------------------------------------

/// INVARIANT (the extent lever, #1063's `one_tilt_two_extents`
/// pattern): the same axis tilt is absorbed by a peg-sized pair and
/// refused for a pair whose trims reach far enough along the axis for
/// the tilt to open past ε — Door 1's pinned 1 m arm does not price
/// the pair, the enclosure's own gate does.
#[test]
fn one_axis_tilt_two_extents_two_answers() {
    let eps = Tol::witness().eps();
    // A tilt of 40·k·eps radians about y through the A-origin: at
    // axial extent 0.2 it opens ~8·k·eps (definitely apart at the
    // gate's lever there only when the extent is long).
    let tilt = 40.0 * Tol::witness().k() * eps;
    let tilted = |z0: f64, z1: f64| -> (Body<f64>, FaceKey) {
        let mut body = Body::<f64>::new();
        let frame = CylFrame {
            origin: Point3::origin(),
            axis: Vec3::new(tilt.sin(), 0.0, tilt.cos()),
            radius: 1.0,
            u_ref: Vec3::new(tilt.cos(), 0.0, -tilt.sin()),
        };
        let f = wall_sheet(&mut body, frame, 7003, 0.2, 1.6, z0, z1);
        (body, f)
    };
    // Short pair: the tilt's displacement over its whole extent stays
    // inside the band ⇒ decided on the geometry, not refused for the
    // tilt.
    let (a_s, fa_s) = sheet_a(0.2, 1.6, 0.0, 1e-3);
    let (b_s, fb_s) = tilted(0.0, 1e-3);
    let short = declared_pair_overlap(&a_s, fa_s, &b_s, fb_s, ContactVerdict::Definite, band());
    assert!(
        !matches!(short, Err(ChartRegionError::CarrierTilt)),
        "a peg-extent pair absorbs the tilt: {short:?}"
    );
    // Long pair, same tilt: the trims reach metres, the same angle is
    // a definite separation there, and the arm refuses typed.
    let (a_l, fa_l) = sheet_a(0.2, 1.6, 0.0, 4.0);
    let (b_l, fb_l) = tilted(0.0, 4.0);
    match declared_pair_overlap(&a_l, fa_l, &b_l, fb_l, ContactVerdict::Definite, band()) {
        Err(ChartRegionError::CarrierTilt) => {}
        other => panic!("a table-extent pair must refuse the same tilt: {other:?}"),
    }
}

/// INVARIANT (three-outcome honesty at the gates): a radius
/// disagreement definitely outside the band refuses typed
/// (`CarrierTilt` — the carriers are definitely apart), and one
/// inside the sliver band escalates naming its row, never decides.
#[test]
fn radius_disagreement_is_three_outcome_honest() {
    let eps = Tol::witness().eps();
    let with_radius = |r: f64| -> (Body<f64>, FaceKey) {
        let mut body = Body::<f64>::new();
        let frame = CylFrame {
            radius: r,
            ..frame_b()
        };
        let f = wall_sheet(&mut body, frame, 7004, 0.7 - 1.3, 0.7 - 0.5, 0.25 - 0.7, 0.25 - 0.3);
        (body, f)
    };
    let (a, fa) = sheet_a(0.2, 1.6, 0.0, 1.0);
    let (b_far, fb_far) = with_radius(1.0 + 1e-3);
    match declared_pair_overlap(&a, fa, &b_far, fb_far, ContactVerdict::Definite, band()) {
        Err(ChartRegionError::CarrierTilt) => {}
        other => panic!("a definite radius disagreement refuses typed: {other:?}"),
    }
    let (b_sliver, fb_sliver) = with_radius(1.0 + 3.0 * eps);
    match declared_pair_overlap(&a, fa, &b_sliver, fb_sliver, ContactVerdict::Definite, band()) {
        Err(ChartRegionError::Escalated(d)) => {
            assert_eq!(d.predicate, Some("chart_region_cyl_radius"));
        }
        other => panic!("an in-band radius disagreement escalates named: {other:?}"),
    }
}

// ---------------------------------------------------------------------
// The declines the enclosure states (issues 1191 / 1435 territory)
// ---------------------------------------------------------------------

/// INVARIANT (the decline posture, stated): a pair whose folded
/// windows cannot co-inhabit one period-wide branch window declines
/// typed (`SeamBranch`) even though the quotient geometry is
/// decidable — the one-global-fold schedule's disclosed
/// incompleteness (the issue-1435 pattern: a decline on decidable
/// geometry, said out loud rather than sampled around).
#[test]
fn a_wrapping_pair_the_fold_cannot_window_declines_typed() {
    // A holds [0.0, 3.0]; B holds [3.5, 6.5] of the same locus — the
    // union spans 6.5 > τ, but B's image wraps through the seam and
    // genuinely overlaps A in [0, ~0.217].
    let (a, fa) = sheet_a(0.0, 3.0, 0.0, 1.0);
    let (b, fb) = sheet_b(3.5, 6.5, 0.2, 0.8);
    match declared_pair_overlap(&a, fa, &b, fb, ContactVerdict::Definite, band()) {
        Err(ChartRegionError::SeamBranch) => {}
        other => panic!("the un-windowable wrap declines typed: {other:?}"),
    }
}

// ---------------------------------------------------------------------
// Per-kind honesty: what stays refused, restated per kind
// ---------------------------------------------------------------------

/// INVARIANT (kind honesty, the spec's deliverable 6): cross-instance
/// declared SPHERE, CONE and TORUS pairs stay refused exactly as
/// before — `ChartDivergence`, the enclosure arm never engaging — with
/// each kind's residue restated at the refusal site (the sanctioned
/// closing shape stays recorded per kind in `chart_region.rs`).
#[test]
fn sphere_cone_and_torus_cross_instance_pairs_stay_refused() {
    let kinds: Vec<(&str, Surface<f64>)> = vec![
        (
            "sphere",
            Surface::Sphere {
                center: Point3::new(0.0, 0.0, 1.0),
                radius: 1.0,
                axis: Vec3::unit_z(),
                u_ref: Vec3::unit_x(),
            },
        ),
        (
            "cone",
            Surface::Cone {
                apex: Point3::new(0.0, 0.0, 1.0),
                axis: Vec3::unit_z(),
                half_angle: 0.5,
                u_ref: Vec3::unit_x(),
            },
        ),
        (
            "torus",
            Surface::Torus {
                center: Point3::new(0.0, 0.0, 1.0),
                axis: Vec3::unit_z(),
                major_radius: 2.0,
                minor_radius: 0.5,
                u_ref: Vec3::unit_x(),
            },
        ),
    ];
    for (kind, surface) in kinds {
        // Two independently authored prisms whose interface faces are
        // re-described as the SAME curved surface — the
        // census_g2_carrier fixture shape, per kind.
        let a: common::Prism<f64> =
            common::prism_z(&[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0)], 0.0, 1.0);
        let b: common::Prism<f64> =
            common::prism_z(&[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0)], 1.0, 2.0);
        let (mut a_body, mut b_body) = (a.body, b.body);
        a_body
            .set_face_surface(a.top_face, FaceSurface::New(surface.clone()))
            .unwrap();
        b_body
            .set_face_surface(b.bottom_face, FaceSurface::New(surface.clone()))
            .unwrap();
        match declared_pair_overlap(
            &a_body,
            a.top_face,
            &b_body,
            b.bottom_face,
            ContactVerdict::Definite,
            band(),
        ) {
            Err(ChartRegionError::ChartDivergence { .. }) => {}
            other => panic!("a declared {kind} cross-instance pair stays refused: {other:?}"),
        }
    }
}
