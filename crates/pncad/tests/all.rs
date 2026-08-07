//! The façade's acceptance suite — ONE test binary for the whole
//! crate (each extra test target cost ~1.9 s of codegen+link on the
//! 2-vCPU CI runner).
//!
//! What this file pins is the **closure property** (`pncad::closure`):
//! every type reachable through the public API of the re-exported
//! surface — every error-enum payload included — is nameable from
//! `pncad` without naming a second crate.
//!
//! The proof mechanism is the manifest, not an assertion. `pncad` has
//! **no dev-dependencies**, so this test binary cannot link a kernel
//! crate even if someone tried: every path below must resolve through
//! `pncad::`, or the file does not compile. A closure regression is
//! therefore a build failure, not a silent weakening.
//!
//! Nothing here executes geometry. These are compile-level pins:
//! functions that destructure each cross-crate payload and hand it to
//! a monomorphic sink whose signature spells the payload's type by
//! its `pncad` path. If the type stops being nameable, the sink's
//! signature stops resolving.

#![allow(clippy::panic, clippy::unwrap_used, clippy::expect_used)]

// The ONLY import root permitted in this file.
use pncad::prelude::*;

/// Consumes a value without executing anything — the sink that makes
/// each payload's type appear in a signature.
fn named<T>(_: T) {}

// ---------------------------------------------------------------
// The headline case, verbatim from the tour's manifest comment:
// "`SurfaceKind` is the payload of
//  `topo::BooleanError::CurvedBooleanUnsupported` but `topo` does not
//  re-export it, so a consumer that wants to MATCH on which surface
//  kind refused must reach for geom-brep itself."
//
// It no longer must. `SurfaceKind` is in the prelude, alongside the
// error that carries it.
// ---------------------------------------------------------------

fn boolean_refusal_surface_kind(e: &BooleanError) -> Option<&'static str> {
    match e {
        BooleanError::CurvedBooleanUnsupported {
            operand,
            face,
            kind,
        } => {
            named::<&Operand>(operand);
            named::<&FaceKey>(face);
            // The whole point: the payload is matched exhaustively,
            // by name, with no second crate in scope.
            Some(match kind {
                SurfaceKind::Plane => "plane",
                SurfaceKind::Cylinder => "cylinder",
                SurfaceKind::Sphere => "sphere",
                SurfaceKind::Cone => "cone",
                SurfaceKind::Torus => "torus",
                SurfaceKind::Nurbs => "nurbs",
            })
        }
        _ => None,
    }
}

// The identical shape in the splitting lane — the same leak, one
// module over. `SplitReduceError` is not in the prelude (splitting is
// below the corpus-wide bar), so this one goes through the module
// re-export, which is the other half of the closure claim.
fn split_reduce_refusal_surface_kind(e: &pncad::topo::SplitReduceError) -> Option<SurfaceKind> {
    match e {
        pncad::topo::SplitReduceError::CurvedBooleanUnsupported { face, kind } => {
            named::<&FaceKey>(face);
            Some(*kind)
        }
        _ => None,
    }
}

// ---------------------------------------------------------------
// The rest of the cross-crate payloads, one match apiece.
// ---------------------------------------------------------------

// topo::MassPropsError carries geom_brep::PropsError.
fn mass_props_payload(e: &MassPropsError) {
    match e {
        MassPropsError::Band { error } => named::<&pncad::geom_core::BandError>(error),
        MassPropsError::Face { face, source } => {
            named::<&FaceKey>(face);
            named::<&pncad::geom_brep::PropsError>(source);
        }
        _ => {}
    }
}

// topo::SplitJoinError carries geom_brep::SectionError.
fn split_join_payload(e: &pncad::topo::SplitJoinError) {
    if let pncad::topo::SplitJoinError::Section { source, .. } = e {
        named::<&pncad::geom_brep::SectionError>(source);
    }
}

// geom_brep::SectionError carries geom_curves::EllipseInvalid.
fn section_payload(e: &pncad::geom_brep::SectionError) {
    if let pncad::geom_brep::SectionError::Carrier(inner) = e {
        named::<&pncad::geom_curves::EllipseInvalid>(inner);
    }
}

// sweep::SkinError carries geom_curves::FitError and — the first of
// the three payloads that are NOT at their owning crate's root —
// geom_core::spline::KnotAlgebraError.
fn skin_payload(e: &pncad::sweep::SkinError) {
    match e {
        pncad::sweep::SkinError::Fit(inner) => named::<&pncad::geom_curves::FitError>(inner),
        pncad::sweep::SkinError::KnotAlgebra(inner) => {
            named::<&pncad::geom_core::spline::KnotAlgebraError>(inner);
        }
        pncad::sweep::SkinError::Structure(inner) => {
            named::<&pncad::geom_core::SplineError>(inner);
        }
        _ => {}
    }
}

// geom_curves::FitError carries the other buried one,
// geom_core::linalg::lsq::LsqError.
fn fit_payload(e: &pncad::geom_curves::FitError) {
    match e {
        pncad::geom_curves::FitError::Lsq(inner) => {
            named::<&pncad::geom_core::linalg::lsq::LsqError>(inner);
        }
        pncad::geom_curves::FitError::KnotAlgebra(inner) => {
            named::<&pncad::geom_core::spline::KnotAlgebraError>(inner);
        }
        pncad::geom_curves::FitError::Structure(inner) => {
            named::<&pncad::geom_core::SplineError>(inner);
        }
        _ => {}
    }
}

