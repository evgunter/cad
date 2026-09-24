#!/usr/bin/env bash
# certification-doors.sh — who may import the certification doors, and
# what such a file may not also have in scope. ONE home; ci.yml's
# "certification doors: the importers and what they may not reach" step
# and local-scripts/ci-local.sh's discipline row both call this file.
#
# THE SUBJECT. `Interval` is one type in two roles (C9,
# `crates/geom-brep/README.md`): the evaluation scalar, whose `Real`
# surface answers evaluation's questions — `Real::is_poison` is NaI or
# empty, the transcendentals clamp and record the violation in the
# decoration — and the substrate certification arithmetic is built
# from, whose refusal is `!is_certified()`. The certification surface is
# the `Certification` trait in `geom_core::interval::certification`
# (the TRAIT'S HOME below), and a file reaches it only by naming that
# module: the trait is sealed, not re-exported at `geom_core`'s root nor
# beside `Interval`, so no glob of either carries it, and REEXPORT below
# keeps every other file from re-exporting it. What this gate holds is
# the SEPARATION: a production file that names the module is on the
# list below, and a listed file's production code has no `Real` in
# scope — so a value TYPED `Interval` there reaches neither
# `Real::is_poison` (which answers `false` on a `Trv` bracket with real
# endpoints: the certifying branch on a value that does not certify)
# nor a transcendental (which certification does not call). The
# compiler is then what refuses `x.is_poison()` or `x.sqrt()` on an
# `Interval` in a listed file (E0599); the gate is what keeps `Real`
# from being brought into scope to make them compile. Generic code over
# a lane scalar in the same file is another thing, and legitimate:
# KNOWN GAP 6.
#
# THE SCAN IS `crates/*/src`, `lib.sh`'s source set, as for every gate
# on it: `demos/`, `benches/`, `crates/*/examples/` and `crates/*/tests/`
# are not read, so an importer there is not UNLISTED and a re-export
# there is not REEXPORT.
#
# THE KEY IS THE MODULE PATH, NOT THE TRAIT'S NAME, and that is forced:
# `Certification` is also an `EulerOpError` variant (`topo/src/euler.rs`)
# and `certification` a struct field (`editor-core/src/edit.rs`), so a
# token key would read both as importers. What every route to the trait
# spells is the module followed by `::` — the import
# (`…::certification::Certification`), a braced or grouped one
# (`…::certification::{…}`), a glob of the module
# (`…::certification::*`), a fully-qualified call
# (`<Interval as …::certification::Certification>::hull`) — or the
# module renamed (`…::certification as c`), which the key reads too. The
# one route that spells none of these is a re-export, through which a
# third file imports the trait by the re-exporter's path; REEXPORT reds
# every one, so the module path stays the only path. The trait's own
# home defines the trait rather than importing it and is held apart
# (below); `interval.rs` declares the module (`pub mod certification;`,
# no `::`), which is not a route to the doors.
#
# WHAT REDS, each with its own tag so a diagnosis names its rule:
#
#   UNLISTED  a production file names the module and is not on the
#             list — a new certification file is added HERE, in the
#             change that writes it, and its production code is then
#             held to the rules below;
#   REEXPORT  a `pub`-qualified `use` — `pub`, `pub(crate)`,
#             `pub(super)`, `pub(in …)` — whose statement names the
#             module or the trait (`certification` or `Certification` as
#             a word, braced groups and renames included), in ANY
#             production file: listed, unlisted, or the home. A
#             re-export is a second path to the doors that does not
#             spell the module, and a file importing through it is
#             invisible to the key, to this list and to the census. The
#             WORD and not the path, because a re-export may name the
#             trait alone (`pub use self::Certification`) or the module
#             alone (`pub use …::interval::certification;`). A re-export
#             of `EulerOpError::Certification` would cry wolf; none
#             exists. Read over each `;`-terminated statement of the
#             line view rather than the statement view, which cuts a
#             braced group at its `{` and would hand the `pub use` and
#             the name it re-exports to two different records;
#   STALE     a listed file's production code no longer names the
#             module — an entry that exempts nothing and holds nothing,
#             which is the moment to drop it (a listed file that has
#             LEFT the tree is `lib.sh`'s home-gone refusal);
#   REAL      the token `Real` anywhere in a listed file's production
#             code — `use`, alias (`Real as R`), path
#             (`geom_core::Real::sqrt(x)`, `<Interval as Real>::…`),
#             bound (`T: Real`, `impl Real`). Every one of them puts
#             `Real`'s methods within reach of a value typed `Interval`;
#   GLOB      a glob import in a listed file's production code
#             (`use geom_core::*`, `use super::*`, a `*` inside a
#             braced list): a glob carries whatever its source has in
#             scope, `Real` included, without the token appearing;
#   EVALHULL  `enclosure_hull` or `SpanLocate` in a listed file's
#             production code: the evaluation hull, through which a
#             refusal FLOWS at the minimum decoration (a `Trv` bracket
#             stays a bracket with real endpoints), where the
#             certification hull `Certification::hull` refuses it to
#             NaI. One `hull` name per meaning, and a certification file
#             reaches one of them;
#   POISON    `is_poison` in a listed file's production code. It cannot
#             resolve on a value typed `Interval` there (no `Real` in
#             scope), so what it names is a lane `T`'s evaluation poison
#             or a local method of that name — and in certification code
#             either is the wrong question: the refusal is
#             `!is_certified()`;
#   HOME      the trait's home is gone, or no longer declares the trait.
#
# THE TRAIT'S HOME is exempt from REAL and from the key, and from
# nothing else: it defines the doors as delegates of `Real`'s own
# (`<Self as Real>::from_f64`, `Real::powi`), so it has `Real` in scope
# by necessity, and it is the one file that does.
#
# KNOWN GAP 1: PRODUCTION CODE ONLY. A `#[cfg(test)]` item is dropped,
# and so is a module file whose `mod` declaration is one (`lib.sh`'s
# resolver). Test code imports `Real` beside the trait on purpose — a
# `Trv` fixture is built with `sqrt` — and an `is_poison` in a test can
# pass a wrong row but ships nothing. The cost: a certification helper
# written in a test module and later promoted arrives unscanned, and
# the promotion is a diff a human reads.
#
# KNOWN GAP 2: HOLDERS. A file that holds a certification value and
# calls no door — reads `is_certified()` and `lo()`/`hi()` on an
# `Interval` it was handed, with `Real` in scope — never names the
# module, so it is outside the key by construction. Today:
# `geom/src/curves/nurbs.rs`'s `rational_span_bound`;
# `geom-brep/src/ssi.rs`'s `pcurve_windows` and its twin in
# `geom-brep/src/ssi/certify.rs`'s `probe_tube_chart`, each refusing a
# span-window hull by `!is_certified()` with `Real` in scope, where an
# `is_poison` would compile and pass every row today (the hull maps any
# refusal to NaI, on which the two questions agree); and the crossings
# in `probe_tube_chart` (the plane normal) and `geom/src/net.rs`'s
# `ring_coords` (`Interval::from_certified` is inherent, the crossing
# INTO certification arithmetic, and is called from files that hold a
# lane `T: Real` legitimately). That population is
# `geom-core/tests/certified_endpoint_census.rs`'s, which asks the
# different question such a read owes (`is_certified` first).
#
# KNOWN GAP 3: A VALUE HANDED ON. A listed file may pass an `Interval`
# to a helper in an unlisted file with `Real` in scope; the helper is a
# holder (GAP 2) or, if it calls a door, UNLISTED.
#
# KNOWN GAP 4: A ROUTE TO THE DOORS THAT IS NOT A `use`. REEXPORT reads
# `pub … use` statements, and every re-export chain starts at one: a
# private import cannot be re-exported (rustc E0365) nor carried out of
# its module by a glob (E0603). What it does not read: a re-export
# written by a macro (GAP 5) or in test code (GAP 1), and a public
# SUBTRAIT — `pub trait D: Certification {}` with a blanket impl, in a
# listed file — which hands the doors to a generic `T: D` in another
# file without that file naming the module. No file declares one today.
#
# KNOWN GAP 5: MACROS AND `include!`. `lib.sh`'s reader lexes a
# `macro_rules!` body as written and does not follow `include!`
# (its header, "WHAT IT CANNOT DO"); a route to the trait assembled from
# token fragments is invisible.
#
# KNOWN GAP 6: THE LANE ROUTE. `Bounds`, `CertifiedBounds` and `Decide`
# are `Real` subtraits, so generic code over a lane `T` bounded by any
# of them calls every `Real` method — the transcendentals, `is_poison`
# — without the token `Real`, and does so at `T = Interval` too
# (`QuadLane::<Interval>`). That is BY DESIGN: it is lane evaluation
# (`topo/src/props/quad_lane.rs`'s norms, `geom-brep/src/props/quad.rs`,
# `mesh/src/chords.rs`), and its result enters certification arithmetic
# only through `Interval::from_certified`, whose `Def`/`Trv` cap carries
# the lane's own verdict across. THIS GATE READS TOKENS: it cannot tell
# lane evaluation at `T = Interval` from certification arithmetic
# written generically over the same bound, so no rule reads a
# transcendental called on a lane `T` (POISON's token still reads
# `is_poison` on any receiver). What holds the line is the type: a value
# typed `Interval` in a listed file reaches neither (E0599), and
# `from_certified` is the one door from a lane in. The selftest's near
# misses keep a `T: Decide` bound passing for exactly this reason.
#
# THE SIBLING INSTRUMENT is the census,
# `crates/geom-core/tests/certified_endpoint_census.rs`: the same
# population asked a different question (does a certification read ask
# `is_certified` first). Its `the_door_importers_are_the_gate_s_allowlist`
# row reads the list below out of this file and holds the two in step,
# so a file added here without the census knowing, or the other way,
# reds there. It draws the production/test line as `lib.sh` does and
# keys each statement as this gate does, so the two instruments read
# one population by one rule.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

