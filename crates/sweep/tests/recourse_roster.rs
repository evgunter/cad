//! **The roster: every predicate name this crate decides is routed to a
//! recourse sentence, or listed here as carrying none and why.**
//!
//! `BlendError::Escalated`'s `Display` routes a recourse by matching the
//! escalated predicate's NAME. A name the match does not know falls
//! through to `geom_core::MissingRecourse` — the sentence that names the
//! hole — and the fall-through is what these rows are about: it must be
//! reached on purpose, by a name somebody decided owes no blend recourse
//! and wrote down, never by a gate that quietly arrived.
//!
//! The crate decides for five doors, and only the blend's own gates
//! escalate onto `BlendError`; the rest reach the caller through their
//! own door's error type. That is the reason most of [`UNROUTED`]
//! carries, and it is the reason a `fillet3_*` gate added to the battery
//! cannot borrow: a new blend gate is unrouted, unlisted, and red.
//!
//! **The reader is fail-loud about what it cannot read.** A `decide*`
//! call whose first argument is not a plain string literal is recorded
//! as an *indirect site* rather than skipped, and must appear in
//! [`INDIRECT`] with what carries the name there; the carriers named
//! there are scanned for the literals they hold. A name that reaches the
//! funnel as a const or as a struct field is on the roster beside the
//! ones written at the call.
//!
//! Which arm answers a name is MEASURED — each name is put through the
//! door's own error value and the rendered text read — never inferred
//! from the order of the arms in the source.

use geom_core::{Band, Indeterminate, MarginDiag, MissingRecourse, Tol};
use std::collections::BTreeSet;
use sweep::blend::{BlendError, BlendSite};

// ------------------------------------------------------------------ the reader

