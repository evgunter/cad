#!/usr/bin/env bash
# certification-interiors.sh — in a file listed as certification code,
# every generic item names `Certifies`.
# ONE home for the FILE LIST; ci.yml's `lint` job runs every gate in this directory.
#
# THE RULE IS NOT HERE. `crates/geom-core/README.md`, "Reading a bracket
# where nothing certifies", DL5.3: code that certifies is closed to a
# dual by type, on its interior as well as at its doors, and "in a file
# listed as certification code, a generic item without `Certifies` fails
# the gate". The trait is `geom_core::Certifies` (`real.rs`): sealed, no
# methods, no bracket right, not implemented by a dual.
#
# WHY A GATE AND NOT ONLY THE TYPE. The lane types are declared over
# `T: Certifies`, so the compiler already asks the bound of every
# function that names a lane. What it cannot ask is the bound of a
# function that names none: a helper a certification routine calls is
# generic over `T: Decide` or `T: Real` unless someone writes otherwise,
# a dual satisfies both, and nothing reds when a construction pass
# starts calling it. The list below is where that is written down, and
# this gate is what keeps a listed file's next helper from leaving the
# bound off.
#
# WHAT IS READ is `lib.sh`'s shared CODE-ONLY view of each listed file,
# less its `#[cfg(test)]` items (`--skip-cfg-test`), in its STATEMENT
# record shape, so a declaration arrives with its generic list and its
# `where` clause as one record whatever the line breaks. Test code is
# not held: a test instantiates at whatever scalar it is about. The
# listed files are the whole subject, so the tree's test-only mounts are
# not resolved; a listed file that is itself mounted under `cfg(test)`
# would be read as production, which cries wolf and cannot go blind.
#
# WHAT FIRES. A record that declares a generic ITEM — `fn`, `impl`,
# `struct`, `enum`, `union`, `trait` or `type` followed (after the name,
# where there is one) by a `<…>` list holding at least one TYPE
# parameter — and names none of `CERT_BOUND_NAMES` anywhere in the
# record. A lifetime and a `const` parameter are not type parameters, so
# `fn f<'a>` and `impl<const N: usize>` are not generic items here.
#
# THE BOUND IS READ BY NAME, ANYWHERE IN THE RECORD, and that is the
# whole matcher: the generic list or the `where` clause, on the scalar
# or on a wrapper of it. `CERT_BOUND_NAMES` holds `Certifies` and each
# NAME that has it as a supertrait, since `T: CertifiedPolicy` is the
# bound spelled through a name this reader cannot resolve. A name is
# added there in the change that declares the trait, with the
# declaration's path beside it.
#
# KNOWN GAP 1: THE NAME IS NOT TIED TO A PARAMETER. `fn f<T: Decide, U:
# Certifies>` passes with `T` unbounded, and so does a record that names
# `Certifies` only inside an argument's type. Tying the name to each
# type parameter needs to know which parameters are scalars, which is a
# resolver and not a grep: a closure parameter `W: FnMut(..)` is generic
# and is no scalar.
#
# KNOWN GAP 2, AND IT IS THE CRY-WOLF HALF OF GAP 1. A method that
# declares a type parameter of its own inside an `impl<T: Certifies>`
# block fires, because the block's header is another record:
#
#     impl<T: Certifies> Lane<T> {
#         fn sweep<W: FnMut(T)>(self, verify: W) { … }     // fires
#     }
#
# Write the parameter in argument position (`verify: impl FnMut(T)`),
# which declares no generic list, or name the bound again in the
# method's own `where` clause. Nothing here reads scope.
#
# KNOWN GAP 3: A LISTED FILE'S CALLEES ARE NOT FOLLOWED. A listed
# routine that calls a `T: Decide` helper in an UNLISTED file hands a
# dual nothing new (the helper was already open to it), but the helper
# is certification interior by use and this gate does not say so. The
# list is kept by hand, like every file list in this directory.
#
# KNOWN GAP 4: `macro_rules!` AND `include!`. A generic item written by
# a macro has no header in the text, and included text is not read
# (`lib.sh`, the reader's own blind spots).
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

CERT_FILES_SUBJECT='a file listed as certification code, whose generic items this gate holds to the Certifies bound'

# THE FILE LIST. A file is listed when all of its production code
# certifies: nothing in it has a meaning at a tangent-transport scalar.
# A file that holds both a body's data and its certification is not
# listed until the two are apart, since a dual body holds the data.
CERT_FILES=(
  # The spline x analytic-face crossings `NurbsLane::surface_crossings`
  # runs: an isolation in certification arithmetic and its settle.
  crates/geom-brep/src/curve_face/composite.rs
)

# The names that say a scalar certifies: the marker, and each trait
# declared with it as a supertrait.
#   Certifies        crates/geom-core/src/real.rs
#   CertifiedPolicy  crates/topo/src/props.rs
CERT_BOUND_NAMES=(Certifies CertifiedPolicy)