# THE TRAIT'S HOME — the one file that defines the doors, with `Real`
# in scope by necessity.
CERT_HOME=crates/geom-core/src/interval/certification.rs
CERT_HOME_SUBJECT='the Certification trait itself, the one file whose production code may name Real beside the doors'

# THE IMPORTERS — every production file that names the module, and no
# other. One path per line; the census reads this array by its name.
CERT_IMPORTERS_SUBJECT='a certification file, whose production code this gate holds to the rules in its header'
CERT_IMPORTERS=(
  crates/geom-brep/src/offset_fit.rs
  crates/geom-brep/src/offset_meters.rs
  crates/geom-brep/src/patch_bound.rs
  crates/geom-brep/src/props/quad.rs
  crates/geom-brep/src/ssi/enclose.rs
  crates/geom-brep/src/ssi/exhaust.rs
  crates/geom-core/src/spline/algebra.rs
  crates/geom-core/src/spline/compose.rs
  crates/geom-core/src/spline/compose/patch.rs
  crates/geom-core/src/spline/compose/tensor.rs
  crates/geom-core/src/spline/hull.rs
  crates/geom-core/src/spline/net.rs
  crates/geom-core/src/sym/signed.rs
  crates/mesh/src/chords.rs
  crates/topo/src/props/quad_lane.rs
)

