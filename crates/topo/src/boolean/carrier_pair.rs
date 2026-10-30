//! **The carrier-pair doors**: one cross-body face pair's carrier
//! verdict, on every kind the `Rest` table names ([`carrier_pair_relation`],
//! [`carrier_pair_decided`]), its planar restriction
//! ([`flush_pair_relation`]), the oriented descriptions they compare
//! ([`face_carrier`]) and the extent a verdict is consumed over
//! ([`pair_extent`]).

use geom_brep::ExtentBall;
use geom_core::{Band, Decide, Point3};

use super::carrier_eq::{CarrierDesc, CarrierEqError, CarrierRelation};
use super::plane_eq::{PlaneEqError, PlaneRelation};
use super::reduce::face_plane;
use crate::body::Body;
use crate::entity::{EntityId, FaceKey};
use crate::face_normal::plane_outward_normal;
use crate::live::BoundaryMember;

/// **The one flush-pair door**: the carrier ladder for a single
/// cross-body PLANAR face pair — [`carrier_pair_relation`] restricted
/// to two planes (descriptions through [`face_plane`], outward and
/// sense-folded, so REST contact is precisely the `SameOpposite`
/// verdict). It is that door, not a mirror of it: the
/// verdict, the `decide` sites and the lever are the ones every carrier
/// pair gets.
///
/// **This door has NO in-tree consumer.** Verify-at-use stopped
/// calling it at M9-1 and the flush detector followed when its scope
/// became the `Rest` ladder's; what to do about a published door with
/// no caller is `work/probe/flush-pair-relation-has-no-caller.md`.
///
/// `Err`: a face that is not a plane, which this door has no
/// description for ([`PairUnread::OutsideInventory`];
/// [`carrier_pair_relation`] is where a caller asks the same question
/// of any carrier the ladder names), or whose extent cannot be read.
///
/// # Errors
///
/// [`PairUnread`], as above.
///
/// # Panics
///
/// Where a face resolves and its surface does not, or a link
/// [`carrier_pair_relation`] reads does not resolve: a torn surface is
/// not one outside the inventory.
pub fn flush_pair_relation<T: Decide>(
    a: &Body<T>,
    fa: FaceKey,
    b: &Body<T>,
    fb: FaceKey,
    band: Band,
) -> Result<Result<PlaneRelation, PlaneEqError>, PairUnread> {
    face_plane(a, fa).ok_or(PairUnread::OutsideInventory)?;
    face_plane(b, fb).ok_or(PairUnread::OutsideInventory)?;
    carrier_pair_relation(a, fa, b, fb, band)
}

/// Which face of a pair, in the order a door took them.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PairFace {
    /// The first face.
    First,
    /// The second face.
    Second,
}

/// Why a face pair has no carrier reading: the door's input, not a
/// verdict.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PairUnread {
    /// A face's surface kind is outside the ladder's
    /// [`super::carrier_eq::INVENTORY`], or its key does not resolve:
    /// there is no description to compare.
    OutsideInventory,
    /// The face's consumed extent cannot be read ([`pair_extent`]): its
    /// key does not resolve, its box has no claim to make (a NURBS
    /// placeholder, a boundary edge with no sound box), or the ball
    /// around it does not read.
    Extent(PairFace),
}

/// **A face's consumed extent**: a ball enclosing every point of it,
/// the region over which a verdict about its carrier is consumed
/// ([`pair_extent`]). A sphere's or a torus's own ball
/// ([`ExtentBall::of_carrier`]: the torus's `R + r`, whatever the
/// trim); otherwise the ball around the face's certified box, from the
/// kernel's one kind→box rule (`census::face_reach`). `None` where
/// `face`, the caller's key, does not resolve, where that box has no
/// claim to make, or where the ball does not read. The face's surface
/// is a link, and its miss panics (on an at-rest operand by tier 1,
/// mid-operation by [`crate::live::OPERATORS_KEEP_LINKS`]).
fn face_ball<T: Decide>(body: &Body<T>, face: FaceKey, band: Band) -> Option<ExtentBall<T>> {
    let f = body.get_face(face)?;
    let ball = match ExtentBall::of_carrier(body.face_surface_linked(face, f)) {
        Some(ball) => ball,
        None => {
            let (lo, hi) = crate::census::face_reach(body, face, band)?;
            ExtentBall::of_box(lo, hi)
        }
    };
    ball.readable()
}

