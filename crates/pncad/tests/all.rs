//! The façade's acceptance suite — ONE test binary for the whole
//! crate (each extra test target cost ~1.9 s of codegen+link on the
//! 2-vCPU CI runner).
//!
//! What this file pins is the **closure property** (`pncad::closure`):
//! every type reachable through the public API of the re-exported
//! surface — every error-enum payload included — is nameable from
//! `pncad` without naming a second crate.
//!
//! # How the pin is enforced, precisely
//!
//! An earlier version of this comment claimed the absence of
//! dev-dependencies made this binary "physically incapable" of naming
//! a kernel crate. **That was false**, and review falsified it by
//! execution: adding `use topo as _;` here compiles clean. Cargo
//! passes `--extern` for a crate's ordinary dependencies to its test
//! targets as well as its dev-dependencies, so the twelve deps are in
//! scope here regardless of what the manifest's dev-dependency
//! section says. An empty dev-dependency list is good hygiene; it is
//! not an enforcement mechanism, and this file no longer pretends it
//! is.
//!
//! What enforces the pin instead is the guard test at the bottom of
//! this file: it reads THIS FILE'S OWN SOURCE at compile time and
//! fails if any kernel crate is named outside a `pncad::` path, or if
//! any `use` statement has a root other than the façade or the
//! standard library. That is a source-level check executed as a test,
//! not a link-level impossibility — honest about its own strength,
//! and it does catch the exact regression the false claim pretended
//! to prevent.
//!
//! The remaining tests are compile-level pins: functions that
//! destructure each cross-crate payload and hand it to a monomorphic
//! sink whose signature spells the payload's type by its façade path.
//! If a type stops being nameable that way, they stop compiling.

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
fn node_error_payload(e: &pncad::document::NodeErrorKind) {
    match e {
        pncad::document::NodeErrorKind::Fillet(inner) => named::<&FilletError>(inner),
        pncad::document::NodeErrorKind::Boolean(inner) => named::<&BooleanError>(inner),
        pncad::document::NodeErrorKind::Transform(inner) => named::<&TransformError>(inner),
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

// The rows a first audit pass missed, added after review. Each was a
// genuine gap in the AUDIT, not in the property: all three were
// already nameable, which is why nothing had to change to pin them.

// `ContainError` is the sharpest of the three: it carries a
// cross-crate `Indeterminate`, and it is re-exported by its own
// crate's `boolean` module but NOT lifted to that crate's root — so
// it is reachable only by module path, exactly the shape that made
// the original leak invisible.
fn contain_payload(e: &pncad::topo::boolean::ContainError) {
    if let pncad::topo::boolean::ContainError::Escalated(inner) = e {
        named::<&pncad::geom_core::Indeterminate>(inner);
    }
}

// Defined directly in its crate's root module with no `pub use` line,
// which is why a re-export-driven scan walked past it.
fn ellipse_payload(e: &pncad::geom_curves::EllipseInvalid) {
    if let pncad::geom_curves::EllipseInvalid::Escalated(inner) = e {
        named::<&pncad::geom_core::Indeterminate>(inner);
    }
}

// A public error-adjacent struct carrying a cross-crate refusal.
fn adoption_payload(a: &pncad::step_import::AdoptionAttempt) {
    named::<&pncad::topo::EulerOpError>(&a.refusal);
    named::<&pncad::step_import::AdoptionCandidate>(&a.candidate);
}

// Two more the audit had wrong rather than missing: the mesh
// validator's error lives below its crate root, and the surfaces
// crate does define an error type (the first audit said it defined
// none).
fn mesh_validate_and_surface_projection_are_nameable() {
    named::<Option<&pncad::mesh::validate::MeshError>>(None);
    named::<Option<&pncad::geom_surfaces::SurfaceProjectionInconclusive>>(None);
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
    named(node_error_payload as fn(&pncad::document::NodeErrorKind));
    named(tessellate_payload as fn(&TessellateError));
    named(step_export_payload as fn(&StepExportError));
    named(step_import_payload as fn(&StepImportError));
    named(contain_payload as fn(&pncad::topo::boolean::ContainError));
    named(ellipse_payload as fn(&pncad::geom_curves::EllipseInvalid));
    named(adoption_payload as fn(&pncad::step_import::AdoptionAttempt));
    mesh_validate_and_surface_projection_are_nameable();
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

/// The validation ladder as the corpus actually walks it.
///
/// Tiers 1 and 2 run on every body. Tier 3 and tier 3′ are
/// **alternatives, not both**: a Boolean result validates as it is,
/// with the operation's own declared contacts (3′); everything else
/// goes through the plain geometric gate (3). An earlier version of
/// this test ran both unconditionally against empty `ContactRecords`,
/// which happens to pass on an all-planar box and misleads anyone who
/// copies it — on a curved body the census gate refuses with
/// `CensusUnsupported`. This mirrors the corpus's real conditional
/// instead.
fn ladder(body: &pncad::topo::Body<f64>, contacts: Option<&ContactRecords>) {
    validate(body).expect("tier 1: structural");
    validate_closed(body).expect("tier 2: closed solid");
    match contacts {
        // 3′ — the Boolean-result path, with the op's declarations.
        Some(declared) => {
            validate_pseudomanifold(body, declared).expect("tier 3': declared-contact");
        }
        // 3 — everything else.
        None => validate_geometric(body).expect("tier 3: geometric"),
    }
}

/// The whole authoring ladder through the prelude alone: author,
/// build, validate, measure, tessellate, export. If any rung needed a
/// second crate, this would not compile.
#[test]
fn the_authoring_ladder_runs_on_one_dependency() {
    let square = ProfileLoop::polygon([p2(0.0, 0.0), p2(2.0, 0.0), p2(2.0, 3.0), p2(0.0, 3.0)]);
    let profile = validated(SketchPlane::<f64>::xy(), vec![square]).expect("profile validates");
    let built = extrude(&profile, Extrusion::Distance(real(0.5))).expect("extrude");

    // A primitive body: no declared contacts, so the tier-3 arm.
    ladder(&built.body, None);

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

/// The other arm of the ladder: a Boolean result carries its own
/// declared contacts and validates at tier 3′ with them. Also the
/// end-to-end proof that the Boolean vocabulary is prelude-complete.
#[test]
fn a_boolean_result_validates_at_tier_3_prime() {
    // An axis-aligned box [x0,x1]x[y0,y1]x[z0,z1].
    let slab = |x: (f64, f64), y: (f64, f64), z: (f64, f64)| {
        let rect = polygon(&[(x.0, y.0), (x.1, y.0), (x.1, y.1), (x.0, y.1)]);
        let plane = SketchPlane::from_frame(
            p3::<f64>(0.0, 0.0, z.0),
            v3(1.0, 0.0, 0.0),
            v3(0.0, 1.0, 0.0),
        );
        let profile = validated(plane, vec![rect]).expect("slab profile");
        extrude(&profile, Extrusion::Distance(real(z.1 - z.0)))
            .expect("slab extrude")
            .body
    };

    // The post is strictly interior in x and y and pokes out of the
    // base's top, so the two bodies genuinely interpenetrate and NO
    // pair of faces is coincident. That matters: the kernel never
    // infers coincidence from values, so two boxes merely TOUCHING on
    // a shared plane refuse with `UndeclaredCoincidence` until the
    // author declares the contact. (An earlier draft of this test did
    // exactly that and was correctly refused — fail-loud working as
    // designed. Declared-contact unions are the corpus's own subject;
    // this test wants the plain seamed path.)
    let base = slab((0.0, 3.0), (0.0, 2.0), (0.0, 1.0)); // 6.0
    let post = slab((0.5, 1.5), (0.5, 1.5), (0.5, 2.0)); // 1.5, of which 0.5 is inside

    let BooleanResult::Body(result) = union(&base, &post).expect("union") else {
        panic!("the two bodies interpenetrate — the union is a real body");
    };

    // The tier-3′ arm, with the operation's OWN contacts — not an
    // empty set. This is what makes 3′ meaningful.
    ladder(&result.body, Some(&result.contacts));

    let props = mass_properties(&result.body).expect("mass properties");
    assert!(
        (props.volume - 7.0).abs() < 1e-12,
        "6.0 + 1.5 - 0.5 overlap = 7.0, got {}",
        props.volume
    );
}

// ---------------------------------------------------------------
// The mechanical pin for the closure property (see the module docs
// for why the manifest is NOT the mechanism).
// ---------------------------------------------------------------

/// Reads this file's own source and fails if it reaches a kernel
/// crate by any route other than a `pncad::` path.
///
/// Two checks, because there are two ways to name a crate: a `use`
/// statement (`use topo as _;` — the exact form that falsified the
/// previous claim, and which has no path separator for a path scan to
/// catch), and an inline qualified path (a bare kernel crate name
/// followed by a path separator). The guard is a plain
/// text scan, deliberately: a parser would be more precise and far
/// more machinery than a one-file invariant deserves, and a text scan
/// errs toward false ALARM rather than false confidence — the safe
/// direction for a guard whose whole job is to not overpromise.
/// Strips `//` comments so the guard judges CODE, not prose — the
/// docs above quote the original leak by its real name on purpose,
/// and documentation naming a thing is not code reaching for it.
fn code_without_comments(src: &str) -> String {
    // Written as code points, not character literals: this function's
    // own source is part of what the guard scans, and a literal quote
    // here would corrupt the string-state tracking below.
    const DQUOTE: u8 = 0x22;
    const BACKSLASH: u8 = 0x5c;
    const SLASH: u8 = 0x2f;

    let mut out = String::with_capacity(src.len());
    for line in src.lines() {
        let b = line.as_bytes();
        let (mut i, mut in_str, mut cut) = (0usize, false, b.len());
        while i < b.len() {
            match b[i] {
                BACKSLASH if in_str => i += 1,
                DQUOTE => in_str = !in_str,
                SLASH if !in_str && b.get(i + 1) == Some(&SLASH) => {
                    cut = i;
                    break;
                }
                _ => {}
            }
            i += 1;
        }
        out.push_str(&line[..cut]);
        out.push('\n'); // preserved, so reported line numbers stay true
    }
    out
}

#[test]
fn this_file_reaches_the_kernel_only_through_pncad() {
    const FACADE: &str = "pncad";
    let src = code_without_comments(include_str!("all.rs"));
    let src: &str = &src;
    // The re-exported crates, plus the one deliberately left interior.
    const KERNEL: [&str; 13] = [
        "bvh",
        "editor_core",
        "geom_brep",
        "geom_core",
        "geom_curves",
        "geom_surfaces",
        "mesh",
        "profile",
        "step_export",
        "step_import",
        "stl",
        "sweep",
        "topo",
    ];

    let mut violations: Vec<String> = Vec::new();

    // Check 1: every `use` statement's root is the façade or std.
    for (n, line) in src.lines().enumerate() {
        let t = line.trim_start();
        let Some(rest) = t.strip_prefix("use ") else {
            continue;
        };
        let root: String = rest
            .chars()
            .take_while(|c| c.is_alphanumeric() || *c == '_')
            .collect();
        if !matches!(root.as_str(), "pncad" | "std" | "core" | "alloc") {
            violations.push(format!("line {}: `use {root}` — not the façade", n + 1));
        }
    }

    // Check 2: no kernel crate name appears as a path root except
    // immediately behind the façade's own prefix.
    let facade_prefix = format!("{FACADE}::");
    for name in KERNEL {
        let needle = format!("{name}::");
        let mut from = 0usize;
        while let Some(off) = src[from..].find(&needle) {
            let at = from + off;
            from = at + needle.len();
            let before = &src[..at];
            // Not a path root if it is the tail of a longer identifier
            // (e.g. `..._mesh::`), and fine if the façade introduces it.
            let is_root = !before
                .chars()
                .next_back()
                .is_some_and(|c| c.is_alphanumeric() || c == '_');
            if is_root && !before.ends_with(&facade_prefix) {
                let line = before.matches('\n').count() + 1;
                violations.push(format!("line {line}: `{name}` named outside the façade"));
            }
        }
    }

    // The third route. The needle is assembled at runtime rather than
    // written as one literal, because this file scans ITSELF: a
    // contiguous literal would be its own first match. (The guard
    // caught exactly that on its first run — a fair sign it works.)
    let extern_decl = ["extern", "crate"].join(" ");
    assert!(
        !src.contains(&extern_decl),
        "an `extern` declaration bypasses both checks above"
    );

    assert!(
        violations.is_empty(),
        "this file must reach the kernel only through `{FACADE}::` — found {} violation(s):\n  {}",
        violations.len(),
        violations.join("\n  ")
    );
}

// ---------------------------------------------------------------
// LB13: the document layer's boundary, guarded.
// ---------------------------------------------------------------

/// **No arena key is nameable through the façade's document-layer
/// surface** — the LB13 boundary, enforced rather than asserted in a
/// report.
///
/// The intended enforcement was a rustdoc-JSON scan of `pncad`'s
/// public API. This toolchain is stable-only (1.97.0) and
/// `--output-format json` is nightly-gated; installing a nightly and
/// teaching CI to use it is a CI change, which is outside this unit's
/// fence. So this is the FALLBACK, built on the U1 self-scanning
/// pattern one file wider — and it is aimed at the exact regression
/// LB13 forbids, not at a vague resemblance to it:
///
/// 1. `pub use editor_core;` — the whole-crate re-export whose removal
///    IS LB13(a). Re-adding it makes `pncad::editor_core::EntityRef`
///    nameable again, and nothing else in the tree would notice.
/// 2. Any `pub use` in `pncad`'s own source that names `EntityRef`,
///    `EntityKey`, or `Entry` — the LIB-U5 seal, kept sealed.
///
/// What this fallback CANNOT see (stated so the next reader does not
/// over-trust it): a key type re-exported under an alias, or one
/// reachable as an associated type or a public field of something
/// this list does allow. A rustdoc-JSON check would catch those; when
/// a nightly is available to CI, replace this test with one.
#[test]
fn no_arena_key_is_nameable_through_the_facade_document_surface() {
    // Every file of the façade's own source. A new module added here
    // without being listed is caught by the companion test below.
    const SOURCES: [(&str, &str); 8] = [
        ("lib.rs", include_str!("../src/lib.rs")),
        ("prelude.rs", include_str!("../src/prelude.rs")),
        ("select.rs", include_str!("../src/select.rs")),
        ("document.rs", include_str!("../src/document.rs")),
        ("authoring.rs", include_str!("../src/authoring.rs")),
        ("closure.rs", include_str!("../src/closure.rs")),
        ("export.rs", include_str!("../src/export.rs")),
        ("guide.rs", include_str!("../src/guide.rs")),
    ];
    // Assembled at runtime: this file is itself scanned by the U1
    // guard, and a contiguous literal would be its own first match.
    let module_reexport = ["pub use editor", "core;"].join("_");
    let keys = ["EntityRef", "EntityKey", "Entry"];

    let mut violations: Vec<String> = Vec::new();
    for (name, src) in SOURCES {
        let code = code_without_comments(src);
        for (n, line) in code.lines().enumerate() {
            let t = line.trim();
            if t.contains(&module_reexport) {
                violations.push(format!(
                    "{name}:{}: the whole-crate `editor_core` re-export is back — \
                     it makes arena keys nameable again (LB13)",
                    n + 1
                ));
            }
            if !t.contains("pub use") {
                continue;
            }
            for k in keys {
                // Word-boundary check: `EntityKind` must not trip on
                // the `EntityKey` needle.
                let mut from = 0usize;
                while let Some(off) = t[from..].find(k) {
                    let at = from + off;
                    from = at + k.len();
                    let after_ok = !t[from..]
                        .chars()
                        .next()
                        .is_some_and(|c| c.is_alphanumeric() || c == '_');
                    let before_ok = !t[..at]
                        .chars()
                        .next_back()
                        .is_some_and(|c| c.is_alphanumeric() || c == '_');
                    if before_ok && after_ok {
                        violations.push(format!(
                            "{name}:{}: `pub use` names the arena key `{k}` (LIB-U5 seal)",
                            n + 1
                        ));
                    }
                }
            }
        }
    }
    assert!(
        violations.is_empty(),
        "the document layer must expose only its curated surface — found {} violation(s):\n  {}",
        violations.len(),
        violations.join("\n  ")
    );
}

/// The guard above scans a FIXED file list; a new façade module that
/// is not listed would be unguarded. This pins the list against the
/// directory.
#[test]
fn the_boundary_guard_scans_every_facade_source_file() {
    let mut on_disk: Vec<String> = std::fs::read_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/src"))
        .expect("the facade's src directory")
        .map(|e| {
            e.expect("a dir entry")
                .file_name()
                .to_string_lossy()
                .into_owned()
        })
        .filter(|n| n.ends_with(".rs"))
        .collect();
    on_disk.sort();
    let mut listed = vec![
        "lib.rs".to_string(),
        "prelude.rs".to_string(),
        "select.rs".to_string(),
        "document.rs".to_string(),
        "authoring.rs".to_string(),
        "closure.rs".to_string(),
        "export.rs".to_string(),
        "guide.rs".to_string(),
    ];
    listed.sort();
    assert_eq!(
        on_disk, listed,
        "a facade source file is missing from the LB13 boundary guard's scan list"
    );
}

// ---------------------------------------------------------------
// LIB-DOORS: the curated persist doors (F1), the export door (F2),
// the result vocabulary (F3/F4), and Expr's own refusal type (F5).
// ---------------------------------------------------------------

/// The F4 set is nameable through the façade (compile-level pins,
/// same style as the payload pins above).
fn lib_doors_vocabulary_is_nameable() {
    named::<Option<pncad::document::Applied<pncad::document::ProfileProgram>>>(None);
    named::<Option<pncad::document::EditRecord>>(None);
    named::<Option<pncad::document::NodeValue<f64>>>(None);
    named::<Option<pncad::document::NodeResult<f64>>>(None);
    named::<Option<pncad::document::EvalOutcome>>(None);
    named::<Option<pncad::document::Loaded>>(None);
    named::<Option<pncad::document::PersistError>>(None);
    named::<Option<pncad::document::MigrationError>>(None);
    named::<Option<pncad::document::NonFiniteSite>>(None);
    named::<Option<pncad::document::ProgramFault>>(None);
    named::<Option<pncad::document::SnapshotError>>(None);
    named::<Option<pncad::document::DimensionError>>(None);
    named::<Option<pncad::export::ExportError>>(None);
}

/// A square profile-program node, `[0,s]²` on the xy-plane.
fn doors_square(s: f64) -> pncad::document::Node<pncad::document::ProfileProgram> {
    use pncad::document::{
        Dimension, Expr, LoopProgram, Node, ProfileProgram, ProgramStep, ProgramTarget,
    };
    let lit = |v: f64| Expr::literal(v, Dimension::Length).unwrap();
    Node::Profile(ProfileProgram {
        plane: SketchPlane::xy(),
        loops: vec![LoopProgram::Chain(vec![
            ProgramStep::At([lit(0.0), lit(0.0)]),
            ProgramStep::LineTo(ProgramTarget::Point([lit(s), lit(0.0)])),
            ProgramStep::LineTo(ProgramTarget::Point([lit(s), lit(s)])),
            ProgramStep::LineTo(ProgramTarget::Point([lit(0.0), lit(s)])),
            ProgramStep::LineTo(ProgramTarget::Start),
        ])],
    })
}

/// Insert a node, returning the (document, minted id) pair.
fn doors_insert(
    doc: pncad::document::ProfileDoc,
    node: pncad::document::Node<pncad::document::ProfileProgram>,
) -> (pncad::document::ProfileDoc, pncad::document::RecipeNodeId) {
    let applied = pncad::document::apply(&doc, &pncad::document::DocEdit::InsertNode { node })
        .expect("the edit is accepted");
    let minted = applied.record.minted.expect("an insert mints an id");
    (applied.doc, minted)
}

/// A one-box document: square(2) extruded 1.5 — volume exactly 6.0.
/// Returns (doc, profile id, body id) — the MINTED ids, so no test
/// couples to mint order (the R1/R2 NOTE).
fn doors_box_doc() -> (
    pncad::document::ProfileDoc,
    pncad::document::RecipeNodeId,
    pncad::document::RecipeNodeId,
) {
    use pncad::document::{Dimension, Expr, Node};
    let lit = |v: f64| Expr::literal(v, Dimension::Length).unwrap();
    let doc = pncad::document::ProfileDoc::empty();
    let (doc, profile) = doors_insert(doc, doors_square(2.0));
    let (doc, body) = doors_insert(
        doc,
        Node::Extrude {
            profile,
            distance: lit(1.5),
        },
    );
    (doc, profile, body)
}

fn doors_evaluate(doc: &pncad::document::ProfileDoc) -> pncad::document::Evaluation<f64> {
    pncad::document::evaluate::<f64>(
        doc,
        None,
        &pncad::document::CancelToken::new(),
        &pncad::document::EvalOptions::default(),
    )
}

#[test]
fn the_persist_doors_round_trip_through_the_facade() {
    lib_doors_vocabulary_is_nameable();
    let (doc, _, body_node) = doors_box_doc();
    let before = doors_evaluate(&doc);
    let volume = mass_properties(
        match &before.value(body_node).expect("the box evaluated").payload {
            pncad::document::ValuePayload::Body(b) => b,
            other => panic!("expected a body, got {}", other.kind_name()),
        },
    )
    .expect("mass properties")
    .volume;
    assert_eq!(volume, 6.0);

    let text = pncad::document::save(&doc, &[]).expect("the document saves");
    let header = format!("schema: {}", pncad::document::SCHEMA_VERSION);
    assert!(
        text.starts_with(&header),
        "the file speaks the current schema"
    );

    let loaded = pncad::document::load(&text).expect("the file loads");
    assert!(loaded.edits.is_empty(), "no edit log was saved");
    assert!(loaded.records.is_empty());
    assert!(
        loaded.doc.bit_eq(&doc),
        "load replays to the SAME document (D9)"
    );

    let after = doors_evaluate(&loaded.doc);
    let replayed = mass_properties(
        match &after
            .value(body_node)
            .expect("the box re-evaluated")
            .payload
        {
            pncad::document::ValuePayload::Body(b) => b,
            other => panic!("expected a body, got {}", other.kind_name()),
        },
    )
    .expect("mass properties")
    .volume;
    assert_eq!(
        volume.to_bits(),
        replayed.to_bits(),
        "bit-exact replay (D9)"
    );
}

#[test]
fn the_export_door_serves_the_one_shot_journey() {
    let (doc, _, body_node) = doors_box_doc();
    let ev = doors_evaluate(&doc);
    let step = pncad::export::step_for_node(&ev, body_node, &StepOptions::default())
        .expect("a body value exports");
    // The oracle is the kernel's own STEP importer: the exported text
    // parses and adopts as a first-class solid whose volume agrees.
    let imported = import_step(&step, &ImportOptions::default()).expect("the export re-imports");
    match imported {
        pncad::step_import::StepImport::Solid { body, .. } => {
            let v = mass_properties(&body)
                .expect("imported mass properties")
                .volume;
            assert!((v - 6.0).abs() < 1e-9, "imported volume {v} differs");
        }
        other => panic!("expected a solid import, got {other:?}"),
    }
}

#[test]
fn the_export_door_refuses_typed_not_vaguely() {
    use pncad::document::{Node, RecipeNodeId};
    use pncad::export::ExportError;
    let (doc, profile_node, first_box) = doors_box_doc();
    // A failing Boolean (undeclared coincidence) and its downstream.
    let (doc, second_profile) = doors_insert(doc, doors_square(1.0));
    let (doc, second_box) = doors_insert(
        doc,
        Node::Extrude {
            profile: second_profile,
            distance: pncad::document::Expr::literal(1.0, pncad::document::Dimension::Length)
                .unwrap(),
        },
    );
    let (doc, cut) = doors_insert(
        doc,
        Node::Boolean {
            op: pncad::document::BooleanOp::Subtract,
            a: first_box,
            b: second_box,
            declare: None,
        },
    );
    let (doc, downstream) = doors_insert(
        doc,
        Node::Boolean {
            op: pncad::document::BooleanOp::Union,
            a: cut,
            b: first_box,
            declare: None,
        },
    );
    let ev = doors_evaluate(&doc);
    let opts = StepOptions::default();
    let door = |node| pncad::export::step_for_node(&ev, node, &opts);
    assert!(matches!(
        door(profile_node),
        Err(ExportError::NotABody {
            kind: "profile",
            ..
        })
    ));
    assert!(matches!(
        door(RecipeNodeId(u64::MAX)),
        Err(ExportError::UnknownNode { .. })
    ));
    assert!(matches!(door(cut), Err(ExportError::NodeFailed { node }) if node == cut));
    assert!(matches!(
        door(downstream),
        Err(ExportError::Poisoned { node, through }) if node == downstream && through == cut
    ));
    // The typed root cause is one door away, F3's promise.
    assert!(ev.node_error(downstream).is_some());
}

#[test]
fn expr_literal_refusals_are_matchable_through_the_facade() {
    use pncad::document::{Dimension, DimensionError, Expr};
    assert!(matches!(
        Expr::literal(f64::NAN, Dimension::Length),
        Err(DimensionError::NonFiniteLiteral)
    ));
    assert!(matches!(
        Expr::literal(2.0, Dimension::Count),
        Err(DimensionError::LiteralCountIsInteger)
    ));
}

// ---------------------------------------------------------------
// R1-PARAMS: named document parameters cross the curated surface.
// ---------------------------------------------------------------

/// Author `plate_param` — the corpus' parametric acceptance scene,
/// mirrored constant for constant from
/// `crates/editor-core/tests/corpus/plate_param.rs` — through
/// `pncad::document` alone. Before R1-PARAMS this function could not
/// compile: `ParamName` and `DocParam` were not curated, which guide
/// §3.2 pinned with a `compile_fail` doctest (now flipped to the same
/// authoring as a passing one).
fn plate_param_facade_only() -> (pncad::document::ProfileDoc, pncad::document::RecipeNodeId) {
    use pncad::document::{BooleanOp, DocParam, ParamName};
    let lit = |v: f64| Expr::literal(v, Dimension::Length).expect("a finite length");
    let hole = |cx: f64, cy: f64| LoopProgram::Circle {
        centre: [lit(cx), lit(cy)],
        radius: Expr::param(ParamName::new("hole_r"), Dimension::Length),
    };

    let doc = pncad::document::ProfileDoc::empty();
    let doc = apply(
        &doc,
        &DocEdit::SetDocParam {
            name: ParamName::new("hole_r"),
            value: DocParam::Continuous {
                dim: Dimension::Length,
                value: 0.25,
            },
        },
    )
    .expect("the parameter edit applies")
    .doc;

    let outline = LoopProgram::Chain(vec![
        ProgramStep::At([lit(0.0), lit(0.0)]),
        ProgramStep::LineTo(ProgramTarget::Point([lit(4.0), lit(0.0)])),
        ProgramStep::LineTo(ProgramTarget::Point([lit(4.0), lit(2.0)])),
        ProgramStep::LineTo(ProgramTarget::Point([lit(0.0), lit(2.0)])),
        ProgramStep::LineTo(ProgramTarget::Start),
    ]);
    let (doc, profile) = doors_insert(
        doc,
        Node::Profile(ProfileProgram {
            plane: SketchPlane::xy(),
            loops: vec![outline, hole(1.0, 1.0), hole(2.2, 1.0)],
        }),
    );
    let (doc, plate) = doors_insert(
        doc,
        Node::Extrude {
            profile,
            distance: lit(0.5),
        },
    );
    let (doc, tab_p) = doors_insert(
        doc,
        Node::Profile(ProfileProgram {
            plane: SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, 0.125))),
            loops: vec![
                LoopProgram::polygon([(3.5, 1.75), (4.5, 1.75), (4.5, 2.5), (3.5, 2.5)])
                    .expect("finite tab corners"),
            ],
        }),
    );
    let (doc, tab) = doors_insert(
        doc,
        Node::Extrude {
            profile: tab_p,
            distance: lit(0.25),
        },
    );
    let (doc, solid) = doors_insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Union,
            a: plate,
            b: tab,
            declare: None,
        },
    );
    (doc, solid)
}

