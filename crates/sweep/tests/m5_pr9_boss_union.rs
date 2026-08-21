//! M5 PR 9 acceptance shape (ii): cylinder boss ∪ plate — the first
//! transverse curved boolean end to end. The boss pierces the plate's
//! top face; the seam is the rim circle where the wall crosses the
//! top plane, minted as THREE conic arcs (the boss's three walls),
//! carried by exact `Circle` carriers on BOTH operands' sides,
//! described intrinsically, certified pcurves at rest, tier-3 valid,
//! volume backstopped.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use geom_core::{Affine3, Point2, Vec3};
use profile::RawLoop;
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane};
use sweep::{Extrusion, extrude};
use topo::Body;

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

/// The plate: a 4×4×1 block, z ∈ [0, 1].
fn plate() -> Body<f64> {
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(0.0, 0.0), 0.0),
        ProfileVertex::new(p2(4.0, 0.0), 0.0),
        ProfileVertex::new(p2(4.0, 4.0), 0.0),
        ProfileVertex::new(p2(0.0, 4.0), 0.0),
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    extrude(&profile, Extrusion::Distance(1.0), Tol::witness())
        .unwrap()
        .body
}

/// The boss: a radius-0.5 disc centered at (2, 2) authored as THREE
/// 120° arcs (three wall faces on ONE cylinder surface, meridian
/// struts every 120° — a 120° seam arc's endpoint tangents face its
/// chord definitely, which the join's mutual-facing germ test
/// requires; the two-arc disc's semicircles leave the germ exactly
/// perpendicular to the chord). Sketched at z = 0.4 (strictly inside
/// the plate), extruded 1.2: it pierces the top face transversally
/// and pokes out to z = 1.6.
fn boss() -> Body<f64> {
    let b120 = (core::f64::consts::PI / 6.0).tan(); // bulge of a 120° arc
    let at = |deg: f64| {
        let th = deg.to_radians();
        p2(2.0 + 0.5 * th.cos(), 2.0 + 0.5 * th.sin())
    };
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(at(0.0), b120),
        ProfileVertex::new(at(120.0), b120),
        ProfileVertex::new(at(240.0), b120),
    ]);
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, 0.4)));
    let profile = Profile::new(plane, vec![lp])
        .validate(Tol::witness())
        .unwrap();
    extrude(&profile, Extrusion::Distance(1.2), Tol::witness())
        .unwrap()
        .body
}

#[test]
fn the_boss_union_lands_end_to_end() {
    let a = plate();
    let b = boss();
    let out = topo::union(&a, &b, Tol::witness()).expect("the first transverse curved boolean");
    let body = &out.body().expect("a seamed result").body;

    // Tier 3 in full: dihedrals, descriptions, pcurve caches, +V.
    if let Err(errs) = topo::validate_geometric(body, Tol::witness()) {
        panic!("shape (ii) must be tier-3 valid: {errs:?}");
    }

    // The seam: circle-carrier edges at z = 1, radius 0.5 about
    // (2, 2), described intrinsically as Intersection.
    let mut seam_arcs = 0;
    for (_, e) in body.edges() {
        let Some(c) = body.get_curve_geom(e.curve).and_then(|g| g.certified()) else {
            continue;
        };
        if let geom::Curve3::Circle { center, radius, .. } = *c.carrier()
            && (center.z - 1.0).abs() < 1e-9
            && (radius - 0.5).abs() < 1e-9
        {
            seam_arcs += 1;
            assert!(
                matches!(
                    c.description(),
                    geom_brep::EdgeGeometry::Intersection { .. }
                ),
                "a transverse curved seam edge is intrinsic (D6)"
            );
        }
    }
    assert_eq!(seam_arcs, 3, "the rim seam is three arcs (three walls)");

    // Volume: plate (16) + protruding boss (π·0.25·0.6), within the
    // exact-B-rep props' honesty.
    let vol = topo::mass_properties(body, Tol::witness()).unwrap().volume;
    let expect = 16.0 + core::f64::consts::PI * 0.25 * 0.6;
    assert!(
        (vol - expect).abs() < 1e-6,
        "vol(A∪B) = {vol}, expected {expect}"
    );
}

