//! The façade's acceptance suite — ONE test binary for the whole
//! crate: on the 2-vCPU CI runner the per-binary codegen+link constant
//! dominated the build job. The figure is not restated here — the
//! LINK/DEBUGINFO note in .github/workflows/ci.yml carries it with its
//! date and provenance run.
//!
//! What this file pins is the **closure property** (the crate docs'
//! contract clause 1): every type reachable through the public API of
//! the re-exported surface — every error-enum payload included — is
//! nameable from `pncad` without naming a second crate.
//!
//! # How the pin is enforced, precisely
//!
//! The absence of dev-dependencies does NOT make this binary
//! incapable of naming a kernel crate: adding `use topo as _;` here
//! compiles clean. Cargo passes `--extern` for a crate's ordinary
//! dependencies to its test targets as well as its dev-dependencies,
//! so every crate this one depends on is in scope here regardless of
//! what the manifest's dev-dependency section says. An empty
//! dev-dependency list is good hygiene; it is not an enforcement
//! mechanism.
//!
//! What enforces the pin instead is the guard test at the bottom of
//! this file: it reads THIS FILE'S OWN SOURCE at compile time and
//! fails if any kernel crate is named outside a `pncad::` path, or if
//! any `use` statement has a root other than the façade or the
//! standard library. That is a source-level check executed as a test,
//! not a link-level impossibility — honest about its own strength.
//!
//! The remaining tests are compile-level pins: functions that
//! destructure each cross-crate payload and hand it to a monomorphic
//! sink whose signature spells the payload's type by its façade path.
//! If a type stops being nameable that way, they stop compiling.

#![allow(clippy::panic, clippy::unwrap_used, clippy::expect_used)]

// The ONLY import root permitted in this file.
use pncad::prelude::*;
use pncad::tolerance::Tol;

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
                SurfaceKind::Approx => "approx",
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

// geom_brep::SectionError carries geom::EllipseInvalid.
fn section_payload(e: &pncad::geom_brep::SectionError) {
    if let pncad::geom_brep::SectionError::Carrier(inner) = e {
        named::<&pncad::geom::EllipseInvalid>(inner);
    }
}

// sweep::SkinError carries geom::FitError and — the first of
// the three payloads that are NOT at their owning crate's root —
// geom_core::spline::KnotAlgebraError.
fn skin_payload(e: &pncad::sweep::SkinError) {
    match e {
        pncad::sweep::SkinError::Fit(inner) => named::<&pncad::geom::FitError>(inner),
        pncad::sweep::SkinError::KnotAlgebra(inner) => {
            named::<&pncad::geom_core::spline::KnotAlgebraError>(inner);
        }
        pncad::sweep::SkinError::Structure(inner) => {
            named::<&pncad::geom_core::SplineError>(inner);
        }
        _ => {}
    }
}

// geom::FitError carries the other buried one,
// geom_core::linalg::lsq::LsqError.
fn fit_payload(e: &pncad::geom::FitError) {
    match e {
        pncad::geom::FitError::Lsq(inner) => {
            named::<&pncad::geom_core::linalg::lsq::LsqError>(inner);
        }
        pncad::geom::FitError::KnotAlgebra(inner) => {
            named::<&pncad::geom_core::spline::KnotAlgebraError>(inner);
        }
        pncad::geom::FitError::Structure(inner) => {
            named::<&pncad::geom_core::SplineError>(inner);
        }
        _ => {}
    }
}

// editor_core::NodeErrorKind is the widest payload set in the tree:
// the document layer's node errors wrap every kernel operation's
// refusal, including the third buried type, sweep::blend::BlendError.
fn node_error_payload(e: &pncad::document::NodeErrorKind) {
    match e {
        pncad::document::NodeErrorKind::Blend { error, .. } => named::<&BlendError>(error),
        pncad::document::NodeErrorKind::Boolean(inner) => named::<&BooleanError>(inner),
        pncad::document::NodeErrorKind::Transform(inner) => named::<&TransformError>(inner),
        _ => {}
    }
}

// `DuplicateName` — the refusal of `NameTable::insert` — is
// re-exported at `editor_core`'s root and carried beside `NameTable`
// by `pncad::select`. Destructuring it by a `pncad::` path is
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

// `ContainError` is the sharpest of these: it carries a
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
fn ellipse_payload(e: &pncad::geom::EllipseInvalid) {
    if let pncad::geom::EllipseInvalid::Escalated(inner) = e {
        named::<&pncad::geom_core::Indeterminate>(inner);
    }
}

// A public error-adjacent struct carrying a cross-crate refusal.
fn adoption_payload(a: &pncad::step_import::AdoptionAttempt) {
    named::<&pncad::topo::EulerOpError>(&a.refusal);
    named::<&pncad::step_import::AdoptionCandidate>(&a.candidate);
}

// The mesh validator's error lives below its crate root, and the
// surfaces crate does define an error type.
fn mesh_validate_and_surface_projection_are_nameable() {
    named::<Option<&pncad::mesh::validate::MeshError>>(None);
    named::<Option<&pncad::geom::SurfaceProjectionInconclusive>>(None);
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
    named(fit_payload as fn(&pncad::geom::FitError));
    named(node_error_payload as fn(&pncad::document::NodeErrorKind));
    named(duplicate_name_payload as fn(&pncad::select::DuplicateName));
    named(tessellate_payload as fn(&TessellateError));
    named(step_export_payload as fn(&StepExportError));
    named(step_import_payload as fn(&StepImportError));
    named(contain_payload as fn(&pncad::topo::boolean::ContainError));
    named(ellipse_payload as fn(&pncad::geom::EllipseInvalid));
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
            validate_pseudomanifold(body, declared, Tol::witness())
                .expect("tier 3': declared-contact");
        }
        // 3 — everything else.
        None => validate_geometric(body, Tol::witness()).expect("tier 3: geometric"),
    }
}

