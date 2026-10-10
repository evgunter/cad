//! **Bars whose edges are chords of a cylinder wall, and cubes touching
//! a drum at a corner: every op answers its closed form or names the
//! door that stops it.**
//!
//! These are poses the pierce lane's sector-side curvature charge
//! refused before it was read at its peak (`boolean::sectors::side_code`).
//! Two families:
//!
//! - **Bars straddling a cap.** A bar crossing a cylinder's top (or
//!   bottom) cap, its long edges inside the cap's slab: an edge at
//!   lateral offset `|y| < r` is a CHORD of the wall, pierced twice, and
//!   between its two pierces the bar's floor meets the wall in an arc
//!   with the chord beside it — a two-edge face. The ∩ is the disc's
//!   band under the bar, times the depth. These poses once returned ∩
//!   bodies missing that face (a wrong volume, negative on some, tier 3
//!   red): the join took the chord for the section segment and never
//!   minted the arc. REACH first fixed it with a geometric test of the
//!   chord against the wall; the join's adjacency skip now reads the
//!   segment's locus instead (JOIN-1), and the section segment here
//!   lies inside the bar's floor, so no edge is ever taken for it.
//! - **Cubes touching a drum's wall at a corner**, their main diagonal
//!   along the wall's normal, inside or outside: each op is the cube's
//!   volume combined with the drum's, exactly.
//!
//! Each row runs ∪, both ∖ and ∩; a body must hold its closed-form
//! volume and pass tier 3, and a refusal must be the door the row
//! names. The bars' rows all build: a cut that notches the cap's wall
//! measures, and one whose section closes inside the wall joins
//! through the pierce rings.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_1_SQRT_2, PI};

use crate::common::germ_pair::cyl;
use crate::common::operands::{framed_bar, three_arc_cylinder};
use geom_core::{Affine3, Mat3, Point2, Point3, Tol, Vec3};
use sweep::test_support::{brick, finished};
use topo::{AtRestBody, BooleanError, BooleanResult};

/// What one op must answer.
#[derive(Clone, Copy, Debug)]
enum Want {
    /// A body of this volume (0 for an empty result), at tier 3.
    Volume(f64),
    /// A body of this volume through tiers 2, 3 and 3′, and a legal
    /// operand.
    Sound(f64),
}

fn check(what: &str, r: Result<BooleanResult<f64>, BooleanError>, want: Want) {
    let tol = Tol::witness();
    match (r, want) {
        (Ok(r), Want::Sound(v)) => {
            let got = match r.body() {
                Some(b) => {
                    topo::validate_closed(&b.body)
                        .unwrap_or_else(|e| panic!("{what}: tier 2, got {e:?}"));
                    topo::validate_geometric(&b.body, tol)
                        .unwrap_or_else(|e| panic!("{what}: tier 3, got {e:?}"));
                    topo::validate_pseudomanifold(&b.body, &b.contacts, tol)
                        .unwrap_or_else(|e| panic!("{what}: tier 3′, got {e:?}"));
                    sweep::test_support::assert_legal_operand(what, &b.body, tol);
                    topo::mass_properties(&b.body, tol).unwrap().volume
                }
                None => 0.0,
            };
            assert!(
                (got - v).abs() <= 1e-9 * v.abs().max(1e-3),
                "{what}: volume {got}, closed form {v}"
            );
        }
        (Ok(r), Want::Volume(v)) => {
            let got = match r.body() {
                Some(b) => {
                    topo::validate_geometric(&b.body, tol)
                        .unwrap_or_else(|e| panic!("{what}: tier 3, got {e:?}"));
                    topo::mass_properties(&b.body, tol).unwrap().volume
                }
                None => 0.0,
            };
            assert!(
                (got - v).abs() <= 1e-9 * v.abs().max(1e-3),
                "{what}: volume {got}, closed form {v}"
            );
        }
        (got, want) => panic!(
            "{what}: wanted {want:?}, got {:?}",
            got.map(|r| r.body().is_some())
        ),
    }
}

/// The four ops of `a` with `b`, against `[∪, A∖B, B∖A, ∩]`.
fn four(what: &str, a: &AtRestBody<f64>, b: &AtRestBody<f64>, want: [Want; 4]) {
    let tol = Tol::witness();
    check(&format!("{what} ∪"), topo::union(a, b, tol), want[0]);
    check(&format!("{what} A∖B"), topo::subtract(a, b, tol), want[1]);
    check(&format!("{what} B∖A"), topo::subtract(b, a, tol), want[2]);
    check(&format!("{what} ∩"), topo::intersect(a, b, tol), want[3]);
}

