//! Geometry attachment setters: certified re-attachment of face
//! surfaces and edge curves on an existing body.
//!
//! The Euler operators attach geometry at mint time (op parameters —
//! the preferred path); these setters cover the attachments that
//! **cannot** happen at mint time, consequences of construction order
//! or of a door that moves a chart:
//!
//! - [`Body::set_face_surface`] — `mvfs`'s seed face is born before its
//!   eventual cap surface exists (the seed's surface is the honest
//!   `Surface::Nurbs` "not yet described" state); the sweep attaches
//!   the real plane once profile data determines it. It is keys-only:
//!   a swap that would leave an edge described against the surface the
//!   face leaves is refused, since a description names its surfaces by
//!   key, and so is one onto another chart that a certified edge of the
//!   face does not name, since only a certificate on the new chart
//!   vouches that the boundary lies on it.
//! - [`Body::set_face_surfaces_describing`] — its describing sibling, as
//!   [`Body::kev_describing`] is [`Body::kev`]'s: it moves faces onto
//!   new or shared charts together with the re-descriptions of the
//!   edges the move would strand, under a band, and re-describes
//!   nothing it is not handed ([`Body::carried_redescriptions`] states
//!   the stored ones on the moved charts). The offset doors and the
//!   split finish, which move charts under described edges, take it.
//! - [`Body::set_edge_curve`] — an intrinsic
//!   ([`geom_brep::EdgeDescription::Intersection`]) description references
//!   its two adjacent faces' surfaces *by key*, and a swept edge is
//!   necessarily minted **before** the side faces it will bound
//!   (struts precede `mef`): the sweep mints with a conventional
//!   description and upgrades to the intrinsic one here, once both
//!   surfaces exist (the prefer-intrinsic rule lands at rest, D2).
//!
//! The setters are **certified mutations**: the same D4 ¶2/¶3 gates as
//! the operators (`EdgeCurve::certify`; typed errors, body untouched on
//! failure), plus — since both adjacent faces exist by the time an
//! edge is re-described — the **description-adjacency coherence** check
//! the mint-time gate cannot run: an `Intersection`'s two surfaces must
//! be exactly the edge's two faces' surfaces, and a `Seam`'s surface
//! must be on both sides ([`EulerOpError::DescriptionNotAdjacent`]).
//! No setter is an Euler operator (no topology changes — the D1
//! "exclusively Euler" rule governs *topology*); each preserves tier 1
//! (geometry arenas stay reference-coherent through the orphan-hygiene
//! paths) and carry the tier-1 debug postcondition on the same terms
//! as the operators: **it is re-derived once per public door**, so a
//! setter a consumer calls directly re-certifies the whole body and
//! one called inside a composing door's surgery scope
//! ([`crate::surgery`]) leaves the sweep to that door's close (Ev's
//! ruling on `work/perf/d1-per-op-tier1-sweep-price`, PR 2305). A
//! surface swap can orphan a key and a door that replaces a whole
//! chart makes one such swap per face, which is the case the rule is
//! about: the check is the same check, taken once over the finished
//! state instead of once per write.
//!
//! Replacement is by **fresh insertion** (new key, old removed iff
//! orphaned): overwriting in place could silently retarget another
//! referent of a shared slot, while key churn is harmless — replay
//! determinism (D9) is about identical histories minting identical
//! keys, and a setter call is part of the history.

use geom::Surface;
use geom_brep::{CertifyError, EdgeCurve, EdgeCurveSpec};
use geom_core::k_stats::decide;
use geom_core::{Band, Decide, Margin, Point3, Real, Sign};

use crate::body::Body;
use crate::entity::{EdgeKey, EntityId, FaceKey, GeomRef, HalfEdgeKey, LoopBoundary, LoopKey};
use crate::euler::{EulerOpError, FaceSurface, ParentSide, RechartDoor};
use crate::geometry::{CurveKey, SurfaceKey};
use crate::pcurves::{SiteHalf, SiteRows};
use geom_core::Tol;

impl<T: Decide> Body<T> {
    /// Replaces `face`'s surface per the [`FaceSurface`] spec,
    /// returning the face's (possibly unchanged) surface key.
    /// `Inherit` is a no-op (the current surface); `New` mints; `Shared`
    /// reuses an existing key. The old surface is removed iff nothing
    /// else references it (faces or edge descriptions).
    ///
    /// **An edge described against the surface the face leaves is
    /// refused, never stranded.** A description names its surfaces by
    /// key — an `Intersection`'s operands, a `Chart`'s chart — so a
    /// swap leaves every description that named the face's old key
    /// naming a surface the face no longer wears. Where such a
    /// description is adjacency-coherent now and the swap would make it
    /// not (tier 3's `DescriptionNotAdjacent` at rest), the door
    /// refuses before mutating, naming every such edge. That question
    /// is exact key adjacency, so the door takes no band;
    /// [`Body::set_face_surfaces_describing`] takes a band and the
    /// re-descriptions.
    ///
    /// **A moved face's boundary is vouched for by its edges'
    /// certificates, or the swap is refused.** A certified edge is
    /// certified on the keys its description names: its samples lie on
    /// each within the band, and its ends on its vertices. So where the
    /// face moves onto another chart, every certified edge on it must
    /// name the key the face wears after the swap; the door refuses
    /// before mutating, naming every one that does not
    /// ([`EulerOpError::RechartUnvouched`]), on every chart kind. A
    /// `New` key is one no description names. A move onto
    /// [`Body::same_chart`]'s one payload is not asked, since a
    /// certificate is a function of the payload it was taken on.
    ///
    /// What the check does not decide: scaffold and null edges carry no
    /// certificate and are not asked; nor is an empty loop's lone
    /// vertex, since tier 2 bans an empty loop at rest.
    /// [`Body::set_face_surfaces_describing`] certifies the
    /// re-descriptions it is handed on every chart, and the boundary's
    /// own residuals only on a plane; a curved chart's containment is
    /// asked by neither door, nor by tier 3 at rest
    /// (`work/restfront/validate-tier3-curved-boundary-containment`,
    /// #638).
    ///
    /// **The face's pcurve rows are not a cache this door may keep.** A
    /// row is a curve stated in a face's CHART
    /// ([`crate::pcurves`]), so re-charting the face in place makes
    /// every one of its rows a statement about a surface the face is no
    /// longer on — the loop-re-parenting doors' defect with the two
    /// sides swapped, and it takes their answer: a swap onto the same
    /// chart carries every row untouched, and a swap onto a different
    /// one drops the face's rows ([`Body::drop_face_rows`]), deriving
    /// nothing. A caller that wants the face's rows on its new chart
    /// runs [`crate::pcurves::mint_pcurves`]. Leaving them was silent
    /// wherever the new surface does not mint — tier 3's pcurve pass
    /// skips such a face — so what the drop removes is a wrong row no
    /// reader could be warned about.
    ///
    /// **Sense** ([`crate::Face::sense`]): the face stands as its own
    /// parent and the door passes [`ParentSide::With`], so the bit is
    /// kept on the face's own chart and stated on any other
    /// ([`Body::resolve_face_surface`]).
    ///
    /// # Errors
    ///
    /// [`EulerOpError::StaleKey`] if `face` does not resolve;
    /// [`EulerOpError::StaleGeometry`] if a `Shared` key does not
    /// resolve; [`EulerOpError::SenseContradictsChart`] if a spec on
    /// the face's own chart states the other bit; then
    /// [`EulerOpError::RechartStrandsDescriptions`], then
    /// [`EulerOpError::RechartUnvouched`] (`StaleKey` / `StaleGeometry`
    /// where a key a walk over the edges follows does not resolve). The
    /// body is untouched on `Err`.
    pub fn set_face_surface(
        &mut self,
        face: FaceKey,
        surface: FaceSurface<T>,
    ) -> Result<SurfaceKey, EulerOpError> {
        let face_data = self.get_face(face).ok_or(EulerOpError::StaleKey {
            key: EntityId::Face(face),
        })?;
        let old = face_data.surface;
        let resolved =
            self.resolve_face_surface(&surface, face, (old, face_data.sense), ParentSide::With)?;
        self.vouch_move(
            RechartDoor::SetFaceSurface,
            face,
            (old, Slot::of_spec(&surface, old)),
            self.edges.keys(),
            |_, _, f| f == face,
            resolved.on_parent_chart,
            None,
        )?;

        // ---- Mutation (infallible from here on). ----
        let new = self.mint_face_surface(surface, old);
        let Some(f) = self.get_face_mut(face) else {
            unreachable!(
                "set_face_surface: `face` resolved in the plan phase and minting a \
                 surface kills no face"
            )
        };
        f.sense = resolved.sense;
        if new != old {
            f.surface = new;
            if !resolved.on_parent_chart {
                self.drop_face_rows(face);
            }
            self.remove_surface_if_orphaned(old);
        }

        #[cfg(debug_assertions)]
        self.assert_tier1_postcondition("set_face_surface");
        Ok(new)
    }

    /// **Failure-injection door** (test builds only: this crate's own
    /// tests, `test-support` and `sweep-testing`):
    /// [`Body::set_face_surface`] with its refusals of a swap it cannot
    /// vouch for taken out, so a swap may leave edges described against
    /// the surface the face leaves — the state tier 3 reports at rest as
    /// `DescriptionNotAdjacent` — or move a face onto a chart no
    /// certified edge of it names. Every other precondition and every
    /// write is the real door's.
    ///
    /// It is for a row that builds such a body on purpose, and only
    /// there: a swap [`Body::set_face_surfaces_describing`] takes goes
    /// through it, with
    /// [`Body::carried_redescriptions`] where the stored descriptions
    /// are what the row wants. Every call carries a one-line `// Lifts`
    /// comment naming the refusal it takes out and why that state is
    /// the row's premise; the reasons live there, beside the code they
    /// govern, and nowhere else.
    ///
    /// # Errors
    ///
    /// [`Body::set_face_surface`]'s, but for
    /// [`EulerOpError::RechartStrandsDescriptions`] and
    /// [`EulerOpError::RechartUnvouched`].
    #[cfg(any(test, feature = "test-support", feature = "sweep-testing"))]
    #[doc(hidden)]
    pub fn set_face_surface_stranding_for_tests(
        &mut self,
        face: FaceKey,
        surface: FaceSurface<T>,
    ) -> Result<SurfaceKey, EulerOpError> {
        self.lifting_rechart_refusals_for_tests(|body| body.set_face_surface(face, surface))
    }

    /// Re-charts faces with the re-descriptions of the edges the move
    /// would strand: [`Body::set_face_surface`]'s swap for every face of
    /// every [`Rechart`], taking a band and, for each edge the swap
    /// would strand, the spec that describes it on the moved charts.
    /// Returns each chart's key, in `charts` order: the minted key of a
    /// [`Rechart::new`] chart, the given one of a [`Rechart::shared`].
    ///
    /// The describing sibling of the keys-only surface setter, as
    /// [`Body::kev_describing`] is [`Body::kev`]'s. It takes every chart
    /// that door takes — a fresh surface ([`FaceSurface::New`]) or one
    /// the body holds ([`FaceSurface::Shared`]) — so every swap that
    /// door refuses has a way through here. Faces, descriptions and the
    /// band arrive together, and nothing is written until everything
    /// has certified.
    ///
    /// It moves several charts in one call because an edge between two
    /// moving charts certifies on neither pair of mixed charts. Two
    /// single-chart calls reach the same body only by describing such
    /// an edge as a scaffold in the first, and that intermediate is a
    /// state tier 3 refuses at rest (`ScaffoldAtRest`); one call never
    /// shows it.
    ///
    /// **Nothing is re-described by default.** An edge the move would
    /// strand — [`Body::set_face_surface`]'s question, asked of every
    /// moved face — is refused unless it is listed
    /// ([`EulerOpError::RechartUndescribed`]), however its stored
    /// description would read on the moved charts.
    /// [`Body::carried_redescriptions`] states those stored descriptions
    /// on the moved charts, for a caller that chooses them.
    ///
    /// **A key an edge's face wore stands, in that edge's listed spec,
    /// for the chart the face moves onto** — unless one of the edge's
    /// faces still wears it after the move, when it stands for itself.
    /// A new chart has no key before it is minted, so a spec names the
    /// chart it is stated in by the key the face wears now; where both
    /// faces wore the key and move apart, the `he_plus` side's chart is
    /// the one named. A listed spec is certified against its current
    /// endpoints and the moved charts, adjacency coherence against its
    /// faces' moved charts included, and its curve is replaced by fresh
    /// insertion. Any certified edge may be listed; a null edge may not
    /// (its first description is [`Body::set_edge_curve`]'s, which mints
    /// its rows).
    ///
    /// **What it certifies:** every listed description, on every chart
    /// kind; and, where a moved face's new chart is a plane and not its
    /// old one, the boundary's residuals — every vertex of its loops,
    /// an empty loop's lone vertex included, and every interior
    /// certification sample of each edge on them (the listed curve, or
    /// the stored one) is within the band of the plane. These are tier
    /// 3's planar residual checks, asked before the move. A curved
    /// chart's containment is not asked, here or by tier 3 at rest
    /// (`work/restfront/validate-tier3-curved-boundary-containment`,
    /// #638): handed no re-descriptions, a move onto a curved chart is
    /// taken unchecked. The keys-only door leaves the lone vertex
    /// unasked, since tier 2 bans an empty loop at rest; this one asks
    /// it because it has the band to.
    ///
    /// Each face's sense is [`FaceSurface`]'s rule, per face; its
    /// pcurve rows are [`Body::set_face_surface`]'s (kept on the same
    /// chart, dropped on another); a re-described edge's rows are
    /// [`Body::set_edge_curve`]'s.
    ///
    /// Minting order (D9): the new charts' surfaces in `charts` order,
    /// then the listed edges' curves in list order.
    ///
    /// # Precondition check order
    ///
    /// `tol` builds a band ([`EulerOpError::Certification`]). Per chart
    /// in order, per face in order: the face resolves
    /// ([`EulerOpError::StaleKey`]), was not listed before
    /// ([`EulerOpError::FaceMovedTwice`]), a shared chart's key resolves
    /// ([`EulerOpError::StaleGeometry`]) and the face states a sense its
    /// chart admits ([`EulerOpError::SenseContradictsChart`]). Per
    /// listed edge in order: it resolves (`StaleKey`), was not listed
    /// before ([`EulerOpError::DuplicateRedescription`]), is not a null
    /// edge ([`EulerOpError::NullScaffoldCurve`]), its spec is
    /// adjacency-coherent on the moved charts
    /// ([`EulerOpError::DescriptionNotAdjacent`]) and certifies at its
    /// endpoints ([`EulerOpError::RechartFalsifies`]; the plane × NURBS
    /// lane is the scalar's policy, [`crate::AtRestPolicy::nurbs_lane`],
    /// and a scalar holding none refuses that class
    /// [`EulerOpError::NurbsLaneUnsupported`]). Then no unlisted
    /// edge is stranded ([`EulerOpError::RechartUndescribed`], every one
    /// named). Then per moved face in order onto a plane that is not its
    /// old chart, per loop (outer, then rings) and half-edge in cycle
    /// order: its start vertex, then its edge's interior samples, lie on
    /// the plane, as does an empty loop's lone vertex
    /// ([`EulerOpError::RechartOffBoundary`] /
    /// [`EulerOpError::RechartBoundaryEscalated`]). `StaleKey` /
    /// `StaleGeometry` where a key a walk follows does not resolve, and
    /// [`EulerOpError::LoopCycleBroken`] where a moved face's loop does
    /// not walk.
    ///
    /// # Errors
    ///
    /// The first failing precondition above; the body is untouched on
    /// `Err`.
    pub fn set_face_surfaces_describing(
        &mut self,
        charts: Vec<Rechart<T>>,
        redescriptions: &[(EdgeKey, EdgeCurveSpec<T>)],
        tol: Tol,
    ) -> Result<Vec<SurfaceKey>, EulerOpError>
    where
        T: crate::props::AtRestPolicy,
    {
        let band = Band::linear(tol).map_err(|e| EulerOpError::Certification {
            error: CertifyError::Band(e),
        })?;
        let faces = self.plan_recharts(&charts)?;
        let moved = |_: HalfEdgeKey, _: LoopKey, f: FaceKey| moved_slot(&faces, f);
        let (body, planned) = (&*self, &charts);
        let resolve = |sides: Sides| {
            move |k: SurfaceKey| body.slot_surface(planned, sides.repoint(k)).cloned()
        };

        // ---- The listed edges. ----
        let mut written: Vec<(EdgeKey, Sides, EdgeCurve<T>)> =
            Vec::with_capacity(redescriptions.len());
        for (edge, spec) in redescriptions {
            let edge = *edge;
            let edge_data = self.get_edge(edge).ok_or(EulerOpError::StaleKey {
                key: EntityId::Edge(edge),
            })?;
            if written.iter().any(|&(e, ..)| e == edge) {
                return Err(EulerOpError::DuplicateRedescription { edge });
            }
            let curve_key = edge_data.curve;
            if self
                .get_curve_geom(curve_key)
                .ok_or(EulerOpError::StaleGeometry {
                    key: GeomRef::Curve(curve_key),
                })?
                .null_scaffold()
                .is_some()
            {
                return Err(EulerOpError::NullScaffoldCurve { curve: curve_key });
            }
            let sides = self.sides(edge, moved)?;
            if !sides.coherent_after(Named::of_spec(&spec.description)) {
                return Err(EulerOpError::DescriptionNotAdjacent { edge: Some(edge) });
            }
            let (p_start, p_end) = self.edge_endpoints(edge)?;
            let curve =
                crate::policy_lane::certify(spec.clone(), p_start, p_end, resolve(sides), band)
                    .map_err(|refusal| match refusal {
                        crate::policy_lane::ByPolicy::NoLane { scalar } => {
                            EulerOpError::NurbsLaneUnsupported {
                                edge: Some(edge),
                                scalar,
                            }
                        }
                        crate::policy_lane::ByPolicy::Refused(error) => {
                            EulerOpError::RechartFalsifies { edge, error }
                        }
                    })?;
            written.push((edge, sides, curve));
        }

        // ---- No unlisted edge stranded. ----
        let undescribed: Vec<EdgeKey> = self
            .rechart_edges(self.edges.keys(), moved, false)?
            .stranded
            .into_iter()
            .map(|(e, _)| e)
            .filter(|e| !written.iter().any(|(w, ..)| w == e))
            .collect();
        if !undescribed.is_empty() {
            return Err(EulerOpError::RechartUndescribed { edges: undescribed });
        }

        // ---- Every moved face's boundary on its new plane. ----
        for m in &faces {
            self.check_moved_boundary(m, &charts, &written, band)?;
        }

        // ---- Mutation (infallible from here on). ----
        let keys: Vec<SurfaceKey> = charts
            .into_iter()
            .map(|chart| match chart.surface {
                ChartSurface::New(surface) => self.add_surface(surface),
                ChartSurface::Shared(key) => key,
            })
            .collect();
        let key_of = |slot: Slot| match slot {
            Slot::Minted(chart) => keys.get(chart).copied(),
            Slot::Kept(k) => Some(k),
        };
        for m in &faces {
            let (Some(f), Some(new)) = (self.faces.get_mut(m.face), key_of(m.after)) else {
                unreachable!(
                    "set_face_surfaces_describing: every face resolved in the plan phase and \
                     every chart was minted"
                )
            };
            f.surface = new;
            f.sense = m.sense;
            if !m.on_parent_chart {
                self.drop_face_rows(m.face);
            }
        }
        for (edge, sides, curve) in written {
            let Some(rekeyed) = curve.with_remapped_surfaces(|k| key_of(sides.repoint(k))) else {
                unreachable!("set_face_surfaces_describing: every moved chart was minted")
            };
            self.replace_edge_curve(edge, rekeyed);
        }
        for m in &faces {
            self.remove_surface_if_orphaned(m.old);
        }

        #[cfg(debug_assertions)]
        self.assert_tier1_postcondition("set_face_surfaces_describing");
        Ok(keys)
    }