# THE MATCHERS, over one statement's text (comments and literals are
# already gone). An identifier boundary is a character that cannot
# continue a Rust identifier, so `Realm`, `RealLike`, `enclosure_hull_of`
# and `is_poisoned` are other names.
CERT_KEY_RE='(^|[^A-Za-z0-9_])certification[[:space:]]*(::|as[[:space:]])'
CERT_REAL_RE='(^|[^A-Za-z0-9_])Real([^A-Za-z0-9_]|$)'
# A glob is a `*` standing as a path element: after `::`, after `,`, or
# alone at the start of a statement (the reader cuts at `{`, so
# `use a::{*, B}` arrives as `use a::` and `*, B`), and followed by `,`
# or the end of the statement. A deref (`*x`) is followed by an operand
# and a product (`a * b`) is preceded by one. The star is a bracket
# class, never `\*`: the patterns reach `awk` through `-v`, which
# processes backslash escapes, and gawk turns `\*` into a bare `*` — a
# quantifier — where mawk keeps it.
CERT_GLOB_RE='(^|::|,)[[:space:]]*[*][[:space:]]*(,|$)'
CERT_EVALHULL_RE='(^|[^A-Za-z0-9_])(enclosure_hull|SpanLocate)([^A-Za-z0-9_]|$)'
CERT_POISON_RE='(^|[^A-Za-z0-9_])is_poison([^A-Za-z0-9_]|$)'
CERT_DECL_RE='(^|[^A-Za-z0-9_])trait[[:space:]]+Certification([^A-Za-z0-9_]|$)'
# REEXPORT's two halves, over one `;`-terminated statement: a
# visibility-qualified `use` (the parenthesis as bracket classes, for
# the `-v` reason above), and the module's or the trait's name as a word.
CERT_PUBUSE_RE='(^|[^A-Za-z0-9_])pub[[:space:]]*([(][^)]*[)])?[[:space:]]*use[[:space:]]'
CERT_NAME_RE='(^|[^A-Za-z0-9_])[Cc]ertification([^A-Za-z0-9_]|$)'

# The rule tags, in the order the diagnoses print. The self-test reads
# the same list to prove each plant fires its own rule and no other.
CERT_RULES=(UNLISTED REEXPORT STALE REAL GLOB EVALHULL POISON HOME)

