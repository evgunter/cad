//! **Topo's construction tier** (DESIGN D4 ¶1 (iv); Ev on PR 5838): the
//! Euler operators, the edge and face description setters and the row
//! and joint doors, as one trait over [`Body`]. It is the kernel's
//! internal API: every curve-and-face pair these doors read is one a
//! kernel crate composed, so a miss there is the kernel's defect.
//!
//! The doors are trait methods, not inherent ones, so the curated
//! façade (`pncad::topo`) can leave them out: an inherent method
//! travels with `Body` through any re-export that names the type.
//! A kernel crate brings the trait into scope (`use
//! topo::construction::Construct;`); each method is the inherent door of
//! the same name, documented there.

#![allow(clippy::too_many_arguments, missing_docs)]

use geom_brep::{EdgeCurveSpec, EdgeDescriptionSpec, PcurveCache};
use geom_core::{Band, Decide, Point3, Real, Tol};

use crate::boolean::{BooleanError, EdgeJoin};
use crate::{
    Body, CurveKey, EdgeKey, EulerOpError, FaceKey, HalfEdgeKey, JointElement, LoopKey, ShellKey,
    SolidKey, SurfaceKey, VertexKey,
};

// The construction tier's own vocabulary: the sites, surfaces and
// results its doors take and return, which nothing outside the tier
// names. The doors' refusal and its payloads (`EulerOpError`,
// `BadArgument`, `RechartDoor`) stay at the crate root.
pub use crate::attach::Rechart;
pub use crate::euler::{FaceSurface, MefCreated, MefSite, MevCreated, MevSite, MvfsCreated};
pub use crate::euler_kill::{KefResult, KevResult, KvfsResult, MergedMember, MfkrhCreated};
pub use crate::euler_ring::{KemrResult, KfmrhResult, MekrResult, MekrSite};
pub use crate::null::{NewVertexSide, NullEdge, NullFacePair};
pub use crate::split::SplitEdgeCreated;
pub use crate::surgery::Surgery;

