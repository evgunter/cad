//! **The axial door's TORUS arm**: the offset-axial reduction one kind
//! wider, and what it does not reach.
//!
//! The claim is not "it hollows". A corner solved to the wrong point
//! still produces a valid two-shell body, so every row here carries the
//! CLOSED FORM of the corner it moved — a circle–line intersection at
//! the offset radii, on dyadic inputs — and asserts the solved `(ρ, h)`
//! against it. The refused-form gap each fixture used to produce is
//! quoted beside it, so a row says what changed as well as what is.
//!
//! The last rows are the boundary, measured on BOTH kinds — and the
//! scope is narrower than "curved walls". It is walls whose PROFILE
//! CONSTRAINT IS A CIRCLE. A partial revolve whose wall is a CYLINDER
//! hollows today: `sf2b_axial`'s quarter-turn wedge is exactly that
//! body, and its rim vertex carries TWO profile constraints (the
//! cylinder's line and the cap's), with the meridian plane supplying
//! only the azimuth. A sphere or torus wall whose profile is the whole
//! closed circle carries ONE — and the two kinds now part ways at the
//! rim's CARRIER (VERBS-RIMCAP):
//!
//! - the SPHERE lune's rim solves end to end: the meridian-pair arm
//!   places its pole corners on the moved caps' meeting line, the
//!   carried-datum arm covers the off-axis one-cap shape, and the
//!   off-axis-circle mint carries the moved rim (a plane cuts a
//!   sphere in a circle, always). What still stands between the lune
//!   and `shell` is the PROPS inventory — the sphere flux arm's
//!   `props_band_coplanar` premise refuses the OPERAND's own wall
//!   today — and that wall is pinned with its payload below;
//! - the KLEIN ELBOW (torus wall) now refuses one door deeper than
//!   its old `TogetherAxialCorner`: its corners solve through the
//!   carried-datum arm, and the rim EDGE has no carrier — the moved
//!   cap cuts the torus in a spiric QUARTIC, and the latitude mint
//!   names the off-axis centre it will not carry. The torus half is
//!   design-gated (the spec's PR-2 conversation), not implemented.
//!
//! So the partial revolve's rim was a circle-profile wall's gap at two
//! doors; the sphere door is built, and the torus door is the spiric
//! carrier's.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Surface;
use geom_core::{Affine3, Band, Point2, Point3, Tol, Vec2, Vec3};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::{Revolution, RevolveAxis, revolve};
use topo::{Body, FaceKey, LoopBoundary, ShellError, VertexKey, transform_rigid};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

fn tol() -> Tol {
    Tol::witness()
}

const FIT_TOL: f64 = 1e-6;

/// The wall thickness every accepting row here hollows by — the tour's
/// own.
const T: f64 = 1.0 / 128.0;

/// **How far the transcript's refused-form gaps and this file's
/// re-derivations may differ, and why one bound covers both.**
///
/// The two quoted numbers are the DOOR's own — the `gap` payload the
/// per-chart re-anchor produced through its whole evaluation chain,
/// copied out of the C5ARMS STOP transcript. The rows re-derive them
/// from the closed form as `5/64 − ‖·‖`, which is a shorter chain, so
/// they agree as REAL NUMBERS and not bit-for-bit, and the residue is
/// per-operand. MEASURED at this head:
///
/// | fixture | re-derived | transcript | ulps | relative |
/// |---|---|---|---|---|
/// | barrel | `6.09730892739932e-3` | `6.0973089273993215e-3` | 2 | `−2.85e-16` |
/// | teapot | `4.422022405807788e-3` | `4.422022405807807e-3` | 22 | `−4.32e-15` |
///
/// **`hypot` is not the cause**, which an earlier note here claimed: a
/// bare `√(x² + y²)` on these two operand pairs returns the SAME BITS
/// as `hypot` — checked, both fixtures, `3f78f97de6fd1660` and
/// `3f721cd399d78bf0`. What differs is the length of the chain the
/// door took to the same real number. One bound, set a factor of two
/// above the larger measured residue, therefore replaces the two
/// unexplained per-row tolerances this file used to carry.
const GAP_REL: f64 = 1e-14;

