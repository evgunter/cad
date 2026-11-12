//! **The endpoint-read census: every production read of one side of a
//! certification bracket, counted and dispositioned.**
//!
//! # Why this row exists
//!
//! A certification bracket's refusal is its DECORATION (`dec < Def`,
//! asked as `!Interval::is_certified()`), not a NaN pair: a quotient by a
//! divisor not proven away from zero, a negative integer power, and a
//! crossing from a scalar that may not certify all carry **ordinary
//! endpoints**. A consumer that reads one endpoint and compares it
//! therefore takes the certifying branch on a value that does not
//! certify, unless the site asks [`geom_core::Interval::is_certified`] by
//! name — and `Real::is_poison`, which at the interval scalar asks only
//! NaI or empty, is not that question.
//!
//! This row is the sweep, executed: it walks the certification code by
//! one rule and pins, per file, how many production endpoint reads it
//! holds and how many of those sit in a function that asks the
//! refusal. **A read added anywhere in that code moves a number here**,
//! so it cannot land without being dispositioned.
//!
//! # What the two numbers mean
//!
//! * `reads` — production `.lo()` and `.hi()` call sites in a file of
//!   the population below ([`endpoint_reads`]). It is a SITE count, read
//!   off the file's text rather than its lines: two reads on one line
//!   are two, and a read rustfmt reflows onto another line is the same
//!   read, so a formatting change moves nothing
//!   ([`a_reflowed_read_moves_no_count`]).
//! * `asking` — of those, the reads whose innermost enclosing `fn` item
//!   ([`fn_items`], from the keyword to the close of its body) mentions
//!   `is_certified()` anywhere. The refusal being IN the function is what
//!   this can see; that it guards THIS read is the reader's judgement,
//!   at the site.
//!
//! The remainder — `reads − asking` — is dispositioned by hand, and the
//! disposition is named per file in [`ROSTER`] below.
//!
//! # The population
//!
//! Two parts, walked as one:
//!
//! * every `crates/*/src` file whose PRODUCTION code reaches the
//!   certification arithmetic ([`reaches_certification`]): it names the
//!   `geom_core::interval::certification` module — the only route to the
//!   [`Certification`](geom_core::interval::certification::Certification)
//!   doors, which are that trait's and are listed there, not here — or it
//!   crosses into certification (`Interval::from_certified`) or refuses a
//!   bracket (`.is_certified()`). The key is what the code does, read off
//!   the same CODE view the counts are, so a file enters when it starts
//!   building or refusing certification brackets and leaves only when it
//!   stops — never because a comment moved. The module half of the key
//!   is `scripts/gates/certification-doors.sh`'s key, and
//!   [`the_door_importers_are_the_gate_s_allowlist`] holds the two
//!   instruments to one population;
//! * every file in [`HOLDERS`]: the files that hold certification
//!   brackets, by name, whether or not they call a door. A file that
//!   reads a bracket it was handed calls no door, and the door key alone
//!   cannot see it.
//!
//! # Blind spots, stated
//!
//! 1. **Type-blind.** It cannot tell a certification endpoint from a
//!    `Bounds::lo()` on an evaluation scalar, so `ssi/certify.rs`'s
//!    `T: Bounds` reads are counted here and are not this census's
//!    subject at all — `Bounds` is decoration-blind by ratification.
//!    They are in the remainder, named below.
//! 2. **Function-scoped.** A refusal anywhere in the innermost enclosing
//!    `fn` item counts, including one that guards a different value; a
//!    closure is not an item, so a read in one is its function's.
//!    Narrowing it to "before this line" would mis-read the sites that
//!    bind an endpoint and refuse on the next line, which is the
//!    ordinary spelling here.
//! 3. **A certification endpoint handed to a helper taking `f64`** is no
//!    longer an enclosure where the comparison happens, and no text scan
//!    can follow it.
//! 4. **Macros are read as text, not expanded.** A read in a macro
//!    invocation's arguments is counted where it is written; a read in a
//!    `macro_rules!` body is counted once, at the definition, attributed
//!    to whatever item encloses the definition (none, at module level,
//!    so not asking); and no read is attributed per expansion, so a
//!    macro that expands a read into ten functions counts it once.
//! 5. The production/test cut is the GATE'S, `scripts/gates/lib.sh`'s,
//!    so the two instruments cannot disagree on which files are
//!    importers: a file a test-only `mod x;` mounts is dropped whole
//!    ([`test_only_mounts`], rustc's placement, `#[path]` and inline
//!    modules included), and inside a file every item under a test-only
//!    `cfg` — a module, a function, a `use`, an `impl` — is dropped line
//!    for line as the gate's reader drops it ([`production`]), which
//!    shares that reader's one-line blind spot (a production item on the
//!    same line as a test-only one's close is dropped with it; rustfmt
//!    never writes one). The key is read per STATEMENT, as the gate
//!    reads it ([`statements`]). It all runs over `test_utils::source`'s
//!    CODE view — comments and string literals blanked, newlines kept —
//!    so every brace and `;` it counts is a real one, and this row rolls
//!    no lexer of its own (`crates/test-utils/tests/reader_census.rs`).
//! 6. **A new holder is invisible until it is listed.** A file that
//!    comes to hold a certification bracket without calling a door, and
//!    is not in [`HOLDERS`], is outside the population: a read there
//!    happens and nothing here counts it. No text key can find such a
//!    file, because the bracket's type is also the evaluation scalar —
//!    telling the two apart needs name resolution, a compiler, not a
//!    lexer. The module key is textual too, but it is the one route to
//!    the doors: the trait is sealed, reached only through its module,
//!    and re-exported nowhere — the gate's REEXPORT reds a `pub … use`
//!    of it in any production file — so a door call cannot be written
//!    without the module's name somewhere in the file. The routes the
//!    gate states it cannot see (its KNOWN GAPs 4 and 5: a public
//!    subtrait, a macro-written path) this census cannot see either.
//! 7. **A read spelled as a path** — `Bounds::lo(x)`, or `Bounds::lo`
//!    handed on as a function — carries no `.lo()` and is not counted.
//!    The production ones in the population are `Bounds` reads on the
//!    evaluation scalar (`ssi/certify.rs`'s `certify_branch`, the tube
//!    ladder's widest rung; `edge_nurbs.rs`'s per-sample lever, a
//!    refusal's payload) or the type's own body (`interval.rs`'s
//!    `from_certified` and span locator).
//! 8. **A const-generic argument written with braces** (`Foo<{ N + 1 }>`)
//!    in a signature is read as the item's body: the walk's terminator
//!    ([`source::item_body`]) treats `<` as no bracket, so the `{` of
//!    `{ N + 1 }` opens a "body" that is balanced and closes at once.
//!    Nothing reds — no item is unterminated — and the true body that
//!    follows is no item's, so its reads fall to the ENCLOSING item: at
//!    module level that is none, a false "not asking"; but inside an
//!    outer `fn` that asks `is_certified()`, it is a false "asks", the
//!    unsafe direction, and silent. No production item in the population
//!    is spelled so today.
//!
//! # Where it lives, and why here
//!
//! In `geom-core/tests/` rather than beside the consumers, because the
//! subject is one type's doors across six crates and no consumer crate
//! can see the others. `geom-core` owns the type.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use test_utils::source;

