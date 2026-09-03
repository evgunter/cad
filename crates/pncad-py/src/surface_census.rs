//! **The census over kernel vocabularies that Python re-spells.**
//!
//! Three surfaces here are hand-written copies of a vocabulary the
//! kernel declares once: the PATHS verbs, the arc-spec modes that
//! travel inside them, and `StepOptions`' fields at the export door.
//! Each copy compiles green while short — a verb the transition table
//! gains, a mode `arc_modes!` gains, a field `StepOptions` gains, all
//! reach `pncad-py` through methods that were never exhaustive over
//! them — so a Python user simply cannot write the thing, and nothing
//! says so.
//!
//! The mechanism is the one `editor-core`'s
//! `switch_program_vocabulary` suite uses one crate over: **the
//! witness is a MATCH on the kernel tag, not a list.** A verb or mode
//! the vocabulary gains has no arm in [`verb_spelling`] or
//! [`mode_class`] below, so it does not fail an assertion here — it
//! fails to COMPILE, and the arm someone then writes is a decision
//! recorded in one of two shapes: a Python spelling, or a
//! [`Spelling::NotBound`] carrying the reason. `StepOptions` is a
//! struct rather than an enum, so its anchor is the same device in
//! pattern form: an exhaustive destructure with no `..`.
//!
//! # Which Python side this reads
//!
//! `pncad.pyi`, as TEXT. The stub is a faithful stand-in for the
//! compiled module because `tests/test_stubs.py` pins the two to each
//! other name for name; reading it instead of importing `pncad` is
//! what lets this census run on the default build path, with no
//! `python` feature, no interpreter and nothing built — the same
//! argument `tests/test_binding_census.py` makes for its own scan.
//!
//! What it therefore does NOT check: that a spelling means what its
//! kernel name means. `PathOpen.to` is claimed here to be the Python
//! spelling of `CloseTo`, and nothing mechanical says it is; the
//! corpus tests and `ty` are where that lives. This census answers
//! one question only — can a Python caller REACH each member of the
//! vocabulary — and it answers it in both directions, because an arm
//! naming a spelling the stub does not declare fails exactly as a
//! missing spelling does.

// Per the workspace convention recorded in the root Cargo.toml: test
// code may allow the panic family, because panicking IS a test's
// failure mechanism.
#![allow(clippy::expect_used, clippy::panic)]

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use pncad::profile::{ArcMode, Verb};
use pncad::step_export::StepOptions;

// ------------------------------------------------------------------
// The Python side: `pncad.pyi` as text
// ------------------------------------------------------------------

/// What the stub declares, at the granularity this census compares.
struct Stub {
    /// Top-level `class`/`def`/annotated names, and every
    /// `Class.member` spelling, in one alphabet — the same alphabet
    /// `tests/test_binding_census.py` compares its rosters in.
    names: BTreeSet<String>,
    /// Each `def`'s source text from `def` to its closing paren,
    /// keyed by the same qualified name.
    defs: BTreeMap<String, String>,
    /// Top-level `NAME = <rhs>` right-hand sides — the stub's private
    /// `TypeAlias`es, which is where the admissibility unions live.
    aliases: BTreeMap<String, String>,
}

/// Whether `hay` mentions `needle` as a whole identifier.
///
/// Substring matching would let `Center` be satisfied by a
/// `CenterSomething`, which is the failure that would make the
/// reachability clause below decorative.
fn mentions(hay: &str, needle: &str) -> bool {
    let boundary = |c: Option<char>| !c.is_some_and(|c| c.is_alphanumeric() || c == '_');
    let mut from = 0;
    while let Some(at) = hay[from..].find(needle) {
        let start = from + at;
        let end = start + needle.len();
        if boundary(hay[..start].chars().next_back()) && boundary(hay[end..].chars().next()) {
            return true;
        }
        from = start + 1;
    }
    false
}