cert_rule_message() {
  case "$1" in
    UNLISTED) printf '%s' "a production file names geom_core::interval::certification and is not on this gate's importer list — add it to CERT_IMPORTERS (it is then held to every rule in this gate's header), or reach certification arithmetic from a file that is" ;;
    REEXPORT) printf '%s' "a pub-qualified use re-exports the certification doors (it names the certification module or the Certification trait) — a file importing through it names neither, so this gate's key, its importer list and the census cannot see it; import the trait by its own path, use geom_core::interval::certification::Certification, in each file that calls a door" ;;
    STALE) printf '%s' "a listed importer's production code no longer names geom_core::interval::certification — drop its entry in the change that stopped it importing the doors" ;;
    REAL) printf '%s' "a certification file names Real in its production code — a value typed Interval there can then reach Real::is_poison (a silent pass on a Trv bracket) and the transcendentals. Do not name Real here: build brackets through the Certification doors, evaluate on a lane T through the bound it already has (Bounds, CertifiedBounds, Decide), and cross a lane value into certification arithmetic through Interval::from_certified" ;;
    GLOB) printf '%s' "a glob import in a certification file's production code — a glob carries whatever its source has in scope, Real included, without naming it; import by name" ;;
    EVALHULL) printf '%s' "the evaluation hull (enclosure_hull / SpanLocate) in a certification file's production code — a refusal flows through it as a bracket with real endpoints; the certification hull is Certification::hull, which refuses it" ;;
    POISON) printf '%s' "is_poison in a certification file's production code — the certification refusal is !is_certified(), and is_poison is evaluation's weaker question" ;;
    HOME) printf '%s' "$CERT_HOME no longer declares trait Certification — move this gate's home with the trait" ;;
  esac
}

