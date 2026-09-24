//! **The census over kernel vocabularies that Python re-spells.**
//!
//! The surfaces here are hand-written copies of a vocabulary the
//! kernel declares once: the PATHS verbs, the arc-spec modes that
//! travel inside them, the fields of the kernel OPTIONS STRUCTS a
//! Python door configures, and the recipe NODE variants (with the
//! datum shapes inside one) the `Node.*` constructors author. Each
//! copy compiles green while short — a verb the transition table
//! gains, a mode `arc_modes!` gains, a field an options struct gains,
//! a node variant the recipe gains, all reach `pncad-py` through
//! methods that were never exhaustive over them — so a Python user
//! simply cannot write the thing, and nothing says so.
//!
//! **Which options structs, exactly.** Those named `*Options` and
//! constructed under `src/py/`, which is a rule a test enforces
//! ([`every_options_type_in_py_is_rostered`]) rather than a claim
//! this paragraph makes. It is narrower than "every configuration
//! struct that reaches Python": `ChecksConfig` and `McConfig` cross
//! as value classes under a `*Config` name and are outside it, with
//! their own row on LIB's slate. That test's doc states the blind
//! spot; this header does not restate it.
//!
//! The mechanism is the one `editor-core`'s
//! `switch_program_vocabulary` suite uses one crate over: **the
//! witness is a MATCH on the kernel tag, not a list.** A verb or mode
//! the vocabulary gains has no arm in [`verb_spelling`] or
//! [`mode_class`] below, so it does not fail an assertion here — it
//! fails to COMPILE, and the arm someone then writes is a decision
//! recorded in one of two shapes: a Python spelling, or a
//! [`Spelling::NotBound`] carrying the reason. An options struct is a
//! struct rather than an enum, so its anchor is the same device in
//! pattern form: an exhaustive destructure with no `..`, one per
//! struct in [`options_doors`]. A node variant carries a payload no
//! test can conjure one of generically, so its anchor is the match
//! `test_utils::f6_variants!` writes over the bare identifiers
//! ([`NODE_VARIANTS`], [`DATUM_VARIANTS`]): a new variant stops that
//! compiling, and once named there it has no entry in
//! [`NODE_CONSTRUCTORS`] until someone writes one.
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

use pncad::document::{Datum, EvalOptions, Node, ProfileProgram};
use pncad::profile::{ArcMode, TargetKind, Verb};
use pncad::step_export::StepOptions;
use pncad::step_import::ImportOptions;
use pncad::stl::{AsciiOptions, BinaryOptions};

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
    /// keyed by the same qualified name — one text per `@overload`,
    /// because the overloads of one verb take different types.
    defs: BTreeMap<String, Vec<String>>,
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
                defs.entry(qualified).or_insert_with(Vec::new).push(block);
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
            .flatten()
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

    /// The parameter names of one `def`, over all its overloads.
    fn parameters(&self, qualified: &str) -> BTreeSet<String> {
        let mut params = BTreeSet::new();
        for block in self.defs.get(qualified).into_iter().flatten() {
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
        }
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
#[derive(Clone)]
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
        // authoring algebra and, since the seam's declared arrival
        // landed, the DOCUMENT and PERSISTED vocabularies too. What it
        // still does not reach is Python: binding a verb there is its
        // own surface work (the stub, its typing, and the binding
        // suite), which no unit so far has owned. Recorded here as
        // ABSENT rather than left to be discovered — which is what this
        // arm is for, and `the_not_bound_roster_decays` is what stops
        // the reason outliving the fact.
        Verb::ContinueTo => Spelling::NotBound {
            would_be: &["PathDirectedPoint.continue_to"],
            reason: "the Rust algebra and the document vocabulary both spell it; the Python \
                     surface is bound by its own units and has not caught up",
        },
        Verb::ArcTo => Bound(&[
            "PathPoint.arc_to",
            "PathDirectedPoint.arc_to",
            "PathDirected.arc_to",
        ]),
        Verb::TangentArcTo => Bound(&["PathDirected.tangent_arc_to"]),
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

/// **The target roster.** The Python class a caller passes to aim a
/// leg at each target form; a form `TargetKind` gains stops this
/// function compiling.
///
/// `None` for an authored point, which is a coordinate tuple and not a
/// class: the forms with a class are the `Start` tokens, one each.
fn target_class(kind: TargetKind) -> Option<&'static str> {
    match kind {
        TargetKind::Point => None,
        TargetKind::Start => Some("StartToken"),
        TargetKind::StartArriving => Some("ArrivesTangentToken"),
    }
}

