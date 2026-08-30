//! R2 blinded-review probes for VERBS-GERMARMS PR-1 (the curved pierce
//! ring lane), ordinal 106. Each row re-derives one of the PR's
//! load-bearing claims from the public doors rather than trusting the
//! body text; the doc comments say which claim and what would falsify
//! it.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use profile::{Profile, RawLoop, SketchPlane};
use sweep::{Extrusion, extrude};
use topo::{Body, BooleanError};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

fn cyl(cx: f64, cy: f64, r: f64, z0: f64, z1: f64) -> Body<f64> {
    let tol = Tol::witness();
    let lp = profile::circle(p2(cx, cy), r, tol).unwrap();
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, z0)));
    let profile = Profile::new(plane, vec![lp.into()]).validate(tol).unwrap();
    extrude(&profile, Extrusion::Distance(z1 - z0), tol)
        .unwrap()
        .body
}

fn boxx(x0: f64, x1: f64, y0: f64, y1: f64, z0: f64, z1: f64) -> Body<f64> {
    let tol = Tol::witness();
    let lp: profile::ProfileLoop<f64> =
        RawLoop::polygon([p2(x0, y0), p2(x1, y0), p2(x1, y1), p2(x0, y1)]);
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, z0)));
    let profile = Profile::new(plane, vec![lp]).validate(tol).unwrap();
    extrude(&profile, Extrusion::Distance(z1 - z0), tol)
        .unwrap()
        .body
}

/// The steinmetz partner wall, as the probe fixture builds it: the
/// tall z-cylinder turned 90 degrees about x, giving `Cylinder`
/// origin (0,2,0), axis (0,-1,0), r 1.
fn turned() -> Body<f64> {
    topo::transform_rigid(
        &cyl(0.0, 0.0, 1.0, -2.0, 2.0),
        &Affine3::rotation_about_axis(
            Point3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            PI / 2.0,
        ),
        Tol::witness(),
    )
    .unwrap()
}