// Every crate the population walks — it reads every `crates/*/src` —
// and the gate whose allowlist the cross-read row reads; held to the
// walk by `the_suite_is_gated_to_every_crate_it_walks`.
test_utils::gated_to![
    "crates/bvh/src/",
    "crates/editor-core/src/",
    "crates/geom/src/",
    "crates/geom-brep/src/",
    "crates/geom-core/src/",
    "crates/mesh/src/",
    "crates/pncad/src/",
    "crates/pncad-py/src/",
    "crates/profile/src/",
    "crates/quantity/src/",
    "crates/source-scope/src/",
    "crates/step-export/src/",
    "crates/step-import/src/",
    "crates/stl/src/",
    "crates/sweep/src/",
    "crates/test-utils/src/",
    "crates/topo/src/",
    "crates/verbs/src/",
    "crates/viewer/src/",
    "scripts/gates/certification-doors.sh",
];

/// The inherent doors whose call puts a file in the population besides
/// the module key: the crossing into certification arithmetic and the
/// predicate that refuses a bracket, as they are spelled at a call site
/// in the CODE view. The trait's doors need no entry: a file cannot call
/// one without naming the module ([`names_the_certification_module`]).
const INHERENT_DOORS: &[&str] = &["Interval::from_certified(", ".is_certified()"];

/// The trait's home: it defines the doors rather than importing them.
const TRAIT_HOME: &str = "crates/geom-core/src/interval/certification.rs";

/// The gate that holds the importers, whose allowlist
/// [`the_door_importers_are_the_gate_s_allowlist`] reads.
const GATE: &str = "scripts/gates/certification-doors.sh";

/// Whether a CODE-view statement names the certification module as a
/// path — `certification::`, or `certification as` followed by
/// whitespace, at an identifier boundary, with any whitespace between
/// the word and what follows it. The gate's `CERT_KEY_RE`, over the
/// same statement it reads ([`statements`]). The bare word is also a
/// struct field elsewhere in the tree (`certification: …`), which is why
/// the key is the path and not the word.
fn names_the_certification_module(statement: &str) -> bool {
    let mut rest = statement;
    while let Some(at) = rest.find("certification") {
        let after = rest[at + "certification".len()..].trim_start();
        if source::boundary_before(rest, at)
            && (after.starts_with("::")
                || after
                    .strip_prefix("as")
                    .and_then(|a| a.chars().next())
                    .is_some_and(char::is_whitespace))
        {
            return true;
        }
        rest = &rest[at + "certification".len()..];
    }
    false
}

/// A file's production lines as `lib.sh`'s STATEMENT view has them: cut
/// at every `{`, `}` and `;`, each run of whitespace (newlines included)
/// one space. This is the view the gate keys on, so a path split across
/// lines — `certification` on one, `::Certification` on the next — is
/// one statement to both instruments rather than a key the gate reads
/// and this row does not.
fn statements(lines: &[String]) -> Vec<String> {
    lines
        .join("\n")
        .split(['{', '}', ';'])
        .map(|s| s.split_whitespace().collect::<Vec<_>>().join(" "))
        .filter(|s| !s.is_empty())
        .collect()
}

/// Whether a file's production statements reach certification
/// arithmetic: the module key, or an inherent door.
fn reaches_certification(lines: &[String]) -> bool {
    // Every needle below holds one of these identifiers whole, and no
    // identifier is split across lines, so a file without one cannot
    // match and is not joined into statements at all.
    if !lines
        .iter()
        .any(|l| l.contains("certification") || l.contains("_certified"))
    {
        return false;
    }
    statements(lines).iter().any(|s| {
        names_the_certification_module(s) || INHERENT_DOORS.iter().any(|door| s.contains(door))
    })
}

/// The files that hold certification brackets, walked whether or not
/// their production code calls a door: every `crates/*/src` file that
/// named the certification type when it was a type of its own, plus the
/// two that read its brackets without naming it (the SSI driver's
/// pcurve windows, and the scalar the crossing starts from).
///
/// **A fixed list, not a rule**, and the choice is forced (blind spot 6):
/// deciding whether a file holds certification brackets needs name
/// resolution. What keeps it honest is that each entry's counts are
/// pinned in [`ROSTER`] like every other file's, so a read arriving in
/// one of them reds, and [`every_listed_holder_exists`] reds when an
/// entry names a file that is gone — a list that silently shrinks is
/// the failure a list invites. An entry ending `/` lists a directory,
/// every file under it ([`held`]): the quadrature's lanes, one module
/// split by file.
const HOLDERS: &[&str] = &[
    "crates/geom-brep/src/offset_fit.rs",
    "crates/geom-brep/src/offset_meters.rs",
    "crates/geom-brep/src/pair_section/spline_implicit.rs",
    "crates/geom-brep/src/patch_bound.rs",
    "crates/geom-brep/src/props/mod.rs",
    "crates/geom-brep/src/props/quad/",
    "crates/geom-brep/src/ssi.rs",
    "crates/geom-brep/src/ssi/certify.rs",
    "crates/geom-brep/src/ssi/enclose.rs",
    "crates/geom-brep/src/ssi/exhaust.rs",
    "crates/geom-core/src/interval.rs",
    "crates/geom-core/src/lib.rs",
    "crates/geom-core/src/real.rs",
    "crates/geom-core/src/spline/algebra.rs",
    "crates/geom-core/src/spline/compose.rs",
    "crates/geom-core/src/spline/compose/patch.rs",
    "crates/geom-core/src/spline/compose/tensor.rs",
    "crates/geom-core/src/spline/hull.rs",
    "crates/geom-core/src/spline/net.rs",
    "crates/geom-core/src/sym/signed.rs",
    "crates/geom/src/curves/nurbs.rs",
    "crates/geom/src/curves/second_derivative.rs",
    "crates/geom/src/net.rs",
    "crates/geom/src/surfaces/nurbs.rs",
    "crates/mesh/src/chords.rs",
    "crates/mesh/src/nurbs_cert.rs",
    "crates/step-import/src/recognize_curve.rs",
    "crates/topo/src/props/quad_lane.rs",
];

/// Whether `rel` is one of [`HOLDERS`]: a listed file, or a file under a
/// listed directory (an entry ending `/`), so a file added to a listed
/// directory is walked without a line here.
fn held(rel: &str) -> bool {
    HOLDERS.iter().any(|h| {
        if h.ends_with('/') {
            rel.starts_with(h)
        } else {
            rel == *h
        }
    })
}

