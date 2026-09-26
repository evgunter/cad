//! **The torus doors of a union, up to the join** — the dumbbell with a
//! torus waist, walked door by door.
//!
//! The fixture is two halves of a dumbbell, each a FULL revolve about
//! `y` of one profile: a bell (a `1.5`-radius cylinder between the
//! planes `|y| = 0.5` and `|y| = 1.5`) on a CONCAVE torus waist — the
//! quarter of a tube of `R = 0.8`, `r = 0.5` that runs from the bell's
//! underside at `ρ = 0.8` down to the joint plane `y = 0` at `ρ = 0.3`,
//! where the two halves meet on a `0.3`-radius joint disc. The two
//! waists are one torus carrier and meet tangentially along the joint
//! circle. The union declares the joint discs and every torus×torus
//! pair `Rest`.
//!
//! **The halves are PRE-MERGED** with `Body::merge_coplanar_faces`. A
//! full revolve mints its planar walls split in two along the meridian
//! half-planes, which the F7 maximal-faces gate refuses; whether the
//! revolve should mint them whole is a separate question (in front of
//! Ev), and this suite is about what happens past F7, so the fixture
//! merges them the way a caller can today. The curved walls stay split
//! (a periodic wall keeps its parameterization cut), which is the
//! canonical maximal form.
//!
//! The doors, in the order the union meets them, each with the row that
//! holds it:
//!
//! 1. the operand gate's KIND roster, which had no torus;
//! 2. the circle rung on the waist's seam meridian against the other
//!    half's waist — a coincident pair whose sampled clearance reads
//!    definitely negative, so the declared cover behind it was never
//!    consulted; the carrier-identity rung now answers first;
//! 3. face-level containment on a torus face, which answered nothing;
//! 4. the sector walk's outward normal on a torus face, which had no
//!    arm.
//!
//! The line×torus crossing is the other door the torus lane needs, and
//! this fixture never reaches it (no line edge of one half meets a
//! torus face of the other), so its rows run on a donut and a bar. The
//! donut's pierces go on to the pierce lane's outward normal, which
//! has a torus arm too, and then to the curved-sector sagitta charge,
//! which refuses a pierce vertex's near-tangent in-face bisectors on a
//! torus exactly as it does on a cylinder; the sweep traces, which stop
//! before that classification, are what those rows read.
//!
//! Past every torus door the union stops at the chord join, exactly
//! where the same dumbbell with a CYLINDER handle stops — the control
//! row says so, and the stop is not a torus door.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::revolve_common;

use geom_core::{Band, Point3, Tol};
use profile::{ProfileLoop, RawLoop, test_support::bulge_loop};
use revolve_common::{axis_y, p2, validated};
use sweep::{Revolution, revolve};
use topo::{
    Body, BooleanDeclarations, BooleanError, ContactClass, FaceContainment, FaceKey,
    FacePairDeclaration, SplitJoinError,
};

/// The waist's tube.
const MAJOR: f64 = 0.8;
const MINOR: f64 = 0.5;
/// A quarter arc, clockwise: `−tan(π/8)`.
const QUARTER_CW: f64 = -0.414_213_562_373_095_03;

fn band() -> Band {
    Band::linear(Tol::witness()).expect("the run's linear band")
}

/// Which handle the half carries.
#[derive(Clone, Copy, Debug)]
enum Handle {
    /// The concave torus waist.
    Torus,
    /// The control: a straight `0.3`-radius cylinder in its place.
    Cylinder,
}

