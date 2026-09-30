//! Geometry attachment setters: certified re-attachment of face
//! surfaces and edge curves on an existing body (M2 PR 3).
//!
//! The Euler operators attach geometry at mint time (op parameters —
//! the preferred path); these two setters cover the attachments that
//! **cannot** happen at mint time, both consequences of construction
//! order:
//!
//! - [`Body::set_face_surface`] — `mvfs`'s seed face is born before its
//!   eventual cap surface exists (the seed's surface is the honest
//!   `Surface::Nurbs` "not yet described" state); the sweep attaches
//!   the real plane once profile data determines it.
//! - [`Body::set_edge_curve`] — an intrinsic
//!   ([`geom_brep::EdgeDescription::Intersection`]) description references
//!   its two adjacent faces' surfaces *by key*, and a swept edge is
//!   necessarily minted **before** the side faces it will bound
//!   (struts precede `mef`): the sweep mints with a conventional
//!   description and upgrades to the intrinsic one here, once both
//!   surfaces exist (the prefer-intrinsic rule lands at rest, D2).
//!
//! Both setters are **certified mutations**: the same D4 ¶2/¶3 gates as
//! the operators (`EdgeCurve::certify`; typed errors, body untouched on
//! failure), plus — since both adjacent faces exist by the time an
//! edge is re-described — the **description-adjacency coherence** check
//! the mint-time gate cannot run: an `Intersection`'s two surfaces must
//! be exactly the edge's two faces' surfaces, and a `Seam`'s surface
//! must be on both sides ([`EulerOpError::DescriptionNotAdjacent`]).
//! Neither setter is an Euler operator (no topology changes — the D1
//! "exclusively Euler" rule governs *topology*); both preserve tier 1
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
use geom_core::{Band, Decide, Point3, Real};