/// **The construction tier, the kernel's internal API** (module docs):
/// each method forwards to the `Body` door of the same name.
pub trait Construct<T: geom_core::Real> {
    fn mvfs(&mut self, point: Point3<T>, sense: bool) -> Result<MvfsCreated, EulerOpError>
    where
        T: Decide;
    fn mev(
        &mut self,
        site: MevSite,
        point: Point3<T>,
        curve: EdgeCurveSpec<T>,
        tol: Tol,
    ) -> Result<MevCreated, EulerOpError>
    where
        T: Decide,
        T: crate::props::AtRestPolicy;
    fn mev_line(
        &mut self,
        site: MevSite,
        point: Point3<T>,
        tol: Tol,
    ) -> Result<MevCreated, EulerOpError>
    where
        T: Decide,
        T: crate::props::AtRestPolicy;
    fn mev_null(
        &mut self,
        site: MevSite,
        new_side: NewVertexSide,
    ) -> Result<MevCreated, EulerOpError>
    where
        T: geom_core::Decide;
    fn mef(
        &mut self,
        site: MefSite,
        curve: EdgeCurveSpec<T>,
        surface: FaceSurface<T>,
        tol: Tol,
    ) -> Result<MefCreated, EulerOpError>
    where
        T: Decide,
        T: crate::props::AtRestPolicy;
    fn mef_chord(&mut self, site: MefSite, tol: Tol) -> Result<MefCreated, EulerOpError>
    where
        T: Decide,
        T: crate::props::AtRestPolicy;
    fn mekr(
        &mut self,
        site: MekrSite,
        curve: EdgeCurveSpec<T>,
        tol: Tol,
    ) -> Result<MekrResult, EulerOpError>
    where
        T: Decide,
        T: crate::props::AtRestPolicy;
    fn mekr_chord(&mut self, site: MekrSite, tol: Tol) -> Result<MekrResult, EulerOpError>
    where
        T: Decide,
        T: crate::props::AtRestPolicy;
    fn mfkrh(
        &mut self,
        ring: LoopKey,
        surface: FaceSurface<T>,
    ) -> Result<MfkrhCreated, EulerOpError>
    where
        T: Decide;
    fn mfkrh_minting(
        &mut self,
        ring: LoopKey,
        surface: FaceSurface<T>,
        tol: Tol,
    ) -> Result<MfkrhCreated, EulerOpError>
    where
        T: Decide;
    fn mfkrh_plug(&mut self, ring: LoopKey, sense: bool) -> Result<MfkrhCreated, EulerOpError>
    where
        T: Decide;
    fn kev(&mut self, he: HalfEdgeKey) -> Result<KevResult, EulerOpError>
    where
        T: Decide;
    fn kev_describing(
        &mut self,
        he: HalfEdgeKey,
        redescriptions: &[(EdgeKey, EdgeCurveSpec<T>)],
        tol: Tol,
    ) -> Result<KevResult, EulerOpError>
    where
        T: Decide,
        T: crate::props::AtRestPolicy;
    fn kev_merged_members(&self, he: HalfEdgeKey) -> Result<Vec<MergedMember<T>>, EulerOpError>
    where
        T: Decide;
    fn kef(&mut self, he: HalfEdgeKey) -> Result<KefResult, EulerOpError>
    where
        T: Decide;
    fn kef_describing(
        &mut self,
        he: HalfEdgeKey,
        redescriptions: &[(EdgeKey, EdgeDescriptionSpec<T>)],
        tol: Tol,
    ) -> Result<KefResult, EulerOpError>
    where
        T: Decide,
        T: crate::props::AtRestPolicy;
    fn kef_minting(&mut self, he: HalfEdgeKey, tol: Tol) -> Result<KefResult, EulerOpError>
    where
        T: Decide;
    fn kef_carried_redescriptions(
        &self,
        he: HalfEdgeKey,
    ) -> Result<Vec<(EdgeKey, EdgeDescriptionSpec<T>)>, EulerOpError>
    where
        T: Decide;
    fn kvfs(&mut self, solid: SolidKey) -> Result<KvfsResult, EulerOpError>
    where
        T: Decide;
    fn kemr(&mut self, he1: HalfEdgeKey, he2: HalfEdgeKey) -> Result<KemrResult, EulerOpError>
    where
        T: Decide;
    fn kemr_minting(
        &mut self,
        he1: HalfEdgeKey,
        he2: HalfEdgeKey,
        tol: Tol,
    ) -> Result<KemrResult, EulerOpError>
    where
        T: Decide;
    fn kfmrh(&mut self, f1: FaceKey, f2: FaceKey) -> Result<KfmrhResult, EulerOpError>
    where
        T: Decide;
    fn kfmrh_describing(
        &mut self,
        f1: FaceKey,
        f2: FaceKey,
        redescriptions: &[(EdgeKey, geom_brep::EdgeDescriptionSpec<T>)],
        tol: Tol,
    ) -> Result<KfmrhResult, EulerOpError>
    where
        T: Decide,
        T: crate::props::AtRestPolicy;
    fn kfmrh_minting(
        &mut self,
        f1: FaceKey,
        f2: FaceKey,
        tol: Tol,
    ) -> Result<KfmrhResult, EulerOpError>
    where
        T: Decide;
    fn kfmrh_carried_redescriptions(
        &self,
        f1: FaceKey,
        f2: FaceKey,
    ) -> Result<Vec<(EdgeKey, geom_brep::EdgeDescriptionSpec<T>)>, EulerOpError>
    where
        T: Decide;
    fn ring_move(&mut self, ring: LoopKey, to_face: FaceKey) -> Result<(), EulerOpError>
    where
        T: Decide;
    fn ring_move_minting(
        &mut self,
        ring: LoopKey,
        to_face: FaceKey,
        tol: Tol,
    ) -> Result<(), EulerOpError>
    where
        T: Decide;
    fn split_edge(
        &mut self,
        edge: EdgeKey,
        t: T,
        tol: Tol,
    ) -> Result<SplitEdgeCreated, EulerOpError>
    where
        T: Decide,
        T: crate::props::AtRestPolicy;
    fn join_edges(&mut self, band: Band, tol: Tol) -> Result<Vec<EdgeJoin>, BooleanError>
    where
        T: Decide + crate::props::AtRestPolicy;
    fn join_edges_within(
        &mut self,
        band: Band,
        tol: Tol,
        within: &dyn Fn(VertexKey) -> bool,
    ) -> Result<Vec<EdgeJoin>, BooleanError>
    where
        T: Decide + crate::props::AtRestPolicy;
    fn movefac(&mut self, shell: ShellKey) -> Result<Vec<ShellKey>, EulerOpError>
    where
        T: Decide;
    fn move_shells_to_new_solid(&mut self, shells: &[ShellKey]) -> Result<SolidKey, EulerOpError>
    where
        T: Decide;
    fn set_edge_curve(
        &mut self,
        edge: EdgeKey,
        curve: EdgeCurveSpec<T>,
        tol: Tol,
    ) -> Result<CurveKey, EulerOpError>
    where
        T: Decide,
        T: crate::props::AtRestPolicy;
    fn set_face_surface(
        &mut self,
        face: FaceKey,
        surface: FaceSurface<T>,
    ) -> Result<SurfaceKey, EulerOpError>
    where
        T: Decide;
    #[cfg(any(test, feature = "test-support", feature = "sweep-testing"))]
    #[doc(hidden)]
    fn set_face_surface_unvouched_for_tests(
        &mut self,
        face: FaceKey,
        surface: FaceSurface<T>,
    ) -> Result<SurfaceKey, EulerOpError>
    where
        T: Decide;
    fn set_face_surfaces_describing(
        &mut self,
        charts: Vec<Rechart<T>>,
        redescriptions: &[(EdgeKey, EdgeCurveSpec<T>)],
        tol: Tol,
    ) -> Result<Vec<SurfaceKey>, EulerOpError>
    where
        T: Decide,
        T: crate::props::AtRestPolicy;
    fn set_face_sense(&mut self, face: FaceKey, sense: bool) -> Result<(), EulerOpError>
    where
        T: Decide;
    fn restate_stored_edge(
        &mut self,
        edge: EdgeKey,
        description: EdgeDescriptionSpec<T>,
        tol: Tol,
    ) -> Result<CurveKey, EulerOpError>
    where
        T: Decide,
        T: crate::props::AtRestPolicy;
    fn describe_at_rest(&mut self, edge: EdgeKey, tol: Tol) -> Result<(), EulerOpError>
    where
        T: Decide,
        T: crate::props::AtRestPolicy;
    fn set_null_face_pair(&mut self, face: FaceKey, pair: NullFacePair) -> Result<(), EulerOpError>
    where
        T: Real;
    fn clear_null_face_pair(&mut self, face: FaceKey) -> Option<NullFacePair>
    where
        T: Real;
    fn attach_pcurve(
        &mut self,
        half_edge: HalfEdgeKey,
        cache: PcurveCache<T>,
    ) -> Option<PcurveCache<T>>
    where
        T: Real;
    fn detach_pcurve(&mut self, half_edge: HalfEdgeKey) -> Option<PcurveCache<T>>
    where
        T: Real;
    fn attach_joint(
        &mut self,
        half_edge: HalfEdgeKey,
        element: JointElement,
    ) -> Option<JointElement>
    where
        T: Real;
    fn detach_joint(&mut self, half_edge: HalfEdgeKey) -> Option<JointElement>
    where
        T: Real;
    #[must_use = "a scope that is opened and immediately dropped silences nothing"]
    fn begin_surgery(&mut self) -> Surgery<'_, T>
    where
        T: Real;
}