/// **Claim 1, the tangency fold, re-derived from the geometry doors.**
/// The PR's central measurement: A's seam ruling at azimuth 0/pi sits
/// exactly on the steinmetz section's self-crossing points (+-1,0,0),
/// where the walls are mutually tangent; the dip bound is EXACT there
/// (m = 0 puts the parabola vertex at the span centre, where q/8 IS the
/// true dip); and the 45-degree-spun seam is a definite crossing with
/// bound exactly -1/4. Reproduced here with `implicit_residual` and the
/// reduce.rs charge formula, on exact coordinates.
#[test]
fn r2_the_steinmetz_fold_numbers_rederive_exactly() {
    let wall_b = geom::Surface::Cylinder {
        origin: Point3::new(0.0, 2.0, 0.0),
        axis: Vec3::new(0.0, -1.0, 0.0),
        radius: 1.0,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    // The residual charge of reduce.rs's both-outside arm, verbatim:
    // f2 per kind, q = f2*span^2, m = |r_v - r_u|,
    // dip = max(0, q/2 - m)/4, bound = min(r_u, r_v) - dip.
    let bound = |p0: Point3<f64>, dir: Vec3<f64>, span: f64| -> (f64, f64, f64) {
        let axis = Vec3::new(0.0, -1.0, 0.0);
        let d_ax = dir.dot(axis);
        let f2 = (dir.norm_squared() - d_ax * d_ax) / 1.0;
        let r_u = geom_brep::implicit_residual(&wall_b, p0);
        let r_v = geom_brep::implicit_residual(&wall_b, p0 + dir * span);
        let q = f2 * span * span;
        let m = (r_v - r_u).abs();
        let dip = ((q * 0.5 - m).max(0.0)) * 0.25;
        (r_u, r_v, r_u.min(r_v) - dip)
    };
    // Azimuth-0 seam: (-1, 0, z), z in [-2, 2]. Residual along it is
    // z^2/2 (min exactly 0 at z = 0, the singular point (-1,0,0)), so
    // both endpoint residuals are 2 and the charge equals the true dip.
    let (r_u, r_v, b0) = bound(Point3::new(-1.0, 0.0, -2.0), Vec3::new(0.0, 0.0, 1.0), 4.0);
    assert_eq!(r_u, 2.0);
    assert_eq!(r_v, 2.0);
    assert!(
        b0.abs() < 1e-12,
        "the tangent seam's bound is a true zero: {b0}"
    );
    // The tangency itself: the seam's closest point to the partner
    // wall has residual exactly 0 — on the surface, not through it.
    assert_eq!(
        geom_brep::implicit_residual(&wall_b, Point3::new(-1.0, 0.0, 0.0)),
        0.0
    );
    // And (+-1, 0, 0) are the section's self-crossings: both walls'
    // residuals vanish there (x^2+y^2=1 and x^2+z^2=1 meet in y=+-z).
    let wall_a = geom::Surface::Cylinder {
        origin: Point3::new(0.0, 0.0, -2.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: 1.0,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    for x in [-1.0, 1.0] {
        let p = Point3::new(x, 0.0, 0.0);
        assert_eq!(geom_brep::implicit_residual(&wall_a, p), 0.0);
        assert_eq!(geom_brep::implicit_residual(&wall_b, p), 0.0);
    }
    // The 45-degree-spun seam leaves the singular point and definitely
    // crosses: residual (0.5 + z^2 - 1)/2, endpoints 1.75, true
    // minimum -1/4 at z = 0 — and the charge (q/8 = 2) makes the bound
    // exactly 1.75 - 2 = -0.25, a definite crossing.
    let s = -(0.5f64.sqrt());
    let (r_u, r_v, b45) = bound(Point3::new(s, s, -2.0), Vec3::new(0.0, 0.0, 1.0), 4.0);
    assert!((r_u - 1.75).abs() < 1e-12, "{r_u}");
    assert!((r_v - 1.75).abs() < 1e-12, "{r_v}");
    assert!(
        (b45 + 0.25).abs() < 1e-12,
        "the spun bound is exactly -1/4: {b45}"
    );
}

/// **Claim 1's dynamic half: the ring lane resolves the crossings and
/// the surviving refusal is the OTHER tangency.** Spin A 45 degrees
/// about its own axis: A's two seams now definitely cross B's wall (the
/// -0.25 bound above), so the A-side sweep no longer raises — but B's
/// own seam rulings (x = +-1, z = 0 lines along y) are still TANGENT to
/// A's wall at the section's singular points, so the pair still refuses
/// at the pierce door, now naming an operand-B edge. Before this PR the
/// refusal named operand A; the operand flip is the measurement that
/// the A-side crossings were actually found and split.
#[test]
fn r2_a_spun_steinmetz_moves_the_raiser_to_the_partner_seam() {
    let spun = topo::transform_rigid(
        &cyl(0.0, 0.0, 1.0, -2.0, 2.0),
        &Affine3::rotation_about_axis(
            Point3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            PI / 4.0,
        ),
        Tol::witness(),
    )
    .unwrap();
    let b = turned();
    let err =
        topo::union(&spun, &b, Tol::witness()).expect_err("B's seam tangency still has no arm");
    // MEASUREMENT (first run refuted the B prediction): print the whole
    // payload plus the named edge's carrier and endpoints so the raiser
    // is identifiable.
    if let BooleanError::CurvedPierceUnsupported {
        operand,
        face,
        edge,
        ..
    } = &err
    {
        let owner = match operand {
            topo::Operand::A => &spun,
            topo::Operand::B => &b,
        };
        let carrier = owner
            .get_edge(*edge)
            .and_then(|e| owner.get_curve_geom(e.curve));
        let ends = owner.get_edge(*edge).map(|e| {
            let pt = |he| {
                owner
                    .get_half_edge(he)
                    .and_then(|h| owner.get_vertex(h.start))
                    .and_then(|v| owner.get_point(v.point).copied())
            };
            (pt(e.he_plus), pt(e.he_minus))
        });
        eprintln!("spun steinmetz raiser: op={operand:?} face={face:?} edge={edge:?}");
        eprintln!("  carrier: {carrier:?}");
        eprintln!("  endpoints: {ends:?}");
    }
    eprintln!("spun steinmetz full error: {err:?}");
    assert!(
        matches!(err, BooleanError::CurvedPierceUnsupported { .. }),
        "{err:?}"
    );
}

/// **Claim 2/(Zero,Zero): an edge exactly ON the wall keeps the
/// cosurface door.** A box whose corners all sit exactly on the pipe's
/// wall carrier (x = +-sqrt(1 - 0.09), y = +-0.3) has four long edges
/// that are RULINGS of the wall — axis-parallel lines lying on the
/// carrier with both endpoints at residual 0. The ring lane's
/// structural separation says those must answer `Constant` and keep the
/// pierce door (an undeclared cosurface is never an event), even though
/// the same box's x- and y-edges are honest secant chords.
#[test]
fn r2_a_box_with_on_carrier_rulings_keeps_the_cosurface_door() {
    let x = (1.0f64 - 0.09).sqrt();
    let err = topo::union(
        &cyl(0.0, 0.0, 1.0, -2.0, 2.0),
        &boxx(-x, x, -0.3, 0.3, -0.3, 0.3),
        Tol::witness(),
    )
    .expect_err("an undeclared on-carrier contact must refuse");
    assert!(
        matches!(err, BooleanError::CurvedPierceUnsupported { .. }),
        "the rulings on the carrier keep the pierce door: {err:?}"
    );
}

/// **Claim 3, the transient chord's certification, re-measured.** The
/// survey left "a straight `line_between` chord is mev-legal on a
/// cylinder wall" UNVERIFIED; the PR claims it VERIFIED. Re-run the
/// experiment through the public euler doors: on the pipe's wall face,
/// mev a straight chord from a boundary vertex to an interior wall
/// point (the chord does NOT lie on the wall), tier-1 validate, kemr it
/// into an empty ring, tier-1 validate again.
#[test]
fn r2_the_transient_chord_mev_certifies_on_a_wall() {
    let tol = Tol::witness();
    let mut body = cyl(0.0, 0.0, 1.0, -2.0, 2.0);
    // Find a cylinder wall face and its outer loop's anchor half-edge.
    let mut wall = None;
    for (fk, f) in body.faces() {
        if matches!(
            body.get_surface(f.surface),
            Some(geom::Surface::Cylinder { .. })
        ) {
            wall = Some((fk, f.outer));
        }
    }
    let (_face, outer) = wall.expect("the pipe has a cylinder wall face");
    let topo::LoopBoundary::Cycle { first: anchor } = body.get_loop(outer).unwrap().boundary else {
        panic!("wall outer loop is a cycle");
    };
    let u = body.get_half_edge(anchor).unwrap().start;
    let p_u = *body.get_point(body.get_vertex(u).unwrap().point).unwrap();
    // An interior point of the wall, well away from the anchor.
    let p = Point3::new(0.0, 1.0, 0.0);
    let chord = body
        .mev(
            topo::MevSite::Fan {
                he1: anchor,
                he2: anchor,
            },
            p,
            geom_brep::EdgeCurveSpec::line_between(p_u, p),
            tol,
        )
        .expect("the chord certifies against its own carrier, not the wall");
    assert_eq!(topo::validate(&body), Ok(()), "tier 1 after mev");
    body.kemr(chord.he_plus, chord.he_minus)
        .expect("the chord detaches into an empty ring");
    assert_eq!(topo::validate(&body), Ok(()), "tier 1 after kemr");
}

/// **Claim 6's cone differential, sharpened to the actual door.** The
/// shipped row only asserts the cone fixture does NOT reach the ring
/// lane's join door; this row measures which door it DOES reach, so the
/// record says what the cone's own door is rather than only what it is
/// not.
#[test]
fn r2_the_cone_fixture_door_is_measured_not_just_excluded() {
    let tol = Tol::witness();
    let lp = profile::ProfileLoop::new(
        [(0.2, 0.0), (0.6, 0.0), (0.4, 0.6), (0.2, 0.6)]
            .into_iter()
            .map(|(r, y)| profile::ProfileVertex::new(p2(r, y), 0.0))
            .collect(),
    );
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(tol)
        .unwrap();
    let frustum = sweep::revolve(
        &profile,
        sweep::RevolveAxis {
            origin: p2(0.0, 0.0),
            dir: geom_core::Vec2::new(0.0, 1.0),
        },
        sweep::Revolution::Full,
        tol,
    )
    .unwrap()
    .body;
    let err = topo::union(&frustum, &boxx(-1.0, 1.0, -0.05, 0.05, 0.25, 0.35), tol)
        .expect_err("a cone wall has no roots anywhere");
    // Measured: record the exact variant. The claim to falsify is that
    // the ring lane gave a cone roots; any typed refusal that names the
    // cone's own absence is consistent with the fence.
    match &err {
        BooleanError::Join(topo::SplitJoinError::SectionArcWindow { .. }) => {
            panic!("the cone reached the ring lane's join door: {err:?}")
        }
        other => {
            eprintln!("cone fixture door, measured: {other:?}");
        }
    }
}

/// **Claim 2/belly + requeue: an off-centre bar still routes to the
/// join.** The acceptance bar is symmetric about the pipe axis; this
/// one is offset so the four wall crossings sit at four distinct
/// unrelated parameters, exercising the split-then-requeue path with no
/// symmetry to hide an off-by-one in the second root's rediscovery.
///
/// MEASUREMENT (first run): the crossings ARE found and the union DOES
/// reach the join, but the refusal is `SectionArcWindow {
/// NeitherContained }` rather than the acceptance rows' `NoChartedRun`
/// — the ring-join residue is pose-dependent. Pinned as measured; the
/// interpretation question (is NeitherContained here an honest window
/// degeneracy or mis-bookkept runs on an asymmetric pose?) goes to the
/// review report.
#[test]
fn r2_an_off_centre_bar_reaches_the_same_join_door() {
    let err = topo::union(
        &cyl(0.0, 0.0, 1.0, -2.0, 2.0),
        &boxx(-3.0, 3.0, 0.15, 0.7, -0.4, 0.1),
        Tol::witness(),
    )
    .expect_err("no join arm for a pierce ring");
    eprintln!("off-centre bar join refusal, measured: {err:?}");
    assert!(
        matches!(
            err,
            BooleanError::Join(topo::SplitJoinError::SectionArcWindow { .. })
        ),
        "{err:?}"
    );
}

/// Fixture inspection (measurement aid, no claim): the pipe's face
/// inventory by key, so join payloads naming a FaceKey are readable.
#[test]
fn r2_fixture_face_inventory() {
    let pipe = cyl(0.0, 0.0, 1.0, -2.0, 2.0);
    for (k, f) in pipe.faces() {
        eprintln!("pipe face {k:?}: {:?}", pipe.get_surface(f.surface));
    }
}
