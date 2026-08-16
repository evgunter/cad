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

// #234: `DuplicateName` — the refusal of `NameTable::insert` — was the
// closure property's second stated exception until `editor_core`
// re-exported it at its root. Destructuring it by a `pncad::` path is
// what "nameable" means here; the field's type is named too, so the
// whole payload has a writable path and not just the outer struct.
fn duplicate_name_payload(e: &pncad::select::DuplicateName) {
    named::<&StableName>(&e.name);
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
    named(duplicate_name_payload as fn(&pncad::select::DuplicateName));
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
    let square: ClosedLoop<f64> = Open
        .at(p2(0.0, 0.0))
        .line_to(p2(2.0, 0.0))
        .and_then(|t| t.line_to(p2(2.0, 3.0)))
        .and_then(|t| t.line_to(p2(0.0, 3.0)))
        .and_then(|t| t.line_to(Start))
        .expect("the rectangle authors");
    let profile =
        validated(SketchPlane::<f64>::xy(), vec![square.into()]).expect("profile validates");
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
        let rect: ClosedLoop<f64> = Open
            .at(p2(x.0, y.0))
            .line_to(p2(x.1, y.0))
            .and_then(|t| t.line_to(p2(x.1, y.1)))
            .and_then(|t| t.line_to(p2(x.0, y.1)))
            .and_then(|t| t.line_to(Start))
            .expect("the slab rectangle authors");
        let plane = SketchPlane::from_frame(
            p3::<f64>(0.0, 0.0, z.0),
            v3(1.0, 0.0, 0.0),
            v3(0.0, 1.0, 0.0),
        );
        let profile = validated(plane, vec![rect.into()]).expect("slab profile");
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
    const SOURCES: [(&str, &str); 10] = [
        ("lib.rs", include_str!("../src/lib.rs")),
        ("prelude.rs", include_str!("../src/prelude.rs")),
        ("profile.rs", include_str!("../src/profile.rs")),
        ("select.rs", include_str!("../src/select.rs")),
        ("document.rs", include_str!("../src/document.rs")),
        ("authoring.rs", include_str!("../src/authoring.rs")),
        ("closure.rs", include_str!("../src/closure.rs")),
        ("export.rs", include_str!("../src/export.rs")),
        ("guide.rs", include_str!("../src/guide.rs")),
        ("workspace.rs", include_str!("../src/workspace.rs")),
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

/// **No raw loop-minting door is nameable through the façade** — Evan's
/// ruling on #413 (LIB-RETTAIL), enforced rather than asserted in a
/// report.
///
/// Same fallback shape as the LB13 guard above (rustdoc JSON is
/// nightly-gated on this toolchain), aimed at the exact regression the
/// ruling forbids:
///
/// 1. `pub use profile;` — the whole-crate re-export whose removal IS
///    the demotion. Re-adding it makes `pncad::profile::RawLoop`
///    importable, and with it `ProfileLoop::polygon`, one hop from the
///    prelude.
/// 2. Any `pub use` in `pncad`'s own source that names `RawLoop`.
/// 3. Any construction call — `ProfileLoop::new` / `ProfileLoop::polygon`
///    — written in façade source (comments excluded), which would mean
///    the façade itself still authors through the retired tier.
///
/// What this CANNOT see, stated so it is not over-trusted: the struct
/// literal. `ProfileLoop` is plain data with public fields, so
/// `ProfileLoop { vertices, tangent_joints }` type-checks wherever the
/// type is nameable, and the type must stay nameable. This guard is
/// about the AUTHORING TIER — the named, documented, prelude-carried
/// way to mint a loop from a coordinate table — not about a seal.
#[test]
fn no_raw_loop_minting_door_is_nameable_through_the_facade() {
    const SOURCES: [(&str, &str); 10] = [
        ("lib.rs", include_str!("../src/lib.rs")),
        ("prelude.rs", include_str!("../src/prelude.rs")),
        ("profile.rs", include_str!("../src/profile.rs")),
        ("select.rs", include_str!("../src/select.rs")),
        ("document.rs", include_str!("../src/document.rs")),
        ("authoring.rs", include_str!("../src/authoring.rs")),
        ("closure.rs", include_str!("../src/closure.rs")),
        ("export.rs", include_str!("../src/export.rs")),
        ("guide.rs", include_str!("../src/guide.rs")),
        ("workspace.rs", include_str!("../src/workspace.rs")),
    ];
    // Assembled at runtime for the same reason as the LB13 guard's: this
    // file is scanned by the U1 guard, and a contiguous literal would be
    // its own first match.
    let module_reexport = ["pub use ", "profile;"].concat();
    let minting = [
        ["ProfileLoop::", "new("].concat(),
        ["ProfileLoop::", "polygon("].concat(),
    ];

    let mut violations: Vec<String> = Vec::new();
    for (name, src) in SOURCES {
        let code = code_without_comments(src);
        for (n, line) in code.lines().enumerate() {
            let t = line.trim();
            if t.contains(&module_reexport) {
                violations.push(format!(
                    "{name}:{}: the whole-crate `profile` re-export is back — it makes \
                     the RawLoop minting doors nameable again (#413)",
                    n + 1
                ));
            }
            if t.contains("pub use") && t.contains("RawLoop") {
                violations.push(format!(
                    "{name}:{}: `pub use` names the raw minting trait `RawLoop` (#413)",
                    n + 1
                ));
            }
            for m in &minting {
                if t.contains(m.as_str()) {
                    violations.push(format!(
                        "{name}:{}: the façade authors through `{m}` — the retired raw tier",
                        n + 1
                    ));
                }
            }
        }
    }
    assert!(
        violations.is_empty(),
        "raw loop construction must not be presented surface — found {} violation(s):\n  {}",
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
        "profile.rs".to_string(),
        "select.rs".to_string(),
        "document.rs".to_string(),
        "authoring.rs".to_string(),
        "closure.rs".to_string(),
        "export.rs".to_string(),
        "guide.rs".to_string(),
        "workspace.rs".to_string(),
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
    let doc = pncad::document::ProfileDoc::empty_derived("all");
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

/// The seam between the two authoring surfaces (LIB-PYG1 finding 1,
/// adopted): a chain written in the PATHS algebra becomes a
/// `ProfileProgram` node, in Rust, through one door.
///
/// Before `LoopProgram::from_recorded` existed, a Rust author holding
/// a `ClosedLoop` had no way to make a document node out of it — the
/// literal helpers take raw numbers, not a recorded program — so this
/// contract had no test because it had no door.
#[test]
fn a_recorded_paths_chain_becomes_a_profile_program_node() {
    use pncad::document::{Dimension, Expr, LoopProgram, Node, ProfileProgram};

    // The guide's rounded outline: a 40 x 30 rectangle with one r = 6
    // corner filleted away. `toward` binds the rays exactly.
    let authored: ClosedLoop<f64> = Open
        .at(p2(0.0, 0.0))
        .line_to(p2(40.0, 0.0))
        .expect("a leg east")
        .toward(0.0, 1.0)
        .expect("north, exactly")
        .fillet(6.0)
        .expect("the corner rounds")
        .toward(-1.0, 0.0)
        .expect("west, exactly")
        .to(p2(0.0, 30.0))
        .expect("the arrival side ends at its far vertex")
        .line_to(Start)
        .expect("the seam closes");

    let lifted = LoopProgram::from_recorded(&authored.program).expect("the recorded program lifts");

    // Replaying the LIFTED program reproduces the AUTHORED loop bit
    // for bit — the lift re-spells the verbs, it does not re-lower.
    let steps = lifted
        .resolve(&ParamEnv::<f64>::default(), 0)
        .expect("literal arguments resolve");
    let replayed = pncad::profile::replay(&steps).expect("the lifted program replays");
    assert_eq!(replayed.vertices.len(), authored.loop_.vertices.len());
    for (got, want) in replayed.vertices.iter().zip(&authored.loop_.vertices) {
        assert_eq!(got.pos.x.to_bits(), want.pos.x.to_bits());
        assert_eq!(got.pos.y.to_bits(), want.pos.y.to_bits());
        assert_eq!(got.bulge.to_bits(), want.bulge.to_bits());
    }

    // And it evaluates as a document node.
    let doc = pncad::document::ProfileDoc::empty_derived("all");
    let (doc, profile) = doors_insert(
        doc,
        Node::Profile(ProfileProgram {
            plane: SketchPlane::xy(),
            loops: vec![lifted],
        }),
    );
    let (doc, body) = doors_insert(
        doc,
        Node::Extrude {
            profile,
            distance: Expr::literal(8.0, Dimension::Length).expect("a finite thickness"),
        },
    );
    let evaluated = doors_evaluate(&doc);
    let volume = mass_properties(
        match &evaluated.value(body).expect("the plate evaluated").payload {
            ValuePayload::Body(b) => b,
            other => panic!("expected a body, got {other:?}"),
        },
    )
    .expect("mass properties")
    .volume;
    // 40 x 30, less what the r = 6 round takes off, times 8 thick.
    let area = 40.0 * 30.0 - (36.0 - core::f64::consts::PI * 36.0 / 4.0);
    assert!((volume - area * 8.0).abs() < 1e-9, "volume {volume}");

    // The one-step complete-loop forms land in their own arms, never
    // as a chain.
    let disc = circle(p2(0.0, 0.0), 5.0).expect("a positive radius");
    assert!(matches!(
        LoopProgram::from_recorded(&disc.program).expect("the circle lifts"),
        LoopProgram::Circle { .. }
    ));
    let boss = circle_split(p2(2.0, 2.0), 0.5, 3, 0.0).expect("three arcs");
    assert!(matches!(
        LoopProgram::from_recorded(&boss.program).expect("the split circle lifts"),
        LoopProgram::CircleSplit { n: 3, .. }
    ));
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

/// A square of side `s` whose lower-left corner sits at `x`.
fn doors_square_at(s: f64, x: f64) -> pncad::document::Node<pncad::document::ProfileProgram> {
    use pncad::document::{
        Dimension, Expr, LoopProgram, Node, ProfileProgram, ProgramStep, ProgramTarget,
    };
    let lit = |v: f64| Expr::literal(v, Dimension::Length).unwrap();
    Node::Profile(ProfileProgram {
        plane: SketchPlane::xy(),
        loops: vec![LoopProgram::Chain(vec![
            ProgramStep::At([lit(x), lit(0.0)]),
            ProgramStep::LineTo(ProgramTarget::Point([lit(x + s), lit(0.0)])),
            ProgramStep::LineTo(ProgramTarget::Point([lit(x + s), lit(s)])),
            ProgramStep::LineTo(ProgramTarget::Point([lit(x), lit(s)])),
            ProgramStep::LineTo(ProgramTarget::Start),
        ])],
    })
}

/// ASM-ROOTS row 3/D-4 at the façade: the WHOLE-DOCUMENT export door
/// ships what the per-node door refuses. Two disjoint tips gather into
/// a 2-solid product, and the kernel's own STEP importer is the oracle
/// — the text re-imports as two solids whose volumes are additive.
#[test]
fn the_document_export_door_ships_the_multi_solid_product() {
    use pncad::document::{Dimension, Expr, Node};
    let lit = |v: f64| Expr::literal(v, Dimension::Length).unwrap();
    let doc = pncad::document::ProfileDoc::empty_derived("asm-roots-doc-export");
    let (doc, p0) = doors_insert(doc, doors_square_at(2.0, 0.0));
    let (doc, b0) = doors_insert(
        doc,
        Node::Extrude {
            profile: p0,
            distance: lit(1.5),
        },
    );
    let (doc, p1) = doors_insert(doc, doors_square_at(1.0, 10.0));
    let (doc, b1) = doors_insert(
        doc,
        Node::Extrude {
            profile: p1,
            distance: lit(1.0),
        },
    );
    assert_eq!(doc.roots(), &[b0, b1][..], "both tips are product roots");
    let ev = doors_evaluate(&doc);

    // The per-node door speaks for ONE node, so no node in this
    // document denotes its product; the whole-document door does.
    let text = pncad::export::export_document_step(&ev, &doc, &StepOptions::default())
        .expect("the product exports");
    let imported = import_step(&text, &ImportOptions::default()).expect("the export re-imports");
    match imported {
        pncad::step_import::StepImport::Solid { body, .. } => {
            assert_eq!(body.solids().count(), 2, "two disjoint solids ship");
            let v = mass_properties(&body)
                .expect("imported mass properties")
                .volume;
            assert!(
                (v - (2.0 * 2.0 * 1.5 + 1.0)).abs() < 1e-9,
                "imported volume {v} is not the parts' sum"
            );
        }
        other => panic!("expected a solid import, got {other:?}"),
    }
}

/// The same door's typed refusal: a profile-only document has no body
/// product, and the refusal says exactly that (ASM-ROOTS row 4).
#[test]
fn the_document_export_door_refuses_a_bodiless_document() {
    use pncad::document::ProductError;
    use pncad::export::ExportError;
    let doc = pncad::document::ProfileDoc::empty_derived("asm-roots-doc-export-bodiless");
    let (doc, _profile) = doors_insert(doc, doors_square_at(2.0, 0.0));
    let ev = doors_evaluate(&doc);
    match pncad::export::export_document_step(&ev, &doc, &StepOptions::default()) {
        Err(ExportError::Product(ProductError::NoBodyRoots)) => {}
        other => panic!("a profile-only document must refuse NoBodyRoots, got {other:?}"),
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

    let doc = pncad::document::ProfileDoc::empty_derived("all");
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
/// `tests/plate_param.v9.pncad` — the fixture the Python audit loads
/// (`crates/pncad-py/tests/test_north_star.py`) to author the
/// `set_doc_param` edit from Python. Python cannot yet author this
/// profile from scratch (audit gaps G1/G9: circles, multi-loop), so
/// the document crosses to Python through the persistence door, and
/// THIS pin keeps that crossing honest: if the scene's constants or
/// the persist schema move, the fixture cannot silently rot.
///
/// The pin is exact except the snapshot's ONE `"epsilon"` line:
/// `empty()` inherits the ambient ε (`CAD_TOLERANCE_EPS`), CI's eps
/// rows sweep it BY DESIGN, and a document authored with an explicit
/// non-ambient ε refuses evaluation (`ToleranceConflict`) under a
/// sweep — so ε is the one line that legitimately varies per run and
/// is excluded from the comparison. The checked-in fixture carries
/// the default ε (regenerate under a default environment).
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
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/plate_param.v9.pncad");
        std::fs::write(path, &text).expect("the fixture writes");
        return; // freshly written; the next compile pins it
    }
    // Everything but the swept ε line must match bit-for-bit (see the
    // doc comment above for why ε is excluded). Each side must carry
    // EXACTLY one ε line: a duplicated or missing ε line is fixture
    // damage, not sweep variance, and must fail the pin here rather
    // than rely on a downstream load refusal.
    let sans_epsilon = |t: &str| -> String {
        let (kept, excluded): (Vec<&str>, Vec<&str>) = t
            .lines()
            .partition(|l| !l.trim_start().starts_with("\"epsilon\":"));
        assert_eq!(
            excluded.len(),
            1,
            "expected exactly one \"epsilon\" line, found {}",
            excluded.len()
        );
        kept.join("\n")
    };
    assert_eq!(
        sans_epsilon(&text),
        sans_epsilon(include_str!("plate_param.v9.pncad")),
        "the saved plate_param text moved — regenerate the fixture with \
         `PNCAD_BLESS=1 cargo test -p pncad plate_param` (default env) and re-run"
    );
}

// ---- ASM-1: the workspace store (spec D-5; acceptance rows 6, 7) ----

/// A fresh scratch directory for one workspace test, cleaned up on
/// drop (best-effort — a leftover scratch dir must never fail a
/// LATER run, so each name is process-unique).
struct WsDir(std::path::PathBuf);

impl WsDir {
    fn new(tag: &str) -> Self {
        let dir = std::env::temp_dir().join(format!("pncad-ws-{tag}-{}", std::process::id()));
        // A stale same-name dir (crashed prior run of THIS pid-slot)
        // would poison the scan; remove then create.
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("scratch dir creates");
        Self(dir)
    }
    fn write(&self, name: &str, text: &str) -> std::path::PathBuf {
        let path = self.0.join(name);
        std::fs::write(&path, text).expect("fixture writes");
        path
    }
}

impl Drop for WsDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

/// A one-block document under the given derived-id label, saved.
fn ws_doc(label: &str) -> (pncad::document::ProfileDoc, String) {
    use pncad::document::{Expr, Node};
    let doc = pncad::document::ProfileDoc::empty(pncad::document::DocumentId::derive(label));
    let (doc, profile) = doors_insert(doc, doors_square(2.0));
    let (doc, _) = doors_insert(
        doc,
        Node::Extrude {
            profile,
            distance: Expr::literal(1.5, pncad::document::Dimension::Length).unwrap(),
        },
    );
    let text = pncad::document::save(&doc, &[]).expect("the document saves");
    (doc, text)
}

/// Open + resolve happy path: the scan maps ids to paths from the
/// header line alone, and a true (id, pin) reference resolves to the
/// replayed document.
#[test]
fn workspace_open_scans_headers_and_resolves_a_pinned_reference() {
    let dir = WsDir::new("ok");
    let (doc_a, text_a) = ws_doc("ws-part-a");
    let (_doc_b, text_b) = ws_doc("ws-part-b");
    dir.write("a.pncad", &text_a);
    dir.write("b.pncad", &text_b);
    // Non-documents are ignored by the scan.
    dir.write("notes.txt", "not a document");

    let ws = pncad::workspace::Workspace::open(&dir.0).expect("the scan is clean");
    assert_eq!(ws.documents().len(), 2);
    assert!(ws.documents().contains_key(&doc_a.id()));

    let wanted = pncad::document::content_pin(&doc_a).expect("the pin computes");
    let resolved = ws
        .resolve(&pncad::document::DocRef {
            id: doc_a.id(),
            pin: wanted,
        })
        .expect("a true reference resolves");
    assert!(
        resolved.bit_eq(&doc_a),
        "resolve hands back the replayed document"
    );
    // The id is data on the resolved value too.
    assert_eq!(resolved.id(), doc_a.id());
}

/// Row 6 — duplicate id: two files claiming one id refuse the OPEN,
/// typed, naming both paths.
#[test]
fn workspace_duplicate_id_refuses_naming_both_paths() {
    let dir = WsDir::new("dup");
    let (_, text) = ws_doc("ws-dup");
    let p1 = dir.write("first.pncad", &text);
    let p2 = dir.write("second.pncad", &text);

    match pncad::workspace::Workspace::open(&dir.0) {
        Err(pncad::workspace::WorkspaceError::DuplicateId { id, first, second }) => {
            assert_eq!(id, pncad::document::DocumentId::derive("ws-dup"));
            // The scan is path-sorted, so first/second are stable.
            assert_eq!((first, second), (p1, p2));
        }
        other => panic!("duplicate ids must refuse DuplicateId, got {other:?}"),
    }
}

/// Row 7 — pin mismatch at resolve: the document changed since the
/// reference was pinned; typed refusal carrying BOTH pins and the
/// accept-updated-version recourse.
#[test]
fn workspace_pin_mismatch_refuses_with_both_pins_and_recourse() {
    use pncad::document::{Dimension, DocEdit, DocParam, ParamName};
    let dir = WsDir::new("pin");
    let (doc, text) = ws_doc("ws-pin");
    let stale_pin = pncad::document::content_pin(&doc).expect("the pin computes");

    // The referenced document moves on: a recorded semantic edit.
    let edited = pncad::document::apply(
        &doc,
        &DocEdit::SetDocParam {
            name: ParamName::new("depth"),
            value: DocParam::Continuous {
                dim: Dimension::Length,
                value: 0.75,
            },
        },
    )
    .expect("the edit applies")
    .doc;
    let new_text = pncad::document::save(&edited, &[]).expect("the edited document saves");
    dir.write("part.pncad", &new_text);
    drop(text);

    let ws = pncad::workspace::Workspace::open(&dir.0).expect("the scan is clean");
    let found_pin = pncad::document::content_pin(&edited).expect("the pin computes");
    match ws.resolve(&pncad::document::DocRef {
        id: doc.id(),
        pin: stale_pin,
    }) {
        Err(pncad::workspace::WorkspaceError::PinMismatch {
            id, wanted, found, ..
        }) => {
            assert_eq!(id, doc.id());
            assert_eq!(wanted, stale_pin);
            assert_eq!(found, found_pin);
            let shown = pncad::workspace::WorkspaceError::PinMismatch {
                id,
                path: std::path::PathBuf::new(),
                wanted,
                found,
            }
            .to_string();
            assert!(
                shown.contains(pncad::workspace::PIN_MISMATCH_RECOURSE),
                "{shown}"
            );
        }
        other => panic!("a moved pin must refuse PinMismatch, got {other:?}"),
    }

    // An id the workspace has never seen refuses typed too.
    match ws.resolve(&pncad::document::DocRef {
        id: pncad::document::DocumentId::derive("ws-absent"),
        pin: stale_pin,
    }) {
        Err(pncad::workspace::WorkspaceError::UnknownId { id }) => {
            assert_eq!(id, pncad::document::DocumentId::derive("ws-absent"));
        }
        other => panic!("an unknown id must refuse UnknownId, got {other:?}"),
    }
}

/// The interactive-authoring id constructor mints DISTINCT ids from
/// OS randomness (document layer only — the kernel has no ambient
/// randomness door).
#[test]
fn random_document_ids_are_distinct() {
    let a = pncad::workspace::random_document_id().expect("OS randomness");
    let b = pncad::workspace::random_document_id().expect("OS randomness");
    assert_ne!(a, b, "128 random bits collide never in practice");
}

/// D-5's pin-the-REPLAYED-document discipline, falsified (R2
/// MINOR-2): a workspace file saved WITH a non-empty edit log. The
/// replayed state's pin resolves; the RAW snapshot's pin refuses
/// PinMismatch — so a resolve that pinned `loaded.snapshot` instead
/// of `loaded.doc` fails this row in both directions.
#[test]
fn workspace_resolve_pins_replayed_state_not_snapshot() {
    use pncad::document::{Dimension, DocEdit, DocParam, ParamName};
    let dir = WsDir::new("log");
    let (origin, _) = ws_doc("ws-logged");
    let edit = DocEdit::SetDocParam {
        name: ParamName::new("depth"),
        value: DocParam::Continuous {
            dim: Dimension::Length,
            value: 0.9,
        },
    };
    // Save snapshot + ONE-edit log; the file's current state is the
    // replayed result, and that is what a resolve must pin.
    let text = pncad::document::save(&origin, std::slice::from_ref(&edit))
        .expect("the logged document saves");
    dir.write("logged.pncad", &text);
    let replayed = pncad::document::apply(&origin, &edit)
        .expect("the edit applies")
        .doc;
    let replayed_pin = pncad::document::content_pin(&replayed).expect("the pin computes");
    let snapshot_pin = pncad::document::content_pin(&origin).expect("the pin computes");
    assert_ne!(
        replayed_pin, snapshot_pin,
        "the log is semantic here, so the two pins must differ for this row to bite"
    );

    let ws = pncad::workspace::Workspace::open(&dir.0).expect("the scan is clean");
    let resolved = ws
        .resolve(&pncad::document::DocRef {
            id: origin.id(),
            pin: replayed_pin,
        })
        .expect("the replayed state's pin is the one that resolves");
    assert!(
        resolved.bit_eq(&replayed),
        "resolve hands back the replayed state"
    );
    match ws.resolve(&pncad::document::DocRef {
        id: origin.id(),
        pin: snapshot_pin,
    }) {
        Err(pncad::workspace::WorkspaceError::PinMismatch { wanted, found, .. }) => {
            assert_eq!(wanted, snapshot_pin);
            assert_eq!(found, replayed_pin);
        }
        other => panic!("the raw snapshot's pin must refuse PinMismatch, got {other:?}"),
    }
}

// ---- ASM-2A: instantiate-part, end to end through a real workspace ----

/// A part document on disk, plus the true reference to it.
fn asm2a_part(dir: &WsDir, file: &str, label: &str) -> pncad::document::DocRef {
    let (doc, text) = ws_doc(label);
    dir.write(file, &text);
    pncad::document::DocRef {
        id: doc.id(),
        pin: pncad::document::content_pin(&doc).expect("the pin computes"),
    }
}

/// An assembly document holding `n` instances of one reference, the
/// second onward displaced along +x so the solids stay disjoint.
fn asm2a_assembly(
    label: &str,
    doc_ref: pncad::document::DocRef,
    n: usize,
) -> (
    pncad::document::ProfileDoc,
    Vec<pncad::document::RecipeNodeId>,
) {
    let mut doc = pncad::document::ProfileDoc::empty(pncad::document::DocumentId::derive(label));
    let mut ids = Vec::new();
    for i in 0..n {
        let (next, id) = doors_insert(doc, pncad::document::Node::InstantiatePart { doc_ref });
        doc = next;
        if i > 0 {
            #[allow(clippy::cast_precision_loss)]
            let dx = 10.0 * i as f64;
            doc = pncad::document::apply(
                &doc,
                &pncad::document::DocEdit::SetPlacement {
                    node: id,
                    frame: pncad::document::Frame::translation([dx, 0.0, 0.0]),
                },
            )
            .expect("the placement is accepted")
            .doc;
        }
        ids.push(id);
    }
    (doc, ids)
}

fn asm2a_eval(
    doc: &pncad::document::ProfileDoc,
    ws: &pncad::workspace::Workspace,
) -> pncad::document::Evaluation<f64> {
    let opts = pncad::document::EvalOptions {
        resolver: Some(std::sync::Arc::new(ws.clone())),
        ..pncad::document::EvalOptions::default()
    };
    pncad::document::evaluate::<f64>(doc, None, &pncad::document::CancelToken::new(), &opts)
}

/// Row 1 (E2E) — author a part, save it into a workspace, and let an
/// assembly of TWO instances at different frames evaluate through the
/// real store: a 2-solid product, volume bit-exactly 2× the part's,
/// solid order = root order.
#[test]
fn asm2a_row1_two_instances_through_a_real_workspace() {
    let dir = WsDir::new("asm2a-e2e");
    let doc_ref = asm2a_part(&dir, "bracket.pncad", "asm2a-e2e-bracket");
    let ws = pncad::workspace::Workspace::open(&dir.0).expect("the scan is clean");

    let (doc, ids) = asm2a_assembly("asm2a-e2e-asm", doc_ref, 2);
    let ev = asm2a_eval(&doc, &ws);
    let body = pncad::document::product(&doc, &ev).expect("the product gathers");
    assert_eq!(body.solids().count(), 2);
    assert_eq!(ev.part_evaluations, 1, "one part, one evaluation");

    // The part's own product, through the same doors.
    let part_doc = ws.resolve(&doc_ref).expect("resolves");
    let part_ev = asm2a_eval(&part_doc, &ws);
    let part_body = pncad::document::product(&part_doc, &part_ev).expect("the part's product");
    let vol = |b: &pncad::topo::Body<f64>| {
        pncad::topo::mass_properties(b)
            .expect("mass properties")
            .volume
    };
    assert_eq!(
        vol(&body).to_bits(),
        (2.0 * vol(&part_body)).to_bits(),
        "the assembly's volume is bit-exactly twice the part's"
    );

    // Solid order = root order: instance 0 is at the origin, instance 1
    // ten units along +x.
    let x_of = |node| match ev.value(node).map(|v| &v.payload) {
        Some(pncad::document::ValuePayload::Body(b)) => b
            .vertices()
            .filter_map(|(_, v)| b.get_point(v.point))
            .map(|p| p.x)
            .fold(f64::INFINITY, f64::min),
        other => panic!("an instance's value is a body, got {other:?}"),
    };
    assert!((x_of(ids[0]) - 0.0).abs() < 1e-12);
    assert!((x_of(ids[1]) - 10.0).abs() < 1e-12);
    assert_eq!(doc.roots(), &ids[..], "both instances are roots, in order");

    // The whole-document export door consumes the assembly with no new
    // arms — A2's uniformity, executed.
    let step = pncad::export::export_document_step(&ev, &doc, &StepOptions::default())
        .expect("the assembly exports");
    assert!(step.contains("MANIFOLD_SOLID_BREP"));
}

/// Row 5b (E2E) — A4's pin gate observed end to end: the part document
/// is edited on disk after the reference was pinned, so evaluation
/// refuses, naming the pin.
#[test]
fn asm2a_row5b_stale_pin_refuses_through_the_real_store() {
    let dir = WsDir::new("asm2a-pin");
    let doc_ref = asm2a_part(&dir, "part.pncad", "asm2a-pin-part");
    // Re-author the SAME id with different content — the "part edited
    // after the assembly pinned it" state.
    let edited = {
        let doc = pncad::document::ProfileDoc::empty(doc_ref.id);
        let (doc, profile) = doors_insert(doc, doors_square(3.0));
        let (doc, _) = doors_insert(
            doc,
            pncad::document::Node::Extrude {
                profile,
                distance: pncad::document::Expr::literal(1.5, pncad::document::Dimension::Length)
                    .unwrap(),
            },
        );
        pncad::document::save(&doc, &[]).expect("saves")
    };
    dir.write("part.pncad", &edited);

    let ws = pncad::workspace::Workspace::open(&dir.0).expect("the scan is clean");
    let (doc, ids) = asm2a_assembly("asm2a-pin-asm", doc_ref, 1);
    let ev = asm2a_eval(&doc, &ws);
    match ev.result(ids[0]) {
        Some(pncad::document::NodeResult::Failed(e)) => match &e.kind {
            pncad::document::NodeErrorKind::Part { doc_ref: r, fault } => {
                assert_eq!(*r, doc_ref, "the refusal names WHICH reference");
                assert!(
                    matches!(
                        fault,
                        pncad::document::PartFault::Unresolved {
                            fault: pncad::document::ResolveFault::PinMismatch,
                            ..
                        }
                    ),
                    "the stale pin is its own classified fault: {fault}"
                );
                let rendered = fault.to_string();
                assert!(
                    rendered.contains("pin") && rendered.contains("accept updated version"),
                    "the message names the pin and the recourse: {rendered}"
                );
            }
            other => panic!("expected a Part refusal, got {other:?}"),
        },
        other => panic!("a stale pin must refuse at evaluation, got {other:?}"),
    }
}

/// Row 1 (D9 across two fresh processes) — the assembly's product
/// volume bits are a function of the recipe alone, not of the process.
#[test]
fn asm2a_row1_product_bits_agree_across_two_fresh_processes() {
    let a = asm2a_spawn_probe("a");
    let b = asm2a_spawn_probe("b");
    assert_eq!(a, b, "two fresh processes agree bit for bit (D9)");
}

const ASM2A_PROBE_OUT: &str = "ASM2A_PROBE_OUT";

/// The child half of the two-process row: build the same assembly and
/// write its product's volume bits.
#[test]
fn asm2a_child_product_probe() {
    let Ok(out) = std::env::var(ASM2A_PROBE_OUT) else {
        return; // not the child — nothing to do
    };
    let dir = WsDir::new("asm2a-probe");
    let doc_ref = asm2a_part(&dir, "part.pncad", "asm2a-probe-part");
    let ws = pncad::workspace::Workspace::open(&dir.0).expect("the scan is clean");
    let (doc, _) = asm2a_assembly("asm2a-probe-asm", doc_ref, 2);
    let ev = asm2a_eval(&doc, &ws);
    let body = pncad::document::product(&doc, &ev).expect("gathers");
    let v = pncad::topo::mass_properties(&body)
        .expect("mass properties")
        .volume;
    std::fs::write(&out, format!("{}", v.to_bits())).expect("probe output writable");
}

fn asm2a_spawn_probe(tag: &str) -> String {
    let exe = std::env::current_exe().expect("test exe path");
    let out = std::env::temp_dir().join(format!("asm2a-probe-{tag}-{}", std::process::id()));
    let probe = match module_path!().split_once("::") {
        Some((_, m)) => format!("{m}::asm2a_child_product_probe"),
        None => "asm2a_child_product_probe".to_string(),
    };
    let status = std::process::Command::new(exe)
        .args([probe.as_str(), "--exact", "--nocapture"])
        .env(ASM2A_PROBE_OUT, &out)
        .status()
        .expect("probe spawns");
    assert!(status.success(), "probe {tag} failed");
    let bits = std::fs::read_to_string(&out).expect("probe wrote");
    let _ = std::fs::remove_file(&out);
    bits
}

// ---- ASM-2B: multi-solid referenced products, end to end ----

/// The 2B workspace: part P (one solid) on disk, sub-assembly B (two
/// instances of P, the second displaced) saved BESIDE it, and the
/// reference to B an outer assembly can pin. B is a document like any
/// other — that it holds instantiate nodes is not a kind of file.
fn asm2b_workspace(dir: &WsDir) -> (pncad::document::DocRef, pncad::document::DocRef) {
    let p = asm2a_part(dir, "part.pncad", "asm2b-part");
    let (b_doc, _) = asm2a_assembly("asm2b-sub", p, 2);
    let text = pncad::document::save(&b_doc, &[]).expect("the sub-assembly saves");
    dir.write("sub.pncad", &text);
    let b = pncad::document::DocRef {
        id: b_doc.id(),
        pin: pncad::document::content_pin(&b_doc).expect("the pin computes"),
    };
    (p, b)
}

/// Two instances of the sub-assembly, the second displaced 100 along
/// +x. Its own spacing, not 2A's: B already spans x in [0, 12], so the
/// spacing is what keeps the copies clear of each other — an
/// overlapping product is a false body the at-rest gate would NOT
/// refuse (inter-solid overlap is outside tier 3's local checks; issue
/// #382), so the fixture must not lean on the gate for it.
fn asm2b_outer(
    label: &str,
    doc_ref: pncad::document::DocRef,
) -> (
    pncad::document::ProfileDoc,
    Vec<pncad::document::RecipeNodeId>,
) {
    let mut doc = pncad::document::ProfileDoc::empty(pncad::document::DocumentId::derive(label));
    let mut ids = Vec::new();
    for i in 0..2 {
        let (next, id) = doors_insert(doc, pncad::document::Node::InstantiatePart { doc_ref });
        doc = next;
        if i > 0 {
            doc = pncad::document::apply(
                &doc,
                &pncad::document::DocEdit::SetPlacement {
                    node: id,
                    frame: pncad::document::Frame::translation([100.0, 0.0, 0.0]),
                },
            )
            .expect("the placement is accepted")
            .doc;
        }
        ids.push(id);
    }
    (doc, ids)
}

/// The product's vertex x's in ARENA order — the graft's own order, so
/// this pins WHICH SOLID CAME FIRST, not merely the aggregate volume.
fn asm2b_signature(body: &pncad::topo::Body<f64>) -> String {
    let mut s = String::new();
    for (_, v) in body.vertices() {
        if let Some(p) = body.get_point(v.point) {
            s.push_str(&format!("{};", p.x.to_bits()));
        }
    }
    s
}

/// Row 2 (E2E) — an assembly of two instances of a two-solid
/// SUB-ASSEMBLY evaluates through the real store: four solids, volume
/// bit-exactly 4× the part's, solid order = root order, and the
/// whole-document export door takes it with no new arms.
#[test]
fn asm2b_row2_sub_assembly_through_a_real_workspace() {
    let dir = WsDir::new("asm2b-e2e");
    let (p, b) = asm2b_workspace(&dir);
    let ws = pncad::workspace::Workspace::open(&dir.0).expect("the scan is clean");

    let (doc, ids) = asm2b_outer("asm2b-e2e-asm", b);
    let ev = asm2a_eval(&doc, &ws);
    let body = pncad::document::product(&doc, &ev).expect("the product gathers");
    assert_eq!(body.solids().count(), 4, "two sub-assemblies of two parts");
    // Two seams crossed, each once: B for both instances, P inside B.
    assert_eq!(ev.part_evaluations, 2);

    let vol = |b: &pncad::topo::Body<f64>| {
        pncad::topo::mass_properties(b)
            .expect("mass properties")
            .volume
    };
    let part_doc = ws.resolve(&p).expect("resolves");
    let part_ev = asm2a_eval(&part_doc, &ws);
    let part_body = pncad::document::product(&part_doc, &part_ev).expect("the part's product");
    assert_eq!(
        vol(&body).to_bits(),
        (4.0 * vol(&part_body)).to_bits(),
        "four copies of the part, bit-exactly"
    );

    // Solid order = root order: instance 0's two solids sit at x = 0
    // and x = 10 (B's own spacing), instance 1's ten further along.
    let xs = |node| match ev.value(node).map(|v| &v.payload) {
        Some(pncad::document::ValuePayload::Body(b)) => {
            assert_eq!(b.solids().count(), 2, "an instance carries both solids");
            let mut v: Vec<f64> = b
                .vertices()
                .filter_map(|(_, e)| b.get_point(e.point))
                .map(|p| p.x)
                .collect();
            v.sort_by(f64::total_cmp);
            (v[0], v[v.len() - 1])
        }
        other => panic!("an instance's value is a body, got {other:?}"),
    };
    let (lo0, hi0) = xs(ids[0]);
    let (lo1, hi1) = xs(ids[1]);
    assert!((lo0 - 0.0).abs() < 1e-12 && (hi0 - 12.0).abs() < 1e-12);
    assert!((lo1 - 100.0).abs() < 1e-12 && (hi1 - 112.0).abs() < 1e-12);

    let step = pncad::export::export_document_step(&ev, &doc, &StepOptions::default())
        .expect("the assembly exports");
    assert!(step.contains("MANIFOLD_SOLID_BREP"));
}

/// Row 2 (D9 across two fresh processes) — the nested assembly's
/// product bits AND its solid order are a function of the recipe
/// alone, not of the process.
#[test]
fn asm2b_row2_nested_product_bits_and_order_agree_across_two_processes() {
    let a = asm2b_spawn_probe("a");
    let b = asm2b_spawn_probe("b");
    assert_eq!(a, b, "two fresh processes agree bit for bit (D9)");
    assert!(a.contains(';'), "the probe really wrote a signature");
}

const ASM2B_PROBE_OUT: &str = "ASM2B_PROBE_OUT";

/// The child half of the two-process row: build the same nested
/// assembly and write its product's volume bits and solid signature.
#[test]
fn asm2b_child_product_probe() {
    let Ok(out) = std::env::var(ASM2B_PROBE_OUT) else {
        return; // not the child — nothing to do
    };
    let dir = WsDir::new("asm2b-probe");
    let (_, b) = asm2b_workspace(&dir);
    let ws = pncad::workspace::Workspace::open(&dir.0).expect("the scan is clean");
    let (doc, _) = asm2b_outer("asm2b-probe-asm", b);
    let ev = asm2a_eval(&doc, &ws);
    let body = pncad::document::product(&doc, &ev).expect("gathers");
    let v = pncad::topo::mass_properties(&body)
        .expect("mass properties")
        .volume;
    let text = format!("{}|{}", v.to_bits(), asm2b_signature(&body));
    std::fs::write(&out, text).expect("probe output writable");
}

fn asm2b_spawn_probe(tag: &str) -> String {
    let exe = std::env::current_exe().expect("test exe path");
    let out = std::env::temp_dir().join(format!("asm2b-probe-{tag}-{}", std::process::id()));
    let probe = match module_path!().split_once("::") {
        Some((_, m)) => format!("{m}::asm2b_child_product_probe"),
        None => "asm2b_child_product_probe".to_string(),
    };
    let status = std::process::Command::new(exe)
        .args([probe.as_str(), "--exact", "--nocapture"])
        .env(ASM2B_PROBE_OUT, &out)
        .status()
        .expect("probe spawns");
    assert!(status.success(), "probe {tag} failed");
    let bits = std::fs::read_to_string(&out).expect("probe wrote");
    let _ = std::fs::remove_file(&out);
    bits
}