/// The recipe node the `Node.*` constructors author, at the payload
/// this crate's `Node` class carries.
type KernelNode = Node<ProfileProgram>;

test_utils::f6_variants! {
    /// **Every kernel `Node` variant**, welded to `Node` by the match
    /// the macro writes: a variant the kernel gains leaves that match
    /// non-exhaustive, and once its identifier is written here
    /// [`NODE_CONSTRUCTORS`] reds until it has a disposition.
    ///
    /// `node_kind`'s match already stops compiling on a new variant,
    /// but what it forces is a KIND WORD — the read half. Nothing there
    /// asks whether a Python caller can AUTHOR the node, and a variant
    /// whose payload reuses types Python already spells mints no new
    /// curated name for the export census to miss either.
    const NODE_VARIANTS: KernelNode = [
        Datum,
        Profile,
        Extrude,
        Revolve,
        Tube,
        HollowTube,
        Loft,
        Sweep,
        Fillet,
        Chamfer,
        Shell,
        Split,
        Boolean,
        Union,
        Transform,
        Pattern,
        Part,
        PlacedUnion,
        Declare,
        InstantiatePart,
        Mate,
        Measure,
        Assertion,
    ];
}

test_utils::f6_variants! {
    /// **Every datum shape**, welded the same way. `Node::Datum` is one
    /// node variant and one kind word over six shapes with six
    /// constructors, so the node roster alone would pass over a shape
    /// the kernel gains inside it.
    const DATUM_VARIANTS: Datum = [Plane, Axis, Point, Frame, AxisInPlane, FaceFrame];
}

/// **The node roster.** The `Node.*` constructors that author each
/// kernel `Node` variant, keyed by [`NODE_VARIANTS`]' identifiers and
/// held equal to them in both directions by
/// [`every_node_variant_has_a_python_constructor`].
///
/// Several constructors author one variant where the Python surface
/// offers more than one way to write it; the census requires every one
/// listed, so a constructor that goes away reds here too.
const NODE_CONSTRUCTORS: &[(&str, Spelling)] = &[
    (
        "Datum",
        Spelling::Bound(&[
            "Node.datum_plane",
            "Node.datum_axis",
            "Node.datum_point",
            "Node.datum_frame",
            "Node.sketch_frame",
            "Node.datum_axis_in_plane",
            "Node.datum_face_frame",
        ]),
    ),
    (
        "Profile",
        Spelling::Bound(&["Node.profile", "Node.polygon"]),
    ),
    ("Extrude", Spelling::Bound(&["Node.extrude"])),
    ("Revolve", Spelling::Bound(&["Node.revolve"])),
    ("Tube", Spelling::Bound(&["Node.tube"])),
    ("HollowTube", Spelling::Bound(&["Node.hollow_tube"])),
    ("Loft", Spelling::Bound(&["Node.loft"])),
    (
        "Sweep",
        Spelling::NotBound {
            would_be: &["Node.sweep"],
            reason: "the kernel's sweep door is banked — `wire_sweep` refuses unconditionally \
                     (`SWEEP_FRONTIER`) — so a constructor would author a node every \
                     evaluation refuses; the north-star audit carries it as gap G2",
        },
    ),
    ("Fillet", Spelling::Bound(&["Node.fillet"])),
    ("Chamfer", Spelling::Bound(&["Node.chamfer"])),
    ("Shell", Spelling::Bound(&["Node.shell"])),
    ("Split", Spelling::Bound(&["Node.split"])),
    ("Boolean", Spelling::Bound(&["Node.boolean"])),
    ("Union", Spelling::Bound(&["Node.union"])),
    ("Transform", Spelling::Bound(&["Node.transform"])),
    ("Pattern", Spelling::Bound(&["Node.pattern"])),
    ("Part", Spelling::Bound(&["Node.part"])),
    (
        "PlacedUnion",
        Spelling::Bound(&["Node.placed_union", "Node.placed_union_at"]),
    ),
    ("Declare", Spelling::Bound(&["Node.declare"])),
    (
        "InstantiatePart",
        Spelling::Bound(&["Node.instantiate_part"]),
    ),
    ("Mate", Spelling::Bound(&["Node.mate"])),
    ("Measure", Spelling::Bound(&["Node.measure"])),
    ("Assertion", Spelling::Bound(&["Node.assertion"])),
];