    /// **The re-descriptions a carry would give**, for a caller of
    /// [`Body::set_face_surfaces_describing`] that chooses them: every
    /// edge that door would refuse unlisted on `charts` whose stored
    /// description is coherent on the moved charts when its keys are
    /// read as that door reads a listed spec's, with that description
    /// restated, in edge-arena order. Carrier, interval and authority
    /// travel verbatim; a chart image is stated in its chart's
    /// coordinates, so it is left for the door to re-derive where the
    /// moved chart's payload is not the old one's.
    ///
    /// Nothing is certified here: the door certifies each under its
    /// band like any other listed spec. An edge the move leaves
    /// incoherent however its keys are read is not in the list, so the
    /// caller states it or the door refuses it. Pure.
    ///
    /// # Errors
    ///
    /// The door's per-face preconditions, in its order
    /// ([`EulerOpError::StaleKey`], [`EulerOpError::FaceMovedTwice`],
    /// [`EulerOpError::StaleGeometry`],
    /// [`EulerOpError::SenseContradictsChart`]); then `StaleKey` /
    /// `StaleGeometry` where a key the walk over the edges follows does
    /// not resolve.
    pub fn carried_redescriptions(
        &self,
        charts: &[Rechart<T>],
    ) -> Result<Vec<(EdgeKey, EdgeCurveSpec<T>)>, EulerOpError> {
        let faces = self.plan_recharts(charts)?;
        let moved = |_: HalfEdgeKey, _: LoopKey, f: FaceKey| moved_slot(&faces, f);
        let mut out = Vec::new();
        for (edge, sides) in self.rechart_edges(self.edges.keys(), moved, true)?.carried {
            out.push((edge, self.carried_spec(edge, sides, charts)?));
        }
        Ok(out)
    }

    /// The faces `charts` move, each resolved once with its moved sense,
    /// in `charts` order: [`Body::set_face_surfaces_describing`]'s
    /// per-face preconditions. Pure.
    fn plan_recharts(&self, charts: &[Rechart<T>]) -> Result<Vec<MovedFace>, EulerOpError> {
        let mut faces: Vec<MovedFace> = Vec::new();
        for (index, chart) in charts.iter().enumerate() {
            for wearer in &chart.faces {
                let face = wearer.face;
                let face_data = self.get_face(face).ok_or(EulerOpError::StaleKey {
                    key: EntityId::Face(face),
                })?;
                if faces.iter().any(|m| m.face == face) {
                    return Err(EulerOpError::FaceMovedTwice { face });
                }
                let old = face_data.surface;
                let resolved = self.resolve_face_surface(
                    &chart.face_surface(wearer.sense),
                    face,
                    (old, face_data.sense),
                    ParentSide::With,
                )?;
                faces.push(MovedFace {
                    face,
                    after: chart.slot(index),
                    old,
                    sense: resolved.sense,
                    on_parent_chart: resolved.on_parent_chart,
                });
            }
        }
        Ok(faces)
    }

    /// The surface `slot` stands for across a re-chart of `charts`.
    fn slot_surface<'a>(&'a self, charts: &'a [Rechart<T>], slot: Slot) -> Option<&'a Surface<T>> {
        match slot {
            Slot::Minted(chart) => match &charts.get(chart)?.surface {
                ChartSurface::New(surface) => Some(surface),
                ChartSurface::Shared(_) => None,
            },
            Slot::Kept(k) => self.surfaces.get(k),
        }
    }