/// The text between a `def` block's OUTER parens — its parameter
/// list, and nothing after it.
///
/// Bracket-depth-keyed rather than `rfind(')')`, so a default value or
/// an annotation carrying its own parens ends where it opens.
fn parameter_text(block: &str) -> String {
    let Some(open) = block.find('(') else {
        return String::new();
    };
    let mut depth = 1;
    let mut out = String::new();
    for ch in block[open + 1..].chars() {
        match ch {
            '(' | '[' => depth += 1,
            ')' | ']' => {
                depth -= 1;
                if depth == 0 {
                    break;
                }
            }
            _ => {}
        }
        out.push(ch);
    }
    out
}

/// The leading identifier of `s`, empty when it does not start with one.
fn leading_ident(s: &str) -> &str {
    let s = s.trim_start();
    let end = s
        .find(|c: char| !(c.is_alphanumeric() || c == '_'))
        .unwrap_or(s.len());
    &s[..end]
}

impl Stub {
    /// Scan `pncad.pyi`.
    ///
    /// Line-oriented and indentation-keyed, which the stub's formatting
    /// makes exact: a class opens at column 0 and its members sit at
    /// four. Triple-quoted blocks are tracked and skipped — the module
    /// docstring is prose at column 0, and reading it as declarations
    /// would invent names out of sentences.
    fn read(src: &str) -> Self {
        let mut names = BTreeSet::new();
        let mut defs = BTreeMap::new();
        let mut aliases = BTreeMap::new();
        let mut class: Option<String> = None;
        let mut in_doc = false;
        let mut lines = src.lines().peekable();

        while let Some(line) = lines.next() {
            let quotes = line.matches("\"\"\"").count();
            if in_doc {
                if quotes % 2 == 1 {
                    in_doc = false;
                }
                continue;
            }
            let trimmed = line.trim_start();
            if trimmed.starts_with('#') || trimmed.is_empty() {
                continue;
            }
            if quotes % 2 == 1 {
                in_doc = true;
                continue;
            }
            let indent = line.len() - trimmed.len();
            if indent > 4 {
                continue;
            }
            if indent == 0 {
                class = None;
            }

            let qualify = |name: &str, class: &Option<String>| match (indent, class) {
                (0, _) => name.to_owned(),
                (_, Some(c)) => format!("{c}.{name}"),
                (_, None) => name.to_owned(),
            };

            if let Some(rest) = trimmed.strip_prefix("class ") {
                let name = leading_ident(rest);
                names.insert(name.to_owned());
                if indent == 0 {
                    class = Some(name.to_owned());
                }
            } else if let Some(rest) = trimmed.strip_prefix("def ") {
                let name = leading_ident(rest);
                // A signature may wrap; the block runs to the paren
                // that closes the parameter list.
                let mut block = String::from(rest);
                let mut depth: i32 = 0;
                let mut seen_open = false;
                loop {
                    for ch in block.chars() {
                        match ch {
                            '(' => {
                                depth += 1;
                                seen_open = true;
                            }
                            ')' => depth -= 1,
                            _ => {}
                        }
                    }
                    if seen_open && depth <= 0 {
                        break;
                    }
                    match lines.next() {
                        Some(next) => {
                            block.push('\n');
                            block.push_str(next);
                        }
                        None => break,
                    }
                    depth = 0;
                    seen_open = false;
                    let recount = block.clone();
                    for ch in recount.chars() {
                        match ch {
                            '(' => {
                                depth += 1;
                                seen_open = true;
                            }
                            ')' => depth -= 1,
                            _ => {}
                        }
                    }
                    if seen_open && depth <= 0 {
                        break;
                    }
                }
                let qualified = qualify(name, &class);
                names.insert(qualified.clone());
                defs.entry(qualified).or_insert(block);
            } else if trimmed.starts_with('@') {
                continue;
            } else if let Some(colon) = trimmed.find(':') {
                // `NAME: annotation` / `NAME: TypeAlias = rhs`.
                let name = leading_ident(trimmed);
                if !name.is_empty() && name.len() == colon {
                    names.insert(qualify(name, &class));
                    if indent == 0
                        && let Some(eq) = trimmed.find('=')
                    {
                        aliases.insert(name.to_owned(), trimmed[eq + 1..].to_owned());
                    }
                }
            } else if let Some(eq) = trimmed.find('=') {
                let name = leading_ident(trimmed);
                if !name.is_empty() && name.len() == trimmed[..eq].trim_end().len() {
                    names.insert(qualify(name, &class));
                    if indent == 0 {
                        aliases.insert(name.to_owned(), trimmed[eq + 1..].to_owned());
                    }
                }
            }
        }

        Stub {
            names,
            defs,
            aliases,
        }
    }

