//! **BLEND-6 review probes (lane r1).** What the unit's own rows do not
//! reach: the material side of the boss's dome rim read off the BODY
//! (a stored sense bit and the dome's own station, not an argument); a
//! convex corner ARC of a mixed outer cycle, where the ladder walk's
//! `max(external, containment)` clears on its EXTERNAL term; and an
//! off-axis ring — a BORE the extrude door mints, no boolean — that
//! puts the exact containment backstop at the front door with a
//! NEGATIVE `fillet3_ring_clearance` reading for each of the two new
//! relations, because the bore's sample lattice and the rim's do not
//! align and predicate 2's sampled gap is strictly larger than the true
//! one.
//!
//! `r1_diag_cylinder_pierces` records why the boolean door is NOT the
//! way to that ring today: an off-axis pierce of a cylinder cap refuses
//! at the split-join's role resolution unless the ball's seam plane is
//! the operand's — and then the ring's closest approach sits at a
//! sample azimuth and the screen reads the exact gap.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Surface;
use geom_core::{Affine3, Point2, Sign, Tol, Vec3};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::blend::BlendError;
use sweep::blend::build::fillet_edges;
use sweep::test_support::{ball_poled_z, boss, prism, revolved_about_y, rim_arcs_at};
use sweep::{Extrusion, Revolution, extrude};
use topo::boolean::{BooleanDeclarations, BooleanOp, SweepStrategy, boolean_op_with};
use topo::{Body, EdgeKey, FaceKey, mass_properties, validate_geometric};

fn tol() -> Tol {
    Tol::witness()
}

fn repaired(up: bool) -> Body<f64> {
    let mut b = boss(up, tol());
    b.merge_coplanar_faces(tol())
        .expect("the pole-split caps repair");
    b
}

/// `a ∖ b` through the public boolean door.
fn subtract(a: &Body<f64>, b: &Body<f64>) -> Body<f64> {
    boolean_op_with(
        BooleanOp::Subtract,
        a,
        b,
        &BooleanDeclarations::none(),
        SweepStrategy::Realized,
        tol(),
    )
    .expect("the subtraction runs")
    .body()
    .expect("the subtraction leaves a body")
    .body
    .clone()
}

/// A ball of radius `r` poled along `y` (the boss's axis), centred at
/// `c` — so it meets a `y`-plane pole-on.
fn ball_poled_y(r: f64, c: Vec3<f64>) -> Body<f64> {
    let b = revolved_about_y(
        vec![
            ProfileVertex::new(Point2::new(0.0, -r), 1.0),
            ProfileVertex::new(Point2::new(0.0, r), 0.0),
        ],
        Revolution::Full,
        tol(),
    );
    topo::transform_rigid(&b, &Affine3::translation(c), tol()).unwrap()
}

/// The two faces of an edge.
fn faces_of(body: &Body<f64>, e: EdgeKey) -> [FaceKey; 2] {
    let ed = body.get_edge(e).unwrap();
    [ed.he_plus, ed.he_minus].map(|he| {
        body.get_loop(body.get_half_edge(he).unwrap().parent_loop)
            .unwrap()
            .face
    })
}

/// A face's OUTWARD normal on a plane, from its stored surface and its
/// stored sense bit (`same_sense`), not from any sampled geometry.
fn plane_outward(body: &Body<f64>, f: FaceKey) -> Vec3<f64> {
    let fd = body.get_face(f).unwrap();
    let Surface::Plane { normal, .. } = body.get_surface(fd.surface).unwrap() else {
        panic!("not a plane")
    };
    if fd.sense { *normal } else { -*normal }
}

/// The reading of whichever clearance predicate refused, by name.
fn refusal(err: BlendError) -> (&'static str, Sign, f64) {
    let margin = match err {
        BlendError::RingClearance { margin, .. }
        | BlendError::FaceClearanceUncertified { margin, .. } => margin,
        other => panic!("expected a clearance refusal, got {other:?}"),
    };
    let v = margin.value().expect("a definite reading");
    (margin.predicate, margin.sign, v)
}

