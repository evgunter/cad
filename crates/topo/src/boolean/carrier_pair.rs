//! **The carrier-pair doors**: one cross-body face pair's carrier
//! verdict, on every kind the `Rest` table names ([`carrier_pair_relation`],
//! [`carrier_pair_decided`]), its planar restriction
//! ([`flush_pair_relation`]), the oriented descriptions they compare
//! ([`face_carrier`]) and the extent a verdict is consumed over
//! ([`pair_extent`]).

use geom_brep::{ExtentBall, PairSide};
use geom_core::{Band, Decide, Point3};

use super::carrier_eq::{
    CarrierDesc, CarrierEqError, CarrierRelation, CarrierVerdict, PairDoorReading,
};
use super::plane_eq::{PlaneEqError, PlaneRelation};
use crate::body::Body;
use crate::entity::{EntityId, FaceKey};
use crate::face_normal::{face_plane, plane_outward_normal};
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
    Extent(PairSide),
}

/// **A face's consumed extent**: a ball enclosing every point of it,
/// the region over which a verdict about its carrier is consumed
/// ([`pair_extent`]). The carrier's whole cover's ball where the carrier
/// is bounded ([`geom::Surface::cover`]: a sphere's ball, a torus's tube,
/// a spline's net, whatever the trim); otherwise the ball around the
/// face's certified box, from the kernel's one kind→box rule
/// (`census::face_reach`). `None` where `face`, the caller's key, does
/// not resolve, where that box has no claim to make, or where the ball
/// does not read. The face's surface is a link, and its miss panics (on
/// an at-rest operand by tier 1, mid-operation by
/// [`crate::live::OPERATORS_KEEP_LINKS`]).
fn face_ball<T: Decide>(body: &Body<T>, face: FaceKey, band: Band) -> Option<ExtentBall<T>> {
    let f = body.face(face)?;
    let ball = match f.surface().cover() {
        Ok(cover) => ExtentBall::of_cover(&cover),
        Err(_) => {
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
    let f = body.face(face)?;
    Some(
        f.boundary()
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
) -> Result<PairExtent<T>, PairSide> {
    let ball_a = face_ball(a, fa, band).ok_or(PairSide::First)?;
    let ball_b = face_ball(b, fb, band).ok_or(PairSide::Second)?;
    // Two readable balls enclose readably short of overflow, which the
    // larger reach is what tips; the second face takes the blame
    // rather than neither.
    let reach = ExtentBall::enclosing(&[ball_a, ball_b])
        .and_then(ExtentBall::readable)
        .ok_or(PairSide::Second)?;
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
    let f = body.face(face)?;
    // `sense` is the material-side bit: true means the face's outward
    // normal IS the chart normal, which for a sphere/cylinder chart
    // points away from the centre/axis. Read as a BIT, never as a
    // comparison on `T` — the scalar backends order intervals, not
    // signs (S10's exact-bit discipline).
    let outward = f.sense();
    match f.surface() {
        geom::Surface::Plane { origin, normal, .. } => Some(CarrierDesc::Plane {
            origin: *origin,
            normal: plane_outward_normal(f.record(), *normal).vec(),
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
        geom::Surface::Spindle { .. } | geom::Surface::Nurbs(_) | geom::Surface::Approx(_) => None,
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
    Ok(carrier_pair_decided(a, fa, b, fb, band)?.map(CarrierVerdict::relation))
}

/// [`carrier_pair_relation`] with the reading that decided it
/// ([`CarrierVerdict`]): a coincidence's margin is the one the glue door
/// records, and the reading the coincidence door runs on two cells'
/// carriers in a replay at `Sym`, where the margin says whether the
/// symbolic tier proved every Zero the verdict stands on
/// ([`geom_core::MarginDiag::proven`]).
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
) -> Result<Result<CarrierVerdict, CarrierEqError>, PairUnread> {
    let carriers = carriers_of(a, fa, b, fb)?;
    let extent = pair_extent(a, fa, b, fb, band).map_err(PairUnread::Extent)?;
    Ok(decided_over((a, fa), (b, fb), carriers, &extent, band))
}

/// **The glue's reading of a one-carrier pair**: [`carrier_pair_decided`]
/// over the faces the verdict moves as well as the two it decides
/// ([`glue_extent`]), so it is read once over everything it acts on
/// (`[coincidence.door]`). `moved` is the moving side and its faces
/// ([`super::glue::GlueCensus`]).
///
/// # Errors
///
/// As [`carrier_pair_decided`]; a moved face whose extent does not
/// read is its side's.
///
/// # Panics
///
/// As [`carrier_pair_decided`].
pub(super) fn carrier_pair_decided_over<T: Decide>(
    a: &Body<T>,
    fa: FaceKey,
    b: &Body<T>,
    fb: FaceKey,
    moved: (super::Operand, &std::collections::BTreeSet<FaceKey>),
    band: Band,
) -> Result<Result<CarrierVerdict, CarrierEqError>, PairUnread> {
    let carriers = carriers_of(a, fa, b, fb)?;
    let extent = glue_extent(a, fa, b, fb, moved, band).map_err(PairUnread::Extent)?;
    Ok(decided_over((a, fa), (b, fb), carriers, &extent, band))
}

/// **The extent a glue verdict is read over**: one ball enclosing the
/// two decided faces and every face the verdict moves ([`face_ball`]).
///
/// # Errors
///
/// The side whose face's extent cannot be read.
pub(super) fn glue_extent<T: Decide>(
    a: &Body<T>,
    fa: FaceKey,
    b: &Body<T>,
    fb: FaceKey,
    (side, moved): (super::Operand, &std::collections::BTreeSet<FaceKey>),
    band: Band,
) -> Result<PairExtent<T>, PairSide> {
    let PairExtent { reach } = pair_extent(a, fa, b, fb, band)?;
    let (body, blame) = match side {
        super::Operand::A => (a, PairSide::First),
        super::Operand::B => (b, PairSide::Second),
    };
    let mut parts = vec![reach];
    for &f in moved {
        parts.push(face_ball(body, f, band).ok_or(blame)?);
    }
    let reach = ExtentBall::enclosing(&parts)
        .and_then(ExtentBall::readable)
        .ok_or(blame)?;
    Ok(PairExtent { reach })
}

/// The two faces' carrier descriptions ([`face_carrier`]), read before
/// their extent: a kind outside the inventory has no extent to read.
fn carriers_of<T: Decide>(
    a: &Body<T>,
    fa: FaceKey,
    b: &Body<T>,
    fb: FaceKey,
) -> Result<(CarrierDesc<T>, CarrierDesc<T>), PairUnread> {
    Ok((
        face_carrier(a, fa).ok_or(PairUnread::OutsideInventory)?,
        face_carrier(b, fb).ok_or(PairUnread::OutsideInventory)?,
    ))
}

/// [`carrier_pair_decided`]'s reading of `(ca, cb)` over `extent`.
fn decided_over<T: Decide>(
    (a, fa): (&Body<T>, FaceKey),
    (b, fb): (&Body<T>, FaceKey),
    (ca, cb): (CarrierDesc<T>, CarrierDesc<T>),
    extent: &PairExtent<T>,
    band: Band,
) -> Result<CarrierVerdict, CarrierEqError> {
    match super::carrier_eq::pair_door_reading(&ca, &cb, &extent.consumed(), band) {
        Ok(PairDoorReading::RingApart {
            relation, upper, ..
        }) => torus_pair_witnessed((a, fa, &ca), (b, fb, &cb), (relation, upper), band),
        Ok(PairDoorReading::Verdict(verdict)) => Ok(verdict),
        Err(refusal) => Err(refusal),
    }
}

/// The row a torus face's witness of two carriers is decided on
/// ([`torus_pair_witnessed`]).
const TORUS_WITNESS: &str = "carrier_torus_witness";

/// The row a torus face's witness decided `Zero` straddles the band on,
/// beside the ring's bound decided past it ([`torus_pair_witnessed`]).
const TORUS_WITNESS_STRADDLE: &str = "carrier_torus_witness_straddle";

/// **Two torus faces the ring reads apart are apart over their extent
/// only where a point of one stands past the band from the other's
/// carrier.** The torus coincidence reads the whole ring
/// ([`geom_brep::tori_coincide`]): a datum past the band moves the ring
/// by that much somewhere, which the faces need not hold, and where
/// they do not the two may stand in band of each other over every point
/// they span. Each face's boundary points ([`face_witnesses`]) lie on
/// it, and a point's distance to a torus is exact in closed form, so
/// the farthest of them bounds from below how far the faces themselves
/// stand apart, which the ring's data, read over the whole carriers, do
/// not: past the band it witnesses two carriers here. The ring's `upper`
/// bounds that distance from above at every point of both tori, and is
/// decided past the band wherever the ring reads apart; so a witness
/// decided `Zero` straddles the band with it, and one in band is
/// undecided on its own reading. `relation` is the orientation the two
/// faces' senses make.
///
/// # Panics
///
/// Where `ca` or `cb` is not a torus, or a face's boundary does not
/// read: the door reads both faces' carriers first.
fn torus_pair_witnessed<T: Decide>(
    (a, fa, ca): (&Body<T>, FaceKey, &CarrierDesc<T>),
    (b, fb, cb): (&Body<T>, FaceKey, &CarrierDesc<T>),
    (relation, upper): (CarrierRelation, geom_core::Decided),
    band: Band,
) -> Result<CarrierVerdict, CarrierEqError> {
    use super::carrier_eq::CoincidenceMeasure;
    use geom_core::{Margin, Sign};
    let on = |body: &Body<T>, f: FaceKey| {
        face_witnesses(body, f)
            .unwrap_or_else(|| unreachable!("{f:?}'s carrier read, so its boundary does"))
    };
    let distances = on(a, fa)
        .into_iter()
        .map(|p| torus_distance(cb, p))
        .chain(on(b, fb).into_iter().map(|p| torus_distance(ca, p)));
    let farthest = distances.fold(T::zero(), |m, d| m.max(d));
    let coincidence = match CoincidenceMeasure::decide(TORUS_WITNESS, Margin::of(farthest), band) {
        Ok(witness) if witness.sign == Sign::Positive => return Ok(CarrierVerdict::Apart(witness)),
        Ok(witness) => CoincidenceMeasure::not_zero(geom_core::k_stats::straddle(
            TORUS_WITNESS_STRADDLE,
            witness,
            upper,
            band,
        )),
        Err(coincidence) => coincidence,
    };
    Err(CarrierEqError::Undecided {
        coincidence,
        relation,
    })
}

/// The distance from `p` to the torus `c`: from its core circle, less
/// the minor radius.
///
/// # Panics
///
/// Where `c` is not a torus.
fn torus_distance<T: Decide>(c: &CarrierDesc<T>, p: Point3<T>) -> T {
    let CarrierDesc::Torus {
        center,
        axis,
        major_radius,
        minor_radius,
        ..
    } = *c
    else {
        unreachable!("a torus distance reads a torus")
    };
    let (z, rho) = geom::Line {
        origin: center,
        dir: axis,
    }
    .height_and_distance(p);
    (((rho - major_radius).powi(2) + z.powi(2)).sqrt() - minor_radius).abs()
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
    use crate::boxes::tests::torus_wall;
    use crate::contact::ContactRefusal;
    use crate::test_support::{CylFrame, cyl_wall_sheet};
    use geom_core::{Point3, Tol, Vec3};

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    fn door(
        a: &Body<f64>,
        fa: FaceKey,
        b: &Body<f64>,
        fb: FaceKey,
    ) -> Result<crate::contact::RestVerdict, ContactRefusal> {
        crate::boolean::rest_pair_verdict(a, fa, b, fb, band())
    }

    /// Two carriers, on the datum decided apart.
    fn apart(read: &Result<crate::contact::RestVerdict, ContactRefusal>) -> bool {
        matches!(
            read,
            Ok(crate::contact::RestVerdict::Ruled(
                crate::contact::RestRuling {
                    row: crate::contact::RestRow::TwoCarriers,
                    by: Some(_),
                }
            ))
        )
    }

    /// A torus of `R + r = 2.5 m` against its twin tilted by `θ` about
    /// `y` through the shared centre. At `θ = 0.6·Kε`, in band at one
    /// metre, the tilt moves the tube's core circle by `θ·R = 1.2·Kε`,
    /// past the band, but at the ring's far side, off the patch, and no
    /// corner of the patch stands past the band from the twin: the door
    /// escalates. At `2·Kε` a corner stands past it, and the door reads
    /// two carriers and contradicts.
    #[test]
    fn a_torus_tilt_is_read_at_the_patch() {
        for (k, witnessed) in [(0.6, false), (2.0, true)] {
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
            if witnessed {
                assert!(
                    apart(&read),
                    "{k}·Kε: a corner past the band reads the carriers apart: {read:?}"
                );
            } else {
                assert!(
                    matches!(read, Err(ContactRefusal::Escalated { .. })),
                    "{k}·Kε: the swing at the ring is no evidence over the patch: {read:?}"
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
            apart(&read),
            "the tilt at the far rim reads the carriers apart: {read:?}"
        );
    }

    /// **A torus pair in band over its patch escalates.** The torus
    /// lever row's `0.6·Kε` tilt: every point of the patch (sampled on a
    /// 41×41 grid of its chart) stands within `Kε` of the tilted twin's
    /// carrier, so the two faces are in band of each other over their
    /// whole extent. The tilt levered at the major radius parts the ring
    /// at its far side, off the patch, so it is no evidence here; only
    /// the displacement's lower bound could part them, and it does not.
    #[test]
    fn a_torus_pair_in_band_over_its_patch_escalates() {
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
            matches!(read, Err(ContactRefusal::Escalated { .. })),
            "a pair in band over its whole extent escalates: worst {worst:e}, {read:?}"
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

    /// The face-pair door's undecided coincidence, and the contact door's
    /// escalation of the same pair, at `band`.
    fn straddled(
        (a, fa): (&Body<f64>, FaceKey),
        (b, fb): (&Body<f64>, FaceKey),
        band: Band,
    ) -> (geom_core::Indeterminate, geom_core::Indeterminate) {
        let at_door = match carrier_pair_decided(a, fa, b, fb, band) {
            Ok(Err(CarrierEqError::Undecided {
                coincidence: super::super::carrier_eq::CoincidenceMeasure::Undecided(diag),
                relation: CarrierRelation::SameOpposite,
            })) => diag,
            other => panic!("the face-pair door: {other:?}"),
        };
        match crate::boolean::rest_pair_verdict(a, fa, b, fb, band) {
            Err(ContactRefusal::Escalated { diag }) => (at_door, diag),
            other => panic!("the contact door: {other:?}"),
        }
    }

    /// **A cylinder pair each of whose data decides `Zero` and whose
    /// reach decides past the band straddles it**, at an escalation factor
    /// of 1.5, which two data just inside the zero band reach (at a factor
    /// of 2 or more their sum is in band): a 1 m band of cylinder wall
    /// against its opposed twin, the axis slid `0.9ε` and the radius
    /// `0.9ε` wider. The axis offset and the radius decide `Zero`; their
    /// sum, the reach, `1.8ε`, decides past `1.5ε`. The face-pair door
    /// and the contact door both read the straddle on
    /// `carrier_cyl_reach_straddle`, the enclosure from the first datum's
    /// `Zero` to the reach. The first datum is the tilt, and the axes
    /// agree exactly, so the low end is 0 and the ending subdivides and,
    /// for this pair, offers no tolerance.
    #[test]
    fn a_cylinder_pair_zero_by_its_data_and_past_the_band_by_its_reach_straddles() {
        let tol = Tol::witness();
        let e = band().zero();
        let pinned = Band::new(e, 1.5 * e).unwrap();
        let (u, v) = ((-0.3, 0.3), (0.0, 1.0));
        let mut a = Body::<f64>::new();
        let fa = cyl_wall_sheet(&mut a, CylFrame::canonical(1.0), u, v, tol);
        let twin = CylFrame {
            origin: Point3::new(0.9 * e, 0.0, 0.0),
            radius: 1.0 + 0.9 * e,
            ..CylFrame::canonical(1.0)
        };
        let mut b = Body::<f64>::new();
        let fb = cyl_wall_sheet(&mut b, twin, u, v, tol);
        let sense = a.get_face(fa).unwrap().sense;
        b.set_face_sense(fb, !sense).unwrap();
        let reach = (1.0 + 0.9 * e - 1.0) + 0.9 * e;
        let (at_door, at_contact) = straddled((&a, fa), (&b, fb), pinned);
        assert_eq!(
            at_door, at_contact,
            "the contact door quotes the face-pair door"
        );
        assert_eq!(
            at_door.predicate,
            Some("carrier_cyl_reach_straddle"),
            "{at_door:?}"
        );
        assert_eq!(
            at_door.margin,
            geom_core::MarginDiag::enclosure(0.0, reach),
            "from the data's Zero to the reach: {at_door:?}"
        );
        let ending = at_door.ending(geom_core::COINCIDENCE_RECOURSE);
        assert!(
            !at_door.offers_tolerance() && ending.contains("subdivide"),
            "a straddle from a tilt of exactly 0 subdivides and offers no tolerance: {ending}"
        );
    }

    /// **A sphere pair each of whose data decides `Zero` and whose reach
    /// decides past the band straddles from its first datum**, at an
    /// escalation factor of 1.5: a zone of the unit sphere against its
    /// opposed twin, the centre slid `0.9ε` and the radius `0.9ε` larger.
    /// The centre and the radius decide `Zero`; the reach, `1.8ε`, decides
    /// past `1.5ε`. Both doors read `carrier_sphere_reach_straddle`, the
    /// enclosure from the centre datum, the identity ladder's first, to
    /// the reach. Its low end is `0.9ε`, so besides subdividing the ending
    /// offers the tolerance below which that datum reads the spheres
    /// apart, wherever a run admits it (not at `ε = 10⁻¹²`).
    #[test]
    fn a_sphere_pair_straddles_from_its_first_datum_and_offers_its_tolerance() {
        use crate::boxes::sphere_rect_rows::sphere_zone_sheet;
        let tol = Tol::witness();
        let e = band().zero();
        let pinned = Band::new(e, 1.5 * e).unwrap();
        let (u, h) = ((0.3, 1.2), (-0.3, 0.3));
        let (a, fa) = sphere_zone_sheet(u, h, tol);
        let (mut b, fb) = sphere_zone_sheet(u, h, tol);
        let key = b.get_face(fb).unwrap().surface;
        b.surfaces[key] = geom::Surface::Sphere {
            center: Point3::new(0.9 * e, 0.0, 0.0),
            radius: 1.0 + 0.9 * e,
            axis: Vec3::unit_z(),
            u_ref: Vec3::unit_x(),
        };
        let sense = a.get_face(fa).unwrap().sense;
        b.set_face_sense(fb, !sense).unwrap();
        let (centre, reach) = (0.9 * e, (1.0 + 0.9 * e - 1.0) + 0.9 * e);
        let (at_door, at_contact) = straddled((&a, fa), (&b, fb), pinned);
        assert_eq!(
            at_door, at_contact,
            "the contact door quotes the face-pair door"
        );
        assert_eq!(
            at_door.predicate,
            Some("carrier_sphere_reach_straddle"),
            "{at_door:?}"
        );
        assert_eq!(
            at_door.margin,
            geom_core::MarginDiag::enclosure(centre, reach),
            "from the centre datum to the reach: {at_door:?}"
        );
        // The offer is the tolerance that reads the centre datum past the
        // band, made only where a run at this K admits it.
        let offered = pinned.offer_deciding(centre).is_some();
        let ending = at_door.ending(geom_core::COINCIDENCE_RECOURSE);
        assert!(
            at_door.offers_tolerance() == offered
                && ending.contains("tighten the tolerance") == offered
                && ending.contains("subdivide"),
            "a straddle from a centre datum of 0.9ε offers its tolerance where a run admits it \
             ({offered}): {ending}"
        );
    }

    /// **Two torus faces the ring reads apart, whose witness decides
    /// `Zero`, straddle the band** at the run's own factor: a patch of a
    /// `R = 2`, `r = 0.5` torus `0.02` wide about the outer equator's
    /// point on `+y`, against its opposed twin tilted by `2·Kε` about `y`.
    /// The tilt levered at the major radius reads the rings apart, and the
    /// ring's bound with it; the tilt moves a corner of the patch by about
    /// `θ·R·|cos u|·|sin v|`, `4·10⁻⁴·Kε`, so the corners' witness decides
    /// `Zero`. Both doors read the straddle on
    /// `carrier_torus_witness_straddle`, the enclosure from the witness to
    /// the ring's bound, whose ending subdivides.
    #[test]
    fn a_torus_pair_the_ring_reads_apart_and_its_witness_zero_straddles() {
        let theta = 2.0 * band().escalate();
        let quarter = core::f64::consts::FRAC_PI_2;
        let (u, v) = ((quarter - 0.01, quarter + 0.01), (-0.01, 0.01));
        let axis = Vec3::new(theta.sin(), 0.0, theta.cos());
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
            axis,
            Vec3::new(theta.cos(), 0.0, -theta.sin()),
            2.0,
            0.5,
            u,
            v,
        );
        let sense = a.get_face(fa).unwrap().sense;
        b.set_face_sense(fb, !sense).unwrap();
        // The twin is the patch turned by θ about y; a point's distance
        // to the untilted torus, exact in closed form, and each corner's
        // to the other carrier: `a`'s turned back by θ, the twin's turned
        // on by it.
        let (sin, cos) = theta.sin_cos();
        let turned = |[x, y, z]: [f64; 3], s: f64| [cos * x + s * z, y, cos * z - s * x];
        let off_ring = |[x, y, z]: [f64; 3]| ((x.hypot(y) - 2.0).hypot(z) - 0.5).abs();
        let mut witness: f64 = 0.0;
        for uu in [u.0, u.1] {
            for vv in [v.0, v.1] {
                let rho = 2.0 + 0.5 * vv.cos();
                let corner = [rho * uu.cos(), rho * uu.sin(), 0.5 * vv.sin()];
                witness = witness
                    .max(off_ring(turned(corner, -sin)))
                    .max(off_ring(turned(corner, sin)));
            }
        }
        assert!(
            witness <= band().zero(),
            "the corners read Zero: {witness:e}"
        );
        // The ring's bound: the tilt alone, its chord `2·sin(θ/2)` levered
        // at the major radius.
        let upper = 2.0 * (2.0 * (theta / 2.0).sin()).max(sin);
        let (at_door, at_contact) = straddled((&a, fa), (&b, fb), band());
        assert_eq!(
            at_door, at_contact,
            "the contact door quotes the face-pair door"
        );
        assert_eq!(
            at_door.predicate,
            Some("carrier_torus_witness_straddle"),
            "{at_door:?}"
        );
        let geom_core::ErrorTextReading::Enclosure { lo, hi } =
            at_door.margin.diagnostic_f64_for_error_text()
        else {
            panic!("a straddle's margin is an enclosure: {at_door:?}");
        };
        assert!(
            (lo - witness).abs() <= 1e-14 && (hi - upper).abs() <= 1e-6 * upper,
            "from the witness {witness:e} to the ring's bound {upper:e}: {at_door:?}"
        );
        let ending = at_door.ending(geom_core::COINCIDENCE_RECOURSE);
        assert!(
            ending.contains("subdivide"),
            "a straddle's ending subdivides: {ending}"
        );
    }
}