    /// Whether the stub declares `name`, a bare top-level name or a
    /// `Class.member` spelling alike.
    fn declares(&self, name: &str) -> bool {
        self.names.contains(name)
    }

    /// Every type name a stub signature ACCEPTS, as one text.
    ///
    /// A mode class the stub declares but no verb accepts is a class a
    /// Python caller can construct and can pass to nothing, which is
    /// the mode-vocabulary shape of the same silent gap. The private
    /// `TypeAlias`es are resolved to a fixpoint because that is where
    /// the admissibility unions are written: `_PointLeg` is what
    /// `arc_to` names, and `Bulge` is inside it.
    ///
    /// PARAMETER positions only — the text between each `def`'s outer
    /// parens, so a return annotation contributes nothing. A class the
    /// surface only ever HANDS BACK is not a class a caller can
    /// author with, and counting it would make "admitted by a verb"
    /// mean "mentioned near one".
    fn signature_reach(&self) -> String {
        let mut text: String = self
            .defs
            .values()
            .map(|block| parameter_text(block))
            .collect::<Vec<_>>()
            .join("\n");
        loop {
            let grown: Vec<&String> = self
                .aliases
                .iter()
                .filter(|(name, rhs)| mentions(&text, name) && !text.contains(rhs.as_str()))
                .map(|(_, rhs)| rhs)
                .collect();
            if grown.is_empty() {
                return text;
            }
            for rhs in grown {
                text.push('\n');
                text.push_str(rhs);
            }
        }
    }

    /// The parameter names of one `def`.
    fn parameters(&self, qualified: &str) -> BTreeSet<String> {
        let Some(block) = self.defs.get(qualified) else {
            return BTreeSet::new();
        };
        let mut params = BTreeSet::new();
        let mut depth = 0;
        let mut piece = String::new();
        for ch in parameter_text(block).chars() {
            match ch {
                '(' | '[' => depth += 1,
                ')' | ']' => depth -= 1,
                ',' if depth == 0 => {
                    params.insert(leading_ident(&piece).to_owned());
                    piece.clear();
                    continue;
                }
                _ => {}
            }
            piece.push(ch);
        }
        params.insert(leading_ident(&piece).to_owned());
        params.remove("");
        params
    }
}

fn stub() -> Stub {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("pncad.pyi");
    Stub::read(
        &std::fs::read_to_string(&path).expect("this crate's stub sits beside its Cargo.toml"),
    )
}

// ------------------------------------------------------------------
// The rosters
// ------------------------------------------------------------------

/// How one member of a kernel vocabulary reaches Python.
enum Spelling {
    /// The stub spellings a caller writes to use it. Every one is
    /// checked to exist, so a plausible-looking name is not a way to
    /// wave a member off.
    Bound(&'static [&'static str]),
    /// Deliberately unreachable from Python, with the reason. Written
    /// rather than defaulted: an unbound member is a decision here,
    /// and the census's whole subject is decisions that were never
    /// written down.
    ///
    /// It names the spelling it WOULD have, and that is what keeps it
    /// falsifiable. A variant carrying a reason alone asserts nothing
    /// a test can check: it contributes no names, so it stays green
    /// forever — including after someone binds the member, at which
    /// point the reason is a lie nobody is reading.
    /// [`the_not_bound_roster_decays`] is the other half, and it is
    /// the half `test_binding_census.py`'s `test_the_rosters_decay`
    /// already models for its own `NOT_BOUND`.
    NotBound {
        /// What a caller would write if this member were bound —
        /// checked to be ABSENT from the stub.
        would_be: &'static [&'static str],
        /// Why it is not.
        reason: &'static str,
    },
}