/// **The M5 pin, RETIRED AS WRITTEN (M9-2)**: the pin promised that a
/// touching curved result "refuses HERE by design" only until the
/// curved census landed. It has: the census admits the curved
/// inventory, so the same body with the same fabricated record now
/// answers with the record's own honest verdict (stale — a == b is
/// not a contact) and NO blanket CensusUnsupported.
#[test]
fn the_curved_inventory_is_admitted_and_the_bogus_record_is_stale() {
    let b = boss();
    let mut contacts = topo::ContactRecords::default();
    let v = b.vertices().next().unwrap().0;
    contacts.vv.push(topo::VvContact { a: v, b: v });
    let errs = topo::validate_pseudomanifold(&b, &contacts, Tol::witness())
        .expect_err("the bogus record is refused");
    assert!(
        errs.iter()
            .all(|e| !matches!(e, topo::ValidationError::CensusUnsupported { .. })),
        "the exact-on-planar wall is retired — no blanket refusal: {errs:?}"
    );
    assert!(
        errs.iter()
            .any(|e| matches!(e, topo::ValidationError::StaleContactDeclaration { .. })),
        "a == b is not a contact — the record is stale: {errs:?}"
    );
}

/// **The M9-2 acceptance: the touching curved result at 3′.** The boss
/// rests ON the plate (a two-instance touching assembly — the graft
/// currency, same as an ASM placement): with its interface DECLARED
/// at the census's granularity it validates at 3′; undeclared it is
/// the F1 hard error. The boss's curved walls and arc rims ride
/// through the admitted curved inventory in both directions.
#[test]
fn a_touching_curved_assembly_validates_declared_and_refuses_undeclared() {
    let a = plate();
    // The boss RESTING on the plate: sketched at the plate's top.
    let b120 = (core::f64::consts::PI / 6.0).tan();
    let at = |deg: f64| {
        let th = deg.to_radians();
        p2(2.0 + 0.5 * th.cos(), 2.0 + 0.5 * th.sin())
    };
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(at(0.0), b120),
        ProfileVertex::new(at(120.0), b120),
        ProfileVertex::new(at(240.0), b120),
    ]);
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, 1.0)));
    let profile = Profile::new(plane, vec![lp])
        .validate(Tol::witness())
        .unwrap();
    let boss_on_top = extrude(&profile, Extrusion::Distance(0.6), Tol::witness())
        .unwrap()
        .body;
    let mut body = a.clone();
    topo::graft_disjoint(&mut body, &boss_on_top, Tol::witness()).unwrap();

    // UNDECLARED: the census finds the boss cap's rim-joint vertices
    // resting on the plate's top face — the hard error, typed.
    let errs =
        topo::validate_pseudomanifold(&body, &topo::ContactRecords::default(), Tol::witness())
            .expect_err("an undeclared touching assembly is the F1 hard error");
    assert!(
        errs.iter()
            .any(|e| matches!(e, topo::ValidationError::UndeclaredContact { .. })),
        "{errs:?}"
    );
    assert!(
        errs.iter()
            .all(|e| !matches!(e, topo::ValidationError::CensusUnsupported { .. })),
        "the refusal is the contact, not the inventory: {errs:?}"
    );

    // DECLARED: the interface's census evidence is the three arc-arc
    // joint vertices of the boss's bottom cap resting on the plate's
    // top face — declare exactly those (v-on-f, the C2 planar-table
    // currency a mate's Rest resolves to).
    let top_face = body
        .faces()
        .find_map(|(k, f)| match body.get_surface(f.surface) {
            Some(geom::Surface::Plane { origin, normal, .. })
                if (origin.z - 1.0).abs() < 1e-9 && normal.z > 0.5 =>
            {
                Some(k)
            }
            _ => None,
        })
        .expect("the plate's top face");
    let mut records = topo::ContactRecords::default();
    for (vk, v) in body.vertices() {
        let p = body.get_point(v.point).unwrap();
        let on_rim = (p.z - 1.0).abs() < 1e-9
            && ((p.x - 2.0).powi(2) + (p.y - 2.0).powi(2) - 0.25).abs() < 1e-9;
        if on_rim {
            records.a_on_b.push(topo::VfContact {
                vertex: vk,
                face: top_face,
            });
        }
    }
    assert_eq!(records.a_on_b.len(), 3, "the three arc joints");
    assert_eq!(
        topo::validate_pseudomanifold(&body, &records, Tol::witness()),
        Ok(()),
        "the declared touching curved assembly validates at 3′"
    );
}

