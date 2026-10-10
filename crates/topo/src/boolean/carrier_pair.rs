//! **The carrier-pair doors**: one cross-body face pair's carrier
//! verdict, on every kind the `Rest` table names ([`carrier_pair_relation`],
//! [`carrier_pair_verdict`]), its planar restriction
//! ([`flush_pair_relation`]), the oriented descriptions they compare
//! ([`face_carrier`]) and the extent a verdict is consumed over
//! ([`pair_extent`]).

use geom_brep::ExtentBall;
use geom_core::{Band, Decide, Point3};

use super::carrier_eq::{CarrierDesc, CarrierEqError, CarrierRelation};
use super::plane_eq::{PlaneEqError, PlaneIdentity, PlaneRelation};
use super::reduce::face_plane;
use crate::body::Body;
use crate::entity::{EntityId, FaceKey};
use crate::face_normal::plane_outward_normal;
use crate::live::BoundaryMember;

/// **The one flush-pair door**: the C4 verify ladder for a single
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
/// no caller is `work/seat/flush-pair-relation-has-no-caller.md`.
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
    declared: bool,
    band: Band,
) -> Result<Result<PlaneRelation, PlaneEqError>, PairUnread> {
    face_plane(a, fa).ok_or(PairUnread::OutsideInventory)?;
    face_plane(b, fb).ok_or(PairUnread::OutsideInventory)?;
    carrier_pair_relation(a, fa, b, fb, declared, band)
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
    /// A face's surface kind is outside the ladder's inventory (cone,
    /// NURBS, `Approx`), or its key does not resolve: there is no
    /// description to compare.
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
/// consumed on each, and each face's boundary vertices, the points
/// known to be consumed ([`super::carrier_eq::ConsumedExtent`]).
#[derive(Clone, Debug)]
pub(crate) struct PairExtent<T: geom_core::Real> {
    /// The ball enclosing both faces.
    pub(crate) reach: ExtentBall<T>,
    /// The first face's boundary vertices, then the second's.
    pub(crate) on: [Vec<Point3<T>>; 2],
}

impl<T: geom_core::Real> PairExtent<T> {
    /// The extent as the ladder reads it, its faces in the order
    /// measured, or the other way round when `swapped`.
    pub(crate) fn consumed(&self, swapped: bool) -> super::carrier_eq::ConsumedExtent<'_, T> {
        let [first, second] = &self.on;
        super::carrier_eq::ConsumedExtent {
            reach: self.reach,
            on: if swapped {
                [second, first]
            } else {
                [first, second]
            },
        }
    }
}

/// [`PairExtent`] of `fa` on `a` and `fb` on `b`. Read on the operands
/// at rest: mid-operation, a face whose boundary carries null
/// scaffolding has no box to read, and the sites there take a declared
/// pair's extent from [`super::DeclaredPairs::consumed`].
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
    let read = |body, face| face_ball(body, face, band).zip(face_witnesses(body, face));
    let (ball_a, on_a) = read(a, fa).ok_or(PairFace::First)?;
    let (ball_b, on_b) = read(b, fb).ok_or(PairFace::Second)?;
    // Two readable balls enclose readably short of overflow, which the
    // larger reach is what tips; the second face takes the blame
    // rather than neither.
    let reach = ExtentBall::enclosing(&[ball_a, ball_b])
        .and_then(ExtentBall::readable)
        .ok_or(PairFace::Second)?;
    Ok(PairExtent {
        reach,
        on: [on_a, on_b],
    })
}

/// The face's **oriented carrier description** — the curved
/// generalization of [`face_plane`], folding the face's sense into
/// the material side exactly as that door does (S10).
///
/// `None` for a surface kind outside the `Rest` ladder's inventory
/// (cone, NURBS, `Approx`): the C4 table names the kinds
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
        _ => None,
    }
}

/// **The one carrier-pair door**: [`flush_pair_relation`] for every
/// carrier kind the `Rest` table names.
///
/// Descriptions through [`face_carrier`] and the pair's consumed
/// extent through [`pair_extent`]: a declared verdict that bridges is
/// one whose displacement stays in band at every point of both faces,
/// and an undeclared coincidence one whose sum decides Zero there
/// ([`super::carrier_eq::pair_door_reading`]). One door for the
/// verify-at-use site and the operation's glue door.
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
    declared: bool,
    band: Band,
) -> Result<Result<CarrierRelation, CarrierEqError>, PairUnread> {
    Ok(carrier_pair_verdict(a, fa, b, fb, declared, band)?.map(|(rel, _)| rel))
}