/// One entry per file walked with at least one endpoint read — every
/// `crates/*/src` file whose production code reaches certification
/// arithmetic ([`reaches_certification`]), and every file in [`HOLDERS`]:
/// the path, the production endpoint reads, and how many of those sit in
/// a function that asks `is_certified()`.
///
/// The remainder is dispositioned here, one note per file that has one.
/// Nothing checks these notes — they are prose beside a number the walk
/// re-derives, exactly like the ratification citations on
/// `bounds-allowlist.sh`'s file list, and they are here so a reader who
/// reds this row learns what the numbers were for.
const ROSTER: &[(&str, usize, usize, &str)] = &[
    (
        "crates/geom-brep/src/carrier_net.rs",
        4,
        0,
        "all 4 are `outer_ends`' reads, `T: Bounds` reads of an edge interval's evaluation \
         bracket (blind spot 1), not a certification bracket: the interval's outer ends, the one \
         spelling of that rule, and it branches on nothing. Its callers are `carrier_span` (the \
         closed-form certificate span and the general image's schedule), the fitted row's \
         coverage gate and the analytic rung-3 tube's piece, each behind a check 2 that has \
         decided `t1 - t0` through `decide` and so refused an uncertified parameter; a NaN end \
         that reached the fitted gate fails it, and `chart_image`'s domain door refuses one in a \
         span",
    ),
    ("crates/geom-brep/src/curve_face/composite.rs", 12, 12, ""),
    (
        "crates/geom-brep/src/curve_face/spline_face.rs",
        35,
        35,
        "every read sits in a function that refuses by name: `certified_window`, the NURBS \
         carrier's window over a parameter enclosure, the sector reading (`sector`, 19), the cut \
         and speed bounds (`cuts`, `speeds`), the Krawczyk step (`krawczyk_with`) and `isolate`",
    ),
    (
        "crates/geom-brep/src/intersect/section_bounds.rs",
        24,
        24,
        "",
    ),
    (
        "crates/geom-brep/src/krawczyk.rs",
        6,
        4,
        "the remainder is `midpoint`'s 2, which `midpoints` reads row by row: the real matrix \
         whose inverse `Y` preconditions the operator, and any `Y` keeps the test sound, so a \
         midpoint read off an enclosure that does not certify can cost a proof and never make \
         one, and `image` itself refuses an image that does not certify",
    ),
    (
        "crates/geom-brep/src/offset_fit.rs",
        11,
        9,
        "nine reads are in `cell_terms`, which refuses to `None` by name. The remainder is \
         `e_low_witness`'s 2, the `e_low` that `cell_bound` hands `cell_terms`: its only \
         production call is there, after both `e_floors` intervals have asked `is_certified()`",
    ),
    (
        "crates/geom-brep/src/offset_meters.rs",
        14,
        14,
        "the mignitude, the curvature join and `cell_distance` refuse by name: a cell whose \
         partial does not certify is read at distance `0`, the vacuous lower bound",
    ),
    ("crates/geom-brep/src/pair_section.rs", 4, 4, ""),
    ("crates/geom-brep/src/pair_section/ruling.rs", 3, 3, ""),
    (
        "crates/geom-brep/src/pair_section/spline_implicit.rs",
        2,
        0,
        "held by name since SECTARMS #6412 removed its one `is_certified()` (the bracket's), \
         which was all the population key saw. Both reads are the pivot's: `pivot`'s shift, the \
         upper end of the foot's lift over the stored normal's length (the normal is certified, \
         and a quotient by a length not proven away from zero has an end that is not finite, \
         which `band.padded` refuses, `SectionError::Unaccounted`); and `Phi::at`'s `lift`, a \
         sum of products of certified points, no quotient, which certifies, its upper end a \
         sound bound on the pivot's stand-off added to a height",
    ),
    (
        "crates/geom-brep/src/pair_section/spline_implicit/curved.rs",
        3,
        3,
        "",
    ),
    (
        "crates/geom-brep/src/pair_section/spline_implicit/sweep/critical.rs",
        37,
        28,
        "the 28 that ask are `krawczyk`'s (12), the critical-value door's (`decide`, 14) and \
         `node_frame`'s width (2), read only where all three of `S(b)`'s coordinates certify and \
         NaN otherwise, which the listing refuses (`NODE_BOX`). Of the other 9, `grown`'s 4 are \
         its doubling's seed half-widths, every box it keeps proved again by `krawczyk`; \
         `read`'s 2 are the value's ceiling in metres, which that door decides only \
         after it asks `at.value.is_certified()`; `mid`'s 2 are the Krawczyk box's centre, a \
         seed; and `along_line`'s 1, its norm, is minted a fresh point, but the slope's \
         numerator carries the gradient's and the direction's decoration, which its caller asks \
         (`slope.is_certified()`)",
    ),
    ("crates/geom-brep/src/pair_section/winds.rs", 12, 12, ""),
    (
        "crates/geom-brep/src/patch_bound.rs",
        4,
        2,
        "two are the rational arm's quotient readings, each guarded by the `is_certified()` this \
         counts: `certified_sup` reads `hi` and `certified_inf` reads `lo` only after asking by \
         name. The remainder is `centre`'s 2, safe because a centre is a translation choice and \
         no enclosure rests on it: it takes midpoints, contributes `0` for a non-finite one and \
         answers `0` for a non-finite mean, so a refused input still yields a finite centre and \
         the reach about it reports the trouble",
    ),
    (
        "crates/geom-brep/src/pcurve_cache.rs",
        7,
        2,
        "`certified_lower`/`certified_upper`'s 2 read only after asking by name. The remainder is \
         5 schedule reads. `Parts::of`'s 3 are `ByTurn`'s count of a span's parts: its controls' \
         extent from their brackets' ends and their least axis distance's `lo`, compared to cut \
         the span finer. A refused bracket only moves the count (a NaN compares false and cuts \
         finer, up to the most), and each part's hull bounds `|f|` at any count through the \
         asking reads. `limb2_offset_sup`'s 2 are its spindle `mid`: the squared distance of a window end from \
         the axis, read at its bracket midpoint only to choose whether that end's grid is graded \
         toward the apex (`apex_grading`). The grid is only breaks of the carrier's enclosing net, \
         which encloses over any breaks, and every span hull of it reads its ends through the \
         two asking reads above; so a refused bracket moves where the net is cut, never what a \
         span certifies, and a NaN reading compares false and grades nothing",
    ),
    ("crates/geom-brep/src/pcurve_cache/projected.rs", 2, 2, ""),
    ("crates/geom-brep/src/props/bracket.rs", 8, 8, ""),
    (
        "crates/geom-brep/src/props/quad/cells.rs",
        5,
        4,
        "`ratio_hull`'s 4 refuse the bracket they read by name. The remainder is \
         `MeasureCells::build`'s 1, `origin`, a frame choice: the measure quotients' `N = \
         A·(A_u×A_v) + c·M̃` holds for every frame `c`, so no enclosure rests on which point it \
         is",
    ),
    (
        "crates/geom-brep/src/props/quad/net.rs",
        4,
        2,
        "`widen`'s 2 refuse the bracket it reads by name. The remainder is `PatchNet::at`'s 1 and \
         `is_rational`'s 1, which read the weight channel that `PatchNet::new`, its one \
         constructor, fills with `pt` of a weight it has checked finite: a point bracket that \
         certifies, whose `lo` IS the weight",
    ),
    (
        "crates/geom-brep/src/props/quad/projected_edge.rs",
        5,
        5,
        "",
    ),
    (
        "crates/geom-brep/src/props/quad/rational.rs",
        3,
        3,
        "`rational_patch_face`'s 3 are weight reads (`net.rs`'s note) in a function that asks of \
         other values",
    ),
    (
        "crates/geom-brep/src/props/quad/rotational.rs",
        20,
        19,
        "19 read after asking by name. `rotational_cut_face_rounds`' 2 are the ring torus's \
         floor, `(R − r).lo()` and `r.lo()`; `torus_face`'s 14 are its count window, every one past a refusal \
         unless `x` and `A` both certify with `A.lo() > 0`, so `x + n·A` at the finite count `n` \
         certifies too. The remainder is `piece_reach`'s 1, the chart radius's \
         upper end scaled to the length a rational chart term's round-0 piece may span: a \
         schedule (`term_schedule`, whose pieces each ask their own speed's `is_certified()`, \
         feeding the rounds' cost), so a refused radius moves how finely the term is first cut, \
         never what a piece certifies",
    ),
    (
        "crates/geom-brep/src/props/quad/trimmed.rs",
        6,
        3,
        "`norm_lo` (1) and `cuts`' crossing bracket (2) refuse the bracket they read by name. The \
         remainder is `composite_piece`'s 3: the located cell's containment test only picks \
         which cell's polynomial the rule reads, and the rule then integrates at `chord.ell` of \
         the same chord, whose slope's decoration rides into the piece's value (an uncertified \
         slope makes an uncertified piece, and `widen` hands one on unwidened); the column \
         bound's `h` is the width of a hull `ParamRange::certified` has just accepted, a \
         difference of two finite point brackets. Every other read in `props/quad/` goes through \
         `lo_or_refuse`/`certified_hi`/`mid` (`props/bracket.rs`), which are the guarded ones, \
         or asks `is_certified()` of the bracket it reads (`rotational.rs`)",
    ),
    ("crates/geom-brep/src/props/spiric.rs", 13, 13, ""),
    (
        "crates/geom-brep/src/props/taylor.rs",
        16,
        13,
        "the remaining 3 are safe by construction. `trig_range`'s 2, its half-width, read `pt` \
         differences of two f64 ends the function has checked finite and ordered, so no operand \
         can be refused. `rate_bound_over`'s 1, its sum, reads the `hi` of a product of point \
         brackets: `q − p` of two finite nodes, and the rate the caller's own `f64` upper bound, \
         `NaN` where it reads none, which the sum carries. `primitive_each`'s 1 is counted \
         asking, but its `is_certified()` is `start`'s: the read is a piece's length `h`, the \
         `hi` of `q − p`. A node is `NaN` where `b − a` overflows, and then `pt` of it is NaI, \
         so `total` refuses; `h` scales only `removable`, the width a later round removes, \
         whose `NaN` makes `Run::stop`'s floor comparison false. Every other read sits in a function \
         that refuses a refused input by name (`composite_by`, 10; `trig_of`, 2), and the \
         integrands' trig jets read through `trig_of` (`trig_jets`)",
    ),
    (
        "crates/geom-brep/src/props/trig.rs",
        8,
        0,
        "`cos_step`/`sin_step` (2 and 4) and the two half-angle clamps (`trig_over` and \
         `spread_cos`, 1 each) build from `pt` of a finite f64 with nonzero exact divisors, so \
         no operand can leave a domain (argued at each)",
    ),
    (
        "crates/geom-brep/src/ssi.rs",
        12,
        0,
        "all 12 are the two march contexts' slab corners (`march_seeds` and `idealized_trace_r3`, \
         6 each), safe by construction: `SsiDomain::slab` is `Box3::around` of a `Point3<f64>` \
         and an `f64` half-extent, and an `f64`'s refusal IS its NaN — the crossing has no \
         decoration channel to carry a refusal in, so a refused slab reads NaN at both ends",
    ),
    (
        "crates/geom-brep/src/ssi/boundary.rs",
        1,
        0,
        "the one is `sine`'s upper end of the speed times the gradient's supremum, two finite \
         positive points (both refused to `0.0` otherwise), so the product certifies, and the \
         quotient rounds down: a lower bound on the sine",
    ),
    (
        "crates/geom-brep/src/ssi/certify.rs",
        25,
        4,
        "the 4 that ask are the chart tube's span-hull window (`chart_tube_windows` refuses \
         either window hull by name before padding it — a refused hull is NaI, and its NaN ends \
         name no window). The other 21 are `T: Bounds` reads on the evaluation scalar and not \
         certification endpoints at all — blind spot 1: `nurbs_limbs`' 5 are its projection \
         seeds, limb 1's at each sample and the image-miss tell's at a refusing break, a \
         parameter the foot search starts from (structure; the carried distance stays honest \
         whatever the seed), `probe_tube_chart`'s 3 are its reads of the pcurve's tangent, which \
         select a direction (structure, not a bound) and whose norm the probe refuses unless \
         positive finite, and `probe_tube_analytic`'s 2 are its midpoint of the box chain's axis \
         at each end, which selects the axis the end's slice is cut across (structure), \
         `probe_tube_pair`'s 8 are its reads of the two pcurves, their tangents (a direction the \
         probe refuses unless positive finite) and their end states (where an end's slice is \
         cut, structure), and `exact`'s 2 and `analytic_limbs`' 1 are the same kind of \
         evaluation read",
    ),
    (
        "crates/geom-brep/src/ssi/enclose.rs",
        39,
        38,
        "`Box3`'s disjointness, containment, centre, split, reach meet and intersection all \
         refuse by name, and so do the mignitude (`zero_free_lower_bound`, 4) and `a_positive`. \
         The remainder is `chart_transverse_margin`'s upper end of the stretch times the \
         gradient's supremum, two finite positive points (each refused otherwise above it), so \
         the product certifies, and the margin over it rounds down: a lower bound",
    ),
    ("crates/geom-brep/src/ssi/exhaust.rs", 2, 2, ""),
    (
        "crates/geom-brep/src/ssi/four.rs",
        7,
        4,
        "the 4 that ask are `image`'s, the Krawczyk image's preconditioner midpoint and image \
         reads, which refuse a piece unless every image row is certified; its containment and \
         miss tests compare the image's certified sides. Of the 3 that do not ask, \
         `mean_value_excludes`' 2 are its column midpoints, which select an exclusion direction: \
         any fixed direction is sound, and the form built along it is read only through `sign`, \
         which refuses an uncertified enclosure by name. `row_cover`'s 1 reads the graph \
         margin's sign only after `tube_margin` has answered `Some`, which it does only for a \
         margin that certifies",
    ),
    (
        "crates/geom-brep/src/ssi/implicit_chart.rs",
        2,
        0,
        "both are `pass_key`'s, the partner's lift rendered into the key a boundary pass is \
         carried over by. They are an identity, not a bound: `{:e}` renders each end exactly, \
         nothing compares them or takes a branch on them, and a pass carries only between \
         sections whose keys are equal",
    ),
    (
        "crates/geom-brep/src/ssi/one_arc.rs",
        47,
        34,
        "the 34 that ask: the Krawczyk test's 14 centre, image and containment reads, which \
         refuse a piece unless every enclosure is certified; `farthest`'s 5 and `nearest`'s 5, \
         which refuse a box with any uncertified side by name before reading it; and \
         `r3_reaches_end`'s 10, which refuse an uncertified end or box side before reading \
         either. The other 13 do not ask: `one_arc`'s 4 are `T: Bounds` reads of the pcurve's \
         evaluation scalar, the midpoint of a chart point that selects where a line is drawn \
         (blind spot 1); `face_roots`' 2 and `face_walk`'s 7 read the sides of a box its one \
         caller, `one_arc_r3`, met with the slab through `Box3::intersection`, which refuses an \
         uncertified side by name, and of the pieces cut from those finite ends",
    ),
    ("crates/geom-brep/src/ssi/residual.rs", 9, 9, ""),
    ("crates/geom-brep/src/ssi/section.rs", 5, 5, ""),
    (
        "crates/geom-core/src/interval.rs",
        34,
        8,
        "the type's own body. The 8 that ask are the two refusal doors (`certified_bracket`, \
         `sign_within`, 2 each) and the directed helpers `certified_hi`, `div_down`, `div_up` \
         and `add_up` (1 each). The other 26 are not certification reads at all — blind spot 1: \
         they are the evaluation scalar's own implementation reads of the `DInterval` it wraps \
         (the `Bounds` forwarders, `repr_bits`, `copysign`, the kink selectors, the two \
         `select_le_zero` arms, and `is_bounded`'s endpoint finiteness, which only refuses: a \
         mint that reads it refuses an unbounded enclosure and certifies nothing), which test \
         NaI and empty themselves and carry the decoration forward rather than certifying",
    ),
    (
        "crates/geom-core/src/interval/certification.rs",
        20,
        20,
        "the reads that serve a door, each refusing first: `narrowed_to` (the one body of \
         `clamped_to` and `meet`), `between` (the interior-ratio clamp skip, the corner choice, \
         the exact-end and point-source fast paths, and the four corners), `width` and `mag`",
    ),
    ("crates/geom-core/src/spline/algebra.rs", 1, 1, ""),
    ("crates/geom-core/src/spline/blossom.rs", 2, 2, ""),
    ("crates/geom-core/src/spline/compose/tensor.rs", 5, 5, ""),
    (
        "crates/geom-core/src/sym/signed.rs",
        20,
        12,
        "the 8 that do not ask read a bracket `enclose_deep` returned (`read_decision`'s 6, the \
         decision read's halves, and `classify`'s 2, the profiling instrument's denominator), \
         and `enclose_deep` hands back `None` for a refused bracket \
         (`is_certified().then_some`), so every bracket read there certifies by construction",
    ),
    ("crates/geom-core/src/test_support.rs", 2, 2, ""),
    (
        "crates/geom/src/curves/nurbs.rs",
        3,
        3,
        "`restricted` asks only of its hull weight (`h[3]`). Its two knot clip ends are safe \
         whatever the ask: they select a span and are re-admitted, clamped to the span's own \
         knots, by the blossom door (`admit`), which refuses one outside it",
    ),
    ("crates/geom/src/curves/second_derivative.rs", 1, 1, ""),
    (
        "crates/geom/src/net.rs",
        2,
        0,
        "the 2 are `cell_origin`'s, a cell image's middle to `f64`: the point a rational net's \
         derivative over the cell is read about, which certifies nothing and needs not \
         (`rational_slope` refuses unless the weight hull is certainly positive)",
    ),
    (
        "crates/geom/src/surfaces/nurbs.rs",
        10,
        0,
        "none asks by name here: the restriction's weight guard is \
         `spline::algebra::certainly_positive`, the one spelling of it, which asks \
         `is_certified()` there. Eight are `read_rect_net`'s and `read_rect_jet`'s 4 knot reads \
         each: the domain check and the span clip select cells and clip ends, which are minted \
         as fresh point brackets and re-admitted to the span's own knots by the blossom door \
         (`admit`). The other 2 are `restricted_jet`'s cell middle, the point its derivative is \
         read about (`net::cell_origin`), which certifies nothing and needs not: what the jet \
         claims is gated by `rational_slope`'s positive-weight guard, and a control point that \
         does not certify carries its decoration through the restriction into the answer",
    ),
    ("crates/mesh/src/chords.rs", 2, 2, ""),
    (
        "crates/topo/src/clearance.rs",
        53,
        16,
        "the 16 that ask are `poisoned` (2), `iso_bounded_domain`'s knots and joins (12) and \
         `extent_of` (2). Of the other 37, `split_axis`'s 4 and the `coincidence` and \
         `meeting_stations` seeds (2 and 4) are selections; `min_separation`'s admission (2), \
         `pair`'s (2), `witness`'s (2) and `overflow`'s (1) read a separation `poisoned` has \
         passed; `window_of`'s windows (6), `boundary_of`'s span (2) and `row_of`'s parameters \
         (2) read values `certified_lift` crossed, which refuses the face unless every crossing \
         certifies; `support_separation`'s 8 are the choice of direction (each cell's normal and \
         the line between the boxes' centres, read at their middles) and of orientation, and the \
         supports' gap it reads after `poisoned` has passed both, a lower bound along any \
         direction; and `box_dependent`'s 2 store a straddling separation's ends as the \
         refusal's payload, which certifies nothing",
    ),
    (
        "crates/topo/src/injective.rs",
        121,
        40,
        "the 40 that ask are `exactly` (2), `forward` (2), `along` (4), `present_chart` (16), \
         `present_plane` (8) and the two `psi`s (8). `present_chart` asks each clip line where \
         it is placed, so its touch, reach, merge, rectangle-side and apex reads see only \
         certified lines; `present_plane` asks its one-edge `y`, and its other reads are behind \
         the SPAN and CORNER funnels; `Curved::psi`'s shift reads finite parameters. Of the \
         other 81, the seeds and selections (`mid`, `graph_read`'s `inside` and lever), the read \
         behind `exactly` (`Pieces::of`'s `wind`), the parameter reads (`monotone_run`, \
         `narrowed`, `leaves_graph`, `leaves_line`) and `run_on`'s, behind `along`'s and the \
         placement asks, are sound. The rest read the window's carrier and the face's boundary, \
         which `clearance`'s `window_of` admits only where every lifted value certifies; \
         `present_wall`'s wall-image reads and `speeds`' jet bound also read an enclosure \
         computed from them that may clamp, which is \
         `work/clear/injective-reads-past-the-trv-census-decide-on-clamped-enclosures`",
    ),
    (
        "crates/topo/src/props.rs",
        7,
        3,
        "the 3 that ask are `deciding_walk`'s confirmation reads, which refine on a re-derivation \
         that does not certify. The other 4: `far_from`'s 2 read the distances that decide \
         whether a region is carried, a choice of reference point, and the carry it selects is \
         refused by name in `carry_of` unless it certifies; `SignCertificate::interval_volume`'s \
         2, its ends, are a refusal's payload (`outward_volume`, the bracket `Undecided` \
         reports) and certify nothing",
    ),
    ("crates/topo/src/props/quad_lane.rs", 25, 25, ""),
    ("crates/topo/src/test_support_fixtures.rs", 2, 2, ""),
];