// ---------------------------------------------------------------------
// R1 review probes (m9-2b-r1)
// ---------------------------------------------------------------------

/// The pin: three 120-degree arcs, radius 0.5 about (2, 2), struts at
/// 60/180/300 degrees (all OFF the cradle's slab so no strut ever
/// meets a cradle plane's region), sketched at z0, extruded by h.
fn r1_pin(z0: f64, h: f64) -> Body<f64> {
    let b120 = (core::f64::consts::PI / 6.0).tan();
    let at = |deg: f64| {
        let th = deg.to_radians();
        p2(2.0 + 0.5 * th.cos(), 2.0 + 0.5 * th.sin())
    };
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(at(60.0), b120),
        ProfileVertex::new(at(180.0), b120),
        ProfileVertex::new(at(300.0), b120),
    ]);
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, z0)));
    let profile = Profile::new(plane, vec![lp])
        .validate(Tol::witness())
        .unwrap();
    extrude(&profile, Extrusion::Distance(h), Tol::witness())
        .unwrap()
        .body
}

/// The cradle: a slab with a 60-degree concave bite of radius 0.5
/// about (2, 2) cut into its left wall — the bite wall is VALUE-EQUAL
/// to the pin's wall carrier (same axis, same radius) with the
/// opposed (concave) sense, but a DISTINCT SurfaceKey once the two
/// instances share an arena.
fn r1_cradle(bulge: f64) -> Body<f64> {
    // Bite endpoints at +/-30 degrees: x = 2 + 0.5 cos 30, y = 2 +/- 0.25.
    let xw = 2.0 + 0.5 * (30f64).to_radians().cos();
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(5.0, 1.0), 0.0),
        ProfileVertex::new(p2(5.0, 3.0), 0.0),
        ProfileVertex::new(p2(xw, 3.0), 0.0),
        ProfileVertex::new(p2(xw, 2.25), bulge),
        ProfileVertex::new(p2(xw, 1.75), 0.0),
        ProfileVertex::new(p2(xw, 1.0), 0.0),
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    extrude(&profile, Extrusion::Distance(1.0), Tol::witness())
        .unwrap()
        .body
}

/// **R1 probe (claim 1), FIXED by the union pass (F1): a conformal
/// curved touch between DISTINCT-KEY value-equal carriers.** The pin
/// rests in the cradle's bite: a genuine 60-degree x 1.0 conformal
/// patch contact between two instances — no coincident vertices, no
/// line-edge events, no planar-face rests, so no exact sweep and no
/// same-key conformal arm can see it. Pre-fix this probe pinned the
/// silent Ok(()) (the R1 review's core finding); the conservative
/// loudness backstop now refuses the pair as UNDECIDABLE, naming the
/// C9-ring class.
#[test]
fn r1_probe_conformal_touch_between_instances_refuses_undecidable() {
    let b60 = (15f64).to_radians().tan(); // bulge of a 60-degree arc
    let cradle = [b60, -b60]
        .into_iter()
        .map(r1_cradle)
        .find(|b| {
            topo::validate_geometric(b, Tol::witness()).is_ok()
                && b.faces().any(|(_, f)| {
                    matches!(
                        b.get_surface(f.surface),
                        Some(geom::Surface::Cylinder { origin, radius, .. })
                            if (origin.x - 2.0).abs() < 1e-9
                                && (origin.y - 2.0).abs() < 1e-9
                                && (radius - 0.5).abs() < 1e-9
                    )
                })
        })
        .expect("one bulge sign yields the valid bitten cradle");
    // Pin taller than the cradle so no cap plane meets a cradle strut.
    let pin = r1_pin(-0.2, 1.4);
    assert_eq!(
        topo::validate_geometric(&pin, Tol::witness()),
        Ok(()),
        "pin is tier-3"
    );
    let mut body = cradle.clone();
    topo::graft_disjoint(&mut body, &pin, Tol::witness()).unwrap();
    assert_eq!(
        topo::validate_geometric(&body, Tol::witness()),
        Ok(()),
        "tier 3 cannot see the touch (the #382 half-1 statement)"
    );
    let cyl_faces = |b: &Body<f64>| -> Vec<topo::FaceKey> {
        b.faces()
            .filter_map(|(k, f)| match b.get_surface(f.surface) {
                Some(geom::Surface::Cylinder { origin, radius, .. })
                    if (origin.x - 2.0).abs() < 1e-9
                        && (origin.y - 2.0).abs() < 1e-9
                        && (radius - 0.5).abs() < 1e-9 =>
                {
                    Some(k)
                }
                _ => None,
            })
            .collect()
    };
    let faces = cyl_faces(&body);
    assert_eq!(faces.len(), 4, "bite wall + three pin walls: {faces:?}");
    // The union fix (F1): the conservative loudness backstop refuses
    // the cross-solid curved pair as UNDECIDABLE — the C9-ring class,
    // named — instead of validating silently. (Pre-fix this probe
    // pinned the silent Ok(()) the R1 review found.)
    let errs =
        topo::validate_pseudomanifold(&body, &topo::ContactRecords::default(), Tol::witness())
            .expect_err("the cross-solid curved proximity refuses loudly");
    assert!(
        errs.iter()
            .any(|e| matches!(e, topo::ValidationError::CensusUndecidable { .. })),
        "the backstop names the undecidable pair: {errs:?}"
    );
    // The DECLARED direction: a patch record naming the cross-key
    // pair cannot certify either — it escalates through the chart
    // predicate's divergence posture (PR deviation 1) or refuses
    // typed; either way the honest lane for this real assembly class
    // does not exist yet.
    let bite = faces[0];
    let pin_wall = *faces.last().unwrap();
    let mut records = topo::ContactRecords::default();
    records.patches.push(topo::PatchContact {
        face_a: bite,
        face_b: pin_wall,
    });
    let errs = topo::validate_pseudomanifold(&body, &records, Tol::witness())
        .expect_err("the cross-key record cannot certify");
    assert!(
        errs.iter().any(|e| matches!(
            e,
            topo::ValidationError::CensusEscalated { .. }
                | topo::ValidationError::CensusUnsupported { .. }
                | topo::ValidationError::ContactContradicted { .. }
        )),
        "{errs:?}"
    );
}