gate() {
  gate_require_crate_sources
  gate_production_sources
  gate_require_homes "$CERT_HOME_SUBJECT" "$CERT_HOME"
  gate_require_homes "$CERT_IMPORTERS_SUBJECT" "${CERT_IMPORTERS[@]}"
  local view
  if ! view=$(gate_rust_code --skip-cfg-test --statements "${GATE_PRODUCTION_FILES[@]}"); then
    gate_error "$(gate_name): the shared Rust reader could not build the statement view, so what it did not match is unknown — that is not a pass"
    exit 1
  fi
  if [ -z "$view" ]; then
    gate_error "$(gate_name): the shared Rust reader returned NOTHING over $GATE_SCAN_FILES production source file(s) — the scan decided nothing, which is not a pass"
    exit 1
  fi
  # THE KEY, through `gate_grep`: it is the population every rule below
  # is about, so a matcher that could not run is diagnosed rather than
  # read as a tree with no importers. The home defines the trait and is
  # not an importer of it.
  local keyed named
  keyed=$(printf '%s\n' "$view" \
    | gate_grep -E "$CERT_KEY_RE" \
    | gate_grep -vE "$(gate_record_anchor "$CERT_HOME")")
  named=$(printf '%s\n' "$keyed" | gate_record_file | sort -u)
  local verdicts status=0
  verdicts=$(printf '%s\n' "$view" | gate_record_awk \
    -v LISTED="$(printf '%s\n' "${CERT_IMPORTERS[@]}")" \
    -v NAMED="$named" \
    -v HOME_FILE="$CERT_HOME" \
    -v KEY_RE="$CERT_KEY_RE" -v REAL_RE="$CERT_REAL_RE" \
    -v GLOB_RE="$CERT_GLOB_RE" -v EVALHULL_RE="$CERT_EVALHULL_RE" \
    -v POISON_RE="$CERT_POISON_RE" -v DECL_RE="$CERT_DECL_RE" '
    BEGIN {
      n = split(LISTED, l, "\n")
      for (i = 1; i <= n; i++) if (l[i] != "") listed[l[i]] = 1
      n = split(NAMED, m, "\n")
      for (i = 1; i <= n; i++) if (m[i] != "") named[m[i]] = 1
    }
    {
      if (!gate_record_split($0)) next
      f = GR_FILE; t = GR_TEXT
      if (f == HOME_FILE) {
        if (t ~ DECL_RE) declared = 1
      } else if ((f in named) && !(f in listed) && t ~ KEY_RE) {
        print "UNLISTED " $0
      }
      if (!(f in listed) && f != HOME_FILE) next
      if (f != HOME_FILE && t ~ REAL_RE) print "REAL " $0
      if (t ~ GLOB_RE) print "GLOB " $0
      if (t ~ EVALHULL_RE) print "EVALHULL " $0
      if (t ~ POISON_RE) print "POISON " $0
    }
    END {
      for (f in listed) if (!(f in named)) print "STALE " f
      if (!declared) print "HOME " HOME_FILE
    }') || status=$?
  [ "$status" -eq 0 ] || gate_reader_died_refusal "the rule walk over the statement view" "$status" \
    "What it did not read is unknown."
  # REEXPORT, over ANY production file. Narrowed first by the raw word —
  # a file that re-exports the doors spells the module or the trait, so
  # one without the word in any byte carries no such statement — then
  # read in the LINE view and cut at `;` only: a `use` statement holds
  # no `;`, so each `pub … use` runs from its visibility to the end of
  # the chunk it sits in, braces and all.
  local reexporters=() lines reexports=""
  mapfile -t reexporters < <(gate_grep -lE "$CERT_NAME_RE" "${GATE_PRODUCTION_FILES[@]}")
  if [ "${#reexporters[@]}" -gt 0 ]; then
    if ! lines=$(gate_rust_code --skip-cfg-test "${reexporters[@]}"); then
      gate_error "$(gate_name): the shared Rust reader could not build the line view of the files naming the certification module or trait, so what they re-export is unknown — that is not a pass"
      exit 1
    fi
    status=0
    reexports=$(printf '%s\n' "$lines" | gate_record_awk \
      -v PUBUSE_RE="$CERT_PUBUSE_RE" -v NAME_RE="$CERT_NAME_RE" '
      {
        if (!gate_record_split($0)) next
        if (GR_FILE != cur) { cur = GR_FILE; buf = "" }
        buf = buf " " GR_TEXT
        while ((k = index(buf, ";")) > 0) {
          chunk = substr(buf, 1, k - 1)
          buf = substr(buf, k + 1)
          if (!match(chunk, PUBUSE_RE)) continue
          s = substr(chunk, RSTART)
          if (s !~ NAME_RE) continue
          gsub(/[ \t]+/, " ", s); sub(/^ /, "", s)
          print "REEXPORT " GR_FILE ":" GR_LINE ": " s
        }
      }') || status=$?
    [ "$status" -eq 0 ] || gate_reader_died_refusal "the re-export walk over the line view" "$status" \
      "What it did not read is unknown."
  fi
  verdicts=$(printf '%s\n%s\n' "$verdicts" "$reexports")
  # ONE DIAGNOSIS PER RULE THAT FIRED, carrying its tag, with the
  # offending records printed above it under the same tag.
  local rule line hit any=false
  for rule in "${CERT_RULES[@]}"; do
    hit=false
    while IFS= read -r line; do
      case "$line" in
        "$rule "*) printf 'CERTIFICATION-DOORS %s: %s\n' "$rule" "${line#"$rule "}"; hit=true ;;
      esac
    done <<< "$verdicts"
    if [ "$hit" = true ]; then
      any=true
      gate_error "$(gate_name) $rule: $(cert_rule_message "$rule")"
    fi
  done
  [ "$any" = false ] || exit 1
  gate_ok "the ${#CERT_IMPORTERS[@]} listed certification files are exactly the production files that name the Certification module, none of them has Real, a glob import, the evaluation hull or is_poison in its production code, and no production file re-exports the doors"
}

# --- SELF-TEST ---------------------------------------------------------
#
# THE CLEAN FIXTURE CARRIES THE HOME AND EVERY LISTED FILE, each in the
# shape the live tree has it: the home defines the trait with `Real` in
# scope, which must pass, and every importer names the module, so the
# clean case reds the moment an entry stops being read (STALE) or the
# home's exemption stops being anchored at the home (REAL).
cert_import_line='use geom_core::interval::certification::Certification;'

gate_plant_clean() {
  gate_plant_clean_sources "$1"
  mkdir -p "$1/${CERT_HOME%/*}"
  {
    printf 'use super::Interval;\n'
    printf 'use crate::real::Real;\n'
    printf 'pub trait Certification: Copy {\n    fn zero() -> Self;\n}\n'
    printf 'impl Certification for Interval {\n'
    printf '    fn zero() -> Self {\n        <Self as Real>::zero()\n    }\n}\n'
  } > "$1/$CERT_HOME"
  local f
  for f in "${CERT_IMPORTERS[@]}"; do
    mkdir -p "$1/${f%/*}"
    {
      printf '%s\n' "$cert_import_line"
      printf 'use geom_core::interval::Interval;\n'
      printf 'pub fn m(x: Interval) -> f64 {\n    x.mag()\n}\n'
    } > "$1/$f"
  done
}

# Each plant writes ONE breach. The listed-file plants append to the
# first entry, which keeps its import, so no other rule has a reason to
# fire; `cert_case_alone` holds them to that.
cert_first_listed() { printf '%s/%s' "$1" "${CERT_IMPORTERS[0]}"; }

plant_unlisted() {
  mkdir -p "$1/crates/planted/src"
  printf '%s\n' "$cert_import_line" > "$1/crates/planted/src/lib.rs"
}
plant_unlisted_module_alias() {
  mkdir -p "$1/crates/planted/src"
  printf 'use geom_core::interval::certification as doors;\n' > "$1/crates/planted/src/lib.rs"
}
plant_unlisted_module_glob() {
  mkdir -p "$1/crates/planted/src"
  printf 'use geom_core::interval::certification::*;\n' > "$1/crates/planted/src/lib.rs"
}
plant_unlisted_braced() {
  mkdir -p "$1/crates/planted/src"
  printf 'use geom_core::interval::{\n    Interval,\n    certification::Certification,\n};\n' \
    > "$1/crates/planted/src/lib.rs"
}
plant_unlisted_qualified_call() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn h(a: Interval) -> Interval {\n    <Interval as geom_core::interval::certification::Certification>::hull(a, a)\n}\n' \
    > "$1/crates/planted/src/lib.rs"
}
# REEXPORT, one plant per visibility, in a listed file (whose own
# private import keeps the key and STALE quiet), in an unlisted one
# (re-exporting the module alone, which the key does not read), and in
# the home (whose key exemption must not reach this rule).
plant_reexport_pub() {
  printf 'pub use geom_core::interval::certification::Certification;\n' >> "$(cert_first_listed "$1")"
}
plant_reexport_crate_renamed() {
  printf 'pub(crate) use geom_core::interval::certification::Certification as Doors;\n' >> "$(cert_first_listed "$1")"
}
plant_reexport_super_braced() {
  printf 'pub(super) use geom_core::interval::{\n    certification::Certification,\n    Interval as I,\n};\n' \
    >> "$(cert_first_listed "$1")"
}
plant_reexport_in_path() {
  printf 'pub(in crate::spline) use self::Certification;\n' >> "$(cert_first_listed "$1")"
}
plant_reexport_module_unlisted() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub use geom_core::interval::certification;\n' > "$1/crates/planted/src/lib.rs"
}
plant_reexport_home() { printf 'pub use self::Certification as Doors;\n' >> "$1/$CERT_HOME"; }
plant_stale() {
  printf 'use geom_core::interval::Interval;\npub fn m(x: Interval) -> Interval {\n    x\n}\n' \
    > "$(cert_first_listed "$1")"
}
plant_real_use() { printf 'use geom_core::Real;\n' >> "$(cert_first_listed "$1")"; }
plant_real_alias() { printf 'use geom_core::Real as R;\n' >> "$(cert_first_listed "$1")"; }
plant_real_grouped() { printf 'use geom_core::{Bounds, Real};\n' >> "$(cert_first_listed "$1")"; }
plant_real_path() {
  printf 'pub fn g(x: Interval) -> Interval {\n    geom_core::Real::sqrt(x)\n}\n' >> "$(cert_first_listed "$1")"
}
plant_real_qualified() {
  printf 'pub fn g(x: Interval) -> Interval {\n    <Interval as geom_core::Real>::sqrt(x)\n}\n' >> "$(cert_first_listed "$1")"
}
plant_real_bound() { printf 'pub fn g<T: Real>(x: T) -> T {\n    x\n}\n' >> "$(cert_first_listed "$1")"; }
plant_real_impl() { printf 'impl Real for Local {}\n' >> "$(cert_first_listed "$1")"; }
plant_real_wrapped_bound() {
  printf 'pub fn g<T>(x: T) -> T\nwhere\n    T: Copy\n        + Real,\n{\n    x\n}\n' >> "$(cert_first_listed "$1")"
}
plant_glob_crate() { printf 'use geom_core::*;\n' >> "$(cert_first_listed "$1")"; }
plant_glob_super() { printf 'use super::*;\n' >> "$(cert_first_listed "$1")"; }
plant_glob_braced() { printf 'use geom_core::{Interval, *};\n' >> "$(cert_first_listed "$1")"; }
plant_glob_braced_first() { printf 'use geom_core::{*, Interval};\n' >> "$(cert_first_listed "$1")"; }
plant_evalhull() {
  printf 'pub fn g(a: Interval) -> Interval {\n    a.enclosure_hull(a)\n}\n' >> "$(cert_first_listed "$1")"
}
plant_spanlocate() { printf 'use geom_core::SpanLocate;\n' >> "$(cert_first_listed "$1")"; }
plant_poison() {
  printf 'pub fn g(x: Local) -> bool {\n    x.is_poison()\n}\n' >> "$(cert_first_listed "$1")"
}
plant_home_undeclared() {
  printf 'use super::Interval;\npub fn zero() -> Interval {\n    Interval::from_bounds(0.0, 0.0)\n}\n' \
    > "$1/$CERT_HOME"
}

