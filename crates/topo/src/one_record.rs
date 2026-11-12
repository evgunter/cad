//! **A coincidence verdict's token and the one act that consumes it**
//! (DESIGN `[coincidence.door]`, its mechanics in this crate's README
//! `[topo.one-record]`).
//!
//! A verdict that faces lie on one carrier ends with them on one
//! surface record. The deciding reading mints a [`OneRecord`]: the
//! faces it moves onto the kept record ([`moved_closure`]), every one of
//! them read over by the verdict; the reading records its own row. [`Body::one_record`]
//! is the one act that consumes a token; no caller hands it a record.
//! A verdict decided from values reaches only the faces it was read
//! over, so a face the token does not move keeps its record, whatever
//! record it wears.
//!
//! Which record is kept is a census of keys ([`kept_of`]): the one
//! alone also worn by faces outside the faces the verdicts leave on one
//! record, else the first-written side's.

use std::collections::BTreeSet;

use geom_core::{Decide, Real, Tol};

use crate::SurfaceKey;
use crate::body::Body;
use crate::entity::FaceKey;
use crate::euler::EulerOpError;

/// **One value-decided coincidence verdict, ready to be acted on**: the
/// faces it moves onto `kept` from its other records.
/// Only a deciding reading mints one ([`Self::minted`]).
#[derive(Clone, Debug)]
pub(crate) struct OneRecord {
    /// The record the moved faces are left on.
    pub(crate) kept: SurfaceKey,
    /// The records they leave, each with whether its chart faces as the
    /// kept one's does: a moved face's sense is restated where it does
    /// not, so its material side is unchanged.
    pub(crate) others: Vec<(SurfaceKey, bool)>,
    /// The faces the verdict was read over that wear an other record: each
    /// decided face on it, every fragment of one, and their closure
    /// under same-record adjacency ([`moved_closure`]).
    pub(crate) moved: BTreeSet<FaceKey>,
}

impl OneRecord {
    /// The token a deciding reading mints: `moved`, read over with the
    /// decided faces, onto `kept` from `other`.
    pub(crate) fn minted(
        (kept, other): (SurfaceKey, SurfaceKey),
        charts_agree: bool,
        moved: BTreeSet<FaceKey>,
    ) -> Self {
        Self {
            kept,
            others: vec![(other, charts_agree)],
            moved,
        }
    }

    /// The record the moved faces are left on.
    pub(crate) fn kept(&self) -> SurfaceKey {
        self.kept
    }
}

/// **Whether two faces of one carrier have charts facing one way**,
/// given whether their material sides agree and each face's sense, which
/// turns its material side to its chart's normal: they face one way
/// where the materials agree and the senses do, or where neither does.
/// The one spelling every act's token takes. Reads no value.
pub(crate) fn charts_agree(materials_agree: bool, [a, b]: [bool; 2]) -> bool {
    materials_agree == (a == b)
}

/// **Whether `face`'s record is also worn by a face of `body` outside
/// `decided`**, the faces the verdicts leave on one record: the census
/// [`kept_of`] reads on each side. Keys only.
pub(crate) fn worn_outside<T: Real>(
    body: &Body<T>,
    face: FaceKey,
    decided: &BTreeSet<FaceKey>,
) -> bool {
    let Some(record) = body.get_face(face).map(|f| f.surface) else {
        return false;
    };
    body.faces()
        .any(|(f, g)| g.surface == record && f != face && !decided.contains(&f))
}

/// **Which side's record a verdict keeps**, the one rule every act's
/// census runs: the side whose record alone is also worn by faces
/// outside those the verdicts leave on one record, else `first`, the
/// first-written side. `worn` is [`worn_outside`] per side, two for a
/// pair, one per record for a merge's run. A key census: it reads no
/// value, so it cannot choose by rounding between two descriptions in
/// band of each other.
pub(crate) fn kept_of<S: Copy>(worn: &[(S, bool)], first: S) -> S {
    let mut outside = worn.iter().filter(|&&(_, w)| w);
    match (outside.next(), outside.next()) {
        (Some(&(side, _)), None) => side,
        _ => first,
    }
}