/// The circle edges of radius `r` whose centre lies at `z = z0`, with
/// the circle's axis along `±z` (so a sphere's seam meridian of the
/// same radius and centre is excluded) and, with `off_axis`, whose
/// centre lies off the `z` axis — resolved to the one rim they seed.
fn z_rim(body: &Body<f64>, r: f64, z0: f64, off_axis: bool) -> Vec<EdgeKey> {
    let seed = body
        .edges()
        .find(|(_, e)| {
            let c = body.get_curve_geom(e.curve).unwrap().certified().unwrap();
            matches!(c.carrier(), geom::Curve3::Circle { radius, center, axis, .. }
                if (radius - r).abs() < 1e-9 && (center.z - z0).abs() < 1e-9
                    && axis.z.abs() > 0.9
                    && (center.x.hypot(center.y) > 0.1) == off_axis)
        })
        .map(|(k, _)| k)
        .expect("the rim's seed edge");
    topo::query::rim_of(body, seed).expect("one rim")
}

// ------------------------------------------------------------------
// C2: the material side, read off the body.
// ------------------------------------------------------------------

/// **The dome rim's material side is a fact of the body, and it is
/// CONCAVE on the boss.** The flat top's outward normal is read from
/// its stored sense bit; the dome leaves the rim toward its pole
/// vertex's station. The 90° wedge between the flat top (running out
/// along `+x`) and the dome's tangent (running toward the pole) is
/// material exactly when the pole lies on the flat top's MATERIAL
/// side (`−n`), i.e. the rim is convex iff `sign(pole.y − 1)·n.y < 0`.
/// The carved band's own sense bit (`Convexity::blend_sense`, true on
/// a convex chain) and the sign of the volume delta must agree with
/// that reading.
#[test]
fn r1_the_dome_rims_material_side_is_read_off_the_body() {
    for up in [true, false] {
        let name = if up { "boss" } else { "dimple" };
        let body = repaired(up);
        let arcs = rim_arcs_at(&body, 0.5, 1.0);
        assert_eq!(arcs.len(), 2);
        let [fa, fb] = faces_of(&body, arcs[0]);
        let is_plane = |f: FaceKey| {
            matches!(
                body.get_surface(body.get_face(f).unwrap().surface).unwrap(),
                Surface::Plane { .. }
            )
        };
        let (top, dome) = if is_plane(fa) { (fa, fb) } else { (fb, fa) };
        assert!(
            is_plane(top) && !is_plane(dome),
            "{name}: a plane–sphere rim"
        );
        let n = plane_outward(&body, top);
        assert!(
            (n.y.abs() - 1.0).abs() < 1e-12,
            "{name}: the flat top's normal is ±y"
        );
        // The dome's pole: the one vertex on the axis away from the rim's plane.
        let pole_y = body
            .vertices()
            .map(|(_, v)| *body.get_point(v.point).unwrap())
            .find(|p| p.x.abs() < 1e-12 && p.z.abs() < 1e-12 && (p.y - 1.0).abs() > 0.1)
            .expect("the pole vertex")
            .y;
        let convex = (pole_y - 1.0).signum() * n.y < 0.0;
        assert_eq!(
            convex, !up,
            "{name}: the boss's dome rim is concave, the dimple's convex"
        );
        let before = mass_properties(&body, tol()).unwrap().volume;
        let out = fillet_edges(&body, &arcs, 0.1, tol()).expect("carves");
        validate_geometric(&out.body, tol()).expect("tier-3 valid");
        let band = out.band_faces[0];
        assert_eq!(
            out.body.get_face(band).unwrap().sense,
            convex,
            "{name}: the band's stored sense bit is the chain's convexity"
        );
        let delta = mass_properties(&out.body, tol()).unwrap().volume - before;
        assert_eq!(
            delta > 0.0,
            !convex,
            "{name}: a concave band ADDS material ({delta})"
        );
    }
}

// ------------------------------------------------------------------
// C3: a convex corner arc of a mixed outer cycle.
// ------------------------------------------------------------------

/// A rounded-rectangle plate `[−1,1]² × [0, 0.4]` with corner radius
/// `rho`, and a hemispherical dimple of radius `a` cut into its top at
/// `(cx, cy)`.
fn dimpled_plate(rho: f64, a: f64, cx: f64, cy: f64) -> Body<f64> {
    let q = (core::f64::consts::FRAC_PI_2 / 4.0).tan();
    let s = 1.0 - rho;
    let v = |x: f64, y: f64, b: f64| ProfileVertex::new(Point2::new(x, y), b);
    // Every joint is a line meeting a corner arc tangentially: declared.
    let lp = ProfileLoop::new(vec![
        v(-s, -1.0, 0.0),
        v(s, -1.0, q),
        v(1.0, -s, 0.0),
        v(1.0, s, q),
        v(s, 1.0, 0.0),
        v(-s, 1.0, q),
        v(-1.0, s, 0.0),
        v(-1.0, -s, q),
    ])
    .with_tangent_joints((0..8).collect());
    let pf = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(tol())
        .expect("the rounded rectangle validates with its joints declared");
    let plate = extrude(&pf, Extrusion::Distance(0.4), tol())
        .expect("the plate extrudes")
        .body;
    subtract(&plate, &ball_poled_z(a, Vec3::new(cx, cy, 0.4), tol()))
}