/// `∫ 2·sqrt(1 − y²) dy` over `[y0, y1] ∩ [−1, 1]`: the area of the
/// unit disc's band between two chords.
fn band_area(y0: f64, y1: f64) -> f64 {
    let f = |y: f64| y * (1.0 - y * y).sqrt() + y.asin();
    let (a, b) = (y0.max(-1.0), y1.min(1.0));
    if b <= a { 0.0 } else { f(b) - f(a) }
}

/// **The reviewed scan.** A bar of width `0.404` along `(1, 1, 0)/√2`,
/// its centreline `c` off the axis of the unit cylinder `z ∈ [0, 2]`,
/// sunk `depth` into the top cap. Its near floor edge, at lateral
/// `c − w/2`, is a chord of the wall, clipped by the bar's own ends.
/// Every op builds, the deeper poses at `c = 0.9` through the pierce
/// rings their sections close on.
#[test]
fn a_diagonal_bar_sunk_into_a_cylinder_cap_answers_its_closed_form() {
    let (w, t0, t1) = (
        0.404_001_346_965_559_2,
        -0.688_250_512_587_890_8,
        1.738_194_969_430_420_4,
    );
    let s = FRAC_1_SQRT_2;
    let a = finished(
        "the cylinder",
        three_arc_cylinder(Point2::new(0.0, 0.0), 1.0, 0.0, 2.0, 0.0),
        Tol::witness(),
    );
    let (vcyl, vbar) = (2.0 * PI, w * w * (t1 - t0));
    for c in [0.9_f64, 1.047, 1.15] {
        let lo = c - w / 2.0;
        let half = (1.0 - lo * lo).sqrt();
        let f = |x: f64| 0.5 * (x * (1.0 - x * x).sqrt() + x.asin()) - lo * x;
        let area = f(half.min(t1)) - f((-half).max(t0));
        for depth in [0.002, 0.0078, 0.03, 0.1, 0.3] {
            let o = Point3::new(c * s, -c * s, 2.0 - depth + w / 2.0);
            let b = finished(
                "the bar",
                framed_bar(o, Vec3::new(s, s, 0.0), t0, t1, w),
                Tol::witness(),
            );
            let i = area * depth;
            let what = format!("c {c}, depth {depth}:");
            four(
                &what,
                &a,
                &b,
                [
                    Want::Sound(vcyl + vbar - i),
                    Want::Sound(vcyl - i),
                    Want::Sound(vbar - i),
                    Want::Sound(i),
                ],
            );
        }
    }
}

/// **The same class along `x`, both caps.** A bar spanning the whole
/// disc along `x`, lateral band `y ∈ [c − h, c + h]`, sunk `0.1` into
/// the top cap or raised `0.1` into the bottom one: ∩ is the disc's
/// band times the depth, whether both floor edges are chords, one is,
/// or the band runs off the disc.
#[test]
fn an_x_bar_sunk_into_either_cap_answers_its_closed_form() {
    let (w, depth) = (0.4_f64, 0.1);
    let h = w / 2.0;
    let a = finished(
        "the cylinder",
        three_arc_cylinder(Point2::new(0.0, 0.0), 1.0, 0.0, 2.0, 0.0),
        Tol::witness(),
    );
    let (vcyl, vbar) = (2.0 * PI, w * w * 6.0);
    for c in [0.3, -0.95, 1.1] {
        let i = band_area(c - h, c + h) * depth;
        for (cap, zc) in [("top", 2.0 - depth + h), ("bottom", depth - h)] {
            let b = finished(
                "the bar",
                framed_bar(
                    Point3::new(0.0, c, zc),
                    Vec3::new(1.0, 0.0, 0.0),
                    -3.0,
                    3.0,
                    w,
                ),
                Tol::witness(),
            );
            four(
                &format!("x-bar c {c}, {cap} cap:"),
                &a,
                &b,
                [
                    Want::Volume(vcyl + vbar - i),
                    Want::Volume(vcyl - i),
                    Want::Volume(vbar - i),
                    Want::Volume(i),
                ],
            );
        }
    }
}

/// The rigid motion taking the cube `[0, l]³`'s corner at the origin to
/// `p`, its main diagonal to `diag`, spun `spin` about it.
fn cube_at(p: Point3<f64>, diag: Vec3<f64>, spin: f64, l: f64) -> AtRestBody<f64> {
    let tol = Tol::witness();
    let u1 = Vec3::new(1.0, 1.0, 1.0).normalize();
    let u2 = Vec3::new(1.0, -1.0, 0.0).normalize();
    let u3 = u1.cross(u2);
    let a = diag.normalize();
    let b0 = a.cross(Vec3::new(0.0, 0.0, 1.0)).normalize();
    let c0 = a.cross(b0);
    let b = b0 * spin.cos() + c0 * spin.sin();
    let c = a.cross(b);
    let col = |j: usize| {
        let pick = |u: Vec3<f64>| [u.x, u.y, u.z][j];
        a * pick(u1) + b * pick(u2) + c * pick(u3)
    };
    let m = Mat3::from_cols(col(0), col(1), col(2));
    let cube = brick((0.0, l), (0.0, l), (0.0, l), tol);
    let cube = topo::transform_rigid(&cube, &Affine3::from_parts(m, p - Point3::origin()), tol)
        .expect("the cube moves");
    finished("the cube", cube, tol)
}

