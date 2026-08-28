//! U2's **collapsed edge description** (`docs/PCURVE-UNIFY-DESIGN.md`,
//! ratified 2026-08-15): the form `geom-brep` certifies and stores,
//! in which every conventional edge is ONE thing — a chart surface
//! and the edge's image in that chart.
//!
//! # The taxonomy, after the collapse
//!
//! [`EdgeDescription`] has four arms and exactly ONE conventional
//! form (and [`EdgeDescriptionSpec`] is its pre-mint twin, the form a
//! CONSTRUCTION states — see that type for why the input needs one):
//!
//! - the two INTRINSIC arms ([`EdgeDescription::Intersection`],
//!   [`EdgeDescription::TangentIntersection`]) are D2's, untouched —
//!   the surfaces determine the locus and the description names them;
//! - [`EdgeDescription::Chart`] is U2's one conventional form,
//!   `(surface, Pcurve)`. `Seam` and `IsoCurve` were not two classes
//!   of locus; they were two ways of writing a chart image down, and
//!   the certification lanes that DO differ survive where they belong
//!   — as [`Pcurve`]'s exactness variants;
//! - [`EdgeDescription::Scaffold`] is the fenced scaffolding door
//!   (D3, Evan 2026-08-27): a [`MappedCurve`] is legal as a
//!   description only while TRANSIENT. Tier 3 refuses it at rest.
//!
//! # The certified statement (D1)
//!
//! One meter: `|C(t) − S(P(t))| ≤ ε`, C4 verbatim, at the
//! certification schedule and on the carrier's OWN parameter (the
//! parameter [`Pcurve`] is defined on). The two seam predicates —
//! half-plane and side — are not a second form; they are an
//! **obligation carried by a chart image that claims to BE a periodic
//! chart's seam** ([`ChartCurve::seam`]), metered beside the one
//! meter and never instead of it.
//!
//! The alternative that was rejected: keeping `Seam` a peer variant.
//! It is cheaper and it forfeits the headline — one conventional
//! form — the migration exists for.
//!
//! # The authority record (U2 Q3)
//!
//! [`EdgeAuthority`] is the per-edge kernel datum that replaces
//! `MappedCurve`'s NEGATIVE SPACE. Tier 3's prefer-intrinsic rules
//! (`topo`'s `TransverseNotIntrinsic` / `TangentNotIntrinsic`) today
//! read "the description is a `MappedCurve`" as "the modeler declared
//! this locus"; once the conventional forms collapse there is no such
//! shape to read, so the declaration is recorded data. Sketch truth is
//! recorded, not derived.

use geom_core::Real;

use crate::keys::SurfaceKey;
use crate::mapped::MappedCurve;
use crate::pcurve_cache::Pcurve;

/// U2's ONE conventional description form: a chart surface and the
/// edge's image in that chart (module docs).
///
/// The certified statement is the unified meter `|C(t) − S(P(t))| ≤ ε`
/// (D1). `pcurve` is on the carrier's own parameter — the same
/// contract [`Pcurve`] carries everywhere else, so a description's
/// image and a stored cache's image are the same object under the
/// same parameterization, which is what makes the two comparable at
/// all.
#[derive(Clone, Debug)]
pub struct ChartCurve<T: Real> {
    /// The chart the edge's locus is described IN (a body-arena key,
    /// lineage-scoped per Q1 — see [`crate::keys`]).
    pub surface: SurfaceKey,
    /// The edge's image in that chart, on the carrier's parameter.
    pub pcurve: Pcurve<T>,
    /// D1's obligation flag: this edge claims to BE the chart's
    /// parameterization seam (the `u_ref`-half-plane meridian), so the
    /// two seam predicates are metered beside the one meter. False on
    /// every other chart image — an iso boundary, a cap rim, a planar
    /// chord — which owe the meter and nothing else.
    ///
    /// It is a claim about THIS EDGE, not about the surface: a
    /// periodic chart has exactly one seam meridian and any number of
    /// other edges.
    pub seam: bool,
}

/// The **authority record** (U2 Q3): who determined this edge's
/// locus. Per-edge kernel data, because tier 3's prefer-intrinsic
/// enforcement must read it and the naming layer is invisible to the
/// kernel.
///
/// `Copy` — [`MappedCurve`] is plain sketch data with no heap payload,
/// so the record costs nothing to carry.
#[derive(Clone, Copy, Debug)]
pub enum EdgeAuthority<T: Real> {
    /// Nothing was declared: the locus is whatever the geometry makes
    /// it. Every intrinsic description, and every conventional chart
    /// image the kernel derived for itself (seams, iso boundaries,
    /// cap rims).
    Derived,
    /// The locus was **declared** by a sketch entity under a sweep map
    /// — the pushforward `MappedCurve` used to say by BEING the
    /// description. The source is kept whole: provenance recorded, not
    /// re-derived.
    Declared(MappedCurve<T>),
}