/// **A LADDER rim inside a MIXED outer cycle clears its convex corner
/// arcs on the EXTERNAL term.** The plate's top face has an outer cycle
/// of four lines and four quarter-circle arcs of radius `rho = 0.3`
/// centred at `(±0.7, ±0.7)`; the dimple ring at `(0.3, 0)` is a ladder
/// rim whose widened trim circle (`si = √(a² + 2ar) ≈ 0.178`) is `0.806`
/// from the nearest corner centre. That corner arc's FULL circle
/// encloses nothing of the trim circle, so the containment term
/// `rho − (d + si)` is negative on it and only the external term
/// `d − si − rho ≈ +0.33` clears — the head carves. Metering the arc on
/// the containment term alone refuses this body (measured: it is the
/// one row in the tree that reds under that mutant).
#[test]
fn r1_a_convex_corner_arc_of_a_mixed_outer_cycle_takes_the_external_term() {
    let body = dimpled_plate(0.3, 0.15, 0.3, 0.0);
    validate_geometric(&body, tol()).expect("the dimpled plate is tier-3 valid");
    let arcs = z_rim(&body, 0.15, 0.4, true);
    let before = mass_properties(&body, tol()).unwrap().volume;
    let out = fillet_edges(&body, &arcs, 0.03, tol())
        .unwrap_or_else(|e| panic!("the ladder rim beside a convex corner arc carves, got {e:?}"));
    validate_geometric(&out.body, tol()).expect("tier-3 valid");
    assert_eq!(out.band_faces.len(), 1);
    let after = mass_properties(&out.body, tol()).unwrap();
    assert_eq!(after.volume_pad, 0.0);
    assert!(
        after.volume < before,
        "a convex dimple rim removes material"
    );
}

// ------------------------------------------------------------------
// C4: the non-coaxial ring that reaches the exact backstop.
// ------------------------------------------------------------------

/// **The bored cylinder — no boolean at all.** One extrude of a profile
/// whose outer loop is the unit circle with its two vertices at
/// azimuths `outer_phi` and `outer_phi + π`, and whose INNER loop is a
/// circle of radius `a` centred at `(d, 0)` with its two vertices on
/// the `x` axis. The top cap is one plane face: outer cycle the top rim
/// (two arcs, mates two half-walls of one cylinder key — the hostless
/// annulus), one RING the bore's top rim (a ladder rim inside a
/// circular outer boundary, mate the bore's two half-walls). The bore's
/// closest approach to the outer rim is its vertex at `(d + a, 0)`, and
/// with `outer_phi = 11.25°` no sample of the outer rim's lattice
/// (`outer_phi + k·22.5°`) lies at azimuth 0 — so predicate 2's sampled
/// gap is strictly larger than the true `1 − (d + a)`.
fn bored_cylinder(a: f64, d: f64, outer_phi: f64) -> Body<f64> {
    let v = |x: f64, y: f64, b: f64| ProfileVertex::new(Point2::new(x, y), b);
    let outer = ProfileLoop::new(vec![
        v(outer_phi.cos(), outer_phi.sin(), 1.0),
        v(-outer_phi.cos(), -outer_phi.sin(), 1.0),
    ]);
    let inner = ProfileLoop::new(vec![v(d + a, 0.0, -1.0), v(d - a, 0.0, -1.0)]);
    let pf = Profile::new(SketchPlane::xy(), vec![outer, inner])
        .validate(tol())
        .expect("a bored disc validates");
    extrude(&pf, Extrusion::Distance(1.0), tol())
        .expect("the bored disc extrudes")
        .body
}