# THE NEAR MISSES, in a listed file and in an unlisted one. Every token
# a rule reads appears here in a place that is not code, not production,
# or not that token: prose, a trailing comment, a string literal, longer
# names, a deref and a product, a test module — and, in the unlisted
# file, the two live collisions the key was chosen around
# (`EulerOpError::Certification`, a `certification:` field) plus
# everything a listed file may not do, which an unlisted file may.
plant_near_misses() {
  {
    printf '// Real, use geom_core::*;, x.is_poison(), enclosure_hull, SpanLocate\n'
    printf '/// `T: Real` in a doc comment, and `use super::*`.\n'
    printf 'pub const WHY: &str = "Real is_poison enclosure_hull SpanLocate use super::*";\n'
    printf 'pub fn ok(x: Interval) -> f64 {\n    x.mag() // Real in a trailing comment\n}\n'
    printf 'pub struct Realm;\npub struct RealLike;\n'
    printf 'pub fn g<T: Decide>(x: T) -> T {\n    x\n}\n'
    printf 'pub fn prod(a: f64, b: f64) -> f64 {\n    a * b\n}\n'
    printf 'pub fn deref(p: &f64, q: &f64) -> (f64, f64) {\n    (*p, *q)\n}\n'
    printf 'pub fn enclosure_hull_of(a: f64) -> f64 {\n    a\n}\n'
    printf 'pub fn is_poisoned() -> bool {\n    false\n}\n'
    printf 'pub use geom_core::interval::Interval as Bracket;\n'
    printf 'pub use crate::margin::certification_margin;\n'
    printf '// pub use geom_core::interval::certification::Certification;\n'
    printf 'pub struct Unit;\nuse geom_core::interval::certification::Certification as C;\n'
    printf 'pub fn local() -> Interval {\n    use geom_core::interval::certification::Certification;\n    Interval::zero()\n}\n'
    printf '#[cfg(test)]\npub(crate) use geom_core::interval::certification::Certification as T;\n'
    printf '#[cfg(test)]\nmod tests {\n    use super::*;\n    use geom_core::Real;\n'
    printf '    fn t(x: Interval) -> bool {\n        x.is_poison()\n    }\n}\n'
  } >> "$(cert_first_listed "$1")"
  mkdir -p "$1/crates/planted/src"
  {
    printf 'use geom_core::*;\nuse geom_core::Real;\n'
    printf 'pub mod certification;\n'
    printf 'pub enum EulerOpError {\n    Certification { error: u8 },\n}\n'
    printf 'pub struct Edit {\n    certification: u8,\n}\n'
    printf 'pub fn f(e: &EulerOpError, x: Interval) -> bool {\n'
    printf '    matches!(e, EulerOpError::Certification { .. }) && x.is_poison()\n}\n'
  } > "$1/crates/planted/src/lib.rs"
}