use crate::body::Body;
use crate::entity::{EdgeKey, EntityId, FaceKey, GeomRef, HalfEdgeKey, LoopKey};
use crate::euler::{EulerOpError, FaceSurface, ParentSide};
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
    /// No residual certification runs here: a face's surface has no
    /// independent cache to certify — the *edges'* certifications and
    /// the tier-3 planar-face residuals are what tie a face's surface
    /// to the body's points, and the residuals are tier 3's to re-check
    /// at rest.
    ///
    /// **An edge described against the surface the face leaves is
    /// refused, never stranded.** A description names its surfaces by
    /// key — an `Intersection`'s operands, a `Chart`'s chart — so a
    /// swap leaves every description that named the face's old key
    /// naming a surface the face no longer wears. Where such a
    /// description is adjacency-coherent now and the swap would make it
    /// not (tier 3's `DescriptionNotAdjacent` at rest), the door
    /// refuses before mutating, naming every such edge. It takes no
    /// band, so it cannot ask whether an edge would certify on the new
    /// chart and carries none; [`Body::set_face_surfaces_describing`]
    /// takes a band and the re-descriptions, and carries the rest.
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
    /// [`EulerOpError::RechartStrandsDescriptions`] (`StaleKey` /
    /// `StaleGeometry` where a key the walk over the edges follows does
    /// not resolve). The body is untouched on `Err`.
    pub fn set_face_surface(
        &mut self,
        face: FaceKey,
        surface: FaceSurface<T>,
    ) -> Result<SurfaceKey, EulerOpError> {
        self.set_face_surface_gated(face, surface, true)
    }

    /// **Failure-injection door** (test builds only: this crate's own
    /// tests, `test-support` and `sweep-testing`): [`Body::set_face_surface`] with its stranding refusal
    /// taken out, so a swap may leave edges described against the
    /// surface the face leaves — the state tier 3 reports at rest as
    /// `DescriptionNotAdjacent`. It is for rows that need a face on a
    /// chart its boundary does not lie on (a tier-3 verdict on the
    /// surface itself, a pcurve-row decision taken with no geometry
    /// behind it); no constructor or door produces that body. Every
    /// other precondition and every write is the real door's.
    ///
    /// # Errors
    ///
    /// [`Body::set_face_surface`]'s, but for
    /// [`EulerOpError::RechartStrandsDescriptions`].
    #[cfg(any(test, feature = "test-support", feature = "sweep-testing"))]
    #[doc(hidden)]
    pub fn set_face_surface_stranding_for_tests(
        &mut self,
        face: FaceKey,
        surface: FaceSurface<T>,
    ) -> Result<SurfaceKey, EulerOpError> {
        self.set_face_surface_gated(face, surface, false)
    }

    /// [`Body::set_face_surface`], its stranding refusal asked iff
    /// `refuse_stranding`.
    fn set_face_surface_gated(
        &mut self,
        face: FaceKey,
        surface: FaceSurface<T>,
        refuse_stranding: bool,
    ) -> Result<SurfaceKey, EulerOpError> {
        let face_data = self.get_face(face).ok_or(EulerOpError::StaleKey {
            key: EntityId::Face(face),
        })?;
        let old = face_data.surface;
        let resolved =
            self.resolve_face_surface(&surface, face, (old, face_data.sense), ParentSide::With)?;
        let after = match surface {
            FaceSurface::Inherit => Slot::Kept(old),
            FaceSurface::New { .. } => Slot::Minted(0),
            FaceSurface::Shared { key, .. } => Slot::Kept(key),
        };
        if refuse_stranding && after != Slot::Kept(old) {
            let moved = |f: FaceKey| (f == face).then_some(after);
            let stranded = self.rechart_edges(moved, false)?.stranded;
            if !stranded.is_empty() {
                return Err(EulerOpError::RechartStrandsDescriptions {
                    edges: stranded.into_iter().map(|(e, _)| e).collect(),
                });
            }
        }

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

    /// Re-charts faces with their edges' re-descriptions:
    /// [`Body::set_face_surface`]'s swap for every face of every
    /// [`Rechart`], taking a band and, for any edge whose description
    /// the move would leave false, the spec that describes it on the
    /// moved charts. Returns each chart's minted key, in `charts` order.
    ///
    /// The describing sibling of the keys-only surface setter, as
    /// [`Body::kev_describing`] is [`Body::kev`]'s: faces, descriptions
    /// and the band arrive together, nothing is written until everything
    /// has certified, and no state between the swaps and the
    /// re-descriptions is ever observable. It moves several charts in
    /// one call because an edge between two moving charts certifies on
    /// neither pair of mixed charts, so one chart at a time has no order
    /// that works.
    ///
    /// **A key an edge's face wore stands, in that edge's description,
    /// for the chart the face moves onto** — unless one of the edge's
    /// faces still wears it after the move, when it stands for itself.
    /// A `New` surface has no key before it is minted, so a spec names
    /// the chart it is stated in by the key the face wears now; where
    /// both faces wore the key and move apart, the `he_plus` side's
    /// chart is the one named.
    ///
    /// - **A listed edge** is certified with its spec against its
    ///   current endpoints and the moved charts, adjacency coherence
    ///   against its faces' moved charts included, and its curve is
    ///   replaced by fresh insertion. Any certified edge may be listed;
    ///   a null edge may not (its first description is
    ///   [`Body::set_edge_curve`]'s, which mints its rows).
    /// - **An unlisted edge on a moved face** whose description names a
    ///   key its moved face wore is carried: its stored description
    ///   restated on that face's new chart (a chart image re-derived,
    ///   unless the new chart shares the old one's payload) and
    ///   certified there under the band. One the move would leave
    ///   incoherent however its keys are read is refused, every one
    ///   named. An edge already incoherent before the move is left as
    ///   found.
    ///
    /// Each face's sense is [`FaceSurface::New`]'s rule, per face; its
    /// pcurve rows are [`Body::set_face_surface`]'s (kept on the same
    /// chart, dropped on another); a re-described edge's rows are
    /// [`Body::set_edge_curve`]'s.
    ///
    /// Minting order (D9): the charts' surfaces in `charts` order; then
    /// the listed edges' curves in list order, then the carried edges'
    /// in edge-arena order.
    ///
    /// # Precondition check order
    ///
    /// `tol` builds a band ([`EulerOpError::Certification`]). Per chart
    /// in order: it has a face ([`EulerOpError::EmptyRechart`]); per
    /// face in order, the face resolves ([`EulerOpError::StaleKey`]),
    /// was not listed before ([`EulerOpError::FaceMovedTwice`]) and
    /// states a sense its chart admits
    /// ([`EulerOpError::SenseContradictsChart`]). Per listed edge in
    /// order: it resolves (`StaleKey`), was not listed before
    /// ([`EulerOpError::DuplicateRedescription`]), is not a null edge
    /// ([`EulerOpError::NullScaffoldCurve`]), its spec is
    /// adjacency-coherent on the moved charts
    /// ([`EulerOpError::DescriptionNotAdjacent`]) and certifies at its
    /// endpoints ([`EulerOpError::RechartFalsifies`]). Then the unlisted
    /// edges: none is stranded
    /// ([`EulerOpError::RechartStrandsDescriptions`]), and each carried
    /// one certifies, in edge-arena order (`RechartFalsifies`).
    /// `StaleKey` / [`EulerOpError::StaleGeometry`] where a key the walk
    /// follows does not resolve.
    ///
    /// # Errors
    ///
    /// The first failing precondition above; the body is untouched on
    /// `Err`.
    pub fn set_face_surfaces_describing(
        &mut self,
        charts: Vec<Rechart<T>>,
        redescriptions: Vec<(EdgeKey, EdgeCurveSpec<T>)>,
        tol: Tol,
    ) -> Result<Vec<SurfaceKey>, EulerOpError> {
        let band = Band::linear(tol).map_err(|e| EulerOpError::Certification {
            error: CertifyError::Band(e),
        })?;

        // ---- The faces: each resolves once, with its moved sense. ----
        let mut faces: Vec<MovedFace> = Vec::new();
        for (index, chart) in charts.iter().enumerate() {
            if chart.faces.is_empty() {
                return Err(EulerOpError::EmptyRechart { index });
            }
            for &(face, sense) in &chart.faces {
                let face_data = self.get_face(face).ok_or(EulerOpError::StaleKey {
                    key: EntityId::Face(face),
                })?;
                if faces.iter().any(|m| m.face == face) {
                    return Err(EulerOpError::FaceMovedTwice { face });
                }
                let old = face_data.surface;
                let spec = FaceSurface::New {
                    surface: chart.surface.clone(),
                    sense,
                };
                let resolved = self.resolve_face_surface(
                    &spec,
                    face,
                    (old, face_data.sense),
                    ParentSide::With,
                )?;
                faces.push(MovedFace {
                    face,
                    chart: index,
                    old,
                    sense: resolved.sense,
                    on_old_chart: resolved.on_parent_chart,
                });
            }
        }
        let moved = |f: FaceKey| {
            faces
                .iter()
                .find(|m| m.face == f)
                .map(|m| Slot::Minted(m.chart))
        };
        let (body, planned) = (&*self, &charts);
        let resolve = |sides: Sides| {
            move |k: SurfaceKey| match sides.repoint(k) {
                Slot::Minted(chart) => planned.get(chart).map(|c| c.surface.clone()),
                Slot::Kept(k) => body.surfaces.get(k).cloned(),
            }
        };

        // ---- The listed edges. ----
        let mut written: Vec<(EdgeKey, Sides, EdgeCurve<T>)> = Vec::new();
        for (edge, spec) in redescriptions {
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
                return Err(EulerOpError::DescriptionNotAdjacent { edge });
            }
            let (p_start, p_end) = self.edge_endpoints(edge)?;
            let curve = EdgeCurve::certify(spec, p_start, p_end, resolve(sides), band)
                .map_err(|error| EulerOpError::RechartFalsifies { edge, error })?;
            written.push((edge, sides, curve));
        }

        // ---- The unlisted edges: none stranded, the moved ones carried. ----
        let unlisted = |e: &(EdgeKey, Sides)| !written.iter().any(|(w, ..)| *w == e.0);
        let walk = self.rechart_edges(moved, true)?;
        let stranded: Vec<EdgeKey> = walk
            .stranded
            .into_iter()
            .filter(unlisted)
            .map(|(e, _)| e)
            .collect();
        if !stranded.is_empty() {
            return Err(EulerOpError::RechartStrandsDescriptions { edges: stranded });
        }
        let carried: Vec<(EdgeKey, Sides)> = walk.carried.into_iter().filter(unlisted).collect();
        for (edge, sides) in carried {
            let curve = self.carried_curve(edge, sides, &charts, resolve(sides), band)?;
            written.push((edge, sides, curve));
        }

        // ---- Mutation (infallible from here on). ----
        let minted: Vec<SurfaceKey> = charts
            .into_iter()
            .map(|chart| self.add_surface(chart.surface))
            .collect();
        for m in &faces {
            let (Some(f), Some(&new)) = (self.faces.get_mut(m.face), minted.get(m.chart)) else {
                unreachable!(
                    "set_face_surfaces_describing: every face resolved in the plan phase and \
                     every chart was minted"
                )
            };
            f.surface = new;
            f.sense = m.sense;
            if !m.on_old_chart {
                self.drop_face_rows(m.face);
            }
        }
        for (edge, sides, curve) in written {
            let rekeyed = curve.with_remapped_surfaces(|k| match sides.repoint(k) {
                Slot::Minted(chart) => minted.get(chart).copied(),
                Slot::Kept(k) => Some(k),
            });
            let Some(rekeyed) = rekeyed else {
                unreachable!("set_face_surfaces_describing: every moved chart was minted")
            };
            self.replace_edge_curve(edge, rekeyed);
        }
        for m in &faces {
            self.remove_surface_if_orphaned(m.old);
        }

        #[cfg(debug_assertions)]
        self.assert_tier1_postcondition("set_face_surfaces_describing");
        Ok(minted)
    }

    /// **The edges a re-chart touches**, in edge-arena order: every edge
    /// with a half on a face `moved` answers for, whose stored
    /// description is adjacency-coherent now. `stranded` holds those the
    /// move leaves incoherent; `carried`, those it leaves coherent
    /// through a key their moved face wore. `repoint` reads a
    /// description's keys through [`Sides::repoint`]; without it, a key
    /// stands for itself — the keys-only door's reading. Pure.
    fn rechart_edges(
        &self,
        moved: impl Fn(FaceKey) -> Option<Slot>,
        repoint: bool,
    ) -> Result<RechartEdges, EulerOpError> {
        let mut out = RechartEdges::default();
        for (edge_key, edge) in &self.edges {
            let named = Named::of(self.get_curve_geom(edge.curve).ok_or(
                EulerOpError::StaleGeometry {
                    key: GeomRef::Curve(edge.curve),
                },
            )?);
            if matches!(named, Named::Nothing) {
                continue;
            }
            let sides = self.sides(edge_key, &moved)?;
            if sides.after == sides.before.map(Slot::Kept) || !sides.coherent_before(named) {
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

    /// `edge`'s stored description restated on the charts its moved
    /// faces go to, and certified there at its current endpoints under
    /// `band`: the carry [`Body::set_face_surfaces_describing`] gives an
    /// unlisted edge. A chart image is stated in its chart's
    /// coordinates, so it is re-derived on a chart whose payload is not
    /// the old one's.
    fn carried_curve(
        &self,
        edge: EdgeKey,
        sides: Sides,
        charts: &[Rechart<T>],
        resolve: impl Fn(SurfaceKey) -> Option<Surface<T>>,
        band: Band,
    ) -> Result<EdgeCurve<T>, EulerOpError> {
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
            && let Slot::Minted(chart) = sides.repoint(*surface)
            && !charts.get(chart).is_some_and(|c| {
                self.same_chart_spec(
                    *surface,
                    &FaceSurface::New {
                        surface: c.surface.clone(),
                        sense: true,
                    },
                )
            })
        {
            *image = None;
        }
        let (p_start, p_end) = self.edge_endpoints(edge)?;
        EdgeCurve::certify(spec, p_start, p_end, resolve, band)
            .map_err(|error| EulerOpError::RechartFalsifies { edge, error })
    }

    /// `edge`'s two faces (`he_plus`'s, then `he_minus`'s) across a
    /// re-chart: the surface each wears now, and the one it wears once
    /// `moved` has moved it.
    fn sides(
        &self,
        edge: EdgeKey,
        moved: impl Fn(FaceKey) -> Option<Slot>,
    ) -> Result<Sides, EulerOpError> {
        let edge_data = self.get_edge(edge).ok_or(EulerOpError::StaleKey {
            key: EntityId::Edge(edge),
        })?;
        let side = |he: HalfEdgeKey| {
            let he_data = self.resolve_half_edge(he)?;
            let face = self
                .get_loop(he_data.parent_loop)
                .ok_or(EulerOpError::StaleKey {
                    key: EntityId::Loop(he_data.parent_loop),
                })?
                .face;
            let surface = self
                .get_face(face)
                .ok_or(EulerOpError::StaleKey {
                    key: EntityId::Face(face),
                })?
                .surface;
            Ok::<_, EulerOpError>((surface, moved(face).unwrap_or(Slot::Kept(surface))))
        };
        let (plus, plus_after) = side(edge_data.he_plus)?;
        let (minus, minus_after) = side(edge_data.he_minus)?;
        Ok(Sides {
            before: [plus, minus],
            after: [plus_after, minus_after],
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
    /// run — so on a COMPLETE face a row left saying the old carrier's
    /// image is refused per half-edge, loud, which is where the surface
    /// setter was silent.
    ///
    /// **Two faces of the pass are silent, and neither is this door's
    /// to close.** A face whose chart mints nothing holds no minted row
    /// for a carrier swap to stale at all. A HALF-MINTED face does hold
    /// them, and the pass skips its re-certification entirely — it
    /// reports the missing rows and then measures nothing else about
    /// that face
    /// (`work/trim/validate-pcurves-never-recertifies-a-face-it-finds-incomplete`),
    /// so a row this door stales there is accepted unmeasured. That is
    /// the pass's property for every content staleness in the tree, not
    /// a fact about carrier swaps, and dropping rows here would buy a
    /// `MissingCache` on that one face at the price of a re-mint on
    /// every swap that certifies — including the upgrades this door
    /// exists for, whose rows stay true within band.
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
    /// failed gate; [`EulerOpError::PcurveMint`] where a null edge's
    /// face is re-minted and a half-edge of it does not resolve. The
    /// body is untouched on `Err`.
    pub fn set_edge_curve(
        &mut self,
        edge: EdgeKey,
        curve: EdgeCurveSpec<T>,
        tol: Tol,
    ) -> Result<CurveKey, EulerOpError> {
        self.set_edge_curve_via(edge, curve, Self::certify_edge_spec, tol)
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
    ) -> Result<(), EulerOpError> {
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

    /// [`Body::set_edge_curve`] with the certification door supplied by
    /// the caller — the one axis on which the two attach doors differ
    /// (the plane × NURBS lane, M7-8). Every precondition, every
    /// adjacency rule and every mutation below is shared verbatim, so
    /// the doors cannot drift apart.
    pub(crate) fn set_edge_curve_via(
        &mut self,
        edge: EdgeKey,
        curve: EdgeCurveSpec<T>,
        certify: impl FnOnce(
            &Self,
            EdgeCurveSpec<T>,
            geom_core::Point3<T>,
            geom_core::Point3<T>,
            Tol,
        ) -> Result<geom_brep::EdgeCurve<T>, EulerOpError>,
        tol: Tol,
    ) -> Result<CurveKey, EulerOpError> {
        let (p_start, p_end) = self.edge_endpoints(edge)?;
        self.check_description_adjacent(edge, &curve.description)?;

        let certified = certify(self, curve, p_start, p_end, tol)?;
        let rows = self.null_description_rows(edge, &certified, tol)?;

        // ---- Mutation (infallible from here on). ----
        let new = self.replace_edge_curve(edge, certified);
        crate::pcurves::apply_site_rows(self, rows, None);

        #[cfg(debug_assertions)]
        self.assert_tier1_postcondition("set_edge_curve");
        Ok(new)
    }

    /// **The rows a null edge's first description writes**, decided
    /// before [`Body::set_edge_curve_via`] mutates: one plan per face
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

    /// The **description-adjacency coherence** check (module docs) for
    /// a spec about to describe `edge`: an intrinsic description's two
    /// surfaces are exactly the edge's two faces' surfaces, a chart
    /// image names one of them, a chart seam names the one surface on
    /// both sides, and a scaffold names none. Pure — the plan-phase
    /// half of every door that re-describes an existing edge.
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
        let sides = self.sides(edge, |_| None)?;
        if !sides.coherent_before(Named::of_spec(description)) {
            return Err(EulerOpError::DescriptionNotAdjacent { edge });
        }
        Ok(())
    }
}

/// One chart [`Body::set_face_surfaces_describing`] mints, and the faces
/// it moves onto it.
#[derive(Clone, Debug)]
pub struct Rechart<T: Real> {
    /// The surface, minted fresh (as [`FaceSurface::New`]'s is).
    pub surface: Surface<T>,
    /// The faces that move onto it, each with the material side it
    /// states against the chart's normal ([`FaceSurface::New`]'s
    /// `sense`, per face).
    pub faces: Vec<(FaceKey, bool)>,
}

/// A face [`Body::set_face_surfaces_describing`] moves, as its plan
/// phase resolved it.
struct MovedFace {
    face: FaceKey,
    /// The chart's position in the call's list.
    chart: usize,
    /// The surface the face wears before the move.
    old: SurfaceKey,
    sense: bool,
    /// Whether the new chart is the old one (the pcurve rows' answer).
    on_old_chart: bool,
}

/// [`Body::rechart_edges`]' answer.
#[derive(Default)]
struct RechartEdges {
    stranded: Vec<(EdgeKey, Sides)>,
    carried: Vec<(EdgeKey, Sides)>,
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
}

/// The surface a face wears, or a description names, once a re-chart
/// lands: a key the body already holds, or the chart the call mints at
/// that position — which has no key until the mutation phase.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Slot {
    Kept(SurfaceKey),
    Minted(usize),
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
        let Some(curve) = curve.certified() else {
            return Self::Nothing;
        };
        match curve.description() {
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
    use geom_core::{Tol, Vec3};

    use super::Rechart;
    use crate::body::Body;
    use crate::entity::{EdgeKey, FaceKey};
    use crate::euler::{EulerOpError, FaceSurface};
    use crate::fixtures::{assert_err_deep_unchanged, deep_snapshot};
    use crate::geometry::SurfaceKey;
    use crate::test_support_fixtures::brick;
    use crate::validate::{ValidationError, validate_geometric};

    /// The unit brick, which [`brick`] describes as intersections, and
    /// its top face (the `z = 1` cap).
    fn brick_and_top() -> (Body<f64>, FaceKey) {
        let body = brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), Tol::witness());
        let top = body
            .faces()
            .find(|(_, f)| {
                matches!(
                    body.get_surface(f.surface),
                    Some(Surface::Plane { origin, normal, .. }) if origin.z == 1.0 && normal.z != 0.0
                )
            })
            .map(|(k, _)| k)
            .unwrap();
        assert_eq!(validate_geometric(&body, Tol::witness()), Ok(()));
        (body, top)
    }

    /// The top cap's own plane, raised by `dz` — a fresh payload, so a
    /// swap onto it is a swap onto another chart.
    fn cap_at(body: &Body<f64>, top: FaceKey, dz: f64) -> Surface<f64> {
        let Some(&Surface::Plane {
            origin,
            normal,
            u_ref,
        }) = body.get_surface(body.get_face(top).unwrap().surface)
        else {
            panic!("the brick's top is a plane")
        };
        Surface::Plane {
            origin: origin + Vec3::new(0.0, 0.0, dz),
            normal,
            u_ref,
        }
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

    /// **The keys-only door refuses the swap it used to strand.** A
    /// swap of the top cap onto a fresh key leaves its four edges'
    /// `Intersection`s naming the key the cap left: the door names all
    /// four and writes nothing. The stranding door (the old one's
    /// behavior) returns `Ok` on the same swap, and tier 3 reports
    /// exactly those four at rest.
    #[test]
    fn a_swap_that_strands_an_edge_refuses_naming_every_one_and_writes_nothing() {
        let (mut body, top) = brick_and_top();
        let sense = body.get_face(top).unwrap().sense;
        let old = body.get_face(top).unwrap().surface;
        let named = edges_naming(&body, old);
        assert_eq!(named.len(), 4, "the cap's four rim edges name its plane");
        let cap = cap_at(&body, top, 0.0);
        let swap = || FaceSurface::New {
            surface: cap.clone(),
            sense,
        };

        let mut stranded = body.clone();
        stranded
            .set_face_surface_stranding_for_tests(top, swap())
            .unwrap();
        let errs = validate_geometric(&stranded, Tol::witness()).unwrap_err();
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
                edges: named.clone(),
            },
            |b| b.set_face_surface(top, swap()).unwrap_err(),
        );
    }

    /// **The describing door carries what the keys-only door refuses.**
    /// The same swap onto an equal plane, through
    /// [`Body::set_face_surfaces_describing`] with nothing listed: the
    /// four edges are re-certified on the new key and the body is valid
    /// at rest. Onto a plane a thousand `eps` above the cap, the first
    /// edge in arena order does not certify there and the door refuses,
    /// naming it, with the body untouched.
    #[test]
    fn the_describing_door_carries_onto_the_moved_chart_and_refuses_where_it_would_go_stale() {
        let (body, top) = brick_and_top();
        let sense = body.get_face(top).unwrap().sense;
        let old = body.get_face(top).unwrap().surface;
        let named = edges_naming(&body, old);

        let mut carried = body.clone();
        let [new] = carried
            .set_face_surfaces_describing(
                vec![Rechart {
                    surface: cap_at(&body, top, 0.0),
                    faces: vec![(top, sense)],
                }],
                Vec::new(),
                Tol::witness(),
            )
            .unwrap()[..]
        else {
            panic!("one chart, one key")
        };
        assert_eq!(carried.get_face(top).unwrap().surface, new);
        assert_eq!(edges_naming(&carried, new), named, "all four carried");
        assert!(carried.get_surface(old).is_none(), "the old cap is reaped");
        assert_eq!(validate_geometric(&carried, Tol::witness()), Ok(()));

        let eps = Tol::witness().eps();
        let mut shifted = body.clone();
        let before = deep_snapshot(&shifted);
        let err = shifted
            .set_face_surfaces_describing(
                vec![Rechart {
                    surface: cap_at(&body, top, 1000.0 * eps),
                    faces: vec![(top, sense)],
                }],
                Vec::new(),
                Tol::witness(),
            )
            .unwrap_err();
        assert!(
            matches!(err, EulerOpError::RechartFalsifies { edge, .. } if edge == named[0]),
            "{err:?}"
        );
        assert_eq!(deep_snapshot(&shifted), before, "body changed on Err");
    }

    /// **A listed spec names the chart by the key the face wears now.**
    /// One rim edge restated as an image in the cap's chart, listed:
    /// it lands as a chart image on the NEW key, while the three
    /// unlisted edges are carried as intersections.
    #[test]
    fn a_listed_spec_is_stated_against_the_moved_chart_by_its_old_key() {
        let (mut body, top) = brick_and_top();
        let sense = body.get_face(top).unwrap().sense;
        let old = body.get_face(top).unwrap().surface;
        let named = edges_naming(&body, old);
        let listed = named[0];
        let mut spec = body
            .get_curve_geom(body.get_edge(listed).unwrap().curve)
            .and_then(crate::CurveGeom::certified)
            .unwrap()
            .restated_spec();
        spec.description = geom_brep::EdgeDescriptionSpec::chart(old);
        let cap = cap_at(&body, top, 0.0);
        let [new] = body
            .set_face_surfaces_describing(
                vec![Rechart {
                    surface: cap,
                    faces: vec![(top, sense)],
                }],
                vec![(listed, spec)],
                Tol::witness(),
            )
            .unwrap()[..]
        else {
            panic!("one chart, one key")
        };
        let description = |e: EdgeKey| {
            body.get_curve_geom(body.get_edge(e).unwrap().curve)
                .and_then(crate::CurveGeom::certified)
                .unwrap()
                .description()
                .clone()
        };
        assert!(
            matches!(description(listed), geom_brep::EdgeDescription::Chart(c) if c.surface == new),
            "the listed edge is an image in the new chart"
        );
        for &e in &named[1..] {
            assert!(
                matches!(
                    description(e),
                    geom_brep::EdgeDescription::Intersection { s1, s2, .. } if s1 == new || s2 == new
                ),
                "an unlisted edge is carried as it was, onto the new key"
            );
        }
        assert_eq!(validate_geometric(&body, Tol::witness()), Ok(()));
    }

    /// The describing door's argument refusals, each with the body
    /// untouched: a chart no face wears, a face moved twice, an edge
    /// re-described twice.
    #[test]
    fn the_describing_doors_argument_refusals_write_nothing() {
        let (mut body, top) = brick_and_top();
        let sense = body.get_face(top).unwrap().sense;
        let old = body.get_face(top).unwrap().surface;
        let edge = edges_naming(&body, old)[0];
        let spec = || {
            body.get_curve_geom(body.get_edge(edge).unwrap().curve)
                .and_then(crate::CurveGeom::certified)
                .unwrap()
                .restated_spec()
        };
        let (first, second) = (spec(), spec());
        let cap = cap_at(&body, top, 0.0);
        let chart = |faces: Vec<(FaceKey, bool)>| Rechart {
            surface: cap.clone(),
            faces,
        };
        let tol = Tol::witness();
        assert_err_deep_unchanged(&mut body, &EulerOpError::EmptyRechart { index: 1 }, |b| {
            b.set_face_surfaces_describing(
                vec![chart(vec![(top, sense)]), chart(Vec::new())],
                Vec::new(),
                tol,
            )
            .unwrap_err()
        });
        assert_err_deep_unchanged(
            &mut body,
            &EulerOpError::FaceMovedTwice { face: top },
            |b| {
                b.set_face_surfaces_describing(
                    vec![chart(vec![(top, sense)]), chart(vec![(top, sense)])],
                    Vec::new(),
                    tol,
                )
                .unwrap_err()
            },
        );
        assert_err_deep_unchanged(
            &mut body,
            &EulerOpError::DuplicateRedescription { edge },
            |b| {
                b.set_face_surfaces_describing(
                    vec![chart(vec![(top, sense)])],
                    vec![(edge, first), (edge, second)],
                    tol,
                )
                .unwrap_err()
            },
        );
    }
}