/// **The hostless ANNULUS rim's containment backstop, reached at the
/// front door by an off-axis ring.** Top rim at `r = 0.1` → trim `0.9`;
/// bore of radius `0.16` at `d = 0.75` reaches `0.91`: exact containment
/// `−0.01`. The outer rim's samples sit at `11.25° + k·22.5°` and the
/// bore's outermost point at azimuth 0, so the sampled gap is ≈ `0.14`
/// against a setback of `0.1` and the screen passes. The carving side
/// (bore `0.14`, `+0.01`) carves tier-3 valid. Reds under the ring
/// walk's form swap (the external form reads `−0.31` here).
#[test]
fn r1_a_bored_cylinders_off_axis_ring_reaches_the_annulus_backstop_at_the_front_door() {
    let phi = 11.25f64.to_radians();
    let body = bored_cylinder(0.16, 0.75, phi);
    validate_geometric(&body, tol()).expect("the bored cylinder is tier-3 valid");
    let arcs = z_rim(&body, 1.0, 1.0, false);
    assert_eq!(arcs.len(), 2, "the top rim is two arcs");
    let err = fillet_edges(&body, &arcs, 0.1, tol())
        .expect_err("a ring reaching into the excised strip refuses")
        .error;
    let (predicate, sign, read) = refusal(err);
    assert_eq!(
        predicate, "fillet3_ring_clearance",
        "the exact backstop answers, not the screen"
    );
    assert_eq!(sign, Sign::Negative);
    assert!(
        (read - (0.9 - (0.75 + 0.16))).abs() < 1e-12,
        "the containment reading `si − (d + a)` (read {read})"
    );
    let wide = bored_cylinder(0.14, 0.75, phi);
    let arcs = z_rim(&wide, 1.0, 1.0, false);
    let out = fillet_edges(&wide, &arcs, 0.1, tol()).expect("a contained off-axis ring carves");
    validate_geometric(&out.body, tol()).expect("tier-3 valid");
    assert_eq!(mass_properties(&out.body, tol()).unwrap().volume_pad, 0.0);
}

/// **The LADDER rim's outer-boundary containment backstop, reached at
/// the front door by the same off-axis ring.** The bore's top rim is a
/// plane–cylinder ladder rim; at `r = 0.03` its plane trim circle has
/// radius `si = a + r = 0.19` (the ball rests in the material at depth
/// `r`, touching the bore wall), so the containment margin against the
/// outer boundary at radius 1 is `1 − (d + 0.19)`: `−0.01` at
/// `d = 0.82`. The external term on the same pair is `≈ −0.37`, so the
/// circle arm's `max` IS the containment reading — the negative side
/// of that `max` the unit's own rows never reach. Sampled gap ≈ `0.095`
/// against a setback of `0.03`: the screen passes. The carving side at
/// `d = 0.80` carves. Reds under the circle arm's external-only mutant.
#[test]
fn r1_a_bored_cylinders_off_axis_ring_reaches_the_ladder_backstop_at_the_front_door() {
    let phi = 11.25f64.to_radians();
    let (a, r) = (0.16, 0.03);
    let d = 0.82;
    let body = bored_cylinder(a, d, phi);
    validate_geometric(&body, tol()).expect("the bored cylinder is tier-3 valid");
    let arcs = z_rim(&body, a, 1.0, true);
    assert_eq!(arcs.len(), 2, "the bore's top rim is two arcs");
    let err = fillet_edges(&body, &arcs, r, tol())
        .expect_err("a trim circle crossing its host's circular boundary refuses")
        .error;
    let (predicate, sign, read) = refusal(err);
    assert_eq!(
        predicate, "fillet3_ring_clearance",
        "the exact backstop answers, not the screen"
    );
    assert_eq!(sign, Sign::Negative);
    assert!(
        (read - (1.0 - (d + a + r))).abs() < 1e-12,
        "the containment reading `R − (d + si)` (read {read}, want {})",
        1.0 - (d + a + r)
    );
    let wide = bored_cylinder(a, 0.80, phi);
    let arcs = z_rim(&wide, a, 1.0, true);
    let out = fillet_edges(&wide, &arcs, r, tol()).expect("a contained trim circle carves");
    validate_geometric(&out.body, tol()).expect("tier-3 valid");
    assert_eq!(mass_properties(&out.body, tol()).unwrap().volume_pad, 0.0);
}