    /// Whether `m`'s boundary lies on the plane it moves onto, under
    /// `band`: [`Body::set_face_surfaces_describing`]'s residual
    /// question, asked of every vertex of the face's loops and every
    /// interior certification sample of each edge on them, its curve
    /// read from `written` where the door re-describes it. Nothing is
    /// asked of a face that stays on its chart or moves onto a curved
    /// one. Pure.
    fn check_moved_boundary(
        &self,
        m: &MovedFace,
        charts: &[Rechart<T>],
        written: &[(EdgeKey, Sides, EdgeCurve<T>)],
        band: Band,
    ) -> Result<(), EulerOpError> {
        if m.on_parent_chart {
            return Ok(());
        }
        let Some(&Surface::Plane { origin, normal, .. }) = self.slot_surface(charts, m.after)
        else {
            return Ok(());
        };
        let on_plane = |p: Point3<T>, on: EntityId| match decide(
            "rechart_boundary_residual",
            Margin::of((p - origin).dot(normal)),
            band,
        ) {
            Ok(Sign::Zero) => Ok(()),
            Ok(Sign::Positive | Sign::Negative) => {
                Err(EulerOpError::RechartOffBoundary { face: m.face, on })
            }
            Err(diag) => Err(EulerOpError::RechartBoundaryEscalated {
                face: m.face,
                on,
                diag,
            }),
        };
        let face = self.get_face(m.face).ok_or(EulerOpError::StaleKey {
            key: EntityId::Face(m.face),
        })?;
        for lk in core::iter::once(face.outer).chain(face.rings.iter().copied()) {
            let first = match self
                .get_loop(lk)
                .ok_or(EulerOpError::StaleKey {
                    key: EntityId::Loop(lk),
                })?
                .boundary
            {
                LoopBoundary::Cycle { first } => first,
                // A lone vertex bounds the face too.
                LoopBoundary::Empty { vertex } => {
                    on_plane(self.resolve_vertex_point(vertex)?, EntityId::Vertex(vertex))?;
                    continue;
                }
            };
            let cycle = self
                .loop_cycle(first)
                .ok_or(EulerOpError::LoopCycleBroken { r#loop: lk })?;
            for he in cycle {
                let he_data = self.resolve_half_edge(he)?;
                on_plane(
                    self.resolve_vertex_point(he_data.start)?,
                    EntityId::Vertex(he_data.start),
                )?;
                let edge = he_data.edge;
                let curve = match written.iter().find(|(e, ..)| *e == edge) {
                    Some((.., curve)) => Some(curve),
                    None => {
                        let curve_key = self
                            .get_edge(edge)
                            .ok_or(EulerOpError::StaleKey {
                                key: EntityId::Edge(edge),
                            })?
                            .curve;
                        self.get_curve_geom(curve_key)
                            .ok_or(EulerOpError::StaleGeometry {
                                key: GeomRef::Curve(curve_key),
                            })?
                            .certified()
                    }
                };
                // A null edge has no carrier to sample; its endpoints
                // are the vertices asked above.
                let Some(curve) = curve else {
                    continue;
                };
                for i in 1..(geom_brep::CERT_SAMPLES - 1) {
                    on_plane(
                        curve.carrier().eval(curve.sample_param(i)),
                        EntityId::Edge(edge),
                    )?;
                }
            }
        }
        Ok(())
    }

    /// **The certified edges a re-chart touches**, in the order `edges`
    /// hands them: every one with a half `moved` answers for, given the
    /// half, its loop and its face, with the key that half's face wears
    /// after the move; a half it does not answer for stays on the key
    /// its face wears now. Scaffold and null edges, and an edge both of
    /// whose halves keep their keys, are skipped.
    /// `unvouched` holds those a moved side's new key is not among the
    /// keys of ([`Sides::vouched`]). Of those whose stored description
    /// is adjacency-coherent now, `stranded` holds those the move
    /// leaves incoherent. With `repoint`, a description's keys are read
    /// through [`Sides::repoint`] and `carried` holds those it leaves
    /// coherent only through a key their moved face wore; without it, a
    /// key stands for itself — the reading every door gives a stored
    /// description — and `carried` is empty. Pure.
    fn rechart_edges(
        &self,
        edges: impl IntoIterator<Item = EdgeKey>,
        moved: impl Fn(HalfEdgeKey, LoopKey, FaceKey) -> Option<Slot>,
        repoint: bool,
    ) -> Result<RechartEdges, EulerOpError> {
        let mut out = RechartEdges::default();
        for edge_key in edges {
            let edge = self.get_edge(edge_key).ok_or(EulerOpError::StaleKey {
                key: EntityId::Edge(edge_key),
            })?;
            let named = Named::of(self.get_curve_geom(edge.curve).ok_or(
                EulerOpError::StaleGeometry {
                    key: GeomRef::Curve(edge.curve),
                },
            )?);
            if matches!(named, Named::Nothing) {
                continue;
            }
            let sides = self.sides(edge_key, &moved)?;
            if sides.after == sides.before.map(Slot::Kept) {
                continue;
            }
            if !sides.vouched(named) {
                out.unvouched.push(edge_key);
            }
            if !sides.coherent_before(named) {
                continue;
            }
            let coherent = if repoint {
                sides.coherent_after(named)
            } else {
                named.adjacent_to(sides.after, Slot::Kept)
            };
            if !coherent {
                out.stranded.push((edge_key, sides));
            } else if repoint && named.keys().any(|k| sides.repoint(k) != Slot::Kept(k)) {
                out.carried.push((edge_key, sides));
            }
        }
        Ok(out)
    }

    /// **The keys-only re-chart refusals, one home for every door that
    /// puts existing half-edges, or a chord it mints, on a face wearing
    /// another key**: [`Body::set_face_surface`], [`Body::mef`],
    /// [`Body::mfkrh`] and [`Body::ring_move`], each with its siblings.
    ///
    /// The half-edges `moves` answers for (given each one's loop and
    /// face) land on a face wearing `after`, every other on the surface
    /// its face wears now; `old` is the key they leave. `edges` are the
    /// edges walked, as [`Body::rechart_edges`] walks them. A move that
    /// keeps `old` asks nothing. Otherwise the door refuses
    /// [`EulerOpError::RechartStrandsDescriptions`] where a description
    /// coherent now names no key either of its edge's faces wears after
    /// the move, then [`EulerOpError::RechartUnvouched`] where a
    /// certified edge lands on a chart it does not name — or `chord`,
    /// the chord a minting door mints with one half on each side, does
    /// — unless `one_payload` (a certificate is a function of the
    /// payload it was taken on, and the move re-reads that payload).
    /// Scaffold and null edges carry no certificate: they neither
    /// strand nor vouch, and are not asked. `face` is the face the
    /// refusal names. Pure.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn vouch_move(
        &self,
        door: RechartDoor,
        face: FaceKey,
        (old, after): (SurfaceKey, Slot),
        edges: impl IntoIterator<Item = EdgeKey>,
        moves: impl Fn(HalfEdgeKey, LoopKey, FaceKey) -> bool,
        one_payload: bool,
        chord: Option<&EdgeCurve<T>>,
    ) -> Result<(), EulerOpError> {
        if after == Slot::Kept(old) || refusals_lifted() {
            return Ok(());
        }
        let RechartEdges {
            stranded,
            unvouched,
            ..
        } = self.rechart_edges(edges, |he, l, f| moves(he, l, f).then_some(after), false)?;
        if !stranded.is_empty() {
            return Err(EulerOpError::RechartStrandsDescriptions {
                door,
                edges: stranded.into_iter().map(|(e, _)| e).collect(),
            });
        }
        let chord_unvouched = chord.is_some_and(|curve| {
            !Sides {
                before: [old, old],
                after: [Slot::Kept(old), after],
            }
            .vouched(Named::of_description(curve.description()))
        });
        if !one_payload && (chord_unvouched || !unvouched.is_empty()) {
            return Err(EulerOpError::RechartUnvouched {
                door,
                face,
                edges: unvouched,
                chord: chord_unvouched,
            });
        }
        Ok(())
    }

    /// **Failure-injection scope** (test builds only: this crate's own
    /// tests, `test-support` and `sweep-testing`): runs `op` with every
    /// keys-only re-chart refusal [`Body::vouch_move`] makes taken out —
    /// [`EulerOpError::RechartStrandsDescriptions`] and
    /// [`EulerOpError::RechartUnvouched`], at every door that raises
    /// them — so a door may leave edges described against a surface
    /// their faces no longer wear, or put a face's boundary on a chart
    /// no certified edge of it names. Every other precondition and
    /// every write is the real door's.
    ///
    /// It is for a row that builds such a body on purpose, and only
    /// there. Every call carries a one-line `// Lifts` comment naming
    /// the refusal it takes out and why that state is the row's
    /// premise.
    #[cfg(any(test, feature = "test-support", feature = "sweep-testing"))]
    #[doc(hidden)]
    pub fn lifting_rechart_refusals_for_tests<R>(&mut self, op: impl FnOnce(&mut Self) -> R) -> R {
        struct Restore(bool);
        impl Drop for Restore {
            fn drop(&mut self) {
                LIFTED.with(|lifted| lifted.set(self.0));
            }
        }
        let _restore = Restore(LIFTED.with(|lifted| lifted.replace(true)));
        op(self)
    }

    /// `edge`'s stored description restated on the charts its moved
    /// faces go to: [`Body::carried_redescriptions`]' entry for it. A
    /// chart image is stated in its chart's coordinates, so it is left
    /// to be re-derived on a chart whose payload is not the old one's.
    fn carried_spec(
        &self,
        edge: EdgeKey,
        sides: Sides,
        charts: &[Rechart<T>],
    ) -> Result<EdgeCurveSpec<T>, EulerOpError> {
        let curve_key = self
            .get_edge(edge)
            .ok_or(EulerOpError::StaleKey {
                key: EntityId::Edge(edge),
            })?
            .curve;
        let mut spec = self
            .get_curve_geom(curve_key)
            .and_then(crate::CurveGeom::certified)
            .ok_or(EulerOpError::StaleGeometry {
                key: GeomRef::Curve(curve_key),
            })?
            .restated_spec();
        if let geom_brep::EdgeDescriptionSpec::Chart { surface, image, .. } = &mut spec.description
        {
            let same = match sides.repoint(*surface) {
                Slot::Kept(k) => self.same_chart(*surface, k),
                Slot::Minted(chart) => charts
                    .get(chart)
                    .is_some_and(|c| self.same_chart_spec(*surface, &c.face_surface(true))),
            };
            if !same {
                *image = None;
            }
        }
        Ok(spec)
    }

    /// `edge`'s two faces (`he_plus`'s, then `he_minus`'s) across a
    /// re-chart: the surface each wears now, and the one it wears once
    /// `moved` has moved it.
    fn sides(
        &self,
        edge: EdgeKey,
        moved: impl Fn(HalfEdgeKey, LoopKey, FaceKey) -> Option<Slot>,
    ) -> Result<Sides, EulerOpError> {
        let sides = crate::readback::edge_sides(self, edge)?;
        let after = |side: crate::readback::EdgeSide| {
            let r#loop = self.resolve_half_edge(side.half_edge)?.parent_loop;
            Ok::<_, EulerOpError>(
                moved(side.half_edge, r#loop, side.face).unwrap_or(Slot::Kept(side.surface)),
            )
        };
        Ok(Sides {
            before: [sides.plus.surface, sides.minus.surface],
            after: [after(sides.plus)?, after(sides.minus)?],
        })
    }

    /// `edge`'s two endpoint points, `he_plus` forward order — the
    /// points every attach door certifies a description against.
    fn edge_endpoints(&self, edge: EdgeKey) -> Result<(Point3<T>, Point3<T>), EulerOpError> {
        let he_plus = self
            .get_edge(edge)
            .ok_or(EulerOpError::StaleKey {
                key: EntityId::Edge(edge),
            })?
            .he_plus;
        let start = self.resolve_half_edge(he_plus)?.start;
        let end = self.half_edge_end(he_plus).ok_or(EulerOpError::StaleKey {
            key: EntityId::HalfEdge(he_plus),
        })?;
        Ok((
            self.resolve_vertex_point(start)?,
            self.resolve_vertex_point(end)?,
        ))
    }

    /// Sets `face`'s orientation sense ([`crate::Face::sense`]) in
    /// place, on the chart it already has.
    ///
    /// A face minted or re-charted states its bit with its chart
    /// instead ([`FaceSurface`]; [`Body::resolve_face_surface`] owns
    /// the rule), so this door is for a bit learned while the chart
    /// stands still. Callers must keep the two encodings of
    /// orientation coherent — the bit and the loop winding — and the
    /// obligation is the CALLER'S, because at rest it is only partly
    /// checkable: tier 3's check 6 falsifies a planar disagreement
    /// whose loop rides `Line`, `Circle` and `Ellipse` carriers, and
    /// passes over one whose loop rides a spiric or NURBS carrier (the
    /// residue named at the arm's banner), so a planar face bounded so
    /// can carry an inverted bit through this door and certify. The
    /// test-only hand-flip door [`Body::flipped_face_sense_for_tests`]
    /// is the deliberate exception to the coherence rule; it is not the
    /// only way to break it.
    ///
    /// Not an Euler operator (no topology changes) and not a numeric
    /// decision (a `bool` is written, nothing compared); tier 1 is
    /// trivially preserved.
    ///
    /// # Errors
    ///
    /// [`EulerOpError::StaleKey`] if `face` does not resolve. The body
    /// is untouched on `Err`.
    pub fn set_face_sense(&mut self, face: FaceKey, sense: bool) -> Result<(), EulerOpError> {
        let f = self.get_face_mut(face).ok_or(EulerOpError::StaleKey {
            key: EntityId::Face(face),
        })?;
        f.sense = sense;
        Ok(())
    }

    /// Replaces `edge`'s curve geometry with a freshly certified
    /// attachment of `curve`, returning the new curve key. The
    /// certification endpoints are the edge's own vertices' points in
    /// `he_plus` forward order; intrinsic/seam descriptions must also
    /// be **adjacency-coherent** (module docs). The old curve is
    /// removed iff no other edge references it.
    ///
    /// **A carrier swap leaves the pcurve rows where they are, and
    /// that is not [`Body::set_face_surface`]'s case.** A row is stated
    /// in a FACE's chart and keyed on a half-edge; this door moves
    /// neither, so no row changes what it is ABOUT. What it does change
    /// is what the row must agree WITH, and the tier-3 pcurve pass
    /// re-derives that agreement from the edge's CURRENT curve on every
    /// run — so a row left saying the old carrier's image is refused
    /// per half-edge, loud, on a complete face and on the rows a
    /// half-minted one stores alike, which is where the surface setter
    /// was silent. A face whose chart mints nothing holds no minted row
    /// for a carrier swap to stale at all.
    ///
    /// **A null edge's first description re-mints its loops.**
    /// [`Body::mev_null`] adds two halves with no carrier to derive a
    /// row from, and returns a minted face missing their rows. The
    /// carrier arrives here, so before the door mutates, each face the
    /// halves are on that the site mint selects — a minted face whose
    /// every loop walks, and whose only gaps are on loops a null edge
    /// holds open or which no null edge is left on once this one is
    /// described — has every loop that no null edge holds open then
    /// re-minted whole, through the site mint the Euler operators run
    /// ([`crate::pcurves`]' `site_rows`): the loop leaves complete — the
    /// rows of halves an operator added while it was held open included,
    /// and on a face no null edge is left on every row it missed — or
    /// the face rowless where the closed-form lane cannot mint it. A
    /// loop another null edge still runs through is left as found, for
    /// that edge to release. A face on a spline chart is left as found.
    ///
    /// # Errors
    ///
    /// [`EulerOpError::StaleKey`] / [`EulerOpError::StaleGeometry`] on
    /// unresolvable topology/points;
    /// [`EulerOpError::DescriptionNotAdjacent`] on an
    /// `Intersection`/`Seam` description whose surfaces are not the
    /// edge's faces' surfaces; [`EulerOpError::Certification`] on a
    /// failed gate, whose plane × NURBS lane is the scalar's policy
    /// ([`crate::AtRestPolicy::nurbs_lane`]), and
    /// [`EulerOpError::NurbsLaneUnsupported`] where that class meets a
    /// scalar holding none; [`EulerOpError::PcurveMint`] where a null
    /// edge's face is re-minted and a half-edge of it does not resolve.
    /// The body is untouched on `Err`.
    pub fn set_edge_curve(
        &mut self,
        edge: EdgeKey,
        curve: EdgeCurveSpec<T>,
        tol: Tol,
    ) -> Result<CurveKey, EulerOpError>
    where
        T: crate::props::AtRestPolicy,
    {
        let (p_start, p_end) = self.edge_endpoints(edge)?;
        self.check_description_adjacent(edge, &curve.description)?;

        let certified = self.certify_edge_spec(Some(edge), curve, p_start, p_end, tol)?;
        let rows = self.null_description_rows(edge, &certified, tol)?;

        // ---- Mutation (infallible from here on). ----
        let new = self.replace_edge_curve(edge, certified);
        crate::pcurves::apply_site_rows(self, rows, None);

        #[cfg(debug_assertions)]
        self.assert_tier1_postcondition("set_edge_curve");
        Ok(new)
    }

    /// Re-states `edge` as an image in `chart`, keeping its carrier,
    /// its parameter interval and — through `at_rest_in_chart` — the
    /// pushforward that scaffolded it, as its authority record.
    ///
    /// This is D2's **conventional split**, at rest: where two
    /// surfaces under-determine an edge's locus (one surface on both
    /// sides, or a smooth pair whose jet is zero), the description
    /// stays conventional rather than intrinsic — but the edge is
    /// between two real faces now, so it is an image in a chart and
    /// not the scaffolding the mint left (D3's transience fence; the
    /// scaffolding door is for edges whose surfaces do not exist yet).
    ///
    /// **The carrier is RESTATED, never rebuilt.** Re-deriving it from
    /// the endpoints recomputes the direction and the interval from a
    /// sum that need not be bitwise what the edge was minted with,
    /// which silently moves geometry in a pass whose whole contract is
    /// that only the DESCRIPTION moves. Through `at_rest_in_chart` the
    /// pushforward that scaffolded the edge stays beside it as the
    /// authority record, which is what keeps tier 3's prefer-intrinsic
    /// reading unchanged.
    ///
    /// One home for a rule three sweep lanes read: `extrude`'s strut
    /// join, `revolve::upgrade`'s join lanes, and `swept`'s
    /// scaffold-retirement pass each spelled these two lines
    /// themselves. It lives here, beside [`Body::set_edge_curve`],
    /// because that is the certified mutation it performs and because
    /// `topo` is the crate both callers already depend on.
    ///
    /// # Errors
    ///
    /// [`EulerOpError::StaleKey`] / [`EulerOpError::StaleGeometry`] on
    /// unresolvable topology or geometry;
    /// [`EulerOpError::NullScaffoldCurve`] on an edge whose curve
    /// carries no certified geometry to re-state; and whatever
    /// [`Body::set_edge_curve`] raises on the re-attachment.
    pub fn describe_at_rest(
        &mut self,
        edge: EdgeKey,
        chart: SurfaceKey,
        tol: Tol,
    ) -> Result<(), EulerOpError>
    where
        T: crate::props::AtRestPolicy,
    {
        let curve_key = self
            .get_edge(edge)
            .ok_or(EulerOpError::StaleKey {
                key: EntityId::Edge(edge),
            })?
            .curve;
        let spec = self
            .get_curve_geom(curve_key)
            .ok_or(EulerOpError::StaleGeometry {
                key: crate::GeomRef::Curve(curve_key),
            })?
            .certified()
            .ok_or(EulerOpError::NullScaffoldCurve { curve: curve_key })?
            .restated_spec()
            .at_rest_in_chart(chart, false);
        self.set_edge_curve(edge, spec, tol)?;
        Ok(())
    }

    /// **The rows a null edge's first description writes**, decided
    /// before [`Body::set_edge_curve`] mutates: one plan per face
    /// the edge's halves are on, for [`crate::pcurves::apply_site_rows`].
    ///
    /// Empty unless `edge` is a null edge ([`crate::CurveGeom::NullScaffold`]):
    /// a certified edge's description moves no key, so no row goes
    /// missing (the door's docs), and a face it finds half-minted is
    /// left as found. A null edge's description is the first door that
    /// can derive its halves' rows, so on each face they are on that the
    /// site mint selects it re-walks every loop, through the Euler
    /// operators' site mint ([`Body::plan_site_mint`]), and mints each
    /// one no other null edge runs through; on a spline chart the face
    /// is left as found.
    ///
    /// # Errors
    ///
    /// [`EulerOpError::StaleKey`] / [`EulerOpError::StaleGeometry`] where
    /// a half, loop, face or surface does not resolve;
    /// [`EulerOpError::Certification`] where `tol` builds no band;
    /// [`EulerOpError::PcurveMint`] naming the face a half-edge of which
    /// did not resolve.
    fn null_description_rows(
        &self,
        edge: EdgeKey,
        curve: &EdgeCurve<T>,
        tol: Tol,
    ) -> Result<Vec<SiteRows<T>>, EulerOpError> {
        let edge_data = self.get_edge(edge).ok_or(EulerOpError::StaleKey {
            key: EntityId::Edge(edge),
        })?;
        let is_null = self
            .get_curve_geom(edge_data.curve)
            .ok_or(EulerOpError::StaleGeometry {
                key: GeomRef::Curve(edge_data.curve),
            })?
            .null_scaffold()
            .is_some();
        if !is_null {
            return Ok(Vec::new());
        }
        let halves = [edge_data.he_plus, edge_data.he_minus];
        let touched = [
            self.resolve_half_edge(halves[0])?.parent_loop,
            self.resolve_half_edge(halves[1])?.parent_loop,
        ];
        let site_half = |h: HalfEdgeKey| {
            if halves.contains(&h) {
                SiteHalf::Described(h)
            } else {
                SiteHalf::Existing(h)
            }
        };
        self.plan_site_mint(
            &touched,
            |body, minted| {
                minted
                    .iter()
                    .map(|(face, from)| {
                        let every_loop: Vec<(LoopKey, Vec<SiteHalf>)> = from
                            .rows
                            .loops
                            .iter()
                            .filter_map(|(lk, cycle)| {
                                Some((
                                    *lk,
                                    cycle.as_deref()?.iter().copied().map(site_half).collect(),
                                ))
                            })
                            .collect();
                        body.site_face(*face, &every_loop, None)
                    })
                    .collect()
            },
            Some(curve),
            tol,
        )
    }

    /// The mutation half of every door that re-describes an existing
    /// edge ([`Body::set_edge_curve`] and its lane twin,
    /// [`Body::kev_describing`]'s re-described members): insert the
    /// certified curve, point `edge` at it, and reap the curve it
    /// replaced iff orphaned. Returns the new key.
    ///
    /// Infallible on the caller's proof that `edge` resolved in its
    /// plan phase and that nothing between that plan and this write
    /// removes it.
    pub(crate) fn replace_edge_curve(
        &mut self,
        edge: EdgeKey,
        certified: EdgeCurve<T>,
    ) -> CurveKey {
        let new = self.add_curve(certified);
        let Some(e) = self.get_edge_mut(edge) else {
            unreachable!(
                "replace_edge_curve: the caller's plan phase resolved `edge`, and nothing \
                 between that plan and this write removes it"
            )
        };
        let old = core::mem::replace(&mut e.curve, new);
        self.remove_curve_if_orphaned(old);
        new
    }

    /// [`require_description_adjacent`] for a spec about to describe
    /// `edge`, against the surfaces its two faces wear now. Pure — the
    /// plan-phase half of every door that re-describes an existing
    /// edge.
    ///
    /// # Errors
    ///
    /// [`EulerOpError::StaleKey`] when the edge, a half, a loop or a
    /// face does not resolve; [`EulerOpError::DescriptionNotAdjacent`]
    /// when the description names surfaces that are not the edge's.
    pub(crate) fn check_description_adjacent(
        &self,
        edge: EdgeKey,
        description: &geom_brep::EdgeDescriptionSpec<T>,
    ) -> Result<(), EulerOpError> {
        let sides = self.sides(edge, |_, _, _| None)?;
        require_description_adjacent(Some(edge), description, sides.before.map(Slot::Kept))
    }
}

/// The **description-adjacency coherence** check (module docs), one
/// home for [`Body::set_edge_curve`] and the minting doors that take a
/// caller's description ([`Body::mef`]): an intrinsic description's two
/// surfaces are exactly `faces`, a chart image names one of them, a
/// chart seam names the one surface on both sides, and a scaffold names
/// none. `faces` are the surfaces the edge's two faces wear, `he_plus`'s
/// first; `edge` is the edge the refusal names, `None` for the one a
/// minting door mints. Pure.
///
/// # Errors
///
/// [`EulerOpError::DescriptionNotAdjacent`] when the description names
/// surfaces that are not `faces`.
pub(crate) fn require_description_adjacent<T: Real>(
    edge: Option<EdgeKey>,
    description: &geom_brep::EdgeDescriptionSpec<T>,
    faces: [Slot; 2],
) -> Result<(), EulerOpError> {
    if Named::of_spec(description).adjacent_to(faces, Slot::Kept) {
        Ok(())
    } else {
        Err(EulerOpError::DescriptionNotAdjacent { edge })
    }

    /// Whether `edge`'s STORED description is adjacency-coherent with
    /// the two faces it lies between — the reading
    /// [`Body::check_description_adjacent`] gives a spec, and tier 3's
    /// `DescriptionNotAdjacent` at rest. Exact: it compares keys and
    /// reads no coordinate.
    ///
    /// # Errors
    ///
    /// [`EulerOpError::StaleKey`] when the edge, a half, a loop or a
    /// face does not resolve; [`EulerOpError::StaleGeometry`] when its
    /// curve does not.
    pub(crate) fn stored_description_adjacent(&self, edge: EdgeKey) -> Result<bool, EulerOpError> {
        let curve = self
            .get_edge(edge)
            .ok_or(EulerOpError::StaleKey {
                key: EntityId::Edge(edge),
            })?
            .curve;
        let stored = self
            .get_curve_geom(curve)
            .ok_or(EulerOpError::StaleGeometry {
                key: crate::GeomRef::Curve(curve),
            })?;
        Ok(self
            .sides(edge, |_| None)?
            .coherent_before(Named::of(stored)))
    }
}

/// One chart [`Body::set_face_surfaces_describing`] moves faces onto,
/// and the faces that move onto it — never none, since a chart no face
/// wears would be a surface nothing references.
#[derive(Clone, Debug)]
pub struct Rechart<T: Real> {
    surface: ChartSurface<T>,
    /// Non-empty: every constructor takes the first face.
    faces: Vec<Wearer>,
}

impl<T: Real> Rechart<T> {
    /// `face` onto a chart minted fresh from `surface`
    /// ([`FaceSurface::New`]'s), stating `sense`, its material side
    /// against the chart's normal.
    #[must_use]
    pub fn new(surface: Surface<T>, face: FaceKey, sense: bool) -> Self {
        Self {
            surface: ChartSurface::New(surface),
            faces: vec![Wearer { face, sense }],
        }
    }

    /// `face` onto the surface the body already holds at `key`
    /// ([`FaceSurface::Shared`]'s), stating `sense` against its normal.
    #[must_use]
    pub fn shared(key: SurfaceKey, face: FaceKey, sense: bool) -> Self {
        Self {
            surface: ChartSurface::Shared(key),
            faces: vec![Wearer { face, sense }],
        }
    }

    /// `face` onto this chart too, stating `sense` against its normal.
    #[must_use]
    pub fn with(mut self, face: FaceKey, sense: bool) -> Self {
        self.faces.push(Wearer { face, sense });
        self
    }

    /// The [`FaceSurface`] a face stating `sense` takes onto this chart.
    fn face_surface(&self, sense: bool) -> FaceSurface<T> {
        match &self.surface {
            ChartSurface::New(surface) => FaceSurface::New {
                surface: surface.clone(),
                sense,
            },
            ChartSurface::Shared(key) => FaceSurface::Shared { key: *key, sense },
        }
    }

    /// The surface a face moved onto this chart wears, this being the
    /// chart at `index` in the call's list.
    fn slot(&self, index: usize) -> Slot {
        match self.surface {
            ChartSurface::New(_) => Slot::Minted(index),
            ChartSurface::Shared(key) => Slot::Kept(key),
        }
    }
}

#[cfg(any(test, feature = "test-support", feature = "sweep-testing"))]
thread_local! {
    /// Whether [`Body::lifting_rechart_refusals_for_tests`] is running
    /// on this thread.
    static LIFTED: core::cell::Cell<bool> = const { core::cell::Cell::new(false) };
}

/// Whether the keys-only re-chart refusals are taken out
/// ([`Body::lifting_rechart_refusals_for_tests`]); never outside a test
/// build.
fn refusals_lifted() -> bool {
    #[cfg(any(test, feature = "test-support", feature = "sweep-testing"))]
    {
        LIFTED.with(core::cell::Cell::get)
    }
    #[cfg(not(any(test, feature = "test-support", feature = "sweep-testing")))]
    {
        false
    }
}

/// A [`Rechart`]'s chart: [`FaceSurface`]'s `New` and `Shared`, the
/// sense stated per face instead.
#[derive(Clone, Debug)]
enum ChartSurface<T: Real> {
    New(Surface<T>),
    Shared(SurfaceKey),
}

/// A face a [`Rechart`] moves, and the material side it states against
/// the chart's normal.
#[derive(Clone, Copy, Debug)]
struct Wearer {
    face: FaceKey,
    sense: bool,
}

/// A face [`Body::set_face_surfaces_describing`] moves, as its plan
/// phase resolved it.
struct MovedFace {
    face: FaceKey,
    /// The surface it wears after the move.
    after: Slot,
    /// The surface the face wears before the move.
    old: SurfaceKey,
    sense: bool,
    /// Whether the new chart is the old one, the face standing as its
    /// own parent ([`crate::euler::ResolvedFace::on_parent_chart`]).
    on_parent_chart: bool,
}

/// The surface `face` wears once `faces` have moved, if it moves.
fn moved_slot(faces: &[MovedFace], face: FaceKey) -> Option<Slot> {
    faces.iter().find(|m| m.face == face).map(|m| m.after)
}

/// [`Body::rechart_edges`]' answer.
#[derive(Default)]
struct RechartEdges {
    stranded: Vec<(EdgeKey, Sides)>,
    carried: Vec<(EdgeKey, Sides)>,
    unvouched: Vec<EdgeKey>,
}

/// An edge's two faces across a re-chart ([`Body::sides`]): the surface
/// each wears before, and after.
#[derive(Clone, Copy)]
struct Sides {
    before: [SurfaceKey; 2],
    after: [Slot; 2],
}

impl Sides {
    /// What `key`, named in this edge's description, stands for once
    /// the re-chart lands: itself while a face of the edge still wears
    /// it, else the chart a face that wore it moves onto (`he_plus`'s
    /// first), else itself.
    fn repoint(self, key: SurfaceKey) -> Slot {
        if self.after.contains(&Slot::Kept(key)) {
            return Slot::Kept(key);
        }
        match self.before {
            [plus, _] if plus == key => self.after[0],
            [_, minus] if minus == key => self.after[1],
            _ => Slot::Kept(key),
        }
    }

    fn coherent_before(self, named: Named) -> bool {
        named.adjacent_to(self.before.map(Slot::Kept), Slot::Kept)
    }

    fn coherent_after(self, named: Named) -> bool {
        named.adjacent_to(self.after, |k| self.repoint(k))
    }

    /// Whether every side that moves lands on a key `named` names: the
    /// edge's certificate vouches for it on each chart its faces move
    /// onto. A scaffold or null edge carries no certificate, so it
    /// vouches for nothing and is not asked.
    fn vouched(self, named: Named) -> bool {
        matches!(named, Named::Nothing)
            || (0..2).all(|i| {
                self.after[i] == Slot::Kept(self.before[i])
                    || named.keys().any(|k| Slot::Kept(k) == self.after[i])
            })
    }
}

/// The surface a face wears, or a description names, once a re-chart
/// lands: a key the body already holds, or the chart the call mints at
/// that position — which has no key until the mutation phase.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Slot {
    Kept(SurfaceKey),
    Minted(usize),
}

impl Slot {
    /// The surface a face given `spec` wears, `own` being the key
    /// `Inherit` keeps; a `New` surface is the one chart its door mints.
    pub(crate) fn of_spec<T: Real>(spec: &FaceSurface<T>, own: SurfaceKey) -> Self {
        match spec {
            FaceSurface::Inherit => Self::Kept(own),
            FaceSurface::New { .. } => Self::Minted(0),
            FaceSurface::Shared { key, .. } => Self::Kept(*key),
        }
    }
}

/// The surfaces an edge description names, in the shape the adjacency
/// rule reads them — one reading for the stored description and the
/// spec, so the attach doors cannot drift apart.
#[derive(Clone, Copy)]
enum Named {
    /// An intrinsic description's two operands (`Intersection` and
    /// `TangentIntersection` alike: the described pair IS the faces'
    /// pair).
    Pair(SurfaceKey, SurfaceKey),
    /// A chart image's chart, and whether the image claims to be the
    /// chart's parameterization seam.
    Chart { surface: SurfaceKey, seam: bool },
    /// A scaffold, or a null edge: there is no surface to name.
    Nothing,
}

impl Named {
    fn of<T: Real>(curve: &crate::CurveGeom<T>) -> Self {
        curve.certified().map_or(Self::Nothing, |curve| {
            Self::of_description(curve.description())
        })
    }