/// **The verb roster.** One arm per verb the transition table
/// declares; a verb it gains stops this function compiling.
///
/// The spellings are METHODS, and several verbs are spelled at more
/// than one lattice state because Rust gives them to more than one
/// state — the census requires every listed spelling, so a state that
/// loses a verb reds here too.
fn verb_spelling(verb: Verb) -> Spelling {
    use Spelling::Bound;
    match verb {
        Verb::At => Bound(&["Open.at", "PathOpen.at", "PathAngle.at"]),
        Verb::Angle => Bound(&["Open.angle", "PathOpen.angle", "PathPoint.angle"]),
        Verb::Toward => Bound(&["Open.toward", "PathOpen.toward", "PathPoint.toward"]),
        Verb::Tangent => Bound(&["PathDirectedPoint.tangent"]),
        Verb::Cusp => Bound(&["PathDirectedPoint.cusp"]),
        Verb::Turn => Bound(&["PathDirectedPoint.turn"]),
        Verb::Line => Bound(&["PathDirected.line"]),
        Verb::LineTo => Bound(&["PathPoint.line_to", "PathDirectedPoint.line_to"]),
        // The declared point-target continuation reaches the Rust
        // authoring algebra and stops there: binding a verb in Python
        // is its own surface work (the stub, its typing, and the
        // binding suite), and the unit that added this one does not
        // own that surface. Recorded here as ABSENT rather than left
        // to be discovered — which is what this arm is for.
        Verb::ContinueTo => Spelling::NotBound {
            would_be: &["PathDirectedPoint.continue_to"],
            reason: "the Rust authoring algebra gained the verb; the Python surface is bound \
                     by its own units and has not caught up",
        },
        Verb::ArcTo => Bound(&[
            "PathPoint.arc_to",
            "PathDirectedPoint.arc_to",
            "PathDirected.arc_to",
        ]),
        Verb::TangentArcTo => Bound(&["PathDirected.tangent_arc_to"]),
        Verb::ArcContinue => Bound(&["PathDirectedPoint.arc_continue"]),
        Verb::Fillet => Bound(&["PathDirected.fillet", "PathDirectedPoint.fillet"]),
        Verb::FilletArc => Bound(&["PathDirected.fillet_arc", "PathDirectedPoint.fillet_arc"]),
        Verb::ArcFillet => Bound(&[
            "Open.arc_fillet",
            "PathPoint.arc_fillet",
            "PathDirectedPoint.arc_fillet",
            "PathDirected.arc_fillet",
        ]),
        Verb::ArcFilletArc => Bound(&[
            "Open.arc_fillet_arc",
            "PathPoint.arc_fillet_arc",
            "PathDirectedPoint.arc_fillet_arc",
            "PathDirected.arc_fillet_arc",
        ]),
        // The far-end anchor and the close are both `to`: the Rust
        // names distinguish them by the state they are legal at, and
        // in Python the CLASS is that state, so one method name at two
        // classes is the same distinction and not a collision.
        Verb::FarEndTo => Bound(&["PathAngle.to"]),
        Verb::CloseTo => Bound(&["PathOpen.to"]),
        // The two complete-loop forms are free functions, not lattice
        // methods, on both surfaces.
        Verb::Circle => Bound(&["circle"]),
        Verb::CircleSplit => Bound(&["circle_split"]),
    }
}

/// **The mode roster.** The Python class a caller constructs to
/// author each arc mode; a mode `arc_modes!` gains stops this
/// function compiling.
///
/// One name rather than a list, because the modes are VALUES in
/// Python — one class each, passed to whichever verb admits it — and
/// which verbs admit which mode is the reachability clause below, not
/// a second roster to keep in step.
fn mode_class(mode: ArcMode) -> &'static str {
    match mode {
        ArcMode::Radius => "Radius",
        ArcMode::Bulge => "Bulge",
        ArcMode::Via => "Via",
        ArcMode::Center => "Center",
        ArcMode::Sweep => "Sweep",
        ArcMode::ArcLen => "ArcLen",
    }
}