/// **Why the boolean door is not the way to that ring.** A cylinder
/// cap — extruded or revolved-and-repaired, ring-free and maximal —
/// takes an on-axis pierce and a pierce whose ball is centred ON the
/// wall's seam plane, and refuses every other position at the
/// split-join's role resolution (`SectionLoopMixed` on a wall half);
/// the repaired boss refuses the same way, and a ball poled along `y`
/// against a `z` cap refuses at the fallback extent. A ball centred on
/// the seam plane puts the ring's closest approach at a sample azimuth
/// of the rim, where the screen reads the exact gap — so the pip the
/// filed item proposes cannot reach the backstop through this door.
/// Recorded, not asserted: a boolean that widens is not this unit's
/// business.
#[test]
fn r1_diag_cylinder_pierces() {
    let cyl = prism(
        vec![
            ProfileVertex::new(Point2::new(1.0, 0.0), 1.0),
            ProfileVertex::new(Point2::new(-1.0, 0.0), 1.0),
        ],
        1.0,
        tol(),
    );
    let try_cut = |name: &str, a: &Body<f64>, ball: Body<f64>| {
        let r = boolean_op_with(
            BooleanOp::Subtract,
            a,
            &ball,
            &BooleanDeclarations::none(),
            SweepStrategy::Realized,
            tol(),
        );
        match r {
            Ok(_) => println!("[r1] {name}: OK"),
            Err(e) => println!("[r1] {name}: {e:?}"),
        }
    };
    let phi = 33.75f64.to_radians();
    try_cut(
        "cyl, z-poled ball on axis",
        &cyl,
        ball_poled_z(0.16, Vec3::new(0.0, 0.0, 1.0), tol()),
    );
    try_cut(
        "cyl, z-poled ball off axis 0.75 @33.75",
        &cyl,
        ball_poled_z(
            0.16,
            Vec3::new(0.75 * phi.cos(), 0.75 * phi.sin(), 1.0),
            tol(),
        ),
    );
    try_cut(
        "cyl, z-poled ball off axis 0.5 @0",
        &cyl,
        ball_poled_z(0.16, Vec3::new(0.5, 0.0, 1.0), tol()),
    );
    try_cut(
        "cyl, z-poled ball off axis 0.5 @90",
        &cyl,
        ball_poled_z(0.16, Vec3::new(0.0, 0.5, 1.0), tol()),
    );
    try_cut(
        "cyl, y-poled ball off axis 0.75 @33.75",
        &cyl,
        ball_poled_y(0.16, Vec3::new(0.75 * phi.cos(), 0.75 * phi.sin(), 1.0)),
    );
    let boss_b = repaired(true);
    try_cut(
        "repaired boss, y-poled ball off axis 0.75 @33.75",
        &boss_b,
        ball_poled_y(0.16, Vec3::new(0.75 * phi.cos(), 1.0, 0.75 * phi.sin())),
    );
    let mut cylr = revolved_about_y(
        vec![
            ProfileVertex::new(Point2::new(0.0, 0.0), 0.0),
            ProfileVertex::new(Point2::new(1.0, 0.0), 0.0),
            ProfileVertex::new(Point2::new(1.0, 1.0), 0.0),
            ProfileVertex::new(Point2::new(0.0, 1.0), 0.0),
        ],
        Revolution::Full,
        tol(),
    );
    cylr.merge_coplanar_faces(tol()).unwrap();
    try_cut(
        "repaired revolve cylinder, y-poled ball on axis",
        &cylr,
        ball_poled_y(0.16, Vec3::new(0.0, 1.0, 0.0)),
    );
    try_cut(
        "repaired revolve cylinder, y-poled ball off axis 0.75 @33.75",
        &cylr,
        ball_poled_y(0.16, Vec3::new(0.75 * phi.cos(), 1.0, 0.75 * phi.sin())),
    );
}

// ------------------------------------------------------------------
// C4: the screen's reading against the exact form, bit by bit.
// ------------------------------------------------------------------

