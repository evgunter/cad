//! **The void-insertion door** — the one birthplace of cavities
//! (DESIGN.md, M2 structural conventions: "every CAVITY is born
//! through the shared void-insertion door"). A cavity's boundary is a
//! disconnected interior shell, and its bookkeeping — orientation,
//! census participation, containment evidence — has exactly one home:
//! this door. Three producers are ratified to call it: boolean
//! subtraction's containment fallback (A∖B with B strictly inside A),
//! the full revolve of a holed profile (`revolve(outer) −
//! revolve(hole-as-outer)`, executed as the degenerate no-crossing
//! arm — `tube_along_arc_hollow`'s full period reaches the same path
//! with hand-built concentric circles and its own wall funnel as the
//! evidence), and `shell`'s sealed hollow
//! (`crates/geom-brep/README.md` O4).
//!
//! # Contract
//!
//! [`insert_void`] takes a valid destination solid, a **positively
//! oriented** single-solid cavity body whose shells are certified
//! strictly inside the destination's material, and the certification
//! itself as [`VoidEvidence`]. [`insert_voids`] is the same door for a
//! cavity of N solids, one destination per cavity solid positionally,
//! and `insert_void` is its `N = 1` case: one implementation, two
//! doors, one evidence discipline. It reverses the cavity body (outward
//! normals flip inward — [`crate::Body::revert`]) and transplants its
//! shells into the destination solid as interior shells (the
//! [`super::combine`] graft: fresh keys, provenance forwarded,
//! certificates carried with their surface handles rewritten).
//!
//! **The certificates are carried, not re-derived.** The graft copies
//! every point, surface, carrier and parameter bit for bit, and the
//! reversal carries every certificate verbatim ([`crate::Body::revert`]).
//! A process has one tolerance, so the cavity's certificates were
//! minted at the band a re-certification here would use, over the same
//! bits: a carried certificate is the one a fresh re-certification
//! mints, bit for bit, wherever the cavity's own certificates are the
//! ones its geometry mints (`sweep`'s `revert_plane_charts` pins that on
//! a reverted cavity; a boolean result's seam meridian can carry one
//! that differs in the last bits, 1.48e-16 against a fresh 2.22e-16,
//! `work/cleave/a-boolean-result-carries-a-seam-meridian-certificate-a-fresh-run-does-not-reproduce.md`).
//! The at-rest gate's check 2 re-derives every
//! carrier at `Band::linear(tol)` and never reads a stored certificate.
//!
//! **The door never derives containment itself.** Callers supply the
//! evidence, one certificate per cavity shell; a shell with no
//! certificate, or a certificate that is not a strict-inside claim,
//! refuses typed before any mutation. This is deliberate: the door's
//! two no-crossing producers each hold a certification the door could
//! not reconstruct — the boolean fallback holds `point_in_solid`
//! verdicts under its boundary-disjointness certificate, and the
//! revolve holds a decided 2-D strict-containment margin (a validated
//! profile's hole-loop clearance, or the tube door's wall/bore/gap
//! verdicts on its hand-built circles), carried to 3-D verbatim by
//! revolution about the shared axis. Deriving
//! containment here (e.g. by extent boxes) would re-import the
//! box-coarseness that refuses non-convex containers (#750).
//!
//! # What the door does NOT run
//!
//! No SSI, no reduction sweep, no crossing census, no classification
//! walk, no containment probe: the caller's evidence says the two
//! boundaries share no point, so there is nothing for the crossing
//! pipeline to do. The door is a structural insertion — evidence
//! check, revert, graft — and makes no predicate-funnel decision at
//! all, `bool_`-named (the crossing pipeline's prefix; the degenerate-
//! arm suites pin that absence) or otherwise.
//!
//! # Validity
//!
//! The door neither validates its inputs nor gates its result —
//! callers own both, per their own postures (the boolean gates the
//! finished result body; the revolve asserts its tiers in debug and
//! re-validates at rest). Its argument refusals — the evidence, a
//! destination solid that does not resolve
//! ([`VoidInsertError::StaleSolid`]), destinations that do not pair
//! with the cavity's solids ([`VoidInsertError::SolidCount`]) — fire
//! before any write. Both bodies are ones every public door keeps
//! tier-1-valid, so a record the graft cannot follow in the cavity is
//! a kernel bug, and the graft panics naming it (D2 row 4) inside its
//! stage, before `dst` is written.

use geom_core::{Decide, Sign};

use super::combine::{Bridge, GraftMap, graft_solids_with};
use crate::body::Body;
use crate::entity::{EdgeKey, FaceKey, VertexKey};
use crate::entity::{ShellKey, SolidKey};
use crate::geometry::SurfaceKey;

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
    /// same axis as the outer (the holed full revolve), the tube
    /// door's own wall funnel deciding a hand-built inner circle
    /// strictly inside its outer one — thickness, bore and the
    /// realized gap between the two STORED radii — before that pair
    /// is revolved about the same axis (`tube_along_arc_hollow`), or
    /// an inward offset's d-vs-reach margin (`shell`, OFFSET-DESIGN
    /// O4). Strict
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
/// [`insert_void`] / [`insert_voids`] call: one certificate per cavity
/// shell, keyed by
/// the cavity body's **own** shell keys (pre-insertion; the door
/// reports the transplanted keys in [`VoidInserted`]).
#[derive(Clone, Debug, Default)]
pub struct VoidEvidence {
    /// The per-shell certificates.
    pub shells: Vec<(ShellKey, VoidContainment)>,
}