# A breach `lib.sh`'s resolver cases append to the file they place: an
# import of the doors, which is UNLISTED wherever it lands.
plant_breach_at() { printf '%s\n' "$cert_import_line" >> "$1"; }

# cert_case_alone RULE PLANTER — `gate_selftest_case` with the stronger
# claim the header makes: the plant fires ITS rule and NO OTHER. A plant
# that also fired a second rule would pass a case asserting only the
# first, and would go on passing with the first rule deleted.
cert_case_alone() {
  local rule=$1; shift
  local case_name=$* tmp out other
  tmp=$(mktemp -d)
  gate_plant_clean "$tmp"
  "$@" "$tmp"
  if out=$("$0" --root "$tmp" 2>&1); then
    rm -rf "$tmp"
    printf 'SELFTEST FAILED: %s PASSED on a planted violation (%s)\n%s\n' "$(gate_name)" "$case_name" "$out" >&2
    exit 1
  fi
  rm -rf "$tmp"
  gate_selftest_assert_diagnosed "$case_name" "$out"
  case "$out" in
    *"$(gate_name) $rule: "*) ;;
    *) printf 'SELFTEST FAILED: %s fired on (%s) without its %s diagnosis:\n%s\n' "$(gate_name)" "$case_name" "$rule" "$out" >&2
       exit 1 ;;
  esac
  for other in "${CERT_RULES[@]}"; do
    [ "$other" != "$rule" ] || continue
    case "$out" in
      *"$(gate_name) $other: "*|*"CERTIFICATION-DOORS $other: "*)
        printf 'SELFTEST FAILED: %s fired %s on (%s), a plant for %s alone:\n%s\n' "$(gate_name)" "$other" "$case_name" "$rule" "$out" >&2
        exit 1 ;;
    esac
  done
}