/// R1-PARAMS: `plate_param` authors façade-only, evaluates to the
/// corpus scene's analytic oracle, and its saved text is pinned as
/// `tests/plate_param.v4.pncad` — the fixture the Python audit loads
/// (`crates/pncad-py/tests/test_north_star.py`) to author the
/// `set_doc_param` edit from Python. Python cannot yet author this
/// profile from scratch (audit gaps G1/G9: circles, multi-loop), so
/// the document crosses to Python through the persistence door, and
/// THIS pin keeps that crossing honest: if the scene's constants or
/// the persist schema move, the fixture cannot silently rot.
#[test]
fn plate_param_authors_facade_only_and_its_saved_text_is_pinned() {
    use pncad::document::BooleanValue;
    let (doc, solid) = plate_param_facade_only();

    let ev = evaluate::<f64>(&doc, None, &CancelToken::new(), &EvalOptions::default());
    let pncad::document::NodeResult::Ok(value) = ev.result(solid).expect("the node is live") else {
        panic!("plate_param evaluated");
    };
    let ValuePayload::Boolean(BooleanValue::Body { body, .. }) = &value.payload else {
        panic!("a union yields a body");
    };
    let volume = mass_properties(body.as_ref())
        .expect("mass properties")
        .volume;
    // Plate + tab − their overlap − two cylinders of radius 0.25: the
    // same closed form `switch_plate_param.rs` asserts, tab included.
    let oracle = 4.0 * 2.0 * 0.5 + 1.0 * 0.75 * 0.25
        - 0.5 * 0.25 * 0.25
        - 2.0 * core::f64::consts::PI * 0.25 * 0.25 * 0.5;
    assert!(
        (volume - oracle).abs() < 1e-6,
        "volume {volume} vs the plate_param oracle {oracle}"
    );

    let text = pncad::document::save(&doc, &[]).expect("the document saves");
    if std::env::var_os("PNCAD_BLESS").is_some() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/plate_param.v4.pncad");
        std::fs::write(path, &text).expect("the fixture writes");
        return; // freshly written; the next compile pins it
    }
    assert_eq!(
        text,
        include_str!("plate_param.v4.pncad"),
        "the saved plate_param text moved — regenerate the fixture with \
         `PNCAD_BLESS=1 cargo test -p pncad plate_param` and re-run"
    );
}
