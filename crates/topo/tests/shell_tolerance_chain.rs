//! **No signature on the shell's offset chain names an `f64`
//! epsilon, and the run's ε is read at one site** — the D4 ¶1 witness
//! rule, made mechanical for the one chain that used to break it.
//!
//! The shell doors took a `tolerance: f64` beside `tol: Tol` and passed
//! it down through the face-replacement doors to the fit engine, so two
//! callers of one verb could fit against two different epsilons and
//! nothing said so. They do not any more: the witness travels the whole
//! way — `topo::shell`, the face-replacement doors, the simultaneous
//! offset door, `topo::props`'s lane doors and `geom-brep`'s five
//! production fit doors all name it — and the value is read once, at
//! `geom_brep::offset_fit::precision_target`.
//!
//! # What guards what
//!
//! **The compiler is the primary guard, and these rows do not replace
//! it, and a production door RENAMED to end in `_at` is exempted by
//! the signature row (the census is what catches that one). The
//! declared-reads roster is exact by construction and stale by hand: it
//! reds when a count moves, and says nothing about whether the sentence
//! beside the count is still true.** No caller on this chain has a number to pass, and no amount of
//! discipline is needed for that: the signatures refuse one. What the
//! compiler cannot say is that a LATER edit did not put one back, or
//! that a second `.eps()` did not appear beside the first, or that a
//! production file did not start calling the numeric-target instrument
//! the fit engine keeps for its own suite. That is what the three rows
//! below are — future-edit coverage over properties the type system
//! holds today or, in the instrument's case, does not hold at all.
//!
//! # What they cannot see, stated
//!
//! An epsilon whose parameter name reads as nothing like one (`slack`,
//! `budget`, `width`); one smuggled inside a struct, a tuple or a
//! closure capture; one reached through a type alias for `f64`; a door
//! MOVED out of a sentinel region (the sentinel comments say not to,
//! and nothing enforces it); an `.eps()` read reached through a helper
//! that is itself outside the regions; and the chain growing a file the
//! roster below does not name — that roster is hand-kept, exactly like
//! the censuses it stands beside, and a new module on the chain is a
//! visit here that nothing mechanical demands. The `_at` census reads
//! call spellings, so a numeric-target routine reached through a
//! function pointer or a re-export under another name is invisible to
//! it, and a production door RENAMED to end in `_at` is exempted by
//! the signature row (the census is what catches that one). The
//! declared-reads roster is exact by construction and stale by hand: it
//! reds when a count moves, and says nothing about whether the sentence
//! beside the count is still true.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

/// One stretch of the chain: a display name, the file's whole text, and
/// the sentinel pair that carves the region out of it (`None` = the
/// whole file is on the chain).
struct Stretch {
    file: &'static str,
    source: &'static str,
    sentinels: Option<(&'static str, &'static str)>,
    /// How many `.eps()` reads this stretch is DECLARED to hold, and
    /// what each is for. Not all of them are the fit target: a decide
    /// margin is a threshold in metres and is ε by its own right. The
    /// number is here so a NEW read anywhere on the chain reds and has
    /// to be explained rather than absorbed.
    eps_reads: usize,
    /// What those reads are, in one line.
    eps_reads_are: &'static str,
}

/// The chain, file by file.
///
/// `shell.rs`, `replace_face.rs` and `offset_axial.rs` are read WHOLE —
/// every door in each is on this chain (`offset_axial`'s
/// `offset_charts_together` is the simultaneous arm `shell.rs` takes for
/// all-planar and axial bodies). `props.rs` and `offset_fit.rs` are
/// not: each hosts unrelated machinery with ε reads of its own — the
/// quadrature lane's, and the fit engine's numeric-target instrument —
/// so each is read through the region its sentinels bracket.
const CHAIN: [Stretch; 5] = [
    Stretch {
        file: "crates/topo/src/shell.rs",
        source: include_str!("../src/shell.rs"),
        sentinels: None,
        eps_reads: 0,
        eps_reads_are: "none — the verb doors pass the witness and read nothing",
    },
    Stretch {
        file: "crates/topo/src/replace_face.rs",
        source: include_str!("../src/replace_face.rs"),
        sentinels: None,
        eps_reads: 2,
        eps_reads_are: "two DECIDE margins (`offset_vertex_agreement`, \
                        `offset_reanchor_on_carrier`) — a coincidence threshold in metres, \
                        which is ε by its own right and not the fit target",
    },
    Stretch {
        file: "crates/topo/src/offset_axial.rs",
        source: include_str!("../src/offset_axial.rs"),
        sentinels: None,
        eps_reads: 0,
        eps_reads_are: "none — the simultaneous door decides on the band alone",
    },
    Stretch {
        file: "crates/topo/src/props.rs",
        source: include_str!("../src/props.rs"),
        sentinels: Some(("SHELL-TOLERANCE-CHAIN BEGIN", "SHELL-TOLERANCE-CHAIN END")),
        eps_reads: 0,
        eps_reads_are: "none — the lane doors hand the witness straight on",
    },
    Stretch {
        file: "crates/geom-brep/src/offset_fit.rs",
        source: include_str!("../../geom-brep/src/offset_fit.rs"),
        sentinels: Some(("SHELL-TOLERANCE-CHAIN BEGIN", "SHELL-TOLERANCE-CHAIN END")),
        eps_reads: 1,
        eps_reads_are: "the FIT TARGET, in `precision_target` — the one this chain exists for",
    },
];