/// One half of the dumbbell, `sign = +1` above the joint plane and `−1`
/// below it, fully revolved and, unless `premerge` is off, pre-merged
/// (module docs).
fn half_as(sign: f64, handle: Handle, premerge: bool) -> Body<f64> {
    let s = sign;
    // CCW in the (ρ, y) half-plane.
    let (mut chain, tangent_joint) = match handle {
        Handle::Torus if s > 0.0 => (
            vec![
                (p2(0.0, 0.0), 0.0),
                (p2(0.3, 0.0), QUARTER_CW),
                (p2(0.8, 0.5), 0.0),
                (p2(1.5, 0.5), 0.0),
                (p2(1.5, 1.5), 0.0),
                (p2(0.0, 1.5), 0.0),
            ],
            Some(2),
        ),
        Handle::Torus => (
            vec![
                (p2(0.0, 0.0), 0.0),
                (p2(0.0, -1.5), 0.0),
                (p2(1.5, -1.5), 0.0),
                (p2(1.5, -0.5), 0.0),
                (p2(0.8, -0.5), QUARTER_CW),
                (p2(0.3, 0.0), 0.0),
            ],
            Some(4),
        ),
        Handle::Cylinder => (
            vec![
                (p2(0.0, 0.0), 0.0),
                (p2(0.3, 0.0), 0.0),
                (p2(0.3, 0.5 * s), 0.0),
                (p2(1.5, 0.5 * s), 0.0),
                (p2(1.5, 1.5 * s), 0.0),
                (p2(0.0, 1.5 * s), 0.0),
            ],
            None,
        ),
    };
    if matches!(handle, Handle::Cylinder) && s < 0.0 {
        // Mirrored, so reversed to stay CCW (every bulge is zero).
        chain.reverse();
    }
    let lp: ProfileLoop<f64> = match tangent_joint {
        Some(j) => bulge_loop(chain).with_tangent_joints(vec![j]),
        None => bulge_loop(chain),
    };
    let mut body = revolve(
        &validated(vec![lp]),
        axis_y(),
        Revolution::Full,
        Tol::witness(),
    )
    .expect("the half revolves")
    .body;
    if premerge {
        body.merge_coplanar_faces(Tol::witness())
            .expect("the split planar walls merge");
    }
    body
}

fn half(sign: f64, handle: Handle) -> Body<f64> {
    half_as(sign, handle, true)
}

fn surface(body: &Body<f64>, f: FaceKey) -> &geom::Surface<f64> {
    body.get_surface(body.get_face(f).unwrap().surface).unwrap()
}

fn faces_where(body: &Body<f64>, pred: impl Fn(&geom::Surface<f64>) -> bool) -> Vec<FaceKey> {
    body.faces()
        .map(|(k, _)| k)
        .filter(|&k| pred(surface(body, k)))
        .collect()
}

fn is_joint_disc(s: &geom::Surface<f64>) -> bool {
    matches!(s, geom::Surface::Plane { origin, .. } if origin.y.abs() < 1e-12)
}

fn is_handle(s: &geom::Surface<f64>) -> bool {
    matches!(s, geom::Surface::Torus { .. })
        || matches!(s, geom::Surface::Cylinder { radius, .. } if (*radius - 0.3).abs() < 1e-12)
}

fn is_torus(s: &geom::Surface<f64>) -> bool {
    matches!(s, geom::Surface::Torus { .. })
}

/// The joint discs declared `Rest`, and every handle×handle pair
/// declared under `handle_class` (none when `None`).
fn declarations(
    a: &Body<f64>,
    b: &Body<f64>,
    handle_class: Option<ContactClass>,
) -> BooleanDeclarations {
    let mut decls = BooleanDeclarations::none();
    for fa in faces_where(a, is_joint_disc) {
        for fb in faces_where(b, is_joint_disc) {
            decls
                .coincident_faces
                .push(FacePairDeclaration::rest(fa, fb));
        }
    }
    if let Some(class) = handle_class {
        for fa in faces_where(a, is_handle) {
            for fb in faces_where(b, is_handle) {
                decls
                    .coincident_faces
                    .push(FacePairDeclaration::new(fa, fb, class));
            }
        }
    }
    decls
}

/// The T2 union: both halves, the joint discs and the handle pairs
/// declared `Rest`.
fn t2(handle: Handle) -> Result<topo::BooleanResult<f64>, BooleanError> {
    let (a, b) = (half(1.0, handle), half(-1.0, handle));
    let decls = declarations(&a, &b, Some(ContactClass::Rest));
    topo::union_with(&a, &b, &decls, Tol::witness())
}

// -------------------------------------------------------------------
// The fixture.
// -------------------------------------------------------------------

