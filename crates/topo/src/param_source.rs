//! **`ParamSource`** — lowered parameter identity, one level finer than
//! [`GeomSource`](crate::source::GeomSource) and deliberately more
//! opaque.
//!
//! A `GeomSource` says which recipe expression produced a whole
//! DESCRIPTION. A [`ParamSource`] says which recipe expression produced
//! one stored scalar FIELD of one description — a cylinder's radius, a
//! sphere's radius, a torus's minor radius. The records are opt-in side
//! tables beside the geometry arenas, exactly as the `GeomSource`
//! records are, and absent for every kernel-direct caller.
//!
//! # What the kernel may do with one: compare it
//!
//! To this crate a `ParamSource` is a fully **opaque token**. It has
//! `Eq`/`Ord`/`Hash` and nothing else — no readable payload, no
//! constructor that builds one out of another, no arithmetic. Two
//! fields carry the same token exactly when the recipe layer evaluated
//! the same expression into both, which is equality *by provenance*:
//! zero numerics, immune to tolerance arguments, true by construction.
//!
//! It carries deliberately LESS structure than a `GeomSource`.
//! `SourceExpr::Placed` exists in this crate only because a rigid
//! placement re-parameterizes a *description*, so whoever runs the op
//! must compose the record. A stored scalar field is
//! **motion-invariant**: no rigid map changes a radius, so no kernel op
//! ever composes or interprets a `ParamSource`, and no second spelling
//! of expression structure enters the kernel.
//!
//! # Propagation, which is therefore trivial
//!
//! Survivors keep their records by key identity (the maps ride the
//! clone every op starts from). Rigid placement carries them
//! **verbatim** — `transform_rigid` clears the `GeomSource` records
//! because it rewrites description bits, and does not clear these
//! because it cannot change a radius. Kills drop them with the surface
//! they were attached to.
//!
//! # Scope of the identity claim
//!
//! Verbatim from [`crate::source`]: identity claims hold **per
//! evaluation against the current document**, never across unaudited
//! document mutations.
//!
//! # Who mints them
//!
//! `editor-core`, and only `editor-core`. It holds the single spelling
//! of expression identity, lowers a slot expression to the token
//! through [`ParamSource::from_lowered`], and inverts the token for
//! diagnosis on its own side of the line. A kernel-derived field — the
//! hollow tube's `minor_radius − wall`, say — carries no source: the
//! kernel does not evaluate expressions, so identity ends where
//! `editor-core`'s evaluation did.

use std::sync::Arc;

use geom::Surface;
use geom_brep::RadiusEvidence;
use geom_core::Real;

use crate::body::Body;
use crate::entity::FaceKey;

/// **The lowered identity of the recipe expression that produced one
/// stored scalar field** — an opaque token.
///
/// The payload is a byte string the minting layer produced and only the
/// minting layer can read; this crate has no decoder and wants none.
/// What it has is equality, which is the whole contract: same token ⇒
/// the same expression was evaluated into both fields ⇒ the fields are
/// equal by construction.
///
/// **Not a digest.** The lowering is injective, so token equality is
/// expression equality outright rather than a hash-collision-shaped
/// claim about it.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ParamSource(Arc<[u8]>);

impl ParamSource {
    /// The token for an already-lowered expression identity.
    ///
    /// The ONE door, and it takes bytes rather than any structured
    /// value on purpose: the recipe vocabulary stays above the layering
    /// line (`GeomSource`'s module docs), and what crosses it is data
    /// this crate compares and never reads.
    #[must_use]
    pub fn from_lowered(lowered: &[u8]) -> Self {
        Self(Arc::from(lowered))
    }
}

