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

use geom_brep::{EdgeCurve, EdgeCurveSpec};
use geom_core::Decide;

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
    /// to the body's points, and re-attaching a surface is exactly what
    /// tier 3 re-checks at rest. (In particular, replacing a surface
    /// can invalidate an adjacent edge's certification; tier 3 reports
    /// it — attach surfaces before upgrading edge descriptions.)
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
    /// the face's own chart states the other bit. The body is
    /// untouched on `Err`.
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
    /// only gaps are on loops a null edge holds open — has every loop
    /// that no null edge holds open once this one is described
    /// re-minted whole, through the site mint the Euler operators run
    /// ([`crate::pcurves`]' `site_rows`): the loop leaves complete — the
    /// rows of halves an operator added while it was held open included
    /// — or the face rowless where the closed-form lane cannot mint it.
    /// A loop another null edge still runs through is left as found,
    /// for that edge to release. A face on a spline chart is left as
    /// found.
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
        let edge_data = self.get_edge(edge).ok_or(EulerOpError::StaleKey {
            key: EntityId::Edge(edge),
        })?;
        let he_plus = edge_data.he_plus;
        let plus_data = self.resolve_half_edge(he_plus)?;
        let end_vertex = self.half_edge_end(he_plus).ok_or(EulerOpError::StaleKey {
            key: EntityId::HalfEdge(he_plus),
        })?;
        let p_start = self.resolve_vertex_point(plus_data.start)?;
        let p_end = self.resolve_vertex_point(end_vertex)?;

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
            curve,
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
        let edge_data = self.get_edge(edge).ok_or(EulerOpError::StaleKey {
            key: EntityId::Edge(edge),
        })?;
        let (he_plus, he_minus) = (edge_data.he_plus, edge_data.he_minus);
        let face_surface = |body: &Self, he: crate::entity::HalfEdgeKey| {
            let he_data = body.resolve_half_edge(he)?;
            let loop_data = body
                .get_loop(he_data.parent_loop)
                .ok_or(EulerOpError::StaleKey {
                    key: EntityId::Loop(he_data.parent_loop),
                })?;
            let face_data = body
                .get_face(loop_data.face)
                .ok_or(EulerOpError::StaleKey {
                    key: EntityId::Face(loop_data.face),
                })?;
            Ok::<SurfaceKey, EulerOpError>(face_data.surface)
        };
        let fs_plus = face_surface(self, he_plus)?;
        let fs_minus = face_surface(self, he_minus)?;
        match *description {
            // Both intrinsic variants carry the same adjacency
            // obligation: the described pair IS the faces' pair
            // (M5 PR 9 — TangentIntersection mirrors Intersection).
            geom_brep::EdgeDescriptionSpec::Intersection { s1, s2, .. }
            | geom_brep::EdgeDescriptionSpec::TangentIntersection { s1, s2, .. } => {
                let matches_pair =
                    (s1 == fs_plus && s2 == fs_minus) || (s1 == fs_minus && s2 == fs_plus);
                if !matches_pair {
                    return Err(EulerOpError::DescriptionNotAdjacent { edge });
                }
            }
            // A chart image names ONE of the edge's two adjacent
            // faces' surfaces (M6-3: a wall–wall seam is the
            // u-boundary iso of either wall; the minted convention
            // picks one, and adjacency accepts either side — the
            // M5-LOG item 6(iii) reading). A chart image that claims
            // to BE the chart's parameterization seam owes more: both
            // sides of a seam are the SAME surface, by what a seam is.
            geom_brep::EdgeDescriptionSpec::Chart { surface, seam, .. } => {
                let adjacent = if seam {
                    surface == fs_plus && surface == fs_minus
                } else {
                    surface == fs_plus || surface == fs_minus
                };
                if !adjacent {
                    return Err(EulerOpError::DescriptionNotAdjacent { edge });
                }
            }
            // The scaffolding door names no surface — there is none.
            geom_brep::EdgeDescriptionSpec::Scaffold(_) => {}
        }
        Ok(())
    }
}