/// [`carrier_pair_relation`] plus the AQ6 trilean — the door the
/// CONTACT verification uses, since only a caller that can see the
/// bridged residue can enforce C4's "trusted exactly there" invariant.
/// One traversal, two projections.
///
/// # Errors
///
/// As [`carrier_pair_relation`].
///
/// # Panics
///
/// As [`carrier_pair_relation`].
pub fn carrier_pair_verdict<T: Decide>(
    a: &Body<T>,
    fa: FaceKey,
    b: &Body<T>,
    fb: FaceKey,
    declared: bool,
    band: Band,
) -> Result<Result<(CarrierRelation, crate::contact::ContactVerdict), CarrierEqError>, PairUnread> {
    Ok(carrier_pair_reading(a, fa, b, fb, declared, band)?.map(|(rel, verdict, _)| (rel, verdict)))
}

/// [`carrier_pair_verdict`] with the margin that decided a coincidence
/// ([`super::carrier_eq::CarrierReading`]), which the declaration door
/// records.
///
/// # Errors
///
/// As [`carrier_pair_relation`].
///
/// # Panics
///
/// As [`carrier_pair_relation`].
pub(crate) fn carrier_pair_reading<T: Decide>(
    a: &Body<T>,
    fa: FaceKey,
    b: &Body<T>,
    fb: FaceKey,
    declared: bool,
    band: Band,
) -> Result<Result<super::carrier_eq::CarrierReading, CarrierEqError>, PairUnread> {
    let ca = face_carrier(a, fa).ok_or(PairUnread::OutsideInventory)?;
    let cb = face_carrier(b, fb).ok_or(PairUnread::OutsideInventory)?;
    let extent = pair_extent(a, fa, b, fb, band).map_err(PairUnread::Extent)?;
    let id = PlaneIdentity { declared };
    Ok(super::carrier_eq::pair_door_reading(
        &ca,
        &cb,
        id,
        &extent.consumed(false),
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
        crate::boolean::contact_pair_verdict(
            a,
            fa,
            b,
            fb,
            crate::contact::ContactClass::Rest,
            None,
            band(),
        )
        .map(|_| ())
    }

    /// A torus of `R + r = 2.5 m` against its twin tilted by `θ` about
    /// `y` through the shared centre. At `θ = 0.6·Kε`, in band at one
    /// metre, the tilt moves the tube's core circle by `θ·R = 1.2·Kε`
    /// while no corner of the patch stands past the band: the door
    /// refuses unsettled rather than bridging. At `2·Kε` a corner stands
    /// past it, and the door contradicts.
    #[test]
    fn a_torus_tilt_is_read_at_the_ring() {
        for (k, unsettled) in [(0.6, true), (2.0, false)] {
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
            b.set_face_sense(fb, false).unwrap();
            let read = door(&a, fa, &b, fb);
            if unsettled {
                assert!(
                    matches!(read, Err(ContactRefusal::Escalated { .. })),
                    "{k}·Kε: the swing at the ring is unsettled: {read:?}"
                );
            } else {
                assert!(
                    matches!(read, Err(ContactRefusal::Contradicted { .. })),
                    "{k}·Kε: a corner past the band contradicts: {read:?}"
                );
            }
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
            "the tilt at the far rim contradicts the declaration: {read:?}"
        );
    }

    /// **The offset and the tilt add.** A 1 m band of cylinder wall at
    /// `9 ≤ v ≤ 10` against its twin tilted by `0.105·K·ε` about `y`
    /// through the axis' foot at the origin: the axis offset at the
    /// patch's middle reads `≈ 1.0·Kε` (just in band), the tilt over the
    /// patch's extent another fraction of it, each in band on its own,
    /// while the rim stands `1.05·Kε` off. Read datum by datum it
    /// bridged; read as one displacement it does not.
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
            matches!(read, Err(ContactRefusal::Contradicted { .. })),
            "the rim stands past the band, so the declaration does not bridge: {read:?}"
        );
    }
}