/// **The stored scalar fields a surface description has**, closed.
///
/// One variant per named scalar in [`Surface`]'s analytic arms. This is
/// the declaration the side records are keyed at — a field a recipe
/// parameter can land in gets a variant here, and every match over it
/// is visited (D3, no wildcard arms). The spline arms (`Nurbs`,
/// `Approx`) have no named scalar a parameter flows into: their data is
/// a control net, and a control point is not a stored parameter.
///
/// Placement data — origins, axes, seam references — is deliberately
/// ABSENT. Those are not motion-invariant, so a token attached to one
/// would have to be composed through rigid placement, which is exactly
/// the structure this channel does not carry (module docs).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SurfaceField {
    /// [`Surface::Cylinder`]'s `radius`.
    CylinderRadius,
    /// [`Surface::Cone`]'s `half_angle`.
    ConeHalfAngle,
    /// [`Surface::Sphere`]'s `radius`.
    SphereRadius,
    /// [`Surface::Torus`]'s `major_radius`.
    TorusMajorRadius,
    /// [`Surface::Torus`]'s `minor_radius`.
    TorusMinorRadius,
}

impl SurfaceField {
    /// Every field in the declaration, for censuses that must be total
    /// over it. Its length is [`SurfaceField::COUNT`], which is the row
    /// width of the per-surface record.
    pub const ALL: &'static [Self] = &[
        Self::CylinderRadius,
        Self::ConeHalfAngle,
        Self::SphereRadius,
        Self::TorusMajorRadius,
        Self::TorusMinorRadius,
    ];

    /// How many fields the declaration holds — the width of one
    /// surface's record row.
    pub const COUNT: usize = 5;

    /// This field's position in a record row.
    const fn index(self) -> usize {
        match self {
            Self::CylinderRadius => 0,
            Self::ConeHalfAngle => 1,
            Self::SphereRadius => 2,
            Self::TorusMajorRadius => 3,
            Self::TorusMinorRadius => 4,
        }
    }

    /// **Whether `surface` actually has this field.** The kind axis is
    /// exhaustive: a surface arm added to [`Surface`] is visited here
    /// and must declare its stored scalars (or say it has none) before
    /// this crate compiles.
    #[must_use]
    pub fn belongs_to<T: Real>(self, surface: &Surface<T>) -> bool {
        match surface {
            Surface::Plane { .. } | Surface::Nurbs(_) | Surface::Approx(_) => false,
            Surface::Cylinder { .. } => self == Self::CylinderRadius,
            Surface::Cone { .. } => self == Self::ConeHalfAngle,
            Surface::Sphere { .. } => self == Self::SphereRadius,
            Surface::Torus { .. } => {
                self == Self::TorusMajorRadius || self == Self::TorusMinorRadius
            }
        }
    }
}

/// One surface's per-field records: at most one token per declared
/// field, absent everywhere the recipe layer attached nothing.
///
/// A fixed row rather than a map, so the storage shape IS the closed
/// declaration: a field cannot be spelled that [`SurfaceField`] does
/// not name.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct FieldSources {
    rows: [Option<ParamSource>; SurfaceField::COUNT],
}

impl FieldSources {
    /// The token on `field`, if one was attached.
    pub(crate) fn get(&self, field: SurfaceField) -> Option<&ParamSource> {
        self.rows[field.index()].as_ref()
    }

    /// Attaches (or replaces) the token on `field`.
    pub(crate) fn set(&mut self, field: SurfaceField, source: ParamSource) {
        self.rows[field.index()] = Some(source);
    }
}

/// A refused [`ParamSource`] attachment (closed enum, D3 style).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParamAttachError {
    /// The surface key does not resolve — attaching identity to nothing
    /// is a caller bug, refused loudly (the
    /// [`crate::SourceAttachError::StaleKey`] posture).
    StaleKey,
    /// The surface at that key has no such field: a cylinder has no
    /// minor radius, a plane has no radius at all. Attaching a
    /// parameter to a field the description does not store is a wiring
    /// bug in the flow declaration, and it is refused rather than
    /// recorded where nothing will ever read it.
    FieldNotOnKind {
        /// The field the caller named.
        field: SurfaceField,
    },
}

impl core::fmt::Display for ParamAttachError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::StaleKey => write!(f, "param-source attachment: stale surface key"),
            Self::FieldNotOnKind { field } => write!(
                f,
                "param-source attachment: the surface has no {field:?} field"
            ),
        }
    }
}

impl std::error::Error for ParamAttachError {}