/// **The export-options roster**: one entry per `StepOptions` field,
/// naming the `Evaluation.step_string` keyword that sets it.
///
/// The destructure above the list is the anchor, and it has no `..`:
/// a field the kernel struct gains does not compile until it is
/// dispositioned here — and the door itself builds its options with a
/// struct literal, which breaks in the same commit for the same
/// reason.
fn step_option_keywords() -> Vec<(&'static str, Spelling)> {
    let options = StepOptions::default();
    let StepOptions {
        product_name: _,
        timestamp: _,
        author: _,
        organization: _,
        originating_system: _,
        uncertainty_m: _,
    } = &options;
    use Spelling::Bound;
    vec![
        ("product_name", Bound(&["product_name"])),
        ("timestamp", Bound(&["timestamp"])),
        ("author", Bound(&["author"])),
        ("organization", Bound(&["organization"])),
        ("originating_system", Bound(&["originating_system"])),
        // The Rust field is a bare `f64` in metres and says so in its
        // name; the Python keyword takes a `Length`, so the suffix
        // would be a second spelling of what the type already says.
        ("uncertainty_m", Bound(&["uncertainty"])),
    ]
}

// ------------------------------------------------------------------
// The censuses
// ------------------------------------------------------------------

/// What a roster entry is missing from the stub, if anything.
fn absent(stub: &Stub, spelling: &Spelling) -> Vec<String> {
    match spelling {
        Spelling::Bound(names) => names
            .iter()
            .filter(|n| !stub.declares(n))
            .map(|n| (*n).to_owned())
            .collect(),
        Spelling::NotBound { .. } => Vec::new(),
    }
}

/// Every roster entry in the file, as `(member, spelling)` — the two
/// rosters that use [`Spelling`], read together so a decay check
/// cannot cover one and quietly skip the other.
fn every_roster_entry() -> Vec<(String, Spelling)> {
    Verb::ALL
        .iter()
        .map(|verb| (format!("Verb::{verb:?}"), verb_spelling(*verb)))
        .chain(
            step_option_keywords()
                .into_iter()
                .map(|(field, spelling)| (format!("StepOptions::{field}"), spelling)),
        )
        .collect()
}

/// **The roster decays.** A member listed as deliberately unbound
/// whose spelling the stub NOW declares is a stale entry: someone
/// bound it and left a reason standing that says they did not.
///
/// This is the half that makes [`Spelling::NotBound`] an assertion
/// rather than a comment. Nothing is unbound today, so it asserts over
/// an empty set — which is the point of writing it now: the first
/// member to be declined arrives with its decay already checked,
/// rather than depending on whoever declines it to think of this.
#[test]
fn the_not_bound_roster_decays() {
    let stub = stub();
    let stale: Vec<String> = every_roster_entry()
        .into_iter()
        .filter_map(|(member, spelling)| match spelling {
            Spelling::NotBound { would_be, reason } => {
                let found: Vec<&str> = would_be
                    .iter()
                    .copied()
                    .filter(|name| stub.declares(name))
                    .collect();
                (!found.is_empty())
                    .then(|| format!("{member} is bound at {found:?}, but says: {reason}"))
            }
            Spelling::Bound(_) => None,
        })
        .collect();
    assert!(
        stale.is_empty(),
        "these members are listed as not bound and the stub binds them: {stale:?}"
    );
}

/// The scanners found a surface, so the set comparisons below mean
/// something. A scan that returned nothing would satisfy every one of
/// them having read no Python at all.
#[test]
fn the_census_is_not_vacuous() {
    let stub = stub();
    assert!(
        stub.names.len() > 200,
        "the stub scanner found {} names — the format moved and this census was about to \
         pass having read nothing",
        stub.names.len()
    );
    assert!(
        stub.defs.len() > 100,
        "the signature scanner found {} defs",
        stub.defs.len()
    );
    assert!(
        !stub.aliases.is_empty(),
        "no top-level alias was read, so the mode reachability clause resolves nothing"
    );
    assert!(!Verb::ALL.is_empty() && !ArcMode::ALL.is_empty());
    assert!(
        stub.defs.contains_key("Evaluation.step_string"),
        "the export door's signature was not found, so its keyword census reads nothing"
    );
}