/// **The faces a verdict moves, from those it decided on the moving
/// side**: `decided`, and every face joined to one of these by an edge
/// across which the two wear one record, to closure. The closure stays
/// on the record each starts from and never follows an edge both of
/// whose sides are one face (a seam, a strut). Keys only.
///
/// # Panics
///
/// Where a face's loop does not walk: a link, which every door keeps
/// live.
pub(crate) fn moved_closure<T: Real>(body: &Body<T>, decided: &[FaceKey]) -> BTreeSet<FaceKey> {
    let mut out: BTreeSet<FaceKey> = decided
        .iter()
        .copied()
        .filter(|&f| body.get_face(f).is_some())
        .collect();
    let mut pending: Vec<FaceKey> = out.iter().copied().collect();
    while let Some(f) = pending.pop() {
        let key = body.proven_face(f).record().surface;
        for he in body.proven_face(f).half_edges() {
            let Some(mate) = body.mate(he).and_then(|m| body.face_of_half_edge(m)) else {
                continue;
            };
            if mate == f || out.contains(&mate) {
                continue;
            }
            if body.get_face(mate).is_some_and(|m| m.surface == key) {
                out.insert(mate);
                pending.push(mate);
            }
        }
    }
    out
}

impl<T: Decide + crate::props::AtRestPolicy> Body<T> {
    /// **Leaves the faces a verdict decided one carrier on one record**
    /// (module docs): each face of `token`'s moved set that still wears
    /// its other record is re-pointed onto the kept one through the
    /// describing re-chart ([`crate::attach::Rechart::shared`]), its
    /// sense restated so its material side is unchanged. The edges the
    /// move strands are restated: a description naming the other record
    /// names the kept one ([`Body::carried_redescriptions`]), and one
    /// naming the pair of them between two faces now both on the kept
    /// record rests in its chart. Every moved face's boundary is
    /// certified on the kept chart at the ordinary write gate. A face
    /// the token does not move keeps its record. Reads no value.
    ///
    /// # Errors
    ///
    /// The re-chart door's
    /// ([`Body::set_face_surfaces_describing`](crate::construction::Construct::set_face_surfaces_describing)).
    /// After a verdict decided Zero over every face it moves, a boundary
    /// that does not certify on the kept chart is a kernel defect, which
    /// the caller types as one.
    pub(crate) fn one_record(
        &mut self,
        site: crate::policy_lane::KernelSite,
        token: &OneRecord,
        tol: Tol,
    ) -> Result<(), EulerOpError> {
        self.recharted_onto(site, token, tol)
    }
}

/// **What a sweep's reading of an edge's wedge decided across it**
/// ([`Body::dihedral_across`]).
#[derive(Debug)]
pub struct DihedralAcross<T: Real> {
    /// What the wedge demands the edge store
    /// ([`crate::edge_wedge_description`]).
    pub description: geom_brep::MustCarryDescription<T>,
    /// Two planes meeting at the π seam are one plane: the verdict that
    /// the two faces are one carrier. `None` for every other wedge and
    /// every other pair of kinds.
    pub one_carrier: Option<SmoothAcross>,
}

/// **The verdict that the two faces either side of an edge are one
/// carrier**, as the wedge reading across it decided two planes' join
/// the π seam.
/// Only that reading constructs one ([`Body::dihedral_across`]), and only
/// the act consumes it ([`Body::one_record_across`]): no caller states a
/// coincidence from an edge and a face.
///
/// ```compile_fail,E0451
/// let _ = topo::SmoothAcross {
///     edge: Default::default(),
///     faces: Default::default(),
///     margin: unimplemented!(),
/// };
/// ```
#[derive(Debug)]
#[must_use]
pub struct SmoothAcross {
    edge: crate::entity::EdgeKey,
    faces: [FaceKey; 2],
    margin: geom_core::MarginDiag,
}

impl SmoothAcross {
    /// The margin the reading decided the join smooth on.
    #[must_use]
    pub fn margin(&self) -> geom_core::MarginDiag {
        self.margin
    }
}

