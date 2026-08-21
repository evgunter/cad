//! `vtxfacclassify` — the vertex-on-face classifier the book never
//! prints ("for space reasons", §15.6.1): the ch. 14 classifier with
//! the three deltas, DESIGNED here:
//!
//! 1. **plane := the pierced face's plane**; classes are [`SideCode`]s
//!    with OUT/IN derived from the F3 chain (a chord's class is its
//!    signed elevation off the face plane against the OUTWARD normal —
//!    `side_code`; 15.7's printed `IN = +1` is never consulted).
//! 2. **On-sectors reclassify per Eq. 15.3** (op-dependent), behind
//!    [`super::oriented_plane_eq`] — a coplanar sector must be
//!    *declaredly* coplanar with the pierced face or the op refuses.
//! 3. **The ring insertion** (the unprinted Euler sequence, designed):
//!    in the pierced body,
//!    `mev(Fan{anchor, anchor})` a **chord strut** from a deterministic
//!    boundary vertex of the face to the pierce point (a real,
//!    certified line edge, dangling — structurally legal mid-op), then
//!    `kemr(chord.he_plus, chord.he_minus)` kills the chord and leaves
//!    the pierce vertex as an **empty-loop ring** inside the face
//!    (KemrResult: he1's strictly-between side is empty ⇒ the ring is
//!    the lone new vertex — the dangling chord exists only between the
//!    two ops), then one `mev_null` **strut per piercing-side run**
//!    hangs the paired null edges off the ring vertex
//!    (`MevSite::Lone` for the first, `Fan{he,he}` after). Dangling
//!    ring null edges are the documented ch. 15 transient; joining
//!    consumes them in PR 5. Tier 1 holds after every op (pinned by the
//!    acceptance test).
//!
//! **On-edges** (an edge through the pierce vertex lying IN the face's
//! plane): resolved by the flanking classes — `(In,·,In) → In`,
//! `(Out,·,Out) → Out`, mixed → `In`. This deliberately DIVERGES from
//! the split lane's F4 table (`BOB → ABOVE`): the split must mint
//! copies to keep the two pieces' fans representable, but a boolean
//! tangential contact is a *legal 3′ touching* (edge-on-face, both
//! flanking faces the same side) already carried by the declared
//! contact records — TOG Table II rows 5/9 (`(In,In)`/`(Out,Out)` ⇒
//! no intersection) confirm no crossing is recorded. Mixed keeps the
//! In side (both witnesses' choice for the split analogue). Flagged in
//! the PR report for ratification.

use geom_core::{Band, Decide, Margin, Sign};

use super::plane_eq::PlaneEqError;
use super::reduce::{face_outward_normal, face_plane};
use super::sectors::{build_sectors, side_code};
use super::tables::eq15_3_lump;
use super::{
    BoolNullEdgeRecord, BooleanError, BooleanOp, NullEdgePairRecord, Operand, PairSite,
    PierceRingRecord, SideCode, VfContact,
};
use crate::body::Body;
use crate::entity::HalfEdgeKey;
use crate::euler::MevSite;
use crate::null::{NewVertexSide, NullEdge};
use crate::validate::decide;
use geom_core::Tol;

/// Output of one vertex-on-face classification.
#[derive(Debug)]
pub(super) struct VtxFacOut<T: geom_core::Real> {
    /// Minted null edges (piercing-side runs + pierced-side ring struts).
    pub edges: Vec<BoolNullEdgeRecord<T>>,
    /// Cross-body correspondence pairs.
    pub pairs: Vec<NullEdgePairRecord>,
    /// The ring insertion, if surgery happened.
    pub ring: Option<PierceRingRecord>,
}

#[derive(Clone, Copy, Debug)]
struct Entry {
    he: HalfEdgeKey,
    is_edge: bool,
    class: SideCode,
}