/// **The halves are what the module says they are**: valid at every
/// tier once pre-merged, each carrying its waist on the `R = 0.8`,
/// `r = 0.5` torus about `y` — a FAT ring (`R < 2r`), which is what
/// makes the waist bend harder along its inner equator than across
/// the tube. And the pre-merge is load-bearing: the revolve's own split
/// planar walls are refused by F7 before any torus door is reached.
#[test]
fn the_half_dumbbell_is_a_valid_torus_waisted_solid_once_premerged() {
    for sign in [1.0, -1.0] {
        let body = half(sign, Handle::Torus);
        assert_eq!(topo::validate(&body), Ok(()));
        assert_eq!(topo::validate_closed(&body), Ok(()));
        assert_eq!(topo::validate_geometric(&body, Tol::witness()), Ok(()));
        let tori = faces_where(&body, is_torus);
        assert_eq!(tori.len(), 2, "a full revolve keeps the wall's cut");
        for f in tori {
            let geom::Surface::Torus {
                center,
                major_radius,
                minor_radius,
                ..
            } = *surface(&body, f)
            else {
                unreachable!()
            };
            assert!(center.distance(Point3::origin()) < 1e-12);
            assert!((major_radius - MAJOR).abs() < 1e-12);
            assert!((minor_radius - MINOR).abs() < 1e-12);
        }
    }
    let (a, b) = (
        half_as(1.0, Handle::Torus, false),
        half_as(-1.0, Handle::Torus, false),
    );
    let err = topo::union_with(&a, &b, &declarations(&a, &b, None), Tol::witness())
        .expect_err("the unmerged halves carry split planar walls");
    assert!(
        matches!(err, BooleanError::NonMaximalFaces { .. }),
        "without the pre-merge the op stops at F7: {err:?}"
    );
}

// -------------------------------------------------------------------
// Door 1: the operand gate admits the torus.
// -------------------------------------------------------------------

/// **The KIND roster has a torus now, and the refusal it used to raise
/// is gone from every variant of the union.** Before, a waist face
/// against the other half's joint disc — boxes overlapping, the pair
/// not declared — was `CurvedPairUnsupported { kind: Torus, other_kind:
/// Plane }` at the gate, declared handle or not.
///
/// What admission must NOT do is turn a torus pair nobody vouched for
/// into a body: undeclared, the coincident waists still refuse typed —
/// at the circle rung, where the sampled clearance is what it is and no
/// declared cover exists to take the endpoint posture.
#[test]
fn the_operand_gate_admits_the_torus_and_the_undeclared_pair_still_refuses() {
    let (a, b) = (half(1.0, Handle::Torus), half(-1.0, Handle::Torus));
    for class in [None, Some(ContactClass::Rest)] {
        let err = topo::union_with(&a, &b, &declarations(&a, &b, class), Tol::witness())
            .expect_err("no torus union builds a body yet");
        assert!(
            !matches!(err, BooleanError::CurvedPairUnsupported { .. }),
            "the gate must admit the torus ({class:?}): {err:?}"
        );
    }
    let err = topo::union_with(&a, &b, &declarations(&a, &b, None), Tol::witness())
        .expect_err("an undeclared coincident torus pair must refuse");
    let BooleanError::CurvedPierceUnsupported { operand, face, .. } = err else {
        panic!("undeclared, the waists refuse at the circle rung: {err:?}");
    };
    assert_eq!(operand, topo::Operand::A);
    assert!(is_torus(surface(&b, face)), "the refusal names B's waist");
}

/// **The ordering fact the gate's class-blind cover predicate rests on,
/// held on a torus.** The gate's covered-pair rung reads ANY declared
/// class as cover, which is safe only while the `Tangent` door refuses
/// every curved pair it cannot verify BEFORE the gate runs. On the
/// waists it does: the conformal screen finds one carrier and a
/// `Tangent` claim on a conformal pair is contradicted at the
/// declaration door. Admitting the torus to the roster means no torus
/// pair ever needs the cover again — and this row is what goes red if
/// the `Tangent` door learns to admit one without the gate's predicate
/// being revisited.
#[test]
fn a_tangent_declared_torus_pair_is_refused_before_the_gate() {
    let (a, b) = (half(1.0, Handle::Torus), half(-1.0, Handle::Torus));
    let decls = declarations(&a, &b, Some(ContactClass::Tangent));
    let err = topo::union_with(&a, &b, &decls, Tol::witness())
        .expect_err("a Tangent claim on one carrier is false");
    let BooleanError::ContactContradicted { declaration, .. } = err else {
        panic!("the declaration door must refuse the Tangent torus pair: {err:?}");
    };
    assert_eq!(declaration.class, ContactClass::Tangent);
    assert!(is_torus(surface(&a, declaration.a)) && is_torus(surface(&b, declaration.b)));
}

// -------------------------------------------------------------------
// Door 2: the carrier-identity rung, before the sampled clearance.
// -------------------------------------------------------------------