/// The repo root: this crate's directory, two levels up.
fn repo_root() -> std::path::PathBuf {
    let crate_dir = source::crate_dir(env!("CARGO_MANIFEST_DIR"));
    crate_dir
        .parent()
        .and_then(std::path::Path::parent)
        .map(std::path::Path::to_path_buf)
        .expect("crates/<name> sits two levels under the repo root")
}

/// Every `crates/*/src` file whose production code reaches certification
/// arithmetic, and every file in [`HOLDERS`], with its production lines. **The shared lexer's CODE view**, not the raw
/// text: a `.lo()` or a door inside a doc comment or a string literal is
/// neither a read nor a call. Blanking keeps every byte in place, but
/// [`production`] then drops the lines of test-only items, so an offset
/// into the joined production lines is an offset into the production
/// text and NOT into the file; a read is pinned by its count, never by
/// its place
/// (`crates/test-utils/tests/reader_census.rs` is the ledger this entry
/// sits in).
fn population(root: &std::path::Path) -> Vec<(String, Vec<String>)> {
    let sources = production_sources(root);
    let mut out = Vec::new();
    for (rel, code) in sources {
        let lines = production(&code);
        if !reaches_certification(&lines) && !held(&rel) {
            continue;
        }
        out.push((rel, lines));
    }
    out.sort();
    out
}