/// Typed refusal of [`insert_void`] (closed enum, D4 ¶3). Every
/// refusal fires before any mutation.
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
    /// A solid of the cavity holds more than one shell. Reverted, a
    /// hollow cavity's voids face outward — pieces of material under the
    /// destination solid — so this public door takes single-shell
    /// cavity solids only: it adds cavities and never a piece
    /// (`docs/DESIGN.md`, "A solid is one piece of material"). The verbs that do insert a hollow cavity (the
    /// boolean's containment fallback, `shell`) sort or re-home the
    /// pieces themselves.
    HollowCavity {
        /// The cavity solid with several shells.
        solid: SolidKey,
    },
    /// The evidence carries two certificates for one shell — a caller
    /// desync, refused rather than resolved by list order (the door
    /// never picks between conflicting claims).
    DuplicateEvidence {
        /// The doubly-certified shell.
        shell: ShellKey,
    },
    /// A destination solid does not resolve in the destination body.
    StaleSolid {
        /// The destination solid key.
        solid: SolidKey,
    },
    /// The destination solids do not pair one to one with the cavity's
    /// solids, or the cavity holds none.
    SolidCount {
        /// How many destination solids the call names.
        destinations: usize,
        /// How many solids the cavity holds.
        cavity_solids: usize,
    },
}

impl core::fmt::Display for VoidInsertError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::MissingEvidence { shell } => write!(
                f,
                "cavity shell {shell:?} carries no containment \
                 certificate — the door never derives containment; the caller \
                 must certify every cavity shell strictly inside the target"
            ),
            Self::NotStrictlyContained { shell } => write!(
                f,
                "cavity shell {shell:?}'s certificate is not a \
                 strict-inside claim — a cavity boundary must be strictly \
                 contained in the target's material"
            ),
            Self::ForeignShell { shell } => write!(
                f,
                "evidence names shell {shell:?}, which the \
                 cavity body does not hold (caller desync)"
            ),
            Self::HollowCavity { .. } => write!(
                f,
                "the cavity is hollow, and the void door inserts a cavity of one shell per \
                 solid; insert each shell's cavity on its own (caller error)"
            ),
            Self::DuplicateEvidence { shell } => write!(
                f,
                "evidence certifies shell {shell:?} twice — the door \
                 never resolves conflicting certificates by list order (caller desync)"
            ),
            Self::StaleSolid { solid } => write!(
                f,
                "destination solid {solid:?} does not resolve in the destination body"
            ),
            Self::SolidCount {
                destinations,
                cavity_solids,
            } => write!(
                f,
                "the void door takes one destination solid per cavity solid, and a cavity of \
                 at least one: the call names {destinations} for a cavity of {cavity_solids}"
            ),
        }
    }
}

/// The key bridge of one [`insert_void`] / [`insert_voids`] call:
/// cavity-body keys →
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
/// `cavity` is a **positively oriented** single-solid closed body of
/// one shell — the material that is being removed, exactly as a
/// subtraction's B operand — consumed by value; the door reverses it
/// and transplants its shell. `evidence` must certify every shell of
/// `cavity` strictly inside `dst_solid`'s material. A hollow cavity
/// refuses ([`VoidInsertError::HollowCavity`]): its voids would face
/// outward once reverted, pieces of material under one solid.
///
/// # Errors
///
/// [`VoidInsertError`] — [`insert_voids`]'s, before any mutation.
pub fn insert_void<T: Decide>(
    dst: &mut Body<T>,
    dst_solid: SolidKey,
    cavity: Body<T>,
    evidence: &VoidEvidence,
) -> Result<VoidInserted, VoidInsertError> {
    insert_voids(dst, &[dst_solid], cavity, evidence)
}

/// [`insert_void`] for a cavity body holding N solids: `dst_solids`
/// names one destination solid per cavity solid, **positionally in the
/// cavity's solid order** ([`super::combine::graft_solids_with`]'s own
/// contract), and the arity must match exactly.
///
/// The evidence discipline is [`insert_void`]'s, unchanged and
/// whole-body: one strict-inside certificate per cavity shell,
/// checked before any mutation. What the arity adds is only WHERE each
/// cavity solid's shells land — every one of them under its own
/// destination, none of them crossing between destinations.
///
/// [`insert_void`] is this door's `N = 1` case and shares its every
/// step, so a single-destination call is entity-for-entity what it
/// always was.
///
/// # Errors
///
/// [`VoidInsertError`] — a destination count that does not match the
/// cavity's solid count, or a cavity of no solid, as
/// [`VoidInsertError::SolidCount`]; a destination solid that does not
/// resolve in `dst` as [`VoidInsertError::StaleSolid`]; the evidence
/// refusals. Every refusal leaves `dst`
/// unchanged.
///
/// # Panics
///
/// Where a record of the cavity does not resolve, naming it (D2 row 4):
/// a body no public door leaves. The graft is staged, so `dst` is
/// unwritten when it fires.
pub fn insert_voids<T: Decide>(
    dst: &mut Body<T>,
    dst_solids: &[SolidKey],
    cavity: Body<T>,
    evidence: &VoidEvidence,
) -> Result<VoidInserted, VoidInsertError> {
    if let Some((solid, _)) = cavity.solids().find(|(_, s)| s.shells.len() > 1) {
        return Err(VoidInsertError::HollowCavity { solid });
    }
    insert_hollow_voids(dst, dst_solids, cavity, evidence)
}

