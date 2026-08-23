//! **The void-insertion door** — the one birthplace of cavities
//! (DESIGN.md, M2 structural conventions: "every CAVITY is born
//! through the shared void-insertion door"). A cavity's boundary is a
//! disconnected interior shell, and its bookkeeping — orientation,
//! census participation, containment evidence — has exactly one home:
//! this door. Three producers are ratified to call it: boolean
//! subtraction's containment fallback (A∖B with B strictly inside A),
//! the full revolve of a holed profile (`revolve(outer) −
//! revolve(hole-as-outer)`, executed as the degenerate no-crossing
//! arm), and `shell`'s sealed hollow (`docs/OFFSET-DESIGN.md` O4).
//!
//! # Contract
//!
//! [`insert_void`] takes a valid destination solid, a **positively
//! oriented** single-solid cavity body whose shells are certified
//! strictly inside the destination's material, and the certification
//! itself as [`VoidEvidence`]. It reverses the cavity body (outward
//! normals flip inward — [`crate::Body::revert`]) and transplants its
//! shells into the destination solid as interior shells (the
//! [`super::combine`] graft: fresh keys, provenance verbatim,
//! descriptions re-certified against the transplanted surfaces).
//!
//! **The door never derives containment itself.** Callers supply the
//! evidence, one certificate per cavity shell; a shell with no
//! certificate, or a certificate that is not a strict-inside claim,
//! refuses typed before any mutation. This is deliberate: the door's
//! two no-crossing producers each hold a certification the door could
//! not reconstruct — the boolean fallback holds `point_in_solid`
//! verdicts under its boundary-disjointness certificate, and the
//! revolve holds the profile's own validated 2-D margins, carried to
//! 3-D verbatim by revolution about the shared axis. Deriving
//! containment here (e.g. by extent boxes) would re-import the
//! box-coarseness that refuses non-convex containers (#750).
//!
//! # What the door does NOT run
//!
//! No SSI, no reduction sweep, no crossing census, no classification
//! walk, no containment probe: the caller's evidence says the two
//! boundaries share no point, so there is nothing for the crossing
//! pipeline to do. The door is a structural insertion — evidence
//! check, revert, graft — and every predicate-funnel decision it can
//! cause comes from the graft's description re-certification, none of
//! it `bool_`-named (the crossing pipeline's prefix; the degenerate-
//! arm suites pin that absence).
//!
//! # Validity
//!
//! The door neither validates its inputs nor gates its result —
//! callers own both, per their own postures (the boolean gates the
//! finished result body; the revolve asserts its tiers in debug and
//! re-validates at rest). A structurally corrupt input surfaces as
//! [`VoidInsertError::Corrupt`] from the graft's own walks, never as
//! a panic.

use geom_core::{Decide, Sign, Tol};

use super::BooleanError;
use super::combine::{GraftMap, graft_solid};
use crate::body::Body;
use crate::entity::{EdgeKey, FaceKey, VertexKey};
use crate::entity::{ShellKey, SolidKey};
use crate::geometry::SurfaceKey;
use crate::revert::RevertError;

/// One cavity shell's strict-containment certificate, supplied by the
/// caller (module docs: the door never derives containment).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VoidContainment {
    /// The verdict of a 3-D material probe of a boundary witness of
    /// this shell against the destination solid
    /// ([`super::point_in_solid`]), sound under the caller's
    /// boundary-disjointness certificate (the boolean containment
    /// fallback's evidence, passed verbatim). Strict containment is
    /// exactly [`super::SolidContainment::In`]; any other verdict
    /// refuses.
    Probed(super::SolidContainment),
    /// The decided sign of a lower-dimensional strict-containment
    /// margin that a shared containment-preserving construction
    /// carries to 3-D verbatim: a validated profile's hole loop —
    /// strictly inside its outer loop, with the profile validation's
    /// decided clearance and containment margins — revolved about the
    /// same axis as the outer (the holed full revolve), or an inward
    /// offset's d-vs-reach margin (`shell`, OFFSET-DESIGN O4). Strict
    /// containment is exactly [`Sign::Positive`]; `Zero` (touching)
    /// and `Negative` (escaped) refuse.
    Carried {
        /// The construction's own decided containment sign.
        sign: Sign,
    },
}