/// Classifies `contact.vertex` (in the piercing body) against
/// `contact.face` (in the pierced body) and performs the paired
/// insertion (module docs).
#[allow(clippy::too_many_arguments)]
pub(super) fn classify_vertex_on_face<T: Decide>(
    piercing_body: &mut Body<T>,
    pierced_body: &mut Body<T>,
    piercing: Operand,
    contact: VfContact,
    op: BooleanOp,
    declared: &super::DeclaredPairs,
    band: Band,
    tol: Tol,
) -> Result<VtxFacOut<T>, BooleanError> {
    let vertex = contact.vertex;
    // Both halves of the pierced face's oriented datum, from the one
    // door: `plane` for the plane algebra, `n_pierced` for the
    // material-side verdicts, which take the typed normal so the
    // `sense_sign` cannot be dropped on the way.
    let (plane, n_pierced) = face_plane(pierced_body, contact.face)
        .zip(face_outward_normal(pierced_body, contact.face))
        .ok_or(BooleanError::ClassificationInvariant {
            what: "pierced face lost its plane",
        })?;
    let sectors = build_sectors(piercing_body, piercing, vertex, band)?;
    let n = sectors.len();

    // Entries = the bounds in orbit order (entry k = sector k's END
    // bound: real chord or subdivision bisector), classed against the
    // pierced face's plane via the F3 primitive. `n_pierced` is the
    // pierced face's OUTWARD normal (S10, minted by
    // `face_outward_normal`) — In/Out here is a material verdict and
    // would read backwards off a chart normal on a reversed face,
    // which is why the primitive takes the typed one.
    let mut entries = Vec::with_capacity(n);
    for s in &sectors {
        entries.push(Entry {
            he: s.he,
            is_edge: s.end_edge,
            class: side_code(s.end, n_pierced, s.arm, band)?,
        });
    }

    // Delta 2 (rule (a) analogue): coplanar sectors lump per Eq. 15.3.
    for (k, s) in sectors.iter().enumerate() {
        let m = Margin::levered(s.normal.vec().cross(plane.normal).norm(), s.arm);
        match decide("bool_sector_coplanar", m, band) {
            Ok(Sign::Zero) => {}
            Ok(_) => continue,
            Err(diag) => return Err(BooleanError::Escalated { diag }),
        }
        // A CURVED sector whose local normal is plane-parallel at the
        // pierce point is a tangent (touching) contact — the typed
        // frontier CONTACT-DESIGN C7 item 1 names for THIS wall: no
        // curved coplanar-lump arm exists, so touching curved
        // configurations refuse. The recourse is a declared
        // Tangent/Rest contact (vocabulary CONTACT-DESIGN C4), under
        // which classification descends to CURVED-DESIGN C7's sector
        // trilean instead of refusing.
        let Some(sector_plane) = face_plane(piercing_body, s.face) else {
            let kind = piercing_body
                .get_face(s.face)
                .and_then(|f| piercing_body.get_surface(f.surface))
                .map_or(geom_brep::SurfaceKind::Nurbs, geom_brep::SurfaceKind::of);
            return Err(BooleanError::CurvedBooleanUnsupported {
                operand: piercing,
                face: s.face,
                kind,
            });
        };
        let pierced_op = piercing.other();
        // Oriented sources (S10): `sector_plane` and `plane` are both
        // OUTWARD normals, so rung 1's syntactic Same± verdict has to
        // see the face senses as well as the surfaces' `orient` tags.
        let (g1, g2) = (
            super::reduce::face_plane_source(piercing_body, s.face),
            super::reduce::face_plane_source(pierced_body, contact.face),
        );
        let id = super::PlaneIdentity {
            s1: g1.as_ref(),
            s2: g2.as_ref(),
            declared: declared.declares_rest(piercing, s.face, pierced_op, contact.face),
        };
        let rel = match super::oriented_plane_eq(&sector_plane, &plane, id, s.arm, band) {
            Ok(super::PlaneRelation::Distinct) => {
                return Err(BooleanError::ClassificationInvariant {
                    what: "geometrically coplanar sector with definitely-distinct plane",
                });
            }
            Ok(rel) => rel,
            Err(PlaneEqError::Escalated(diag)) => return Err(BooleanError::Escalated { diag }),
            Err(PlaneEqError::Undeclared { diag, relation }) => {
                return Err(BooleanError::UndeclaredCoincidence {
                    diag,
                    pair: [(piercing, s.face), (pierced_op, contact.face)],
                    relation,
                });
            }
            Err(PlaneEqError::Contradicted(diag)) => {
                return Err(BooleanError::DeclarationContradicted { diag });
            }
        };
        let lump = eq15_3_lump(op, piercing, rel);
        entries[k].class = lump;
        entries[(k + 1) % n].class = lump;
    }

    // On-edge resolution (module docs; the deliberate divergence).
    for k in 0..n {
        if entries[k].class == SideCode::On && entries[(k + 1) % n].class == SideCode::On {
            return Err(BooleanError::ClassificationInvariant {
                what: "consecutive On entries after Eq. 15.3 lumping",
            });
        }
    }
    for k in 0..n {
        if entries[k].class != SideCode::On {
            continue;
        }
        let prev = entries[(k + n - 1) % n].class;
        let next = entries[(k + 1) % n].class;
        entries[k].class = match (prev, next) {
            (SideCode::Out, SideCode::Out) => SideCode::Out,
            _ => SideCode::In,
        };
    }

    // Out-runs (the copy takes the OUT side — above ≙ OUT).
    let runs = out_runs(&entries);
    let mut out = VtxFacOut {
        edges: Vec::new(),
        pairs: Vec::new(),
        ring: None,
    };
    if runs.is_empty() {
        return Ok(out); // tangential touch: 3′ contact only, no surgery
    }

    // Piercing-side null edges, one per run (PR 2's insertion pattern).
    // Germ facings (F9 data): the run's two boundary transitions are
    // its germs; both lie in the pierced face's plane, so the germ's
    // face pair = (transition sector's face, pierced face) in operand
    // order. Parity: the class after crossing forward — Out at the
    // run's start germ, In at its end germ (site-shared with the ring
    // strut below).
    let germ_pair = |own: crate::entity::FaceKey| match piercing {
        Operand::A => (own, contact.face),
        Operand::B => (contact.face, own),
    };
    let mut run_germs = Vec::new();
    let mut run_edges = Vec::new();
    for run in &runs {
        let s_start = &sectors[(run.0 + n - 1) % n];
        let s_end = &sectors[(run.0 + run.1 - 1) % n];
        let dir_start = pierce_germ_dir(s_start, plane.normal, band)?;
        let dir_end = pierce_germ_dir(s_end, plane.normal, band)?;
        run_germs.push((
            (germ_pair(s_start.face), dir_start),
            (germ_pair(s_end.face), dir_end),
        ));
        let members = (0..run.1).map(|j| entries[(run.0 + j) % n]);
        let mut real = members.filter(|e| e.is_edge);
        let first = real.next();
        let last = real.next_back().or(first);
        let (site, dangling) = match (first, last) {
            (Some(first), Some(last)) => {
                let mate = piercing_body
                    .mate(last.he)
                    .ok_or(BooleanError::CorruptOperand {
                        operand: piercing,
                        vertex,
                    })?;
                let he2 = piercing_body
                    .get_half_edge(mate)
                    .ok_or(BooleanError::CorruptOperand {
                        operand: piercing,
                        vertex,
                    })?
                    .next;
                (MevSite::Fan { he1: first.he, he2 }, false)
            }
            _ => {
                let after = entries[(run.0 + run.1) % n];
                (
                    MevSite::Fan {
                        he1: after.he,
                        he2: after.he,
                    },
                    true,
                )
            }
        };
        // Sense theorem (join module docs): the half facing the run's
        // START germ (forward code Out) must be the UP half — for the
        // dangling splice that half is he_minus (starts at the copy),
        // so the SIDE swaps with the facing (mint side flipped to keep
        // the body scaffold attribute and the record one datum).
        let side = if dangling {
            NewVertexSide::Below
        } else {
            NewVertexSide::Above
        };
        let created = piercing_body.mev_null(site, side)?;
        let Some(&((gs, ds), (ge, de))) = run_germs.last() else {
            return Err(BooleanError::ClassificationInvariant {
                what: "run germ bookkeeping desynchronized",
            });
        };
        let (start_he, end_he) = if dangling {
            (created.he_minus, created.he_plus)
        } else {
            (created.he_plus, created.he_minus)
        };
        let attr = match side {
            NewVertexSide::Below => NullEdge {
                below_end: created.vertex,
                above_end: vertex,
            },
            NewVertexSide::Above => NullEdge {
                below_end: vertex,
                above_end: created.vertex,
            },
        };
        let rec = BoolNullEdgeRecord {
            operand: piercing,
            at_vertex: vertex,
            edge: created.edge,
            attr,
            dangling,
            germs: [
                super::HalfGerm {
                    he: start_he,
                    a_face: gs.0,
                    b_face: gs.1,
                    dir: ds,
                },
                super::HalfGerm {
                    he: end_he,
                    a_face: ge.0,
                    b_face: ge.1,
                    dir: de,
                },
            ],
        };
        run_edges.push(rec);
        out.edges.push(rec);
    }

    // Delta 3: the pierce ring in the pierced face (module docs).
    let p = *piercing_body
        .get_point(
            piercing_body
                .get_vertex(vertex)
                .ok_or(BooleanError::CorruptOperand {
                    operand: piercing,
                    vertex,
                })?
                .point,
        )
        .ok_or(BooleanError::CorruptOperand {
            operand: piercing,
            vertex,
        })?;
    let pierced = piercing.other();
    let face_data =
        pierced_body
            .get_face(contact.face)
            .ok_or(BooleanError::ClassificationInvariant {
                what: "pierced face vanished",
            })?;
    let crate::entity::LoopBoundary::Cycle { first: anchor } = pierced_body
        .get_loop(face_data.outer)
        .ok_or(BooleanError::ClassificationInvariant {
            what: "pierced face outer loop vanished",
        })?
        .boundary
    else {
        return Err(BooleanError::ClassificationInvariant {
            what: "pierced face outer loop is not a cycle",
        });
    };
    let u = pierced_body
        .get_half_edge(anchor)
        .ok_or(BooleanError::ClassificationInvariant {
            what: "anchor half-edge vanished",
        })?
        .start;
    let p_u = *pierced_body
        .get_point(
            pierced_body
                .get_vertex(u)
                .ok_or(BooleanError::CorruptOperand {
                    operand: pierced,
                    vertex: u,
                })?
                .point,
        )
        .ok_or(BooleanError::CorruptOperand {
            operand: pierced,
            vertex: u,
        })?;
    // (1) chord strut u → pierce point (certified line, transient).
    let chord = pierced_body.mev(
        MevSite::Fan {
            he1: anchor,
            he2: anchor,
        },
        p,
        geom_brep::EdgeCurveSpec::line_between(p_u, p),
        tol,
    )?;
    // (2) detach as an empty-loop ring at the pierce vertex.
    let kemr = pierced_body.kemr(chord.he_plus, chord.he_minus)?;
    let w = chord.vertex;
    out.ring = Some(PierceRingRecord {
        operand: pierced,
        face: contact.face,
        ring_vertex: w,
    });
    // (3) one ring null-edge strut per piercing-side run. Side labels
    // are DERIVED sense data (PR 5.5, join module docs): the pierced
    // solid's sense at each germ is the negation of the piercing
    // solid's (the cross-solid anti-correlation theorem), so the half
    // facing the run's start germ (piercing UP) is the pierced DOWN
    // half — he_minus, starting at the created copy = `above_end`.
    let mut ring_anchor: Option<HalfEdgeKey> = None;
    for (run_edge, ((gs, ds), (ge, de))) in run_edges.iter().zip(&run_germs) {
        let site = match ring_anchor {
            None => MevSite::Lone { r#loop: kemr.ring },
            Some(he) => MevSite::Fan { he1: he, he2: he },
        };
        let created = pierced_body.mev_null(site, NewVertexSide::Above)?;
        ring_anchor.get_or_insert(created.he_plus);
        let rec = BoolNullEdgeRecord {
            operand: pierced,
            at_vertex: w,
            edge: created.edge,
            attr: NullEdge {
                below_end: w,
                above_end: created.vertex,
            },
            dangling: true,
            germs: [
                super::HalfGerm {
                    he: created.he_minus,
                    a_face: gs.0,
                    b_face: gs.1,
                    dir: *ds,
                },
                super::HalfGerm {
                    he: created.he_plus,
                    a_face: ge.0,
                    b_face: ge.1,
                    dir: *de,
                },
            ],
        };
        out.edges.push(rec);
        let (a_edge, b_edge, site) = match piercing {
            Operand::A => (
                run_edge.edge,
                created.edge,
                PairSite::VertexAOnFaceB(contact),
            ),
            Operand::B => (
                created.edge,
                run_edge.edge,
                PairSite::VertexBOnFaceA(contact),
            ),
        };
        out.pairs.push(NullEdgePairRecord {
            a_edge,
            b_edge,
            site,
        });
    }
    Ok(out)
}

/// The germ direction at a pierce-site transition: the unit
/// intersection direction of the transition sector's face plane with
/// the pierced plane, signed to lie within the sector (grazes count —
/// an on-edge germ IS a bound). Ambiguity refuses loudly.
///
/// Sense-invariant given its sources (S10): the cross product names a
/// LINE and `within` picks the ray, so neither normal's sign survives
/// into the answer. Both arrive oriented already — `s.normal` from
/// `sectors::sector_face`, `plane_normal` from `reduce::face_plane` —
/// and neither is multiplied again here.
fn pierce_germ_dir<T: Decide>(
    s: &super::sectors::BoolSector<T>,
    plane_normal: geom_core::Vec3<T>,
    band: Band,
) -> Result<geom_core::Vec3<T>, BooleanError> {
    let int = s.normal.vec().cross(plane_normal);
    match decide("bool_germ_line", Margin::levered(int.norm(), s.arm), band) {
        Ok(Sign::Positive) => {}
        Ok(_) => {
            return Err(BooleanError::ClassificationInvariant {
                what: "pierce transition on a coplanar sector",
            });
        }
        Err(diag) => return Err(BooleanError::Escalated { diag }),
    }
    let d = int.normalize();
    let plus = super::sectors::within(s, d, false, band)?;
    let minus = super::sectors::within(s, -d, false, band)?;
    match (plus, minus) {
        (true, false) => Ok(d),
        (false, true) => Ok(-d),
        _ => Err(BooleanError::ClassificationInvariant {
            what: "pierce germ direction not uniquely within its sector",
        }),
    }
}

/// Maximal cyclic Out-runs `(start, len)` (PR 2's `above_runs` on
/// [`SideCode`]; anchored at the first In entry; one-sided
/// neighborhoods have none).
fn out_runs(entries: &[Entry]) -> Vec<(usize, usize)> {
    let n = entries.len();
    let Some(anchor) = entries.iter().position(|e| e.class == SideCode::In) else {
        return Vec::new();
    };
    let mut runs = Vec::new();
    let mut k = 0;
    while k < n {
        let idx = (anchor + 1 + k) % n;
        if entries[idx].class == SideCode::Out {
            let start = idx;
            let mut len = 0;
            while k < n && entries[(anchor + 1 + k) % n].class == SideCode::Out {
                len += 1;
                k += 1;
            }
            runs.push((start, len));
        } else {
            k += 1;
        }
    }
    runs
}