/// [`insert_voids`] admitting a hollow cavity, for the verbs that file
/// its pieces themselves: a hollow cavity's voids face outward once
/// reverted, so every shell landing under one solid is the caller's
/// transient, never its result. The shell verb re-homes each
/// transplanted void twin with the operand void it pairs with
/// ([`crate::shell`](mod@crate::shell)'s thin-solid step, paired off
/// the graft map), and every boolean result is sorted into pieces at
/// its exit ([`crate::pieces`]).
///
/// # Errors
///
/// As [`insert_voids`], less [`VoidInsertError::HollowCavity`].
pub(crate) fn insert_hollow_voids<T: Decide>(
    dst: &mut Body<T>,
    dst_solids: &[SolidKey],
    cavity: Body<T>,
    evidence: &VoidEvidence,
) -> Result<VoidInserted, VoidInsertError> {
    // ---- Argument checks (pure reads, first — no mutation happens
    // unless the destinations resolve and pair with the cavity's
    // solids, and every cavity shell is certified strictly inside). ----
    let cavity_solids = cavity.solids().count();
    if dst_solids.len() != cavity_solids || cavity_solids == 0 {
        return Err(VoidInsertError::SolidCount {
            destinations: dst_solids.len(),
            cavity_solids,
        });
    }
    if let Some(&solid) = dst_solids.iter().find(|&&k| dst.get_solid(k).is_none()) {
        return Err(VoidInsertError::StaleSolid { solid });
    }
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
    let reversed = cavity.revert();
    let graft =
        graft_solids_with(dst, dst_solids, &reversed, Bridge::RemapKeys).unwrap_or_else(|e| {
            unreachable!(
                "the void graft refused ({e:?}): its every argument refusal was checked above \
                 (destinations live and one per cavity solid), and a handle-remapping graft \
                 re-certifies nothing"
            )
        });
    #[cfg(debug_assertions)]
    dst.assert_euler_postcondition(before, transplant, "insert_voids");
    Ok(VoidInserted { graft })
}

/// **A torn cavity panics before the destination is written.** The
/// cavity is a body every public door keeps tier-1-valid, so a record
/// the door cannot follow is a kernel bug (D2 row 4), and the panic
/// names it. No public door tears a body, so the row tears one in-crate:
/// a vertex's point removed. Where debug assertions are on, the
/// reversal's tier-1 postcondition meets it first; without them the
/// graft does, inside its stage. Either names the vertex and its point.
#[cfg(test)]
#[allow(clippy::expect_used)]
mod torn_cavity_rows {
    use super::{VoidContainment, VoidEvidence, insert_void};
    use crate::test_support_fixtures::brick;
    use geom_core::{Sign, Tol};

    #[test]
    fn a_torn_cavity_panics_naming_the_record_with_dst_unchanged() {
        let tol = Tol::witness();
        let mut dst = brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol);
        let solid = dst.solids().next().expect("a solid").0;
        let mut cavity = brick::<f64>((0.25, 0.75), (0.25, 0.75), (0.25, 0.75), tol);
        let (vertex, point) = cavity
            .vertices()
            .next()
            .map(|(k, v)| (k, v.point))
            .expect("a vertex");
        cavity.points.remove(point).expect("its point");
        let evidence = VoidEvidence {
            shells: cavity
                .shells()
                .map(|(s, _)| {
                    let sign = Sign::Positive;
                    (s, VoidContainment::Carried { sign })
                })
                .collect(),
        };
        let before = format!("{dst:?}");
        let report = crate::surgery::tests::panic_message(std::panic::AssertUnwindSafe(|| {
            let _ = insert_void(&mut dst, solid, cavity, &evidence);
        }));
        for fragment in [format!("{vertex:?}"), format!("{point:?}")] {
            assert!(report.contains(&fragment), "want {fragment:?} in: {report}");
        }
        #[cfg(not(debug_assertions))]
        {
            use crate::entity::{EntityId, GeomRef};
            let want = format!(
                "{}'s point names {}, which does not resolve",
                EntityId::Vertex(vertex),
                GeomRef::Point(point)
            );
            assert!(report.contains(&want), "want {want:?} in: {report}");
        }
        assert_eq!(
            format!("{dst:?}"),
            before,
            "the panic wrote the destination"
        );
    }
}