impl VoidContainment {
    /// Is this certificate a strict-inside claim?
    fn strict(self) -> bool {
        match self {
            Self::Probed(v) => v == super::SolidContainment::In,
            Self::Carried { sign } => sign == Sign::Positive,
        }
    }
}

/// The caller-supplied containment certification for one
/// [`insert_void`] call: one certificate per cavity shell, keyed by
/// the cavity body's **own** shell keys (pre-insertion; the door
/// reports the transplanted keys in [`VoidInserted`]).
#[derive(Clone, Debug, Default)]
pub struct VoidEvidence {
    /// The per-shell certificates.
    pub shells: Vec<(ShellKey, VoidContainment)>,
}

/// Typed refusal of [`insert_void`] (closed enum, D4 ¶3). The
/// evidence refusals fire before any mutation.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VoidInsertError {
    /// A cavity shell arrived with no containment certificate — the
    /// door never derives containment (module docs), so absence
    /// refuses rather than probes.
    MissingEvidence {
        /// The uncertified cavity shell.
        shell: ShellKey,
    },
    /// A certificate is not a strict-inside claim (a probed verdict
    /// other than `In`, or a carried sign other than `Positive`).
    NotStrictlyContained {
        /// The shell whose certificate failed.
        shell: ShellKey,
    },
    /// The evidence names a shell the cavity body does not hold — a
    /// caller desync, refused rather than ignored.
    ForeignShell {
        /// The unresolvable shell key.
        shell: ShellKey,
    },
    /// The evidence carries two certificates for one shell — a caller
    /// desync, refused rather than resolved by list order (the door
    /// never picks between conflicting claims).
    DuplicateEvidence {
        /// The doubly-certified shell.
        shell: ShellKey,
    },
    /// The cavity body's orientation reversal failed (tier-1-invalid
    /// cavity).
    Revert(RevertError),
    /// The graft found a structurally corrupt body (the graft's own
    /// `JoinDesync` reasons, verbatim).
    Corrupt {
        /// What was wrong.
        what: &'static str,
    },
    /// A transplanted edge description failed re-certification
    /// against the transplanted surfaces.
    Recertify(geom_brep::CertifyError),
}

impl core::fmt::Display for VoidInsertError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::MissingEvidence { shell } => write!(
                f,
                "void insertion: cavity shell {shell:?} carries no containment \
                 certificate — the door never derives containment; the caller \
                 must certify every cavity shell strictly inside the target"
            ),
            Self::NotStrictlyContained { shell } => write!(
                f,
                "void insertion: cavity shell {shell:?}'s certificate is not a \
                 strict-inside claim — a cavity boundary must be strictly \
                 contained in the target's material"
            ),
            Self::ForeignShell { shell } => write!(
                f,
                "void insertion: evidence names shell {shell:?}, which the \
                 cavity body does not hold (caller desync)"
            ),
            Self::DuplicateEvidence { shell } => write!(
                f,
                "void insertion: evidence certifies shell {shell:?} twice — the door \
                 never resolves conflicting certificates by list order (caller desync)"
            ),
            Self::Revert(e) => write!(f, "void insertion: cavity revert failed: {e:?}"),
            Self::Corrupt { what } => write!(f, "void insertion: {what}"),
            Self::Recertify(e) => {
                write!(f, "void insertion: graft re-certification refused: {e}")
            }
        }
    }
}

/// The key bridge of one [`insert_void`] call: cavity-body keys →
/// destination-body keys (the graft's map, exposed read-only — the
/// ONLY bridge between the two key spaces; consumers read it as data,
/// never key equality across bodies).
#[derive(Debug)]
pub struct VoidInserted {
    pub(crate) graft: GraftMap,
}

impl VoidInserted {
    /// The destination key of a cavity-body vertex.
    pub fn vertex(&self, v: VertexKey) -> Option<VertexKey> {
        self.graft.vertices.get(v).copied()
    }