gate_selftest() {
  gate_selftest_clean
  # A `grep` that cannot run is the failure this gate cannot see for
  # itself: the key comes back empty, and an empty key is every listed
  # file STALE — a red, but for the wrong reason.
  gate_selftest_without_tool grep "it is grep saying it could not search"
  cert_case_alone UNLISTED plant_unlisted
  cert_case_alone UNLISTED plant_unlisted_module_alias
  cert_case_alone UNLISTED plant_unlisted_module_glob
  cert_case_alone UNLISTED plant_unlisted_braced
  cert_case_alone UNLISTED plant_unlisted_qualified_call
  cert_case_alone REEXPORT plant_reexport_pub
  cert_case_alone REEXPORT plant_reexport_crate_renamed
  cert_case_alone REEXPORT plant_reexport_super_braced
  cert_case_alone REEXPORT plant_reexport_in_path
  cert_case_alone REEXPORT plant_reexport_module_unlisted
  cert_case_alone REEXPORT plant_reexport_home
  cert_case_alone STALE plant_stale
  cert_case_alone REAL plant_real_use
  cert_case_alone REAL plant_real_alias
  cert_case_alone REAL plant_real_grouped
  cert_case_alone REAL plant_real_path
  cert_case_alone REAL plant_real_qualified
  cert_case_alone REAL plant_real_bound
  cert_case_alone REAL plant_real_impl
  cert_case_alone REAL plant_real_wrapped_bound
  cert_case_alone GLOB plant_glob_crate
  cert_case_alone GLOB plant_glob_super
  cert_case_alone GLOB plant_glob_braced
  cert_case_alone GLOB plant_glob_braced_first
  cert_case_alone EVALHULL plant_evalhull
  cert_case_alone EVALHULL plant_spanlocate
  cert_case_alone POISON plant_poison
  cert_case_alone HOME plant_home_undeclared
  # The `T: Decide` bound below MUST pass: a lane bound carries `Real`'s
  # methods without the token, and that is lane evaluation, which a
  # certification file does legitimately (the header's KNOWN GAP 6). A
  # gate that refused the bound would refuse `quad_lane` itself.
  gate_selftest_passes "prose, a trailing comment and a string literal carrying every forbidden token, Realm/RealLike, a T: Decide bound, a product and a deref, enclosure_hull_of and is_poisoned, a pub use naming neither the module nor the trait, a longer name, a commented-out re-export, a private import after a pub item, a function-local import in a pub fn, a cfg(test) re-export, Real and is_poison in a cfg(test) module; and, in an unlisted file, a glob, Real, is_poison, a mod certification declaration, an EulerOpError::Certification variant and a certification: field" \
    plant_near_misses
  gate_selftest_test_module_homes "$(gate_name) UNLISTED: " plant_breach_at
  gate_selftest_homes --narrowed --subject "$CERT_HOME_SUBJECT" "$CERT_HOME" \
    --subject "$CERT_IMPORTERS_SUBJECT" "${CERT_IMPORTERS[@]}"
  printf '%s selftest OK: passes a clean fixture carrying the home (Real in scope, exempt) and every listed importer; fires, each plant on its OWN rule and no other, on an unlisted importer by name, by module alias, by module glob, inside a braced group and through a fully-qualified call; on a re-export at pub, pub(crate) renamed, pub(super) braced over lines and pub(in path), of the module alone from an unlisted file, and from the home; on a listed file that stopped importing; on Real in a listed file as a use, an alias, a grouped import, a path, a qualified path, a bound, an impl and a rustfmt-wrapped bound; on a glob of a crate, of super, and inside a braced list at either end; on enclosure_hull and SpanLocate; on is_poison; and on a home that stopped declaring the trait; passes every near miss the rules read past; places a cfg(test) breach where rustc mounts it; reds on each home gone or out of the scan; and stays RED, with a diagnosis, when grep itself cannot run\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