/// **What the sampled enclosure says about the waist's seam meridian,
/// and why it cannot be the rung that decides.** The meridian lies ON
/// the other half's waist carrier — the residual is identically zero
/// along it — but the arc's sampled enclosure is `±charge` about that
/// zero, and the carrier's harmonic one is wider still, so the folded
/// one-sidedness margin is `−charge`: 1.73e-5 m here, definitely
/// negative at every eps row the run matrix draws. The declared-cover
/// rung needs a `Zero`, so on the enclosures alone the covered pair
/// could never reach it. The carrier-identity rung answers first.
#[test]
fn the_waist_meridian_reads_definitely_negative_on_the_sampled_enclosure() {
    let (a, b) = (half(1.0, Handle::Torus), half(-1.0, Handle::Torus));
    let mut meridians = 0;
    for (_, e) in a.edges() {
        let c = a.get_curve_geom(e.curve).unwrap().certified().unwrap();
        let geom::Curve3::Circle {
            center,
            axis,
            radius,
            u_ref,
        } = *c.carrier()
        else {
            continue;
        };
        if (radius - MINOR).abs() > 1e-12 {
            continue;
        }
        meridians += 1;
        let (t0, t1) = c.params();
        for fb in faces_where(&b, is_torus) {
            let s = surface(&b, fb);
            let (lo, hi) =
                geom_brep::circle_residual_extremes(s, center, axis, radius, u_ref).unwrap();
            let (arc_lo, arc_hi) =
                geom_brep::circle_arc_residual_range(s, center, axis, radius, u_ref, t0, t1)
                    .unwrap();
            let margin = (lo.max(-hi)).max(arc_lo.max(-arc_hi));
            assert!(
                (margin + 1.733_498_408_828_498e-5).abs() < 1e-12,
                "the folded margin is the chord-dip charge: {margin}"
            );
            assert!(
                margin < -band().escalate(),
                "and it is definitely negative at this run's band: {margin}"
            );
        }
    }
    assert_eq!(meridians, 2, "the waist carries its seam meridian twice");
}

/// **The declared waists pass the crossing layer.** With both waist
/// pairs declared `Rest`, the seam meridian reaches the declared-cover
/// rung through the carrier identity, and neither the circle rung's
/// frontier nor its escalation is what the union answers.
#[test]
fn the_declared_waists_pass_the_circle_rung() {
    let err = t2(Handle::Torus).expect_err("the union still stops downstream");
    assert!(
        !matches!(
            err,
            BooleanError::CurvedPierceUnsupported { .. } | BooleanError::Escalated { .. }
        ),
        "the carrier-identity rung must carry the declared waists past the circle rung: {err:?}"
    );
}

// -------------------------------------------------------------------
// Door 3: face containment on a torus face.
// -------------------------------------------------------------------

/// A point on the upper waist at azimuth `alpha` about `y` and profile
/// angle `theta` (`0` at the joint circle, `π/2` at the bell), pushed
/// `off` metres along the tube's outward normal.
fn waist_point(alpha: f64, theta: f64, off: f64, sign: f64) -> Point3<f64> {
    let (st, ct) = theta.sin_cos();
    let rho = MAJOR - (MINOR + off) * ct;
    let y = sign * (MINOR + off) * st;
    let (sa, ca) = alpha.sin_cos();
    Point3::new(rho * ca, y, rho * sa)
}

/// **The two waist faces partition their band, and nothing else is in
/// either.** Every point sampled strictly inside the upper waist, away
/// from the seam azimuths, is `In` exactly one of A's two waist faces
/// and `Out` of the other; its mirror image below the joint plane — the
/// SAME carrier, outside both faces' minor window — is `Out` of both;
/// and a point a millimetre off the tube is `Out` of both at the
/// carrier test. A containment that answered on the wrong side of
/// either window, or read a wrap where the face is trimmed, puts a
/// point in both faces or in neither.
#[test]
fn the_waist_faces_partition_their_band_under_face_containment() {
    let a = half(1.0, Handle::Torus);
    let tori = faces_where(&a, is_torus);
    assert_eq!(tori.len(), 2);
    let at = |f, p| {
        topo::curved_face_containment(&a, f, p, band())
            .expect("containment decides without escalating")
    };
    for k in 0..12 {
        let alpha = 0.1 + f64::from(k) * core::f64::consts::TAU / 12.0;
        for theta in [0.2, 0.7, 1.3] {
            let p = waist_point(alpha, theta, 0.0, 1.0);
            let verdicts: Vec<_> = tori.iter().map(|&f| at(f, p)).collect();
            let ins = verdicts
                .iter()
                .filter(|v| **v == Some(FaceContainment::In))
                .count();
            let outs = verdicts
                .iter()
                .filter(|v| **v == Some(FaceContainment::Out))
                .count();
            assert_eq!(
                (ins, outs),
                (1, 1),
                "alpha {alpha}, theta {theta}: {verdicts:?}"
            );
            for q in [
                waist_point(alpha, theta, 0.0, -1.0),
                waist_point(alpha, theta, 1e-3, 1.0),
            ] {
                for &f in &tori {
                    assert_eq!(
                        at(f, q),
                        Some(FaceContainment::Out),
                        "{q:?} is in no waist face"
                    );
                }
            }
        }
    }
}