/// How a carrier holds the names it hands the funnel.
#[derive(Clone, Copy)]
enum Carrier {
    /// The crate's own funnel. It forwards the parameter its callers
    /// supply, and those callers are literal sites in their own right,
    /// so the funnel itself holds no name.
    Funnel,
    /// A call or a struct literal: the names are the plain string
    /// literals inside the bracket group the token opens.
    Call(&'static str),
    /// A `const NAME: &str = "…";` — the name is its initializer.
    Const(&'static str),
}

/// The `decide*` calls whose first argument the reader cannot read at
/// the site, and what carries the name there. Keyed `<file>: <expr>`,
/// which is what [`funnel_calls`] reports.
const INDIRECT: &[(&str, Carrier, &str)] = &[
    (
        "blend/mod.rs: name",
        Carrier::Funnel,
        "the blend module's classification funnel",
    ),
    (
        "blend/surgery.rs: RING_CLEARANCE",
        Carrier::Const("RING_CLEARANCE"),
        "the ring-clearance gate names itself once, as a const, because the surgery both \
         decides it and quotes it back in the refusal's payload",
    ),
    (
        "swept.rs: name",
        Carrier::Funnel,
        "the swept-traversal funnel",
    ),
    (
        "swept.rs: names.lines",
        Carrier::Call("CosurfaceNames"),
        "the cosurface decision reports under the row names its caller's table supplies, \
         so the extrude's side walls and the revolve's walls are separate rows of the K \
         inventory",
    ),
    (
        "swept.rs: names.arcs",
        Carrier::Call("CosurfaceNames"),
        "the arc half of the same table",
    ),
];

/// Every name this crate's `src` decides, and every `decide*` call the
/// reader could not read at the site.
fn funnel_calls() -> (BTreeSet<String>, BTreeSet<String>) {
    let src = test_utils::source::crate_dir(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut names = BTreeSet::new();
    let mut indirect = BTreeSet::new();
    for path in test_utils::source::rust_sources(&src) {
        let text = std::fs::read_to_string(&path).expect("a readable source file");
        // The code-and-literals view blanks comments in place, so prose
        // that spells `decide (` is not a call site and the offsets
        // still index the source byte for byte.
        let code = test_utils::source::code_and_literals(&text);
        let rel = path
            .strip_prefix(&src)
            .expect("a file under the crate's src")
            .to_string_lossy()
            .replace('\\', "/");
        for open in call_sites(&code, "decide", true) {
            let arg = code[open..].trim_start();
            match plain_literal(arg) {
                Some(name) => {
                    names.insert(name.to_string());
                }
                None => {
                    let expr = arg
                        .chars()
                        .take_while(|c| *c != ',' && *c != ')')
                        .collect::<String>()
                        .trim()
                        .to_string();
                    indirect.insert(format!("{rel}: {expr}"));
                }
            }
        }
        for (_, carrier, _) in INDIRECT {
            match carrier {
                Carrier::Funnel => {}
                Carrier::Call(token) => {
                    for open in call_sites(&code, token, false) {
                        names.extend(group_literals(&code, open));
                    }
                }
                Carrier::Const(token) => {
                    names.extend(const_literal(&code, token));
                }
            }
        }
    }
    (names, indirect)
}

/// The byte offsets just past each `token(` or `token {` in `code`.
///
/// `suffixed` admits a token whose name carries a suffix — the funnel's
/// `_flagged` and `_invariant` doors are `decide` calls too.
fn call_sites(code: &str, token: &str, suffixed: bool) -> Vec<usize> {
    let mut out = Vec::new();
    let mut at = 0usize;
    while let Some(hit) = code[at..].find(token) {
        let start = at + hit;
        at = start + token.len();
        if start > 0
            && code[..start]
                .chars()
                .next_back()
                .is_some_and(|c| c == '_' || c.is_ascii_alphanumeric())
        {
            continue;
        }
        let rest = &code[at..];
        let tail = rest.trim_start();
        let skipped = rest.len() - tail.len();
        if suffixed {
            let suffix: String = rest.chars().take_while(|c| *c != '(').collect();
            if !suffix.chars().all(|c| c == '_' || c.is_ascii_alphanumeric()) {
                continue;
            }
            if let Some(i) = rest.find('(') {
                out.push(at + i + 1);
                at += i + 1;
            }
            continue;
        }
        if tail.starts_with('(') || tail.starts_with('{') {
            out.push(at + skipped + 1);
        }
    }
    out
}

/// The plain string literals inside the bracket group whose contents
/// start at `open`.
fn group_literals(code: &str, open: usize) -> Vec<String> {
    let mut out = Vec::new();
    let mut depth = 0usize;
    let bytes = code.as_bytes();
    let mut i = open;
    while i < bytes.len() {
        match bytes[i] {
            b'(' | b'[' | b'{' => depth += 1,
            b')' | b']' | b'}' => {
                if depth == 0 {
                    break;
                }
                depth -= 1;
            }
            b'"' => {
                if let Some(name) = plain_literal(&code[i..]) {
                    out.push(name.to_string());
                    i += name.len() + 2;
                    continue;
                }
            }
            _ => {}
        }
        i += 1;
    }
    out
}

/// The initializer of `const <token>`, when this file declares it.
fn const_literal(code: &str, token: &str) -> Option<String> {
    let decl = code.find(&format!("const {token}"))?;
    let stmt = &code[decl..];
    let end = stmt.find(';')?;
    let open = stmt[..end].find('"')?;
    plain_literal(&stmt[open..]).map(str::to_string)
}

/// The contents of a plain string literal at the head of `text`, if
/// that is what it opens with. A literal carrying an escape is not
/// plain and is not a predicate name.
fn plain_literal(text: &str) -> Option<&str> {
    let rest = text.strip_prefix('"')?;
    let end = rest.find('"')?;
    let body = &rest[..end];
    (!body.contains('\\')).then_some(body)
}

// ------------------------------------------------------------------ the roster

/// Every decided name with no blend recourse, and why it needs none.
///
/// A name here renders the gap sentence at `BlendError::Escalated`: the
/// refusal says the blend table records nothing for it. For all of them
/// the reason is the same shape — the gate belongs to another of this
/// crate's doors, whose refusals are typed on that door's own error and
/// rendered there, so `BlendError` is not on the escalation's path.
const UNROUTED: &[(&str, &str)] = &[
    ("axis_arc_apex", AXIS),
    ("axis_arc_center", AXIS),
    ("axis_arc_clearance", AXIS),
    ("axis_arc_span", AXIS),
    ("axis_line_axial", AXIS),
    ("axis_line_radial", AXIS),
    ("axis_vertex_radius", AXIS),
    ("revolve_axis_direction", AXIS),
    ("revolve_angle", REVOLVE),
    ("revolve_angle_headroom", REVOLVE),
    ("extrusion_normal_component", EXTRUDE),
    ("extrusion_obliquity", EXTRUDE),
    ("loft_stacking", LOFT),
    ("tube_frame_orthogonal", TUBE),
    ("tube_frame_unit", TUBE),
    ("tube_wall", TUBE),
    ("tube_wall_bore", TUBE),
    ("tube_wall_gap", TUBE),
    ("tube_window_headroom", TUBE),
    ("tube_window_span", TUBE),
    ("side_cylinders_cosurface", COSURFACE),
    ("side_planes_cosurface", COSURFACE),
    ("wall_arcs_cosurface", COSURFACE),
    ("wall_lines_cosurface", COSURFACE),
];

const AXIS: &str = "the revolve axis's own classifications: an escalation here is typed on \
                    `RevolveError` and rendered by that door";
const REVOLVE: &str = "the revolve window's classifications, typed on `RevolveError`";
const EXTRUDE: &str = "the extrusion direction's classifications, typed on `ExtrudeError`";
const LOFT: &str = "the loft's section stacking, typed on `LoftError`";
const TUBE: &str = "the tube door's frame, window and wall classifications, typed on \
                    `TubeError` — whose Display names which of the two tube doors a wall \
                    escalation came from";
const COSURFACE: &str = "the swept traversal's cosurface decision, one row name per calling \
                         verb; the escalation is typed on that verb's error";

// ------------------------------------------------------------------ rows

/// An escalation carrying `name`, its margin inside the run's band.
fn escalation(name: &'static str) -> Indeterminate {
    let band = Band::linear(Tol::witness()).expect("the run's band forms");
    Indeterminate {
        margin: MarginDiag::Value((band.zero() + band.escalate()) / 2.0),
        band,
        predicate: Some(name),
    }
}

/// Renders the blend door's refusal for an escalation under `name`.
fn rendered(name: &'static str) -> String {
    BlendError::Escalated {
        site: BlendSite::Chain,
        source: escalation(name),
    }
    .to_string()
}

/// Whether the refusal for `name` is the fall-through — measured by
/// reading the text, not by reading the match.
fn falls_through(name: &'static str) -> bool {
    rendered(name).contains(&MissingRecourse(Some(name)).to_string())
}

/// **Every name the crate decides is routed to a sentence, or listed
/// with the reason it carries none.**
///
/// The two sides are read from different places and compared: the names
/// out of the crate's `src`, the routing out of the rendered refusal.
/// A battery gate added without a sentence is unrouted and unlisted, so
/// it reds here; a gate renamed under a routed arm stops being routed,
/// so it reds here too; and a name struck off `UNROUTED` without a
/// sentence reds as a stale entry.
#[test]
fn every_decided_name_is_routed_or_listed_with_its_reason() {
    let (names, _) = funnel_calls();
    assert!(
        names.contains("fillet3_radius_headroom") && names.contains("tube_wall"),
        "the reader found no funnel calls it should have: {names:?}"
    );
    let listed: BTreeSet<&str> = UNROUTED.iter().map(|(n, _)| *n).collect();
    assert_eq!(
        listed.len(),
        UNROUTED.len(),
        "a name is listed twice in UNROUTED"
    );
    for (name, _) in UNROUTED {
        assert!(
            names.contains(*name),
            "`{name}` is listed as carrying no blend recourse, but nothing in the crate's \
             src decides it — a stale entry"
        );
    }
    for name in &names {
        let name: &'static str = Box::leak(name.clone().into_boxed_str());
        let unrouted = falls_through(name);
        assert_eq!(
            unrouted,
            listed.contains(name),
            "`{name}` is {} by the door and {} in UNROUTED; the refusal reads: {}",
            if unrouted { "unrouted" } else { "routed" },
            if listed.contains(name) {
                "listed"
            } else {
                "unlisted"
            },
            rendered(name)
        );
    }
}

/// **A name no arm carries renders the gap sentence, and says nothing
/// else about the escalation.**
///
/// The same text `profile`'s door renders on ITS unknown name: one
/// sentence, one home, so the two tables cannot drift into two answers
/// to the same question.
#[test]
fn an_unknown_name_names_the_hole() {
    let unknown = "roster_unknown_probe";
    let (names, _) = funnel_calls();
    assert!(
        !names.contains(unknown),
        "the probe name must be one the crate does not decide"
    );
    let text = rendered(unknown);
    assert!(
        text.contains(&MissingRecourse(Some(unknown)).to_string()),
        "the refusal names the hole: {text}"
    );
    assert!(
        text.contains(unknown),
        "the refusal names the predicate that escalated: {text}"
    );
    // A refusal with no predicate at all says so, rather than reading
    // as a name.
    let nameless = BlendError::Escalated {
        site: BlendSite::Chain,
        source: Indeterminate {
            predicate: None,
            ..escalation(unknown)
        },
    }
    .to_string();
    assert!(
        nameless.contains(&MissingRecourse(None).to_string()),
        "an escalation with no predicate name still names the hole: {nameless}"
    );
}

/// **Every `decide*` call the reader cannot read at the site is
/// declared, with what carries the name there.**
///
/// A name built somewhere other than the call — a const, a struct
/// field, a `concat!` — is exactly the name a census misses. The reader
/// records the expression instead of skipping it, so an expression
/// nobody has written down fails here rather than leaving a gate off the
/// roster.
#[test]
fn every_indirect_funnel_call_is_declared() {
    let (_, indirect) = funnel_calls();
    let declared: BTreeSet<String> = INDIRECT.iter().map(|(site, _, _)| site.to_string()).collect();
    assert_eq!(
        indirect, declared,
        "the funnel calls whose name the reader cannot read at the site are not the ones \
         declared in INDIRECT"
    );
    // The const carrier is resolved, not just declared: the gate it
    // names is on the roster.
    let (names, _) = funnel_calls();
    assert!(
        names.contains("fillet3_ring_clearance"),
        "the ring-clearance gate reaches the funnel as a const and must still be rostered"
    );
}