/// **A cube touching a drum's wall at one corner.** Its main diagonal
/// runs along the wall's normal, so every edge leaves the corner at
/// `d̂·n̂ = ∓1/√3`: inward, the cube lies inside the drum (its corners
/// say so, and the drum is convex); outward, it lies beyond the wall's
/// tangent plane.
#[test]
fn a_cube_touching_a_drum_at_a_corner_answers_its_closed_form() {
    let drum = finished("the drum", cyl(1.0, 2.0), Tol::witness());
    let vd = 4.0 * PI;
    let phi = 0.7_f64;
    let p = Point3::new(phi.cos(), phi.sin(), 0.3);
    let n = Vec3::new(phi.cos(), phi.sin(), 0.0);
    for l in [0.65, 0.8] {
        for spin in [0.0, 0.4] {
            let cube3 = l * l * l;
            let inner = cube_at(p, -n, spin, l);
            assert!(
                inner
                    .vertex_points()
                    .map(|(_, q)| q)
                    .filter(|q| (*q - p).norm() > 1e-9)
                    .all(|q| q.x.hypot(q.y) < 1.0 && q.z.abs() < 2.0),
                "l {l} spin {spin}: the inner cube's corners are inside the drum"
            );
            four(
                &format!("inner cube l {l} spin {spin}:"),
                &drum,
                &inner,
                [
                    Want::Volume(vd),
                    Want::Volume(vd - cube3),
                    Want::Volume(0.0),
                    Want::Volume(cube3),
                ],
            );
            four(
                &format!("outer cube l {l} spin {spin}:"),
                &drum,
                &cube_at(p, n, spin, l),
                [
                    Want::Volume(vd + cube3),
                    Want::Volume(vd),
                    Want::Volume(cube3),
                    Want::Volume(0.0),
                ],
            );
        }
    }
    // Tier 3′ (the door gates at tier 3; the census is parked,
    // `work/reachhold/boolean-door-runs-the-census-over-its-result.md`), at
    // every pose. The drum ∖ the inner cube is the drum with a cubic void
    // whose corner touches the wall, recorded vertex-on-face, and passes.
    // The drum ∪ the outer cube carries the same record, confirmed, beside
    // a curved pair the census's cross-solid lane cannot decide
    // (`work/restread/census-cross-solid-curved-pairs-undecidable-on-shell-results.md`):
    // red when that is fixed.
    let tol = Tol::witness();
    for l in [0.65, 0.8] {
        for spin in [0.0, 0.4] {
            let body = |r: Result<BooleanResult<f64>, BooleanError>| {
                let Ok(BooleanResult::Body(b)) = r else {
                    panic!("l {l} spin {spin}: a body");
                };
                b
            };
            let void = body(topo::subtract(&drum, &cube_at(p, -n, spin, l), tol));
            assert_eq!(
                (
                    void.body.shells().count(),
                    void.contacts.a_on_b.len() + void.contacts.b_on_a.len()
                ),
                (2, 1),
                "l {l} spin {spin}: drum ∖ inner is the drum and a void, its corner on the wall"
            );
            topo::validate_pseudomanifold(&void.body, &void.contacts, tol)
                .unwrap_or_else(|e| panic!("l {l} spin {spin}: drum ∖ inner, tier 3′: {e:?}"));
            let pair = body(topo::union(&drum, &cube_at(p, n, spin, l), tol));
            let undecided = topo::validate_pseudomanifold(&pair.body, &pair.contacts, tol)
                .expect_err("below tier 3′");
            assert!(
                !undecided.is_empty()
                    && undecided
                        .iter()
                        .all(|e| matches!(e, topo::ValidationError::CensusUndecidable { .. })),
                "l {l} spin {spin}: drum ∪ outer is the undecidable curved pair alone: {undecided:?}"
            );
        }
    }
}