/// **R1 DELTA probe (backstop exclusion attack): curved x planar
/// transverse overlap with NO vertex/line/planar evidence.** A unit
/// ball (revolved meridian — curved faces and curved seam edges only,
/// no line edges) grafted with a plate whose slab swallows the ball's
/// upper cap: genuine cross-instance material overlap. The backstop
/// excludes curved x planar pairs ("their contact classes either
/// leave vertex/line/planar evidence ... or are the pure-tangency
/// class"); this configuration leaves neither, and is transverse
/// interference, not tangency. The probe records the gate's answer.
#[test]
fn r1_delta_probe_ball_cap_embedded_in_plate() {
    let ball_lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(0.0, -1.0), 1.0),
        ProfileVertex::new(p2(0.0, 1.0), 0.0),
    ]);
    let axis = sweep::RevolveAxis {
        origin: p2(0.0, 0.0),
        dir: geom_core::Vec2::new(0.0, 1.0),
    };
    let profile = Profile::new(SketchPlane::xy(), vec![ball_lp])
        .validate(Tol::witness())
        .unwrap();
    let ball = sweep::revolve(&profile, axis, sweep::Revolution::Full, Tol::witness())
        .unwrap()
        .body;
    assert_eq!(
        topo::validate_geometric(&ball, Tol::witness()),
        Ok(()),
        "ball is tier-3"
    );
    // The plate: x, y in [-3, 3], z in [0.3, 1.3] — the ball's upper
    // cap (z > 0.3) is inside the plate's material; the ball's poles
    // (y = +/-1, z = 0) and seam circles (z = 0 plane) never meet the
    // plate's boundary, and the plate's line edges (|x| = 3 or
    // |y| = 3) are far from the ball.
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(-3.0, -3.0), 0.0),
        ProfileVertex::new(p2(3.0, -3.0), 0.0),
        ProfileVertex::new(p2(3.0, 3.0), 0.0),
        ProfileVertex::new(p2(-3.0, 3.0), 0.0),
    ]);
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, 0.3)));
    let plate = extrude(
        &Profile::new(plane, vec![lp])
            .validate(Tol::witness())
            .unwrap(),
        Extrusion::Distance(1.0),
        Tol::witness(),
    )
    .unwrap()
    .body;
    assert_eq!(
        topo::validate_geometric(&plate, Tol::witness()),
        Ok(()),
        "plate is tier-3"
    );
    let mut body = plate.clone();
    topo::graft_disjoint(&mut body, &ball, Tol::witness()).unwrap();
    let verdict =
        topo::validate_pseudomanifold(&body, &topo::ContactRecords::default(), Tol::witness());
    println!("ball-cap-in-plate verdict: {verdict:?}");
    let errs = verdict.expect_err(
        "R1 DELTA FINDING if this fires as Ok: a curved x planar transverse \
         overlap validated SILENTLY — the class the backstop's curved x planar \
         exclusion claims always leaves evidence or is pure tangency",
    );
    assert!(
        errs.iter().any(|e| matches!(
            e,
            topo::ValidationError::CensusUndecidable { .. }
                | topo::ValidationError::UndeclaredContact { .. }
        )),
        "loud somehow: {errs:?}"
    );
}

