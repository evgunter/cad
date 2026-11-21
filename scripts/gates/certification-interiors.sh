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
# WHAT FIRES. A record that declares a generic ITEM and names none of
# `CERT_BOUND_NAMES` anywhere in the record. A generic item is either
#
#   a `fn`, `impl`, `struct`, `enum`, `union`, `trait` or `type`
#   followed (after the name, where there is one) by a `<…>` list
#   holding at least one TYPE parameter (a lifetime and a `const`
#   parameter are not type parameters, so `fn f<'a>` and
#   `impl<const N: usize>` are not generic items here); or
#
#   a `fn` whose signature writes `impl Trait` as a type, in argument
#   position or inside an argument's type (`_t: impl Decide`,
#   `&NurbsCurve3<impl Real>`): an anonymous type parameter. A callback
#   or a sequence (`impl Fn(..)`, `impl FnMut(..)`, `impl FnOnce(..)`,
#   `impl Iterator<..>` and its kin; the reader's `NOT_A_SCALAR`) is
#   not counted: it is no scalar, and it is how a method inside a
#   bounded `impl` takes one without a list of its own. Any other
#   trait counts, so a new one cries wolf until it is added there.
#
# THE BOUND IS READ BY NAME, ANYWHERE IN THE RECORD, and that is the
# whole matcher: the generic list or the `where` clause, on the scalar
# or on a wrapper of it. `CERT_BOUND_NAMES` holds every name that keeps
# a dual out: `Certifies`; each NAME that has it as a supertrait
# (`CertifiedPolicy`), since that is the bound spelled through a name
# this reader cannot resolve; and the two certifying bracket doors,
# `CertifiedBounds` and `CertifiedEnclosure`, which no dual implements.
# So a sole `T: CertifiedBounds` passes here as it is: it is not made
# to name `Certifies`, which would turn it into the compound
# `bounds-allowlist.sh` reads for. A name is added there in the change
# that declares the trait, with the declaration's path beside it.
#
# TWO LISTS. `CERT_FILES` holds a file whose every generic item is held.
# `CERT_ITEMS` holds one function of a file that cannot be listed whole
# (it also declares data a dual body holds, or passes a dual runs): the
# `fn` of that name in that file is held, and an entry whose function is
# no longer declared there is a red.
#
# KNOWN GAP 1: THE NAME IS NOT TIED TO A PARAMETER. `fn f<T: Decide, U:
# Certifies>` passes with `T` unbounded, and so does a record that names
# a certifying bound only inside an argument's or the return's type.
# Tying the name to each type parameter needs to know which parameters
# are scalars, which is a resolver and not a grep.
#
# KNOWN GAP 2, AND IT IS THE CRY-WOLF HALF OF GAP 1. A method that
# declares a type parameter of its own inside an `impl<T: Certifies>`
# block fires, because the block's header is another record:
#
#     impl<T: Certifies> Lane<T> {
#         fn sweep<W: FnMut(T)>(self, verify: W) { … }     // fires
#     }
#
# Write a callback as `verify: impl FnMut(T)`, or name the bound again
# in the method's own `where` clause. Nothing here reads scope.
#
# KNOWN GAP 3: A LISTED FILE'S CALLEES ARE NOT FOLLOWED, and an item at
# a concrete non-certifying scalar is not read (`fn d(_t: Dual64)`,
# `impl Cell<Dual64>`): the reader sees generic items only.
#
# KNOWN GAP 4: `include!`. Included text is not read (`lib.sh`, the
# reader's own blind spots). A generic item written literally in a
# `macro_rules!` body is read as written.
#
# KNOWN GAP 5: A `where` CLAUSE AFTER A `[T; N]` TYPE IS CUT OFF. The
# statement view ends a record at `;` whatever brackets it is inside
# (`lib.sh`), so a bound written only in a `where` clause after an array
# type in the signature is in the next record, and the item cries wolf.
# Write the bound in the generic list.
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

# One function of a file that is not listed whole: `PATH FN`.
CERT_ITEMS=(
  # The fitted lane's four checks and the two doors that run them with
  # the lane in hand.
  'crates/geom-brep/src/pcurve_cache.rs run_fitted_checks'
  'crates/geom-brep/src/pcurve_cache.rs certify_fitted'
  'crates/geom-brep/src/pcurve_cache/stated.rs certify_stated_bound'
  # The certifying edge doors' engine.
  'crates/geom-brep/src/certify.rs run_checks'
  # The interval re-derivation of a sign walk's volume, and the
  # per-region carries.
  'crates/topo/src/props.rs rederive'
  'crates/topo/src/props.rs rederive_faces'
  'crates/topo/src/props.rs shell_polygons'
  'crates/topo/src/props.rs rederive_polygons'
  'crates/topo/src/props.rs vertex_rings'
  'crates/topo/src/props.rs corner_of'
  'crates/topo/src/props.rs region_carries'
)

