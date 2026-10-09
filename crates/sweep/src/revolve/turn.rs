//! A one-segment loop's revolve: D1's full turn swept whole, far end
//! first, as one torus wall whose strut — the latitude circle through
//! the loop's one vertex — is its wrap edge in `v` (D1: a torus
//! parallel wraps `v`). Both revolve cases build it here, through the
//! shared [`crate::swept::build_full_turn`] extrude also sweeps a full
//! turn with; the full revolve then closes the wall on itself in `u`
//! as well (`full::build_turn_lamina`).

use geom::Curve3;
use geom_brep::{EdgeCurveSpec, EdgeDescriptionSpec, MappedCurve};
use geom_core::{Affine3, Decide, Point3, Tol, Vec3};
use topo::{Body, FaceSurface, LoopKey};

use super::axis::{AxisFrame, LoopClasses, WallClass};
use super::partial::LoopSwept;
use super::surfaces::wall_surface;
use super::{RevolveError, SweptSeg};
use crate::swept::{self, FullTurn, face_surface_key, placed_segment_spec};

/// Where a one-segment loop's turn runs: its vertex at the start and at
/// the end of the rotation, and the end's placement.
pub(super) struct TurnEnds<T: geom_core::Real> {
    /// The vertex on the sketch placement (the near end).
    pub(super) near: Point3<T>,
    /// The vertex rotated through `theta` (the far end).
    pub(super) far: Point3<T>,
    /// The sketch placement rotated through `theta`.
    pub(super) place_far: Affine3<T>,
    /// Its sketch normal.
    pub(super) n_far: Vec3<T>,
}

/// Sweeps the one-segment loop `seg` (loop `loop_index`, whose classes
/// are `cls`) through `theta` from `r#loop`'s lone vertex, which sits
/// at `ends.far`: the far rim, the strut back to `ends.near`, and the
/// near rim, whose new face takes `near_cap`. The strut is described as
/// the wall's wrap edge. `axis_c` is the θ-signed carrier axis of a
/// strut that runs near to far; this one runs back, about `−axis_c`.
#[allow(clippy::too_many_arguments)] // the sweep's fixed context, as `sweep_loop`'s
pub(super) fn sweep_turn<T: Decide + topo::AtRestPolicy>(
    body: &mut Body<T>,
    frame: &AxisFrame<T>,
    cls: &LoopClasses<T>,
    seg: &SweptSeg<T>,
    r#loop: LoopKey,
    ends: &TurnEnds<T>,
    theta: T,
    axis_c: Vec3<T>,
    near_cap: FaceSurface<T>,
    tol: Tol,
) -> Result<(FullTurn, LoopSwept), RevolveError> {
    // A full turn's one segment is a circle clear of the axis: the axis
    // classes refuse every other full turn before a build starts
    // (`revolve`), so its wall is a torus and its vertex is off-axis.
    let WallClass::Wall { kind, sense } = cls.walls[0] else {
        unreachable!("a one-segment loop clear of the axis sweeps a wall")
    };
    let wall = FaceSurface::New {
        surface: wall_surface(&kind, seg, frame),
        sense,
    };
    let center = frame.foot3(seg.a);
    let radius = cls.verts[0].r;
    let from_far = ends.far - center;
    swept::register_rim_identity(from_far, radius, tol);
    let strut = EdgeCurveSpec {
        description: EdgeDescriptionSpec::Scaffold(MappedCurve::RevolvedPoint {
            point: seg.a,
            place: ends.place_far,
            axis_origin: frame.o3,
            axis_dir: frame.a3,
            angle: T::zero() - theta,
            range: geom_brep::SweepRange::whole(),
        }),
        carrier: Curve3::Circle {
            center,
            axis: Vec3::zero() - axis_c,
            radius,
            u_ref: from_far.normalize(),
        },
        param_start: T::zero(),
        param_end: theta.abs(),
    };
    let turn = swept::build_full_turn(
        body,
        r#loop,
        ends.near,
        placed_segment_spec(seg, ends.place_far, ends.n_far, ends.far, ends.far, tol),
        wall,
        strut,
        placed_segment_spec(seg, frame.place, frame.n3, ends.near, ends.near, tol),
        near_cap,
        tol,
    )?;
    let wall_key = face_surface_key(body, turn.wall);
    swept::describe_wrap_edge(body, turn.strut, wall_key, tol)?;
    let swept = LoopSwept {
        faces: vec![Some(turn.wall)],
        rims: vec![Some(turn.strut)],
        tops: vec![Some(turn.far)],
    };
    Ok((turn, swept))
}