/// **The verb census.** Every verb the transition table declares is
/// reachable from Python at every state this roster claims, or is
/// recorded as deliberately unbound.
#[test]
fn every_path_verb_has_a_python_spelling() {
    let stub = stub();
    let missing: Vec<String> = Verb::ALL
        .iter()
        .flat_map(|verb| {
            absent(&stub, &verb_spelling(*verb))
                .into_iter()
                .map(move |name| format!("{verb:?} -> {name}"))
        })
        .collect();
    assert!(
        missing.is_empty(),
        "the roster claims Python spellings pncad.pyi does not declare: {missing:?}"
    );
}

/// **The mode census.** Every arc mode is a class a Python caller can
/// construct, AND that class is admitted by at least one signature —
/// a class no verb accepts is authored by nobody.
#[test]
fn every_arc_mode_has_a_python_spelling() {
    let stub = stub();
    let reach = stub.signature_reach();
    let undeclared: Vec<&str> = ArcMode::ALL
        .iter()
        .map(|mode| mode_class(*mode))
        .filter(|class| !stub.declares(class))
        .collect();
    assert!(
        undeclared.is_empty(),
        "pncad.pyi declares no class for these arc modes: {undeclared:?}"
    );
    let unreachable: Vec<&str> = ArcMode::ALL
        .iter()
        .map(|mode| mode_class(*mode))
        .filter(|class| !mentions(&reach, class))
        .collect();
    assert!(
        unreachable.is_empty(),
        "these arc-mode classes appear in no signature, so no verb admits them: {unreachable:?}"
    );
}

/// **The export-options census.** Every `StepOptions` field is a
/// keyword of the Python door, or recorded as deliberately withheld.
#[test]
fn every_step_option_reaches_the_python_door() {
    let stub = stub();
    let parameters = stub.parameters("Evaluation.step_string");
    let missing: Vec<String> = step_option_keywords()
        .into_iter()
        .flat_map(|(field, spelling)| {
            let names = match spelling {
                Spelling::Bound(names) => names,
                Spelling::NotBound { .. } => &[][..],
            };
            names
                .iter()
                .filter(|kw| !parameters.contains(**kw))
                .map(move |kw| format!("{field} -> {kw}"))
                .collect::<Vec<_>>()
        })
        .collect();
    assert!(
        missing.is_empty(),
        "the export door's Python signature is short of StepOptions: {missing:?}; \
         it declares {parameters:?}"
    );
}

/// The scanner reads the shapes it claims to. Without this, a scanner
/// that silently stopped at, say, wrapped signatures would report
/// everything present by reading nothing about it.
#[test]
fn the_stub_scanner_reads_what_it_claims() {
    let stub = Stub::read(
        "\"\"\"Module doc.\n\nclass NotADeclaration:\n\"\"\"\n\
         _Alias: TypeAlias = Bulge | Via\n\
         class Widget:\n    \
         def wrapped(\n        self,\n        spec: _Alias,\n    ) -> None: ...\n    \
         def plain(self) -> ReturnedOnly: ...\n\
         def free(x: int) -> None: ...\n",
    );
    assert!(!stub.declares("NotADeclaration"), "prose read as a class");
    assert!(stub.declares("Widget"));
    assert!(stub.declares("Widget.wrapped"));
    assert!(stub.declares("Widget.plain"));
    assert!(stub.declares("free"));
    assert_eq!(
        stub.parameters("Widget.wrapped"),
        ["self", "spec"].iter().map(|s| (*s).to_owned()).collect(),
        "a wrapped signature's parameters"
    );
    let reach = stub.signature_reach();
    assert!(
        mentions(&reach, "Bulge"),
        "an alias named in a signature is resolved into the reach"
    );
    assert!(!mentions(&reach, "Center"), "the reach invents nothing");
    assert!(
        !mentions(&reach, "ReturnedOnly"),
        "a type only ever RETURNED is not a type a caller can author with, so it is \
         not in the reach"
    );
    assert!(mentions("a Center,", "Center") && !mentions("Centered", "Center"));
}
