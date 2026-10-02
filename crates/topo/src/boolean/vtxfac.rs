//! `vtxfacclassify` — the vertex-on-face classifier the book never
//! prints ("for space reasons", §15.6.1): the ch. 14 classifier with
//! the three deltas, DESIGNED here:
//!
//! 1. **datum := the pierced face's TANGENT plane at the pierce
//!    point**; classes are [`SideCode`]s with OUT/IN derived from the
//!    F3 chain (a chord's class is its signed elevation off that plane
//!    against the OUTWARD normal — `side_code`; 15.7's printed
//!    `IN = +1` is never consulted).
//!
//!    The datum is a DIRECTION, never an origin: `side_code`
//!    (`sectors.rs`) takes `(dir, reach, OutwardNormal, lever, band)`
//!    and the germ direction takes a cross product, so both are
//!    first-order primitives (charged for the face's curvature through
//!    `lever`) and generalize to a curved pierced face by substituting
//!    the per-point outward normal
//!    ([`crate::face_normal::face_outward_normal_at`]). On a plane that
//!    normal is the plane's own, so the planar lane's arithmetic is
//!    bit-identical. What does NOT generalize is Delta 2's carrier
//!    algebra, which needs an origin — it refuses typed on a curved
//!    pierced face rather than inventing one.
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
//! `(Out,·,Out) → Out`, mixed → `In`, the one fold rule
//! ([`super::sectors::fold_on_bound`]) the vertex-vertex attribution
//! shares. This deliberately DIVERGES from the split lane's F4 table
//! (`BOB → ABOVE`): the split must mint copies to keep the two pieces'
//! fans representable, but a boolean tangential contact is a *legal 3′
//! touching* (edge-on-face, both flanking faces the same side) already
//! carried by the declared contact records — TOG Table II rows 5/9
//! (`(In,In)`/`(Out,Out)` ⇒ no intersection) confirm no crossing is
//! recorded.

use geom_core::{Band, Decide, Margin, Sign};

use super::plane_eq::PlaneEqError;
use super::reduce::face_plane;
use super::sectors::{build_sectors, side_code};
use super::tables::{eq15_3_lump, lump_keeps_one};
use super::{
    BoolNullEdgeRecord, BooleanError, BooleanOp, NullEdgePairRecord, Operand, PairSite,
    PierceRingRecord, SideCode, VfContact,
};
use super::{Coincide, Contradiction, DeclarationRead};
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
    /// `(A face, B face)` for each coincident sector whose lump keeps
    /// one copy of the region (`BooleanReduction::covered`).
    pub covered: Vec<(crate::entity::FaceKey, crate::entity::FaceKey)>,
}