/// Revolved about the `y` axis through the origin, so a vertex's axial
/// coordinates are `(hypot(x, z), y)`.
fn revolved(lp: ProfileLoop<f64>, turn: Revolution<f64>) -> Body<f64> {
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(tol())
        .expect("the meridian validates");
    revolve(
        &profile,
        RevolveAxis {
            origin: p2(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        turn,
        tol(),
    )
    .expect("the meridian revolves")
    .body
}

/// The bulge (`tan(θ/4)`) of the arc from `a` to `b` about `c`.
fn bulge(a: Point2<f64>, b: Point2<f64>, c: Point2<f64>) -> f64 {
    let (u, v) = (a - c, b - c);
    (u.perp_dot(v).atan2(u.dot(v)) / 4.0).tan()
}

/// `p` in the `(ρ, h)` half-plane of the `y` axis.
fn axial(p: Point3<f64>) -> (f64, f64) {
    ((p.x * p.x + p.z * p.z).sqrt(), p.y)
}

/// `body`'s distinct corners in that half-plane.
fn corners(body: &Body<f64>) -> Vec<(f64, f64)> {
    let mut out: Vec<(f64, f64)> = Vec::new();
    for (_, v) in body.vertices() {
        let c = axial(*body.get_point(v.point).expect("a vertex carries a point"));
        if !out
            .iter()
            .any(|q| (q.0 - c.0).abs() < 1e-12 && (q.1 - c.1).abs() < 1e-12)
        {
            out.push(c);
        }
    }
    out
}

/// `(ρ, h)` is among `body`'s corners to `1e-14` m — an ABSOLUTE bound
/// on bodies whose radii are ~5e-2 m, so a relative agreement of about
/// `2e-13`, stated both ways.
fn has_corner(body: &Body<f64>, rho: f64, h: f64, what: &str) {
    let all = corners(body);
    assert!(
        all.iter()
            .any(|q| (q.0 - rho).abs() <= 1e-14 && (q.1 - h).abs() <= 1e-14),
        "{what}: no corner at the closed form (ρ, h) = ({rho}, {h}); the body has {all:?}"
    );
}

/// The sealed hollow, with tier 3 and the two-shell shape first.
fn hollowed(what: &str, body: &Body<f64>) -> Body<f64> {
    let out = topo::shell(body, T, FIT_TOL, tol())
        .unwrap_or_else(|e| panic!("{what}: the axial door must hollow this, got {e}"));
    assert_eq!(
        topo::validate_geometric(&out, tol()),
        Ok(()),
        "{what}: tier 3"
    );
    assert_eq!(out.shells().count(), 2, "{what}: outer + cavity");
    out
}

/// Every vertex on `face`'s own boundary.
fn face_vertices(body: &Body<f64>, face: FaceKey) -> Vec<VertexKey> {
    let data = body.get_face(face).expect("face");
    let mut out = Vec::new();
    for lk in core::iter::once(data.outer).chain(data.rings.iter().copied()) {
        let LoopBoundary::Cycle { first } = body.get_loop(lk).expect("loop").boundary else {
            continue;
        };
        for he in body.loop_cycle(first).expect("cycle") {
            out.push(body.get_half_edge(he).expect("half edge").start);
        }
    }
    out
}

/// A point's signed distance from an analytic surface, in the `y`-axis
/// frame these fixtures share.
fn residual(s: &Surface<f64>, p: Point3<f64>) -> f64 {
    let (rho, h) = axial(p);
    match s {
        Surface::Plane { origin, normal, .. } => normal.dot(p - *origin),
        Surface::Cylinder { radius, .. } => rho - *radius,
        Surface::Sphere { center, radius, .. } => p.distance(*center) - *radius,
        Surface::Torus {
            center,
            major_radius,
            minor_radius,
            ..
        } => (rho - *major_radius).hypot(h - center.y) - *minor_radius,
        other => panic!("no fixture here carries a {other:?}"),
    }
}

// ---------------------------------------------------------------------
// The two full-revolve consumers
// ---------------------------------------------------------------------

/// **The barrel bulged about a centre OFF the axis.** The same two
/// junction stations and the same `5/64` meridian radius as the tour's
/// sphere-zone barrel, about the OTHER centre on their perpendicular
/// bisector — so the wall is a TORUS: `R = 6/64`, `r = 5/64`,
/// `h_c = 4/64`, a 3-4-5 at each junction with both residuals exactly
/// zero.
fn torus_barrel() -> Body<f64> {
    let c = p2(6.0 / 64.0, 1.0 / 16.0);
    let (lo, hi) = (p2(3.0 / 64.0, 0.0), p2(3.0 / 64.0, 8.0 / 64.0));
    revolved(
        RawLoop::new(vec![
            ProfileVertex::new(p2(0.0, 0.0), 0.0),
            ProfileVertex::new(lo, bulge(lo, hi, c)),
            ProfileVertex::new(hi, 0.0),
            ProfileVertex::new(p2(0.0, 8.0 / 64.0), 0.0),
        ]),
        Revolution::Full,
    )
}

/// **The teapot's wall-1 belly.** The pot's own foot and mouth, its
/// belly bulged about `(7/64, 5/64)` — off the axis, so a TORUS with
/// `R = 7/64`, `r = 5/64`, `h_c = 5/64`.
fn torus_belly() -> Body<f64> {
    let c = p2(7.0 / 64.0, 5.0 / 64.0);
    let (lo, hi) = (p2(4.0 / 64.0, 1.0 / 64.0), p2(3.0 / 64.0, 8.0 / 64.0));
    revolved(
        RawLoop::new(vec![
            ProfileVertex::new(p2(0.0, 0.0), 0.0),
            ProfileVertex::new(p2(4.0 / 64.0, 0.0), 0.0),
            ProfileVertex::new(lo, bulge(lo, hi, c)),
            ProfileVertex::new(hi, 0.0),
            ProfileVertex::new(p2(0.0, 8.0 / 64.0), 0.0),
        ]),
        Revolution::Full,
    )
}

/// **The barrel's cap × wall corners solve as Station × torus-circle
/// roots, in closed form.**
///
/// Both caps move inward by `T` to the stations `1/128` and `15/128`.
/// The wall's arc runs CW about a centre OUTSIDE it, so the tube's
/// material side faces the axis and an inward wall moves the minor
/// radius OUT, to `r + T = 11/128`, about the UNMOVED tube centre
/// `(R, h_c) = (12/128, 8/128)`. Each station stands `7/128` from
/// `h_c`, so the circle's half-chord there is
/// `√(11² − 7²)/128 = √72/128 = 6√2/128`, and the root a small offset
/// keeps is the INNER one:
///
/// ```text
/// ρ = (12 − 6√2)/128 = 3(2 − √2)/64     h ∈ {1/128, 15/128}
/// ```
///
/// The refused form this replaces left the corner on the UNMOVED
/// meridian circle, `5/64 − hypot(3/64 − 6/64, 1/128 − 1/16)` =
/// `6.0973089273993215e-3` m off its own carrier — nine orders above ε,
/// and the number the per-chart loop refused on.
#[test]
fn torax_the_torus_barrel_corners_solve_in_closed_form() {
    let out = hollowed("the torus barrel", &torus_barrel());
    let rho = 3.0 * (2.0 - 2.0f64.sqrt()) / 64.0;
    has_corner(&out, rho, 1.0 / 128.0, "the barrel's base junction");
    has_corner(&out, rho, 15.0 / 128.0, "the barrel's mouth junction");

    let gap = 5.0 / 64.0 - (3.0f64 / 64.0 - 6.0 / 64.0).hypot(1.0 / 128.0 - 1.0 / 16.0);
    assert!(
        (gap - 6.0973089273993215e-3).abs() <= GAP_REL * gap,
        "the C5ARMS STOP transcript's barrel gap, got {gap}"
    );
}

/// **The teapot's wall-1 corners, one of each shape.**
///
/// Same waisted sense, so the minor radius again moves out to
/// `r + T = 11/128` about `(R, h_c) = (14/128, 10/128)`. The foot
/// junction is a CYLINDER × torus-circle root: the foot wall moves to
/// `ρ = 7/128`, which stands `7/128` from `R`, so
/// `h = (10 − √72)/128 = (10 − 6√2)/128`. The mouth junction is a
/// Station × torus-circle root at `h = 15/128`, `5/128` from `h_c`, so
/// `ρ = (14 − √96)/128 = (7 − 2√6)/64`.
///
/// The refused form, quoted:
/// `5/64 − hypot(3/64 − 7/64, 8/64 − 1/128 − 5/64)` =
/// `4.422022405807807e-3` m.
#[test]
fn torax_the_teapot_belly_corners_solve_in_closed_form() {
    let out = hollowed("the teapot's torus belly", &torus_belly());
    has_corner(
        &out,
        7.0 / 128.0,
        (10.0 - 6.0 * 2.0f64.sqrt()) / 128.0,
        "the belly's foot junction",
    );
    has_corner(
        &out,
        (7.0 - 2.0 * 6.0f64.sqrt()) / 64.0,
        15.0 / 128.0,
        "the belly's mouth junction",
    );

    let gap =
        5.0 / 64.0 - (3.0f64 / 64.0 - 7.0 / 64.0).hypot(8.0 / 64.0 - 1.0 / 128.0 - 5.0 / 64.0);
    assert!(
        (gap - 4.422022405807807e-3).abs() <= GAP_REL * gap,
        "the C5ARMS STOP transcript's teapot gap, got {gap}"
    );
}

/// **Every corner lands on BOTH its own moved surfaces.**
///
/// The closed forms say where a corner is; this says it is ON the
/// geometry it belongs to, which is the claim a wrong corner breaks
/// without any tier noticing. Read from the outside: each face's own
/// boundary vertices are metered against that face's surface.
#[test]
fn torax_every_torus_corner_lies_on_its_own_moved_surfaces() {
    for (what, body) in [
        ("the torus barrel", torus_barrel()),
        ("the teapot's torus belly", torus_belly()),
    ] {
        let out = hollowed(what, &body);
        let mut metered = 0usize;
        for (face, f) in out.faces() {
            let surface = out
                .get_surface(f.surface)
                .expect("a face carries a surface");
            for v in face_vertices(&out, face) {
                let p = *out
                    .get_point(out.get_vertex(v).expect("vertex").point)
                    .expect("point");
                let d = residual(surface, p);
                assert!(
                    d.abs() <= 1e-15,
                    "{what}: {v:?} stands {d} m off {face:?}'s own surface"
                );
                metered += 1;
            }
        }
        assert!(metered >= 16, "{what}: only {metered} incidences metered");
    }
}

/// **The corner solve reads no global axis.** Hollowing and re-posing
/// commute: the same body posed by a rigid motion hollows to the same
/// points, compared in the posed frame so no inverse is invented.
///
/// **The pose carries a TRANSLATION, and that is the point of it.** The
/// door canonicalizes its axis point to the axis line's own foot at the
/// world origin (`offset_axial`'s `axial_frame`), and a rotation ABOUT
/// THE ORIGIN leaves that foot at the origin — every station is
/// unchanged and the canonicalization is never asked to do anything. So
/// the anchor here is `(1/4, −1/2, 1/8)`, dyadic and off the axis: the
/// posed body's axis line misses the world origin, its foot is a
/// computed point, and every station is a different number from the
/// unposed body's.
///
/// The comparison is a BIJECTION, not a subset with a matching length —
/// a solve that collapsed two corners onto one point and invented a
/// third elsewhere would pass a subset check with the right count. Each
/// wanted point consumes its nearest unused match, and the pool must
/// empty. MEASURED at this head: worst matched distance `2.9e-17` m on
/// the barrel, `2.1e-17` m on the belly — sub-ulp at these coordinates,
/// so the bound is the file's own tightest (`1e-15`) rather than the
/// `1e-12` this row was first written with.
#[test]
fn torax_the_torus_corners_survive_a_rigid_re_pose() {
    let map = Affine3::rotation_about_axis(
        Point3::new(0.25, -0.5, 0.125),
        Vec3::new(1.0, 0.0, 0.0),
        0.7,
    );
    for (what, body) in [
        ("the torus barrel", torus_barrel()),
        ("the teapot's torus belly", torus_belly()),
    ] {
        let posed_after = transform_rigid(&hollowed(what, &body), &map, tol())
            .unwrap_or_else(|e| panic!("{what}: the hollow re-poses, got {e}"));
        let posed_first = transform_rigid(&body, &map, tol())
            .unwrap_or_else(|e| panic!("{what}: the operand re-poses, got {e}"));
        let hollow_after = topo::shell(&posed_first, T, FIT_TOL, tol())
            .unwrap_or_else(|e| panic!("{what}, re-posed: {e}"));
        let want: Vec<Point3<f64>> = posed_after
            .vertices()
            .map(|(_, v)| *posed_after.get_point(v.point).expect("point"))
            .collect();
        let mut pool: Vec<Point3<f64>> = hollow_after
            .vertices()
            .map(|(_, v)| *hollow_after.get_point(v.point).expect("point"))
            .collect();
        assert_eq!(want.len(), pool.len(), "{what}: vertex count under re-pose");
        for w in &want {
            let (i, d) = pool
                .iter()
                .enumerate()
                .map(|(i, g)| (i, (*g - *w).norm()))
                .min_by(|a, b| a.1.total_cmp(&b.1))
                .expect("the pool is non-empty while wanted points remain");
            assert!(
                d <= 1e-15,
                "{what}: hollow-then-pose has {w:?}; the nearest unmatched \
                 pose-then-hollow point is {d} m away"
            );
            pool.remove(i);
        }
        assert!(
            pool.is_empty(),
            "{what}: the match is not a bijection — {} pose-then-hollow points \
             are unclaimed",
            pool.len()
        );
    }
}

/// **The operand gate, named — and named as NOT this arm's floor.** A
/// wall thicker than the tube is refused before any torus arithmetic
/// happens: `shell`'s `wall_clearance` sees the two planar caps facing
/// each other across `1/8` m of material while two walls need `3/16`,
/// and says so. That gate is KIND-AGNOSTIC — a box of the same height
/// refuses identically — so this row is a check that the operand door
/// still fires and still names both numbers, and it is deliberately
/// NOT evidence about the torus arm. The arm's own floor is the row
/// below.
#[test]
fn torax_a_wall_thicker_than_the_tube_refuses_typed() {
    let e = topo::shell(&torus_barrel(), 6.0 / 64.0, FIT_TOL, tol())
        .expect_err("a wall thicker than the tube has no cavity");
    println!("[torax] the over-thick wall refuses: {e}");
    let ShellError::WallClearance {
        gap,
        needed,
        face,
        other,
    } = e
    else {
        panic!("the over-thick wall is the operand clearance gate's, got {e}");
    };
    assert_ne!(face, other, "the gate names two distinct faces");
    assert!(
        (gap - 0.125).abs() <= 1e-15 && (needed - 0.1875).abs() <= 1e-15,
        "the caps stand 1/8 m apart and two walls of 6/64 need 3/16, got \
         gap {gap} needed {needed}"
    );
}

/// **The torus arm's OWN floor: the ring closing.** An inward wall on
/// these waisted fixtures moves the minor radius OUT, so the ring
/// convention `R > r > 0` is what eventually stops them — not the
/// operand's clearance gate, which has decades of room left. The mint
/// refuses [`geom_brep::OffsetError::TorusRing`], and this row brackets
/// the floor rather than asserting one side of it: `1/128` hollows,
/// `2/128` does not, and the realized minor radius the refusal echoes
/// is the tube's own major radius `R = 12/128`.
///
/// **This row is the one that is sensitive to the arm.** The refusal is
/// only reachable BECAUSE the axial door now takes a torus body: with
/// the `(Torus, Torus)` arm removed from `classify`, `is_axial` answers
/// `false` and `shell` falls to the per-chart loop — which, at the time
/// this was measured, refused at the C5 table's
/// `NeighborPairUnroutable(Plane, Torus)`. VERBS-C5ARMS has since
/// routed the pair, so the same counterfactual would now proceed one
/// door deeper and refuse at the per-chart reanchor gate — the
/// corner-accumulation family `offd2_r1_probes` pins at `8.331e-4` m
/// on the elbow cap — never through the mint. Nothing else in this
/// suite refuses through the mint.
#[test]
fn torax_the_torus_arms_floor_is_the_ring_closing() {
    // Below the floor: the same fixture the closed-form rows use.
    hollowed("the torus barrel at t = 1/128", &torus_barrel());

    // At it. `r + t` reaches `R`, so the offset tube would swallow its
    // own hole.
    let e = topo::shell(&torus_barrel(), 2.0 / 128.0, FIT_TOL, tol())
        .expect_err("an offset tube that reaches the major radius has no ring left");
    println!("[torax] the closed ring refuses: {e:?}");
    let ShellError::Face { error, .. } = e else {
        panic!("not the offset door's refusal: {e}");
    };
    let topo::ReplaceFaceError::Offset { error, .. } = *error else {
        panic!("the ring floor is the surface mint's, not a corner's: {error}");
    };
    let geom_brep::OffsetError::TorusRing { realized_minor } = error else {
        panic!("the mint must name the ring convention, got {error}");
    };
    assert!(
        (realized_minor - 12.0 / 128.0).abs() <= 1e-15,
        "the realized minor radius reaches R = 12/128 exactly, got {realized_minor}"
    );
}

// ---------------------------------------------------------------------
// The boundary: the partial revolve's rim, on both kinds (VERBS-RIMCAP)
// ---------------------------------------------------------------------

/// A partial revolve of a half-disc whose diameter lies on the axis: a
/// sphere wall, two meridian caps, poles on the axis. `turn` is the
/// revolve angle; the quarter turn is this file's acceptance fixture
/// and the half turn is the meridian-pair arm's refusing side.
fn lune(r: f64, turn: f64) -> Body<f64> {
    let turn = Revolution::Partial(turn);
    let profile = Profile::new(
        SketchPlane::xy(),
        vec![ProfileLoop::new(vec![
            ProfileVertex::new(p2(0.0, -r), 0.0),
            ProfileVertex::new(p2(0.0, r), -1.0),
        ])],
    )
    .validate(tol())
    .expect("the lune's cross-section validates");
    revolve(
        &profile,
        RevolveAxis {
            origin: p2(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        turn,
        tol(),
    )
    .expect("the lune revolves")
    .body
}

/// Every chart of `body` moved inward by `t` through the simultaneous
/// door — the same moves `shell` builds, spelled at the door itself.
fn hollow_moves(body: &Body<f64>, t: f64) -> Vec<topo::ChartMove<f64>> {
    let mut charts: Vec<(topo::SurfaceKey, Vec<FaceKey>)> = Vec::new();
    for (k, f) in body.faces() {
        match charts.iter_mut().find(|(s, _)| *s == f.surface) {
            Some((_, v)) => v.push(k),
            None => charts.push((f.surface, vec![k])),
        }
    }
    charts
        .into_iter()
        .map(|(_, faces)| {
            let sense = body.get_face(faces[0]).expect("face").sense;
            topo::ChartMove {
                faces,
                distance: if sense { -t } else { t },
            }
        })
        .collect()
}

/// **The klein elbow's rim now stops one door deeper, and the boundary
/// is the CARRIER, not the corner.**
///
/// Until VERBS-RIMCAP this row pinned `TogetherAxialCorner { surfaces:
/// 2, what: "one profile constraint…" }`: the rim vertex meets the
/// torus wall and ONE meridian cap, and the corner solve had no arm
/// for a lone profile circle off the axis. The carried-datum arm now
/// answers that corner — the old corner's profile point moved
/// concentrically with its circle, the azimuth solved from the moved
/// cap exactly as the wedge's is — so the corner SOLVES, and the
/// refusal moves to the rim EDGE's carrier: the old rim is the profile
/// circle in the cap's plane, centred `R` off the axis, and the
/// latitude mint's own predicate says so. The payload below is the
/// measured door at this head.
///
/// **What stands between the elbow and a hollow is the torus half's
/// own boundary, stated rather than glossed**: the moved cap stands
/// `t` off the axis and parallel to it, and a plane in that posture
/// cuts a torus in a SPIRIC quartic — sampled below as the oval's own
/// half-width against its half-height, which a circle would make
/// equal. `Curve3` has no quartic carrier, so the sphere half's
/// off-axis-circle mint has no torus sibling to gain here; that half
/// is design-gated (the VERBS-RIMCAP spec's PR-2 conversation), and
/// the klein rows stay measured-red until it is funded.
#[test]
fn torax_the_klein_elbow_rim_refuses_at_the_carrier_mint() {
    let r = 0.275_f64;
    let elbow = {
        let profile = Profile::new(
            SketchPlane::xy(),
            vec![ProfileLoop::new(vec![
                ProfileVertex::new(p2(-r, 0.0), 1.0),
                ProfileVertex::new(p2(r, 0.0), 1.0),
            ])],
        )
        .validate(tol())
        .expect("the elbow's cross-section validates");
        revolve(
            &profile,
            RevolveAxis {
                origin: p2(1.2, 0.0),
                dir: Vec2::new(0.0, -1.0),
            },
            Revolution::Partial(-core::f64::consts::FRAC_PI_2),
            tol(),
        )
        .expect("the elbow revolves")
        .body
    };
    let e = topo::shell(&elbow, 0.05, FIT_TOL, tol())
        .expect_err("the elbow's rim circle is a quartic section away from a carrier");
    let ShellError::Face { error, .. } = e else {
        panic!("not the offset door's refusal: {e}");
    };
    let topo::ReplaceFaceError::TogetherAxialEdge { what, .. } = *error else {
        panic!("the rim must refuse at the carrier it cannot mint: {error:?}");
    };
    assert_eq!(
        what, "a circular edge between two charts whose centre is off the axis",
        "the latitude mint names the off-axis centre"
    );
    println!("[torax] the elbow rim, one door deeper: {what}");

    // The section that rim edge would need is not a circle.
    let (big, small) = (1.2_f64, r - 0.05);
    let d = 0.05_f64;
    let out = ((big + small).powi(2) - d * d).sqrt();
    let inn = ((big - small).powi(2) - d * d).sqrt();
    let half_width = (out - inn) / 2.0;
    let half_height = small;
    assert!(
        (half_width - half_height).abs() > 1e-6,
        "a plane parallel to the axis at {d} m cuts this torus in a quartic, not a circle: \
         {half_width} vs {half_height}"
    );
    println!(
        "[torax] the moved rim section is spiric: half-width {half_width}, half-height \
         {half_height}, difference {}",
        half_width - half_height
    );
}

/// **The sphere lune's rim SOLVES through the axial door, in closed
/// form** — the row that used to pin its refusal, flipped, with the
/// old door recorded.
///
/// **The old door, verbatim (measured at the unit's head before the
/// fix):** `TogetherEdgeDisagreement { edge: EdgeKey(1v1), gap: 0.05 }`
/// — the AXIS edge between the two caps, raised at the
/// `offset_axial_edge_on_surface` meter: the pole arm had answered both
/// corners ON the axis (`(ρ, h) = (0, ∓0.25)`) while both moved caps
/// stood exactly `t = 0.05` m off it, so the minted axis line's
/// midpoint missed each moved cap by the whole wall thickness. The rim
/// great circles behind it fell to the latitude mint's plane predicate
/// ("whose plane is not normal to the axis") — two mechanisms, one
/// displacement: the moved caps stop containing the axis.
///
/// **What answers now.** The two moved caps meet in a line parallel to
/// the axis at `ρ_L = t·√2` (their common perpendicular splits the
/// quarter turn's right angle), and the corner is that line against
/// the moved profile circle:
///
/// ```text
/// ρ = t√2          h = ±√((r − t)² − 2t²)
/// ```
///
/// with the rim carrier the moved cap's own plane∩sphere section —
/// centre `t` along the cap normal, radius `√((r − t)² − t²)` — and
/// the axis edge's carrier the caps' meeting line itself, which the
/// existing line arm mints once the corners are right. Every number
/// below is asserted against those closed forms on dyadic-input
/// arithmetic.
///
/// **Why this row is the DIRECT door and not `shell`.** The whole
/// hollow pipeline now runs — corners, carriers, parameters, pcurves,
/// the void door's containment — and `shell`'s LAST act, tier 3's +V
/// invariant, refuses: the sphere flux closed form's
/// `props_band_coplanar` premise covers only meridians on ONE great
/// circle, and the lune's wall (the OPERAND's own wall, today, before
/// any offset) has meridians on two. That standing wall is pinned with
/// its payload by `torax_the_sphere_lune_next_door_is_the_props_inventory`
/// below; this row pins what this unit built.
///
/// **The cavity's closed-form volume, derived here for the day the
/// props inventory reaches it.** The cavity is the ball of radius
/// `R = r − t` cut by two perpendicular planes each `a = t` from its
/// centre, on the inner side of both (the wedge's `two_chord_area`
/// story, one dimension up). Integrating the two-chord disc sections
/// along the axis — or by parts against the segment areas — with
/// `Q = √(R² − 2a²)`:
///
/// ```text
/// V = (2/3)·[ R³·atan(R·Q/a²) − a·(3R² − a²)·atan(Q/a) + a²·Q ]
/// ```
///
/// (`a → 0` recovers the quarter ball `πR³/3`; verified against direct
/// quadrature of the section areas to 13 significant figures at these
/// inputs, `V = 7.909058628579758e-3` m³, so the hollow wall is
/// `πr³/3 − V = 2.0365275253728374e-2` m³.)
#[test]
fn torax_the_sphere_lune_rim_solves_in_closed_form() {
    let (r, t) = (0.3_f64, 0.05_f64);
    let body = lune(r, core::f64::consts::FRAC_PI_2);
    let mut cavity = body.clone();
    let band = Band::linear(tol()).expect("band");
    topo::offset_charts_together(&mut cavity, &hollow_moves(&body, t), band, tol())
        .expect("the meridian-pair arm and the off-axis mint answer the lune's rim");

    // The corners, at the closed form.
    let rho = t * 2.0_f64.sqrt();
    let h = ((r - t).powi(2) - 2.0 * t * t).sqrt();
    has_corner(&cavity, rho, h, "the lune's upper rim corner");
    has_corner(&cavity, rho, -h, "the lune's lower rim corner");

    // The carriers: two off-axis rim circles and the caps' meeting
    // line — kinds, centres, radii, all against the closed forms.
    let rim_radius = ((r - t).powi(2) - t * t).sqrt();
    let (mut lines, mut rims) = (0usize, 0usize);
    for (_, d) in cavity.edges() {
        let c = cavity
            .get_curve_geom(d.curve)
            .and_then(|g| g.certified())
            .expect("a certified carrier");
        match c.carrier() {
            geom::Curve3::Line { origin, dir } => {
                lines += 1;
                let (rho_l, _) = axial(*origin);
                assert!(
                    (rho_l - rho).abs() <= 1e-15,
                    "the axis edge moved to the caps' meeting line at ρ = t√2, got {rho_l}"
                );
                assert!(
                    (dir.x.abs() + dir.z.abs()) <= 1e-15 && (dir.y.abs() - 1.0).abs() <= 1e-15,
                    "the meeting line stays parallel to the axis, got {dir:?}"
                );
            }
            geom::Curve3::Circle { center, radius, .. } => {
                rims += 1;
                let (rho_c, h_c) = axial(*center);
                assert!(
                    (rho_c - t).abs() <= 1e-15 && h_c.abs() <= 1e-15,
                    "a rim centre stands t off the axis at the equator station, got \
                     ({rho_c}, {h_c})"
                );
                assert!(
                    (radius - rim_radius).abs() <= 1e-15,
                    "the rim radius is the section's √((r−t)² − t²), got {radius}"
                );
            }
            other => panic!("no lune edge carries a {other:?}"),
        }
    }
    assert_eq!((lines, rims), (1, 2), "one meeting line, two rim circles");

    // Every corner lies on every moved surface at its own faces — the
    // same outside read the torus rows make.
    for (face, f) in cavity.faces() {
        let surface = cavity.get_surface(f.surface).expect("surface");
        for v in face_vertices(&cavity, face) {
            let p = *cavity
                .get_point(cavity.get_vertex(v).expect("vertex").point)
                .expect("point");
            let d = residual(surface, p);
            assert!(
                d.abs() <= 1e-15,
                "{v:?} stands {d} m off {face:?}'s own moved surface"
            );
        }
    }
}

/// **The lune's corner solve reads no global frame**: offsetting and
/// re-posing commute, compared as a BIJECTION in the posed frame —
/// the torus rows' re-pose rule carried to the sphere rim. The anchor
/// is off the axis and dyadic, so the posed body's stations are all
/// different numbers and the axis foot is a computed point.
#[test]
fn torax_the_lune_cavity_survives_a_rigid_re_pose() {
    let (r, t) = (0.3_f64, 0.05_f64);
    let map = Affine3::rotation_about_axis(
        Point3::new(0.25, -0.5, 0.125),
        Vec3::new(1.0, 0.0, 0.0),
        0.7,
    );
    let body = lune(r, core::f64::consts::FRAC_PI_2);
    let band = Band::linear(tol()).expect("band");

    let mut offset_first = body.clone();
    topo::offset_charts_together(&mut offset_first, &hollow_moves(&body, t), band, tol())
        .expect("the unposed lune offsets");
    let posed_after =
        transform_rigid(&offset_first, &map, tol()).expect("the offset lune re-poses");

    let posed_first = transform_rigid(&body, &map, tol()).expect("the operand re-poses");
    let mut offset_after = posed_first.clone();
    topo::offset_charts_together(
        &mut offset_after,
        &hollow_moves(&posed_first, t),
        band,
        tol(),
    )
    .expect("the posed lune offsets");

    let want: Vec<Point3<f64>> = posed_after
        .vertices()
        .map(|(_, v)| *posed_after.get_point(v.point).expect("point"))
        .collect();
    let mut pool: Vec<Point3<f64>> = offset_after
        .vertices()
        .map(|(_, v)| *offset_after.get_point(v.point).expect("point"))
        .collect();
    assert_eq!(want.len(), pool.len(), "vertex count under re-pose");
    for w in &want {
        let (i, d) = pool
            .iter()
            .enumerate()
            .map(|(i, g)| (i, (*g - *w).norm()))
            .min_by(|a, b| a.1.total_cmp(&b.1))
            .expect("the pool is non-empty while wanted points remain");
        assert!(
            d <= 1e-15,
            "offset-then-pose has {w:?}; the nearest unmatched pose-then-offset point is \
             {d} m away"
        );
        pool.remove(i);
    }
    assert!(pool.is_empty(), "the match is not a bijection");
}

/// **What still stands between the lune and `shell`, named with its
/// payload — and it is the props inventory's, not this rim's.**
///
/// `shell`'s last act is tier 3, whose +V invariant computes the exact
/// B-rep volume, and the sphere flux arm's `props_band_coplanar`
/// premise (all boundary meridians on ONE great circle, `Δu = π`)
/// covers full-revolve bands only. A lune's wall carries meridians on
/// two great circles, so its volume is `VolumeUncomputable` — for the
/// OPERAND, today, before any offset is asked for: both reads below
/// return the SAME payload, which is what places this wall upstream of
/// the unit rather than inside it (D2 addendum row 2: valid input,
/// lane not built — `cross.step`'s standing class). The day the sphere
/// arm measures a lune, this row goes red and the family's acceptance
/// moves to the hollow's closed-form wall volume, derived and parked
/// in `torax_the_sphere_lune_rim_solves_in_closed_form`'s docs.
#[test]
fn torax_the_sphere_lune_next_door_is_the_props_inventory() {
    let body = lune(0.3, core::f64::consts::FRAC_PI_2);

    // The operand's own tier 3, first: the wall predates this unit.
    let operand = topo::validate_geometric(&body, tol())
        .expect_err("the lune's wall volume is outside the sphere flux arm's premise");
    assert!(
        matches!(
            operand[..],
            [topo::ValidationError::VolumeUncomputable {
                source: topo::MassPropsError::Face {
                    source: geom_brep::PropsError::NotIsoRectangle {
                        what: "props_band_coplanar"
                    },
                    ..
                },
            }]
        ),
        "the operand refuses at the sphere flux premise, got {operand:?}"
    );

    // And shell walks the WHOLE hollow — corners, carriers, pcurves,
    // containment — before the same premise refuses its closing gate.
    let e = topo::shell(&body, 0.05, FIT_TOL, tol())
        .expect_err("shell's +V invariant needs the volume the flux arm cannot yet give");
    println!("[torax] the lune's next door: {e}");
    let ShellError::NotValid { errors } = e else {
        panic!("the hollow must reach tier 3 and stop at the props inventory, got {e:?}");
    };
    assert!(
        matches!(
            errors[..],
            [topo::ValidationError::VolumeUncomputable {
                source: topo::MassPropsError::Face {
                    source: geom_brep::PropsError::NotIsoRectangle {
                        what: "props_band_coplanar"
                    },
                    ..
                },
            }]
        ),
        "the same premise, one body later: {errors:?}"
    );
}

/// **The meridian-pair arm's refusing side, on a buildable body**: the
/// HALF-turn lune's caps are two half-planes of one plane, so their
/// inward offsets are parallel and meet in no line — the corner is
/// genuinely under-determined by its surfaces, and the arm says so
/// typed rather than falling back to the pole answer the moved caps
/// have left. This is also the row that keeps the circle-profile rim
/// family's refusal REACHABLE now that the quarter lune solves.
#[test]
fn torax_the_half_turn_lune_refuses_the_parallel_cap_pair() {
    let e = topo::shell(&lune(0.3, core::f64::consts::PI), 0.05, FIT_TOL, tol())
        .expect_err("parallel moved caps leave the rim corner under-determined");
    println!("[torax] the half-turn lune: {e}");
    let ShellError::Face { error, .. } = e else {
        panic!("not the offset door's refusal: {e}");
    };
    let topo::ReplaceFaceError::TogetherAxialCorner { surfaces, what, .. } = *error else {
        panic!("the parallel caps must refuse at the corner they under-determine: {error:?}");
    };
    assert_eq!(surfaces, 3, "sphere wall + two meridian caps");
    assert!(
        what.contains("parallel"),
        "the refusal names the parallel caps, got {what}"
    );
}