/// Every `crates/*/src` file, repo-relative with `/` separators, with
/// its CODE view — less the files a test-only `mod x;` mounts
/// ([`test_only_mounts`]), which are test code however they read: the
/// production set `scripts/gates/lib.sh`'s `gate_production_sources`
/// hands the gate.
pub(crate) fn production_sources(root: &std::path::Path) -> Vec<(String, String)> {
    let crates = root.join("crates");
    let mut dirs: Vec<std::path::PathBuf> = std::fs::read_dir(&crates)
        .expect("crates/ is readable")
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.join("src").is_dir())
        .collect();
    dirs.sort();
    let mut all = Vec::new();
    for d in dirs {
        for path in source::rust_sources(&d.join("src")) {
            let raw = std::fs::read_to_string(&path).expect("a readable source file");
            let rel = path
                .strip_prefix(root)
                .expect("a walked file lies under the repo root")
                .to_string_lossy()
                .replace('\\', "/");
            let code = source::code_only(&raw);
            all.push((rel, raw, code));
        }
    }
    let mut files = Vec::new();
    let mut dirs = Vec::new();
    for (rel, raw, code) in &all {
        for (target, own) in test_only_mounts(rel, raw, code) {
            if !own {
                files.push(target.clone());
            }
            dirs.push(format!("{}/", target.trim_end_matches(".rs")));
        }
    }
    all.into_iter()
        .filter(|(rel, _, _)| {
            !files.contains(rel) && !dirs.iter().any(|d| rel.starts_with(d.as_str()))
        })
        .map(|(rel, _, code)| (rel, code))
        .collect()
}