// -------------------------------------------------------------------
// The line × torus crossing, on a donut.
// -------------------------------------------------------------------

fn bar(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> Body<f64> {
    use geom_core::{Affine3, Mat3, Vec3};
    let lp = ProfileLoop::polygon([p2(x.0, y.0), p2(x.1, y.0), p2(x.1, y.1), p2(x.0, y.1)]);
    let plane = profile::SketchPlane::new(Affine3::from_parts(
        Mat3::from_cols(Vec3::unit_x(), Vec3::unit_y(), Vec3::unit_z()),
        Point3::new(0.0, 0.0, z.0) - Point3::origin(),
    ));
    let vp = profile::Profile::new(plane, vec![lp])
        .validate(Tol::witness())
        .expect("the bar profile validates");
    sweep::extrude(&vp, sweep::Extrusion::Distance(z.1 - z.0), Tol::witness())
        .expect("the bar extrudes")
        .body
}

fn donut() -> Body<f64> {
    let vp = validated(vec![revolve_common::donut_profile()]);
    revolve(&vp, axis_y(), Revolution::Full, Tol::witness())
        .expect("the donut revolves")
        .body
}

/// **A segment through the tube is pierced at both quartic roots, one
/// in each half of the donut.** The bar runs along `z` at `y ≈ 0.3`,
/// from the hole (`z = 1`) to beyond the ring (`z = 3`): both ends are
/// OUTSIDE the tube, so each of its four long edges is a
/// `(Positive, Positive)` span whose residual dips through the tube —
/// the belly the cylinder's convex bound covers and the torus's
/// non-convex residual does not. The certified quartic finds both
/// roots; each edge is split once in the donut's inner face and once
/// in its outer face, so the sweep mints eight fragments. A missed root
/// mints fewer; a skipped belly mints none.
#[test]
fn a_segment_through_the_tube_is_pierced_at_both_quartic_roots() {
    let d = donut();
    let b = bar((-0.05, 0.05), (0.25, 0.35), (1.0, 3.0));
    let original: Vec<_> = b.edges().map(|(k, _)| k).collect();
    let (_, b_on_a) =
        topo::sweep_traces(&d, &b, topo::SweepStrategy::Realized, None, Tol::witness())
            .expect("the sweep completes on a transverse bar");
    let mut minted: Vec<_> = b_on_a
        .accepted
        .iter()
        .map(|&(e, _)| e)
        .filter(|e| !original.contains(e))
        .collect();
    minted.sort();
    minted.dedup();
    assert_eq!(
        minted.len(),
        8,
        "two pierces on each of four edges: {b_on_a:?}"
    );
    let pierced: std::collections::BTreeSet<_> = b_on_a.accepted.iter().map(|&(_, f)| f).collect();
    assert_eq!(
        pierced,
        d.faces().map(|(k, _)| k).collect(),
        "both donut faces are pierced"
    );
}

/// **A chord across the hole is pierced, not passed.** The bar runs
/// along `z` through the hole with both ends INSIDE the tube: a
/// `(Negative, Negative)` span, which a convex residual would clear at
/// its endpoints and a torus's does not — the line leaves the tube, runs
/// through the hole and re-enters. The quartic splits it; what then
/// refuses is the fragment between the two pierces, a chord with both
/// ends on the carrier tested against the donut's OTHER face, where the
/// undeclared `(Zero, Zero)` arm keeps its strict rule (only a recorded
/// end counts). That rule is kind-generic and is the next door this
/// lane meets (`work/germ/undeclared-chord-between-two-pierces-refuses-on-the-sibling-face`);
/// the row pins that the refusal names a FRAGMENT, which a passed chord
/// never mints.
#[test]
fn a_chord_across_the_hole_is_pierced_not_passed() {
    let d = donut();
    let b = bar((-0.1, 0.1), (-0.1, 0.1), (-2.0, 2.0));
    let original: Vec<_> = b.edges().map(|(k, _)| k).collect();
    let err = topo::sweep_traces(&d, &b, topo::SweepStrategy::Realized, None, Tol::witness())
        .expect_err("the chord between the pierces refuses on the sibling face");
    let BooleanError::CurvedPierceUnsupported { operand, edge, .. } = err else {
        panic!("the undeclared chord arm's frontier: {err:?}");
    };
    assert_eq!(operand, topo::Operand::B);
    assert!(
        !original.contains(&edge),
        "the refused edge is a fragment the pierce minted: {edge:?}"
    );
}

// -------------------------------------------------------------------
// Door 4, the sector walk, and where the union stops.
// -------------------------------------------------------------------

/// **Past every torus door, the union stops where the cylinder-handled
/// dumbbell stops.** The torus-waisted union used to refuse
/// `CurvedBooleanUnsupported { kind: Torus }` at the sector walk; with
/// the torus arm it reaches the chord join, and the chord join refuses
/// `UnpairedLooseEnds { count: 4 }` — the answer the SAME dumbbell with
/// a straight cylinder handle gets, under the same declarations. The
/// declared-REST zip that takes over a refused declared union declines
/// both at its segment enumeration, so the join's refusal surfaces
/// verbatim for both. That stop is not a torus door
/// (`work/zip/dumbbell-joint-union-leaves-four-loose-ends`).
#[test]
fn the_torus_waisted_union_stops_at_the_join_like_the_cylinder_control() {
    for handle in [Handle::Torus, Handle::Cylinder] {
        let err = t2(handle).expect_err("the dumbbell's joint does not zip yet");
        assert!(
            matches!(
                err,
                BooleanError::Join(SplitJoinError::UnpairedLooseEnds { count: 4 })
            ),
            "{handle:?}: {err:?}"
        );
    }
}

// -------------------------------------------------------------------
// A certified root the landing point contradicts keeps the door.
// -------------------------------------------------------------------

/// A `w × w` square bar along the unit direction `d`, from `o + d·t0`
/// to `o + d·t1`: the square lies in the plane normal to `d` at the
/// start, in the frame `u = normalize(d × ŷ)`, `v = d × u`.
fn framed_bar(o: Point3<f64>, d: geom_core::Vec3<f64>, t0: f64, t1: f64, w: f64) -> Body<f64> {
    use geom_core::{Affine3, Mat3, Vec3};
    let d = d.normalize();
    let u = d.cross(Vec3::new(0.0, 1.0, 0.0)).normalize();
    let v = d.cross(u);
    let h = w / 2.0;
    let lp = ProfileLoop::polygon([p2(-h, -h), p2(h, -h), p2(h, h), p2(-h, h)]);
    let start = o + d * t0;
    let plane = profile::SketchPlane::new(Affine3::from_parts(
        Mat3::from_cols(u, v, d),
        start - Point3::origin(),
    ));
    let vp = profile::Profile::new(plane, vec![lp])
        .validate(Tol::witness())
        .expect("the framed bar's profile validates");
    sweep::extrude(&vp, sweep::Extrusion::Distance(t1 - t0), Tol::witness())
        .expect("the framed bar extrudes")
        .body
}

/// **A bar through the tube is never answered as disjoint.** A near-
/// perpendicular pose puts a certified quartic root a hair off the
/// tube, and the landing point then reads definitely OFF the carrier.
/// That used to be taken for "outside this face's trim", the root was
/// stepped over, and the union came back as an assembly of two solids
/// that overlap. Whatever the band does with this pose, it must not
/// be that.
#[test]
fn a_near_perpendicular_bar_through_the_tube_never_comes_back_disjoint() {
    let d = geom_core::Vec3::new(
        -0.990_360_666_876_138_8,
        1.376_996_009_986_983_2e-4,
        0.138_512_564_568_957,
    )
    .normalize();
    let o = Point3::new(
        -2.109_637_800_205_744_5,
        0.170_221_792_550_834_86,
        1.920_645_887_674_835_4,
    );
    // A square narrower than a hundred ε is not a valid profile at the
    // run's band, so the thinnest bar is taken only where it is one.
    let floor = 100.0 * Tol::witness().get().eps;
    for w in [1e-6, 1e-3].into_iter().filter(|&w| w > floor) {
        let b = framed_bar(o, d, -4.6, -0.1, w);
        if let Ok(r) = topo::union_with(&donut(), &b, &BooleanDeclarations::none(), Tol::witness())
        {
            panic!(
                "a bar through the tube is not disjoint (w = {w}): {:?}",
                r.body().map(|x| x.kind)
            );
        }
    }
}

/// **A rod lying inside the tube is crossed twice, and never passed
/// silently.** Its span leaves and re-enters the tube, so the sweep
/// either accepts a crossing or refuses; and whatever the union
/// answers, it is not two solids side by side.
#[test]
fn a_rod_inside_the_tube_is_not_passed_silently() {
    let d = donut();
    let rod = framed_bar(
        Point3::new(
            -1.647_779_393_496_495_5,
            0.270_473_450_406_354_4,
            1.497_185_557_815_917,
        ),
        geom_core::Vec3::new(
            -0.980_697_285_232_897,
            7.766_907_203_911_848e-5,
            -0.195_532_167_952_848_92,
        ),
        -3.6,
        4.5,
        // The pose's own width, raised to a valid profile at a coarse
        // run band (a square must stand clear of a hundred ε).
        1e-5_f64.max(200.0 * Tol::witness().get().eps),
    );
    if let Ok((_, on_d)) = topo::sweep_traces(
        &d,
        &rod,
        topo::SweepStrategy::Realized,
        None,
        Tol::witness(),
    ) {
        assert!(!on_d.accepted.is_empty(), "the rod crosses the tube twice");
    }
    if let Ok(r) = topo::union(&d, &rod, Tol::witness()) {
        assert!(!matches!(
            r.body().expect("non-empty").kind,
            topo::BooleanResultKind::Assembly
        ));
    }
}

/// **A grazing line keeps the door.** A rod along `y` whose edge
/// touches the donut's outer equator at one point — a double root, a
/// root count the quartic cannot certify — must refuse typed at the
/// crossing layer. An uncertain count read as a miss would pass the
/// rod as clear.
#[test]
fn a_rod_grazing_the_outer_equator_refuses_at_the_crossing_layer() {
    let d = donut();
    let rod = bar((-1e-3, 0.0), (-1.0, 1.0), (2.5, 2.501));
    let err = topo::sweep_traces(
        &d,
        &rod,
        topo::SweepStrategy::Realized,
        None,
        Tol::witness(),
    )
    .expect_err("a tangent line has no certified root count");
    assert!(
        matches!(
            err,
            BooleanError::CurvedPierceUnsupported {
                operand: topo::Operand::B,
                ..
            }
        ),
        "the grazing edge keeps the crossing layer's door: {err:?}"
    );
}

/// **A tilted segment through the tube, off the midplane** — the
/// quartic's FERRARI arm (`e = d·a ≠ 0`, so the odd coefficient is
/// definite and the resolvent cubic is solved), where every row above
/// runs the biquadratic arm. The bar climbs from the hole to beyond the
/// ring; each of its four long edges crosses the inner half and the
/// outer half once, so the sweep mints eight fragments.
#[test]
fn a_tilted_segment_through_the_tube_is_pierced_on_the_ferrari_arm() {
    let d = donut();
    let dir = geom_core::Vec3::new(0.0, 0.3, 1.0).normalize();
    let b = framed_bar(Point3::new(0.0, 0.25, 2.0), dir, -1.0, 1.0, 0.02);
    let original: Vec<_> = b.edges().map(|(k, _)| k).collect();
    let (_, b_on_d) =
        topo::sweep_traces(&d, &b, topo::SweepStrategy::Realized, None, Tol::witness())
            .expect("the sweep completes on the tilted bar");
    let mut minted: Vec<_> = b_on_d
        .accepted
        .iter()
        .map(|&(e, _)| e)
        .filter(|e| !original.contains(e))
        .collect();
    minted.sort();
    minted.dedup();
    assert_eq!(
        minted.len(),
        8,
        "two pierces on each of four edges: {b_on_d:?}"
    );
}