impl<T: Decide> Body<T> {
    /// **The wedge across `edge` between its faces' records
    /// `surfaces`**: the whole-edge fold over its walk, levered at its
    /// faces' extent, and what it demands the edge store
    /// ([`crate::edge_wedge_description`]), the reading a sweep takes at
    /// an edge it wrote. Two planes at the π seam come back with the
    /// verdict that its two faces are one carrier ([`SmoothAcross`]), on
    /// the wedge decision's margin at the carrier's middle: two planes'
    /// normals are constant along their line, so every station decides
    /// on that margin.
    ///
    /// # Errors
    ///
    /// [`BadArgument::Stale`](crate::euler::BadArgument::Stale) where
    /// `edge` does not resolve, carries no certified curve, or
    /// `surfaces` are not its two faces' records; the inner `Err` is the
    /// wedge's refusal.
    pub fn dihedral_across(
        &self,
        edge: crate::entity::EdgeKey,
        surfaces: [SurfaceKey; 2],
        band: geom_core::Band,
    ) -> Result<Result<DihedralAcross<T>, geom_brep::PairRefusal>, EulerOpError> {
        let curve = self.stored_curve(edge)?;
        let stale = || {
            EulerOpError::Argument(crate::euler::BadArgument::Stale {
                role: "surfaces",
                key: crate::entity::EntityId::Edge(edge),
            })
        };
        let sides = crate::wedge_reach::edge_faces(self, edge);
        let wearing = |s: SurfaceKey| {
            sides
                .into_iter()
                .find(|&f| self.get_face(f).is_some_and(|face| face.surface == s))
        };
        let (Some(f1), Some(f2)) = (wearing(surfaces[0]), wearing(surfaces[1])) else {
            return Err(stale());
        };
        if f1 == f2 {
            return Err(stale());
        }
        let surface = |s: SurfaceKey| self.get_surface(s).ok_or_else(stale);
        let (s1, s2) = (surface(surfaces[0])?, surface(surfaces[1])?);
        let verdict = crate::edge_wedge(self, edge, band).ok_or_else(stale)?;
        let description = crate::edge_wedge::demanded(self, edge, verdict).ok_or_else(stale)?;
        let description = match description {
            Ok(description) => description,
            Err(refusal) => return Ok(Err(refusal)),
        };
        let planes = matches!(
            (s1, s2),
            (geom::Surface::Plane { .. }, geom::Surface::Plane { .. })
        );
        let one_carrier = if planes && matches!(verdict, geom_brep::WedgeVerdict::Seam { .. }) {
            let reach = crate::wedge_reach::wedge_reach(self, sides, curve.carrier(), curve.params());
            match geom_brep::classify_dihedral_reported(s1, s2, curve.mid_point(), reach, band) {
                Ok((_, margin)) => Some(SmoothAcross {
                    edge,
                    faces: [f1, f2],
                    margin,
                }),
                Err(escalation) => return Ok(Err(geom_brep::PairRefusal::InBand(escalation))),
            }
        } else {
            None
        };
        Ok(Ok(DihedralAcross {
            description,
            one_carrier,
        }))
    }
}