# The reader. One record in, the record out if it declares a generic
# item and names no certifying bound.
gate_interior_reader() {
  local names
  names=$(gate_ere_alternation "${CERT_BOUND_NAMES[@]}")
  CERT_NAMES_RE="(^|[^A-Za-z0-9_])(${names})([^A-Za-z0-9_]|$)" gate_record_awk '
    # The balanced `<…>` list starting at position p of s (which must be
    # `<`), or "" if it does not close in the record. `->` is an arrow
    # and not a closer.
    function generics(s, p,    d, i, n, c) {
      d = 0; n = length(s)
      for (i = p; i <= n; i++) {
        c = substr(s, i, 1)
        if (c == "-" && substr(s, i + 1, 1) == ">") { i++; continue }
        if (c == "<") d++
        else if (c == ">") { d--; if (d == 0) return substr(s, p + 1, i - p - 1) }
      }
      return ""
    }
    # Does the list declare a TYPE parameter: a top-level entry that is
    # neither a lifetime nor a `const` parameter.
    function has_type_param(list,    d, i, n, c, start, entry) {
      d = 0; n = length(list); start = 1
      for (i = 1; i <= n + 1; i++) {
        c = (i <= n) ? substr(list, i, 1) : ","
        if (c == "-" && substr(list, i + 1, 1) == ">") { i++; continue }
        if (c == "<" || c == "(" || c == "[") d++
        else if (c == ">" || c == ")" || c == "]") d--
        else if (c == "," && d == 0) {
          entry = substr(list, start, i - start)
          gsub(/^[ \t]+|[ \t]+$/, "", entry)
          start = i + 1
          if (entry == "") continue
          if (entry ~ /^\x27/) continue
          if (entry ~ /^const[ \t]/) continue
          return 1
        }
      }
      return 0
    }
    {
      if (!gate_record_split($0)) next
      t = GR_TEXT
      generic = 0
      rest = t
      # Every item keyword on the record, since `pub(crate) fn` and an
      # `impl` inside a `const _: () = …` both put text before it.
      while (match(rest, /(^|[^A-Za-z0-9_])(fn|impl|struct|enum|union|trait|type)([^A-Za-z0-9_]|$)/)) {
        kw = substr(rest, RSTART, RLENGTH)
        gsub(/[^A-Za-z]/, "", kw)
        after = substr(rest, RSTART + RLENGTH - 1)
        rest = substr(rest, RSTART + RLENGTH)
        sub(/^[ \t]+/, "", after)
        if (kw != "impl") {
          # The item name, then its list.
          if (!match(after, /^[A-Za-z_][A-Za-z0-9_]*[ \t]*/)) continue
          after = substr(after, RLENGTH + 1)
        }
        if (substr(after, 1, 1) != "<") continue
        list = generics(after, 1)
        if (list != "" && has_type_param(list)) { generic = 1; break }
      }
      if (!generic) next
      if (t ~ ENVIRON["CERT_NAMES_RE"]) next
      print $0
    }
  '
}