/// Whether a CODE-view text carries a TEST-ONLY `cfg` attribute:
/// `lib.sh`'s `GATE_CFG_TEST_RE` less its `GATE_CFG_TEST_NOT_RE`, one
/// predicate in two languages. `test` counts alone or inside an
/// `all(…)` — directly after `cfg(`, or after a `(` or a `,` and any
/// whitespace, and followed by `,` or `)`, within the attribute's first
/// `]` — and an attribute holding `any(` or `not(` anywhere in that
/// span refuses the whole text: `not(test)` marks production code and
/// `any(test, …)` an item that also exists without the test build.
fn is_test_only_cfg(text: &str) -> bool {
    let bodies: Vec<&str> = text
        .match_indices("#[cfg(")
        .map(|(at, open)| {
            let body = &text[at + open.len()..];
            &body[..body.find(']').unwrap_or(body.len())]
        })
        .collect();
    let test_at = |body: &str| {
        body.match_indices("test").any(|(q, _)| {
            let closes = matches!(body[q + 4..].chars().next(), Some(',' | ')'));
            let opens =
                q == 0 || matches!(body[..q].trim_end().chars().next_back(), Some('(' | ','));
            closes && opens
        })
    };
    bodies.iter().any(|b| test_at(b))
        && !bodies
            .iter()
            .any(|b| b.contains("any(") || b.contains("not("))
}

/// The production half of the CODE view, cut the way `lib.sh`'s reader
/// cuts it under `--skip-cfg-test`, line for line: a line carrying a
/// test-only attribute ([`is_test_only_cfg`]) opens a skip at the
/// current brace depth, and the skip ends on the line where the braces
/// it opened close again — or, if no brace opened, on the first line
/// with a `;`. So a `#[cfg(test)]` module, function, `use`, `impl` or
/// `const` is dropped whole, and so is anything that shares its last
/// line (the reader's own property, and its one-line blind spot). Over
/// a blanked view every brace is a real brace.
pub(crate) fn production(code: &str) -> Vec<String> {
    let mut out = Vec::new();
    let (mut depth, mut skip_depth) = (0i64, 0i64);
    let (mut skipping, mut seen_open) = (false, false);
    for line in code.lines() {
        let opens = i64::try_from(line.matches('{').count()).expect("a line's brace count");
        let closes = i64::try_from(line.matches('}').count()).expect("a line's brace count");
        if !skipping && is_test_only_cfg(line) {
            skipping = true;
            seen_open = false;
            skip_depth = depth;
        }
        if skipping {
            seen_open |= opens > 0;
            depth += opens - closes;
            if (seen_open && depth <= skip_depth) || (!seen_open && line.contains(';')) {
                skipping = false;
            }
            continue;
        }
        depth += opens - closes;
        out.push(line.to_string());
    }
    out
}

/// Where each test-only `mod x;` in one file mounts its module, as
/// `lib.sh`'s `gate_test_only_mounts` places it: the target file, and
/// whether that file is the declarer itself (`mod lib;` in `lib.rs`,
/// which excludes only the directory form). Read over the CODE view,
/// cut at `{`, `}` and `;` like the statement view, so the attributes a
/// declaration carries are the text since the last delimiter.
///
/// Rustc's resolution: `mod bar;` in a crate root or a `mod.rs` names
/// the sibling `dir/bar.rs`, in any other `dir/foo.rs` it names
/// `dir/foo/bar.rs`; each enclosing inline `mod y { … }` adds `y/`; and
/// a `#[path = "P"]` names `P`, relative to the declaring file's
/// directory at top level and to the inline module's directory inside
/// one. A declaration under a brace that is not a module is refused
/// loudly, as the gate refuses it: rustc mounts such a module only
/// through `#[path]`, and a mount this row cannot place is a production
/// set it does not know.
fn test_only_mounts(rel: &str, raw: &str, code: &str) -> Vec<(String, bool)> {
    if !code.contains("#[cfg(") {
        return Vec::new();
    }
    let dir = rel.rsplit_once('/').map_or("", |(d, _)| d);
    let file_name = rel.rsplit('/').next().unwrap_or(rel);
    let positional = if matches!(file_name, "mod.rs" | "lib.rs" | "main.rs") {
        dir.to_string()
    } else {
        rel.trim_end_matches(".rs").to_string()
    };
    let mut chain: Vec<Option<String>> = Vec::new();
    let mut pending: Option<String> = None;
    let mut start = 0usize;
    let mut out = Vec::new();
    let b = code.as_bytes();
    let mut i = 0usize;
    while i < b.len() {
        match b[i] {
            b'{' => {
                chain.push(pending.take());
                start = i + 1;
            }
            b'}' => {
                chain.pop();
                pending = None;
                start = i + 1;
            }
            b';' => {
                let stmt = &code[start..i];
                if let Some(name) = pending.take()
                    && is_test_only_cfg(stmt)
                {
                    let Some(names) = chain.iter().cloned().collect::<Option<Vec<String>>>() else {
                        panic!(
                            "{rel} declares a test-only `mod {name};` inside a brace that is not a \
                             module, where rustc mounts a non-inline module only through #[path]; \
                             where it lives was not decided, and neither is this row's production set"
                        );
                    };
                    let inner = names.join("/");
                    let target = if stmt.contains("#[path") {
                        let lits = source::blanked(source::code_and_literals, rel, raw);
                        let payload = path_payload(&lits[start..i]).unwrap_or_else(|| {
                            panic!(
                                "{rel}'s test-only `mod {name};` has a #[path] whose payload is \
                                 not a plain literal"
                            )
                        });
                        let base = if inner.is_empty() {
                            dir.to_string()
                        } else {
                            format!("{positional}/{inner}")
                        };
                        normalized(&format!("{base}/{payload}"))
                    } else if inner.is_empty() {
                        format!("{positional}/{name}.rs")
                    } else {
                        format!("{positional}/{inner}/{name}.rs")
                    };
                    let own = target == rel;
                    out.push((target, own));
                }
                pending = None;
                start = i + 1;
            }
            b'm' if code[i..].starts_with("mod")
                && source::boundary_before(code, i)
                && code[i + 3..].starts_with(char::is_whitespace) =>
            {
                let after = source::skip_ws(code, i + 3);
                let name = source::ident(code, after);
                if !name.is_empty() {
                    pending = Some(name.to_string());
                    i = after + name.len();
                    continue;
                }
            }
            _ => {}
        }
        i += 1;
    }
    out
}