/// The whole authoring ladder through the prelude alone: author,
/// build, validate, measure, tessellate, export. If any rung needed a
/// second crate, this would not compile.
#[test]
fn the_authoring_ladder_runs_on_one_dependency() {
    let square: ClosedLoop<f64> = Open
        .at(p2(0.0, 0.0))
        .line_to(p2(2.0, 0.0), Tol::witness())
        .and_then(|t| t.line_to(p2(2.0, 3.0), Tol::witness()))
        .and_then(|t| t.line_to(p2(0.0, 3.0), Tol::witness()))
        .and_then(|t| t.line_to(Start, Tol::witness()))
        .expect("the rectangle authors");
    let profile = validated(
        SketchPlane::<f64>::xy(),
        vec![square.into()],
        Tol::witness(),
    )
    .expect("profile validates");
    let built = extrude(&profile, Extrusion::Distance(real(0.5)), Tol::witness()).expect("extrude");

    // A primitive body: no declared contacts, so the tier-3 arm.
    ladder(&built.body, None);

    let props = mass_properties(&built.body, Tol::witness()).expect("mass properties");
    assert!(
        (props.volume - 3.0).abs() < 1e-12,
        "volume {}",
        props.volume
    );

    let mesh = tessellate(&built.body, 0.05, Tol::witness()).expect("tessellate");
    assert!(!mesh.positions.is_empty());

    let mut stl_out: Vec<u8> = Vec::new();
    write_binary(&mesh, &BinaryOptions::default(), &mut stl_out).expect("stl");
    assert!(!stl_out.is_empty());

    let step = step_string(&built.body, &StepOptions::default(), Tol::witness()).expect("step");
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
            .line_to(p2(x.1, y.0), Tol::witness())
            .and_then(|t| t.line_to(p2(x.1, y.1), Tol::witness()))
            .and_then(|t| t.line_to(p2(x.0, y.1), Tol::witness()))
            .and_then(|t| t.line_to(Start, Tol::witness()))
            .expect("the slab rectangle authors");
        let plane = SketchPlane::from_frame(
            p3::<f64>(0.0, 0.0, z.0),
            v3(1.0, 0.0, 0.0),
            v3(0.0, 1.0, 0.0),
        );
        let profile = validated(plane, vec![rect.into()], Tol::witness()).expect("slab profile");
        extrude(
            &profile,
            Extrusion::Distance(real(z.1 - z.0)),
            Tol::witness(),
        )
        .expect("slab extrude")
        .body
    };

    // The post is strictly interior in x and y and pokes out of the
    // base's top, so the two bodies genuinely interpenetrate and NO
    // pair of faces is coincident. That matters: the kernel never
    // infers coincidence from values, so two boxes merely TOUCHING on
    // a shared plane refuse with `UndeclaredCoincidence` until the
    // author declares the contact. (Declared-contact unions are the
    // corpus's own subject; this test wants the plain seamed path.)
    let base = slab((0.0, 3.0), (0.0, 2.0), (0.0, 1.0)); // 6.0
    let post = slab((0.5, 1.5), (0.5, 1.5), (0.5, 2.0)); // 1.5, of which 0.5 is inside

    let BooleanResult::Body(result) = union(&base, &post, Tol::witness()).expect("union") else {
        panic!("the two bodies interpenetrate — the union is a real body");
    };

    // The tier-3′ arm, with the operation's OWN contacts — not an
    // empty set. This is what makes 3′ meaningful.
    ladder(&result.body, Some(&result.contacts));

    let props = mass_properties(&result.body, Tol::witness()).expect("mass properties");
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
    const KERNEL: [&str; 12] = [
        "bvh",
        "editor_core",
        "geom",
        "geom_brep",
        "geom_core",
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

/// Every file of the façade's own source, name and text. The
/// source-scanning guards below all read this one list;
/// `the_boundary_guard_scans_every_facade_source_file` pins it
/// against the directory, so a new module cannot arrive unguarded.
const FACADE_SOURCES: [(&str, &str); 11] = [
    ("lib.rs", include_str!("../src/lib.rs")),
    ("analysis.rs", include_str!("../src/analysis.rs")),
    ("prelude.rs", include_str!("../src/prelude.rs")),
    ("profile.rs", include_str!("../src/profile.rs")),
    ("select.rs", include_str!("../src/select.rs")),
    ("document.rs", include_str!("../src/document.rs")),
    ("authoring.rs", include_str!("../src/authoring.rs")),
    ("export.rs", include_str!("../src/export.rs")),
    ("guide.rs", include_str!("../src/guide.rs")),
    ("tolerance.rs", include_str!("../src/tolerance.rs")),
    ("workspace.rs", include_str!("../src/workspace.rs")),
];

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
/// this list does allow. A rustdoc-JSON check would catch those;
/// whether CI grows one is **#696**, which carries this deferral and
/// the two others that share it.
#[test]
fn no_arena_key_is_nameable_through_the_facade_document_surface() {
    // Every file of the façade's own source. A new module added here
    // without being listed is caught by the companion test below.
    // Assembled at runtime: this file is itself scanned by the U1
    // guard, and a contiguous literal would be its own first match.
    let module_reexport = ["pub use editor", "core;"].join("_");
    let keys = ["EntityRef", "EntityKey", "Entry"];

    let mut violations: Vec<String> = Vec::new();
    for (name, src) in FACADE_SOURCES {
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
/// nightly-gated on this toolchain — **#696**), aimed at the exact
/// regression the ruling forbids:
///
/// 1. `pub use profile;` — the whole-crate re-export whose removal IS
///    the demotion. Re-adding it makes `pncad::profile::RawLoop`
///    importable, and with it `ProfileLoop::polygon`, one hop from the
///    prelude.
/// 2. Any `pub use` in `pncad`'s own source that names `RawLoop`.
/// 3. Any construction call — `ProfileLoop::new` / `ProfileLoop::polygon`
///    — written in façade source (comments excluded), which would mean
///    the façade itself still authors through the retired tier.
/// 4. Any `ProfileLoop`/`ProfileVertex` STRUCT LITERAL in façade
///    source. This row's declared blind spot until the seal landed:
///    the fields were public, so a literal type-checked wherever the
///    type was nameable, and the type must stay nameable. The fields
///    are private now and the compiler refuses a literal out of crate
///    (E0451, pinned by a `compile_fail` doctest on `ProfileLoop`), so
///    this pattern is belt-and-braces — a façade module that ever
///    reached for one would be reaching for a construction route that
///    is no longer supposed to exist at all.
///
/// The guard is about the AUTHORING TIER — the named, documented,
/// prelude-carried way to mint a loop from a coordinate table. The seal
/// is what makes that tier the only one; the two are complementary, and
/// neither alone is the claim.
#[test]
fn no_raw_loop_minting_door_is_nameable_through_the_facade() {
    // Assembled at runtime for the same reason as the LB13 guard's: this
    // file is scanned by the U1 guard, and a contiguous literal would be
    // its own first match.
    let module_reexport = ["pub use ", "profile;"].concat();
    let minting = [
        ["ProfileLoop::", "new("].concat(),
        ["ProfileLoop::", "polygon("].concat(),
        ["ProfileLoop", "{"].concat(),
        ["ProfileVertex", "{"].concat(),
    ];

    let mut violations: Vec<String> = Vec::new();
    for (name, src) in FACADE_SOURCES {
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
            // Matched against the line with ALL whitespace removed, so
            // `ProfileLoop{`, `ProfileLoop  {` and `ProfileLoop::new (`
            // are one pattern to this guard.
            let squashed: String = t.chars().filter(|c| !c.is_whitespace()).collect();
            for m in &minting {
                if squashed.contains(m.as_str()) {
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
    let mut listed: Vec<String> = FACADE_SOURCES
        .iter()
        .map(|(name, _)| (*name).to_string())
        .collect();
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
    named::<Option<pncad::document::NonFiniteSite>>(None);
    named::<Option<pncad::document::ProgramFault>>(None);
    named::<Option<pncad::document::SnapshotError>>(None);
    named::<Option<pncad::document::DimensionError>>(None);
    named::<Option<pncad::export::ExportError>>(None);
}

/// The world xy frame — the plane these door fixtures sketch on.
fn doors_xy_frame() -> pncad::document::Node<pncad::document::ProfileProgram> {
    use pncad::document::{Datum, Dimension, Expr, Node};
    let len = |v: f64| Expr::literal(v, Dimension::Length).unwrap();
    let scl = |v: f64| Expr::literal(v, Dimension::Scalar).unwrap();
    Node::Datum(Datum::Frame {
        origin: [len(0.0), len(0.0), len(0.0)],
        u: [scl(1.0), scl(0.0), scl(0.0)],
        v: [scl(0.0), scl(1.0), scl(0.0)],
    })
}

/// A square profile-program node, `[0,s]²` on `plane`.
fn doors_square(
    plane: pncad::document::RecipeNodeId,
    s: f64,
) -> pncad::document::Node<pncad::document::ProfileProgram> {
    use pncad::document::{
        Dimension, Expr, LoopProgram, Node, ProfileProgram, ProgramStep, ProgramTarget,
    };
    let lit = |v: f64| Expr::literal(v, Dimension::Length).unwrap();
    Node::Profile(ProfileProgram {
        plane,
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
    let applied = pncad::document::apply(
        &doc,
        &pncad::document::DocEdit::InsertNode { node },
        Tol::witness(),
    )
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
    let doc = pncad::document::ProfileDoc::empty_derived("all", Tol::witness());
    let (doc, plane) = doors_insert(doc, doors_xy_frame());
    let (doc, profile) = doors_insert(doc, doors_square(plane, 2.0));
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
        Tol::witness(),
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
        .line_to(p2(40.0, 0.0), Tol::witness())
        .expect("a leg east")
        .toward(0.0, 1.0, Tol::witness())
        .expect("north, exactly")
        .fillet(6.0, Tol::witness())
        .expect("the corner rounds")
        .toward(-1.0, 0.0, Tol::witness())
        .expect("west, exactly")
        .to(p2(0.0, 30.0), Tol::witness())
        .expect("the arrival side ends at its far vertex")
        .line_to(Start, Tol::witness())
        .expect("the seam closes");

    let lifted = LoopProgram::from_recorded(&authored.program).expect("the recorded program lifts");

    // Replaying the LIFTED program reproduces the AUTHORED loop bit
    // for bit — the lift re-spells the verbs, it does not re-lower.
    let steps = lifted
        .resolve(&ParamEnv::<f64>::default(), 0)
        .expect("literal arguments resolve");
    let replayed =
        pncad::profile::replay(&steps, Tol::witness()).expect("the lifted program replays");
    assert_eq!(replayed.vertices().len(), authored.loop_.vertices().len());
    for (got, want) in replayed.vertices().iter().zip(authored.loop_.vertices()) {
        assert_eq!(got.pos().x.to_bits(), want.pos().x.to_bits());
        assert_eq!(got.pos().y.to_bits(), want.pos().y.to_bits());
        assert_eq!(got.bulge().to_bits(), want.bulge().to_bits());
    }

    // And it evaluates as a document node.
    let doc = pncad::document::ProfileDoc::empty_derived("all", Tol::witness());
    let (doc, plane) = doors_insert(doc, doors_xy_frame());
    let (doc, profile) = doors_insert(
        doc,
        Node::Profile(ProfileProgram {
            plane,
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
        Tol::witness(),
    )
    .expect("mass properties")
    .volume;
    // 40 x 30, less what the r = 6 round takes off, times 8 thick.
    let area = 40.0 * 30.0 - (36.0 - core::f64::consts::PI * 36.0 / 4.0);
    assert!((volume - area * 8.0).abs() < 1e-9, "volume {volume}");

    // The one-step complete-loop forms land in their own arms, never
    // as a chain.
    let disc = circle(p2(0.0, 0.0), 5.0, Tol::witness()).expect("a positive radius");
    assert!(matches!(
        LoopProgram::from_recorded(&disc.program).expect("the circle lifts"),
        LoopProgram::Circle { .. }
    ));
    let boss = circle_split(p2(2.0, 2.0), 0.5, 3, 0.0, Tol::witness()).expect("three arcs");
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
        Tol::witness(),
    )
    .expect("mass properties")
    .volume;
    assert_eq!(volume, 6.0);

    let text = pncad::document::save(&doc, &[], Tol::witness()).expect("the document saves");
    assert!(
        text.starts_with(&format!("id: {}\n", doc.id())),
        "the file's header names the document"
    );

    let loaded = pncad::document::load(&text, Tol::witness()).expect("the file loads");
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
        Tol::witness(),
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
    let step =
        pncad::export::step_for_node(&ev, body_node, &StepOptions::default(), Tol::witness())
            .expect("a body value exports");
    // The oracle is the kernel's own STEP importer: the exported text
    // parses and adopts as a first-class solid whose volume agrees.
    let imported = import_step(&step, &ImportOptions::default(), Tol::witness())
        .expect("the export re-imports");
    match imported {
        pncad::step_import::StepImport::Solid { body, .. } => {
            let v = mass_properties(&body, Tol::witness())
                .expect("imported mass properties")
                .volume;
            assert!((v - 6.0).abs() < 1e-9, "imported volume {v} differs");
        }
        other => panic!("expected a solid import, got {other:?}"),
    }
}

/// A square of side `s` on `plane`, lower-left corner at `x`.
fn doors_square_at(
    plane: pncad::document::RecipeNodeId,
    s: f64,
    x: f64,
) -> pncad::document::Node<pncad::document::ProfileProgram> {
    use pncad::document::{
        Dimension, Expr, LoopProgram, Node, ProfileProgram, ProgramStep, ProgramTarget,
    };
    let lit = |v: f64| Expr::literal(v, Dimension::Length).unwrap();
    Node::Profile(ProfileProgram {
        plane,
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
    let doc = pncad::document::ProfileDoc::empty_derived("asm-roots-doc-export", Tol::witness());
    let (doc, plane) = doors_insert(doc, doors_xy_frame());
    let (doc, p0) = doors_insert(doc, doors_square_at(plane, 2.0, 0.0));
    let (doc, b0) = doors_insert(
        doc,
        Node::Extrude {
            profile: p0,
            distance: lit(1.5),
        },
    );
    let (doc, plane) = doors_insert(doc, doors_xy_frame());
    let (doc, p1) = doors_insert(doc, doors_square_at(plane, 1.0, 10.0));
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
    let text =
        pncad::export::export_document_step(&ev, &doc, &StepOptions::default(), Tol::witness())
            .expect("the product exports");
    let imported = import_step(&text, &ImportOptions::default(), Tol::witness())
        .expect("the export re-imports");
    match imported {
        pncad::step_import::StepImport::Solid { body, .. } => {
            assert_eq!(body.solids().count(), 2, "two disjoint solids ship");
            let v = mass_properties(&body, Tol::witness())
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
    let doc =
        pncad::document::ProfileDoc::empty_derived("asm-roots-doc-export-bodiless", Tol::witness());
    let (doc, plane) = doors_insert(doc, doors_xy_frame());
    let (doc, _profile) = doors_insert(doc, doors_square_at(plane, 2.0, 0.0));
    let ev = doors_evaluate(&doc);
    match pncad::export::export_document_step(&ev, &doc, &StepOptions::default(), Tol::witness()) {
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
    let (doc, plane) = doors_insert(doc, doors_xy_frame());
    let (doc, second_profile) = doors_insert(doc, doors_square(plane, 1.0));
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
    let door = |node| pncad::export::step_for_node(&ev, node, &opts, Tol::witness());
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

    let doc = pncad::document::ProfileDoc::empty_derived("all", Tol::witness());
    let doc = apply(
        &doc,
        &DocEdit::SetDocParam {
            name: ParamName::new("hole_r"),
            value: DocParam::continuous(Dimension::Length, 0.25),
        },
        Tol::witness(),
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
    let (doc, plane) = doors_insert(doc, doors_xy_frame());
    let (doc, profile) = doors_insert(
        doc,
        Node::Profile(ProfileProgram {
            plane,
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
    // The tab sits inside the plate's slab: its own plane, so its own
    // frame.
    let scl = |v: f64| {
        pncad::document::Expr::literal(v, pncad::document::Dimension::Scalar).unwrap()
    };
    let (doc, tab_plane) = doors_insert(
        doc,
        Node::Datum(pncad::document::Datum::Frame {
            origin: [lit(0.0), lit(0.0), lit(0.125)],
            u: [scl(1.0), scl(0.0), scl(0.0)],
            v: [scl(0.0), scl(1.0), scl(0.0)],
        }),
    );
    let (doc, tab_p) = doors_insert(
        doc,
        Node::Profile(ProfileProgram {
            plane: tab_plane,
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
    // A MEASURE and its ASSERTION (ERROR-DESIGN E3/E10), so the
    // fixture the Python audit loads carries the two node kinds whose
    // READING door Python ships (`Value.measure`, `Value.assertion`).
    // Python cannot author one — that is B-MEASURES in the binding
    // census — so, exactly as with this profile's circles, the
    // document crosses through the persistence door and this pin keeps
    // the crossing honest.
    //
    // The references are the plate's own cylindrical walls, selected
    // through the public door rather than hand-written as role paths.
    // Which two of the four the canonical order yields is not asserted
    // here: the geometric oracles for the closed forms live in
    // `editor-core`'s `m10_2_measure.rs`, and what this fixture owes
    // is a document a Python caller can READ a measure and a verdict
    // out of.
    let walls = {
        let ev = evaluate::<f64>(
            &doc,
            None,
            &CancelToken::new(),
            &EvalOptions::default(),
            Tol::witness(),
        );
        let mut found = pncad::select::select_where(
            &ev,
            plate,
            &pncad::select::Selector::of(pncad::select::NamePat::of_kind(
                pncad::select::EntityKind::Face,
            )),
            &[pncad::select::GeomPred::SurfaceKind(
                pncad::select::SurfaceKindSet::just(pncad::geom_brep::SurfaceKind::Cylinder),
            )],
            &doc.param_env::<f64>(),
            Tol::witness(),
        )
        .expect("the surface-kind atom is exact");
        found.sort();
        // Each hole's wall is TWO faces sharing one cylinder carrier, so
        // the first two in canonical order are one hole's halves and
        // measure zero apart. Take the first and the LAST, which are
        // different holes, so the measure is the holes' axis separation
        // and the Python row that reads it has a real number to pin.
        assert_eq!(found.len(), 4, "two holes, two wall faces each");
        let last = found.pop().expect("four walls");
        let first = found.remove(0);
        vec![first, last]
    };
    let (doc, measure) = doors_insert(
        doc,
        Node::measure(
            pncad::document::MeasureExpr::primitive(pncad::document::MeasurePrimitive::Distance {
                a: 0,
                b: 1,
            }),
            // Read AT the plate extrude the walls were selected from —
            // nothing places this geometry, so the reading site is that
            // node, spelled explicitly rather than assumed.
            walls
                .into_iter()
                .map(|name| pncad::document::MeasureRef::new(plate, name))
                .collect(),
        )
        .expect("both indices address a reference"),
    );
    // A distance is a magnitude, so `>= 0` holds for any selection —
    // the verdict is about the READ door, not about the geometry.
    let (doc, _) = doors_insert(
        doc,
        Node::Assertion {
            measure,
            bound: lit(0.0),
            dir: pncad::document::AssertionDir::AtLeast,
        },
    );
    (doc, solid)
}

/// R1-PARAMS: `plate_param` authors façade-only, evaluates to the
/// corpus scene's analytic oracle, and its saved text is pinned as
/// `tests/plate_param.pncad` — the fixture the Python audit loads
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

    let ev = evaluate::<f64>(
        &doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    );
    let pncad::document::NodeResult::Ok(value) = ev.result(solid).expect("the node is live") else {
        panic!("plate_param evaluated");
    };
    let ValuePayload::Boolean(BooleanValue::Body { body, .. }) = &value.payload else {
        panic!("a union yields a body");
    };
    let volume = mass_properties(body.as_ref(), Tol::witness())
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

    let text = pncad::document::save(&doc, &[], Tol::witness()).expect("the document saves");
    if std::env::var_os("PNCAD_BLESS").is_some() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/plate_param.pncad");
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
        sans_epsilon(include_str!("plate_param.pncad")),
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
    let doc = pncad::document::ProfileDoc::empty(
        pncad::document::DocumentId::derive(label),
        Tol::witness(),
    );
    let (doc, plane) = doors_insert(doc, doors_xy_frame());
    let (doc, profile) = doors_insert(doc, doors_square(plane, 2.0));
    let (doc, _) = doors_insert(
        doc,
        Node::Extrude {
            profile,
            distance: Expr::literal(1.5, pncad::document::Dimension::Length).unwrap(),
        },
    );
    let text = pncad::document::save(&doc, &[], Tol::witness()).expect("the document saves");
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

    let wanted = pncad::document::content_pin(&doc_a, Tol::witness()).expect("the pin computes");
    let resolved = ws
        .resolve(
            &pncad::document::DocRef {
                id: doc_a.id(),
                pin: wanted,
            },
            Tol::witness(),
        )
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
    let stale_pin = pncad::document::content_pin(&doc, Tol::witness()).expect("the pin computes");

    // The referenced document moves on: a recorded semantic edit.
    let edited = pncad::document::apply(
        &doc,
        &DocEdit::SetDocParam {
            name: ParamName::new("depth"),
            value: DocParam::continuous(Dimension::Length, 0.75),
        },
        Tol::witness(),
    )
    .expect("the edit applies")
    .doc;
    let new_text =
        pncad::document::save(&edited, &[], Tol::witness()).expect("the edited document saves");
    dir.write("part.pncad", &new_text);
    drop(text);

    let ws = pncad::workspace::Workspace::open(&dir.0).expect("the scan is clean");
    let found_pin =
        pncad::document::content_pin(&edited, Tol::witness()).expect("the pin computes");
    match ws.resolve(
        &pncad::document::DocRef {
            id: doc.id(),
            pin: stale_pin,
        },
        Tol::witness(),
    ) {
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
    match ws.resolve(
        &pncad::document::DocRef {
            id: pncad::document::DocumentId::derive("ws-absent"),
            pin: stale_pin,
        },
        Tol::witness(),
    ) {
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
        value: DocParam::continuous(Dimension::Length, 0.9),
    };
    // Save snapshot + ONE-edit log; the file's current state is the
    // replayed result, and that is what a resolve must pin.
    let text = pncad::document::save(&origin, std::slice::from_ref(&edit), Tol::witness())
        .expect("the logged document saves");
    dir.write("logged.pncad", &text);
    let replayed = pncad::document::apply(&origin, &edit, Tol::witness())
        .expect("the edit applies")
        .doc;
    let replayed_pin =
        pncad::document::content_pin(&replayed, Tol::witness()).expect("the pin computes");
    let snapshot_pin =
        pncad::document::content_pin(&origin, Tol::witness()).expect("the pin computes");
    assert_ne!(
        replayed_pin, snapshot_pin,
        "the log is semantic here, so the two pins must differ for this row to bite"
    );

    let ws = pncad::workspace::Workspace::open(&dir.0).expect("the scan is clean");
    let resolved = ws
        .resolve(
            &pncad::document::DocRef {
                id: origin.id(),
                pin: replayed_pin,
            },
            Tol::witness(),
        )
        .expect("the replayed state's pin is the one that resolves");
    assert!(
        resolved.bit_eq(&replayed),
        "resolve hands back the replayed state"
    );
    match ws.resolve(
        &pncad::document::DocRef {
            id: origin.id(),
            pin: snapshot_pin,
        },
        Tol::witness(),
    ) {
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
        pin: pncad::document::content_pin(&doc, Tol::witness()).expect("the pin computes"),
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
    let mut doc = pncad::document::ProfileDoc::empty(
        pncad::document::DocumentId::derive(label),
        Tol::witness(),
    );
    let mut ids = Vec::new();
    for i in 0..n {
        let (next, id) = doors_insert(doc, pncad::document::Node::instantiate_part(doc_ref));
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
                Tol::witness(),
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
    pncad::document::evaluate::<f64>(
        doc,
        None,
        &pncad::document::CancelToken::new(),
        &opts,
        Tol::witness(),
    )
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
    let body = pncad::document::product(&doc, &ev, Tol::witness()).expect("the product gathers");
    assert_eq!(body.solids().count(), 2);
    assert_eq!(ev.part_evaluations, 1, "one part, one evaluation");

    // The part's own product, through the same doors.
    let part_doc = ws.resolve(&doc_ref, Tol::witness()).expect("resolves");
    let part_ev = asm2a_eval(&part_doc, &ws);
    let part_body =
        pncad::document::product(&part_doc, &part_ev, Tol::witness()).expect("the part's product");
    let vol = |b: &pncad::topo::Body<f64>| {
        pncad::topo::mass_properties(b, Tol::witness())
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
    let step =
        pncad::export::export_document_step(&ev, &doc, &StepOptions::default(), Tol::witness())
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
        let doc = pncad::document::ProfileDoc::empty(doc_ref.id, Tol::witness());
        let (doc, plane) = doors_insert(doc, doors_xy_frame());
        let (doc, profile) = doors_insert(doc, doors_square(plane, 3.0));
        let (doc, _) = doors_insert(
            doc,
            pncad::document::Node::Extrude {
                profile,
                distance: pncad::document::Expr::literal(1.5, pncad::document::Dimension::Length)
                    .unwrap(),
            },
        );
        pncad::document::save(&doc, &[], Tol::witness()).expect("saves")
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
    let body = pncad::document::product(&doc, &ev, Tol::witness()).expect("gathers");
    let v = pncad::topo::mass_properties(&body, Tol::witness())
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

// ---- ASM-R2a: mates, end to end and across two processes ----

/// A MATED assembly: two instances of one part, the second placed by a
/// frame-coincidence mate 30 along +x (the part spans 2, so the pair
/// stays disjoint and the product is a clean two-solid gather).
///
/// The instance names are the A12 shape — an `InPart`-headed name whose
/// HEAD is the instantiate node, which is exactly what the reading edge
/// is recomputed from.
fn asm_r2a_mated_assembly(
    label: &str,
    doc_ref: pncad::document::DocRef,
) -> (
    pncad::document::ProfileDoc,
    Vec<pncad::document::RecipeNodeId>,
) {
    use pncad::document::{Alignment, AxisSense, MateFrame, MatePrimitive, Node, RecipeNodeId};
    use pncad::prelude::StableName;
    use pncad::select::{CapEnd, ContactClass, EntityKind, RoleSeg};
    let mut doc = pncad::document::ProfileDoc::empty(
        pncad::document::DocumentId::derive(label),
        Tol::witness(),
    );
    let mut ids = Vec::new();
    for _ in 0..2 {
        let (next, id) = doors_insert(doc, Node::instantiate_part(doc_ref));
        doc = next;
        ids.push(id);
    }
    let name = |node| StableName {
        kind: EntityKind::Face,
        node,
        path: vec![RoleSeg::InPart {
            of: Box::new(StableName {
                kind: EntityKind::Face,
                node: RecipeNodeId(1),
                path: vec![RoleSeg::Cap(CapEnd::Bottom)],
            }),
        }],
    };
    let axis = |origin: [f64; 3]| MateFrame {
        origin,
        axis: [0.0, 0.0, 1.0],
        reference: [1.0, 0.0, 0.0],
    };
    let (doc, _) = doors_insert(
        doc,
        Node::Mate {
            a: name(ids[0]),
            b: name(ids[1]),
            class: ContactClass::Rest,
            alignment: Alignment {
                a: axis([30.0, 0.0, 0.0]),
                b: axis([0.0, 0.0, 0.0]),
                primitive: MatePrimitive::FrameCoincidence,
                sense: AxisSense::Aligned,
                clocking: None,
            },
        },
    );
    (doc, ids)
}

/// ASM-R2a row 1, the DOCUMENT-layer half (review MINOR-2): a
/// MATE-BEARING assembly's product bits are a function of the recipe
/// alone, across two fresh processes. The editor-core suite pins two
/// evaluations within one process; a process hosts one ε, so this is
/// where the cross-process claim can actually be made.
#[test]
fn asm_r2a_mated_product_bits_agree_across_two_fresh_processes() {
    let a = asm_r2a_spawn_probe("a");
    let b = asm_r2a_spawn_probe("b");
    assert_eq!(a, b, "two fresh processes agree bit for bit (D9)");
}

const ASM_R2A_PROBE_OUT: &str = "ASM_R2A_PROBE_OUT";

/// The child half: build the same MATED assembly, solve it, and write
/// the product's volume bits beside the saved document's own bytes —
/// so the row covers evaluation AND save bytes, as D-5 asks.
#[test]
fn asm_r2a_child_mated_probe() {
    let Ok(out) = std::env::var(ASM_R2A_PROBE_OUT) else {
        return; // not the child — nothing to do
    };
    let dir = WsDir::new("asm-r2a-probe");
    let doc_ref = asm2a_part(&dir, "part.pncad", "asm-r2a-probe-part");
    let ws = pncad::workspace::Workspace::open(&dir.0).expect("the scan is clean");
    let (doc, ids) = asm_r2a_mated_assembly("asm-r2a-probe-asm", doc_ref);
    // The mate SOLVED the second instance's placement: it is recipe
    // data, not a recorded frame, so the registry stays empty.
    assert!(
        doc.placements().is_empty(),
        "the pose is solved, not stored"
    );
    let poses = pncad::document::solve_document(&doc, Tol::witness());
    let placed = poses.placement(&doc, ids[1]).expect("the pair determines");
    let ev = asm2a_eval(&doc, &ws);
    let body = pncad::document::product(&doc, &ev, Tol::witness()).expect("gathers");
    let v = pncad::topo::mass_properties(&body, Tol::witness())
        .expect("mass properties")
        .volume;
    let text = pncad::document::save(&doc, &[], Tol::witness()).expect("the document saves");
    std::fs::write(
        &out,
        format!(
            "{}\n{:?}\n{}",
            v.to_bits(),
            placed.translation,
            pncad::document::content_pin(&doc, Tol::witness()).expect("the pin computes")
        ),
    )
    .expect("probe output writable");
    let _ = text;
}

fn asm_r2a_spawn_probe(tag: &str) -> String {
    let exe = std::env::current_exe().expect("test exe path");
    let out = std::env::temp_dir().join(format!("asm-r2a-probe-{tag}-{}", std::process::id()));
    let probe = match module_path!().split_once("::") {
        Some((_, m)) => format!("{m}::asm_r2a_child_mated_probe"),
        None => "asm_r2a_child_mated_probe".to_string(),
    };
    let status = std::process::Command::new(exe)
        .args([probe.as_str(), "--exact", "--nocapture"])
        .env(ASM_R2A_PROBE_OUT, &out)
        .status()
        .expect("probe spawns");
    assert!(status.success(), "probe {tag} failed");
    let bits = std::fs::read_to_string(&out).expect("probe wrote");
    let _ = std::fs::remove_file(&out);
    bits
}

// ---- ASM-R2b: the crossing-bearing document, across two processes ----

const ASM_R2B_PROBE_OUT: &str = "ASM_R2B_PROBE_OUT";

/// The child half of ASM-R2b's D9 row (spec row 7; review NOTE-2): a
/// document that is MATED, MINTED, SPLIT, and CROSSING-BEARING, built
/// and evaluated and saved in a fresh process.
///
/// The crossing record is authored through `Node::instantiate_part_with`
/// rather than harvested from the split, and deliberately so: for a
/// PROPER mate edge no accepted cut can produce a crossing (the
/// whole-cluster precondition — see editor-core's `row5_a`), and the
/// one shape that does mint one today has semantics pending Evan's
/// AQ8 ruling. Authoring the record keeps this row about D9 — the
/// same bits from the same recipe — rather than about a semantics
/// question that may move.
#[test]
fn asm_r2b_child_crossing_probe() {
    use pncad::document::{DocEdit, Node, RecipeNodeId};
    use pncad::prelude::StableName;
    use pncad::select::{CapEnd, ContactClass, EntityKind, RoleSeg};
    let Ok(out) = std::env::var(ASM_R2B_PROBE_OUT) else {
        return; // not the child — nothing to do
    };
    let dir = WsDir::new("asm-r2b-probe");
    let doc_ref = asm2a_part(&dir, "part.pncad", "asm-r2b-probe-part");
    let ws = pncad::workspace::Workspace::open(&dir.0).expect("the scan is clean");

    // A mated pair (the minting subject), then a THIRD instance
    // carrying an authored crossing record (the wire subject).
    let (doc, ids) = asm_r2a_mated_assembly("asm-r2b-probe-asm", doc_ref);
    let record = pncad::document::InterfaceRecord {
        crossings: vec![pncad::document::InterfaceCrossing::Mate {
            mate: ids[0],
            class: ContactClass::Rest,
            outer: StableName {
                kind: EntityKind::Face,
                node: RecipeNodeId(1),
                path: vec![RoleSeg::Cap(CapEnd::Top)],
            },
            inner: StableName {
                kind: EntityKind::Face,
                node: RecipeNodeId(1),
                path: vec![RoleSeg::Cap(CapEnd::Bottom)],
            },
        }],
    };
    let doc = pncad::document::apply(
        &doc,
        &DocEdit::InsertNode {
            node: Node::instantiate_part_with(doc_ref, record),
        },
        Tol::witness(),
    )
    .expect("the crossing-bearing instance inserts")
    .doc;

    // The whole cluster split out — accepted, and the remainder is
    // itself a crossing-bearing document.
    let split = pncad::document::split(
        &doc,
        &ids.iter().copied().collect(),
        pncad::document::DocumentId::derive("asm-r2b-probe-split"),
        Tol::witness(),
    )
    .expect("a whole-cluster cut splits");
    let text =
        pncad::document::save(&split.remainder, &[], Tol::witness()).expect("the remainder saves");
    dir.write(
        "split.pncad",
        &pncad::document::save(&split.part, &[], Tol::witness()).expect("the part saves"),
    );

    let ev = asm2a_eval(&doc, &ws);
    let assembled = pncad::document::product(&doc, &ev, Tol::witness()).expect("gathers");
    let v = pncad::topo::mass_properties(&assembled, Tol::witness())
        .expect("mass properties")
        .volume;
    std::fs::write(
        &out,
        format!(
            "{}\n{}\n{}\n{}",
            v.to_bits(),
            pncad::document::content_pin(&doc, Tol::witness()).expect("the pin computes"),
            pncad::document::content_pin(&split.remainder, Tol::witness())
                .expect("the pin computes"),
            text.len()
        ),
    )
    .expect("probe output writable");
}

fn asm_r2b_spawn_probe(tag: &str) -> String {
    let exe = std::env::current_exe().expect("test exe path");
    let out = std::env::temp_dir().join(format!("asm-r2b-probe-{tag}-{}", std::process::id()));
    let probe = match module_path!().split_once("::") {
        Some((_, m)) => format!("{m}::asm_r2b_child_crossing_probe"),
        None => "asm_r2b_child_crossing_probe".to_string(),
    };
    let status = std::process::Command::new(exe)
        .args([probe.as_str(), "--exact", "--nocapture"])
        .env(ASM_R2B_PROBE_OUT, &out)
        .status()
        .expect("probe spawns");
    assert!(status.success(), "probe {tag} failed");
    let bits = std::fs::read_to_string(&out).expect("probe wrote");
    let _ = std::fs::remove_file(&out);
    bits
}

/// ASM-R2b acceptance row 7, the DOCUMENT-layer half (review NOTE-2):
/// evaluation bits AND save bytes for a mated, minted,
/// split-and-crossing-bearing document are a function of the recipe
/// alone, across two FRESH PROCESSES. editor-core's `row7` pins two
/// evaluations inside one process; a process hosts one ε, so this is
/// where the cross-process claim can actually be made.
#[test]
fn asm_r2b_crossing_bearing_bits_agree_across_two_fresh_processes() {
    let a = asm_r2b_spawn_probe("a");
    let b = asm_r2b_spawn_probe("b");
    assert_eq!(a, b, "two fresh processes agree bit for bit (D9)");
}

// ---- ASM-2B: multi-solid referenced products, end to end ----

/// The 2B workspace: part P (one solid) on disk, sub-assembly B (two
/// instances of P, the second displaced) saved BESIDE it, and the
/// reference to B an outer assembly can pin. B is a document like any
/// other — that it holds instantiate nodes is not a kind of file.
fn asm2b_workspace(dir: &WsDir) -> (pncad::document::DocRef, pncad::document::DocRef) {
    let p = asm2a_part(dir, "part.pncad", "asm2b-part");
    let (b_doc, _) = asm2a_assembly("asm2b-sub", p, 2);
    let text = pncad::document::save(&b_doc, &[], Tol::witness()).expect("the sub-assembly saves");
    dir.write("sub.pncad", &text);
    let b = pncad::document::DocRef {
        id: b_doc.id(),
        pin: pncad::document::content_pin(&b_doc, Tol::witness()).expect("the pin computes"),
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
    let mut doc = pncad::document::ProfileDoc::empty(
        pncad::document::DocumentId::derive(label),
        Tol::witness(),
    );
    let mut ids = Vec::new();
    for i in 0..2 {
        let (next, id) = doors_insert(doc, pncad::document::Node::instantiate_part(doc_ref));
        doc = next;
        if i > 0 {
            doc = pncad::document::apply(
                &doc,
                &pncad::document::DocEdit::SetPlacement {
                    node: id,
                    frame: pncad::document::Frame::translation([100.0, 0.0, 0.0]),
                },
                Tol::witness(),
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
    let body = pncad::document::product(&doc, &ev, Tol::witness()).expect("the product gathers");
    assert_eq!(body.solids().count(), 4, "two sub-assemblies of two parts");
    // Two seams crossed, each once: B for both instances, P inside B.
    assert_eq!(ev.part_evaluations, 2);

    let vol = |b: &pncad::topo::Body<f64>| {
        pncad::topo::mass_properties(b, Tol::witness())
            .expect("mass properties")
            .volume
    };
    let part_doc = ws.resolve(&p, Tol::witness()).expect("resolves");
    let part_ev = asm2a_eval(&part_doc, &ws);
    let part_body =
        pncad::document::product(&part_doc, &part_ev, Tol::witness()).expect("the part's product");
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

    let step =
        pncad::export::export_document_step(&ev, &doc, &StepOptions::default(), Tol::witness())
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
    let body = pncad::document::product(&doc, &ev, Tol::witness()).expect("gathers");
    let v = pncad::topo::mass_properties(&body, Tol::witness())
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

// ---- ASM-4: split and inline through the real store ----

/// D-1 — the workspace write side: `create` mints `{id}.pncad` and the
/// scan sees it; `resave` rewrites in place (the pin moves, and a
/// stale reference refuses typed); the misuse doors refuse typed —
/// duplicate id at create (acceptance row 3), unknown id at resave.
#[test]
fn asm4_workspace_create_and_resave() {
    use pncad::document as d;
    let dir = WsDir::new("asm4-ws");
    let mut ws = pncad::workspace::Workspace::open(&dir.0).expect("empty scan");

    let (doc, _) = ws_doc("asm4-ws-part");
    let path = ws.create(&doc, Tol::witness()).expect("the create writes");
    assert_eq!(
        path.file_name().and_then(|n| n.to_str()),
        Some(format!("{}.pncad", doc.id()).as_str()),
        "the file name is a pure function of the identity (D9)"
    );
    let doc_ref = d::DocRef {
        id: doc.id(),
        pin: d::content_pin(&doc, Tol::witness()).expect("pins"),
    };
    // A fresh scan agrees with the incremental map, and resolves.
    let reopened = pncad::workspace::Workspace::open(&dir.0).expect("rescan");
    assert!(
        reopened
            .resolve(&doc_ref, Tol::witness())
            .expect("resolves")
            .bit_eq(&doc)
    );

    // Duplicate id at create: refused naming both paths, nothing
    // written.
    let (dup, _) = ws_doc("asm4-ws-part");
    match ws.create(&dup, Tol::witness()) {
        Err(pncad::workspace::WorkspaceError::DuplicateId { id, first, second }) => {
            assert_eq!(id, doc.id());
            assert_eq!(first, path);
            assert_eq!(second, path, "the same id names the same file");
        }
        other => panic!("expected DuplicateId, got {other:?}"),
    }

    // Resave rewrites in place; the old pin no longer holds and the
    // stale reference is a typed PinMismatch (A4 — never retargeted).
    let (moved, plane) = doors_insert(doc.clone(), doors_xy_frame());
    let (moved, _) = doors_insert(moved, doors_square(plane, 3.0));
    let resaved = ws
        .resave(&moved, Tol::witness())
        .expect("the resave writes");
    assert_eq!(resaved, path, "the file keeps its path");
    let reopened = pncad::workspace::Workspace::open(&dir.0).expect("rescan");
    match pncad::workspace::Workspace::resolve(&reopened, &doc_ref, Tol::witness()) {
        Err(pncad::workspace::WorkspaceError::PinMismatch { .. }) => {}
        other => panic!("expected PinMismatch, got {other:?}"),
    }

    // Resave of an id the store never scanned refuses typed.
    let (foreign, _) = ws_doc("asm4-ws-foreign");
    match ws.resave(&foreign, Tol::witness()) {
        Err(pncad::workspace::WorkspaceError::UnknownId { id }) => {
            assert_eq!(id, foreign.id());
        }
        other => panic!("expected UnknownId, got {other:?}"),
    }
}

/// The document-layer end-to-end: split through the real store —
/// create the part, resave the remainder, reopen, and the A4 identity
/// holds; inline back through the workspace resolver and it holds
/// against the original.
#[test]
fn asm4_split_and_inline_through_the_real_store() {
    use pncad::document as d;
    let dir = WsDir::new("asm4-e2e");
    let part_ref = asm2a_part(&dir, "part.pncad", "asm4-e2e-part");
    let (doc, ids) = asm2a_assembly("asm4-e2e-asm", part_ref, 2);
    let ws = pncad::workspace::Workspace::open(&dir.0).expect("scan");
    let mut ws_mut = ws.clone();
    let ev1 = asm2a_eval(&doc, &ws);
    let body1 = d::product(&doc, &ev1, Tol::witness()).expect("gathers");
    let vol = |b: &pncad::topo::Body<f64>| {
        pncad::topo::mass_properties(b, Tol::witness())
            .expect("mass properties")
            .volume
            .to_bits()
    };

    let cut = std::collections::BTreeSet::from([ids[1]]);
    let out = d::split(
        &doc,
        &cut,
        d::DocumentId::derive("asm4-e2e-new"),
        Tol::witness(),
    )
    .expect("legal");
    ws_mut
        .create(&out.part, Tol::witness())
        .expect("the part lands in the store");
    // The assembly itself is not in this store (it was never saved);
    // saving the remainder under its id is the caller's create-or-
    // resave choice — here the assembly starts on disk too.
    let ws2 = pncad::workspace::Workspace::open(&dir.0).expect("rescan");
    let ev2 = asm2a_eval(&out.remainder, &ws2);
    let body2 = d::product(&out.remainder, &ev2, Tol::witness()).expect("gathers");
    assert_eq!(body1.solids().count(), body2.solids().count());
    assert_eq!(body1.faces().count(), body2.faces().count());
    assert_eq!(
        vol(&body1),
        vol(&body2),
        "volumes bit-equal through the store"
    );

    let inlined =
        d::inline(&out.remainder, out.instance, &ws2, Tol::witness()).expect("inlines back");
    let ev3 = asm2a_eval(&inlined.doc, &ws2);
    let body3 = d::product(&inlined.doc, &ev3, Tol::witness()).expect("gathers");
    assert_eq!(
        vol(&body1),
        vol(&body3),
        "the round trip's volume is bit-equal"
    );
    assert_eq!(body1.solids().count(), body3.solids().count());
}

/// Row 6 (D9) — split twice in FRESH processes produces byte-identical
/// documents (both sides; the minted id is caller-supplied and
/// derived, so the whole pair is a pure function of the recipe).
#[test]
fn asm4_row6_split_bytes_agree_across_two_fresh_processes() {
    let a = asm4_spawn_probe("a");
    let b = asm4_spawn_probe("b");
    assert_eq!(a, b, "two fresh processes split to identical bytes (D9)");
    assert!(
        a.contains("\u{1e}"),
        "the probe really wrote both documents"
    );
}

const ASM4_PROBE_OUT: &str = "ASM4_PROBE_OUT";

/// The child half of row 6: build the deterministic two-cluster
/// assembly, split its second cluster out, and write both documents'
/// save bytes.
#[test]
fn asm4_child_split_probe() {
    use pncad::document as d;
    let Ok(out) = std::env::var(ASM4_PROBE_OUT) else {
        return; // not the child — nothing to do
    };
    let dir = WsDir::new("asm4-probe");
    let part_ref = asm2a_part(&dir, "part.pncad", "asm4-probe-part");
    let (doc, ids) = asm2a_assembly("asm4-probe-asm", part_ref, 2);
    let cut = std::collections::BTreeSet::from([ids[1]]);
    let split_out = d::split(
        &doc,
        &cut,
        d::DocumentId::derive("asm4-probe-new"),
        Tol::witness(),
    )
    .expect("legal");
    let text = format!(
        "{}\u{1e}{}",
        d::save(&split_out.part, &[], Tol::witness()).expect("part saves"),
        d::save(&split_out.remainder, &[], Tol::witness()).expect("remainder saves"),
    );
    std::fs::write(&out, text).expect("probe output writable");
}

fn asm4_spawn_probe(tag: &str) -> String {
    let exe = std::env::current_exe().expect("test exe path");
    let out = std::env::temp_dir().join(format!("asm4-probe-{tag}-{}", std::process::id()));
    let probe = match module_path!().split_once("::") {
        Some((_, m)) => format!("{m}::asm4_child_split_probe"),
        None => "asm4_child_split_probe".to_string(),
    };
    let status = std::process::Command::new(exe)
        .args([probe.as_str(), "--exact", "--nocapture"])
        .env(ASM4_PROBE_OUT, &out)
        .status()
        .expect("probe spawns");
    assert!(status.success(), "probe {tag} failed");
    let bytes = std::fs::read_to_string(&out).expect("probe wrote");
    let _ = std::fs::remove_file(&out);
    bytes
}

// ---- ASM-UPD: the pin-update door through the real store ----

/// Re-authors the document id `id`'s file with a `side`-wide square
/// extruded 1.5 tall — the "the part changed on disk" move — and
/// returns the store's new current pin for it.
fn asm_upd_resave_part(
    ws: &mut pncad::workspace::Workspace,
    id: pncad::document::DocumentId,
    side: f64,
) -> pncad::document::ContentPin {
    use pncad::document::{Expr, Node};
    let doc = pncad::document::ProfileDoc::empty(id, Tol::witness());
    let (doc, plane) = doors_insert(doc, doors_xy_frame());
    let (doc, profile) = doors_insert(doc, doors_square(plane, side));
    let (doc, _) = doors_insert(
        doc,
        Node::Extrude {
            profile,
            distance: Expr::literal(1.5, pncad::document::Dimension::Length).unwrap(),
        },
    );
    ws.resave(&doc, Tol::witness()).expect("the part rewrites");
    pncad::document::content_pin(&doc, Tol::witness()).expect("the pin computes")
}

/// Row 3 — the store convenience, end to end: an assembly pinned to a
/// part, the part resaved on disk, `update_to_store` computing the new
/// pin from the store, and the applied result EVALUATING to the new
/// geometry through the real workspace.
#[test]
fn asm_upd_row3_update_to_store_picks_up_the_resaved_part() {
    use pncad::document as d;
    let dir = WsDir::new("asm-upd-e2e");
    let part_ref = asm2a_part(&dir, "part.pncad", "asm-upd-e2e-part");
    let (doc, ids) = asm2a_assembly("asm-upd-e2e-asm", part_ref, 2);
    let mut ws = pncad::workspace::Workspace::open(&dir.0).expect("the scan is clean");

    let vol = |b: &pncad::topo::Body<f64>| {
        pncad::topo::mass_properties(b, Tol::witness())
            .expect("mass properties")
            .volume
    };
    let ev = asm2a_eval(&doc, &ws);
    let before = vol(&d::product(&doc, &ev, Tol::witness()).expect("gathers"));

    // The part changes on disk. The assembly is a self-contained
    // reproducible value, so nothing about it moves yet — that is A4,
    // and it is what makes an update an EDIT.
    let new_pin = asm_upd_resave_part(&mut ws, part_ref.id, 4.0);
    assert_ne!(new_pin, part_ref.pin, "the part's content really moved");
    let stale = asm2a_eval(&doc, &ws);
    match stale.result(ids[0]) {
        Some(d::NodeResult::Failed(e)) => match &e.kind {
            d::NodeErrorKind::Part { fault, .. } => assert!(
                matches!(
                    fault,
                    d::PartFault::Unresolved {
                        fault: d::ResolveFault::PinMismatch,
                        ..
                    }
                ),
                "the un-updated assembly still names the old version: {fault}"
            ),
            other => panic!("expected a Part refusal, got {other:?}"),
        },
        other => panic!("the stale pin must refuse, got {other:?}"),
    }

    // The convenience reads the pin off the store; the caller applies.
    let edits = pncad::workspace::update_to_store(&doc, part_ref.id, &ws, Tol::witness())
        .expect("both sites elaborate against the store");
    assert_eq!(edits.len(), 2, "one edit per site, computed not supplied");
    let mut updated = doc.clone();
    for e in &edits {
        updated = d::apply(&updated, e, Tol::witness())
            .expect("the group applies")
            .doc;
    }

    let after_ev = asm2a_eval(&updated, &ws);
    let after = vol(&d::product(&updated, &after_ev, Tol::witness()).expect("gathers"));
    assert_eq!(
        after_ev.part_evaluations, 1,
        "both sites name one version again"
    );
    // The square door's fixture is `side`-wide; 2.0 → 4.0 at the same
    // 1.5 height is exactly four times the material, per instance.
    assert!(
        (after - 4.0 * before).abs() < 1e-9,
        "the new geometry is served: {before} → {after}"
    );
    assert!(
        d::mixed_pins(&updated).is_empty(),
        "a completed update leaves no multiplicity to report"
    );
}

/// Row 3b — the store's own refusals reach the convenience unchanged:
/// an id the store never scanned refuses `UnknownId` (a store miss,
/// through the existing vocabulary), and an id the store HAS but the
/// document never references refuses `Update` (an assembly question,
/// under its own arm).
#[test]
fn asm_upd_row3b_store_miss_and_unreferenced_id_refuse_apart() {
    let dir = WsDir::new("asm-upd-refuse");
    let part_ref = asm2a_part(&dir, "part.pncad", "asm-upd-refuse-part");
    let other_ref = asm2a_part(&dir, "other.pncad", "asm-upd-refuse-other");
    let (doc, _) = asm2a_assembly("asm-upd-refuse-asm", part_ref, 1);
    let ws = pncad::workspace::Workspace::open(&dir.0).expect("the scan is clean");

    let ghost = pncad::document::DocumentId::derive("asm-upd-refuse-ghost");
    match pncad::workspace::update_to_store(&doc, ghost, &ws, Tol::witness()) {
        Err(pncad::workspace::WorkspaceError::UnknownId { id }) => assert_eq!(id, ghost),
        other => panic!("a store miss must refuse UnknownId, got {other:?}"),
    }
    match pncad::workspace::update_to_store(&doc, other_ref.id, &ws, Tol::witness()) {
        Err(pncad::workspace::WorkspaceError::Update {
            error: pncad::document::UpdateError::NoSuchReference { id },
        }) => assert_eq!(id, other_ref.id),
        other => panic!("an unreferenced id must refuse Update, got {other:?}"),
    }
    // The current pin equals the reference's, so an update-all is a
    // whole-document no-op and refuses rather than reporting success.
    match pncad::workspace::update_to_store(&doc, part_ref.id, &ws, Tol::witness()) {
        Err(pncad::workspace::WorkspaceError::Update {
            error: pncad::document::UpdateError::AlreadyPinned { id, pin },
        }) => {
            assert_eq!(id, part_ref.id);
            assert_eq!(pin, part_ref.pin);
        }
        other => panic!("an already-current id must refuse AlreadyPinned, got {other:?}"),
    }
}

/// Row 6 (D9) — the UPDATED assembly's save bytes and its evaluated
/// product agree across two fresh processes: a document reached by a
/// recorded pin move is as reproducible as one authored at the new pin
/// directly.
#[test]
fn asm_upd_row6_updated_bytes_and_product_agree_across_two_fresh_processes() {
    let a = asm_upd_spawn_probe("a");
    let b = asm_upd_spawn_probe("b");
    assert_eq!(a, b, "two fresh processes agree bit for bit (D9)");
    assert!(a.contains('\u{1e}'), "the probe really wrote both halves");
}

const ASM_UPD_PROBE_OUT: &str = "ASM_UPD_PROBE_OUT";

/// The child half of row 6: build the assembly, resave the part,
/// update to the store, and write the updated document's save bytes
/// alongside its product volume bits.
#[test]
fn asm_upd_child_update_probe() {
    use pncad::document as d;
    let Ok(out) = std::env::var(ASM_UPD_PROBE_OUT) else {
        return; // not the child — nothing to do
    };
    let dir = WsDir::new("asm-upd-probe");
    let part_ref = asm2a_part(&dir, "part.pncad", "asm-upd-probe-part");
    let (doc, _) = asm2a_assembly("asm-upd-probe-asm", part_ref, 2);
    let mut ws = pncad::workspace::Workspace::open(&dir.0).expect("the scan is clean");
    asm_upd_resave_part(&mut ws, part_ref.id, 4.0);
    let edits = pncad::workspace::update_to_store(&doc, part_ref.id, &ws, Tol::witness())
        .expect("the elaboration holds");
    let mut updated = doc;
    for e in &edits {
        updated = d::apply(&updated, e, Tol::witness()).expect("applies").doc;
    }
    let ev = asm2a_eval(&updated, &ws);
    let volume = pncad::topo::mass_properties(
        &d::product(&updated, &ev, Tol::witness()).expect("gathers"),
        Tol::witness(),
    )
    .expect("mass properties")
    .volume;
    let text = format!(
        "{}\u{1e}{}",
        d::save(&updated, &[], Tol::witness()).expect("the updated document saves"),
        volume.to_bits(),
    );
    std::fs::write(&out, text).expect("probe output writable");
}

fn asm_upd_spawn_probe(tag: &str) -> String {
    let exe = std::env::current_exe().expect("test exe path");
    let out = std::env::temp_dir().join(format!("asm-upd-probe-{tag}-{}", std::process::id()));
    let probe = match module_path!().split_once("::") {
        Some((_, m)) => format!("{m}::asm_upd_child_update_probe"),
        None => "asm_upd_child_update_probe".to_string(),
    };
    let status = std::process::Command::new(exe)
        .args([probe.as_str(), "--exact", "--nocapture"])
        .env(ASM_UPD_PROBE_OUT, &out)
        .status()
        .expect("probe spawns");
    assert!(status.success(), "probe {tag} failed");
    let bytes = std::fs::read_to_string(&out).expect("probe wrote");
    let _ = std::fs::remove_file(&out);
    bytes
}

// ---------------------------------------------------------------
// The curated re-export lists, kept complete by a test.
// ---------------------------------------------------------------

/// `editor-core`'s root exports that the façade deliberately does
/// NOT carry. Names, not reasons: the reasons cluster, and the
/// clusters are what the header below states.
///
/// The families here, and why each stays interior:
///
/// - **Arena keys and the naming table's interior** (`EntityRef`,
///   `EntityKey`, `Entry`, `NamingKey`, `entity_name`, `body_name`,
///   `vertex_name`): body-lineage-scoped, meaningful only against the
///   evaluation that minted them. Not carrying them IS the LB13
///   boundary the guard above enforces.
///
///   `MeshPatchKey` and `TieWitness` are here with them, and the
///   reason is worth stating because a previous revision of this file
///   got it wrong: the seal is a **naming** barrier, not a capability
///   one. Not carrying a type stops a consumer reaching it by
///   accident — every remaining route is a contortion a reader can
///   see — but a payload bound out of a carried enum can still be
///   stored in a generic field and compared across evaluations. The
///   list below is therefore cut to what a consumer needs to ASK, not
///   to what it could be trusted not to misuse.
/// - **Appearance** (`Appearance*`, `Attr*`, `Rgba8`,
///   `*rebind_suggestions`, `enrich_appearance_loss*`): a GUI-side
///   presentation layer with no authoring door yet.
/// - **The witness/verdict/diff instrumentation** (`Branch*`,
///   `Summary*`, `Verdict*`, `Witness*`, `NodeVerdict*`, `FlipSet`,
///   `Diagnosis`, `Implicated`, `PredicateDivergence`, `SideVerdict`,
///   `DocDiff`, `NodeChange`, `diff_*`, `verdict_summary`, `Epoch`,
///   `Tombstone`, `RecipeEditRef`): the editor's own re-evaluation
///   telemetry, not a modelling vocabulary. GUI-2 carried these
///   briefly as the payloads of a resolution failure and then put them
///   back: the panel renders the failure through its `Display`, so
///   nothing consumed the payload types, and a door carried for a
///   consumer that does not exist is a claim nobody is checking.
/// - **Naming interior** (`Qualifier`, `Coset`, `Resolved`,
///   `ResolveError`, `ResolutionFailure`, `ResolveIndeterminate`,
///   `resolve_with_prior`): the shapes the name algebra and the
///   resolution ladder work in, below the verdict that is the curated
///   face.
///
///   **The resolution VERDICT left this family at GUI-2** — exactly
///   three names: `resolve`, `RunCtx`, `Resolution`. It is not
///   plumbing behind a door; it IS the door for the question a
///   consumer that stores names must ask on every re-evaluation, and
///   `Resolution`'s arms answer it (`Resolved(_)` / `Failed(f)` /
///   `Indeterminate(c)`) through pattern matching and `Display`,
///   without naming a payload type. The ladder's own vocabulary stays
///   here until something consumes it.
/// - **Evaluation interior** (`EvalScalar`, `RunStatus`,
///   `ContentKey`, `apply_with_names`, `derivation_nodes`): the
///   service's own machinery behind `evaluate`.
///
///   **`eval`, `eval_count` and `EvalError` used to be in this family
///   and were wrong to be.** They are not machinery behind
///   `evaluate` — they are the EXPRESSION read side, the only way to
///   answer "what does this slot say right now" for a slot driven by
///   a parameter or by arithmetic. `Expr::literal_value` answers only
///   for a bare literal, so without them a consumer holding the
///   curated `Expr` + `ParamEnv` pair had no door from an expression
///   to its value and would have had to re-implement the evaluator to
///   display one. `crate::document` carries all three now.
/// - **Types whose curated face is a different shape**
///   (`ProfilePayload`, `ProgramRefusal`, `ExprPath`, `ParamValue`,
///   `Product`, `product_recorded`, `BifurcationKind`, `NamingError`,
///   `MetaValue`, `MetaError`, `MetaVersionError`, `from_value`,
///   `to_value`): each has a curated door of its own or is machinery
///   behind one. (`ClassAdmission`/`class_admission` left this family
///   at GUI-4: a mate-authoring consumer needs the admission table
///   BEFORE committing, so `crate::document` carries them now.)
///
///   **The A5 gate used to be in this family and was wrong to be.**
///   `assemble` and its vocabulary (`Assembly`, `AssemblyError`,
///   `AtRestFinding`, `Attribution`, `MintedDeclaration`,
///   `RefusedRef`) had no curated door of their own and were not
///   machinery behind one: they ARE the door that answers whether an
///   assembly is valid at rest, and the façade carried the whole
///   authoring vocabulary that constructs one. A consumer could build
///   an assembly and not check it. `crate::document` carries them
///   now; `product_recorded` stays out because `product`/
///   `product_named` are the curated gather and `assemble` is what
///   needs the recorded one.
///
///   **`MintRefusal` is not part of that carry**, and the split is
///   the point: it is the GATHER's row for a mate whose declaration
///   could not be minted, reached through `Product`, which is itself
///   interior. What a façade consumer asks is what the A5 gate
///   ANSWERED, and they get that whole — `AssemblyError::Reference`
///   and `AssemblyError::NoAtRestRecord` are exactly these two
///   refusals, raised by the door that is carried.
///   **The hit-test service's NAMED half left this list at GUI-2**
///   (`NodePick`, `NodePickError`, `PickHit`, `PickTarget`,
///   `pick_face`, `HitTestError`, and `Ray` — a `bvh` re-export riding
///   the service's door). GUI-1 held the whole service out on the
///   argument that its inputs are display-side state the Python
///   authoring surface does not hold. Its first consumer landed and
///   that argument did not survive it: the service's whole public
///   ANSWER is a `StableName`, the same currency `crate::select`'s
///   other doors speak, and the alternative — the viewer taking a
///   direct `editor-core` edge — hands layer 3 the arena keys the
///   façade's curation exists to seal.
///
///   **`MeshPick` and `MeshPickError` stay, and that is what closes
///   #1098's lane at the façade.** They are the raw index a
///   hand-assembled `PickTarget` needs, and `PickTarget::pick` is a
///   `&MeshPick` — so with the index unnameable here, the target whose
///   contract warns of a confidently wrong name has no constructor a
///   façade consumer can reach, and `NodePick` is not merely the
///   preferred door but the only one. `PickTarget` is carried because
///   `pick_face`'s signature names it, not because it can be built.
/// - **The E6 driver and its parameter box** (`drive`, `DriveConfig`,
///   `DriveRefusal`, `ParamBoxVerdict`, `CertifiedLeaf`,
///   `RefusedLeaf`, `RefusalReason`, `BudgetKind`, `FlipEvidence`, `StructureFlip`,
///   `ReasonClass`, `Receipt`, `LeafResults`, `MeasureAccounting`,
///   `ReplayOutcome`, `VerdictVector`, `VerdictRow`,
///   `VerdictVectorKey`, `DEFAULT_MAX_DEPTH`, `DEFAULT_MAX_LEAVES`,
///   `ParamBox`, `BoxAxis`, `ParamBoxError`, `AxisScalar`,
///   `param_env_over`): the analysis lane's subdivision service and
///   the box it drives over.
///
///   Interior because the curated face is a DIFFERENT shape and is
///   not built yet: what a consumer asks the analysis lane is "does
///   this measurement hold over its tolerances", and E5's answer to
///   that is a typed per-measurement stackup report whose INPUT is a
///   leaf set. Carrying the leaf vocabulary now would door the
///   intermediate and then have to un-door it. `drive` is also gated
///   on the `interval` feature — there is no leaf to certify without
///   the certified scalar — so a façade row for it would be a
///   conditional door, which this surface does not have and should
///   not acquire for a type its consumer does not want yet.
const NOT_CARRIED: [&str; 101] = [
    "AppearanceLoss",
    "AppearanceLossCause",
    "AppearanceMap",
    "AppearanceRecord",
    "AppearanceResolution",
    "Attr",
    "AttrKind",
    "AttrSet",
    "AxisScalar",
    "BifurcationKind",
    "BoxAxis",
    "BranchCertification",
    "BranchMarginEvidence",
    "BudgetKind",
    "CertifiedLeaf",
    "ContentKey",
    "Coset",
    "DEFAULT_MAX_DEPTH",
    "DEFAULT_MAX_LEAVES",
    "Diagnosis",
    "DocDiff",
    "DriveConfig",
    "DriveRefusal",
    "EntityKey",
    "EntityRef",
    "Entry",
    "Epoch",
    "EvalScalar",
    "ExprPath",
    "FlipEvidence",
    "FlipSet",
    "Implicated",
    "LeafResults",
    "MeasureAccounting",
    "MeshPatchKey",
    "MeshPick",
    "MeshPickError",
    "MetaError",
    "MetaValue",
    "MetaVersionError",
    "MintRefusal",
    "NamingError",
    "NamingKey",
    "NodeChange",
    "NodeVerdictDelta",
    "NodeVerdicts",
    "ParamBox",
    "ParamBoxError",
    "ParamBoxVerdict",
    "ParamValue",
    "PredicateDivergence",
    "Product",
    "ProfilePayload",
    "ProgramRefusal",
    "Qualifier",
    "ReasonClass",
    "Receipt",
    "RecipeEditRef",
    "RefusalReason",
    "RefusedLeaf",
    "ReplayOutcome",
    "ResolutionFailure",
    "ResolveError",
    "ResolveIndeterminate",
    "Resolved",
    "Rgba8",
    "RunStatus",
    "SideVerdict",
    "StructureFlip",
    "SummaryDelta",
    "SummaryDivergence",
    "SummaryFlip",
    "SummaryFlipSet",
    "TieWitness",
    "Tombstone",
    "VerdictFlip",
    "VerdictRow",
    "VerdictSummary",
    "VerdictVector",
    "VerdictVectorKey",
    "WitnessAge",
    "WitnessBifurcation",
    "WitnessDatum",
    "appearance_rebind_suggestions",
    "apply_with_names",
    "body_name",
    "derivation_nodes",
    "diff_summaries",
    "diff_verdicts",
    "drive",
    "enrich_appearance_loss",
    "enrich_appearance_loss_with_prior",
    "entity_name",
    "from_value",
    "param_env_over",
    "product_recorded",
    "rebind_suggestions",
    "resolve_with_prior",
    "to_value",
    "verdict_summary",
    "vertex_name",
];

/// Every name a `pub use` statement of `src` introduces, restricted
/// to statements whose path ROOT is `root` — so the answer is "which
/// of that crate's names does this file carry", not "which leaf
/// identifiers appear anywhere".
///
/// The restriction is what keeps the completeness check below from
/// being satisfied by a coincidence: without it, a name re-exported
/// from `sweep` or `topo` would count as carrying an identically
/// spelled document-layer name, and the guard would pass while the
/// name was uncarried.
///
/// Comments are stripped first, so prose naming a type is not read as
/// an export. A statement with no `::` (a whole-crate `pub use foo;`)
/// introduces the crate name itself and belongs to no root.
///
/// A leading `::` is stripped before the root is read: the façade
/// spells one of its layers with the absolute prefix, because that
/// file's own module shadows the crate name.
fn pub_use_names(src: &str, root: &str) -> std::collections::BTreeSet<String> {
    let code = code_without_comments(src);
    let prefix = format!("{root}::");
    let mut names = std::collections::BTreeSet::new();
    let mut rest: &str = &code;
    while let Some(at) = rest.find("pub use ") {
        rest = &rest[at + "pub use ".len()..];
        let Some(end) = rest.find(';') else { break };
        let stmt = rest[..end].trim_start();
        let stmt = stmt.strip_prefix("::").unwrap_or(stmt);
        rest = &rest[end + 1..];
        if !stmt.starts_with(&prefix) {
            continue;
        }
        let items = match (stmt.find('{'), stmt.rfind('}')) {
            (Some(open), Some(close)) if open < close => stmt[open + 1..close].to_string(),
            // A single path: the leaf is the name it introduces.
            _ => stmt.rsplit("::").next().unwrap_or(stmt).to_string(),
        };
        for item in items.split(',') {
            let item = item.trim();
            if !item.is_empty() {
                names.insert(item.rsplit("::").next().unwrap_or(item).to_string());
            }
        }
    }
    names
}

/// Every name a `pub use` statement introduces, with no root
/// restriction — the form for reading a crate's OWN `lib.rs`, where
/// each statement's root is one of that crate's modules.
fn module_pub_use_names(src: &str) -> std::collections::BTreeSet<String> {
    let code = code_without_comments(src);
    let mut names = std::collections::BTreeSet::new();
    let mut rest: &str = &code;
    while let Some(at) = rest.find("pub use ") {
        rest = &rest[at + "pub use ".len()..];
        let Some(end) = rest.find(';') else { break };
        let stmt = &rest[..end];
        rest = &rest[end + 1..];
        let items = match (stmt.find('{'), stmt.rfind('}')) {
            (Some(open), Some(close)) if open < close => stmt[open + 1..close].to_string(),
            _ => stmt.rsplit("::").next().unwrap_or(stmt).to_string(),
        };
        for item in items.split(',') {
            let item = item.trim();
            if !item.is_empty() {
                names.insert(item.rsplit("::").next().unwrap_or(item).to_string());
            }
        }
    }
    names
}

/// **The curated lists are complete, and the incompleteness is a
/// test rather than a habit.**
///
/// The façade exposes the document layer through hand-written `pub
/// use` lists (`crate::document`, `crate::select`, `crate::prelude`)
/// rather than a whole-crate re-export, because a whole-crate
/// re-export would hand out arena keys. That choice buys the LB13
/// boundary and costs a standing sync obligation: when the document
/// layer grows a public name, nothing makes anyone carry it.
///
/// This is the mechanism. Every name the document layer exports at
/// its root is either carried by one of the façade's `pub use` lists
/// or listed in `NOT_CARRIED` above — and a name that is neither
/// fails here, at the moment it lands, naming itself.
///
/// What it does NOT claim: that each `NOT_CARRIED` entry is
/// individually argued (they are argued by family, in that constant's
/// docs), and that the same completeness holds for the other kernel
/// crates. It does not need to for them — they are re-exported whole,
/// so their surfaces cannot drift from the façade's by construction.
///
/// Two blind spots, both needing rustdoc JSON to close (**#696**):
///
/// 1. A public name reachable only by module path
///    (`editor_core::persist::Foo`) and never lifted to that crate's
///    root. This scan reads the root, exactly as the first closure
///    audit did, and that is the same structural hole that audit's
///    second pass found.
/// 2. A `pub` item written DIRECTLY in `editor-core/src/lib.rs`
///    rather than re-exported. That file has no direct `pub` items
///    today, so this one is held shut by a coincidence, not a rule.
///
/// A third — a leaf name colliding across crates, so that carrying
/// `Foo` from `sweep` looked like carrying the document layer's
/// `Foo` — is CLOSED: the façade side counts only names introduced
/// by a `pub use editor_core::…` statement, not every leaf in the
/// file.
#[test]
fn every_document_layer_root_export_is_carried_or_listed() {
    let kernel_lib =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../editor-core/src/lib.rs");
    let src = std::fs::read_to_string(&kernel_lib)
        .unwrap_or_else(|e| panic!("reading {}: {e}", kernel_lib.display()));
    let exported = module_pub_use_names(&src);
    assert_layer_root_exports_are_carried_or_listed(
        "editor_core",
        "the document layer",
        &exported,
        150,
        &NOT_CARRIED,
        "Carry each through `crate::document` or `crate::select`, or add it \
         to NOT_CARRIED with the family it belongs to.",
        "NOT_CARRIED",
    );
}

/// The completeness check itself, shared by every layer guarded this
/// way. It is one function rather than one copy per layer because
/// hand-synced copies are how the sync obligation these guards exist
/// to enforce fails in the first place.
///
/// `exported` is the layer's root surface, gathered by the caller —
/// the two layers do not gather it identically, and that difference is
/// the reason the gathering stays outside this function. Everything
/// after it is common: the vacuity floor, the façade side restricted
/// to `pub use <layer>::…` statements, the uncarried report, and the
/// staleness report that keeps the exclusion list from outliving the
/// decisions it records.
fn assert_layer_root_exports_are_carried_or_listed(
    layer: &str,
    layer_prose: &str,
    exported: &std::collections::BTreeSet<String>,
    min_exports: usize,
    not_carried: &[&str],
    remedy: &str,
    list_name: &str,
) {
    assert!(
        exported.len() > min_exports,
        "the scanner found only {} root exports for {layer_prose} — the \
         file's shape changed and this guard was about to pass vacuously",
        exported.len()
    );

    // The façade side: only what it carries FROM this layer. A
    // `prelude` entry that re-exports through one of the façade's own
    // curated modules loses nothing by the restriction — its origin is
    // a statement in this same scan.
    let mut carried = std::collections::BTreeSet::new();
    for (_, facade_src) in FACADE_SOURCES {
        carried.append(&mut pub_use_names(facade_src, layer));
    }

    let uncarried: Vec<&str> = exported
        .iter()
        .map(String::as_str)
        .filter(|n| !carried.contains(*n) && !not_carried.contains(n))
        .collect();
    assert!(
        uncarried.is_empty(),
        "{layer_prose} exports {} name(s) the façade neither carries nor \
         lists as deliberately interior:\n  {}\n{remedy}",
        uncarried.len(),
        uncarried.join("\n  ")
    );

    // The list decays in the other direction too: an entry that is no
    // longer exported, or that the façade has since started carrying,
    // is a stale exclusion claiming a decision nobody is making.
    let stale: Vec<&str> = not_carried
        .iter()
        .copied()
        .filter(|n| !exported.contains(*n) || carried.contains(*n))
        .collect();
    assert!(
        stale.is_empty(),
        "{list_name} lists {} name(s) that are no longer uncarried root \
         exports — remove them:\n  {}",
        stale.len(),
        stale.join("\n  ")
    );
}

// ---------------------------------------------------------------
// The same completeness for the OTHER curated layer, and for the
// claim that there are only two.
// ---------------------------------------------------------------

/// Every `pub` item a crate root DECLARES rather than re-exports:
/// `pub struct`/`enum`/`fn`/`trait`/`type`/`const`/`static`/`union`,
/// plus the `pub mod` declarations, written at column 0.
///
/// Column 0 is the whole scope rule — an item inside a `mod` block in
/// the same file is indented, and is not a root export.
///
/// [`module_pub_use_names`] alone misses all of these. For the
/// document layer that costs nothing today, which is why its guard
/// records it as a blind spot rather than closing it: that root is a
/// module tree, its declarations are the crate's twenty-six interior
/// modules, and the façade curates ACROSS them rather than carrying
/// them. The profile layer's root is the opposite shape — a presented
/// surface that declares five of the types the façade carries and one
/// it deliberately does not — so for that layer the same omission
/// would be a hole, and this closes it.
fn root_declared_pub_names(src: &str) -> std::collections::BTreeSet<String> {
    let code = code_without_comments(src);
    let mut names = std::collections::BTreeSet::new();
    for line in code.lines() {
        let Some(rest) = line.strip_prefix("pub ") else {
            continue;
        };
        let Some((keyword, tail)) = rest.split_once(' ') else {
            continue;
        };
        if !matches!(
            keyword,
            "mod" | "struct" | "enum" | "fn" | "trait" | "type" | "const" | "static" | "union"
        ) {
            continue;
        }
        let name: String = tail
            .trim_start()
            .chars()
            .take_while(|c| c.is_alphanumeric() || *c == '_')
            .collect();
        if !name.is_empty() {
            names.insert(name);
        }
    }
    names
}

/// The profile layer's interior: root exports the façade's curated
/// `profile` module does not carry, by family. One family, one name.
///
/// - **The minting tier** (`RawLoop`): the one name whose absence is
///   the module's entire reason for existing. It carries `new` and
///   `polygon`; leaving the trait unnameable is what makes
///   `ProfileLoop::polygon(…)` fail to resolve while `ProfileLoop`
///   itself stays nameable. Carrying it here would undo the curation.
const PROFILE_NOT_CARRIED: [&str; 1] = ["RawLoop"];

/// **The document layer's guard, for the other layer curated the same
/// way.**
///
/// The façade re-exports ten of its twelve kernel layers whole, so
/// their surfaces cannot drift from it by construction. Two are
/// curated by hand instead, and each hand-written list carries the
/// standing sync obligation the guard above describes: when the layer
/// grows a public name, nothing makes anyone carry it. Only one of the
/// two was watched, and the asymmetry was not a decision — the
/// unwatched layer's `PathNoCornerReason` was missing from its carrier
/// statement for eighteen days with nothing to say so.
///
/// This layer's root differs from the document layer's in one way that
/// matters to the scan: it DECLARES types, so the export set is its
/// `pub use` names plus its root declarations
/// ([`root_declared_pub_names`]), and the guard's second blind spot —
/// a `pub` item written directly in the root — is closed here rather
/// than held shut by a coincidence.
///
/// The other two blind spots stand, unchanged and shared:
///
/// 1. A public name reachable only by module path and never lifted to
///    the crate root. This scan reads the root.
/// 2. A name the façade carries only under a SUBMODULE of this layer
///    counts as carrying an identically spelled root name — the
///    façade's arrival-spec statements name a submodule path, and four
///    of the document layer's do too. Both would need the leaf's
///    origin, not its spelling, to separate.
#[test]
fn every_profile_layer_root_export_is_carried_or_listed() {
    let layer_lib = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../profile/src/lib.rs");
    let src = std::fs::read_to_string(&layer_lib)
        .unwrap_or_else(|e| panic!("reading {}: {e}", layer_lib.display()));
    let mut exported = module_pub_use_names(&src);
    exported.append(&mut root_declared_pub_names(&src));

    assert_layer_root_exports_are_carried_or_listed(
        "profile",
        "the profile layer",
        &exported,
        40,
        &PROFILE_NOT_CARRIED,
        "Carry each through `crate::profile`, or add it to \
         PROFILE_NOT_CARRIED with the family it belongs to.",
        "PROFILE_NOT_CARRIED",
    );
}

/// The layers whose surfaces the façade curates name by name, and
/// which therefore have a completeness guard above. Anything not here
/// must be re-exported whole; the test below is what makes that an
/// enforced dichotomy rather than a description.
const PER_NAME_GUARDED: [&str; 2] = ["editor_core", "profile"];

/// Every crate the façade re-exports WHOLE at its root — `pub use
/// foo;`, no path and no brace list — so every name that crate's root
/// exports is nameable one hop past the façade by construction.
fn whole_crate_re_exports(src: &str) -> std::collections::BTreeSet<String> {
    let code = code_without_comments(src);
    let mut names = std::collections::BTreeSet::new();
    let mut rest: &str = &code;
    while let Some(at) = rest.find("pub use ") {
        rest = &rest[at + "pub use ".len()..];
        let Some(end) = rest.find(';') else { break };
        let stmt = rest[..end].trim();
        rest = &rest[end + 1..];
        if !stmt.is_empty() && stmt.chars().all(|c| c.is_alphanumeric() || c == '_') {
            names.insert(stmt.to_string());
        }
    }
    names
}

/// Every path dependency of the façade's manifest, as a crate
/// identifier. A path dependency is a workspace layer; a registry or
/// workspace-inherited one (the OS entropy crate) is not a surface
/// this crate presents, and the `path` key is what tells them apart.
fn facade_layer_dependencies(manifest: &str) -> std::collections::BTreeSet<String> {
    let mut names = std::collections::BTreeSet::new();
    let mut in_dependencies = false;
    for line in manifest.lines() {
        let line = line.trim();
        if line.starts_with('[') {
            in_dependencies = line == "[dependencies]";
            continue;
        }
        if !in_dependencies || line.starts_with('#') {
            continue;
        }
        let Some((name, value)) = line.split_once('=') else {
            continue;
        };
        if !value.contains("path = \"../") {
            continue;
        }
        names.insert(name.trim().replace('-', "_"));
    }
    names
}

/// **The scope of the guards above is measured, not asserted.**
///
/// Each completeness guard covers one layer, and the reason the
/// others need none is that they are re-exported whole. That reason
/// was a sentence in a doc comment: nothing checked that the whole
/// re-export was still there, and nothing noticed that a second layer
/// had already left the whole-re-export set and gained no guard.
///
/// So this reads the manifest for the layers the façade depends on and
/// puts each in exactly one of two buckets — whole-re-exported at the
/// façade root, or per-name guarded here. A layer in neither is the
/// unwatched case, and a layer in both is a guard maintained over a
/// surface that cannot drift. Adding a dependency, or narrowing a
/// whole re-export into a curated module the way the profile layer's
/// was narrowed, lands in this test on the same commit.
///
/// It does NOT claim the curated lists are the right ones — that is
/// the guards' job for two layers and nobody's for the other ten,
/// which is exactly right: for those ten there is no list to be wrong.
#[test]
fn every_facade_layer_is_whole_re_exported_or_per_name_guarded() {
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let manifest_path = manifest_dir.join("Cargo.toml");
    let manifest = std::fs::read_to_string(&manifest_path)
        .unwrap_or_else(|e| panic!("reading {}: {e}", manifest_path.display()));
    let layers = facade_layer_dependencies(&manifest);
    assert!(
        layers.len() > 8,
        "the manifest scanner found only {} path dependencies — the \
         manifest's shape changed and this guard was about to pass \
         vacuously",
        layers.len()
    );

    let (_, root_src) = FACADE_SOURCES
        .iter()
        .find(|(name, _)| *name == "lib.rs")
        .unwrap_or_else(|| panic!("FACADE_SOURCES no longer lists the façade root"));
    let whole = whole_crate_re_exports(root_src);

    let unclassified: Vec<&str> = layers
        .iter()
        .map(String::as_str)
        .filter(|layer| whole.contains(*layer) == PER_NAME_GUARDED.contains(layer))
        .collect();
    assert!(
        unclassified.is_empty(),
        "{} façade layer(s) are not in exactly one of the two buckets:\n  {}\n\
         A layer is either re-exported whole at the façade root — in which \
         case its surface cannot drift from the façade's — or curated name \
         by name, in which case it needs a completeness guard in this file \
         and an entry in PER_NAME_GUARDED.",
        unclassified.len(),
        unclassified.join("\n  ")
    );

    // The buckets are about THIS crate's layers, so neither may name
    // something the manifest does not.
    let unknown: Vec<&str> = whole
        .iter()
        .map(String::as_str)
        .chain(PER_NAME_GUARDED)
        .filter(|name| !layers.contains(*name))
        .collect();
    assert!(
        unknown.is_empty(),
        "{} name(s) are bucketed as façade layers but are not path \
         dependencies of this crate:\n  {}",
        unknown.len(),
        unknown.join("\n  ")
    );
}

/// **The crate doc's claim about the authoring seams, guarded.**
///
/// The claim is that every [`pncad::authoring`] seam is a single
/// kernel constructor call except `validated`, which is the two-call
/// form. Its predecessor was a COUNT ("six of the seven"), and the
/// count went stale the moment the `polygon` door was removed —
/// nothing was watching, so the sentence outlived the surface it
/// described by two units.
///
/// This watches the failure mode that actually happened: the roster
/// moving under a sentence about it. Adding or removing a seam fails
/// here, and so does a second seam chaining a follow-up kernel call
/// onto its constructor.
///
/// **Not guarded, stated:** "a single kernel constructor call" is
/// about a body's SHAPE, and counting calls in source text is the
/// kind of scan that reports its own parser rather than the code. The
/// roster plus the chain check is what a text scan can honestly
/// assert; the rest is the per-function rustdoc and its doctests.
#[test]
fn the_authoring_seam_roster_is_what_the_crate_doc_claims() {
    let code = code_without_comments(include_str!("../src/authoring.rs"));
    let mut seams: Vec<&str> = Vec::new();
    let mut chaining: Vec<&str> = Vec::new();
    let mut current: Option<&str> = None;
    for line in code.lines() {
        if let Some(rest) = line.strip_prefix("pub fn ") {
            let name = rest
                .split(|c: char| !(c.is_alphanumeric() || c == '_'))
                .next()
                .unwrap_or("");
            seams.push(name);
            current = Some(name);
            continue;
        }
        // A body line: column 0 `}` closes the function.
        if line.starts_with('}') {
            current = None;
        } else if let Some(name) = current
            && line.contains(").validate(")
        {
            chaining.push(name);
        }
    }
    seams.sort_unstable();
    assert_eq!(
        seams,
        ["p2", "p3", "real", "v2", "v3", "validated"],
        "the authoring seam roster moved — re-read the crate doc's \
         sentence about it before changing this list"
    );
    assert_eq!(
        chaining,
        ["validated"],
        "`validated` is documented as the ONE two-call seam; another \
         seam now chains a follow-up kernel call, so the crate doc's \
         claim needs re-wording"
    );
}

// ---------------------------------------------------------------
// The north-star audit's roster, guarded.
// ---------------------------------------------------------------

/// Every `demos/tour/src/*.rs` file, by file name and comment-stripped
/// text, read from disk rather than `include_str!`-ed one by one.
///
/// Read from disk ON PURPOSE: `demos/tour` is a workspace-EXCLUDED
/// root, so this crate cannot depend on it and cannot see its types —
/// the tour's roster is only ever available here as source TEXT. A
/// fixed list of `include_str!`s would also be a second hand-kept
/// roster, which is exactly the drift the guard below exists to
/// catch: a new scene module has to be picked up by the scan itself,
/// with no edit here.
///
/// `main.rs` is excluded: it DEFINES `struct Stop` and builds none.
/// Files behind a cargo feature (`probe`, `tessbudget`) are read like
/// any other — a text scan cannot see `cfg`, and reading them is the
/// safe direction, since a stop the tour builds only sometimes is
/// still a stop the audit owes a row.
fn tour_sources() -> Vec<(String, String)> {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../demos/tour/src");
    let entries =
        std::fs::read_dir(&dir).unwrap_or_else(|e| panic!("reading {}: {e}", dir.display()));
    let mut out: Vec<(String, String)> = Vec::new();
    for entry in entries {
        let path = entry.expect("a readable directory entry").path();
        if path.extension().and_then(|e| e.to_str()) != Some("rs") {
            continue;
        }
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .expect("a UTF-8 file name")
            .to_string();
        if name == "main.rs" {
            continue;
        }
        let text = std::fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("reading {}: {e}", path.display()));
        out.push((name, code_without_comments(&text)));
    }
    out.sort();
    assert!(
        !out.is_empty(),
        "no tour scene sources at {}",
        dir.display()
    );
    out
}

/// The text of a struct literal's FIRST field: everything from `from`
/// (the index just past the literal's opening brace) to the first
/// comma at nesting depth zero, with runs of whitespace collapsed.
///
/// Depth tracking is what lets a `match` arm list or a nested call sit
/// inside the field without ending it, and string tracking is what
/// keeps a comma inside a caption from ending it.
fn first_struct_field(code: &str, from: usize) -> String {
    let b = code.as_bytes();
    let (mut i, mut depth, mut in_str) = (from, 0usize, false);
    let mut out = String::new();
    while i < b.len() {
        let c = b[i] as char;
        if in_str {
            if c == '\\' {
                out.push(c);
                i += 1;
                if i < b.len() {
                    out.push(b[i] as char);
                    i += 1;
                }
                continue;
            }
            if c == '"' {
                in_str = false;
            }
            out.push(c);
            i += 1;
            continue;
        }
        match c {
            '"' => in_str = true,
            '(' | '[' | '{' => depth += 1,
            ')' | ']' | '}' => {
                if depth == 0 {
                    break;
                }
                depth -= 1;
            }
            ',' if depth == 0 => break,
            _ => {}
        }
        out.push(c);
        i += 1;
    }
    out.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Every string literal in `s`, in order.
fn string_literals(s: &str) -> Vec<String> {
    let b = s.as_bytes();
    let (mut i, mut out) = (0usize, Vec::new());
    while i < b.len() {
        if b[i] == b'"' {
            let mut lit = String::new();
            i += 1;
            while i < b.len() && b[i] != b'"' {
                if b[i] == b'\\' {
                    i += 1;
                }
                if i < b.len() {
                    lit.push(b[i] as char);
                    i += 1;
                }
            }
            i += 1;
            out.push(lit);
        } else {
            i += 1;
        }
    }
    out
}

/// True when the byte at `at` starts a whole identifier token (the
/// character before it cannot continue an identifier).
fn token_starts_at(code: &str, at: usize) -> bool {
    !code[..at]
        .chars()
        .next_back()
        .is_some_and(|c| c.is_alphanumeric() || c == '_')
}

/// Every first-position string-literal argument of a call to `ident`
/// in `code` — the resolver for a stop name that arrives as a
/// `&'static str` PARAMETER rather than as a literal at the struct.
/// Definitions (`fn ident(`) are skipped; only calls are read.
fn first_arg_literals(code: &str, ident: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut from = 0usize;
    while let Some(off) = code[from..].find(ident) {
        let at = from + off;
        from = at + ident.len();
        if !token_starts_at(code, at) {
            continue;
        }
        let rest = &code[at + ident.len()..];
        let paren = rest.len() - rest.trim_start().len();
        if rest.as_bytes().get(paren) != Some(&b'(') {
            continue;
        }
        let before = code[..at].trim_end();
        if before.ends_with("fn") || before.ends_with("let") {
            continue;
        }
        let arg = first_struct_field(code, at + ident.len() + paren + 1);
        let lits = string_literals(&arg);
        if lits.len() == 1 && arg == format!("\"{}\"", lits[0]) {
            out.push(lits.into_iter().next().expect("one literal"));
        }
    }
    out
}

/// **The tour's stop roster, read out of its own source text.**
///
/// Returns the names, the number of `Stop { … }` literal sites the
/// scan found, and one complaint per site it could not resolve. A
/// complaint is a FAILURE, never a silent omission: the whole point
/// is that the guard below cannot under-report the roster.
///
/// The scan walks every `Stop {` struct literal (a `-> Stop {`
/// function signature is not one) and reads its FIRST field, which is
/// `name` in every case because that is the field's position in
/// `main.rs`'s definition. Three forms are understood, and they are
/// the three the tour actually writes:
///
/// 1. `name: "literal"` — the common case;
/// 2. `name: match … { … "a", … "b" }` — every literal in the arms
///    (`letterforms`' shadow trio);
/// 3. `name,` — the field-init shorthand, where the name is either a
///    `let name: &'static str = match …` a few lines up (`heatsink`)
///    or a `&'static str` PARAMETER of the enclosing `fn` or closure,
///    in which case the names are the first-position literals at that
///    helper's call sites (`bodies`' `stop`, `skinned`'s `shadow`).
///
/// **What this scan can and cannot see, stated rather than assumed.**
/// It can see any stop whose name reaches `Stop.name` as a literal by
/// one of those three routes, which is every stop the tour has. It
/// CANNOT see a name computed at run time (a `format!`, a name read
/// from a file, a literal reached through a second helper hop) — and
/// it does not pretend to: such a site produces a complaint naming
/// the file and the field text, so the failure mode is a red build
/// asking for the scan to be taught, never a quietly short roster.
/// It also cannot see `cfg`, so it reads feature-gated modules too
/// (the safe direction — see [`tour_sources`]).
fn tour_stop_roster() -> (std::collections::BTreeSet<String>, usize, Vec<String>) {
    const DECL: &str = "name: &'static str";
    let mut names = std::collections::BTreeSet::new();
    let mut sites = 0usize;
    let mut complaints: Vec<String> = Vec::new();

    for (file, code) in tour_sources() {
        let mut from = 0usize;
        while let Some(off) = code[from..].find("Stop") {
            let at = from + off;
            from = at + "Stop".len();
            if !token_starts_at(&code, at) {
                continue;
            }
            let rest = &code[at + "Stop".len()..];
            let gap = rest.len() - rest.trim_start().len();
            if rest.as_bytes().get(gap) != Some(&b'{') {
                continue;
            }
            // `-> Stop {` opens a function body, not a literal.
            if code[..at].trim_end().ends_with("->") {
                continue;
            }
            sites += 1;
            let field = first_struct_field(&code, at + "Stop".len() + gap + 1);
            let found: Vec<String> = if field == "name" {
                // The shorthand: resolve the binding that dominates.
                let Some(decl) = code[..at].rfind(DECL) else {
                    complaints.push(format!(
                        "{file}: a `name,` shorthand with no `{DECL}` before it"
                    ));
                    continue;
                };
                let after = code[decl + DECL.len()..].trim_start();
                if let Some(init) = after.strip_prefix('=') {
                    let end = init.find(';').unwrap_or(init.len());
                    string_literals(&init[..end])
                } else {
                    // A parameter: find the `(` or `|` that opens the
                    // list, then the `fn`/`let` name in front of it.
                    let head = &code[..decl];
                    let Some(open) = head.rfind(['(', '|']) else {
                        complaints.push(format!(
                            "{file}: `{DECL}` with no parameter-list opener before it"
                        ));
                        continue;
                    };
                    let owner: String = head[..open]
                        .trim_end()
                        .trim_end_matches('=')
                        .trim_end()
                        .chars()
                        .rev()
                        .take_while(|c| c.is_alphanumeric() || *c == '_')
                        .collect::<Vec<_>>()
                        .into_iter()
                        .rev()
                        .collect();
                    if owner.is_empty() {
                        complaints.push(format!(
                            "{file}: `{DECL}` in a parameter list with no named owner"
                        ));
                        continue;
                    }
                    first_arg_literals(&code, &owner)
                }
            } else if let Some(expr) = field.strip_prefix("name:") {
                let expr = expr.trim();
                let lits = string_literals(expr);
                // Form 1 (one literal, and the expression IS that
                // literal) or form 2 (a `match` whose arms are
                // literals) — the same answer either way, the two
                // shapes kept apart so a third form falls through.
                let one_literal = lits.len() == 1 && expr == format!("\"{}\"", lits[0]);
                if one_literal || (expr.starts_with("match") && !lits.is_empty()) {
                    lits
                } else {
                    complaints.push(format!(
                        "{file}: a `Stop` whose name is neither a literal nor a \
                         match over literals: `{expr}`"
                    ));
                    continue;
                }
            } else {
                complaints.push(format!(
                    "{file}: a `Stop` literal whose FIRST field is not `name`: `{field}`"
                ));
                continue;
            };
            if found.is_empty() {
                complaints.push(format!("{file}: a `Stop` site resolved to no name at all"));
            }
            names.extend(found);
        }
    }
    (names, sites, complaints)
}

/// The page's own text, so the guards below read exactly what
/// `guide::north_star_audit` renders.
const AUDIT_PAGE: &str = include_str!("../../../docs/guide/north-star-audit.md");

/// The body of one `## `-level section of a Markdown page: everything
/// after the heading line, up to the next `## ` heading or the end.
///
/// Scoping every table read to its own section is what keeps a row of
/// one table from being read as a row of another — the audit's row
/// numbers and the gap list's ids live in different sections and are
/// parsed by different rules.
fn markdown_section<'a>(page: &'a str, heading: &str) -> &'a str {
    let at = page
        .find(heading)
        .unwrap_or_else(|| panic!("the audit page has no `{heading}` heading"));
    let rest = &page[at + heading.len()..];
    match rest.find("\n## ") {
        Some(end) => &rest[..end],
        None => rest,
    }
}

/// One Markdown table row's cells, or `None` for a line that is not a
/// row (or is the `|---|` separator).
fn table_cells(line: &str) -> Option<Vec<&str>> {
    let t = line.trim();
    if !t.starts_with('|') {
        return None;
    }
    let cells: Vec<&str> = t.trim_matches('|').split('|').map(str::trim).collect();
    if cells
        .iter()
        .all(|c| !c.is_empty() && c.chars().all(|ch| ch == '-' || ch == ':'))
    {
        return None;
    }
    Some(cells)
}

/// The first backtick-quoted run in a cell — a row's scene name, kept
/// apart from the annotations the cell also carries (`(glued)`,
/// `(15 bodies)`).
fn first_backticked(cell: &str) -> Option<&str> {
    let start = cell.find('`')? + 1;
    let len = cell[start..].find('`')?;
    Some(&cell[start..start + len])
}

/// One audit row: its number, its scene, its verdict text and its gap
/// cell.
struct AuditRow {
    number: usize,
    scene: String,
    verdict: String,
    gap: String,
}

/// Every row of the audit table — the rows of the `## The audit`
/// section whose first cell is a number, which is what tells a scene
/// row apart from that section's prose and its header.
fn audit_rows() -> Vec<AuditRow> {
    let mut out = Vec::new();
    for line in markdown_section(AUDIT_PAGE, "\n## The audit\n").lines() {
        let Some(cells) = table_cells(line) else {
            continue;
        };
        let Ok(number) = cells[0].parse::<usize>() else {
            continue;
        };
        assert!(
            cells.len() >= 4,
            "audit row {number} has {} cells, not the table's five",
            cells.len()
        );
        let scene = first_backticked(cells[1])
            .unwrap_or_else(|| panic!("audit row {number} names no scene in backticks"));
        out.push(AuditRow {
            number,
            scene: scene.to_string(),
            verdict: cells[2].to_string(),
            gap: cells[3].to_string(),
        });
    }
    out
}

/// **The audit page has a row for every tour stop, and every row is a
/// tour stop.**
///
/// `docs/guide/north-star-audit.md` measures the tour against the
/// ratified goal — every demo authorable through the Python bindings
/// — scene by scene. Its test (`crates/pncad-py/tests/test_north_star.py`)
/// checks that its YES rows still build and that its NO rows' gaps
/// are still absent, so a door LANDING fails loudly. Nothing checked
/// the other axis: a tour stop ARRIVING was a silent non-row, and the
/// page sat at 34 rows against a tour of 47 until this guard was
/// written.
///
/// So this is that axis, in both directions:
///
/// - **growth** — a stop with no row fails here, naming itself;
/// - **decay** — a row naming a scene the tour no longer builds fails
///   too, because a stale row is a measurement of nothing.
///
/// The roster comes from the tour's own source text
/// ([`tour_stop_roster`], whose docs state exactly what that scan can
/// and cannot see), because `demos/tour` is a workspace-excluded root
/// this crate cannot depend on. The scan REFUSES rather than
/// under-reports: a `Stop` whose name it cannot resolve is a
/// complaint here, not a missing row.
///
/// **Not guarded, stated:** that each row's verdict is CORRECT. That
/// is what the Python suite executes for the YES rows and pins as
/// absences for the NO rows; a text scan can only insist that every
/// scene is graded.
#[test]
fn the_north_star_audit_has_a_row_for_every_tour_stop() {
    let (roster, sites, complaints) = tour_stop_roster();
    assert!(
        complaints.is_empty(),
        "the tour builds {} `Stop` value(s) this scan cannot resolve to a name — \
         teach it the new form rather than letting the roster run short:\n  {}",
        complaints.len(),
        complaints.join("\n  ")
    );
    // Vacuity floors: a scan that found almost nothing would pass
    // every set comparison below while measuring nothing at all.
    assert!(
        sites >= 30,
        "the scan found only {sites} `Stop` literal site(s) in the tour — its \
         source shape changed and this guard was about to pass vacuously"
    );
    assert!(
        roster.len() >= 40,
        "the scan resolved only {} stop name(s) — same alarm",
        roster.len()
    );

    let rows = audit_rows();
    assert!(
        rows.len() >= 40,
        "the audit table parsed to only {} row(s) — its shape changed and this \
         guard was about to pass vacuously",
        rows.len()
    );
    let numbering: Vec<usize> = rows.iter().map(|r| r.number).collect();
    assert_eq!(
        numbering,
        (1..=rows.len()).collect::<Vec<_>>(),
        "the audit table's row numbers are not 1..{} in order — the count in \
         the page's headline is read off them",
        rows.len()
    );

    let listed: std::collections::BTreeSet<String> = rows.iter().map(|r| r.scene.clone()).collect();
    assert_eq!(
        listed.len(),
        rows.len(),
        "the audit table names a scene twice"
    );

    let unrowed: Vec<&String> = roster.difference(&listed).collect();
    assert!(
        unrowed.is_empty(),
        "the tour builds {} stop(s) the north-star audit has no row for:\n  {}\n\
         Grade each against the bound surface and add its row (and re-derive \
         every count on the page off the table you end up with).",
        unrowed.len(),
        unrowed
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<_>>()
            .join("\n  ")
    );
    let retired: Vec<&String> = listed.difference(&roster).collect();
    assert!(
        retired.is_empty(),
        "the north-star audit has {} row(s) for scene(s) the tour no longer \
         builds:\n  {}\n\
         Remove each and re-derive the counts.",
        retired.len(),
        retired
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<_>>()
            .join("\n  ")
    );
}

/// **The audit page's tallies are counted off its own rows.**
///
/// The page says so in terms — *"the rows are the record and the
/// tallies are derived — every number above is counted off the table
/// just now, never carried forward"* — and it says so because the
/// discipline has failed twice in its history (a headline that read
/// 26 = 23 + 3 against a table saying 25 + 3; two gap counts off by
/// one). Prose cannot hold a promise like that; this does.
///
/// Two tallies are checked, both purely mechanical:
///
/// 1. the **headline paragraph** — every number it writes, in order:
///    authorable of total, then the outright/degraded split, then the
///    blocked count — against the rows' own YES/YES\*/NO verdicts.
///    The whole paragraph rather than its first clause, because the
///    drift that happened was BETWEEN the headline's total and its
///    own split;
/// 2. each gap's **stops** column against the number of rows naming
///    that gap as their primary blocker.
///
/// **Not guarded, stated:** the prose arithmetic sentence under the
/// gap list, which re-says the partition row by row, and the per-row
/// narrative in the last column. Those are re-derived by hand at each
/// revision; what this guard buys is that the numbers they are
/// derived FROM cannot drift unnoticed.
#[test]
fn the_north_star_audits_tallies_are_derived_from_its_rows() {
    let rows = audit_rows();
    let (mut yes, mut yes_star, mut no) = (0usize, 0usize, 0usize);
    for row in &rows {
        if row.verdict.contains("NO") {
            no += 1;
        } else if row.verdict.contains("YES") {
            // The page writes an outright YES in bold (`**YES**`) and
            // the degraded mark as `YES` plus an ESCAPED asterisk, so
            // the backslash is what tells them apart — the bold
            // markers are asterisks too.
            if row.verdict.contains('\\') {
                yes_star += 1;
            } else {
                yes += 1;
            }
        } else {
            panic!(
                "audit row {} has an unreadable verdict: `{}`",
                row.number, row.verdict
            );
        }
    }
    assert_eq!(
        yes + yes_star + no,
        rows.len(),
        "every row is YES, YES* or NO"
    );

    // The headline PARAGRAPH — every number in it, in the order it
    // writes them: authorable of total, then the outright/degraded
    // split, then the blocked count. Reading the whole paragraph
    // rather than one clause is deliberate: the drift that happened
    // was between the headline's total and its own split, which a
    // guard reading only the first clause would have missed.
    let head = "**Result: ";
    let at = AUDIT_PAGE
        .find(head)
        .expect("the audit page opens with its Result headline");
    let tail = &AUDIT_PAGE[at + head.len()..];
    let end = tail.find("\n\n").expect("the headline is a paragraph");
    let sentence: String = tail[..end].split_whitespace().collect::<Vec<_>>().join(" ");
    let numbers: Vec<usize> = sentence
        .split(|c: char| !c.is_ascii_digit())
        .filter(|w| !w.is_empty())
        .map(|w| w.parse::<usize>().expect("a decimal count"))
        .collect();
    assert_eq!(
        numbers,
        vec![yes + yes_star, rows.len(), yes, yes_star, no],
        "the headline reads `{sentence}`, but the table says {} of {} \
         (YES {yes} + YES* {yes_star}, NO {no})",
        yes + yes_star,
        rows.len()
    );

    // The gap list's `stops` column, per gap id.
    let mut blocked: std::collections::BTreeMap<String, usize> = std::collections::BTreeMap::new();
    for row in &rows {
        if let Some(id) = row.gap.strip_prefix('G')
            && id.chars().all(|c| c.is_ascii_digit())
        {
            *blocked.entry(row.gap.clone()).or_default() += 1;
        }
    }

    let section = markdown_section(AUDIT_PAGE, "\n## The gap list\n");
    let mut stops_col: Option<usize> = None;
    let mut checked = 0usize;
    for line in section.lines() {
        let Some(cells) = table_cells(line) else {
            // A blank line or prose ends the table that was in flight.
            if !line.trim().starts_with('|') {
                stops_col = None;
            }
            continue;
        };
        if let Some(col) = cells.iter().position(|c| *c == "stops") {
            stops_col = Some(col);
            continue;
        }
        let Some(col) = stops_col else { continue };
        let id = cells[0];
        if !(id.starts_with('G') && id[1..].chars().all(|c| c.is_ascii_digit())) {
            continue;
        }
        let claimed: usize = cells[col]
            .split(|c: char| !c.is_ascii_digit())
            .find(|w| !w.is_empty())
            .unwrap_or_else(|| panic!("gap {id}'s `stops` cell states no number"))
            .parse()
            .expect("a decimal count");
        let actual = blocked.get(id).copied().unwrap_or(0);
        assert_eq!(
            claimed, actual,
            "gap {id} claims it blocks {claimed} stop(s); {actual} row(s) name \
             it as their primary blocker"
        );
        checked += 1;
    }
    assert!(
        checked >= 3,
        "only {checked} gap row(s) were read out of the gap list — the table's \
         shape changed and this guard was about to pass vacuously"
    );
    let unlisted: Vec<&String> = blocked
        .keys()
        .filter(|id| !section.contains(&format!("| {id} |")))
        .collect();
    assert!(
        unlisted.is_empty(),
        "rows name gap id(s) the gap list does not carry: {:?}",
        unlisted
    );
}

// ---- M10-1: distributions authored, saved, reloaded, and read ----

/// **A first-time user's whole loop, façade-only** (ERROR-DESIGN
/// E1/E2): declare two parameters — one with a normal, one with a
/// worst-case band — save the document, load it back, then read the
/// analyzed box and the mass columns.
///
/// The two halves the row exists to pin: the annotation survives the
/// round trip bit for bit, and the band REFUSES to be priced while the
/// normal answers, so the difference between "I know the spread" and
/// "I know only the limits" is visible from outside the crate.
#[test]
fn distributions_author_save_reload_and_analyze_through_the_facade() {
    use pncad::analysis::{AnalysisPolicy, MeasureUnavailable, analyzed_box, box_mass, tail_mass};
    use pncad::document::{
        Dimension, Distribution, DocEdit, DocParam, ParamName, ProfileDoc, apply, load, save,
    };

    let declare = |doc: &ProfileDoc, name: &str, value: DocParam| {
        apply(
            doc,
            &DocEdit::SetDocParam {
                name: ParamName::new(name),
                value,
            },
            Tol::witness(),
        )
        .expect("the parameter declaration applies")
        .doc
    };
    let doc = ProfileDoc::empty_derived("m10-1-e2e", Tol::witness());
    let doc = declare(
        &doc,
        "bore_r",
        DocParam::continuous_with(
            Dimension::Length,
            0.004,
            Distribution::Normal { sigma: 5e-6 },
        ),
    );
    let doc = declare(
        &doc,
        "plate_t",
        DocParam::continuous_with(
            Dimension::Length,
            0.012,
            Distribution::Band {
                lo: -2e-4,
                hi: 2e-4,
            },
        ),
    );

    let text = save(&doc, &[], Tol::witness()).expect("the annotated document saves");
    let back = load(&text, Tol::witness()).expect("and loads").doc;
    assert!(back.bit_eq(&doc), "the annotation round-trips bit for bit");

    let policy = AnalysisPolicy::default();
    let boxed = analyzed_box(&back, &policy);
    let bore = boxed
        .get(&ParamName::new("bore_r"))
        .expect("the annotated parameter is an axis");
    let plate = boxed
        .get(&ParamName::new("plate_t"))
        .expect("so is the banded one");

    // The normal's box is the ±3σ quantile box; the band's IS its
    // support.
    assert!(
        (bore.offsets.hi - 15e-6).abs() < 1e-8,
        "±3σ of 5 µm, got {}",
        bore.offsets.hi
    );
    assert_eq!(plate.offsets.lo, -2e-4);
    assert_eq!(plate.offsets.hi, 2e-4);
    assert_eq!(
        plate.absolute(),
        (0.012 - 2e-4, 0.012 + 2e-4),
        "absolute limits read off the nominal"
    );

    // The tail column: the normal leaves a little outside its box, the
    // band leaves nothing outside its own support.
    let bore_tail = tail_mass(
        &ParamName::new("bore_r"),
        &bore.distribution.expect("annotated"),
        &bore.offsets,
    )
    .expect("a normal is priceable");
    assert!(
        bore_tail > 0.0 && bore_tail < 1e-2,
        "the ±3σ box leaves ~0.27% outside, got {bore_tail}"
    );
    assert_eq!(
        tail_mass(
            &ParamName::new("plate_t"),
            &plate.distribution.expect("annotated"),
            &plate.offsets
        ),
        Ok(0.0)
    );

    // Pricing a sub-box: the normal answers, the band refuses BY NAME.
    let half = box_mass(
        &ParamName::new("bore_r"),
        &bore.distribution.expect("annotated"),
        (0.0, bore.offsets.hi),
    )
    .expect("a normal prices a leaf");
    assert!((half - 0.5 * (1.0 - bore_tail)).abs() < 1e-9, "{half}");
    match box_mass(
        &ParamName::new("plate_t"),
        &plate.distribution.expect("annotated"),
        (0.0, 1e-4),
    ) {
        Err(MeasureUnavailable::BandHasNoMeasure { param }) => {
            assert_eq!(param, ParamName::new("plate_t"));
        }
        other => panic!("a band must refuse to price a leaf, got {other:?}"),
    }
}