/// The face's boundary vertex positions (outer loop then rings; an
/// empty loop contributes its lone vertex): points known to lie on the
/// face. `None` where `face`, the caller's key, does not resolve.
///
/// Every hop past the face is a link (its loops, their walks, each
/// member's edge, start vertex and its point, a lone vertex's point; a null strut's half-edges walk
/// like any other), and a miss panics (on an at-rest body by tier 1,
/// mid-operation by [`crate::live::OPERATORS_KEEP_LINKS`]).
pub(crate) fn face_witnesses<T: Decide>(body: &Body<T>, face: FaceKey) -> Option<Vec<Point3<T>>> {
    let f = body.get_face(face)?;
    Some(
        body.face_boundary_linked(face, f)
            .map(|member| match member {
                BoundaryMember::Isolated { point, .. } => point,
                BoundaryMember::Edge { he, half, .. } => {
                    body.linked_vertex_point(half.start, EntityId::HalfEdge(he), "start")
                }
            })
            .collect(),
    )
}

/// **A face pair's consumed extent**, as the carrier doors read it:
/// one ball enclosing both faces ([`face_ball`]), since the verdict is
/// consumed on each ([`super::carrier_eq::ConsumedExtent`]).
#[derive(Clone, Copy, Debug)]
pub(crate) struct PairExtent<T: geom_core::Real> {
    /// The ball enclosing both faces.
    pub(crate) reach: ExtentBall<T>,
}

impl<T: geom_core::Real> PairExtent<T> {
    /// The extent as the ladder reads it.
    pub(crate) fn consumed(&self) -> super::carrier_eq::ConsumedExtent<T> {
        super::carrier_eq::ConsumedExtent::new(self.reach)
    }
}

/// [`PairExtent`] of `fa` on `a` and `fb` on `b`. Read on the operands
/// at rest: mid-operation, a face whose boundary carries null
/// scaffolding has no box to read, and the sites there take a glued
/// pair's extent from [`super::GluedPairs::reach_of`].
///
/// # Errors
///
/// The face whose extent cannot be read ([`PairUnread::Extent`]).
pub(crate) fn pair_extent<T: Decide>(
    a: &Body<T>,
    fa: FaceKey,
    b: &Body<T>,
    fb: FaceKey,
    band: Band,
) -> Result<PairExtent<T>, PairFace> {
    let ball_a = face_ball(a, fa, band).ok_or(PairFace::First)?;
    let ball_b = face_ball(b, fb, band).ok_or(PairFace::Second)?;
    // Two readable balls enclose readably short of overflow, which the
    // larger reach is what tips; the second face takes the blame
    // rather than neither.
    let reach = ExtentBall::enclosing(&[ball_a, ball_b])
        .and_then(ExtentBall::readable)
        .ok_or(PairFace::Second)?;
    Ok(PairExtent { reach })
}