impl Stretch {
    /// The guarded text, and the FILE line number its first line is at —
    /// so a hit is reported where a reader can open it, not at an
    /// offset into a region.
    fn region(&self) -> (&'static str, usize) {
        let Some((open, close)) = self.sentinels else {
            return (self.source, 1);
        };
        let start = self
            .source
            .find(open)
            .unwrap_or_else(|| panic!("{}: the opening sentinel is gone", self.file));
        let end = self
            .source
            .find(close)
            .unwrap_or_else(|| panic!("{}: the closing sentinel is gone", self.file));
        assert!(start < end, "{}: the sentinels are inverted", self.file);
        let first_line = self.source[..start].lines().count();
        (&self.source[start..end], first_line)
    }
}

/// A parameter name that reads as a tolerance. Deliberately broader than
/// the one spelling that was there, so a rename does not walk past this
/// row.
fn reads_as_a_tolerance(name: &str) -> bool {
    let n = name.to_ascii_lowercase();
    ["tolerance", "eps", "epsilon", "precision"]
        .iter()
        .any(|w| n.contains(w))
}

/// **No `f64` tolerance parameter anywhere on the chain.**
///
/// Every `name: type` pair on a line is read, not just the first — a
/// one-line `fn f(d: f64, tolerance: f64)` is the shape rustfmt keeps
/// on one line and therefore the easiest hole to leave open.
///
/// Comments AND string literals are blanked (`code_only`), because the
/// needle is a code fragment: prose explaining why the parameter is gone
/// must not read as the parameter coming back, and neither must a
/// refusal's message text.
///
/// **The one exemption, by name**: a routine whose `fn` name ends in
/// `_at` is `geom-brep`'s numeric-target instrument, whose whole point
/// is a chosen target (its own docs and the `_at` census below). The
/// exemption is the name, so renaming a production door to end in `_at`
/// would walk past this row — and would be caught by the census, which
/// then has a door in a set it says only the transform lane reaches.
#[test]
fn no_signature_on_the_shell_chain_names_an_f64_epsilon() {
    let mut hits: Vec<String> = Vec::new();
    for stretch in &CHAIN {
        let (region, first_line) = stretch.region();
        let code = test_utils::source::code_only(region);
        // Which `fn` the current line belongs to, so a numeric-target
        // routine's own signature can be exempted by NAME rather than
        // by hoping it does not look like a door's.
        let mut in_instrument = false;
        for (n, line) in code.lines().enumerate() {
            if let Some(rest) = line.split_once("fn ") {
                let name: String = rest
                    .1
                    .chars()
                    .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
                    .collect();
                in_instrument = name.ends_with("_at");
            }
            if in_instrument {
                continue;
            }
            // Every `ident: f64` on the line, at any position.
            for piece in line.split(',') {
                let Some((lhs, rhs)) = piece.split_once(':') else {
                    continue;
                };
                if !rhs.trim_start().starts_with("f64") {
                    continue;
                }
                let name = lhs
                    .trim()
                    .trim_start_matches(['(', '<', '|'])
                    .trim()
                    .trim_start_matches('_');
                if !name.is_empty()
                    && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
                    && reads_as_a_tolerance(name)
                {
                    hits.push(format!(
                        "{}:{}: {}",
                        stretch.file,
                        first_line + n,
                        line.trim()
                    ));
                }
            }
        }
    }
    assert!(
        hits.is_empty(),
        "an f64 epsilon is back on the shell's offset chain, where the Tol witness is the only \
         tolerance a signature may take (D4 ¶1):\n{}",
        hits.join("\n")
    );
}

