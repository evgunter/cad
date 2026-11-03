//! **The every-op census harness**: an operand and a partner run through
//! ∪, ∩ and ∖ in both operand orders, each outcome held to what a census
//! holds it to — a typed refusal that is never a kernel invariant, or a
//! body measured against its volume oracle — and the ops that build held
//! to one shared `V(O ∩ B)` by inclusion–exclusion. The censuses over
//! spline operands (`spline_spiric_operands`,
//! `spline_subtract_intersect_census`) share it, so the measure, its band
//! and the inclusion–exclusion check are one reading.

use geom_core::Tol;
use topo::{AtRestBody, BooleanError, BooleanResult, Operand};

/// The resolution (mean boundary displacement, m) every body here is
/// measured at: the width at which a reading's bracket lies within
/// [`band`] of the volumes the censuses hold, at every ε row. It is no
/// finer than the coarsest row's ε, 1e-6.
pub const RESOLUTION: f64 = 1e-6;

/// How far a volume `v` may read from its oracle beyond the brackets.
pub fn band(v: f64) -> f64 {
    1e-6 * v.abs().max(1.0)
}

/// `body`'s volume at [`RESOLUTION`] and its bracket's half-width, which
/// is asserted to lie within [`band`].
///
/// # Panics
///
/// Where the measure refuses, or its bracket is wider than its band.
pub fn measure(what: &str, body: &AtRestBody<f64>) -> (f64, f64) {
    let m = topo::test_support::volume_at(what, body, Tol::witness(), RESOLUTION);
    assert!(
        m.volume_pad <= band(m.volume),
        "{what}: V = {} ± {:e}, a bracket wider than its band {:e}",
        m.volume,
        m.volume_pad,
        band(m.volume)
    );
    (m.volume, m.volume_pad)
}

/// The refusals that name a broken kernel rather than an input or a
/// frontier.
pub fn kernel_invariant(e: &BooleanError) -> bool {
    matches!(
        e,
        BooleanError::ClassificationInvariant { .. } | BooleanError::JoinDesync { .. }
    ) || matches!(e, BooleanError::Join(j) if matches!(j.error(), topo::SplitJoinError::SectionInvariant { .. }))
}

/// Which region of the two operands an op keeps.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Keep {
    /// `O ∪ B`.
    Union,
    /// `O ∩ B`.
    Meet,
    /// `O ∖ B`.
    OLessB,
    /// `B ∖ O`.
    BLessO,
}

impl Keep {
    /// The op's slot in a `[∪, ∩, O ∖ B, B ∖ O]` closed-form row.
    pub fn slot(self) -> usize {
        match self {
            Keep::Union => 0,
            Keep::Meet => 1,
            Keep::OLessB => 2,
            Keep::BLessO => 3,
        }
    }

    /// Whether a point in `O` (`in_o`) and in `B` (`in_b`) is kept.
    pub fn holds(self, in_o: bool, in_b: bool) -> bool {
        match self {
            Keep::Union => in_o || in_b,
            Keep::Meet => in_o && in_b,
            Keep::OLessB => in_o && !in_b,
            Keep::BLessO => in_b && !in_o,
        }
    }

    /// `V(O ∩ B)` from the op's volume `got`, given `V(O)` and `V(B)`.
    pub fn meet(self, vo: f64, vb: f64, got: f64) -> f64 {
        match self {
            Keep::Union => vo + vb - got,
            Keep::Meet => got,
            Keep::OLessB => vo - got,
            Keep::BLessO => vb - got,
        }
    }
}

/// One op of a case: its name, its two operands in order, what it keeps,
/// and its outcome.
pub struct Run<'a> {
    /// `"O∪B"`, `"B∖O"`, …
    pub op: &'static str,
    /// The operands, in the order the op took them.
    pub pair: [&'a AtRestBody<f64>; 2],
    /// The region it keeps.
    pub keep: Keep,
    /// What it returned.
    pub got: Result<BooleanResult<f64>, BooleanError>,
}