impl<T: Decide + crate::props::AtRestPolicy> Body<T> {
    /// **The two faces a smooth join was decided across, left on one
    /// record**: the act ([`Body::one_record`]) on the token `verdict`
    /// mints, and the row it records. The kept record is the one alone
    /// also worn by faces outside the two ([`kept_of`]), else `first`'s,
    /// the face the sweep wrote first; the moved face's closure under
    /// same-record adjacency moves with it ([`moved_closure`]). A smooth
    /// join that is no slit faces one way across the edge, so the two
    /// faces' material sides agree.
    ///
    /// Returns the row, its cells the two faces, `first`'s first; `None`,
    /// and nothing written, where they already wear one record.
    ///
    /// # Errors
    ///
    /// [`BadArgument::Stale`](crate::euler::BadArgument::Stale) where the
    /// verdict's edge no longer joins its two faces or `first` is not one
    /// of them, and the re-chart's ([`Body::one_record`]).
    pub fn one_record_across(
        &mut self,
        verdict: SmoothAcross,
        first: FaceKey,
        tol: Tol,
    ) -> Result<Option<crate::Coincidence>, EulerOpError> {
        let SmoothAcross {
            edge,
            faces,
            margin,
        } = verdict;
        crate::live::lookup(
            &self.edges,
            edge,
            crate::entity::EntityId::Edge,
            crate::live::Arg("edge"),
        )?;
        let sides = crate::wedge_reach::edge_faces(self, edge);
        let stale = |role, face| {
            EulerOpError::Argument(crate::euler::BadArgument::Stale {
                role,
                key: crate::entity::EntityId::Face(face),
            })
        };
        if let Some(&f) = faces.iter().find(|f| !sides.contains(f)) {
            return Err(stale("verdict", f));
        }
        let second = match faces {
            [a, b] if a == first => b,
            [a, b] if b == first => a,
            _ => return Err(stale("first", first)),
        };
        let (k1, k2) = (
            self.proven_face(first).record().surface,
            self.proven_face(second).record().surface,
        );
        if k1 == k2 {
            return Ok(None);
        }
        let decided: BTreeSet<FaceKey> = [first, second].into();
        let kept = kept_of(
            &[
                (first, worn_outside(self, first, &decided)),
                (second, worn_outside(self, second, &decided)),
            ],
            first,
        );
        let moving = if kept == first { second } else { first };
        let (kept_key, other_key) = (
            self.proven_face(kept).record().surface,
            self.proven_face(moving).record().surface,
        );
        let charts_agree = charts_agree(
            true,
            [
                self.proven_face(kept).record().sense,
                self.proven_face(moving).record().sense,
            ],
        );
        let token = OneRecord::minted(
            (kept_key, other_key),
            charts_agree,
            moved_closure(self, &[moving]),
        );
        self.one_record(crate::policy_lane::KernelSite::OneRecord, &token, tol)?;
        Ok(Some(crate::Coincidence {
            cells: [first, second].map(crate::census::result_face),
            more: [None; 3],
            relation: crate::Relation::SameOriented,
            site: crate::DecisionSite::SmoothJoin,
            margin,
            discharge: crate::Discharge::of(margin),
        }))
    }
}

impl<T: Decide + crate::props::AtRestPolicy> Body<T> {
    /// **A merge's run of faces its margins decided one carrier, left on
    /// one record without the surgery** (a curved run the merge records
    /// as skipped): the act ([`Body::one_record`]) on the token the
    /// run's verdicts mint, moving every record the run wears besides the
    /// kept one in one move. The kept record is the one alone also worn by faces outside
    /// the run ([`kept_of`]'s rule), else its first face's; each other
    /// record's faces in the run move with their closure under
    /// same-record adjacency ([`moved_closure`]). A run faces one way
    /// across every edge it was decided over, so each record's charts
    /// face as its faces' senses say.
    ///
    /// # Errors
    ///
    /// The re-chart's ([`Body::one_record`]).
    pub(crate) fn one_record_of_run(
        &mut self,
        run: &[FaceKey],
        tol: Tol,
    ) -> Result<(), EulerOpError> {
        let record = |body: &Self, f: FaceKey| {
            let face = body.proven_face(f).record();
            (face.surface, face.sense)
        };
        // Each record the run wears, with its first face in the run.
        let mut records: Vec<((SurfaceKey, bool), FaceKey)> = Vec::new();
        for &f in run {
            let r = record(self, f);
            if !records.iter().any(|&((k, _), _)| k == r.0) {
                records.push((r, f));
            }
        }
        let [(first, _), ..] = records[..] else {
            return Ok(());
        };
        if records.len() == 1 {
            return Ok(());
        }
        let decided: BTreeSet<FaceKey> = run.iter().copied().collect();
        let worn: Vec<((SurfaceKey, bool), bool)> = records
            .iter()
            .map(|&(r, f)| (r, worn_outside(self, f, &decided)))
            .collect();
        let kept = kept_of(&worn, first);
        let records: Vec<(SurfaceKey, bool)> = records.iter().map(|&(r, _)| r).collect();
        let others: Vec<(SurfaceKey, bool)> = records
            .iter()
            .filter(|&&(k, _)| k != kept.0)
            .map(|&(k, sense)| (k, charts_agree(true, [sense, kept.1])))
            .collect();
        let on: Vec<FaceKey> = run
            .iter()
            .copied()
            .filter(|&f| record(self, f).0 != kept.0)
            .collect();
        let token = OneRecord {
            kept: kept.0,
            others,
            moved: moved_closure(self, &on),
        };
        self.one_record(crate::policy_lane::KernelSite::OneRecord, &token, tol)?;
        Ok(())
    }
}