impl<T: geom_core::Real> Construct<T> for Body<T> {
    fn mvfs(&mut self, point: Point3<T>, sense: bool) -> Result<MvfsCreated, EulerOpError>
    where
        T: Decide,
    {
        Body::mvfs(self, point, sense)
    }

    fn mev(
        &mut self,
        site: MevSite,
        point: Point3<T>,
        curve: EdgeCurveSpec<T>,
        tol: Tol,
    ) -> Result<MevCreated, EulerOpError>
    where
        T: Decide,
        T: crate::props::AtRestPolicy,
    {
        Body::mev(self, site, point, curve, tol)
    }

    fn mev_line(
        &mut self,
        site: MevSite,
        point: Point3<T>,
        tol: Tol,
    ) -> Result<MevCreated, EulerOpError>
    where
        T: Decide,
        T: crate::props::AtRestPolicy,
    {
        Body::mev_line(self, site, point, tol)
    }

    fn mev_null(
        &mut self,
        site: MevSite,
        new_side: NewVertexSide,
    ) -> Result<MevCreated, EulerOpError>
    where
        T: geom_core::Decide,
    {
        Body::mev_null(self, site, new_side)
    }

    fn mef(
        &mut self,
        site: MefSite,
        curve: EdgeCurveSpec<T>,
        surface: FaceSurface<T>,
        tol: Tol,
    ) -> Result<MefCreated, EulerOpError>
    where
        T: Decide,
        T: crate::props::AtRestPolicy,
    {
        Body::mef(self, site, curve, surface, tol)
    }