/// The payload of the first `#[path = "…"]` in a literal-keeping slice.
fn path_payload(lits: &str) -> Option<&str> {
    let at = lits.find("#[path")?;
    let rest = lits[at + "#[path".len()..].trim_start().strip_prefix('=')?;
    let end = rest.find(']')?;
    source::plain_string_literal(&rest[..end])
}

/// A `/`-joined path with its `.` and `..` segments taken out, as
/// `lib.sh`'s `gate_norm_path` has it: an exclusion is compared to the
/// walked paths as text, and an unnormalised one excludes nothing.
fn normalized(path: &str) -> String {
    let mut seg: Vec<&str> = Vec::new();
    for s in path.split('/') {
        match s {
            "" | "." => {}
            ".." if seg.last().is_some_and(|l| *l != "..") => {
                seg.pop();
            }
            s => seg.push(s),
        }
    }
    seg.join("/")
}

/// **Every endpoint read in a file's production lines, one per site**:
/// each `.lo()` and each `.hi()` call, with whether the innermost `fn`
/// item enclosing it asks `is_certified()`. Read off the text, not its
/// lines, so a read rustfmt reflows onto another line is the same read
/// and two reads on one line are two. The items are the shared walk's
/// ([`source::fn_items`] over [`source::item_body`]), and an item the
/// walk cannot terminate is an error, not a skip: the reads after it
/// would otherwise be attributed to nothing with no count moving.
fn endpoint_reads(lines: &[String]) -> Result<Vec<(usize, bool)>, String> {
    let text = lines.join("\n");
    let items = source::fn_items(&text)?;
    let mut out = Vec::new();
    for needle in [".lo()", ".hi()"] {
        for (at, _) in text.match_indices(needle) {
            let asks = items
                .iter()
                .filter(|(_, body)| body.contains(&at))
                .min_by_key(|(_, body)| body.len())
                .is_some_and(|(_, body)| text[body.clone()].contains("is_certified()"));
            out.push((at, asks));
        }
    }
    out.sort_unstable();
    Ok(out)
}

#[test]
fn every_production_endpoint_read_is_counted_and_dispositioned() {
    let root = repo_root();
    let mut found: Vec<(String, usize, usize)> = Vec::new();
    for (name, lines) in population(&root) {
        let reads = endpoint_reads(&lines)
            .unwrap_or_else(|why| panic!("{name}: the census cannot place a read: {why}"));
        if reads.is_empty() {
            continue;
        }
        let asking = reads.iter().filter(|(_, asks)| *asks).count();
        found.push((name, reads.len(), asking));
    }
    let pinned: Vec<(String, usize, usize)> = ROSTER
        .iter()
        .map(|(p, reads, asking, _)| ((*p).to_string(), *reads, *asking))
        .collect();
    assert_eq!(
        found, pinned,
        "the certification endpoint-read census moved. This is not a number to \
         silence. A certification bracket's refusal is its DECORATION, so a bracket \
         that may not certify carries ordinary endpoints and a one-sided read of one \
         takes the certifying branch: a new read has to ask `is_certified()` by name, \
         or be argued safe at the site and its file's remainder note above extended. \
         A read that went AWAY needs the same note shortened. The classes and the \
         sweep's blind spots are in this file's docs."
    );
    // The census is about a population, so it says how big that
    // population is: a walk that found nothing would satisfy every
    // per-file row above by finding no rows at all.
    let total: usize = found.iter().map(|(_, reads, _)| reads).sum();
    assert!(
        total > 60,
        "the walk found only {total} endpoint reads across {} files — it is reading the \
         wrong tree",
        found.len()
    );
}

/// **A reflow moves no count**: the same reads spelled on one line and
/// split across several by rustfmt are the same sites with the same
/// answers, and two reads on one line are two.
#[test]
fn a_reflowed_read_moves_no_count() {
    let lines = |text: &str| text.lines().map(str::to_string).collect::<Vec<_>>();
    let one = "fn f(x: Interval) -> f64 {\n    if !x.is_certified() { return 0.0; }\n    \
               (x.lo(), x.hi()).0\n}\nfn g(y: Interval) -> f64 { y.lo() }\n";
    let split = "fn f(x: Interval) -> f64 {\n    if !x.is_certified() {\n        return 0.0;\n    \
                 }\n    (\n        x.lo(),\n        x.hi(),\n    )\n        .0\n}\n\
                 fn g(y: Interval) -> f64 {\n    y.lo()\n}\n";
    let asks = |text: &str| -> Vec<bool> {
        endpoint_reads(&lines(text))
            .expect("a well-formed fixture")
            .into_iter()
            .map(|(_, asks)| asks)
            .collect()
    };
    assert_eq!(asks(one), [true, true, false]);
    assert_eq!(asks(split), asks(one));
    // A `;` inside a bracketed type in the signature ends no item.
    let typed = "fn h(m: [[Interval; 3]; 3]) -> [f64; 2] {\n    \
                 if !m[0][0].is_certified() { return [0.0; 2]; }\n    [m[0][0].lo(), m[0][0].hi()]\n}\n";
    assert_eq!(asks(typed), [true, true]);
    // A nested `fn` is its own item: the outer one's refusal is not its.
    let nested = "fn outer(x: Interval) -> f64 {\n    fn inner(y: Interval) -> f64 { y.lo() }\n    \
                  if !x.is_certified() { return 0.0; }\n    inner(x) + x.hi()\n}\n";
    assert_eq!(asks(nested), [false, true]);
    // A trait's declaration ends at its `;` and takes no body: the
    // default methods after it are their own items.
    let defaults = "trait T {\n    fn bare(&self) -> Interval;\n    \
                    fn asks(&self) -> f64 {\n        let v = self.bare();\n        \
                    if !v.is_certified() { return 0.0; }\n        v.lo()\n    }\n    \
                    fn plain(&self) -> f64 { self.bare().hi() }\n}\n";
    assert_eq!(asks(defaults), [true, false]);
    // A `where` clause stands between the signature and the body.
    let bounded = "fn w<T>(x: T) -> f64\nwhere\n    T: Into<Interval>,\n{\n    \
                   let v: Interval = x.into();\n    \
                   if !v.is_certified() { return 0.0; }\n    v.lo()\n}\n";
    assert_eq!(asks(bounded), [true]);
    // An item that never closes is an error, not a skip.
    assert!(endpoint_reads(&lines("fn broken(x: Interval) {\n    x.lo()\n")).is_err());
}