    /// The destination key of a cavity-body edge.
    pub fn edge(&self, e: EdgeKey) -> Option<EdgeKey> {
        self.graft.edges.get(e).copied()
    }

    /// The destination key of a cavity-body face.
    pub fn face(&self, f: FaceKey) -> Option<FaceKey> {
        self.graft.faces.get(f).copied()
    }

    /// The destination key of a cavity-body surface.
    pub fn surface(&self, s: SurfaceKey) -> Option<SurfaceKey> {
        self.graft.surfaces.get(s).copied()
    }

    /// The destination key of a cavity-body shell — the interior
    /// (cavity) shell the door inserted for it.
    pub fn shell(&self, s: ShellKey) -> Option<ShellKey> {
        self.graft.shells.get(s).copied()
    }
}

/// Inserts a certified-strictly-contained cavity into `dst_solid` of
/// `dst` (module docs: the contract, the evidence discipline, and
/// what the door does not run).
///
/// `cavity` is a **positively oriented** single-solid closed body —
/// the material that is being removed, exactly as a subtraction's B
/// operand — consumed by value; the door reverses it and transplants
/// its shells. `evidence` must certify every shell of `cavity`
/// strictly inside `dst_solid`'s material.
///
/// # Errors
///
/// [`VoidInsertError`] — evidence refusals before any mutation;
/// revert/graft refusals verbatim.
pub fn insert_void<T: Decide>(
    dst: &mut Body<T>,
    dst_solid: SolidKey,
    cavity: Body<T>,
    evidence: &VoidEvidence,
    tol: Tol,
) -> Result<VoidInserted, VoidInsertError> {
    // ---- Evidence check (pure reads, first — no mutation happens
    // unless every cavity shell is certified strictly inside). ----
    for (i, &(shell, _)) in evidence.shells.iter().enumerate() {
        if cavity.get_shell(shell).is_none() {
            return Err(VoidInsertError::ForeignShell { shell });
        }
        if evidence.shells[..i].iter().any(|(s, _)| *s == shell) {
            return Err(VoidInsertError::DuplicateEvidence { shell });
        }
    }
    for (shell, _) in cavity.shells() {
        let cert = evidence
            .shells
            .iter()
            .find(|(s, _)| *s == shell)
            .map(|(_, c)| *c)
            .ok_or(VoidInsertError::MissingEvidence { shell })?;
        if !cert.strict() {
            return Err(VoidInsertError::NotStrictlyContained { shell });
        }
    }

    // ---- The insertion itself: revert + graft (bit-for-bit the
    // boolean containment fallback's cavity step, factored here). The
    // declared arena delta is the whole cavity transplanted — every
    // entity re-created under fresh keys, its shells landing under
    // `dst_solid` (no new solid).
    #[cfg(debug_assertions)]
    let (before, transplant) = {
        let c = cavity.arena_counts();
        // Casts are lossless in every reachable regime: an arena
        // length that overflows isize is unrepresentable long before.
        #[allow(clippy::cast_possible_wrap)]
        let transplant = crate::euler::ArenaDelta {
            solids: 0,
            shells: c.shells as isize,
            faces: c.faces as isize,
            loops: c.loops as isize,
            half_edges: c.half_edges as isize,
            edges: c.edges as isize,
            vertices: c.vertices as isize,
        };
        (dst.arena_counts(), transplant)
    };
    let reversed = cavity.revert().map_err(VoidInsertError::Revert)?;
    let graft = graft_solid(dst, dst_solid, &reversed, tol).map_err(|e| match e {
        BooleanError::JoinDesync { what } => VoidInsertError::Corrupt { what },
        BooleanError::GraftRecertify(c) => VoidInsertError::Recertify(c),
        // The graft's error surface is exactly the two arms above;
        // anything else arriving here is a kernel bug, surfaced typed
        // rather than panicked (D9: never a panic on an error path).
        _ => VoidInsertError::Corrupt {
            what: "graft refused outside its own error surface (kernel bug)",
        },
    })?;
    #[cfg(debug_assertions)]
    dst.assert_euler_postcondition(before, transplant, "insert_void");
    Ok(VoidInserted { graft })
}