/// **R1 FINAL-DELTA probe (F5 lemma edge): a REFLEX (270-degree) arc
/// cap.** The plane reach box's lemma says an arc bulges at most its
/// radius from its chord — false for major arcs (sagitta up to 2r),
/// so the cap's box misses the far half of the disc. A ball tangent
/// to the cap INSIDE that uncovered zone probes whether the system
/// still refuses (the arc's neighbouring wall face's own sound box
/// must catch the ball) or validates silently through the lemma gap.
#[test]
fn r1_final_delta_probe_reflex_arc_cap_stays_loud() {
    // Pac-man prism: 270-degree arc r 1 about the origin, endpoints
    // at angles -135/+135 degrees (chord on the left), extruded 1.
    let b270 = (67.5f64).to_radians().tan();
    let at = |deg: f64| {
        let th = deg.to_radians();
        p2(th.cos(), th.sin())
    };
    let lp = ProfileLoop::new(vec![
        // chord from 135 to 225 (straight)
        ProfileVertex::new(at(135.0), 0.0),
        // the 270-degree arc from 225 around to 135
        ProfileVertex::new(at(225.0), b270),
    ]);
    let pac = extrude(
        &Profile::new(SketchPlane::xy(), vec![lp])
            .validate(Tol::witness())
            .unwrap(),
        Extrusion::Distance(1.0),
        Tol::witness(),
    )
    .unwrap()
    .body;
    assert_eq!(
        topo::validate_geometric(&pac, Tol::witness()),
        Ok(()),
        "pac prism tier-3"
    );
    // The ball: radius 0.3, tangent to the top cap (z = 1) at
    // (0.65, 0), i.e. resting on the cap deep inside the reflex zone
    // (the cap's vertex hull is the chord x = cos 135 ~ -0.707, so
    // the +x half of the disc is beyond hull + pad on no axis... the
    // probe asks the gate, not the box).
    let ball_lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(0.0, -0.3), 1.0),
        ProfileVertex::new(p2(0.0, 0.3), 0.0),
    ]);
    let axis = sweep::RevolveAxis {
        origin: p2(0.0, 0.0),
        dir: geom_core::Vec2::new(0.0, 1.0),
    };
    // Place the ball by translating the SKETCH plane: center lands at
    // (0.65, 0, 1.3) — beyond even the cap's padded box on x (box hi
    // ~ 0.293), tangent to the cap plane z = 1 at (0.65, 0, 1).
    let ball_plane = SketchPlane::new(Affine3::translation(Vec3::new(0.65, 0.0, 1.3)));
    let ball = sweep::revolve(
        &Profile::new(ball_plane, vec![ball_lp])
            .validate(Tol::witness())
            .unwrap(),
        axis,
        sweep::Revolution::Full,
        Tol::witness(),
    )
    .unwrap()
    .body;
    assert_eq!(
        topo::validate_geometric(&ball, Tol::witness()),
        Ok(()),
        "ball tier-3"
    );
    let mut body = pac.clone();
    topo::graft_disjoint(&mut body, &ball, Tol::witness()).unwrap();
    let verdict =
        topo::validate_pseudomanifold(&body, &topo::ContactRecords::default(), Tol::witness());
    println!("reflex-arc cap + tangent ball verdict: {verdict:?}");
    let errs = verdict.expect_err(
        "R1 FINAL-DELTA FINDING if Ok: the reflex-arc lemma gap admits a silent \
         tangent touch",
    );
    assert!(
        errs.iter()
            .any(|e| matches!(e, topo::ValidationError::CensusUndecidable { .. })),
        "loud somehow else: {errs:?}"
    );
}