/// [`HOLDERS`] is a list kept by hand, so it is held to the tree: an
/// entry whose file was renamed or deleted would drop out of the walk
/// with no count moving, and the population would shrink unannounced.
#[test]
fn every_listed_holder_exists() {
    let root = repo_root();
    let gone: Vec<&str> = HOLDERS
        .iter()
        .copied()
        .filter(|p| {
            let at = root.join(p);
            if p.ends_with('/') {
                !at.is_dir()
            } else {
                !at.is_file()
            }
        })
        .collect();
    assert!(
        gone.is_empty(),
        "{gone:?} in HOLDERS no longer exist. A renamed file takes its entry with it, \
         under its new path; a deleted one leaves the list, and the certification \
         brackets it held are either gone or now live somewhere that has to be listed"
    );
}

/// The census and the gate are two instruments over one population: the
/// files whose production code names the certification module are
/// exactly `scripts/gates/certification-doors.sh`'s `CERT_IMPORTERS`.
/// Read out of the gate's own text, so neither list can gain or lose a
/// file without the other — and read loudly: an array that parses to
/// nothing is a broken reader, not an empty population.
#[test]
fn the_door_importers_are_the_gate_s_allowlist() {
    let root = repo_root();
    let gate = std::fs::read_to_string(root.join(GATE)).expect("the gate is readable");
    let allowlist = gate_importers(&gate);
    assert!(
        allowlist.len() >= 10,
        "read {} entries out of {GATE}'s CERT_IMPORTERS array — the array moved or its \
         reader broke, and a cross-read over too few entries proves nothing",
        allowlist.len()
    );
    let importers = importers(&root);
    assert!(
        importers.len() >= 10,
        "the walk found {} files naming the certification module — it is reading the \
         wrong tree",
        importers.len()
    );
    assert_eq!(
        importers, allowlist,
        "the files whose production code names geom_core::interval::certification (left) \
         are not {GATE}'s CERT_IMPORTERS (right). The two are one population: a file that \
         imports the certification doors is added to the gate's list in the change that \
         makes it import them, and leaves it in the change that stops"
    );
}

/// This file, whose `gated_to!` [`the_suite_is_gated_to_every_crate_it_walks`]
/// reads.
const SELF: &str = "crates/geom-core/tests/certified_endpoint_census.rs";

/// The `gated_to!` at the top of this file is a list kept by hand, and
/// the change filter skips this suite on a pull request that touches
/// nothing it names. The walk reads EVERY `crates/*/src`, so a crate
/// added to the tree is walked from the day it lands — and, were the
/// list trusted, not gated: a certification read written in it would
/// wait for the nightly. So the list is held to the walk: exactly one
/// `crates/<name>/src/` per crate directory the walk reads, plus the
/// gate the cross-read row reads.
#[test]
fn the_suite_is_gated_to_every_crate_it_walks() {
    let root = repo_root();
    let text =
        std::fs::read_to_string(root.join(SELF)).expect("this suite's own source is readable");
    let code = source::blanked(source::code_only, SELF, &text);
    let lits = source::blanked(source::code_and_literals, SELF, &text);
    let marks: Vec<usize> = code.match_indices("gated_to!").map(|(at, _)| at).collect();
    assert_eq!(
        marks.len(),
        1,
        "{SELF} carries {} gated_to! invocations, not one",
        marks.len()
    );
    let open = source::skip_ws(&code, marks[0] + "gated_to!".len());
    let end = source::balanced_end(&code, open).expect("the gated_to! invocation closes");
    let mut named: Vec<String> = source::top_level_split(&code[open + 1..end], ',')
        .into_iter()
        .map(|r| {
            lits[open + 1 + r.start..open + 1 + r.end]
                .trim()
                .to_string()
        })
        .filter(|s| !s.is_empty())
        .map(|s| {
            source::plain_string_literal(&s)
                .unwrap_or_else(|| panic!("a gated_to! entry that is not a plain literal: {s}"))
                .to_string()
        })
        .collect();
    named.sort();
    let mut walked: Vec<String> = std::fs::read_dir(root.join("crates"))
        .expect("crates/ is readable")
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.join("src").is_dir())
        .map(|p| {
            format!(
                "crates/{}/src/",
                p.file_name()
                    .expect("a crate directory has a name")
                    .to_string_lossy()
            )
        })
        .chain(std::iter::once(GATE.to_string()))
        .collect();
    walked.sort();
    assert!(
        walked.len() > 10,
        "the walk found {} crates — it is reading the wrong tree",
        walked.len()
    );
    assert_eq!(
        named, walked,
        "{SELF}'s gated_to! list, on the left, is not every crates/*/src this census walks plus {GATE} \
         (on the right). A crate the walk reads and the list does not name is walked on every run \
         and gated on none that touches it: name it in the list"
    );
}

/// `CERT_IMPORTERS=( … )`'s entries, sorted, from the gate's text, read
/// as bash reads an array body: each line's words up to the first one
/// that opens a `# comment`, with a quoted word's quotes taken off — so
/// an entry carrying a trailing note, or quoted, is the path bash sees
/// and not a false red. An unterminated array is a reader failure, not
/// a short list.
fn gate_importers(gate: &str) -> Vec<String> {
    let start = gate
        .lines()
        .position(|l| l.trim_start().starts_with("CERT_IMPORTERS=("))
        .expect("the gate declares CERT_IMPORTERS=(");
    let mut out = Vec::new();
    for line in gate.lines().skip(start + 1) {
        if line.trim() == ")" {
            out.sort();
            return out;
        }
        out.extend(
            line.split_whitespace()
                .take_while(|w| !w.starts_with('#'))
                .map(|w| w.trim_matches(|c| c == '\'' || c == '"').to_string()),
        );
    }
    panic!("{GATE}'s CERT_IMPORTERS array is never closed");
}

/// Every `crates/*/src` file whose production code names the
/// certification module, less the trait's home, sorted.
fn importers(root: &std::path::Path) -> Vec<String> {
    let mut out: Vec<String> = population(root)
        .into_iter()
        .filter(|(name, lines)| {
            name != TRAIT_HOME
                && statements(lines)
                    .iter()
                    .any(|s| names_the_certification_module(s))
        })
        .map(|(name, _)| name)
        .collect();
    out.sort();
    out
}