    fn mef_chord(&mut self, site: MefSite, tol: Tol) -> Result<MefCreated, EulerOpError>
    where
        T: Decide,
        T: crate::props::AtRestPolicy,
    {
        Body::mef_chord(self, site, tol)
    }

    fn mekr(
        &mut self,
        site: MekrSite,
        curve: EdgeCurveSpec<T>,
        tol: Tol,
    ) -> Result<MekrResult, EulerOpError>
    where
        T: Decide,
        T: crate::props::AtRestPolicy,
    {
        Body::mekr(self, site, curve, tol)
    }

    fn mekr_chord(&mut self, site: MekrSite, tol: Tol) -> Result<MekrResult, EulerOpError>
    where
        T: Decide,
        T: crate::props::AtRestPolicy,
    {
        Body::mekr_chord(self, site, tol)
    }

    fn mfkrh(
        &mut self,
        ring: LoopKey,
        surface: FaceSurface<T>,
    ) -> Result<MfkrhCreated, EulerOpError>
    where
        T: Decide,
    {
        Body::mfkrh(self, ring, surface)
    }

    fn mfkrh_minting(
        &mut self,
        ring: LoopKey,
        surface: FaceSurface<T>,
        tol: Tol,
    ) -> Result<MfkrhCreated, EulerOpError>
    where
        T: Decide,
    {
        Body::mfkrh_minting(self, ring, surface, tol)
    }

    fn mfkrh_plug(&mut self, ring: LoopKey, sense: bool) -> Result<MfkrhCreated, EulerOpError>
    where
        T: Decide,
    {
        Body::mfkrh_plug(self, ring, sense)
    }

    fn kev(&mut self, he: HalfEdgeKey) -> Result<KevResult, EulerOpError>
    where
        T: Decide,
    {
        Body::kev(self, he)
    }

    fn kev_describing(
        &mut self,
        he: HalfEdgeKey,
        redescriptions: &[(EdgeKey, EdgeCurveSpec<T>)],
        tol: Tol,
    ) -> Result<KevResult, EulerOpError>
    where
        T: Decide,
        T: crate::props::AtRestPolicy,
    {
        Body::kev_describing(self, he, redescriptions, tol)
    }

    fn kev_merged_members(&self, he: HalfEdgeKey) -> Result<Vec<MergedMember<T>>, EulerOpError>
    where
        T: Decide,
    {
        Body::kev_merged_members(self, he)
    }

    fn kef(&mut self, he: HalfEdgeKey) -> Result<KefResult, EulerOpError>
    where
        T: Decide,
    {
        Body::kef(self, he)
    }

    fn kef_describing(
        &mut self,
        he: HalfEdgeKey,
        redescriptions: &[(EdgeKey, EdgeDescriptionSpec<T>)],
        tol: Tol,
    ) -> Result<KefResult, EulerOpError>
    where
        T: Decide,
        T: crate::props::AtRestPolicy,
    {
        Body::kef_describing(self, he, redescriptions, tol)
    }