/// The face's **oriented carrier description** — the curved
/// generalization of [`face_plane`], folding the face's sense into
/// the material side exactly as that door does (S10).
///
/// `None` for a surface kind outside the `Rest` ladder's
/// [`super::carrier_eq::INVENTORY`]: the C4 table names the kinds
/// [`mod@super::carrier_eq`] carries a rung for, and a kind it cannot
/// compare refuses typed at the caller rather than being approximated
/// by one it can. `None` too where `face`, the caller's key, does not
/// resolve.
///
/// # Panics
///
/// Where the face's surface does not resolve: a link, which every
/// public door keeps live, and which the reduction's operators keep
/// live mid-operation ([`crate::live::OPERATORS_KEEP_LINKS`]).
pub fn face_carrier<T: Decide>(body: &Body<T>, face: FaceKey) -> Option<CarrierDesc<T>> {
    let f = body.get_face(face)?;
    // `sense` is the material-side bit: true means the face's outward
    // normal IS the chart normal, which for a sphere/cylinder chart
    // points away from the centre/axis. Read as a BIT, never as a
    // comparison on `T` — the scalar backends order intervals, not
    // signs (S10's exact-bit discipline).
    let outward = f.sense;
    match body.face_surface_linked(face, f) {
        geom::Surface::Plane { origin, normal, .. } => Some(CarrierDesc::Plane {
            origin: *origin,
            normal: plane_outward_normal(f, *normal).vec(),
        }),
        geom::Surface::Sphere { center, radius, .. } => Some(CarrierDesc::Sphere {
            center: *center,
            radius: *radius,
            outward,
        }),
        geom::Surface::Cylinder {
            origin,
            axis,
            radius,
            ..
        } => Some(CarrierDesc::Cylinder {
            origin: *origin,
            axis: *axis,
            radius: *radius,
            outward,
        }),
        geom::Surface::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            ..
        } => Some(CarrierDesc::Torus {
            center: *center,
            axis: *axis,
            major_radius: *major_radius,
            minor_radius: *minor_radius,
            outward,
        }),
        geom::Surface::Cone {
            apex,
            axis,
            half_angle,
            ..
        } => Some(CarrierDesc::Cone {
            apex: *apex,
            axis: *axis,
            half_angle: *half_angle,
            outward,
        }),
        geom::Surface::Nurbs(_) | geom::Surface::Approx(_) => None,
    }
}

/// **The one carrier-pair door**: [`flush_pair_relation`] for every
/// carrier kind the `Rest` table names.
///
/// Descriptions through [`face_carrier`] and the pair's consumed
/// extent through [`pair_extent`]: a coincidence is one whose
/// displacement's bound decides Zero at every point of both faces
/// ([`super::carrier_eq::pair_door_reading`]). The operation's glue
/// door reads its pairs here, and so do the census and the merge.
///
/// # Errors
///
/// [`PairUnread`]: a face whose surface kind is outside the ladder's
/// inventory — there is no description to compare — or whose extent
/// cannot be read. The ladder's own refusals ride inside the `Ok`.
///
/// # Panics
///
/// Where a link of either face does not resolve: its surface
/// ([`face_carrier`]), its loops, their walks, or a boundary vertex's
/// point.
pub fn carrier_pair_relation<T: Decide>(
    a: &Body<T>,
    fa: FaceKey,
    b: &Body<T>,
    fb: FaceKey,
    band: Band,
) -> Result<Result<CarrierRelation, CarrierEqError>, PairUnread> {
    Ok(carrier_pair_decided(a, fa, b, fb, band)?.map(|(rel, _)| rel))
}