    fn of_description<T: Real>(description: &geom_brep::EdgeDescription<T>) -> Self {
        match description {
            geom_brep::EdgeDescription::Intersection { s1, s2, .. }
            | geom_brep::EdgeDescription::TangentIntersection { s1, s2, .. } => {
                Self::Pair(*s1, *s2)
            }
            geom_brep::EdgeDescription::Chart(c) => Self::Chart {
                surface: c.surface,
                seam: c.seam,
            },
            geom_brep::EdgeDescription::Scaffold(_) => Self::Nothing,
        }
    }

    fn of_spec<T: Real>(description: &geom_brep::EdgeDescriptionSpec<T>) -> Self {
        match *description {
            geom_brep::EdgeDescriptionSpec::Intersection { s1, s2, .. }
            | geom_brep::EdgeDescriptionSpec::TangentIntersection { s1, s2, .. } => {
                Self::Pair(s1, s2)
            }
            geom_brep::EdgeDescriptionSpec::Chart { surface, seam, .. } => {
                Self::Chart { surface, seam }
            }
            geom_brep::EdgeDescriptionSpec::Scaffold(_) => Self::Nothing,
        }
    }

    fn keys(self) -> impl Iterator<Item = SurfaceKey> {
        let (a, b) = match self {
            Self::Pair(s1, s2) => (Some(s1), Some(s2)),
            Self::Chart { surface, .. } => (Some(surface), None),
            Self::Nothing => (None, None),
        };
        a.into_iter().chain(b)
    }