impl<T: Real> EdgeAuthority<T> {
    /// Whether a modeler DECLARED this locus — the read tier 3's
    /// prefer-intrinsic rules make, in place of the pre-collapse
    /// "the description is a pushforward" shape test.
    pub fn is_declared(&self) -> bool {
        matches!(*self, EdgeAuthority::Declared(_))
    }
}

/// The canonical edge description (module docs): D2's two intrinsic
/// arms, U2's ONE conventional form, and the fenced scaffolding door.
///
/// **Not `Copy`** — [`Pcurve`] carries heap payloads on two of its
/// four variants. See [`crate::EdgeCurve::description`] for how the
/// certified product hands it out (by reference, never by copy: an
/// edge description is read, not moved around).
#[derive(Clone, Debug)]
pub enum EdgeDescription<T: Real> {
    /// Intrinsic: the transverse intersection component selected by
    /// `witness` (D2, verbatim).
    Intersection {
        /// The first surface.
        s1: SurfaceKey,
        /// The second surface.
        s2: SurfaceKey,
        /// The mid-parameter witness point.
        witness: geom_core::Point3<T>,
    },
    /// Intrinsic, one differential order up: the tangential contact
    /// component selected by `witness` (D2 as sharpened by OQ7).
    TangentIntersection {
        /// The first surface.
        s1: SurfaceKey,
        /// The second surface.
        s2: SurfaceKey,
        /// The mid-parameter witness point.
        witness: geom_core::Point3<T>,
    },
    /// **Conventional — the one form** (U2): a chart and the edge's
    /// image in it.
    Chart(ChartCurve<T>),
    /// The fenced scaffolding door (D3): a pushforward standing in as
    /// a description while the edge is TRANSIENT — Euler-op ring
    /// scaffolding whose surfaces do not exist yet. Legal here, and
    /// refused by tier 3 at rest.
    Scaffold(MappedCurve<T>),
}

impl<T: Real> EdgeDescription<T> {
    /// The chart image, when this description has one — the accessor
    /// consumers read instead of matching two conventional variants.
    pub fn chart(&self) -> Option<&ChartCurve<T>> {
        match self {
            EdgeDescription::Chart(c) => Some(c),
            _ => None,
        }
    }
}

/// The description **as a construction states it** — the pre-mint form
/// of [`EdgeDescription`], carried by [`crate::EdgeCurveSpec`].
///
/// # Why the input needs its own type
///
/// **The taxonomy is NOT doubled here — read the next paragraph before
/// concluding the collapse left two of anything.** These arms are
/// [`EdgeDescription`]'s, one for one: two intrinsic, ONE conventional
/// [`EdgeDescriptionSpec::Chart`], the fenced scaffolding door. What
/// differs is not what an edge can BE; it is who derives the chart
/// image.
///
/// An analytic chart's image is minted from the carrier through
/// [`crate::chart_pcurve`], which needs the **resolved surface**. A
/// construction holds arena KEYS and hands certification the resolver
/// (`topo`'s `set_edge_curve` takes none), so a construction cannot
/// mint an image even in principle. Certification therefore mints it
/// at the door, inside the check sequence, where a degenerate interval
/// still refuses in its own order and no chart image is derived from
/// an interval the kernel has not yet accepted (D4/D9).
///
/// **`image: None` is a REQUEST — "derive this at the door" — and
/// never a second description form.** There is no edge whose
/// description is "a chart with no image": every certified
/// [`EdgeDescription::Chart`] carries a real [`Pcurve`], whether the
/// construction stated it or the door minted it. So the conventional
/// form is still singular, and these two types are one taxonomy
/// photographed on either side of the one place a chart image is
/// derived.
///
/// The alternative — one type, image always present — was measured and
/// rejected: it forces the mint out to the callers, which relocates
/// [`crate::CertifyError::ChartImageUnavailable`] and its ORDERING
/// against the interval checks, and that is a verdict change on every
/// degenerate-span row. A second input type is the cheaper honesty.
#[derive(Clone, Debug)]
pub enum EdgeDescriptionSpec<T: Real> {
    /// Intrinsic: the transverse intersection component selected by
    /// `witness` (D2).
    Intersection {
        /// The first surface.
        s1: SurfaceKey,
        /// The second surface.
        s2: SurfaceKey,
        /// The mid-parameter witness point.
        witness: geom_core::Point3<T>,
    },
    /// Intrinsic, one differential order up: the tangential contact
    /// component selected by `witness` (D2 as sharpened by OQ7).
    TangentIntersection {
        /// The first surface.
        s1: SurfaceKey,
        /// The second surface.
        s2: SurfaceKey,
        /// The mid-parameter witness point.
        witness: geom_core::Point3<T>,
    },
    /// **Conventional — the one form** (U2): this edge's locus is the
    /// image of `image` in the chart `surface`.
    Chart {
        /// The chart the locus is described in.
        surface: SurfaceKey,
        /// The image, when the construction KNOWS it exactly — a
        /// spline chart's iso boundary, or a restatement of an image
        /// already certified.
        ///
        /// `None` is a REQUEST, not a description: it asks the
        /// certification door to derive the image from the carrier
        /// ([`crate::chart_pcurve`]), the one place chart images are
        /// derived anywhere in this kernel. The certified form that
        /// comes back carries an image either way.
        image: Option<crate::pcurve_cache::Pcurve<T>>,
        /// D1's obligation: this edge claims to BE the chart's
        /// parameterization seam, so the two seam predicates are
        /// metered beside the one meter.
        seam: bool,
        /// The authority record this construction states (U2 Q3):
        /// `Some` when a sketch entity under a sweep map DECLARED the
        /// locus that this chart image describes. Carried separately
        /// from the description precisely because a declared locus and
        /// a derived one can be the same chart image — the difference
        /// is who determined it, which is data, not shape.
        declared: Option<MappedCurve<T>>,
    },
    /// The fenced scaffolding door (D3): a pushforward standing in as
    /// a description while the edge is TRANSIENT.
    Scaffold(MappedCurve<T>),
}