    fn kef_minting(&mut self, he: HalfEdgeKey, tol: Tol) -> Result<KefResult, EulerOpError>
    where
        T: Decide,
    {
        Body::kef_minting(self, he, tol)
    }

    fn kef_carried_redescriptions(
        &self,
        he: HalfEdgeKey,
    ) -> Result<Vec<(EdgeKey, EdgeDescriptionSpec<T>)>, EulerOpError>
    where
        T: Decide,
    {
        Body::kef_carried_redescriptions(self, he)
    }

    fn kvfs(&mut self, solid: SolidKey) -> Result<KvfsResult, EulerOpError>
    where
        T: Decide,
    {
        Body::kvfs(self, solid)
    }

    fn kemr(&mut self, he1: HalfEdgeKey, he2: HalfEdgeKey) -> Result<KemrResult, EulerOpError>
    where
        T: Decide,
    {
        Body::kemr(self, he1, he2)
    }

    fn kemr_minting(
        &mut self,
        he1: HalfEdgeKey,
        he2: HalfEdgeKey,
        tol: Tol,
    ) -> Result<KemrResult, EulerOpError>
    where
        T: Decide,
    {
        Body::kemr_minting(self, he1, he2, tol)
    }

    fn kfmrh(&mut self, f1: FaceKey, f2: FaceKey) -> Result<KfmrhResult, EulerOpError>
    where
        T: Decide,
    {
        Body::kfmrh(self, f1, f2)
    }

    fn kfmrh_describing(
        &mut self,
        f1: FaceKey,
        f2: FaceKey,
        redescriptions: &[(EdgeKey, geom_brep::EdgeDescriptionSpec<T>)],
        tol: Tol,
    ) -> Result<KfmrhResult, EulerOpError>
    where
        T: Decide,
        T: crate::props::AtRestPolicy,
    {
        Body::kfmrh_describing(self, f1, f2, redescriptions, tol)
    }

    fn kfmrh_minting(
        &mut self,
        f1: FaceKey,
        f2: FaceKey,
        tol: Tol,
    ) -> Result<KfmrhResult, EulerOpError>
    where
        T: Decide,
    {
        Body::kfmrh_minting(self, f1, f2, tol)
    }

    fn kfmrh_carried_redescriptions(
        &self,
        f1: FaceKey,
        f2: FaceKey,
    ) -> Result<Vec<(EdgeKey, geom_brep::EdgeDescriptionSpec<T>)>, EulerOpError>
    where
        T: Decide,
    {
        Body::kfmrh_carried_redescriptions(self, f1, f2)
    }

    fn ring_move(&mut self, ring: LoopKey, to_face: FaceKey) -> Result<(), EulerOpError>
    where
        T: Decide,
    {
        Body::ring_move(self, ring, to_face)
    }

    fn ring_move_minting(
        &mut self,
        ring: LoopKey,
        to_face: FaceKey,
        tol: Tol,
    ) -> Result<(), EulerOpError>
    where
        T: Decide,
    {
        Body::ring_move_minting(self, ring, to_face, tol)
    }

    fn split_edge(
        &mut self,
        edge: EdgeKey,
        t: T,
        tol: Tol,
    ) -> Result<SplitEdgeCreated, EulerOpError>
    where
        T: Decide,
        T: crate::props::AtRestPolicy,
    {
        Body::split_edge(self, edge, t, tol)
    }

    fn join_edges(&mut self, band: Band, tol: Tol) -> Result<Vec<EdgeJoin>, BooleanError>
    where
        T: Decide + crate::props::AtRestPolicy,
    {
        Body::join_edges(self, band, tol)
    }

    fn join_edges_within(
        &mut self,
        band: Band,
        tol: Tol,
        within: &dyn Fn(VertexKey) -> bool,
    ) -> Result<Vec<EdgeJoin>, BooleanError>
    where
        T: Decide + crate::props::AtRestPolicy,
    {
        Body::join_edges_within(self, band, tol, within)
    }

    fn movefac(&mut self, shell: ShellKey) -> Result<Vec<ShellKey>, EulerOpError>
    where
        T: Decide,
    {
        Body::movefac(self, shell)
    }