/// **The declared-equality evidence for one stored field of two faces'
/// carriers** — the production side of the coincidence ladder's
/// "structural or declared, never inferred" rule.
///
/// [`RadiusEvidence::Declared`] iff both carriers hold a token on
/// `field` and the two tokens are EQUAL: the recipe layer evaluated one
/// expression into both fields, so they are equal by construction and
/// nothing was measured. Everything else is
/// [`RadiusEvidence::None`] — differing tokens, one side absent, both
/// absent (imported geometry, hand-built bodies, kernel-derived
/// fields) — and `None` routes the general rung permanently. There is
/// no numeric arm and there will not be one: comparing the stored
/// values would be measurement masquerading as structure.
///
/// The two faces may live in different bodies (they do at every boolean
/// germ) — a token is a fact about an expression, not about an arena.
///
/// `RadiusEvidence` is today's one consumer-side evidence type and is
/// named for its one consumer; the channel this reads is per-field and
/// serves whatever typed evidence a later position needs.
#[must_use]
pub fn field_source_evidence<T: Real>(
    a: &Body<T>,
    a_face: FaceKey,
    b: &Body<T>,
    b_face: FaceKey,
    field: SurfaceField,
) -> RadiusEvidence {
    let of = |body: &Body<T>, face: FaceKey| {
        body.get_face(face)
            .and_then(|f| body.surface_field_source(f.surface, field))
            .cloned()
    };
    match (of(a, a_face), of(b, b_face)) {
        (Some(x), Some(y)) if x == y => RadiusEvidence::Declared,
        _ => RadiusEvidence::None,
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use geom_core::{Point3, Tol, Vec3};

    use super::*;
    use crate::euler::FaceSurface;

    fn cyl(radius: f64) -> Surface<f64> {
        Surface::Cylinder {
            origin: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            radius,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        }
    }

    /// A cube with one face re-surfaced as a cylinder of `radius`,
    /// returning the body, that face and its surface key.
    fn cube_with_cylinder(radius: f64) -> (Body<f64>, FaceKey, crate::geometry::SurfaceKey) {
        let mut body = crate::fixtures::ops_cube(Tol::witness()).body;
        let face = crate::query::all_faces(&body)[0];
        let key = body
            .set_face_surface(face, FaceSurface::New(cyl(radius)))
            .expect("a live face takes a new surface");
        (body, face, key)
    }

    /// **[`SurfaceField::ALL`] is the whole declaration**, compile-forced
    /// the way the verb vocabulary's census is: the match below is
    /// exhaustive, so a field added to the enum fails this file until it
    /// is visited, and every arm names the same total, so visiting it
    /// means writing the new count — which then reds until `ALL` and
    /// `COUNT` have both grown.
    #[test]
    fn all_is_the_whole_field_declaration() {
        let rows = match SurfaceField::CylinderRadius {
            SurfaceField::CylinderRadius => 5,
            SurfaceField::ConeHalfAngle => 5,
            SurfaceField::SphereRadius => 5,
            SurfaceField::TorusMajorRadius => 5,
            SurfaceField::TorusMinorRadius => 5,
        };
        for (i, field) in SurfaceField::ALL.iter().enumerate() {
            assert!(
                !SurfaceField::ALL[..i].contains(field),
                "{field:?} appears twice in SurfaceField::ALL"
            );
        }
        assert_eq!(
            SurfaceField::ALL.len(),
            rows,
            "SurfaceField::ALL has drifted from the declaration"
        );
        assert_eq!(
            SurfaceField::COUNT,
            rows,
            "the record row width has drifted from the declaration"
        );
        for &field in SurfaceField::ALL {
            assert!(field.index() < SurfaceField::COUNT);
        }
    }

    /// **A field belongs to exactly the kinds that store it**, which is
    /// what makes the attach door's refusal a real check rather than a
    /// formality.
    #[test]
    fn a_field_belongs_only_to_its_own_kinds() {
        let torus = Surface::Torus {
            center: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            major_radius: 2.0,
            minor_radius: 0.5,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let plane = Surface::Plane {
            origin: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        assert!(SurfaceField::CylinderRadius.belongs_to(&cyl(1.0)));
        assert!(!SurfaceField::SphereRadius.belongs_to(&cyl(1.0)));
        assert!(SurfaceField::TorusMinorRadius.belongs_to(&torus));
        assert!(SurfaceField::TorusMajorRadius.belongs_to(&torus));
        assert!(!SurfaceField::CylinderRadius.belongs_to(&torus));
        for &field in SurfaceField::ALL {
            assert!(
                !field.belongs_to(&plane),
                "a plane stores no scalar a parameter lands in, yet {field:?} claimed it"
            );
        }
    }

    /// **The attach door refuses both caller bugs**, rather than
    /// recording identity where nothing will ever read it.
    #[test]
    fn the_attach_door_refuses_a_stale_key_and_a_foreign_field() {
        let (mut body, _face, key) = cube_with_cylinder(1.0);
        let token = ParamSource::from_lowered(b"r");
        assert_eq!(
            body.set_surface_field_source(key, SurfaceField::SphereRadius, token.clone()),
            Err(ParamAttachError::FieldNotOnKind {
                field: SurfaceField::SphereRadius
            }),
            "a cylinder has no sphere radius"
        );
        let stale = {
            let (mut b, _f, k) = cube_with_cylinder(1.0);
            b.set_surface_field_source(k, SurfaceField::CylinderRadius, token.clone())
                .expect("its own key resolves");
            k
        };
        let mut other = Body::<f64>::new();
        assert_eq!(
            other.set_surface_field_source(stale, SurfaceField::CylinderRadius, token),
            Err(ParamAttachError::StaleKey),
            "attaching identity to nothing is refused loudly"
        );
    }

    /// **The evidence door's whole truth table.** Every pair below
    /// carries BIT-IDENTICAL radii, so each `None` is a refusal a value
    /// comparison would have called `Declared` — which is the
    /// never-infer rule made executable rather than asserted.
    #[test]
    fn evidence_is_token_equality_and_nothing_else() {
        let r = ParamSource::from_lowered(b"radius-r");
        let s = ParamSource::from_lowered(b"radius-s");
        let mk = |token: Option<&ParamSource>| {
            let (mut body, face, key) = cube_with_cylinder(1.0);
            if let Some(t) = token {
                body.set_surface_field_source(key, SurfaceField::CylinderRadius, t.clone())
                    .expect("a live cylinder key");
            }
            (body, face)
        };
        let (a, fa) = mk(Some(&r));
        let (b, fb) = mk(Some(&r));
        let (c, fc) = mk(Some(&s));
        let (d, fd) = mk(None);
        let ev = |x: &Body<f64>, fx, y: &Body<f64>, fy| {
            field_source_evidence(x, fx, y, fy, SurfaceField::CylinderRadius)
        };
        assert_eq!(ev(&a, fa, &b, fb), RadiusEvidence::Declared, "same token");
        assert_eq!(ev(&a, fa, &c, fc), RadiusEvidence::None, "different tokens");
        assert_eq!(ev(&a, fa, &d, fd), RadiusEvidence::None, "one side absent");
        assert_eq!(ev(&d, fd, &d, fd), RadiusEvidence::None, "both absent");
    }

    /// **A record dies with its surface.** The side table is parallel to
    /// the arena, so orphan hygiene must reach it or a re-minted key
    /// would inherit a stranger's identity.
    #[test]
    fn a_record_dies_with_the_surface_it_names() {
        let (mut body, face, key) = cube_with_cylinder(1.0);
        body.set_surface_field_source(
            key,
            SurfaceField::CylinderRadius,
            ParamSource::from_lowered(b"r"),
        )
        .expect("a live cylinder key");
        assert!(
            body.surface_field_source(key, SurfaceField::CylinderRadius)
                .is_some()
        );
        let plane = Surface::Plane {
            origin: Point3::new(0.0, 0.0, 1.0),
            normal: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        body.set_face_surface(face, FaceSurface::New(plane))
            .expect("a live face takes a new surface");
        assert!(
            body.surface_field_source(key, SurfaceField::CylinderRadius)
                .is_none(),
            "the record outlived the surface it named"
        );
    }
}