# The names that keep a dual out: the marker, each trait declared with
# it as a supertrait, and the two certifying bracket doors.
#   Certifies           crates/geom-core/src/real.rs
#   CertifiedPolicy     crates/topo/src/props.rs
#   CertifiedBounds     crates/geom-core/src/real.rs
#   CertifiedEnclosure  crates/geom-core/src/real.rs
CERT_BOUND_NAMES=(Certifies CertifiedPolicy CertifiedBounds CertifiedEnclosure)

# The reader. One record in, the record out if it declares a generic
# item and names no certifying bound.
gate_interior_reader() {
  local names
  names=$(gate_ere_alternation "${CERT_BOUND_NAMES[@]}")
  CERT_NAMES_RE="(^|[^A-Za-z0-9_])(${names})([^A-Za-z0-9_]|$)" gate_record_awk -v ITEMS="${1:-}" '
    BEGIN {
      NI = split(ITEMS, rows, "\n")
      for (k = 1; k <= NI; k++) { split(rows[k], f2, " "); IP[k] = f2[1]; IN_[k] = f2[2] }
      if (ITEMS == "") NI = 0
      # Traits no scalar is taken through: a callback, a sequence.
      split("Fn FnMut FnOnce Iterator IntoIterator DoubleEndedIterator ExactSizeIterator", ns, " ")
      for (k in ns) NOT_A_SCALAR[ns[k]] = 1
    }
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
      # `impl Trait` written as a type in a fn signature: an anonymous
      # type parameter. A closure trait is not one.
      if (!generic && t ~ /(^|[^A-Za-z0-9_])fn[ \t]+[A-Za-z_]/) {
        sig = t
        while (match(sig, /(^|[^A-Za-z0-9_])impl[ \t]+[A-Za-z_:]+/)) {
          what = substr(sig, RSTART, RLENGTH)
          sig = substr(sig, RSTART + RLENGTH)
          sub(/^.*impl[ \t]+/, "", what)
          n = split(what, seg, "::")
          if (!(seg[n] in NOT_A_SCALAR)) { generic = 1; break }
        }
      }
      if (ITEMS != "") {
        # Item mode: only the listed functions of each file, generic or not.
        hit = 0
        for (k = 1; k <= NI; k++) {
          if (IP[k] != GR_FILE) continue
          # A method taking `self` of the same name is the lane trait
          # entry that calls the function, not the function.
          if (t ~ ("(^|[^A-Za-z0-9_])fn[ \t]+" IN_[k] "[ \t]*\\([ \t]*&?[ \t]*(mut[ \t]+)?self([^A-Za-z0-9_]|$)")) continue
          if (t ~ ("(^|[^A-Za-z0-9_])fn[ \t]+" IN_[k] "[ \t]*[<(]")) { SEEN[k] = 1; hit = 1 }
        }
        if (!hit) next
        if (t ~ ENVIRON["CERT_NAMES_RE"]) next
        print $0
        next
      }
      if (!generic) next
      if (t ~ ENVIRON["CERT_NAMES_RE"]) next
      print $0
    }
    END {
      for (k = 1; k <= NI; k++) if (!SEEN[k]) print "MISSING " IP[k] " " IN_[k]
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
    gate_error "$(gate_name): a generic item in a file listed as certification code (listed above) names no bound that keeps a dual out. Write the bound that says which scalars the item means something at: T: Certifies (geom-core/src/real.rs; it grants no bracket read and needs no entry anywhere), in the generic list or the where clause; a sole T: CertifiedBounds or T: CertifiedEnclosure already passes. If the item is not certification code, it does not belong in a listed file. See crates/geom-core/README.md, DL5.3"
    exit 1
  fi
  local -a item_files=()
  local entry
  for entry in "${CERT_ITEMS[@]}"; do item_files+=("${entry%% *}"); done
  mapfile -t item_files < <(printf '%s\n' "${item_files[@]}" | sort -u)
  gate_require_homes "$CERT_FILES_SUBJECT" "${item_files[@]}"
  local iview ihits
  if ! iview=$(gate_rust_code --skip-cfg-test --statements "${item_files[@]}"); then
    gate_error "$(gate_name): the shared Rust reader could not build the statement view of the listed items' files, so what it did not match is unknown — that is not a pass"
    exit 1
  fi
  ihits=$(printf '%s\n' "$iview" | gate_interior_reader "$(printf '%s\n' "${CERT_ITEMS[@]}")")
  if [ -n "$ihits" ]; then
    printf '%s\n' "$ihits"
    gate_error "$(gate_name): a function listed as a certification interior (listed above) names no bound that keeps a dual out, or is no longer declared in its file (MISSING). It runs certification with a lane in hand, so it is declared over T: Certifies (geom-core/src/real.rs) and takes the lane type itself; if it moved or was renamed, move its CERT_ITEMS entry with it. See crates/geom-core/README.md, DL5.3"
    exit 1
  fi
  GATE_SCAN_FILES=$((${#CERT_FILES[@]} + ${#item_files[@]}))
  GATE_SCAN_NOUN='certification file'
  gate_ok "every generic item in the ${#CERT_FILES[@]} files listed whole, and each of the ${#CERT_ITEMS[@]} functions listed by name, names a bound that keeps a dual out"
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
  # Each listed function, bounded, beside an unbounded neighbour the
  # item list does not hold.
  local entry
  for entry in "${CERT_ITEMS[@]}"; do
    home=${entry%% *}
    mkdir -p "$1/${home%/*}"
    [ -f "$1/$home" ] || printf 'pub fn neighbour<T: Decide>(_t: T) {}\n' > "$1/$home"
    printf 'fn %s<T: geom_core::Certifies>(_t: T) {}\n' "${entry##* }" >> "$1/$home"
  done
}

# The first listed item, re-declared in its file with `$2` as its bound.
plant_item() {
  local entry=${CERT_ITEMS[0]} home
  home=${entry%% *}
  printf 'pub fn neighbour<T: Decide>(_t: T) {}\n' > "$1/$home"
  [ -z "$2" ] || printf 'fn %s<T: %s>(_t: T) {}\n' "${entry##* }" "$2" >> "$1/$home"
  for entry in "${CERT_ITEMS[@]:1}"; do
    [ "${entry%% *}" != "$home" ] || printf 'fn %s<T: geom_core::Certifies>(_t: T) {}\n' "${entry##* }" >> "$1/$home"
  done
}
plant_item_weaker() { plant_item "$1" Decide; }
plant_item_gone() { plant_item "$1" ''; }
plant_ok_item_bracket_door() { plant_item "$1" 'Certifies + CertifiedBounds'; }
plant_impl_trait_argument() { plant_into "$1" 'pub fn helper(_t: impl geom_core::Decide) {}'; }
plant_impl_trait_nested() { plant_into "$1" 'fn helper(_t: &geom::NurbsCurve3<impl Real>) {}'; }
plant_ok_sole_certified_bounds() {
  plant_into "$1" 'fn helper<T: geom_core::CertifiedBounds>(_t: T) {}'
  plant_into "$1" 'impl<T: CertifiedEnclosure> Offset<T> { fn at(&self) {} }'
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
  plant_into "$1" 'fn one_signed(values: impl Iterator<Item = Interval>) -> Option<Sign> { None }'
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
  local want="in a file listed as certification code (listed above) names no bound that keeps a dual out"
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
  gate_selftest_case "$want" plant_impl_trait_argument
  gate_selftest_case "$want" plant_impl_trait_nested
  gate_selftest_case "a function listed as a certification interior" plant_item_weaker
  gate_selftest_case --also "MISSING" "a function listed as a certification interior" plant_item_gone
  gate_selftest_passes "a listed function bounded beside a bracket door" plant_ok_item_bracket_door
  gate_selftest_passes "a sole certifying bracket door, on a fn and on an impl" plant_ok_sole_certified_bounds
  gate_selftest_passes "the bound in a rustfmt-wrapped where clause, beside a bracket door" plant_ok_where_clause
  gate_selftest_passes "the bound through a registered name" plant_ok_alias
  gate_selftest_passes "a lifetime-only list, a const-only list, a concrete impl and a non-generic fn" plant_ok_lifetime_and_const
  gate_selftest_passes "a method with no list of its own in a bounded impl" plant_ok_method_in_bounded_impl
  gate_selftest_passes "a generic item under cfg(test)" plant_ok_test_code
  gate_selftest_passes "an unbounded generic item in a file that is not listed" plant_ok_unlisted_file
  gate_selftest_homes --subject "$CERT_FILES_SUBJECT" "${CERT_FILES[@]}"
  printf '%s selftest OK: passes a clean fixture carrying every listed file; fires on a fn, an impl, a struct, an enum, a trait and a type alias whose generic list holds a type parameter and whose record names no certifying bound, whether the bound is absent, weaker (Decide, Real), spread over a rustfmt where block, present only in prose, or a longer name; fires on a method that declares its own type parameter inside a bounded impl (KNOWN GAP 2) and on `impl Trait` written as a type in a signature, bare or nested; passes the bound in a where clause, through a registered name, a sole certifying bracket door, a closure parameter, lifetime-only and const-only lists, test code and unlisted files; and for the functions listed by name, fires on a weaker bound and on a function no longer declared, and passes one bounded beside a bracket door\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