gate() {
  gate_require_crate_sources
  gate_require_homes "$CERT_FILES_SUBJECT" "${CERT_FILES[@]}"
  local view
  if ! view=$(gate_rust_code --skip-cfg-test --statements "${CERT_FILES[@]}"); then
    gate_error "$(gate_name): the shared Rust reader could not build the statement view, so what it did not match is unknown — that is not a pass"
    exit 1
  fi
  if [ -z "$view" ]; then
    gate_error "$(gate_name): the shared Rust reader returned NOTHING over the ${#CERT_FILES[@]} listed certification file(s) — the scan decided nothing, which is not a pass"
    exit 1
  fi
  local hits
  hits=$(printf '%s\n' "$view" | gate_interior_reader)
  if [ -n "$hits" ]; then
    printf '%s\n' "$hits"
    gate_error "$(gate_name): a generic item without Certifies in a file listed as certification code (listed above) — a dual can instantiate it. Write the bound that says which scalars the item means something at: T: Certifies (geom-core/src/real.rs; it grants no bracket read and needs no entry anywhere), in the generic list or the where clause. If the item is not certification code, it does not belong in a listed file. See crates/geom-core/README.md, DL5.3"
    exit 1
  fi
  GATE_SCAN_FILES=${#CERT_FILES[@]}
  GATE_SCAN_NOUN='listed certification file'
  gate_ok "every generic item in a listed certification file's production code names Certifies"
}

# The clean fixture carries every listed file, each with a generic item
# that names the bound, so a listed file the reader stops seeing reds
# the clean case.
gate_plant_clean() {
  gate_plant_clean_sources "$1"
  local home
  for home in "${CERT_FILES[@]}"; do
    mkdir -p "$1/${home%/*}"
    printf 'pub fn certified<T: geom_core::Certifies>(_t: T) {}\n' > "$1/$home"
  done
}

# Each planter appends to the FIRST listed file.
plant_into() { printf '%s\n' "$2" >> "$1/${CERT_FILES[0]}"; }

plant_fn_decide() { plant_into "$1" 'fn helper<T: Decide>(_t: T) {}'; }
plant_fn_unbounded() { plant_into "$1" 'pub(super) fn helper<T>(_t: T) {}'; }
plant_impl_real() { plant_into "$1" 'impl<T: Real> Cell<T> { fn at(&self) {} }'; }
plant_struct_unbounded() { plant_into "$1" 'struct Cell<T> { at: T }'; }
plant_enum_real() { plant_into "$1" 'enum Read<T: Real> { Hit(T), Miss }'; }
plant_trait_generic() { plant_into "$1" 'trait Reads<T: Decide> { fn read(&self) -> T; }'; }
plant_type_alias() { plant_into "$1" 'type Control<T> = ([T; 4], [f64; 4]);'; }
plant_rustfmt_where() {
  {
    printf 'fn helper<T>(_t: T)\n'
    printf 'where\n'
    printf '    T: Decide\n'
    printf '        + SpanLocate,\n'
    printf '{\n'
    printf '}\n'
  } >> "$1/${CERT_FILES[0]}"
}
plant_bound_only_in_prose() {
  {
    printf '/// `T: Certifies` is what this owes.\n'
    printf 'fn helper<T: Decide>(_t: T) -> &'"'"'static str { "T: Certifies" } // Certifies\n'
  } >> "$1/${CERT_FILES[0]}"
}
plant_longer_name() { plant_into "$1" 'fn helper<T: Certifiesish>(_t: T) {}'; }
plant_method_generic_in_bounded_impl() {
  plant_into "$1" 'impl<T: Certifies> Lane<T> { fn sweep<W: FnMut(T)>(self, _w: W) {} }'
}

plant_ok_where_clause() {
  {
    printf 'fn helper<T>(_t: T)\n'
    printf 'where\n'
    printf '    T: geom_core::Certifies + Bounds,\n'
    printf '{\n'
    printf '}\n'
  } >> "$1/${CERT_FILES[0]}"
}
plant_ok_alias() { plant_into "$1" 'fn helper<T: CertifiedPolicy + CertifiedBounds>(_t: T) {}'; }
plant_ok_lifetime_and_const() {
  plant_into "$1" 'fn window<'"'"'a>(x: &'"'"'a [f64]) -> &'"'"'a [f64] { x }'
  plant_into "$1" 'fn fixed<const N: usize>(x: [f64; N]) -> [f64; N] { x }'
  plant_into "$1" 'impl Cell<f64> { fn at(&self) {} }'
  plant_into "$1" 'fn plain(x: Vec<f64>) -> Option<f64> { x.first().copied() }'
}
plant_ok_method_in_bounded_impl() {
  plant_into "$1" 'impl<T: Certifies> Lane<T> { fn run(self, _w: impl FnMut(T)) {} }'
}
plant_ok_test_code() {
  {
    printf '#[cfg(test)]\n'
    printf 'mod rows {\n'
    printf '    fn each<T: Decide>(_t: T) {}\n'
    printf '}\n'
  } >> "$1/${CERT_FILES[0]}"
}
plant_ok_unlisted_file() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn helper<T: Decide>(_t: T) {}\n' > "$1/crates/planted/src/lib.rs"
}

gate_selftest() {
  local want="a generic item without Certifies in a file listed as certification code"
  gate_selftest_clean
  gate_selftest_without_tool awk "could not build the statement view"
  gate_selftest_case "$want" plant_fn_decide
  gate_selftest_case "$want" plant_fn_unbounded
  gate_selftest_case "$want" plant_impl_real
  gate_selftest_case "$want" plant_struct_unbounded
  gate_selftest_case "$want" plant_enum_real
  gate_selftest_case "$want" plant_trait_generic
  gate_selftest_case "$want" plant_type_alias
  gate_selftest_case "$want" plant_rustfmt_where
  gate_selftest_case "$want" plant_bound_only_in_prose
  gate_selftest_case "$want" plant_longer_name
  gate_selftest_case "$want" plant_method_generic_in_bounded_impl
  gate_selftest_passes "the bound in a rustfmt-wrapped where clause, beside a bracket door" plant_ok_where_clause
  gate_selftest_passes "the bound through a registered name" plant_ok_alias
  gate_selftest_passes "a lifetime-only list, a const-only list, a concrete impl and a non-generic fn" plant_ok_lifetime_and_const
  gate_selftest_passes "a method with no list of its own in a bounded impl" plant_ok_method_in_bounded_impl
  gate_selftest_passes "a generic item under cfg(test)" plant_ok_test_code
  gate_selftest_passes "an unbounded generic item in a file that is not listed" plant_ok_unlisted_file
  gate_selftest_homes --subject "$CERT_FILES_SUBJECT" "${CERT_FILES[@]}"
  printf '%s selftest OK: passes a clean fixture carrying every listed file; fires on a fn, an impl, a struct, an enum, a trait and a type alias whose generic list holds a type parameter and whose record names no certifying bound, whether the bound is absent, weaker (Decide, Real), spread over a rustfmt where block, present only in prose, or a longer name; fires on a method that declares its own type parameter inside a bounded impl (KNOWN GAP 2); passes the bound in a where clause, through a registered name, lifetime-only and const-only lists, test code and unlisted files\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