/// **The datum roster.** The constructor that authors each datum
/// shape, keyed by [`DATUM_VARIANTS`]' identifiers. A frame has two:
/// `sketch_frame` is the frame a profile's plane names, spelled as a
/// `SketchPlane` or an elevation.
const DATUM_CONSTRUCTORS: &[(&str, Spelling)] = &[
    ("Plane", Spelling::Bound(&["Node.datum_plane"])),
    ("Axis", Spelling::Bound(&["Node.datum_axis"])),
    ("Point", Spelling::Bound(&["Node.datum_point"])),
    (
        "Frame",
        Spelling::Bound(&["Node.datum_frame", "Node.sketch_frame"]),
    ),
    (
        "AxisInPlane",
        Spelling::Bound(&["Node.datum_axis_in_plane"]),
    ),
    ("FaceFrame", Spelling::Bound(&["Node.datum_face_frame"])),
];

/// **Where one roster's spellings are looked for in the stub.**
///
/// A verb is bound by being a DECLARED name; an options field is bound
/// by being its door's KEYWORD, which is never a declared name. A
/// check that reaches for the wrong one passes whatever the stub says,
/// and that is not hypothetical: the decay half of this census read
/// declared names for every roster, so the first options entry to be
/// declined would have stayed green forever.
///
/// So the alphabet is **stored on the roster, never chosen at the
/// check**. Every check below asks [`Roster::alphabet`] rather than
/// deciding again, which is what makes a sixth roster with a third
/// alphabet a one-line decision instead of four independent ones.
enum Alphabet {
    /// Declared names — classes and `Class.member` spellings.
    Declared,
    /// The parameter names of one stub `def`.
    Keywords(&'static str),
}

impl Alphabet {
    /// What the stub offers in this alphabet.
    fn of(&self, stub: &Stub) -> BTreeSet<String> {
        match self {
            Alphabet::Declared => stub.names.clone(),
            Alphabet::Keywords(door) => stub.parameters(door),
        }
    }

    /// Where a failure message should say it looked.
    fn site(&self) -> &'static str {
        match self {
            Alphabet::Declared => "pncad.pyi",
            Alphabet::Keywords(door) => door,
        }
    }
}

/// One kernel vocabulary, its alphabet, and one entry per member.
struct Roster {
    /// What the members are, for the failure messages — the kernel
    /// type whose growth this roster is anchored on.
    subject: &'static str,
    /// Where a spelling of this roster lives. See [`Alphabet`].
    alphabet: Alphabet,
    /// One entry per member, in the kernel type's own order.
    entries: Vec<(String, Spelling)>,
}