#[derive(Clone, Copy, Debug)]
struct Entry {
    he: HalfEdgeKey,
    is_edge: bool,
    class: SideCode,
    /// The class is Delta 2's lump, not this bound's own reading.
    lumped: bool,
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
    let pierced_op = piercing.other();
    // The pierce point, read BEFORE the classification: on a curved
    // pierced face the oriented datum is point-dependent, so `p` is an
    // input to the sector algebra rather than only to Delta 3's ring.
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
    // The pierced face's oriented datum at `p`, from the one door.
    // `n_pierced` carries the material side, typed so the sense flip
    // cannot be dropped on the way; on a PLANE it is bit-identically
    // what [`face_plane`] reports as its `normal`
    // ([`super::reduce::face_plane`] builds it from this very door),
    // which is what keeps the planar lane's arithmetic unmoved.
    //
    // `plane` itself survives only for Delta 2's planar carrier
    // algebra, which needs an ORIGIN as well as a direction; the
    // curved lane refuses there rather than building one (below).
    let plane = face_plane(pierced_body, contact.face);
    let n_pierced =
        match crate::face_normal::face_outward_normal_at(pierced_body, contact.face, p, band) {
            Ok(Some(n)) => n,
            // Cone / NURBS pierced faces: the C5 typed refusal, naming
            // the kind that has no arm.
            Ok(None) => {
                return Err(BooleanError::CurvedBooleanUnsupported {
                    operand: pierced_op,
                    face: contact.face,
                    kind: pierced_kind(pierced_body, contact.face),
                });
            }
            Err(refusal) => {
                return Err(BooleanError::of_pierced_normal(
                    refusal,
                    pierced_op,
                    contact.face,
                ));
            }
        };
    // The pierced face's smallest radius of curvature — the lever the
    // sector side verdicts charge their sagitta against (`side_code`'s
    // argument), so it must bound the tightest bend, not the chart's
    // scale: on a fat torus those differ. A plane reports `f64::MAX`, so
    // its charge is vacuous and the planar lane's verdicts are unmoved.
    let pierced_lever = pierced_body
        .get_face(contact.face)
        .and_then(|f| pierced_body.get_surface(f.surface))
        .map_or_else(super::sectors::NO_CURVATURE, |s| {
            geom_brep::min_radius_of_curvature(s, p)
        });
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
            is_edge: s.end_edge(),
            class: side_code(s.end, s.end_reach, n_pierced, pierced_lever, band)?,
            lumped: false,
        });
    }

    // Delta 2 (rule (a) analogue): coplanar sectors lump per Eq. 15.3.
    // "Coplanar" is against the pierced face's TANGENT plane at `p`,
    // whose normal is `n_pierced` — the same vector `face_plane` hands
    // back on a planar face, so the planar lane's margin is unmoved.
    //
    // Lumping overwrites both bounds' readings, so both must read On,
    // each at its reach — a line bound at its far vertex, in metres; a
    // bound read definitely off the plane decides the sector off it.
    // Where both read On, the normals' parallelism at the arm proposes
    // coplanarity. A tilt the bounds read On and the normals do not (in
    // band, or decided off: a thin sector tilted about one bound rises
    // at its arm by more than at its other bound) leaves the question
    // undecided at this tolerance, and the margin that decides it is the
    // bounds' own: a smaller tolerance reads the steeper bound off the
    // plane and the sector with it.
    let read: Vec<SideCode> = entries.iter().map(|e| e.class).collect();
    let mut covered = Vec::new();
    for (k, s) in sectors.iter().enumerate() {
        if read[k] != SideCode::On || read[(k + 1) % n] != SideCode::On {
            continue;
        }
        let m = Margin::levered(s.normal.vec().cross(n_pierced.vec()).norm(), s.arm);
        let tilt = decide("bool_sector_coplanar", m, band);
        // The declaration is read before the tilt refuses: a declared
        // pair's lump takes an in-band residue (the carrier ladder's
        // declared rung bridges it on a planar pierced face; the
        // `Tangent` lump descends to the second order), so only an
        // undeclared pair refuses it here, and the class the door admits
        // for this pierced face would change its verdict. A tilt decided
        // off is no coincidence a declaration settles: a declared pair's
        // claim is contradicted by it, and an undeclared pair's sector is
        // undecided on its bounds' margin with no declaration to offer.
        let class = declared.class_of(piercing, s.face, pierced_op, contact.face);
        let decided_tilt = matches!(tilt, Ok(Sign::Positive | Sign::Negative));
        if let (true, Some(class)) = (decided_tilt, class) {
            let (a, b) = match piercing {
                Operand::A => (s.face, contact.face),
                Operand::B => (contact.face, s.face),
            };
            let planar = plane.is_some()
                && matches!(
                    super::rest::face_carrier(piercing_body, s.face),
                    Some(super::carrier_eq::CarrierDesc::Plane { .. })
                );
            let fact = (planar && class == crate::contact::ContactClass::Rest)
                .then_some(Contradiction::PlanesNotParallel);
            return Err(BooleanError::ContactContradicted {
                declaration: crate::contact::DeclaredContact { a, b, class },
                steer: fact.and_then(super::contact_verify::fit_steer),
                fact,
                margin: geom_core::Indeterminate {
                    margin: geom_core::MarginDiag::INVALID,
                    band,
                    predicate: Some("bool_sector_coplanar"),
                    terminal_sliver: false,
                },
            });
        }
        let refused = match (&tilt, class) {
            (Ok(Sign::Zero), _) | (Err(_), Some(_)) => false,
            (Ok(Sign::Positive | Sign::Negative), _) | (Err(_), None) => true,
        };
        if refused {
            // `Rest` bridges the residue only against a planar pierced
            // face (a curved one refuses below whatever is declared);
            // `Tangent` only where the door's witness lane derives the
            // pair's tangency, which it checks before it admits one.
            fn surface<T: Decide>(
                body: &Body<T>,
                f: crate::entity::FaceKey,
            ) -> Option<&geom::Surface<T>> {
                body.get_face(f)
                    .and_then(|face| body.get_surface(face.surface))
            }
            let tangent = match (
                surface(piercing_body, s.face),
                surface(pierced_body, contact.face),
            ) {
                (Some(a), Some(b)) => geom_brep::tangent_locus(a, b, band).is_ok(),
                _ => false,
            };
            let admitted: &[crate::contact::ContactClass] = match (plane.is_some(), tangent) {
                (true, true) => &[
                    crate::contact::ContactClass::Rest,
                    crate::contact::ContactClass::Tangent,
                ],
                (true, false) => &[crate::contact::ContactClass::Rest],
                (false, true) => &[crate::contact::ContactClass::Tangent],
                (false, false) => &[],
            };
            // A decided tilt admits no class: no declaration settles it.
            let admitted = if decided_tilt { &[] } else { admitted };
            let read = declared.read(
                &[(piercing, s.face, pierced_op, contact.face)],
                Coincide::Sectors,
                admitted,
            );
            let nv = n_pierced.vec();
            let steeper = s
                .start_reach
                .departure(s.start, nv)
                .abs()
                .max(s.end_reach.departure(s.end, nv).abs());
            return match crate::validate::decide_nonzero_reported(
                "bool_sector_coplanar",
                Margin::of(steeper),
                band,
            ) {
                Err(diag) => Err(BooleanError::coincidence(Coincide::Sectors, read, diag)),
                Ok(_) => Err(BooleanError::ClassificationInvariant {
                    what: "a sector bound read On departs definitely from the plane",
                }),
            };
        }
        // Declared-`Tangent` (distinct carriers touching): the lump
        // verdict is the second-order sector trilean — which side the
        // sector's carrier CURVES to relative to the pierced face's
        // material ([`super::sectors::tangent_lump`]), read for the WHOLE
        // sector. Per bound, the bound riding the locus reads `On`, which
        // the on-entry resolution below settles from its neighbours; but
        // an arc tangent at this vertex is split at the band's edge, and
        // the sliver's arm puts the arc's second-order margin in the zero
        // band too, so the two `On`s are the consecutive-`On` refusal
        // (`work/hone/an-arc-tangent-to-a-face-at-its-end-is-split-at-the-edge-of-the-band.md`;
        // pinned by `m9_3_zip::a_tangent_curved_sector_on_a_face_lumps_whole`).
        if class == Some(crate::contact::ContactClass::Tangent) {
            let surface_of = |body: &Body<T>, f| {
                body.get_face(f)
                    .and_then(|face| body.get_surface(face.surface))
                    .cloned()
                    .ok_or(BooleanError::ClassificationInvariant {
                        what: "declared-Tangent face lost its surface",
                    })
            };
            let s_sector = surface_of(piercing_body, s.face)?;
            let s_pierced = surface_of(pierced_body, contact.face)?;
            let read = declared.read(
                &[(piercing, s.face, pierced_op, contact.face)],
                Coincide::TangentSide,
                &[],
            );
            let lump = super::sectors::tangent_lump(
                &s_sector, &s_pierced, n_pierced, p, op, piercing, s.face, s.arm, read, band,
            )?;
            entries[k].class = lump;
            entries[(k + 1) % n].class = lump;
            entries[k].lumped = true;
            entries[(k + 1) % n].lumped = true;
            continue;
        }
        // The conformal (carrier) lump. The sector's oriented carrier
        // description generalizes the plane one (`face_carrier` folds
        // the face sense exactly as `face_plane` does — S10); a kind
        // outside the `Rest` ladder's inventory (cone, torus, NURBS)
        // keeps the C5 typed refusal.
        let Some(sector_carrier) = super::rest::face_carrier(piercing_body, s.face) else {
            let kind = piercing_body
                .get_face(s.face)
                .and_then(|f| piercing_body.get_surface(f.surface))
                .map_or(geom::SurfaceKind::Nurbs, geom::Surface::kind);
            return Err(BooleanError::CurvedBooleanUnsupported {
                operand: piercing,
                face: s.face,
                kind,
            });
        };
        let declared_rest = class == Some(crate::contact::ContactClass::Rest);
        // C8: a CURVED on-carrier sector is opened by a VERIFIED
        // declaration and by nothing else — undeclared touching keeps
        // this typed frontier refusal. The recourse is a declared
        // Tangent/Rest contact (vocabulary CONTACT-DESIGN C4), under
        // which classification descends to the carrier ladder or the
        // C7 sector trilean instead of refusing.
        if !declared_rest && !matches!(sector_carrier, super::carrier_eq::CarrierDesc::Plane { .. })
        {
            let kind = piercing_body
                .get_face(s.face)
                .and_then(|f| piercing_body.get_surface(f.surface))
                .map_or(geom::SurfaceKind::Nurbs, geom::Surface::kind);
            return Err(BooleanError::CurvedBooleanUnsupported {
                operand: piercing,
                face: s.face,
                kind,
            });
        }
        // **UNTESTED, and that is a statement rather than an
        // omission.** No authorable body reaches this arm today: it
        // needs a sector face geometrically TANGENT to a CURVED pierced
        // face at the pierce point, and every curved pierce still dies
        // at the join before a second operand pose can be built around
        // one. The arm is written because the alternative is a plane
        // built from a face that has none — see the argument below —
        // and it is named here so a later unit that opens the ring's
        // join knows this is the first row it owes a fixture.
        //
        // **Delta 2 stays SHUT on a curved pierced face.** The rung
        // below descends to `carrier_eq` against a plane built from the
        // pierced face, and there is no plane to build: a sector face
        // TANGENT to a curved pierced face at `p` is a
        // cosurface/tangency question, and CONTACT-DESIGN C2/C4 forbid
        // inferring either gluing at any ε. The recourse is the same
        // one the undeclared crossing-layer rung names — a verified
        // declaration, under which the `Tangent` branch above already
        // owns the case.
        let Some(plane) = plane else {
            return Err(BooleanError::CurvedBooleanUnsupported {
                operand: pierced_op,
                face: contact.face,
                kind: pierced_kind(pierced_body, contact.face),
            });
        };
        let pierced_carrier = super::carrier_eq::CarrierDesc::Plane {
            origin: plane.origin,
            normal: plane.normal,
        };
        // Oriented sources (S10): both descriptions carry OUTWARD
        // material sides, so rung 1's syntactic Same± verdict has to
        // see the face senses as well as the surfaces' `orient` tags.
        let (g1, g2) = (
            super::reduce::face_oriented_source(piercing_body, s.face),
            super::reduce::face_oriented_source(pierced_body, contact.face),
        );
        let id = super::PlaneIdentity {
            s1: g1.as_ref(),
            s2: g2.as_ref(),
            declared: declared_rest,
        };
        let rel =
            match super::carrier_eq::carrier_eq(&sector_carrier, &pierced_carrier, id, s.arm, band)
            {
                Ok(super::PlaneRelation::Distinct) => {
                    return Err(BooleanError::ClassificationInvariant {
                        what: "geometrically coplanar sector with definitely-distinct plane",
                    });
                }
                Ok(rel) => rel,
                // In band, unreachable here: `bool_sector_coplanar` read
                // this same margin (the two normals' cross, at `s.arm`)
                // zero above for an undeclared pair, and a declared `Rest`
                // pair's rung bridges it. The door is `recl`'s, which
                // reaches it at another arm.
                Err(PlaneEqError::Escalated { rung, diag }) => {
                    return Err(BooleanError::plane_identity(
                        rung,
                        declared.on_pair_door((piercing, s.face, pierced_op, contact.face)),
                        diag,
                    ));
                }
                Err(PlaneEqError::Undeclared { diag, relation }) => {
                    return Err(BooleanError::UndeclaredCoincidence {
                        diag,
                        pair: [(piercing, s.face), (pierced_op, contact.face)],
                        relation,
                    });
                }
                Err(PlaneEqError::Contradicted { fact, .. }) => {
                    return Err(BooleanError::DeclarationContradicted { fact });
                }
            };
        if lump_keeps_one(op, rel) {
            covered.push(match piercing {
                Operand::A => (s.face, contact.face),
                Operand::B => (contact.face, s.face),
            });
        }
        let lump = eq15_3_lump(op, piercing, rel);
        entries[k].class = lump;
        entries[(k + 1) % n].class = lump;
        entries[k].lumped = true;
        entries[(k + 1) % n].lumped = true;
    }

    // On-edge resolution (module docs; the deliberate divergence).
    for k in 0..n {
        if entries[k].class == SideCode::On && entries[(k + 1) % n].class == SideCode::On {
            return Err(BooleanError::ClassificationInvariant {
                what: "consecutive On entries after Eq. 15.3 lumping",
            });
        }
    }
    resolve_on_entries(&mut entries, band)?;

    // Out-runs (the copy takes the OUT side — above ≙ OUT).
    let runs = out_runs(&entries);
    let mut out = VtxFacOut {
        edges: Vec::new(),
        pairs: Vec::new(),
        ring: None,
        covered,
    };
    if runs.is_empty() {
        return Ok(out); // tangential touch: 3′ contact only, no surgery
    }

    // Piercing-side null edges, one per run (PR 2's insertion pattern).
    // Germ facings (F9 data): the run's two boundary transitions are
    // its germs; both lie in the pierced face's TANGENT plane at `p`,
    // so the germ's face pair = (transition sector's face, pierced
    // face) in operand order, and its loci are the transition sector's
    // cell and the pierced face. Parity: the class after crossing forward
    // — Out at the run's start germ, In at its end germ (site-shared
    // with the ring strut below).
    // Every run's germs are read before any run is minted: a mint moves
    // the orbit the germ loci are read from.
    let germ_of = |t: usize| -> Result<Germ<T>, BooleanError> {
        let s = &sectors[t];
        let own = super::sectors::germ_locus(piercing_body, s, (read[(t + 1) % n], read[t]))?;
        let pierced = super::Locus::InFace(contact.face);
        let cells = match piercing {
            Operand::A => ((s.face, contact.face), (own, pierced)),
            Operand::B => ((contact.face, s.face), (pierced, own)),
        };
        Ok((cells, pierce_germ_dir(s, n_pierced.vec(), band)?))
    };
    let run_germs = runs
        .iter()
        .map(|run| {
            Ok((
                germ_of((run.0 + n - 1) % n)?,
                germ_of((run.0 + run.1 - 1) % n)?,
            ))
        })
        .collect::<Result<Vec<_>, BooleanError>>()?;
    let mut run_edges = Vec::new();
    for (run, &(start_germ, end_germ)) in runs.iter().zip(&run_germs) {
        let members = (0..run.1).map(|j| entries[(run.0 + j) % n]);
        let mut real = members.filter(|e| e.is_edge);
        let first = real.next();
        let last = real.next_back().or(first);
        // `strut`: the site is an empty fan, so `mev_null` splices the
        // null edge as a spike [he_plus, he_minus] into one corner.
        let (site, strut) = match (first, last) {
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
                // A run holding every real edge of the orbit leaves the
                // In side strictly inside one physical sector, the one
                // before `first`: `he2` comes back round to `first.he`,
                // and the empty fan is a strut spliced before it.
                (MevSite::Fan { he1: first.he, he2 }, he2 == first.he)
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
        // START germ (forward code Out) is the UP half, starting at
        // `below_end`. A fan puts he_plus (old → copy) at the start
        // germ's cut, so the copy is the above end. A strut's spike
        // faces its start germ with he_minus (copy → old), so the copy
        // is the below end — the mint side follows, keeping the body's
        // scaffold attribute and the record one datum.
        let side = if strut {
            NewVertexSide::Below
        } else {
            NewVertexSide::Above
        };
        let created = piercing_body.mev_null(site, side)?;
        let (start_he, end_he) = if strut {
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
            dangling: strut,
            germs: [half_germ(start_he, start_germ), half_germ(end_he, end_germ)],
        };
        run_edges.push(rec);
        out.edges.push(rec);
    }

    // Delta 3: the pierce ring in the pierced face (module docs).
    let pierced = pierced_op;
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
    for (run_edge, &(start_germ, end_germ)) in run_edges.iter().zip(&run_germs) {
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
                half_germ(created.he_minus, start_germ),
                half_germ(created.he_plus, end_germ),
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

/// On-edge resolution (module docs; the deliberate divergence), after
/// Delta 2's lumps.
///
/// An edge entry's On is a real one for a LINE edge: its far vertex is
/// within the band of the plane (`side_code`, `Reach::Chord`), so the
/// edge lies in it to the tolerance and the resolution is ε-true of it.
/// A curved edge's On is its departure's, to first order
/// (`Reach::Extent`; the residue is
/// `work/contact/boolean-conic-side-code-zero-is-first-order`).
///
/// A bisector entry's On is a direction's, levered at its sector's arm,
/// and it is resolved only where its code changes no topology. Its two
/// neighbours are its physical sector's real bounds. Mixed, the
/// bisector's code only picks which twin of that one sector holds the
/// transition; the fan it moves crosses no edge. Both definitely on one
/// side S (readings, not Delta 2 lumps), the bisector cannot read Zero
/// when K > 2: with `a` the arm, which is the shorter bound's length,
/// and `s` that bound's slope, the bound's own reading gives
/// `s·a ≥ K·zero`. A reflex bisector `−(â + b̂)/|â + b̂|` then reads at
/// least `(s_a + s_b)·a/2 ≥ K·zero/2`. A straight-band one, `n × b̂`,
/// reads at least `a·cos δ` (δ, the deviation from π, has `sin δ·a`
/// inside the band). So a Zero there is the ambiguity band's (K ≤ 2),
/// and the reflex case would read the wrong side: it refuses
/// ([`super::sectors::bisector_zero_refusal`]) rather than resolve.
fn resolve_on_entries(entries: &mut [Entry], band: Band) -> Result<(), BooleanError> {
    let n = entries.len();
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
        let (before, after) = (&entries[(k + n - 1) % n], &entries[(k + 1) % n]);
        let (prev, next) = (before.class, after.class);
        if !entries[k].is_edge && !before.lumped && !after.lumped && prev == next {
            return Err(super::sectors::bisector_zero_refusal(band));
        }
        entries[k].class = super::sectors::fold_on_bound(prev, next);
    }
    Ok(())
}

/// The germ direction at a pierce-site transition: the unit
/// intersection direction of the transition sector's face plane with
/// the pierced face's TANGENT plane at the pierce point, signed to lie
/// within the sector (grazes count — an on-edge germ IS a bound).
/// Ambiguity refuses loudly.
///
/// **Why the tangent plane is the exact datum, not an approximation of
/// one.** A germ direction is the initial direction of the curve along
/// which the two faces meet, taken AT `p`. That curve is the section of
/// the sector's carrier with the pierced carrier, and the tangent of a
/// section conic at a point on it is the intersection LINE of the two
/// surfaces' tangent planes there — a first-order statement about a
/// first-order object, exact for every smooth pair, not a linearization
/// with an error term. A plane is its own tangent plane everywhere, so
/// the planar lane is the special case where `pierced_normal` happens
/// not to vary with `p`; substituting the per-point normal computes the
/// same cross product on a curved carrier and gets the section's
/// tangent rather than a chord.
///
/// Sense-invariant given its sources (S10): the cross product names a
/// LINE and `within` picks the ray, so neither normal's sign survives
/// into the answer. Both arrive oriented already — `s.normal` from
/// `sectors::sector_face`, `pierced_normal` from the one door
/// ([`crate::face_normal`]) — and neither is multiplied again here.
pub(super) fn pierce_germ_dir<T: Decide>(
    s: &super::sectors::BoolSector<T>,
    pierced_normal: geom_core::Vec3<T>,
    band: Band,
) -> Result<geom_core::Vec3<T>, BooleanError> {
    let int = s.normal.vec().cross(pierced_normal);
    // Levered at the sector's farther reach. A transition sector has a
    // bound read definitely off the pierced plane, beyond `K·zero`, at
    // its reach `L`. That reading is at most `L·|n_s × n_p|` plus the
    // departures of the vertices it is taken between from the sector's
    // own plane, each up to `zero` in a valid body, so this gate reads
    // at least `(K − 2)·zero`: never zero, but in band where the reading
    // lies within `2·zero` of the band's edge and the vertices stand off
    // their face by as much. That is the sector's tilt against the
    // pierced plane undecided, which a smaller tolerance decides.
    match decide(
        "bool_germ_line",
        Margin::levered(int.norm(), s.span()),
        band,
    ) {
        Ok(Sign::Positive) => {}
        Ok(_) => {
            return Err(BooleanError::ClassificationInvariant {
                what: "pierce transition on a coplanar sector",
            });
        }
        Err(diag) => {
            return Err(BooleanError::coincidence(
                Coincide::Sectors,
                DeclarationRead::Moot,
                diag,
            ));
        }
    }
    let d = int.normalize();
    let moot = DeclarationRead::Moot;
    let plus = super::sectors::within(s, d, false, moot, band)?;
    let minus = super::sectors::within(s, -d, false, moot, band)?;
    match (plus, minus) {
        (true, false) => Ok(d),
        (false, true) => Ok(-d),
        _ => Err(BooleanError::ClassificationInvariant {
            what: "pierce germ direction not uniquely within its sector",
        }),
    }
}

/// The surface kind a refusal about `face` cites. A face whose surface
/// cannot be read at all is reported as the kind with no arm anywhere,
/// which is what the sibling refusal sites in this module do.
fn pierced_kind<T: Decide>(body: &Body<T>, face: crate::entity::FaceKey) -> geom::SurfaceKind {
    body.get_face(face)
        .and_then(|f| body.get_surface(f.surface))
        .map_or(geom::SurfaceKind::Nurbs, geom::Surface::kind)
}

/// A pierce germ as a run reads it: `(A face, B face)` and
/// `(A locus, B locus)`, then its direction.
type Germ<T> = (
    (
        (crate::entity::FaceKey, crate::entity::FaceKey),
        (super::Locus, super::Locus),
    ),
    geom_core::Vec3<T>,
);

fn half_germ<T: geom_core::Real>(
    he: HalfEdgeKey,
    ((faces, loci), dir): Germ<T>,
) -> super::HalfGerm<T> {
    super::HalfGerm {
        he,
        a_face: faces.0,
        b_face: faces.1,
        a_locus: loci.0,
        b_locus: loci.1,
        dir,
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

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use SideCode::{In, On, Out};

    fn entry(is_edge: bool, class: SideCode) -> Entry {
        Entry {
            he: HalfEdgeKey::default(),
            is_edge,
            class,
            lumped: false,
        }
    }

    /// A bisector reading On between two bounds read Out is the band's
    /// Zero (reachable only at K ≤ 2, which no suite runs), and it
    /// refuses; an edge's On there, a real one, resolves; a bisector
    /// between mixed neighbours resolves, since its code moves no edge.
    #[test]
    fn a_bisector_on_between_one_sided_bounds_refuses() {
        let band = Band::linear(Tol::witness()).unwrap();
        let mut one_sided = [entry(true, Out), entry(false, On), entry(true, Out)];
        assert!(
            matches!(
                resolve_on_entries(&mut one_sided, band),
                Err(BooleanError::Escalated {
                    decision: super::super::BooleanDecision::BisectorSide,
                    diag,
                }) if diag.predicate == Some("bool_sector_bisector_side")
            ),
            "the bisector's Zero refuses as its own decision"
        );
        let mut edge = [entry(true, Out), entry(true, On), entry(true, Out)];
        resolve_on_entries(&mut edge, band).unwrap();
        assert_eq!(edge[1].class, Out, "an edge's On resolves");
        let mut mixed = [entry(true, Out), entry(false, On), entry(true, In)];
        resolve_on_entries(&mut mixed, band).unwrap();
        assert_eq!(
            mixed[1].class, In,
            "a bisector between mixed bounds resolves"
        );
    }

    /// **An in-band pierce germ line is the sector's tilt undecided**,
    /// not the kernel's: a transition sector reads it at least
    /// `(K − 2)·zero` (its vertices may stand off their face by up to
    /// the band), so a sector whose face parts from the pierced face by
    /// an in-band angle at its reach escalates as the corners' overlap,
    /// with its lever and the tolerance its margin gives.
    #[test]
    fn an_in_band_pierce_germ_line_is_the_sectors_tilt_undecided() {
        use super::super::sectors::{BoolSector, Reach};
        use geom_brep::OutwardNormal;
        use geom_core::{KERNEL_DEFECT_ENDING, Point3, Vec3};
        let band = Band::linear(Tol::witness()).unwrap();
        let mid = (band.zero() + band.escalate()) / 2.0;
        let o = Point3::new(0.0, 0.0, 0.0);
        let (x, y) = (Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
        let s = BoolSector {
            he: HalfEdgeKey::default(),
            start: x,
            end: y,
            start_reach: Reach::Chord {
                base: o,
                far: o + x,
            },
            end_reach: Reach::Chord {
                base: o,
                far: o + y,
            },
            face: crate::entity::FaceKey::default(),
            normal: OutwardNormal::from_chart(Vec3::new(0.0, 0.0, 1.0), true),
            arm: 1.0,
        };
        let err =
            pierce_germ_dir(&s, Vec3::new(mid.sin(), 0.0, mid.cos()), band).expect_err("in band");
        assert!(
            matches!(
                err,
                BooleanError::Escalated {
                    decision: super::super::BooleanDecision::Coincidence(
                        Coincide::Sectors,
                        DeclarationRead::Moot
                    ),
                    ..
                }
            ),
            "{err:?}"
        );
        let text = err.to_string();
        assert!(
            !text.contains(KERNEL_DEFECT_ENDING) && text.contains("tighten the tolerance below"),
            "{text}"
        );
    }
}