// editor_core::NodeErrorKind is the widest payload set in the tree:
// the document layer's node errors wrap every kernel operation's
// refusal, including the third buried type, sweep::fillet::FilletError.
fn node_error_payload(e: &pncad::editor_core::NodeErrorKind) {
    match e {
        pncad::editor_core::NodeErrorKind::Fillet(inner) => named::<&FilletError>(inner),
        pncad::editor_core::NodeErrorKind::Boolean(inner) => named::<&BooleanError>(inner),
        pncad::editor_core::NodeErrorKind::Transform(inner) => named::<&TransformError>(inner),
        _ => {}
    }
}

// The display/export crates carry topo entity keys.
fn tessellate_payload(e: &TessellateError) {
    if let TessellateError::UnsupportedSurface { face, .. } = e {
        named::<&FaceKey>(face);
    }
}

fn step_export_payload(e: &StepExportError) {
    if let StepExportError::UnsupportedSurface { face, .. } = e {
        named::<&FaceKey>(face);
    }
}

fn step_import_payload(e: &StepImportError) {
    if let StepImportError::Assembly { source, .. } = e {
        named::<&pncad::topo::EulerOpError>(source);
    }
}

// ---------------------------------------------------------------
// Runtime rows. The compile-level pins above are the real content;
// these keep the functions live (an unused private fn is a warning,
// and CI runs with `-D warnings`) and give the suite a green row.
// ---------------------------------------------------------------

#[test]
fn cross_crate_error_payloads_are_nameable_through_the_facade() {
    // The headline: a curved-Boolean refusal, constructed and matched
    // entirely through `pncad`.
    let refusal = BooleanError::CurvedBooleanUnsupported {
        operand: Operand::A,
        face: FaceKey::default(),
        kind: SurfaceKind::Torus,
    };
    assert_eq!(boolean_refusal_surface_kind(&refusal), Some("torus"));

    let split = pncad::topo::SplitReduceError::CurvedBooleanUnsupported {
        face: FaceKey::default(),
        kind: SurfaceKind::Cone,
    };
    assert_eq!(
        split_reduce_refusal_surface_kind(&split),
        Some(SurfaceKind::Cone)
    );

    // Keep the remaining pins referenced.
    named(mass_props_payload as fn(&MassPropsError));
    named(split_join_payload as fn(&pncad::topo::SplitJoinError));
    named(section_payload as fn(&pncad::geom_brep::SectionError));
    named(skin_payload as fn(&pncad::sweep::SkinError));
    named(fit_payload as fn(&pncad::geom_curves::FitError));
    named(node_error_payload as fn(&pncad::editor_core::NodeErrorKind));
    named(tessellate_payload as fn(&TessellateError));
    named(step_export_payload as fn(&StepExportError));
    named(step_import_payload as fn(&StepImportError));
}

/// The f64-first seam is exact: `from_f64` embeds without rounding,
/// so the façade constructors are pure renaming. A behavior change
/// here would be a defect, not a convenience.
#[test]
fn the_f64_seam_is_exact() {
    let p = p3::<f64>(0.1, -2.5, 1e-17);
    assert_eq!((p.x, p.y, p.z), (0.1, -2.5, 1e-17));
    let v = v3::<f64>(1.0 / 3.0, 0.0, f64::MIN_POSITIVE);
    assert_eq!((v.x, v.y, v.z), (1.0 / 3.0, 0.0, f64::MIN_POSITIVE));
    assert_eq!(real::<f64>(0.1), 0.1);
    let q = p2::<f64>(7.25, -0.0);
    assert_eq!((q.x, q.y), (7.25, -0.0));
}

/// The whole authoring ladder through the prelude alone: author,
/// build, validate at all three tiers, measure, tessellate, export.
/// If any rung needed a second crate, this would not compile.
#[test]
fn the_authoring_ladder_runs_on_one_dependency() {
    let square = ProfileLoop::polygon([p2(0.0, 0.0), p2(2.0, 0.0), p2(2.0, 3.0), p2(0.0, 3.0)]);
    let profile = validated(SketchPlane::<f64>::xy(), vec![square]).expect("profile validates");
    let built = extrude(&profile, Extrusion::Distance(real(0.5))).expect("extrude");

    validate(&built.body).expect("tier 1");
    validate_closed(&built.body).expect("tier 2");
    validate_pseudomanifold(&built.body, &ContactRecords::default())
        .expect("tier 2 (pseudomanifold)");
    validate_geometric(&built.body).expect("tier 3");

    let props = mass_properties(&built.body).expect("mass properties");
    assert!(
        (props.volume - 3.0).abs() < 1e-12,
        "volume {}",
        props.volume
    );

    let mesh = tessellate(&built.body, 0.05).expect("tessellate");
    assert!(!mesh.positions.is_empty());

    let mut stl_out: Vec<u8> = Vec::new();
    write_binary(&mesh, &mut stl_out).expect("stl");
    assert!(!stl_out.is_empty());

    let step = step_string(&built.body, &StepOptions::default()).expect("step");
    assert!(step.starts_with("ISO-10303-21;"));
}