/// [`carrier_pair_relation`] with the margin that decided a coincidence
/// ([`super::carrier_eq::CarrierReading`]): the margin the glue door
/// records, and the reading the coincidence door runs on two cells'
/// carriers in a replay at `Sym`, where the margin says whether the
/// symbolic tier proved every Zero the verdict stands on
/// ([`geom_core::MarginDiag::proven`]). `None` beside `Distinct`.
///
/// # Errors
///
/// As [`carrier_pair_relation`].
///
/// # Panics
///
/// As [`carrier_pair_relation`].
pub fn carrier_pair_decided<T: Decide>(
    a: &Body<T>,
    fa: FaceKey,
    b: &Body<T>,
    fb: FaceKey,
    band: Band,
) -> Result<Result<(CarrierRelation, Option<geom_core::MarginDiag>), CarrierEqError>, PairUnread> {
    let ca = face_carrier(a, fa).ok_or(PairUnread::OutsideInventory)?;
    let cb = face_carrier(b, fb).ok_or(PairUnread::OutsideInventory)?;
    let extent = pair_extent(a, fa, b, fb, band).map_err(PairUnread::Extent)?;
    Ok(super::carrier_eq::pair_door_reading(
        &ca,
        &cb,
        &extent.consumed(),
        band,
    ))
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use crate::entity::LoopBoundary;
    use geom_core::Tol;

    /// **The carrier reads answer a caller's stale face `None` and panic
    /// on a torn link past it.** A face the body once held, freed so the
    /// body around it is sound, reads no carrier, ball or witnesses; a
    /// live face whose surface was dropped panics in `face_carrier` and
    /// `face_ball` naming the surface, and one whose boundary vertex lost
    /// its point panics in `face_witnesses` naming the point. A read that
    /// took either tear for an absent record would answer instead.
    #[test]
    fn the_carrier_reads_answer_a_stale_face_none_and_panic_on_a_torn_link() {
        use crate::review_d18::{ROW_FOUR, assert_torn_op_panics};
        let band = Band::linear(Tol::witness()).unwrap();
        let fresh = || crate::test_support_fixtures::geometric_cube::<f64>(Tol::witness()).body;
        let mut body = fresh();
        let (face, data) = body.faces().next().map(|(k, f)| (k, f.clone())).unwrap();
        let stale = body.faces.insert(data.clone());
        body.faces.remove(stale);
        assert!(
            face_carrier(&body, face).is_some()
                && face_ball(&body, face, band).is_some()
                && face_witnesses(&body, face).is_some(),
            "the live face reads"
        );
        assert!(face_carrier(&body, stale).is_none(), "face_carrier, stale");
        assert!(face_ball(&body, stale, band).is_none(), "face_ball, stale");
        assert!(
            face_witnesses(&body, stale).is_none(),
            "face_witnesses, stale"
        );

        body.surfaces.remove(data.surface);
        let surface = format!("{}'s surface names", EntityId::Face(face));
        assert_torn_op_panics("face_carrier", &mut body, &[&surface, ROW_FOUR], |b| {
            face_carrier(b, face)
        });
        assert_torn_op_panics("face_ball", &mut body, &[&surface, ROW_FOUR], |b| {
            face_ball(b, face, band)
        });

        let mut body = fresh();
        let LoopBoundary::Cycle { first } = body.get_loop(data.outer).unwrap().boundary else {
            panic!("the cube's faces are bounded by cycles");
        };
        let vertex = body.get_half_edge(first).unwrap().start;
        let point = body.get_vertex(vertex).unwrap().point;
        body.points.remove(point);
        let named = format!("{}'s point names", EntityId::Vertex(vertex));
        assert_torn_op_panics("face_witnesses", &mut body, &[&named, ROW_FOUR], |b| {
            face_witnesses(b, face)
        });
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod lever_rows {
    use super::*;
    use crate::boolean::boxes::tests::torus_wall;
    use crate::contact::ContactRefusal;
    use crate::test_support::{CylFrame, cyl_wall_sheet};
    use geom_core::{Point3, Tol, Vec3};

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    fn door(a: &Body<f64>, fa: FaceKey, b: &Body<f64>, fb: FaceKey) -> Result<(), ContactRefusal> {
        crate::boolean::rest_pair_verdict(a, fa, b, fb, band()).map(|_| ())
    }

    /// A torus of `R + r = 2.5 m` against its twin tilted by `θ` about
    /// `y` through the shared centre. At `θ = 0.6·Kε`, in band at one
    /// metre, the tilt moves the tube's core circle by `θ·R = 1.2·Kε`,
    /// past the band: the door reads two carriers and contradicts, as it
    /// does at `2·Kε`.
    #[test]
    fn a_torus_tilt_is_read_at_the_ring() {
        for k in [0.6, 2.0] {
            let theta = k * band().escalate();
            let (u, v) = ((0.3, 1.9), (-0.7, 0.8));
            let (a, fa) = torus_wall(
                Point3::origin(),
                Vec3::unit_z(),
                Vec3::unit_x(),
                2.0,
                0.5,
                u,
                v,
            );
            let (mut b, fb) = torus_wall(
                Point3::origin(),
                Vec3::new(theta.sin(), 0.0, theta.cos()),
                Vec3::new(theta.cos(), 0.0, -theta.sin()),
                2.0,
                0.5,
                u,
                v,
            );
            let sense = a.get_face(fa).unwrap().sense;
            b.set_face_sense(fb, !sense).unwrap();
            assert_ne!(
                b.get_face(fb).unwrap().sense,
                sense,
                "{k}·Kε: the twin is the opposed face"
            );
            let read = door(&a, fa, &b, fb);
            assert!(
                matches!(read, Err(ContactRefusal::Contradicted { .. })),
                "{k}·Kε: the swing at the ring is past the band: {read:?}"
            );
        }
    }

    /// A 10 m cylinder wall against its twin tilted by `0.5·K·ε` about
    /// `y` through the shared axis point at its foot: `0.5·Kε` at one
    /// metre, while the far rim stands `5·Kε` off.
    #[test]
    fn a_cylinder_tilt_is_read_at_the_far_rim() {
        let tol = Tol::witness();
        let theta = 0.5 * band().escalate();
        let (u, v) = ((0.2, 1.6), (0.0, 10.0));
        let mut a = Body::<f64>::new();
        let fa = cyl_wall_sheet(&mut a, CylFrame::canonical(1.0), u, v, tol);
        let mut b = Body::<f64>::new();
        let fb = cyl_wall_sheet(&mut b, CylFrame::tilted(1.0, theta), u, v, tol);
        let sense = a.get_face(fa).unwrap().sense;
        b.set_face_sense(fb, !sense).unwrap();
        let read = door(&a, fa, &b, fb);
        assert!(
            matches!(read, Err(ContactRefusal::Contradicted { .. })),
            "the tilt at the far rim contradicts the contact: {read:?}"
        );
    }

    /// **R1 probe (review of #6079).** The torus lever row's `0.6·Kε`
    /// tilt: every point of the patch (sampled on a 41×41 grid of its
    /// chart) stands within `Kε` of the tilted twin's carrier, so the two
    /// faces are in band of each other over their whole extent, yet the
    /// door reads two carriers definitely apart and contradicts: the ring
    /// the torus datum is read at lies off the patch.
    #[test]
    fn r1_probe_a_torus_pair_in_band_over_its_patch_reads_definite() {
        let k = 0.6;
        let theta = k * band().escalate();
        let (u, v) = ((0.3, 1.9), (-0.7, 0.8));
        let (big, small) = (2.0, 0.5);
        let (a, fa) = torus_wall(
            Point3::origin(),
            Vec3::unit_z(),
            Vec3::unit_x(),
            big,
            small,
            u,
            v,
        );
        let (mut b, fb) = torus_wall(
            Point3::origin(),
            Vec3::new(theta.sin(), 0.0, theta.cos()),
            Vec3::new(theta.cos(), 0.0, -theta.sin()),
            big,
            small,
            u,
            v,
        );
        let sense = a.get_face(fa).unwrap().sense;
        b.set_face_sense(fb, !sense).unwrap();
        // The distance from `p` to the twin: `p` turned back by θ about y,
        // read against the untilted torus.
        let to_twin = |x: f64, y: f64, z: f64| {
            let (s, c) = theta.sin_cos();
            let (x, z) = (c * x - s * z, s * x + c * z);
            let rho = (x * x + y * y).sqrt();
            (((rho - big).powi(2) + z * z).sqrt() - small).abs()
        };
        let n = 40;
        let mut worst: f64 = 0.0;
        for i in 0..=n {
            for j in 0..=n {
                let uu = u.0 + (u.1 - u.0) * f64::from(i) / f64::from(n);
                let vv = v.0 + (v.1 - v.0) * f64::from(j) / f64::from(n);
                let r = big + small * vv.cos();
                worst = worst.max(to_twin(r * uu.cos(), r * uu.sin(), small * vv.sin()));
            }
        }
        let read = door(&a, fa, &b, fb);
        assert!(
            worst < band().escalate() && worst > band().zero(),
            "the patch stands in band of its twin: worst {worst:e}"
        );
        assert!(
            !matches!(read, Err(ContactRefusal::Contradicted { .. })),
            "a pair in band over its whole extent reads definite (Contradicted): worst {worst:e}, \
             {read:?}"
        );
    }

    /// **The offset and the tilt in band refuse.** A 1 m band of
    /// cylinder wall at `9 ≤ v ≤ 10` against its twin tilted by
    /// `0.105·K·ε` about `y` through the axis' foot at the origin: the
    /// axis offset at the patch's middle reads `≈ 1.0·Kε` (just in band),
    /// the tilt over the patch's extent another fraction of it, each in
    /// band, while the rim stands `1.05·Kε` off. The door decides no
    /// carrier, and bridges nothing: it escalates.
    #[test]
    fn an_offset_and_a_tilt_in_band_each_do_not_bridge_their_sum() {
        let tol = Tol::witness();
        let theta = 0.105 * band().escalate();
        let (u, v) = ((-0.3, 0.3), (9.0, 10.0));
        let mut a = Body::<f64>::new();
        let fa = cyl_wall_sheet(&mut a, CylFrame::canonical(1.0), u, v, tol);
        let mut b = Body::<f64>::new();
        let fb = cyl_wall_sheet(&mut b, CylFrame::tilted(1.0, theta), u, v, tol);
        let sense = a.get_face(fa).unwrap().sense;
        b.set_face_sense(fb, !sense).unwrap();
        let read = door(&a, fa, &b, fb);
        assert!(
            matches!(read, Err(ContactRefusal::Escalated { .. })),
            "the pair in band escalates: {read:?}"
        );
    }
}

/// A surface's carrier description as its chart reads it — the outward
/// side the chart's own normal, since the coincidence question
/// ([`rest_locus`]) does not read material sides. `None` for a kind
/// outside the ladder's inventory, as [`face_carrier`].
fn chart_carrier<T: Decide>(surface: &geom::Surface<T>) -> Option<CarrierDesc<T>> {
    match surface {
        geom::Surface::Plane { origin, normal, .. } => Some(CarrierDesc::Plane {
            origin: *origin,
            normal: *normal,
        }),
        geom::Surface::Sphere { center, radius, .. } => Some(CarrierDesc::Sphere {
            center: *center,
            radius: *radius,
            outward: true,
        }),
        geom::Surface::Cylinder {
            origin,
            axis,
            radius,
            ..
        } => Some(CarrierDesc::Cylinder {
            origin: *origin,
            axis: *axis,
            radius: *radius,
            outward: true,
        }),
        geom::Surface::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            ..
        } => Some(CarrierDesc::Torus {
            center: *center,
            axis: *axis,
            major_radius: *major_radius,
            minor_radius: *minor_radius,
            outward: true,
        }),
        geom::Surface::Cone {
            apex,
            axis,
            half_angle,
            ..
        } => Some(CarrierDesc::Cone {
            apex: *apex,
            axis: *axis,
            half_angle: *half_angle,
            outward: true,
        }),
        geom::Surface::Nurbs(_) | geom::Surface::Approx(_) => None,
    }
}

/// **Where a conventional edge between two surfaces rests** — the one
/// home of D2's split between a locus one chart defines and one two
/// surfaces under-determine ([`geom_brep::RestLocus`]); every door
/// states what it decides.
///
/// - One surface record on both sides is one surface: its chart.
/// - Two records are read by the carrier-equality ladder
///   ([`super::carrier_eq::carrier_eq`]) over the region
///   the description is consumed on: a ball enclosing the edge's own
///   extent and each named face's boundary vertices (`sides`' faces,
///   where the face exists yet; a chord minted before its section face
///   names only the face it divides). The faces, not the edge alone:
///   two planes meeting smoothly over a short edge are still two
///   surfaces where their faces spread. A coincidence decided **Zero**
///   is one carrier, and incidence on both of one carrier is incidence
///   on one, which determines nothing: the edge rests in the chart of
///   the pair's first key.
/// - Definitely **distinct** carriers are the pair, and so is a kind
///   outside the ladder's [`super::carrier_eq::INVENTORY`], which has no
///   coincidence to read.
/// - A coincidence **in band**, a rung that cannot decide, or a region
///   that does not read is the pair too, and that is an answer, not a
///   guess (D4 forbids guessing): the pair's certificate is incidence
///   on both surfaces, which holds whether or not they are one carrier,
///   so the description is true under both readings and needs no
///   decision. D2's same-surface clause covers surfaces decided one.
///
/// **What a reader must not do with a pair.** An `OnPair` here may lie
/// between two near-coincident surfaces — two planes `ε` apart — whose
/// intersection is ill-conditioned or absent. The stored carrier is the
/// pair's defining data: no reader derives the edge's geometry from the
/// pair's section. A door that moves or re-anchors such an edge carries
/// or re-sections its CARRIER and re-asks the rule over it
/// ([`geom_brep::reask_pair`]); a door that would re-derive a section
/// from the moved surfaces refuses typed where that section does not
/// certify.
///
/// Each side's surface is `body`'s at its key, a link of the edge being
/// described.
///
/// # Panics
///
/// Where a side's surface or face does not resolve: links of the edge
/// being described.
pub fn rest_locus<T: Decide>(
    body: &Body<T>,
    sides: [(crate::SurfaceKey, Option<FaceKey>); 2],
    carrier: &geom::Curve3<T>,
    (t0, t1): (T, T),
    band: Band,
) -> geom_brep::RestLocus {
    let [(ka, fa), (kb, fb)] = sides;
    if ka == kb {
        return geom_brep::RestLocus::Chart(ka);
    }
    let pair = geom_brep::SurfacePair::new(ka, kb);
    let surface = |k: crate::SurfaceKey| {
        body.get_surface(k)
            .unwrap_or_else(|| unreachable!("{k:?} is a surface the described edge lies on"))
    };
    let (Some(ca), Some(cb)) = (chart_carrier(surface(ka)), chart_carrier(surface(kb))) else {
        return geom_brep::RestLocus::Pair(pair);
    };
    let on = |f: Option<FaceKey>| {
        f.map_or_else(Vec::new, |f| {
            face_witnesses(body, f)
                .unwrap_or_else(|| unreachable!("{f:?} is a face the described edge bounds"))
        })
    };
    let (on_a, on_b) = (on(fa), on(fb));
    // The ball about the edge's middle at its reach from there encloses
    // it, and is no larger than the edge's own measure from that point.
    let mid = carrier.mid_point(t0, t1);
    let edge = ExtentBall::new(mid, geom_brep::edge_reach_from(carrier, t0, t1, mid));
    let parts: Vec<_> = core::iter::once(edge)
        .chain(on_a.iter().chain(&on_b).map(|&p| ExtentBall::point(p)))
        .collect();
    let Some(reach) = ExtentBall::enclosing(&parts).and_then(ExtentBall::readable) else {
        return geom_brep::RestLocus::Pair(pair);
    };
    match super::carrier_eq::carrier_eq(&ca, &cb, &super::ConsumedExtent::new(reach), band) {
        Ok(CarrierRelation::SameOriented | CarrierRelation::SameOpposite) => {
            geom_brep::RestLocus::Chart(pair.keys()[0])
        }
        Ok(CarrierRelation::Distinct) | Err(_) => geom_brep::RestLocus::Pair(pair),
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod extent_rows {
    use super::*;
    use geom_core::{Point3, Tol, Vec3};

    /// **A steep spline edge rests on the pair two hinged planes make,
    /// read over the whole edge.** A rational quadratic arc (weights
    /// `1, 10, 1`) 2 m across its ends rises some 45 m to its vertex,
    /// in the plane `x = 0`. A second plane through the vertex is
    /// hinged off it by `θ = 0.45·ε` about the line along `ŷ` there,
    /// so it stands `θ·s` off at `s` metres down the arc: under a band
    /// within the chord's 2 m, past `K·ε` at the arc's ends. The rest
    /// locus reads the planes over a ball about the arc's middle (its
    /// vertex) at the edge's extent from above, which holds the ends:
    /// the pair. A ball at the chord reads them one chart.
    #[test]
    fn a_steep_spline_rests_on_the_pair_its_ends_separate() {
        let band = Band::linear(Tol::witness()).unwrap();
        let arc = geom::Curve3::Nurbs(std::sync::Arc::new(
            geom::NurbsCurve3::new(
                geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2)
                    .unwrap(),
                vec![
                    Point3::new(0.0, -1.0, 0.0),
                    Point3::new(0.0, 0.0, 50.0),
                    Point3::new(0.0, 1.0, 0.0),
                ],
                vec![1.0, 10.0, 1.0],
            )
            .unwrap(),
        ));
        let vertex = arc.mid_point(0.0, 1.0);
        let chord = arc.eval(0.0).distance(arc.eval(1.0));
        let theta = 0.45 * band.zero();
        assert!(
            chord * theta <= band.zero()
                && vertex.distance(arc.eval(0.0)) * theta >= band.escalate(),
            "the hinge is under a band within the chord and past K·ε at the ends"
        );
        let mut body = Body::<f64>::new();
        let mut plane = |normal: Vec3<f64>| {
            body.add_surface(geom::Surface::Plane {
                origin: vertex,
                normal,
                u_ref: Vec3::unit_y(),
            })
        };
        let (ka, kb) = (
            plane(Vec3::unit_x()),
            plane(Vec3::new(theta.cos(), 0.0, theta.sin())),
        );
        match rest_locus(&body, [(ka, None), (kb, None)], &arc, (0.0, 1.0), band) {
            geom_brep::RestLocus::Pair(pair) => assert_eq!(
                pair,
                geom_brep::SurfacePair::new(ka, kb),
                "the pair of the two planes"
            ),
            geom_brep::RestLocus::Chart(k) => {
                panic!("the hinged planes read one chart {k:?} over the steep edge")
            }
        }
    }

    /// **The rest ball reaches a spike the edge's samples miss.** A
    /// polyline spline (degree 1) runs about a metre each way of its
    /// middle `C`, but between `t = 0.19` and `0.21` it spikes 100 m off
    /// it, between every one of the samples the extent from below reads.
    /// Two planes through `C` hinged by `θ` stand under a band apart over
    /// that bound from below and past `K·ε` at the spike, so a ball at the
    /// bound from below reads one chart. The ball reads the edge's reach
    /// from its middle, which holds the spike: the pair.
    #[test]
    fn the_rest_ball_reaches_a_spike_the_samples_miss() {
        let band = Band::linear(Tol::witness()).unwrap();
        let p = Point3::new;
        let arc = geom::Curve3::Nurbs(std::sync::Arc::new(
            geom::NurbsCurve3::new(
                geom_core::spline::KnotVector::clamped(
                    vec![0.0, 0.0, 0.19, 0.2, 0.21, 0.5, 1.0, 1.0],
                    1,
                )
                .unwrap(),
                vec![
                    p(0.0, -1.0, 0.0),
                    p(0.0, -0.95, 0.0),
                    p(0.0, 0.0, 100.0),
                    p(0.0, -0.05, 0.0),
                    p(0.0, 0.0, 0.0),
                    p(0.0, 1.0, 0.0),
                ],
                vec![1.0; 6],
            )
            .unwrap(),
        ));
        let mid = arc.mid_point(0.0, 1.0);
        let chord = arc.eval(0.0).distance(arc.eval(1.0));
        let below = geom_brep::edge_extent_inf(&arc, 0.0, 1.0, chord);
        let reach = geom_brep::edge_reach_from(&arc, 0.0, 1.0, mid);
        let theta = 0.45 * band.zero() / below;
        assert!(
            reach * theta >= band.escalate(),
            "the spike stands past K·ε: reach {reach}, the bound from below {below}"
        );
        let mut body = Body::<f64>::new();
        let mut plane = |normal: Vec3<f64>| {
            body.add_surface(geom::Surface::Plane {
                origin: mid,
                normal,
                u_ref: Vec3::unit_y(),
            })
        };
        let (ka, kb) = (
            plane(Vec3::unit_x()),
            plane(Vec3::new(theta.cos(), 0.0, theta.sin())),
        );
        match rest_locus(&body, [(ka, None), (kb, None)], &arc, (0.0, 1.0), band) {
            geom_brep::RestLocus::Pair(pair) => assert_eq!(
                pair,
                geom_brep::SurfacePair::new(ka, kb),
                "the pair of the two planes"
            ),
            geom_brep::RestLocus::Chart(k) => {
                panic!("the hinged planes read one chart {k:?} past the spike")
            }
        }
    }
}
