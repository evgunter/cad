//! The one dispatch site over the kernel's op doors, and what comes
//! back out of it.

use core::fmt;

use geom_core::{Bounds, Decide, Real, Tol};
use sweep::blend::BlendRefusal;
use sweep::blend::naming::BlendNaming;
use topo::Body;

use crate::verb::Verb;

/// **What a verb produced**: the body, and the birth record for what
/// the operation minted.
///
/// The record is NOT restated here — it is the operation's own, moved
/// across by value, so a change to what the blend surgery records
/// reaches this door with no edit. `naming` keeps the op door's
/// permanent `Option`, whose whole point is that an EMPTY record must
/// not be constructible: `None` says "this body has no birth records",
/// which the document layer refuses as a kernel bug, and an empty
/// struct would be refused by nothing.
///
/// The blend doors also hand back their solid, shell and per-role face
/// lists. Those are geometry a caller who wants them reads off the door
/// directly; the lowering reads the body and the record, so those are
/// what a verb result carries.
#[derive(Clone, Debug)]
pub struct VerbOut<T: Real> {
    /// The operation's output body.
    pub body: Body<T>,
    /// Per-entity birth records: what the operation minted and which
    /// source entity each mint was made for.
    pub naming: Option<BlendNaming>,
}

/// **Why a verb refused**, carrying the op door's own typed refusal
/// unaltered.
///
/// Closed with no wildcard arm (D3): the arms grow one per migrated op
/// family, and a consumer that renders refusals is forced to visit the
/// new one. The blend pair shares a single arm because the two doors
/// share one refusal vocabulary and the door itself records which verb
/// refused (`BlendRefusal::verb`).
#[derive(Clone, Debug)]
pub enum VerbError {
    /// A blend door refused.
    Blend(BlendRefusal),
}

impl fmt::Display for VerbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Blend(refusal) => write!(f, "{refusal}"),
        }
    }
}

impl core::error::Error for VerbError {}

impl<T: Decide + Bounds + geom_brep::PcurveFittedLane> Verb<T> {
    /// **Run this verb against its operand.**
    ///
    /// One body in for both blend verbs (the arity the declaration
    /// states), borrowed rather than owned. Every check, every refusal
    /// and every minted entity is the op door's — this dispatches and
    /// re-wraps, and adds no decision of its own.
    ///
    /// # Errors
    ///
    /// [`VerbError::Blend`] carrying the door's [`BlendRefusal`]
    /// verbatim; the door's own docs
    /// (`sweep::blend::build::fillet_edges`,
    /// `sweep::blend::build::chamfer_edges`) enumerate the cases.
    pub fn run(&self, operand: &Body<T>, tol: Tol) -> Result<VerbOut<T>, VerbError> {
        let blended = match self {
            Self::Fillet { edges, radius } => {
                sweep::blend::build::fillet_edges(operand, edges, *radius, tol)
            }
            Self::Chamfer { edges, distance } => {
                sweep::blend::build::chamfer_edges(operand, edges, *distance, tol)
            }
        }
        .map_err(VerbError::Blend)?;
        Ok(VerbOut {
            body: blended.body,
            naming: blended.naming,
        })
    }
}