/// One options struct's roster: its door's keywords are the alphabet.
fn options_roster(
    subject: &'static str,
    door: &'static str,
    fields: Vec<(&'static str, Spelling)>,
) -> Roster {
    Roster {
        subject,
        alphabet: Alphabet::Keywords(door),
        entries: fields
            .into_iter()
            .map(|(field, spelling)| (field.to_owned(), spelling))
            .collect(),
    }
}

/// **The options rosters**, one entry per field of each struct, naming
/// the keyword that sets it.
///
/// Each destructure is the anchor, and none has a `..`: a field the
/// kernel struct gains does not compile until it is dispositioned
/// here — and every one of these doors builds its options with a
/// struct literal that names each field, which breaks in the same
/// commit for the same reason. The two halves are deliberately
/// redundant: the literal is what makes the DOOR decide, the roster
/// is what makes the decision legible and keeps it falsifiable
/// against the stub.
///
/// **What anchors the LIST itself** is
/// [`every_options_type_in_py_is_rostered`]: a type named `*Options`
/// constructed anywhere under `src/py/` and absent from here fails
/// that test. That is the membership rule this census enforces, and
/// it is narrower than "every configuration struct" — see the test.
fn options_doors() -> Vec<Roster> {
    use Spelling::Bound;

    let step = StepOptions::default();
    let StepOptions {
        product_name: _,
        timestamp: _,
        author: _,
        organization: _,
        originating_system: _,
        uncertainty_m: _,
    } = &step;

    let import = ImportOptions::default();
    let ImportOptions {
        eps_in: _,
        declared_contacts: _,
        examine_chart_coherence: _,
    } = &import;

    let ascii = AsciiOptions::default();
    let AsciiOptions { solid_name: _ } = &ascii;

    let binary = BinaryOptions::default();
    let BinaryOptions { header: _ } = &binary;

    let eval = EvalOptions::default();
    let EvalOptions {
        epoch: _,
        parallel: _,
        boolean_sweep: _,
        resolver: _,
        profile_lift: _,
        param_box: _,
        seed: _,
    } = &eval;

    vec![
        options_roster(
            "StepOptions",
            "Evaluation.step_string",
            vec![
                ("product_name", Bound(&["product_name"])),
                ("timestamp", Bound(&["timestamp"])),
                ("author", Bound(&["author"])),
                ("organization", Bound(&["organization"])),
                ("originating_system", Bound(&["originating_system"])),
                // The Rust field is a bare `f64` in metres and says so
                // in its name; the Python keyword takes a `Length`, so
                // the suffix would be a second spelling of what the
                // type already says.
                ("uncertainty_m", Bound(&["uncertainty"])),
            ],
        ),
        options_roster(
            "ImportOptions",
            "import_step",
            vec![
                ("eps_in", Bound(&["eps_in"])),
                // The import-side declaration channel is a list of
                // `ImportContact`, and that element type has no Python
                // spelling: binding the field means minting a value
                // class for the position anchor, which is its own
                // surface work and its own row. Recorded here as
                // ABSENT rather than left to be discovered, and the
                // decay check below is what stops the reason
                // outliving the fact.
                (
                    "declared_contacts",
                    Spelling::NotBound {
                        would_be: &["declared_contacts"],
                        reason: "its element type `ImportContact` has no Python spelling, so \
                                 the keyword would take a list of nothing a caller can build",
                    },
                ),
                // The flag is a `bool` a Python keyword could carry
                // trivially; what it cannot carry is the ANSWER. The
                // field it turns on reports a `topo::CoherenceReport`
                // on `StepImport::Solid`, and that type has no Python
                // spelling, so a bound keyword would set a switch
                // whose result `ImportReport` does not expose — a
                // caller could ask and never read. **So the pair is
                // unbound together**: binding the flag without the
                // report is the shape this census exists to prevent,
                // and the decay check is what stops that reason
                // outliving the fact.
                (
                    "examine_chart_coherence",
                    Spelling::NotBound {
                        would_be: &["examine_chart_coherence"],
                        reason: "the report it produces (`topo::CoherenceReport`) has no Python \
                                 spelling and no field on `ImportReport`, so the keyword would \
                                 set a switch whose answer a caller cannot read",
                    },
                ),
            ],
        ),
        options_roster(
            "AsciiOptions",
            "Mesh.to_stl_ascii",
            vec![("solid_name", Bound(&["solid_name"]))],
        ),
        options_roster(
            "BinaryOptions",
            "Mesh.to_stl_binary",
            vec![("header", Bound(&["header"]))],
        ),
        options_roster(
            "EvalOptions",
            "evaluate",
            vec![
                (
                    "epoch",
                    Spelling::NotBound {
                        would_be: &["epoch"],
                        reason: "minted per run — an evaluation's identity is not a caller's \
                                 choice",
                    },
                ),
                (
                    "parallel",
                    Spelling::NotBound {
                        would_be: &["parallel"],
                        reason: "an ANSWER-PRESERVING runtime switch the kernel documents as \
                                 test-facing (D9's determinism cross-check compares both \
                                 schedules in one run); a performance door would be its own \
                                 unit and its own entry",
                    },
                ),
                (
                    "boolean_sweep",
                    Spelling::NotBound {
                        would_be: &["boolean_sweep"],
                        reason: "answer-preserving in the same way — the BVH differential \
                                 suite pins the two candidate paths bit-identical, so no \
                                 answer is unreachable through it",
                    },
                ),
                ("resolver", Bound(&["resolver"])),
                (
                    "profile_lift",
                    Spelling::NotBound {
                        would_be: &["profile_lift"],
                        reason: "it decides whether profile geometry is elaborated at THIS \
                                 evaluation's parameters instead of the nominal f64 pass's, \
                                 and only a non-nominal `param_box` can show that \
                                 difference — which this door never has. NOT a no-op at f64 \
                                 in general: `editor-core`'s MC lane sets `Guided` at f64 \
                                 precisely because its box is not the nominal one",
                    },
                ),
                (
                    "param_box",
                    Spelling::NotBound {
                        would_be: &["param_box"],
                        reason: "a box IS reachable at f64 and from Python — the MC lane \
                                 evaluates under one and `monte_carlo` is its door — but only \
                                 the DEGENERATE form, the point sample `AxisScalar for f64` \
                                 admits. A box with width needs an evaluation at a scalar \
                                 that carries a bracket, and this door has only f64",
                    },
                ),
                (
                    "seed",
                    Spelling::NotBound {
                        would_be: &["seed"],
                        reason: "the E4 tangent seed needs a scalar that carries a tangent, \
                                 and an f64 evaluation carries none — the box's twin, but \
                                 with no degenerate form that reaches f64 at all",
                    },
                ),
            ],
        ),
    ]
}

// ------------------------------------------------------------------
// The censuses
// ------------------------------------------------------------------

impl Roster {
    /// Spellings this roster claims that its alphabet does not offer.
    fn missing(&self, stub: &Stub) -> Vec<String> {
        let offered = self.alphabet.of(stub);
        let site = self.alphabet.site();
        self.entries
            .iter()
            .filter_map(|(member, spelling)| match spelling {
                Spelling::Bound(names) => Some((member, names)),
                Spelling::NotBound { .. } => None,
            })
            .flat_map(|(member, names)| {
                names
                    .iter()
                    .filter(|n| !offered.contains(**n))
                    .map(move |n| format!("{}::{member} -> {site} offers no {n}", self.subject))
                    .collect::<Vec<_>>()
            })
            .collect()
    }

    /// Entries claiming to be unbound whose spelling the alphabet NOW
    /// offers — someone bound the member and left the reason standing.
    fn stale(&self, stub: &Stub) -> Vec<String> {
        let offered = self.alphabet.of(stub);
        let site = self.alphabet.site();
        self.entries
            .iter()
            .filter_map(|(member, spelling)| match spelling {
                Spelling::NotBound { would_be, reason } => {
                    let found: Vec<&str> = would_be
                        .iter()
                        .copied()
                        .filter(|n| offered.contains(*n))
                        .collect();
                    (!found.is_empty()).then(|| {
                        format!(
                            "{}::{member} is bound at {site} as {found:?}, but says: {reason}",
                            self.subject
                        )
                    })
                }
                Spelling::Bound(_) => None,
            })
            .collect()
    }
}

/// The verb roster, as a [`Roster`]: its alphabet is declared names.
fn verb_roster() -> Roster {
    Roster {
        subject: "Verb",
        alphabet: Alphabet::Declared,
        entries: Verb::ALL
            .iter()
            .map(|verb| (format!("{verb:?}"), verb_spelling(*verb)))
            .collect(),
    }
}

/// A constructor table as a [`Roster`]: its alphabet is declared names.
fn constructor_roster(subject: &'static str, table: &'static [(&str, Spelling)]) -> Roster {
    Roster {
        subject,
        alphabet: Alphabet::Declared,
        entries: table
            .iter()
            .map(|(variant, spelling)| ((*variant).to_owned(), spelling.clone()))
            .collect(),
    }
}

/// The node and datum rosters, each beside the welded identifiers its
/// keys must equal.
fn constructor_rosters() -> Vec<(Roster, &'static [&'static str])> {
    vec![
        (
            constructor_roster("Node", NODE_CONSTRUCTORS),
            NODE_VARIANTS.identifiers(),
        ),
        (
            constructor_roster("Datum", DATUM_CONSTRUCTORS),
            DATUM_VARIANTS.identifiers(),
        ),
    ]
}