/// The six ops of `o` and `b`: ∪, ∩ and ∖, each in both operand orders.
pub fn six_runs<'a>(o: &'a AtRestBody<f64>, b: &'a AtRestBody<f64>) -> [Run<'a>; 6] {
    let t = Tol::witness();
    let run = |op, pair: [&'a AtRestBody<f64>; 2], keep, got| Run {
        op,
        pair,
        keep,
        got,
    };
    [
        run("O∪B", [o, b], Keep::Union, topo::union(o, b, t)),
        run("B∪O", [b, o], Keep::Union, topo::union(b, o, t)),
        run("O∩B", [o, b], Keep::Meet, topo::intersect(o, b, t)),
        run("B∩O", [b, o], Keep::Meet, topo::intersect(b, o, t)),
        run("O∖B", [o, b], Keep::OLessB, topo::subtract(o, b, t)),
        run("B∖O", [b, o], Keep::BLessO, topo::subtract(b, o, t)),
    ]
}

/// A built result's volume and bracket: `0` for an empty result.
pub fn result_volume(what: &str, r: &BooleanResult<f64>) -> (f64, f64) {
    match r {
        BooleanResult::Empty => (0.0, 0.0),
        BooleanResult::Body(b) => measure(what, &b.body),
    }
}

/// **Inclusion–exclusion over the ops that built**: each `(op, V(O ∩ B),
/// pad)` it implies lies in `[0, min(V(O), V(B))]` and agrees with the
/// others within their brackets and `v_pad`, the bracket `V(O)` carries
/// when it is a measure rather than a closed form.
///
/// # Panics
///
/// Where one lies outside the range or two disagree.
pub fn assert_one_meet(name: &str, implied: &[(&str, f64, f64)], vo: f64, v_pad: f64, vb: f64) {
    let Some(&(_, first, first_pad)) = implied.first() else {
        return;
    };
    let scale = vo.max(vb);
    let slack = 1e-9 * scale + first_pad + v_pad;
    assert!(
        (-slack..=vo.min(vb) + slack).contains(&first),
        "{name}: V(O ∩ B) = {first} ± {:e} lies outside [0, min(V(O), V(B))]",
        first_pad + v_pad
    );
    for &(op, meet, pad) in implied {
        assert!(
            (meet - first).abs() <= 1e-9 * scale + pad + first_pad,
            "{name}: {op} implies V(O ∩ B) = {meet} ± {pad:e}, the others {first} ± {first_pad:e}"
        );
    }
}

/// Where a refusal names a face, that it is a face of the operand its
/// site says owns it — resolving there and not in the other — of the
/// kind it states; and where it names an edge, a spiric or spline edge
/// of its operand.
pub fn names_its_own_face(e: &BooleanError, [a, b]: [&AtRestBody<f64>; 2]) -> Result<(), String> {
    let body = |operand: Operand| match operand {
        Operand::A => a,
        Operand::B => b,
    };
    let owned = |operand: Operand, face| {
        body(operand).get_face(face).is_some() && body(operand.other()).get_face(face).is_none()
    };
    let kind_of = |operand: Operand, face| {
        body(operand)
            .get_face(face)
            .and_then(|f| body(operand).get_surface(f.surface))
            .map(geom::Surface::kind)
    };
    let check = |ok: bool, why: String| ok.then_some(()).ok_or(why);
    match *e {
        BooleanError::CurvedBooleanUnsupported {
            operand,
            face,
            kind,
        } => check(
            owned(operand, face) && kind_of(operand, face) == Some(kind),
            format!("{face:?} is no {kind:?} face of {operand:?} alone"),
        ),
        BooleanError::CurvedPairUnsupported {
            operand,
            face,
            kind,
            other_face,
            other_kind,
            ..
        } => check(
            owned(operand, face)
                && kind_of(operand, face) == Some(kind)
                && owned(operand.other(), other_face)
                && kind_of(operand.other(), other_face) == Some(other_kind),
            format!(
                "{face:?}/{other_face:?} are not {kind:?}/{other_kind:?} faces of {operand:?} and its partner"
            ),
        ),
        BooleanError::EdgeCarrierUnsupported {
            operand,
            edge,
            face,
            face_of,
            ..
        } => check(
            matches!(
                topo::query::edge_carrier_kind(body(operand), edge),
                Some(topo::query::CurveKind::Nurbs | topo::query::CurveKind::Spiric)
            ) && owned(face_of, face),
            format!("{edge:?} of {operand:?} against {face:?} of {face_of:?}"),
        ),
        BooleanError::SplineCrossingUnsettled {
            operand,
            edge,
            face,
            face_of,
            ..
        } => check(
            topo::query::edge_carrier_kind(body(operand), edge)
                == Some(topo::query::CurveKind::Nurbs)
                && owned(face_of, face),
            format!("{edge:?} of {operand:?} against {face:?} of {face_of:?}"),
        ),
        _ => Ok(()),
    }
}