    /// Whether the description is coherent with faces wearing `plus`
    /// and `minus`, its own keys read through `slot_of`. A chart image
    /// names ONE of the two faces' surfaces (a wall–wall seam is the
    /// u-boundary iso of either wall, and the minted convention picks
    /// one); an image that claims to BE the chart's parameterization
    /// seam names the one surface on both sides, by what a seam is; a
    /// scaffold names none.
    fn adjacent_to(self, [plus, minus]: [Slot; 2], slot_of: impl Fn(SurfaceKey) -> Slot) -> bool {
        match self {
            Self::Pair(s1, s2) => {
                let (s1, s2) = (slot_of(s1), slot_of(s2));
                (s1 == plus && s2 == minus) || (s1 == minus && s2 == plus)
            }
            Self::Chart { surface, seam } => {
                let surface = slot_of(surface);
                if seam {
                    surface == plus && surface == minus
                } else {
                    surface == plus || surface == minus
                }
            }
            Self::Nothing => true,
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use geom::Surface;
    use geom_brep::{EdgeCurveSpec, EdgeDescription, EdgeDescriptionSpec};
    use geom_core::{Point3, Tol, Vec3};

    use super::Rechart;
    use crate::body::Body;
    use crate::entity::{EdgeKey, EntityId, FaceKey, HalfEdgeKey, LoopBoundary, LoopKey};
    use crate::euler::{EulerOpError, FaceSurface, MefSite, RechartDoor};
    use crate::fixtures::{assert_err_deep_unchanged, deep_snapshot};
    use crate::geometry::SurfaceKey;
    use crate::test_support_fixtures::{brick, plant_ring_face};
    use crate::validate::{ValidationError, validate_geometric};

    fn tol() -> Tol {
        Tol::witness()
    }

    fn unit_brick() -> Body<f64> {
        brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol())
    }

    /// The brick face on the plane `coord[axis] == value`.
    fn face_at(body: &Body<f64>, axis: usize, value: f64) -> FaceKey {
        body.faces()
            .find(|(_, f)| match body.get_surface(f.surface) {
                Some(Surface::Plane { origin, normal, .. }) => {
                    let (o, n) = (origin.to_array(), normal.to_array());
                    n[axis].abs() > 0.5 && o[axis] == value
                }
                _ => false,
            })
            .map(|(k, _)| k)
            .unwrap()
    }

    /// The unit brick, which [`brick`] describes as intersections, and
    /// its top face (the `z = 1` cap).
    fn brick_and_top() -> (Body<f64>, FaceKey) {
        let body = unit_brick();
        let top = face_at(&body, 2, 1.0);
        assert_eq!(validate_geometric(&body, tol()), Ok(()));
        (body, top)
    }

    fn surf(body: &Body<f64>, face: FaceKey) -> SurfaceKey {
        body.get_face(face).unwrap().surface
    }

    fn sense(body: &Body<f64>, face: FaceKey) -> bool {
        body.get_face(face).unwrap().sense
    }

    /// `face`'s own plane, moved by `by` — a fresh payload, so a swap
    /// onto it is a swap onto another chart.
    fn plane_moved(body: &Body<f64>, face: FaceKey, by: Vec3<f64>) -> Surface<f64> {
        let Some(&Surface::Plane {
            origin,
            normal,
            u_ref,
        }) = body.get_surface(surf(body, face))
        else {
            panic!("a brick face is a plane")
        };
        Surface::Plane {
            origin: origin + by,
            normal,
            u_ref,
        }
    }

    /// The top cap's own plane, raised by `dz`.
    fn cap_at(body: &Body<f64>, top: FaceKey, dz: f64) -> Surface<f64> {
        plane_moved(body, top, Vec3::new(0.0, 0.0, dz))
    }

    /// The edges whose description names `surface`, in edge-arena order.
    fn edges_naming(body: &Body<f64>, surface: SurfaceKey) -> Vec<EdgeKey> {
        body.edges()
            .filter(|(_, e)| {
                Body::<f64>::description_surfaces(body.get_curve_geom(e.curve).unwrap())
                    .contains(&surface)
            })
            .map(|(k, _)| k)
            .collect()
    }

    /// `edge`'s two faces, `he_plus`'s first.
    fn faces_of(body: &Body<f64>, edge: EdgeKey) -> [FaceKey; 2] {
        let (plus, minus) = crate::readback::edge_sides(body, edge).unwrap().faces();
        [plus, minus]
    }

    /// The edges with a half on `face`, in edge-arena order.
    fn edges_of_face(body: &Body<f64>, face: FaceKey) -> Vec<EdgeKey> {
        body.edges()
            .map(|(k, _)| k)
            .filter(|&k| faces_of(body, k).contains(&face))
            .collect()
    }

    fn description(body: &Body<f64>, edge: EdgeKey) -> EdgeDescription<f64> {
        body.get_curve_geom(body.get_edge(edge).unwrap().curve)
            .and_then(crate::CurveGeom::certified)
            .unwrap()
            .description()
            .clone()
    }

    fn restated(body: &Body<f64>, edge: EdgeKey) -> EdgeCurveSpec<f64> {
        body.get_curve_geom(body.get_edge(edge).unwrap().curve)
            .and_then(crate::CurveGeom::certified)
            .unwrap()
            .restated_spec()
    }

    /// Tier 3's verdicts on `body`, by kind.
    fn kinds(body: &Body<f64>) -> Vec<String> {
        let mut kinds: Vec<String> = validate_geometric(body, tol())
            .err()
            .unwrap_or_default()
            .iter()
            .map(|e| {
                format!("{e:?}")
                    .split([' ', '{', '('])
                    .next()
                    .unwrap()
                    .to_string()
            })
            .collect();
        kinds.sort();
        kinds.dedup();
        kinds
    }

    /// The unit brick with a square membrane planted in its top cap: a
    /// ring face on the cap's key, whose four edges are scaffolds until
    /// [`brick_with_inlay`] describes each as an image in that chart —
    /// an inlay whose every edge is smooth, so no edge of it names a
    /// surface only the membrane wears. After the review's C1 probe
    /// (PR 3580).
    fn brick_with_scaffold_inlay() -> (Body<f64>, FaceKey, FaceKey) {
        let mut body = unit_brick();
        let top = face_at(&body, 2, 1.0);
        let LoopBoundary::Cycle { first } = body
            .get_loop(body.get_face(top).unwrap().outer)
            .unwrap()
            .boundary
        else {
            panic!("the cap's outer loop is a cycle")
        };
        let rim = [
            Point3::new(0.25, 0.25, 1.0),
            Point3::new(0.75, 0.25, 1.0),
            Point3::new(0.75, 0.75, 1.0),
            Point3::new(0.25, 0.75, 1.0),
        ];
        let membrane = plant_ring_face(&mut body, first, &rim, tol()).membrane.face;
        let cap = surf(&body, top);
        assert_eq!(
            surf(&body, membrane),
            cap,
            "the membrane is on the cap's key"
        );
        (body, top, membrane)
    }

    /// [`brick_with_scaffold_inlay`] with every membrane edge described
    /// as an image in the cap's chart.
    fn brick_with_inlay() -> (Body<f64>, FaceKey, FaceKey) {
        let (mut body, top, membrane) = brick_with_scaffold_inlay();
        let cap = surf(&body, top);
        for edge in edges_of_face(&body, membrane) {
            let mut spec = restated(&body, edge);
            spec.description = EdgeDescriptionSpec::chart(cap);
            body.set_edge_curve(edge, spec, tol()).unwrap();
        }
        (body, top, membrane)
    }

    /// **The keys-only door refuses the swap it used to strand.** A
    /// swap of the top cap onto a fresh key leaves its four edges'
    /// `Intersection`s naming the key the cap left: the door names all
    /// four and writes nothing. The stranding door returns `Ok` on the
    /// same swap, and tier 3 reports exactly those four at rest.
    #[test]
    fn a_swap_that_strands_an_edge_refuses_naming_every_one_and_writes_nothing() {
        let (mut body, top) = brick_and_top();
        let sense = sense(&body, top);
        let named = edges_naming(&body, surf(&body, top));
        assert_eq!(named.len(), 4, "the cap's four rim edges name its plane");
        let cap = cap_at(&body, top, 0.0);
        let swap = || FaceSurface::New {
            surface: cap.clone(),
            sense,
        };

        let mut stranded = body.clone();
        // Lifts RechartStrandsDescriptions: the stranded state tier 3 reports at rest is the row.
        stranded
            .set_face_surface_stranding_for_tests(top, swap())
            .unwrap();
        let errs = validate_geometric(&stranded, tol()).unwrap_err();
        let at_rest: Vec<EdgeKey> = errs
            .iter()
            .filter_map(|e| match e {
                ValidationError::DescriptionNotAdjacent { edge } => Some(*edge),
                _ => None,
            })
            .collect();
        assert_eq!(at_rest, named, "tier 3 reports the four at rest: {errs:?}");

        assert_err_deep_unchanged(
            &mut body,
            &EulerOpError::RechartStrandsDescriptions {
                door: RechartDoor::SetFaceSurface,
                edges: named.clone(),
            },
            |b| b.set_face_surface(top, swap()).unwrap_err(),
        );
    }

    /// **The describing door re-describes nothing by default.** The same
    /// swap with nothing listed refuses, naming the four rim edges, with
    /// the body untouched. With the stored descriptions
    /// [`Body::carried_redescriptions`] states listed, the four are
    /// re-certified on the new key and the body is valid at rest. Onto a
    /// plane a thousand `eps` above the cap, the first of them does not
    /// certify and the door refuses, naming it, with the body untouched.
    #[test]
    fn the_describing_door_takes_only_what_it_is_handed_and_refuses_where_it_goes_stale() {
        let (mut body, top) = brick_and_top();
        let sense = sense(&body, top);
        let old = surf(&body, top);
        let named = edges_naming(&body, old);
        let onto = |dz: f64, b: &Body<f64>| vec![Rechart::new(cap_at(b, top, dz), top, sense)];

        assert_err_deep_unchanged(
            &mut body,
            &EulerOpError::RechartUndescribed {
                edges: named.clone(),
            },
            |b| {
                b.set_face_surfaces_describing(onto(0.0, b), &[], tol())
                    .unwrap_err()
            },
        );

        let mut carried = body.clone();
        let specs = carried
            .carried_redescriptions(&onto(0.0, &carried))
            .unwrap();
        assert_eq!(
            specs.iter().map(|(e, _)| *e).collect::<Vec<_>>(),
            named,
            "the helper states the four, in edge-arena order"
        );
        let [new] = carried
            .set_face_surfaces_describing(onto(0.0, &body), &specs, tol())
            .unwrap()[..]
        else {
            panic!("one chart, one key")
        };
        assert_eq!(carried.get_face(top).unwrap().surface, new);
        assert_eq!(edges_naming(&carried, new), named, "all four carried");
        assert!(carried.get_surface(old).is_none(), "the old cap is reaped");
        assert_eq!(validate_geometric(&carried, tol()), Ok(()));

        let eps = tol().eps();
        let before = deep_snapshot(&body);
        let shifted = onto(1000.0 * eps, &body);
        let specs = body.carried_redescriptions(&shifted).unwrap();
        let err = body
            .set_face_surfaces_describing(shifted, &specs, tol())
            .unwrap_err();
        assert!(
            matches!(err, EulerOpError::RechartFalsifies { edge, .. } if edge == named[0]),
            "{err:?}"
        );
        assert_eq!(deep_snapshot(&body), before, "body changed on Err");
    }

    /// **A listed spec names the chart by the key the face wears now.**
    /// One rim edge restated as an image in the cap's chart, beside the
    /// carried descriptions of the other three: it lands as a chart
    /// image on the NEW key, and the three as intersections on it.
    #[test]
    fn a_listed_spec_is_stated_against_the_moved_chart_by_its_old_key() {
        let (mut body, top) = brick_and_top();
        let sense = sense(&body, top);
        let old = surf(&body, top);
        let named = edges_naming(&body, old);
        let listed = named[0];
        let mut spec = restated(&body, listed);
        spec.description = EdgeDescriptionSpec::chart(old);
        let charts = vec![Rechart::new(cap_at(&body, top, 0.0), top, sense)];
        let mut specs = vec![(listed, spec)];
        specs.extend(
            body.carried_redescriptions(&charts)
                .unwrap()
                .into_iter()
                .filter(|(e, _)| *e != listed),
        );
        let [new] = body
            .set_face_surfaces_describing(charts, &specs, tol())
            .unwrap()[..]
        else {
            panic!("one chart, one key")
        };
        assert!(
            matches!(description(&body, listed), EdgeDescription::Chart(c) if c.surface == new),
            "the listed edge is an image in the new chart"
        );
        for &e in &named[1..] {
            assert!(
                matches!(
                    description(&body, e),
                    EdgeDescription::Intersection { s1, s2, .. } if s1 == new || s2 == new
                ),
                "an edge listed from the helper keeps its kind, on the new key"
            );
        }
        assert_eq!(validate_geometric(&body, tol()), Ok(()));
    }

    /// **The describing door takes a shared chart too**, so a `Shared`
    /// swap the keys-only door refuses has a way through: the cap onto
    /// a key the body holds with the cap's own payload. The keys-only
    /// door names the four rim edges; the describing door, handed their
    /// carried descriptions, lands them on the shared key, reaps the
    /// cap's old key and leaves the body valid at rest.
    #[test]
    fn a_shared_chart_the_keys_only_door_refuses_goes_through_the_describing_door() {
        let (mut body, top) = brick_and_top();
        let sense = sense(&body, top);
        let old = surf(&body, top);
        let named = edges_naming(&body, old);
        let shared = body.add_surface(cap_at(&body, top, 0.0));
        assert_eq!(
            body.clone()
                .set_face_surface(top, FaceSurface::Shared { key: shared, sense }),
            Err(EulerOpError::RechartStrandsDescriptions {
                door: RechartDoor::SetFaceSurface,
                edges: named.clone()
            }),
        );
        let charts = vec![Rechart::shared(shared, top, sense)];
        let specs = body.carried_redescriptions(&charts).unwrap();
        assert_eq!(
            body.set_face_surfaces_describing(charts, &specs, tol()),
            Ok(vec![shared])
        );
        assert_eq!(surf(&body, top), shared);
        assert_eq!(edges_naming(&body, shared), named);
        assert!(body.get_surface(old).is_none(), "the old cap is reaped");
        assert_eq!(validate_geometric(&body, tol()), Ok(()));
    }

    /// **A swap off the face's own boundary is refused by both doors**
    /// (the review's C1 witness, PR 3580). The inlay's edges are images
    /// in the cap's chart, which the cap still wears, so no edge is
    /// stranded by moving the membrane alone. Onto a plane four units
    /// above, or onto the front face's key, no edge names the key the
    /// membrane would wear: the keys-only door refuses, naming all four,
    /// and the describing door refuses, naming the membrane's first
    /// vertex, each with the body untouched. Through the test-only door
    /// the swap lands, and tier 3 reports the membrane's residuals at
    /// rest.
    #[test]
    fn a_move_off_the_faces_own_boundary_is_refused_by_both_doors() {
        let (mut body, _, membrane) = brick_with_inlay();
        assert_eq!(validate_geometric(&body, tol()), Ok(()));
        let sense = sense(&body, membrane);
        let front = face_at(&body, 1, 0.0);
        let far = plane_moved(&body, membrane, Vec3::new(0.0, 0.0, 4.0));
        let rim = edges_of_face(&body, membrane);
        assert_eq!(rim.len(), 4);
        let first_vertex = {
            let LoopBoundary::Cycle { first } = body
                .get_loop(body.get_face(membrane).unwrap().outer)
                .unwrap()
                .boundary
            else {
                panic!("the membrane's loop is a cycle")
            };
            body.get_half_edge(first).unwrap().start
        };

        for (plain, chart) in [
            (
                FaceSurface::New {
                    surface: far.clone(),
                    sense,
                },
                Rechart::new(far.clone(), membrane, sense),
            ),
            (
                FaceSurface::Shared {
                    key: surf(&body, front),
                    sense,
                },
                Rechart::shared(surf(&body, front), membrane, sense),
            ),
        ] {
            let mut unvouched = body.clone();
            // Lifts RechartUnvouched: tier 3's verdict on the membrane off its own boundary is the row.
            unvouched
                .set_face_surface_stranding_for_tests(membrane, plain.clone())
                .unwrap();
            let at_rest = kinds(&unvouched);
            assert!(
                at_rest.contains(&"PlanarFaceResidual".to_string())
                    && at_rest.contains(&"PlanarBoundaryResidual".to_string())
                    && !at_rest.contains(&"DescriptionNotAdjacent".to_string()),
                "{at_rest:?}"
            );
            assert_err_deep_unchanged(
                &mut body,
                &EulerOpError::RechartUnvouched {
                    door: RechartDoor::SetFaceSurface,
                    face: membrane,
                    edges: rim.clone(),
                    chord: false,
                },
                |b| b.set_face_surface(membrane, plain).unwrap_err(),
            );
            assert_err_deep_unchanged(
                &mut body,
                &EulerOpError::RechartOffBoundary {
                    face: membrane,
                    on: EntityId::Vertex(first_vertex),
                },
                |b| {
                    b.set_face_surfaces_describing(vec![chart], &[], tol())
                        .unwrap_err()
                },
            );
        }
    }

    /// **The keys-only door moves a face onto the key its edges name,
    /// and onto no other.** The inlay's membrane, moved by the
    /// describing door onto a copy of the cap's plane, is bounded by
    /// four images in the cap's chart. The keys-only door takes it back
    /// onto the cap's key, which all four name, and the body is valid
    /// at rest. Onto another copy of the same plane it refuses, naming
    /// all four, since a minted key is one no edge names.
    #[test]
    fn the_keys_only_door_moves_a_face_onto_the_key_its_edges_name() {
        let (mut body, top, membrane) = brick_with_inlay();
        let sense = sense(&body, membrane);
        let cap = surf(&body, top);
        let rim = edges_of_face(&body, membrane);
        let [copy] = body
            .set_face_surfaces_describing(
                vec![Rechart::new(cap_at(&body, top, 0.0), membrane, sense)],
                &[],
                tol(),
            )
            .unwrap()[..]
        else {
            panic!("one chart, one key")
        };
        assert_ne!(copy, cap);
        assert!(
            rim.iter().all(|&e| matches!(
                description(&body, e),
                EdgeDescription::Chart(c) if c.surface == cap
            )),
            "the four still name the cap"
        );
        assert_eq!(validate_geometric(&body, tol()), Ok(()));

        assert_err_deep_unchanged(
            &mut body,
            &EulerOpError::RechartUnvouched {
                door: RechartDoor::SetFaceSurface,
                face: membrane,
                edges: rim.clone(),
                chord: false,
            },
            |b| {
                let again = cap_at(b, top, 0.0);
                b.set_face_surface(
                    membrane,
                    FaceSurface::New {
                        surface: again,
                        sense,
                    },
                )
                .unwrap_err()
            },
        );
        assert_eq!(
            body.set_face_surface(membrane, FaceSurface::Shared { key: cap, sense }),
            Ok(cap)
        );
        assert!(body.get_surface(copy).is_none(), "the copy is reaped");
        assert_eq!(validate_geometric(&body, tol()), Ok(()));
    }

    /// **A scaffold vouches for nothing and is not asked.** The inlay
    /// with one rim edge re-described as a scaffold: the swap off the
    /// cap's plane refuses, naming the three certified edges and not
    /// the scaffold. With all four scaffolds, nothing is certified on
    /// any chart and the keys-only door takes the swap.
    #[test]
    fn a_scaffold_on_the_face_vouches_for_nothing_and_is_not_asked() {
        let (mut body, _, membrane) = brick_with_inlay();
        let sense = sense(&body, membrane);
        let far = plane_moved(&body, membrane, Vec3::new(0.0, 0.0, 4.0));
        let swap = || FaceSurface::New {
            surface: far.clone(),
            sense,
        };
        let rim = edges_of_face(&body, membrane);
        let scaffold = |b: &mut Body<f64>, edge: EdgeKey| {
            let (p0, p1) = b.edge_endpoints(edge).unwrap();
            b.set_edge_curve(edge, EdgeCurveSpec::line_between(p0, p1), tol())
                .unwrap();
        };

        scaffold(&mut body, rim[0]);
        assert_err_deep_unchanged(
            &mut body,
            &EulerOpError::RechartUnvouched {
                door: RechartDoor::SetFaceSurface,
                face: membrane,
                edges: rim[1..].to_vec(),
                chord: false,
            },
            |b| b.set_face_surface(membrane, swap()).unwrap_err(),
        );

        for &edge in &rim[1..] {
            scaffold(&mut body, edge);
        }
        assert!(body.set_face_surface(membrane, swap()).is_ok());
    }

    /// A bilinear NURBS patch over `[-1, 2]²` at height `z`: at `z = 1`,
    /// the cap's plane as a curved-chart payload.
    fn nurbs_plane(z: f64) -> Surface<f64> {
        use geom::surfaces::nurbs::NurbsSurface;
        use geom_core::spline::KnotVector;
        let kv = KnotVector::unit_segment(core::num::NonZeroUsize::new(1).unwrap());
        let control = vec![
            Point3::new(-1.0, -1.0, z),
            Point3::new(-1.0, 2.0, z),
            Point3::new(2.0, -1.0, z),
            Point3::new(2.0, 2.0, z),
        ];
        Surface::Nurbs(std::sync::Arc::new(
            NurbsSurface::new(kv.clone(), kv, control, vec![1.0; 4]).unwrap(),
        ))
    }

    /// **On a curved chart the keys-only door refuses, and the
    /// describing door takes the swap whether or not the boundary lies
    /// on it** (adopted from the PR 3598 review's
    /// `offr_p1_curved_swaps`). The membrane onto a patch that is the
    /// cap's plane, onto the same patch raised to z = 5, and onto a
    /// cylinder off the brick: the keys-only door refuses each, naming
    /// all four rim edges, with the body untouched. The describing door,
    /// handed no re-descriptions, returns `Ok` on all three. The two off
    /// the boundary are #638's residual
    /// (`work/restfront/validate-tier3-curved-boundary-containment`):
    /// once a curved chart's containment is asked, their `Ok` is what
    /// this row reports.
    #[test]
    fn a_curved_swap_is_refused_keys_only_and_taken_unchecked_by_the_describing_door() {
        let cylinder = Surface::Cylinder {
            origin: Point3::new(10.0, 10.0, 10.0),
            axis: Vec3::unit_z(),
            radius: 0.5,
            u_ref: Vec3::unit_x(),
        };
        for (label, surface, on_boundary) in [
            ("the patch through the cap", nurbs_plane(1.0), true),
            ("the patch at z = 5", nurbs_plane(5.0), false),
            ("the cylinder off the brick", cylinder, false),
        ] {
            let (mut body, _, membrane) = brick_with_inlay();
            let sense = sense(&body, membrane);
            let rim = edges_of_face(&body, membrane);
            assert_err_deep_unchanged(
                &mut body,
                &EulerOpError::RechartUnvouched {
                    door: RechartDoor::SetFaceSurface,
                    face: membrane,
                    edges: rim,
                    chord: false,
                },
                |b| {
                    b.set_face_surface(
                        membrane,
                        FaceSurface::New {
                            surface: surface.clone(),
                            sense,
                        },
                    )
                    .unwrap_err()
                },
            );
            let got = body.set_face_surfaces_describing(
                vec![Rechart::new(surface, membrane, sense)],
                &[],
                tol(),
            );
            if on_boundary {
                assert!(
                    got.is_ok(),
                    "{label}: a curved swap onto the boundary: {got:?}"
                );
            } else {
                assert!(
                    got.is_ok(),
                    "{label}: the describing door no longer takes a curved swap off the \
                     boundary unchecked, so #638's residual has moved and this row with it: \
                     {got:?}"
                );
            }
        }
    }

    /// **Both keys-only re-chart refusals end in the chart as their
    /// lever** (D4 ¶1 (i)), each on a real raise: the cap onto a copy
    /// of its plane strands its four rim intersections, and the inlay's
    /// membrane onto a plane four units up is vouched for by none of
    /// its four edges.
    #[test]
    fn the_keys_only_rechart_refusals_end_in_the_chart_as_their_lever() {
        let (mut body, top) = brick_and_top();
        let swap = FaceSurface::New {
            surface: cap_at(&body, top, 0.0),
            sense: sense(&body, top),
        };
        let strands = body.set_face_surface(top, swap).unwrap_err();
        assert!(
            matches!(strands, EulerOpError::RechartStrandsDescriptions { .. }),
            "{strands:?}"
        );
        let (mut body, _, membrane) = brick_with_inlay();
        let swap = FaceSurface::New {
            surface: plane_moved(&body, membrane, Vec3::new(0.0, 0.0, 4.0)),
            sense: sense(&body, membrane),
        };
        let unvouched = body.set_face_surface(membrane, swap).unwrap_err();
        assert!(
            matches!(unvouched, EulerOpError::RechartUnvouched { .. }),
            "{unvouched:?}"
        );
        for (err, lever) in [
            (
                strands,
                "Recourse: leave the face on the chart those edges name, or re-describe them \
                 on the chart it moves onto (set_face_surfaces_describing takes their \
                 re-descriptions under a band, and carried_redescriptions states the stored \
                 ones there)",
            ),
            (
                unvouched,
                "Recourse: move the face onto a chart its certified edges name, or re-describe \
                 them on the new chart (set_face_surfaces_describing certifies each \
                 re-description it is handed against that chart)",
            ),
        ] {
            let text = err.to_string();
            assert!(text.ends_with(lever), "{text}");
        }
    }

    /// Every vertex at `y == 0` moved to `y = -d`, and at `z == 1` to
    /// `z = 1 + d`: the brick's front and top pushed out by `d`, their
    /// charts not yet moved.
    fn push_out_top_and_front(body: &mut Body<f64>, d: f64) {
        let vertices: Vec<_> = body.vertices().map(|(k, _)| k).collect();
        for v in vertices {
            let mut p = *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
            if p.y == 0.0 {
                p.y = -d;
            }
            if p.z == 1.0 {
                p.z = 1.0 + d;
            }
            body.move_vertices(&[v], p).unwrap();
        }
    }

    /// `edge` as the line between its endpoints, described as the
    /// intersection of the surfaces its faces wear now.
    fn intersection_line(body: &Body<f64>, edge: EdgeKey) -> EdgeCurveSpec<f64> {
        let (p0, p1) = body.edge_endpoints(edge).unwrap();
        let [plus, minus] = faces_of(body, edge);
        let mut spec = EdgeCurveSpec::line_between(p0, p1);
        spec.description = EdgeDescriptionSpec::Intersection {
            s1: surf(body, plus),
            s2: surf(body, minus),
            witness: p0.lerp(p1, 0.5),
        };
        spec
    }

    /// **Two moving charts move in one call, each face onto its own.**
    /// The brick's top and front pushed out by `d` (after the review's
    /// C3 probe, PR 3580): one call with both charts and every edge of
    /// either face lands each face on its own chart's plane and returns
    /// the keys in `charts` order, whichever order the charts come in.
    /// One chart alone refuses, since the edge the two faces share
    /// certifies on neither mixed pair. Two single-chart calls reach the
    /// same place only through a first call that describes that edge
    /// as a scaffold, and tier 3 refuses that intermediate at rest.
    #[test]
    fn two_moving_charts_move_in_one_call_each_face_onto_its_own() {
        let d = 0.25;
        let mut body = unit_brick();
        let (top, front) = (face_at(&body, 2, 1.0), face_at(&body, 1, 0.0));
        let top_plane = cap_at(&body, top, d);
        let front_plane = plane_moved(&body, front, Vec3::new(0.0, -d, 0.0));
        let (st, sf) = (sense(&body, top), sense(&body, front));
        push_out_top_and_front(&mut body, d);
        // The edges on neither face stay between the charts they were
        // on, and are restated at the endpoints the push gave them.
        let unmoved: Vec<EdgeKey> = body
            .edges()
            .map(|(k, _)| k)
            .filter(|&e| !faces_of(&body, e).iter().any(|f| [top, front].contains(f)))
            .collect();
        for e in unmoved {
            body.set_edge_curve(e, intersection_line(&body, e), tol())
                .unwrap();
        }
        let shared = edges_of_face(&body, top)
            .into_iter()
            .find(|e| faces_of(&body, *e).contains(&front))
            .unwrap();
        let specs_of = |b: &Body<f64>, face: FaceKey| -> Vec<_> {
            edges_of_face(b, face)
                .into_iter()
                .map(|e| (e, intersection_line(b, e)))
                .collect()
        };
        let mut every = specs_of(&body, top);
        every.extend(
            specs_of(&body, front)
                .into_iter()
                .filter(|(e, _)| *e != shared),
        );
        let top_chart = || Rechart::new(top_plane.clone(), top, st);
        let front_chart = || Rechart::new(front_plane.clone(), front, sf);

        for flipped in [false, true] {
            let mut both = body.clone();
            let charts = if flipped {
                vec![front_chart(), top_chart()]
            } else {
                vec![top_chart(), front_chart()]
            };
            let keys = both
                .set_face_surfaces_describing(charts, &every, tol())
                .unwrap();
            let (top_key, front_key) = if flipped {
                (keys[1], keys[0])
            } else {
                (keys[0], keys[1])
            };
            assert_eq!(surf(&both, top), top_key, "flipped={flipped}");
            assert_eq!(surf(&both, front), front_key, "flipped={flipped}");
            assert!(
                matches!(both.get_surface(top_key), Some(Surface::Plane { origin, .. }) if origin.z == 1.0 + d),
                "the top wears the top's plane"
            );
            assert!(
                matches!(both.get_surface(front_key), Some(Surface::Plane { origin, .. }) if origin.y == -d),
                "the front wears the front's plane"
            );
            assert_eq!(validate_geometric(&both, tol()), Ok(()));
        }

        for (chart, face) in [(top_chart(), top), (front_chart(), front)] {
            let err = body
                .clone()
                .set_face_surfaces_describing(vec![chart], &specs_of(&body, face), tol())
                .unwrap_err();
            assert!(
                matches!(err, EulerOpError::RechartFalsifies { .. }),
                "one chart alone: {err:?}"
            );
        }

        let mut seq = body.clone();
        let (p0, p1) = seq.edge_endpoints(shared).unwrap();
        let first: Vec<_> = specs_of(&seq, top)
            .into_iter()
            .map(|(e, s)| {
                (
                    e,
                    if e == shared {
                        EdgeCurveSpec::line_between(p0, p1)
                    } else {
                        s
                    },
                )
            })
            .collect();
        seq.set_face_surfaces_describing(vec![top_chart()], &first, tol())
            .unwrap();
        assert!(
            kinds(&seq).contains(&"ScaffoldAtRest".to_string()),
            "{:?}",
            kinds(&seq)
        );
        let second = specs_of(&seq, front);
        seq.set_face_surfaces_describing(vec![front_chart()], &second, tol())
            .unwrap();
        assert_eq!(validate_geometric(&seq, tol()), Ok(()));
    }

    /// **Where both faces wore a key and move apart, a spec naming it
    /// names the `he_plus` side's chart.** The inlay's membrane and the
    /// cap both wear the cap's key; each moves onto a chart of its own
    /// with the cap's payload, and every edge the move strands is
    /// handed its carried description. Each inlay edge lands as an
    /// image in the chart of the face its `he_plus` half is on.
    #[test]
    fn a_key_both_faces_wore_names_the_he_plus_sides_chart() {
        let (mut body, top, membrane) = brick_with_inlay();
        let plane = cap_at(&body, top, 0.0);
        let charts = vec![
            Rechart::new(plane.clone(), top, sense(&body, top)),
            Rechart::new(plane, membrane, sense(&body, membrane)),
        ];
        let inlay = edges_of_face(&body, membrane);
        let specs = body.carried_redescriptions(&charts).unwrap();
        for e in &inlay {
            assert!(specs.iter().any(|(s, _)| s == e), "{e:?} is carried");
        }
        body.set_face_surfaces_describing(charts, &specs, tol())
            .unwrap();
        for e in inlay {
            let [plus, _] = faces_of(&body, e);
            assert!(
                matches!(description(&body, e), EdgeDescription::Chart(c) if c.surface == surf(&body, plus)),
                "{e:?} names its he_plus side's chart"
            );
        }
        assert_eq!(validate_geometric(&body, tol()), Ok(()));
    }

    /// The describing door's argument refusals, each with the body
    /// untouched: a face moved twice (across charts and within one), an
    /// edge re-described twice, a stale face, a stale shared key, a
    /// sense against the face's own chart (shared onto its own key), a
    /// stale edge, a listed spec
    /// not adjacent on the moved charts, and a listed null edge. After
    /// the review's C3 atomicity rows (PR 3580).
    #[test]
    fn the_describing_doors_argument_refusals_write_nothing() {
        let (mut body, top) = brick_and_top();
        let sense = sense(&body, top);
        let rim = edges_naming(&body, surf(&body, top));
        let edge = rim[0];
        let cap = cap_at(&body, top, 0.0);
        let own_key = surf(&body, top);
        let chart = || Rechart::new(cap.clone(), top, sense);
        let tol = tol();
        let refuses = |b: &mut Body<f64>,
                       charts: Vec<Rechart<f64>>,
                       specs: &[(EdgeKey, EdgeCurveSpec<f64>)]| {
            b.set_face_surfaces_describing(charts, specs, tol)
                .unwrap_err()
        };

        let moved_twice = EulerOpError::FaceMovedTwice { face: top };
        assert_err_deep_unchanged(&mut body, &moved_twice, |b| {
            refuses(b, vec![chart(), chart()], &[])
        });
        assert_err_deep_unchanged(&mut body, &moved_twice, |b| {
            refuses(b, vec![chart().with(top, sense)], &[])
        });
        let twice = vec![(edge, restated(&body, edge)), (edge, restated(&body, edge))];
        assert_err_deep_unchanged(
            &mut body,
            &EulerOpError::DuplicateRedescription { edge },
            |b| refuses(b, vec![chart()], &twice),
        );
        assert_err_deep_unchanged(
            &mut body,
            &EulerOpError::StaleKey {
                key: EntityId::Face(FaceKey::default()),
            },
            |b| {
                refuses(
                    b,
                    vec![Rechart::new(cap.clone(), FaceKey::default(), sense)],
                    &[],
                )
            },
        );
        assert_err_deep_unchanged(
            &mut body,
            &EulerOpError::StaleGeometry {
                key: crate::GeomRef::Surface(SurfaceKey::default()),
            },
            |b| {
                refuses(
                    b,
                    vec![Rechart::shared(SurfaceKey::default(), top, sense)],
                    &[],
                )
            },
        );
        assert_err_deep_unchanged(
            &mut body,
            &EulerOpError::SenseContradictsChart {
                face: top,
                stated: !sense,
                derived: sense,
            },
            |b| refuses(b, vec![Rechart::shared(own_key, top, !sense)], &[]),
        );
        let stale_edge = vec![(EdgeKey::default(), restated(&body, edge))];
        assert_err_deep_unchanged(
            &mut body,
            &EulerOpError::StaleKey {
                key: EntityId::Edge(EdgeKey::default()),
            },
            |b| refuses(b, vec![chart()], &stale_edge),
        );
        let bottom = surf(&body, face_at(&body, 2, 0.0));
        let front = surf(&body, face_at(&body, 1, 0.0));
        let mut far_pair = restated(&body, edge);
        let EdgeDescriptionSpec::Intersection { s1, s2, .. } = &mut far_pair.description else {
            panic!("the brick's rim is described as intersections")
        };
        (*s1, *s2) = (bottom, front);
        let far_pair = vec![(edge, far_pair)];
        assert_err_deep_unchanged(
            &mut body,
            &EulerOpError::DescriptionNotAdjacent { edge: Some(edge) },
            |b| refuses(b, vec![chart()], &far_pair),
        );

        let v = body.vertices().map(|(k, _)| k).next().unwrap();
        let he = body.get_vertex(v).unwrap().emanating.unwrap();
        let null = body
            .mev_null(
                crate::MevSite::Fan { he1: he, he2: he },
                crate::null::NewVertexSide::Above,
            )
            .unwrap();
        let face = body.face_of_half_edge(null.he_plus).unwrap();
        let (p0, p1) = body.edge_endpoints(null.edge).unwrap();
        let null_spec = vec![(null.edge, EdgeCurveSpec::line_between(p0, p1))];
        let own = body.get_surface(surf(&body, face)).unwrap().clone();
        let face_sense = self::sense(&body, face);
        let curve = body.get_edge(null.edge).unwrap().curve;
        assert_err_deep_unchanged(&mut body, &EulerOpError::NullScaffoldCurve { curve }, |b| {
            refuses(
                b,
                vec![Rechart::new(own.clone(), face, face_sense)],
                &null_spec,
            )
        });
    }

    // -----------------------------------------------------------------
    // The Euler doors' keys-only re-chart refusals (`Body::vouch_move`).
    // -----------------------------------------------------------------

    /// `face`'s outer loop, in cycle order from its first half-edge.
    fn outer_cycle(body: &Body<f64>, face: FaceKey) -> Vec<HalfEdgeKey> {
        let LoopBoundary::Cycle { first } = body
            .get_loop(body.get_face(face).unwrap().outer)
            .unwrap()
            .boundary
        else {
            panic!("the face's outer loop is a cycle")
        };
        body.loop_cycle(first).unwrap()
    }

    fn start_point(body: &Body<f64>, he: HalfEdgeKey) -> Point3<f64> {
        let v = body.get_half_edge(he).unwrap().start;
        *body.get_point(body.get_vertex(v).unwrap().point).unwrap()
    }

    /// The diagonal `mef` across `face`'s four-cornered outer loop: from
    /// its first half-edge's start to the corner two steps on, with a
    /// scaffold chord. Returns the site, the chord, and the two edges of
    /// the run `[he1 .. he2)` the new face takes, in run order.
    fn diagonal(body: &Body<f64>, face: FaceKey) -> (MefSite, EdgeCurveSpec<f64>, Vec<EdgeKey>) {
        let cycle = outer_cycle(body, face);
        assert_eq!(cycle.len(), 4, "a four-cornered loop");
        let (he1, he2) = (cycle[0], cycle[2]);
        let chord = EdgeCurveSpec::line_between(start_point(body, he1), start_point(body, he2));
        let run = cycle[..2]
            .iter()
            .map(|&he| body.get_half_edge(he).unwrap().edge)
            .collect();
        (MefSite::Chords { he1, he2 }, chord, run)
    }

    /// The membrane's loop as a ring of the cap (`kfmrh`), and its four
    /// edges in cycle order.
    fn demoted(body: &mut Body<f64>, top: FaceKey, membrane: FaceKey) -> (LoopKey, Vec<EdgeKey>) {
        let edges: Vec<EdgeKey> = outer_cycle(body, membrane)
            .into_iter()
            .map(|he| body.get_half_edge(he).unwrap().edge)
            .collect();
        let ring = body.kfmrh(top, membrane).unwrap().ring;
        (ring, edges)
    }

    fn tier3_kinds_include(body: &Body<f64>, want: &[&str], label: &str) {
        let at_rest = kinds(body);
        for kind in want {
            assert!(
                at_rest.contains(&(*kind).to_string()),
                "{label}: tier 3 reports {kind} at rest: {at_rest:?}"
            );
        }
    }

    /// **`mef` onto a chart of its own strands the run it moves** (the
    /// row's first witness). Across the brick's top cap between opposite
    /// corners, with `New` holding the cap's own plane, the run's two
    /// rim edges keep `Intersection`s naming the cap's key, which the
    /// new face no longer wears. The door refuses, naming both in run
    /// order, and writes nothing; through the lift the same call lands
    /// and tier 3 reports exactly those two at rest.
    #[test]
    fn mef_across_the_top_cap_onto_a_chart_of_its_own_strands_the_run() {
        let (mut body, top) = brick_and_top();
        let (site, chord, run) = diagonal(&body, top);
        let (cap, sense) = (cap_at(&body, top, 0.0), sense(&body, top));
        let swap = || FaceSurface::New {
            surface: cap.clone(),
            sense,
        };

        let mut lifted = body.clone();
        // Lifts RechartStrandsDescriptions: the stranded run tier 3 reports at rest is the row.
        lifted
            .lifting_rechart_refusals_for_tests(|b| b.mef(site, chord.clone(), swap(), tol()))
            .unwrap();
        let errs = validate_geometric(&lifted, tol()).unwrap_err();
        let mut at_rest: Vec<EdgeKey> = errs
            .iter()
            .filter_map(|e| match e {
                ValidationError::DescriptionNotAdjacent { edge } => Some(*edge),
                _ => None,
            })
            .collect();
        at_rest.sort();
        let mut want = run.clone();
        want.sort();
        assert_eq!(at_rest, want, "tier 3 reports the run at rest: {errs:?}");

        assert_err_deep_unchanged(
            &mut body,
            &EulerOpError::RechartStrandsDescriptions {
                door: RechartDoor::Mef,
                edges: run,
            },
            |b| b.mef(site, chord, swap(), tol()).unwrap_err(),
        );
    }

    /// **`mef` onto a chart its run's edges do not name is refused
    /// unvouched** (the inlay's `mef(Chords)` probe). Across the
    /// membrane between opposite corners onto a plane four units up, a
    /// scaffold chord: the run's two edges are images in the cap's
    /// chart, which the cap still wears, so nothing strands; neither
    /// names the new face's key. The door names both, and not the
    /// scaffold chord, and writes nothing; through the lift the call
    /// lands and tier 3 reports the new face's residuals at rest.
    #[test]
    fn mef_across_the_membrane_onto_a_far_plane_is_refused_unvouched() {
        let (mut body, _, membrane) = brick_with_inlay();
        let (site, chord, run) = diagonal(&body, membrane);
        let far = plane_moved(&body, membrane, Vec3::new(0.0, 0.0, 4.0));
        let sense = sense(&body, membrane);
        let swap = || FaceSurface::New {
            surface: far.clone(),
            sense,
        };

        let mut lifted = body.clone();
        // Lifts RechartUnvouched: tier 3's verdict on the new face off its own boundary is the row.
        lifted
            .lifting_rechart_refusals_for_tests(|b| b.mef(site, chord.clone(), swap(), tol()))
            .unwrap();
        tier3_kinds_include(
            &lifted,
            &["PlanarFaceResidual", "PlanarBoundaryResidual"],
            "mef",
        );
        assert!(!kinds(&lifted).contains(&"DescriptionNotAdjacent".to_string()));

        assert_err_deep_unchanged(
            &mut body,
            &EulerOpError::RechartUnvouched {
                door: RechartDoor::Mef,
                face: membrane,
                edges: run,
                chord: false,
            },
            |b| b.mef(site, chord, swap(), tol()).unwrap_err(),
        );
    }

    /// A cylinder seed with a lone vertex on its rim, and two more
    /// shells whose seeds wear a second key on the same cylinder and a
    /// plane key, so every key is worn: the body, the seed, and the
    /// keys `(cylinder, second cylinder, plane)`, with the plane's
    /// payload.
    fn cylinder_seed() -> (
        Body<f64>,
        crate::euler::MvfsCreated,
        [SurfaceKey; 3],
        Surface<f64>,
    ) {
        let cylinder = Surface::Cylinder {
            origin: Point3::origin(),
            axis: Vec3::unit_z(),
            radius: 1.0,
            u_ref: Vec3::unit_x(),
        };
        let plane = Surface::Plane {
            origin: Point3::origin(),
            normal: Vec3::unit_z(),
            u_ref: Vec3::unit_x(),
        };
        let mut body = Body::<f64>::new();
        let mut wear = |at: Point3<f64>, surface: &Surface<f64>| {
            let seed = body.mvfs(at, true).unwrap();
            let key = body
                .set_face_surface(
                    seed.face,
                    FaceSurface::New {
                        surface: surface.clone(),
                        sense: true,
                    },
                )
                .unwrap();
            (seed, key)
        };
        let (seed, cyl) = wear(Point3::new(1.0, 0.0, 0.0), &cylinder);
        let (_, cyl2) = wear(Point3::new(0.0, 1.0, 0.0), &cylinder);
        let (_, cap) = wear(Point3::new(3.0, 0.0, 0.0), &plane);
        (body, seed, [cyl, cyl2, cap], plane)
    }

    /// The unit rim circle at `z = 0`, described as the intersection of
    /// `s1` and `s2`.
    fn rim(s1: SurfaceKey, s2: SurfaceKey) -> EdgeCurveSpec<f64> {
        EdgeCurveSpec {
            description: EdgeDescriptionSpec::Intersection {
                s1,
                s2,
                witness: Point3::new(-1.0, 0.0, 0.0),
            },
            carrier: geom::Curve3::Circle {
                center: Point3::origin(),
                axis: Vec3::unit_z(),
                radius: 1.0,
                u_ref: Vec3::unit_x(),
            },
            param_start: 0.0,
            param_end: core::f64::consts::TAU,
        }
    }

    /// **`mef`'s chord bounding a face on another key is asked the
    /// adjacency question [`Body::set_edge_curve`] asks, against the two
    /// keys its faces will wear** (one decision, one variant). A lone
    /// vertex on a cylinder seed, its self-loop the rim circle:
    /// described as the cylinder's intersection with the plane, `mef`
    /// mints the cap onto the plane's key (the no-over-refusal half).
    /// Described against a second key on the same cylinder, which the
    /// parent does not wear, `mef` refuses as `set_edge_curve` refuses
    /// the same spec on the edge it minted; and onto a `New` key the
    /// intersection names a key the new face does not wear. Each refusal
    /// writes nothing. After the review's C3 probe (PR 3673).
    #[test]
    fn mefs_chord_is_asked_the_adjacency_question_set_edge_curve_asks() {
        let (mut body, seed, [cyl, cyl2, cap], plane) = cylinder_seed();
        let site = MefSite::Lone {
            r#loop: seed.r#loop,
        };
        let shared = FaceSurface::Shared {
            key: cap,
            sense: true,
        };
        let refused = EulerOpError::DescriptionNotAdjacent { edge: None };

        let mut minted = body.clone();
        let made = minted
            .mef(site, rim(cyl, cap), shared.clone(), tol())
            .unwrap();
        assert_eq!(surf(&minted, made.face), cap);
        assert_err_deep_unchanged(
            &mut minted,
            &EulerOpError::DescriptionNotAdjacent {
                edge: Some(made.edge),
            },
            |b| {
                b.set_edge_curve(made.edge, rim(cyl2, cap), tol())
                    .unwrap_err()
            },
        );

        assert_err_deep_unchanged(&mut body, &refused, |b| {
            b.mef(site, rim(cyl2, cap), shared.clone(), tol())
                .unwrap_err()
        });
        assert_err_deep_unchanged(&mut body, &refused, |b| {
            b.mef(
                site,
                rim(cyl, cap),
                FaceSurface::New {
                    surface: plane.clone(),
                    sense: true,
                },
                tol(),
            )
            .unwrap_err()
        });
    }

    /// **Where the new face keeps the parent's key, `mef` takes a chord
    /// whose description names a key neither face wears, unasked** —
    /// pinned as it stands: the boolean pipeline's chord joins rely on
    /// it (`work/topo/minting-doors-take-a-callers-description-unasked-where-the-new-face-keeps-the-key`).
    /// The rim circle described as the cylinder's intersection with the
    /// plane, under `Inherit`, is `Ok`; and so is the chord across the
    /// described membrane as an image in a second key holding the cap's
    /// plane, which tier 3 reports `DescriptionNotAdjacent` at rest. The
    /// unit that closes this half flips the row. After the review's C3
    /// probe (PR 3673).
    #[test]
    fn under_inherit_mef_takes_a_chord_naming_a_key_neither_face_wears_unasked() {
        let not_adjacent = |body: &Body<f64>| -> Vec<EdgeKey> {
            validate_geometric(body, tol())
                .err()
                .unwrap_or_default()
                .iter()
                .filter_map(|e| match e {
                    ValidationError::DescriptionNotAdjacent { edge } => Some(*edge),
                    _ => None,
                })
                .collect()
        };

        let (mut body, seed, [cyl, _, cap], _) = cylinder_seed();
        let site = MefSite::Lone {
            r#loop: seed.r#loop,
        };
        let made = body.mef(site, rim(cyl, cap), FaceSurface::Inherit, tol());
        assert!(made.is_ok(), "the rim: {made:?}");

        let (mut body, top, membrane) = brick_with_inlay();
        let own = body.add_surface(cap_at(&body, top, 0.0));
        let (site, mut chord, _) = diagonal(&body, membrane);
        chord.description = EdgeDescriptionSpec::chart(own);
        let made = body.mef(site, chord, FaceSurface::Inherit, tol()).unwrap();
        assert_eq!(not_adjacent(&body), vec![made.edge], "the membrane's chord");
    }

    /// **A certified chord that names only the parent's key does not
    /// vouch for a face minted on another** — the chord's two arms of
    /// [`EulerOpError::RechartUnvouched`]. The chord across the
    /// membrane described as an image in the cap's chart is
    /// adjacency-coherent (it names the key the parent keeps), and
    /// names nothing the new face wears four units up. On the scaffold
    /// inlay it is refused alone; on the described inlay with the run's
    /// two edges, which name the cap too. Under `Inherit` it is taken.
    #[test]
    fn a_certified_chord_naming_only_the_parents_key_is_refused_unvouched() {
        for (label, (mut body, top, membrane), run_is_named) in [
            ("scaffold inlay", brick_with_scaffold_inlay(), false),
            ("described inlay", brick_with_inlay(), true),
        ] {
            let (site, mut chord, run) = diagonal(&body, membrane);
            chord.description = EdgeDescriptionSpec::chart(surf(&body, top));
            let far = FaceSurface::New {
                surface: plane_moved(&body, membrane, Vec3::new(0.0, 0.0, 4.0)),
                sense: sense(&body, membrane),
            };
            let edges = if run_is_named { run } else { vec![] };
            let (err, named) = if run_is_named {
                (
                    EulerOpError::RechartUnvouched {
                        door: RechartDoor::Mef,
                        face: membrane,
                        edges: edges.clone(),
                        chord: true,
                    },
                    format!("its certified edges {edges:?} and the chord it mints do not name"),
                )
            } else {
                (
                    EulerOpError::RechartUnvouched {
                        door: RechartDoor::Mef,
                        face: membrane,
                        edges: vec![],
                        chord: true,
                    },
                    "the certified chord it mints does not name".to_string(),
                )
            };
            assert!(err.to_string().contains(&named), "{label}: {err}");
            assert_err_deep_unchanged(&mut body, &err, |b| {
                b.mef(site, chord.clone(), far.clone(), tol()).unwrap_err()
            });
            let made = body.mef(site, chord, FaceSurface::Inherit, tol());
            assert!(made.is_ok(), "{label}: {made:?}");
        }
    }

    /// **`mfkrh` onto a chart the ring's edges do not name is refused at
    /// every door of the family** (the inlay's `mfkrh` probe). The
    /// membrane demoted into the cap (`kfmrh`, one key, so no move),
    /// its ring promoted onto a plane four units up: no edge strands,
    /// none names the new key. `mfkrh`, `mfkrh_minting` and
    /// `mfkrh_plug` (on its placeholder, naming itself) each name all
    /// four, in cycle order, and write
    /// nothing; through the lift `mfkrh` lands and tier 3 reports the
    /// promoted face's residuals at rest.
    #[test]
    fn mfkrh_onto_a_far_plane_is_refused_unvouched_at_every_door() {
        let (mut body, top, membrane) = brick_with_inlay();
        let sense = sense(&body, membrane);
        let far = plane_moved(&body, membrane, Vec3::new(0.0, 0.0, 4.0));
        let (ring, edges) = demoted(&mut body, top, membrane);
        let swap = || FaceSurface::New {
            surface: far.clone(),
            sense,
        };
        let want = EulerOpError::RechartUnvouched {
            door: RechartDoor::Mfkrh,
            face: top,
            edges,
            chord: false,
        };

        let mut lifted = body.clone();
        // Lifts RechartUnvouched: tier 3's verdict on the promoted face off its own boundary is the row.
        lifted
            .lifting_rechart_refusals_for_tests(|b| b.mfkrh(ring, swap()))
            .unwrap();
        tier3_kinds_include(
            &lifted,
            &["PlanarFaceResidual", "PlanarBoundaryResidual"],
            "mfkrh",
        );
        assert!(!kinds(&lifted).contains(&"DescriptionNotAdjacent".to_string()));

        assert_err_deep_unchanged(&mut body, &want, |b| b.mfkrh(ring, swap()).unwrap_err());
        assert_err_deep_unchanged(&mut body, &want, |b| {
            b.mfkrh_minting(ring, swap(), tol()).unwrap_err()
        });
        let EulerOpError::RechartUnvouched { face, edges, .. } = want else {
            unreachable!()
        };
        let want = EulerOpError::RechartUnvouched {
            door: RechartDoor::MfkrhPlug,
            face,
            edges,
            chord: false,
        };
        assert_err_deep_unchanged(&mut body, &want, |b| b.mfkrh_plug(ring, sense).unwrap_err());
    }

    /// **`ring_move` onto a face on a chart the ring's edges do not name
    /// is refused at both doors** (the inlay's `ring_move` probe). The
    /// demoted membrane's ring moved onto the brick's front face: no
    /// edge strands, none names the front's key. Both doors name all
    /// four and write nothing; through the lift the move lands and tier
    /// 3 reports the front's residuals at rest.
    #[test]
    fn ring_move_onto_the_front_face_is_refused_unvouched_at_both_doors() {
        let (mut body, top, membrane) = brick_with_inlay();
        let front = face_at(&body, 1, 0.0);
        let (ring, edges) = demoted(&mut body, top, membrane);
        let want = EulerOpError::RechartUnvouched {
            door: RechartDoor::RingMove,
            face: front,
            edges,
            chord: false,
        };

        let mut lifted = body.clone();
        // Lifts RechartUnvouched: tier 3's verdict on the front face holding a ring off its plane is the row.
        lifted
            .lifting_rechart_refusals_for_tests(|b| b.ring_move(ring, front))
            .unwrap();
        tier3_kinds_include(
            &lifted,
            &["PlanarFaceResidual", "PlanarBoundaryResidual"],
            "ring_move",
        );

        assert_err_deep_unchanged(&mut body, &want, |b| b.ring_move(ring, front).unwrap_err());
        assert_err_deep_unchanged(&mut body, &want, |b| {
            b.ring_move_minting(ring, front, tol()).unwrap_err()
        });
    }

    /// **No over-refusal on scaffolds.** The same three moves on the
    /// inlay before its edges are described: a scaffold carries no
    /// certificate, so it neither strands nor vouches, and every door
    /// takes the move.
    #[test]
    fn the_euler_doors_take_a_move_of_scaffold_edges_onto_any_chart() {
        let (body, top, membrane) = brick_with_scaffold_inlay();
        let far = plane_moved(&body, membrane, Vec3::new(0.0, 0.0, 4.0));
        let sense = sense(&body, membrane);
        let front = face_at(&body, 1, 0.0);

        let mut b = body.clone();
        let (site, chord, _) = diagonal(&b, membrane);
        let made = b.mef(
            site,
            chord,
            FaceSurface::New {
                surface: far.clone(),
                sense,
            },
            tol(),
        );
        assert!(made.is_ok(), "mef: {made:?}");

        let mut b = body.clone();
        let (ring, _) = demoted(&mut b, top, membrane);
        let made = b.mfkrh(
            ring,
            FaceSurface::New {
                surface: far,
                sense,
            },
        );
        assert!(made.is_ok(), "mfkrh: {made:?}");

        let mut b = body;
        let (ring, _) = demoted(&mut b, top, membrane);
        assert_eq!(b.ring_move(ring, front), Ok(()), "ring_move");
    }

    /// **No over-refusal on a certified ring that names its new key.**
    /// The membrane moved onto a key of its own holding the cap's plane,
    /// its four edges re-described as images in that chart (the body is
    /// valid at rest), then demoted into the cap and promoted back onto
    /// that key: every edge names the face it lands on, so `mfkrh`
    /// takes it, and the body is valid at rest again.
    #[test]
    fn mfkrh_takes_a_ring_back_onto_the_key_its_edges_name() {
        let (mut body, top, membrane) = brick_with_inlay();
        let sense = sense(&body, membrane);
        let own = body.add_surface(cap_at(&body, top, 0.0));
        let specs: Vec<(EdgeKey, EdgeCurveSpec<f64>)> = edges_of_face(&body, membrane)
            .into_iter()
            .map(|edge| {
                let mut spec = restated(&body, edge);
                spec.description = EdgeDescriptionSpec::chart(own);
                (edge, spec)
            })
            .collect();
        body.set_face_surfaces_describing(
            vec![Rechart::shared(own, membrane, sense)],
            &specs,
            tol(),
        )
        .unwrap();
        assert_eq!(validate_geometric(&body, tol()), Ok(()));

        let (ring, _) = demoted(&mut body, top, membrane);
        let made = body
            .mfkrh(ring, FaceSurface::Shared { key: own, sense })
            .unwrap();
        assert_eq!(surf(&body, made.face), own);
        assert_eq!(validate_geometric(&body, tol()), Ok(()));
    }

    /// **`kef` between two coplanar faces on distinct keys strands the
    /// remnant, unasked** — the row's `kef` witness, pinned as it stands:
    /// its refusal is not built
    /// (`work/topo/kef-and-kfmrh-across-keys-want-a-describing-door-or-reordered-callers`).
    /// The top cap split along its diagonal, the chord described as an
    /// image in the cap's chart, one half moved onto a key of its own
    /// holding the same plane with its rim re-described there (valid at
    /// rest); then that half killed into the other. `kef` answers `Ok`,
    /// and tier 3 reports the two rim edges it moved, which still name
    /// the dying half's key. The unit that gives `kef` its refusal flips
    /// this row.
    #[test]
    fn kef_between_coplanar_faces_on_distinct_keys_strands_unasked() {
        let (mut body, top) = brick_and_top();
        let (site, chord, _) = diagonal(&body, top);
        let sense = sense(&body, top);
        let split = body.mef(site, chord, FaceSurface::Inherit, tol()).unwrap();
        let cap = surf(&body, top);
        let mut spec = restated(&body, split.edge);
        spec.description = EdgeDescriptionSpec::chart(cap);
        body.set_edge_curve(split.edge, spec, tol()).unwrap();
        let charts = vec![Rechart::new(cap_at(&body, top, 0.0), split.face, sense)];
        let specs = body.carried_redescriptions(&charts).unwrap();
        body.set_face_surfaces_describing(charts, &specs, tol())
            .unwrap();
        assert_eq!(validate_geometric(&body, tol()), Ok(()));
        assert_ne!(surf(&body, split.face), surf(&body, top));

        let mut moved: Vec<EdgeKey> = specs.iter().map(|(e, _)| *e).collect();
        moved.sort();
        assert_eq!(moved.len(), 2, "the half's two rim edges");
        body.kef(split.he_minus).unwrap();
        let errs = validate_geometric(&body, tol()).unwrap_err();
        let mut at_rest: Vec<EdgeKey> = errs
            .iter()
            .filter_map(|e| match e {
                ValidationError::DescriptionNotAdjacent { edge } => Some(*edge),
                _ => None,
            })
            .collect();
        at_rest.sort();
        assert_eq!(at_rest, moved, "{errs:?}");
    }

    /// The 4 × 2 × 2 block with one square through-hole, its faces
    /// described as intersections, and its top face (`z = 2`), which
    /// holds the hole's rim as a ring whose edges name the top's key and
    /// a wall's: a promotion or a move off the top strands every one.
    fn holed_top() -> (Body<f64>, FaceKey, LoopKey) {
        let mut body = crate::test_support_fixtures::holed_block::<f64>(4.0, &[2.0], tol());
        crate::test_support_fixtures::describe_as_intersections(&mut body, tol());
        let top = face_at(&body, 2, 2.0);
        let ring = body.get_face(top).unwrap().rings[0];
        (body, top, ring)
    }

    /// **Each Euler door's refusals end in its own lever** (D4 ¶1 (i)):
    /// a minting door names the chart it mints the face on, a moving
    /// door the face it moves the loop onto, and the plug, whose caller
    /// picks no chart, the door that takes one. Each door's strand text
    /// and unvouched text is raised for real, at that door, and pinned
    /// apart.
    #[test]
    fn the_euler_doors_refusals_end_in_their_own_lever() {
        let mint = "Recourse: mint the face on a chart its certified edges name, or mint it on \
                    its parent's chart and move it with set_face_surfaces_describing, which \
                    certifies each re-description it is handed against the new chart";
        let mint_strand = "Recourse: mint the face on the chart those edges name, then move it \
                           onto its own (set_face_surfaces_describing takes their \
                           re-descriptions under a band, and carried_redescriptions states the \
                           stored ones there)";
        let moving = "Recourse: move the loop onto a face on a chart its edges name";
        let moving_strand = "Recourse: move the loop onto a face on the chart its edges name";
        let plug = "Recourse: promote the ring with mfkrh onto a chart its certified edges name";
        let plug_strand = "Recourse: promote the ring with mfkrh onto the chart those edges name";

        let (mut body, top) = brick_and_top();
        let (site, chord, _) = diagonal(&body, top);
        let new = FaceSurface::New {
            surface: cap_at(&body, top, 0.0),
            sense: sense(&body, top),
        };
        let mef_strands = body.mef(site, chord, new, tol()).unwrap_err();

        let (mut body, _, membrane) = brick_with_inlay();
        let (site, chord, _) = diagonal(&body, membrane);
        let far = FaceSurface::New {
            surface: plane_moved(&body, membrane, Vec3::new(0.0, 0.0, 4.0)),
            sense: true,
        };
        let mef = body.mef(site, chord, far.clone(), tol()).unwrap_err();

        let (mut body, top, membrane) = brick_with_inlay();
        let front = face_at(&body, 1, 0.0);
        let (ring, _) = demoted(&mut body, top, membrane);
        let mfkrh = body.mfkrh(ring, far.clone()).unwrap_err();
        let mfkrh_plug = body.mfkrh_plug(ring, true).unwrap_err();
        let ring_move = body.ring_move(ring, front).unwrap_err();

        let (mut body, top, ring) = holed_top();
        let bottom = face_at(&body, 2, 0.0);
        let lid = FaceSurface::New {
            surface: cap_at(&body, top, 1.0),
            sense: sense(&body, top),
        };
        let mfkrh_strands = body.mfkrh(ring, lid).unwrap_err();
        let plug_strands = body.mfkrh_plug(ring, true).unwrap_err();
        let ring_move_strands = body.ring_move(ring, bottom).unwrap_err();

        for (label, err, door, strands, lever) in [
            ("mef", mef_strands, RechartDoor::Mef, true, mint_strand),
            ("mef", mef, RechartDoor::Mef, false, mint),
            (
                "mfkrh",
                mfkrh_strands,
                RechartDoor::Mfkrh,
                true,
                mint_strand,
            ),
            ("mfkrh", mfkrh, RechartDoor::Mfkrh, false, mint),
            (
                "mfkrh_plug",
                plug_strands,
                RechartDoor::MfkrhPlug,
                true,
                plug_strand,
            ),
            (
                "mfkrh_plug",
                mfkrh_plug,
                RechartDoor::MfkrhPlug,
                false,
                plug,
            ),
            (
                "ring_move",
                ring_move_strands,
                RechartDoor::RingMove,
                true,
                moving_strand,
            ),
            ("ring_move", ring_move, RechartDoor::RingMove, false, moving),
        ] {
            let raised = match err {
                EulerOpError::RechartStrandsDescriptions { door, .. } => (door, true),
                EulerOpError::RechartUnvouched { door, .. } => (door, false),
                _ => panic!("{label}: a re-chart refusal, got {err:?}"),
            };
            assert_eq!(raised, (door, strands), "{label}: {err:?}");
            let text = err.to_string();
            assert!(text.starts_with(&format!("{label}: ")), "{label}: {text}");
            assert!(text.ends_with(lever), "{label}: {text}");
        }
    }

    /// **`mfkrh_plug`'s lever is the door that takes a chart.** The plug
    /// refuses a ring its placeholder strands (the hole's rim, whose
    /// edges name the top) and one it cannot vouch for (the demoted
    /// membrane, whose edges are images in the cap's chart), writing
    /// nothing; the move each refusal names, `mfkrh` onto the chart the
    /// ring's edges name — here the demoting face's own, `Inherit` — is
    /// taken.
    #[test]
    fn mfkrh_plugs_refusals_name_mfkrh_onto_the_chart_the_ring_names() {
        let (mut body, _, ring) = holed_top();
        let stranded = body.mfkrh_plug(ring, true).unwrap_err();
        assert!(
            matches!(
                stranded,
                EulerOpError::RechartStrandsDescriptions {
                    door: RechartDoor::MfkrhPlug,
                    ..
                }
            ),
            "{stranded:?}"
        );
        assert_err_deep_unchanged(&mut body, &stranded, |b| {
            b.mfkrh_plug(ring, true).unwrap_err()
        });
        let made = body.mfkrh(ring, FaceSurface::Inherit);
        assert!(made.is_ok(), "the hole's rim: {made:?}");

        let (mut body, top, membrane) = brick_with_inlay();
        let (ring, edges) = demoted(&mut body, top, membrane);
        assert_err_deep_unchanged(
            &mut body,
            &EulerOpError::RechartUnvouched {
                door: RechartDoor::MfkrhPlug,
                face: top,
                edges,
                chord: false,
            },
            |b| b.mfkrh_plug(ring, true).unwrap_err(),
        );
        let made = body.mfkrh(ring, FaceSurface::Inherit);
        assert!(made.is_ok(), "the membrane: {made:?}");
    }
}