/// **Every ε read on the chain is a declared one, and exactly one of
/// them is the FIT TARGET.**
///
/// Counted as `.eps()` READS rather than as calls to the helper that
/// wraps one: a second `let _ = tol.eps();` beside an intact helper is
/// exactly the defect this row exists for, and counting call spellings
/// would miss it — so would pinning a spelling like `precision_target(tol)`,
/// which a local binding or a trailing comma passes by luck.
///
/// Not every read on the chain is the fit target, and the roster says
/// so rather than pretending: `replace_face.rs` decides two coincidence
/// margins at ε, which is ε used as a threshold in metres and has
/// nothing to do with what an offset fit must reach. What the row holds
/// is that the roster is exact — a new read anywhere reds and has to be
/// declared with what it is — and that the fit target's read is where
/// this chain's docs say it is.
#[test]
fn the_chain_reads_epsilon_at_one_site() {
    for stretch in &CHAIN {
        let (region, _) = stretch.region();
        let code = test_utils::source::code_only(region);
        let found = code.matches(".eps()").count();
        assert_eq!(
            found, stretch.eps_reads,
            "{} holds {found} `.eps()` read(s) on the chain, not the {} it declares. What the \
             declared ones are: {}. A new read is not forbidden — it is undeclared, and this row \
             is where it gets said what it is for.",
            stretch.file, stretch.eps_reads, stretch.eps_reads_are
        );
    }
    // The fit target itself: one read, in the module the chain ends in,
    // inside the function the docs name.
    let fit = CHAIN
        .iter()
        .find(|s| s.file == "crates/geom-brep/src/offset_fit.rs")
        .expect("the fit module is on the chain");
    assert_eq!(
        fit.eps_reads, 1,
        "the fit target must be read exactly once; the roster says {}",
        fit.eps_reads
    );
    let (region, _) = fit.region();
    let code = test_utils::source::code_only(region);
    assert!(
        code.contains("fn precision_target"),
        "the declared read site `precision_target` is gone from the guarded region"
    );
    let body = code
        .split_once("fn precision_target")
        .expect("the site is there")
        .1;
    let end = body.find("\n}").expect("the site has a body");
    assert!(
        body[..end].contains(".eps()"),
        "`precision_target` no longer reads ε — the one read has moved somewhere the roster \
         still counts but the docs no longer name"
    );
}

/// **The numeric-target instrument has exactly one production caller**,
/// and it is named.
///
/// `geom-brep`'s `_at` routines take a chosen target and exist so the
/// fit engine's own suite can measure it. A production file reaching one
/// is a caller choosing an epsilon, which is the thing the witness rule
/// removes — except at the transform lane, which classifies a mapped
/// pair against the tolerance the SURFACE's claim was made at (a stored
/// datum, argued at `topo::transform::map_approx`). That exception is
/// listed here by file, so a second one reds.
#[test]
fn only_the_transform_lane_reaches_the_numeric_target_routines() {
    const AT_ROUTINES: [&str; 5] = [
        "fit_offset_at(",
        "certify_offset_at(",
        "certify_offset_over_at(",
        "approx_offset_surface_at(",
        "recertify_approx_at(",
    ];
    // The routines' own home, where the `Tol` doors delegate, and the
    // one ratified exception.
    const ALLOWED: [&str; 2] = [
        "crates/geom-brep/src/offset_fit.rs",
        "crates/geom-brep/src/pcurve_cache.rs",
    ];
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(std::path::Path::parent)
        .expect("the crate sits two levels under the workspace root")
        .join("crates");
    let mut hits: Vec<String> = Vec::new();
    let mut seen_exception = false;
    let mut stack = vec![root.clone()];
    let mut scanned = 0_usize;
    while let Some(dir) = stack.pop() {
        for entry in std::fs::read_dir(&dir).expect("the crate tree is readable") {
            let path = entry.expect("a readable directory entry").path();
            if path.is_dir() {
                stack.push(path);
                continue;
            }
            if path.extension().is_none_or(|e| e != "rs") {
                continue;
            }
            let shown = path.to_string_lossy().replace('\\', "/");
            // `src/` only: a suite calling the instrument is what the
            // instrument is for.
            if !shown.contains("/src/") {
                continue;
            }
            scanned += 1;
            let text = std::fs::read_to_string(&path).expect("a readable source file");
            let code = test_utils::source::code_only(&text);
            if !AT_ROUTINES.iter().any(|r| code.contains(r)) {
                continue;
            }
            if shown.ends_with(ALLOWED[1]) {
                seen_exception = true;
                continue;
            }
            if ALLOWED.iter().any(|a| shown.ends_with(a)) {
                continue;
            }
            hits.push(shown);
        }
    }
    // A walk that read nothing would pass vacuously.
    assert!(
        scanned > 100,
        "the production-source walk found only {scanned} files — it is looking in the wrong place"
    );
    assert!(
        hits.is_empty(),
        "a production file reaches `geom-brep`'s numeric-target fit routines, which means a \
         caller is choosing an epsilon: {hits:?}"
    );
    assert!(
        seen_exception,
        "the transform lane no longer reaches the numeric-target routine — either it moved (this \
         census is now measuring the wrong set) or the exception is gone and should be deleted"
    );
}