impl<T: Real> EdgeDescriptionSpec<T> {
    /// A chart image the door derives from the carrier, owed the one
    /// meter and nothing else.
    pub fn chart(surface: SurfaceKey) -> Self {
        EdgeDescriptionSpec::Chart {
            surface,
            image: None,
            seam: false,
            declared: None,
        }
    }

    /// The chart's parameterization seam: a derived image carrying
    /// D1's seam obligation.
    pub fn seam(surface: SurfaceKey) -> Self {
        EdgeDescriptionSpec::Chart {
            surface,
            image: None,
            seam: true,
            declared: None,
        }
    }

    /// A chart image the construction states exactly (spline charts,
    /// and restrictions of already-certified images).
    pub fn chart_image(surface: SurfaceKey, image: crate::pcurve_cache::Pcurve<T>) -> Self {
        EdgeDescriptionSpec::Chart {
            surface,
            image: Some(image),
            seam: false,
            declared: None,
        }
    }

    /// The `u = const` iso image of `surface`, on the carrier's own
    /// parameter: `P(t) = (u, v0 + slope·(t − t0))` with
    /// `slope = (v1 − v0)/(t1 − t0)` — the exact form every stored
    /// cache of this class carries, which is what makes a description's
    /// image and a cache's image the same object.
    pub fn iso(surface: SurfaceKey, u: T, v0: T, v1: T, t0: T, t1: T) -> Self {
        let slope = (v1 - v0) / (t1 - t0);
        Self::chart_image(
            surface,
            crate::pcurve_cache::Pcurve::IsoLine {
                p0: geom_core::Point2::new(u, v0 - slope * t0),
                pl: geom_core::Vec2::new(T::zero(), slope),
            },
        )
    }

    /// The same description with `mc` recorded as the authority that
    /// declared the locus (U2 Q3). No-op on the arms that carry their
    /// own authority: an intrinsic locus is derived by definition, and
    /// a scaffold's pushforward IS its declaration.
    #[must_use]
    pub fn declared_by(self, mc: MappedCurve<T>) -> Self {
        match self {
            EdgeDescriptionSpec::Chart {
                surface,
                image,
                seam,
                ..
            } => EdgeDescriptionSpec::Chart {
                surface,
                image,
                seam,
                declared: Some(mc),
            },
            other => other,
        }
    }
}

/// The authority record a construction's description states (U2 Q3):
/// who determined this edge's locus.
///
/// A scaffold's pushforward IS the declaration; a chart image carries
/// its declaration beside itself when a sketch entity determined the
/// locus it draws; everything else is derived.
pub fn authority_of<T: Real>(description: &EdgeDescriptionSpec<T>) -> EdgeAuthority<T> {
    match *description {
        EdgeDescriptionSpec::Scaffold(mc) => EdgeAuthority::Declared(mc),
        EdgeDescriptionSpec::Chart {
            declared: Some(mc), ..
        } => EdgeAuthority::Declared(mc),
        EdgeDescriptionSpec::Chart { declared: None, .. }
        | EdgeDescriptionSpec::Intersection { .. }
        | EdgeDescriptionSpec::TangentIntersection { .. } => EdgeAuthority::Derived,
    }
}