/// Every roster in the file, each carrying its own alphabet — read
/// together so a check cannot cover one and quietly skip another.
fn every_roster() -> Vec<Roster> {
    std::iter::once(verb_roster())
        .chain(options_doors())
        .chain(constructor_rosters().into_iter().map(|(roster, _)| roster))
        .collect()
}

/// **The rosters decay.** A member listed as deliberately unbound
/// whose spelling Python NOW offers is a stale entry: someone bound it
/// and left a reason standing that says they did not.
///
/// This is the half that makes [`Spelling::NotBound`] an assertion
/// rather than a comment. **What it does NOT check is the reason** —
/// only the spelling. A reason that was never true, or stopped being
/// true without the member being bound, passes here; that is a
/// reader's job and the reasons are written to be checkable by one.
#[test]
fn the_not_bound_roster_decays() {
    let stub = stub();
    let stale: Vec<String> = every_roster()
        .iter()
        .flat_map(|roster| roster.stale(&stub))
        .collect();
    assert!(
        stale.is_empty(),
        "these members are listed as not bound and Python offers them: {stale:?}"
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
    let unfound: Vec<&str> = options_doors()
        .iter()
        .filter_map(|roster| match roster.alphabet {
            Alphabet::Keywords(door) => (!stub.defs.contains_key(door)).then_some(door),
            Alphabet::Declared => None,
        })
        .collect();
    assert!(
        unfound.is_empty(),
        "these options doors' signatures were not found in the stub, so their keyword \
         censuses read nothing: {unfound:?}"
    );
    let empty: Vec<&str> = every_roster()
        .iter()
        .filter(|roster| roster.entries.is_empty())
        .map(|roster| roster.subject)
        .collect();
    assert!(
        empty.is_empty(),
        "these rosters have no entries, so every check over them passes reading nothing: \
         {empty:?}"
    );
}

/// **The verb census.** Every verb the transition table declares is
/// reachable from Python at every state this roster claims, or is
/// recorded as deliberately unbound.
#[test]
fn every_path_verb_has_a_python_spelling() {
    let stub = stub();
    let missing = verb_roster().missing(&stub);
    assert!(
        missing.is_empty(),
        "the roster claims Python spellings pncad.pyi does not declare: {missing:?}"
    );
}

/// **The node census.** Every kernel `Node` variant, and every datum
/// shape inside `Node::Datum`, is authored by a `Node.*` constructor
/// `pncad.pyi` declares, or is recorded as deliberately unbound.
///
/// The keys are held to the welded identifiers in BOTH directions: a
/// variant with no entry is the gap this census exists for, and an
/// entry naming no variant is a disposition for something the kernel
/// no longer has, which reads as coverage while covering nothing.
#[test]
fn every_node_variant_has_a_python_constructor() {
    let stub = stub();
    for (roster, variants) in constructor_rosters() {
        let keys: Vec<&str> = roster.entries.iter().map(|(k, _)| k.as_str()).collect();
        let keyed: BTreeSet<&str> = keys.iter().copied().collect();
        assert_eq!(
            keyed.len(),
            keys.len(),
            "the {} roster names a variant twice: {keys:?}",
            roster.subject
        );
        let welded: BTreeSet<&str> = variants.iter().copied().collect();
        let unrostered: Vec<&&str> = welded.difference(&keyed).collect();
        let foreign: Vec<&&str> = keyed.difference(&welded).collect();
        assert!(
            unrostered.is_empty() && foreign.is_empty(),
            "the {} roster has moved away from the kernel's variants: no disposition for \
             {unrostered:?}; a disposition for no variant at {foreign:?}",
            roster.subject
        );
        let missing = roster.missing(&stub);
        assert!(
            missing.is_empty(),
            "the roster claims constructors pncad.pyi does not declare: {missing:?}"
        );
    }
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

/// **The target census.** Every target form's token is a class
/// `pncad.pyi` declares AND some signature admits — the mode census's
/// two halves, over the other vocabulary a leg's end is spelled in.
///
/// SOME signature, not each closer's: which verbs take which token is
/// the `ty` fixtures' claim (`tests/ty_fixtures/legal.py`), and the
/// binding's runtime extraction is `tests/test_paths.py`'s.
#[test]
fn every_target_form_has_a_python_spelling() {
    let stub = stub();
    let reach = stub.signature_reach();
    let classes: Vec<&str> = TargetKind::ALL
        .iter()
        .filter_map(|k| target_class(*k))
        .collect();
    assert!(
        !classes.is_empty(),
        "no target form has a class, so this census reads nothing"
    );
    let undeclared: Vec<&str> = classes
        .iter()
        .copied()
        .filter(|c| !stub.declares(c))
        .collect();
    assert!(
        undeclared.is_empty(),
        "pncad.pyi declares no class for these target forms: {undeclared:?}"
    );
    let unreachable: Vec<&str> = classes
        .iter()
        .copied()
        .filter(|c| !mentions(&reach, c))
        .collect();
    assert!(
        unreachable.is_empty(),
        "these target tokens appear in no signature, so no verb admits them: {unreachable:?}"
    );
}

/// **The options census.** Every field of every ROSTERED options
/// struct is a keyword of its door, or recorded as deliberately
/// withheld. Which structs are rostered is
/// [`every_options_type_in_py_is_rostered`]'s question, not this one's.
#[test]
fn every_option_field_reaches_the_python_door() {
    let stub = stub();
    let missing: Vec<String> = options_doors()
        .iter()
        .flat_map(|roster| roster.missing(&stub))
        .collect();
    assert!(
        missing.is_empty(),
        "these Python doors are short of the options struct they configure: {missing:?}"
    );
}

/// Every `*Options` type CONSTRUCTED in `code` — a struct literal or a
/// `::default()` — by its bare name.
///
/// Name-shaped AND construction-shaped, because either alone
/// over-reads: an options type is named in `use` lines and in `&T`
/// argument positions, and neither of those is a door building one.
fn constructed_options_types(code: &str) -> BTreeSet<String> {
    let is_ident = |c: char| c.is_alphanumeric() || c == '_';
    let mut out = BTreeSet::new();
    let mut from = 0;
    while let Some(at) = code[from..].find("Options") {
        let start = from + at;
        let end = start + "Options".len();
        from = start + 1;
        // Only where the occurrence ENDS the identifier, so
        // `OptionsDoor` is not read as `Options`.
        if code[end..].starts_with(is_ident) {
            continue;
        }
        let head = code[..start]
            .char_indices()
            .rev()
            .take_while(|(_, c)| is_ident(*c))
            .last()
            .map_or(start, |(i, _)| i);
        let name = &code[head..end];
        if !name.starts_with(char::is_uppercase) {
            continue;
        }
        let rest = code[end..].trim_start();
        if rest.starts_with("::default()") || rest.starts_with('{') {
            out.insert(name.to_owned());
        }
    }
    out
}

/// **The roster LIST is not hand-kept.** A type named `*Options` that
/// any door under `src/py/` constructs — as a struct literal or
/// through `::default()` — must appear in [`options_doors`].
///
/// Without this, `options_doors`'s membership would be exactly the
/// hand-kept list this module exists to abolish: a sixth struct wired
/// to a new door would red nothing, and every check above would keep
/// passing over the five it happens to name.
///
/// **What it enforces is a NAME rule over construction sites in one
/// directory**, which is narrower than "every configuration struct
/// that reaches Python", and the module header says the narrow thing
/// for that reason. Two configuration structs in this crate are
/// deliberately outside it: `ChecksConfig` and `McConfig` cross as
/// value classes with their own constructors and are named `*Config`.
/// Their anchor is LIB's row
/// `checks-config-door-respells-four-kernel-defaults`. A third such
/// struct arriving under a third name is this rule's stated blind
/// spot rather than a silent one.
#[test]
fn every_options_type_in_py_is_rostered() {
    let dir = test_utils::source::crate_dir(env!("CARGO_MANIFEST_DIR")).join("src/py");
    let files = test_utils::source::rust_sources(&dir);
    assert!(
        !files.is_empty(),
        "no Rust source found under {} — this check was about to pass having read nothing",
        dir.display()
    );
    let mut found: BTreeSet<String> = BTreeSet::new();
    for path in &files {
        let text = std::fs::read_to_string(path).expect("a listed source file reads");
        found.extend(constructed_options_types(&test_utils::source::code_only(
            &text,
        )));
    }
    assert!(
        !found.is_empty(),
        "the scan found no `*Options` construction under src/py, so it read nothing"
    );
    let rostered: BTreeSet<&str> = options_doors().iter().map(|r| r.subject).collect();
    let unrostered: Vec<&String> = found
        .iter()
        .filter(|name| !rostered.contains(name.as_str()))
        .collect();
    assert!(
        unrostered.is_empty(),
        "these `*Options` types are constructed under src/py and are in no roster: \
         {unrostered:?}; the rosters hold {rostered:?}"
    );
}

/// The scan reads construction and not mention. Without this, a scan
/// that matched every `Options` in sight would report a roster hole
/// out of a `use` line, and one that matched none would report none.
#[test]
fn the_options_scan_reads_construction_only() {
    let found = constructed_options_types(
        "use pncad::step_export::StepOptions;\n\
         fn take(options: &StepOptions) {}\n\
         fn build() { let a = stl::AsciiOptions { solid_name: n }; \
         let b = MissingOptions::default(); let c = OptionsDoor { x: 1 }; \
         let d = lower_options {}; }\n",
    );
    assert_eq!(
        found,
        ["AsciiOptions", "MissingOptions"]
            .iter()
            .map(|s| (*s).to_owned())
            .collect::<BTreeSet<String>>(),
        "the scan should read the two CONSTRUCTED names and neither the imported one, \
         the borrowed one, `OptionsDoor` nor a lowercase binding"
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
         def plain(self) -> ReturnedOnly: ...\n    \
         @overload\n    \
         def over(self, a: First) -> None: ...\n    \
         @overload\n    \
         def over(self, b: Later) -> None: ...\n\
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
    assert_eq!(
        stub.parameters("Widget.over"),
        ["self", "a", "b"].iter().map(|s| (*s).to_owned()).collect(),
        "every overload's parameters, not the first's"
    );
    let reach = stub.signature_reach();
    assert!(
        mentions(&reach, "Later"),
        "a LATER overload's types are in the reach"
    );
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