    fn move_shells_to_new_solid(&mut self, shells: &[ShellKey]) -> Result<SolidKey, EulerOpError>
    where
        T: Decide,
    {
        Body::move_shells_to_new_solid(self, shells)
    }

    fn set_edge_curve(
        &mut self,
        edge: EdgeKey,
        curve: EdgeCurveSpec<T>,
        tol: Tol,
    ) -> Result<CurveKey, EulerOpError>
    where
        T: Decide,
        T: crate::props::AtRestPolicy,
    {
        Body::set_edge_curve(self, edge, curve, tol)
    }

    fn set_face_surface(
        &mut self,
        face: FaceKey,
        surface: FaceSurface<T>,
    ) -> Result<SurfaceKey, EulerOpError>
    where
        T: Decide,
    {
        Body::set_face_surface(self, face, surface)
    }

    #[cfg(any(test, feature = "test-support", feature = "sweep-testing"))]
    #[doc(hidden)]
    fn set_face_surface_unvouched_for_tests(
        &mut self,
        face: FaceKey,
        surface: FaceSurface<T>,
    ) -> Result<SurfaceKey, EulerOpError>
    where
        T: Decide,
    {
        Body::set_face_surface_unvouched_for_tests(self, face, surface)
    }

    fn set_face_surfaces_describing(
        &mut self,
        charts: Vec<Rechart<T>>,
        redescriptions: &[(EdgeKey, EdgeCurveSpec<T>)],
        tol: Tol,
    ) -> Result<Vec<SurfaceKey>, EulerOpError>
    where
        T: Decide,
        T: crate::props::AtRestPolicy,
    {
        Body::set_face_surfaces_describing(self, charts, redescriptions, tol)
    }

    fn set_face_sense(&mut self, face: FaceKey, sense: bool) -> Result<(), EulerOpError>
    where
        T: Decide,
    {
        Body::set_face_sense(self, face, sense)
    }

    fn restate_stored_edge(
        &mut self,
        edge: EdgeKey,
        description: EdgeDescriptionSpec<T>,
        tol: Tol,
    ) -> Result<CurveKey, EulerOpError>
    where
        T: Decide,
        T: crate::props::AtRestPolicy,
    {
        Body::restate_stored_edge(self, edge, description, tol)
    }

    fn describe_at_rest(&mut self, edge: EdgeKey, tol: Tol) -> Result<(), EulerOpError>
    where
        T: Decide,
        T: crate::props::AtRestPolicy,
    {
        Body::describe_at_rest(self, edge, tol)
    }

    fn set_null_face_pair(&mut self, face: FaceKey, pair: NullFacePair) -> Result<(), EulerOpError>
    where
        T: Real,
    {
        Body::set_null_face_pair(self, face, pair)
    }

    fn clear_null_face_pair(&mut self, face: FaceKey) -> Option<NullFacePair>
    where
        T: Real,
    {
        Body::clear_null_face_pair(self, face)
    }

    fn attach_pcurve(
        &mut self,
        half_edge: HalfEdgeKey,
        cache: PcurveCache<T>,
    ) -> Option<PcurveCache<T>>
    where
        T: Real,
    {
        Body::attach_pcurve(self, half_edge, cache)
    }

    fn detach_pcurve(&mut self, half_edge: HalfEdgeKey) -> Option<PcurveCache<T>>
    where
        T: Real,
    {
        Body::detach_pcurve(self, half_edge)
    }

    fn attach_joint(
        &mut self,
        half_edge: HalfEdgeKey,
        element: JointElement,
    ) -> Option<JointElement>
    where
        T: Real,
    {
        Body::attach_joint(self, half_edge, element)
    }

    fn detach_joint(&mut self, half_edge: HalfEdgeKey) -> Option<JointElement>
    where
        T: Real,
    {
        Body::detach_joint(self, half_edge)
    }

    fn begin_surgery(&mut self) -> Surgery<'_, T>
    where
        T: Real,
    {
        Body::begin_surgery(self)
    }
}