/// What tier 3′ reads of `record` alone, injected into an otherwise
/// empty record set of `body`: `Ok`, or the errors naming its vertex or
/// face.
fn vf_reading(
    body: &topo::Body<f64>,
    record: topo::VfContact,
) -> Result<(), Vec<topo::ValidationError>> {
    let contacts = topo::ContactRecords {
        b_on_a: vec![topo::Cited::new(record, topo::Cites::decided(0))],
        ..topo::ContactRecords::default()
    };
    let errors: Vec<_> = topo::validate_pseudomanifold(body, &contacts, Tol::witness())
        .err()
        .unwrap_or_default()
        .into_iter()
        .filter(|e| match e {
            topo::ValidationError::StaleContactDeclaration {
                declaration: topo::StaleDeclaration::VertexOnFace { vertex, face },
            } => (*vertex, *face) == (record.vertex, record.face),
            topo::ValidationError::CensusUnsupported {
                subject: topo::CensusSubject::Entity(topo::EntityId::Face(face)),
                ..
            } => *face == record.face,
            _ => false,
        })
        .collect();
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// **Tier 3′ reads a vertex-on-face record on a curved face by the
/// curved containment door.** The drum less a cube whose corner touches
/// the wall from inside, the corner's record injected onto each face it
/// could name: its own wall face confirms it; the other wall face, a
/// vertex on the wall's boundary, and the corner of a cube set `δ` in
/// from the wall past the band's escalation edge read it stale; a cube
/// set in by half the band confirms it. A record on a face whose trim the door
/// does not read (a swept elbow's NURBS wall) refuses typed.
#[test]
fn a_vertex_on_a_curved_face_is_confirmed_by_its_trim() {
    let tol = Tol::witness();
    let drum = finished("the drum", cyl(1.0, 2.0), tol);
    let phi = 0.7_f64;
    let n = Vec3::new(phi.cos(), phi.sin(), 0.0);
    let p = Point3::new(phi.cos(), phi.sin(), 0.3);
    let stale = |r: &Result<(), Vec<topo::ValidationError>>| {
        matches!(
            r.as_ref().map_err(Vec::as_slice),
            Err([topo::ValidationError::StaleContactDeclaration { .. }])
        )
    };
    let eps = tol.eps();
    // In the band (half of it), past its escalation edge (`10ε`), far.
    for delta in [0.0, 0.5 * eps, 20.0 * eps, 1e-4] {
        let corner_at = p - n * delta;
        let Ok(BooleanResult::Body(b)) =
            topo::subtract(&drum, &cube_at(corner_at, -n, 0.0, 0.65), tol)
        else {
            panic!("δ {delta}: drum ∖ inner builds");
        };
        let body = &b.body;
        let (corner, _) = body
            .vertex_points()
            .min_by(|(_, a), (_, b)| a.distance(corner_at).total_cmp(&b.distance(corner_at)))
            .unwrap();
        let walls: Vec<_> = body
            .faces()
            .filter(|(_, f)| {
                matches!(
                    body.get_surface(f.surface),
                    Some(geom::Surface::Cylinder { .. })
                )
            })
            .map(|(k, _)| k)
            .collect();
        assert_eq!(walls.len(), 2, "δ {delta}: the drum's two wall faces");
        let reads: Vec<_> = walls
            .iter()
            .map(|&face| {
                vf_reading(
                    body,
                    topo::VfContact {
                        vertex: corner,
                        face,
                    },
                )
            })
            .collect();
        if delta < eps {
            assert_eq!(
                reads.iter().filter(|r| r.is_ok()).count(),
                1,
                "δ {delta}: the corner confirms on its own wall face: {reads:?}"
            );
            assert_eq!(
                reads.iter().filter(|r| stale(r)).count(),
                1,
                "δ {delta}: and is stale on the other: {reads:?}"
            );
        } else {
            assert!(
                reads.iter().all(stale),
                "δ {delta}: a corner off the wall is stale on both: {reads:?}"
            );
        }
        if delta == 0.0 {
            // A vertex on a wall face's own boundary is not inside it.
            let face = walls[0];
            let rim = body
                .vertex_points()
                .map(|(v, _)| v)
                .find(|&v| body.faces_of_vertex(v).is_some_and(|fs| fs.contains(&face)))
                .expect("a vertex on the wall face's boundary");
            let r = vf_reading(body, topo::VfContact { vertex: rim, face });
            assert!(stale(&r), "a boundary vertex is stale: {r:?}");
        }
    }
    let elbow = sweep::test_support::swept_elbow(tol);
    let (face, vertex) = elbow
        .faces()
        .filter(|(_, f)| matches!(elbow.get_surface(f.surface), Some(geom::Surface::Nurbs(_))))
        .find_map(|(face, _)| {
            elbow
                .vertex_points()
                .map(|(v, _)| v)
                .find(|&v| {
                    elbow
                        .faces_of_vertex(v)
                        .is_some_and(|fs| !fs.contains(&face))
                })
                .map(|v| (face, v))
        })
        .expect("a NURBS wall and a vertex off it");
    let r = vf_reading(&elbow, topo::VfContact { vertex, face });
    assert!(
        matches!(
            r.as_ref().map_err(Vec::as_slice),
            Err([topo::ValidationError::CensusUnsupported {
                cause: topo::CensusUnsupportedCause::ContactLane(
                    topo::ContactRefusal::NotCertifiable { .. }
                ),
                ..
            }])
        ),
        "a NURBS wall's trim has no reading: {r:?}"
    );
}