/// Prints the refusing predicate and the reading's bits on the two
/// coaxial refusing fixtures, so a run with the screen disabled (a
/// local mutant) shows whether the exact form's reading is the SAME
/// double as the screen's. Measured at the frozen head: the annulus
/// pair IS bit-identical (`0.9 − 0.92` both ways); the ladder pair is
/// NOT — the screen's `(rr − 0.5) − (√0.35 − 0.5)` and the pass's
/// `rr − si` differ by 16 ulps (`…1bd0` vs `…1be0`).
#[test]
fn r1_print_the_coaxial_refusal_readings() {
    let q = (core::f64::consts::FRAC_PI_2 / 4.0).tan();
    let v = |x: f64, y: f64, b: f64| ProfileVertex::new(Point2::new(x, y), b);
    let mut narrowed = revolved_about_y(
        vec![
            v(0.0, 0.0, 0.0),
            v(0.55, 0.0, 0.0),
            v(0.55, 1.0, 0.0),
            v(0.5, 1.0, q),
            v(0.0, 1.5, 0.0),
        ],
        Revolution::Full,
        tol(),
    );
    narrowed.merge_coplanar_faces(tol()).unwrap();
    let mut domed = revolved_about_y(
        vec![
            v(0.0, 0.0, 0.0),
            v(1.0, 0.0, 0.0),
            v(1.0, 1.0, 0.0),
            v(0.92, 1.0, q),
            v(0.0, 1.92, 0.0),
        ],
        Revolution::Full,
        tol(),
    );
    domed.merge_coplanar_faces(tol()).unwrap();
    for (name, body, rim) in [
        ("narrowed 0.55, dome rim", &narrowed, (0.5, 1.0)),
        ("domed 0.92, top rim", &domed, (1.0, 1.0)),
    ] {
        let arcs = rim_arcs_at(body, rim.0, rim.1);
        let (p, s, v) = refusal(
            fillet_edges(body, &arcs, 0.1, tol())
                .expect_err("refuses")
                .error,
        );
        println!("[r1] {name}: {p} {s:?} {v:?} bits={:#018x}", v.to_bits());
    }
    println!(
        "[r1] derived: 0.55-sqrt(0.35) = {:?} bits={:#018x}; 0.9-0.92 = {:?} bits={:#018x}",
        0.55 - 0.35f64.sqrt(),
        (0.55 - 0.35f64.sqrt()).to_bits(),
        0.9 - 0.92,
        (0.9f64 - 0.92).to_bits()
    );
}

// ------------------------------------------------------------------
// C6: the K-stream cost, counted.
// ------------------------------------------------------------------

#[cfg(feature = "probe")]
mod recorded {
    use geom_core::k_stats::{self, Probe};
    use geom_core::{Point2, Tol};
    use profile::ProfileVertex;
    use sweep::blend::build::fillet_edges;
    use sweep::test_support::{revolved_about_y_at, rim_arcs_at};
    use topo::Body;

    fn boss() -> Body<Probe> {
        let q = (core::f64::consts::FRAC_PI_2 / 4.0).tan();
        let p =
            |x: f64, y: f64, b: f64| ProfileVertex::new(Point2::new(Probe(x), Probe(y)), Probe(b));
        let mut b = revolved_about_y_at(
            vec![
                p(0.0, 0.0, 0.0),
                p(1.0, 0.0, 0.0),
                p(1.0, 1.0, 0.0),
                p(0.5, 1.0, q),
                p(0.0, 1.5, 0.0),
            ],
            sweep::Revolution::Full,
            Tol::witness(),
        );
        b.merge_coplanar_faces(Tol::witness()).unwrap();
        b
    }

    /// **`fillet3_ring_clearance` decisions per carve on the boss's
    /// three rims**: base rim (hostless, ring-free host) 0; top outer
    /// rim (hostless annulus, one ring) 1; dome rim (ladder, no other
    /// ring, a two-arc circular outer cycle) 2 — one per ring per
    /// touched host plus one per outer-cycle edge of a ladder rim's
    /// host, and `circle_margins` mints no sample of its own.
    #[test]
    fn r1_ring_clearance_decisions_per_carve() {
        for (rim, want) in [((1.0, 0.0), 0usize), ((1.0, 1.0), 1), ((0.5, 1.0), 2)] {
            let body = boss();
            let arcs = rim_arcs_at(&body, rim.0, rim.1);
            k_stats::start_recording();
            fillet_edges(&body, &arcs, Probe(0.1), Tol::witness()).expect("carves");
            let samples = k_stats::take_samples();
            let rings = samples
                .iter()
                .filter(|s| s.predicate == "fillet3_ring_clearance")
                .count();
            println!(
                "[r1] rim {rim:?}: {} samples, {rings} fillet3_ring_clearance",
                samples.len()
            );
            assert_eq!(rings, want, "rim {rim:?}");
        }
    }
}
