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
# AND ONE CHECK OVER EVERY CRATE: THE LANE HELD BY VALUE. A function
# that takes a lane type itself (`lane: QuadLane<T>`) carries the bound
# because the type is declared over it; the compiler holds that. What
# the compiler does not hold is the lane taken BY VALUE through its
# holder: `lane: impl QuadLaneOf<T>` or `lane: T::QuadLane` at a bound
# a dual meets is a function that runs with a lane in hand and is
# well-formed at a dual. A holder is for a pass that may hold none
# (`Option<impl …Of<T>>`, `Option<T::…Lane>`); with a lane in hand the
# function is declared over a certifying bound and takes the lane type.
# So in every `crates/*/src` production file, a `fn` whose signature
# takes a holder by value and names no certifying bound fails, unless
# it is in `CERT_HOLDER_EXCEPTIONS` with its reason. Not read: a lane
# bound to a LOCAL after the pass asked for it (`let lane =
# lane.ok_or(..)?`), which is a body and not a signature.
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
  # Importers of the `Interval` certification doors
  # (`certification-doors.sh`'s list) that declare a generic item and
  # whose every one already names a bound that keeps a dual out. The
  # importers that declare no generic item are not listed: a file with
  # nothing to hold is no coverage.
  crates/geom-brep/src/props/rational_rim.rs
  crates/geom-brep/src/ssi/four.rs
  crates/geom-core/src/spline/blossom.rs
  crates/geom-core/src/spline/net.rs
  crates/mesh/src/nurbs_cert.rs
)

# How a holder is spelled: a lane trait applied, or the policy's lane.
CERT_HOLDER_RE='LaneOf<|FootOf<|DoorOf<|T::(QuadLane|RegionLane|SsiLane|FittedLane|SectionLane|ClearanceLane|OffsetFitLane|SplineFoot|ShellDoor)'

# A function that takes a lane's holder by value at a bound a dual
# meets, with the reason it may: `PATH FN`.
CERT_HOLDER_EXCEPTIONS=(
  # The verb seat's shell door. `Shell::run_shell` is the seat's own
  # door at every scalar and its one use of the value is `door.open`;
  # `verbs/tests/lane3_review_probes.rs` pins that at a dual its fourth
  # argument has no values.
  'crates/verbs/src/run.rs run_shell'
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
  CERT_NAMES_RE="(^|[^A-Za-z0-9_])(${names})([^A-Za-z0-9_]|$)" gate_record_awk -v HOLDERS="${1:-}" -v EXC="${2:-}" '
    BEGIN {
      NE = split(EXC, rows, "\n")
      for (k = 1; k <= NE; k++) { split(rows[k], f2, " "); EP[k] = f2[1]; EF[k] = f2[2] }
      if (EXC == "") NE = 0
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
      # A bound on an associated type inside an `impl Trait` argument
      # (`impl Iterator<Item: Real>`) is a type parameter too, whatever
      # the outer trait.
      if (!generic && t ~ /(^|[^A-Za-z0-9_])fn[ \t]+[A-Za-z_]/ && t ~ /(^|[^A-Za-z0-9_])impl[ \t]+[A-Za-z_:]+<[^>]*[A-Za-z0-9_][ \t]*:[^:]/) generic = 1
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
      if (HOLDERS != "") {
        # Holder mode: a fn signature taking a lane holder by value.
        if (t !~ /(^|[^A-Za-z0-9_])fn[ \t]+[A-Za-z_]/) next
        if (t !~ /[(,][ \t]*(mut[ \t]+)?[a-z_][a-z0-9_]*[ \t]*:[ \t]*(impl[ \t]+[A-Za-z_:]*(LaneOf|FootOf|DoorOf)<|T::(QuadLane|RegionLane|SsiLane|FittedLane|SectionLane|ClearanceLane|OffsetFitLane|SplineFoot|ShellDoor)([^A-Za-z0-9_]|$))/) next
        if (t ~ ENVIRON["CERT_NAMES_RE"]) next
        for (k = 1; k <= NE; k++)
          if (EP[k] == GR_FILE && t ~ ("(^|[^A-Za-z0-9_])fn[ \t]+" EF[k] "[ \t]*[<(]")) { USED[k] = 1; next }
        print $0
        next
      }
      if (!generic) next
      if (t ~ ENVIRON["CERT_NAMES_RE"]) next
      print $0
    }
    END {
      if (HOLDERS != "") for (k = 1; k <= NE; k++) if (!USED[k]) print "STALE " EP[k] " " EF[k]
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
  # Only a file that spells a holder can hold a hit (the spelling is
  # one token, so a wrapped signature still has it on a line), and the
  # reader is slow over a thousand files.
  local hview hhits
  local -a hfiles=()
  mapfile -t hfiles < <(grep -lE "$CERT_HOLDER_RE" "${GATE_SOURCE_FILES[@]}")
  if [ "${#hfiles[@]}" -eq 0 ]; then
    gate_error "$(gate_name): no crate source spells a lane holder at all — the holders were renamed or the scan is reading nothing, and neither is a pass"
    exit 1
  fi
  if ! hview=$(gate_rust_code --skip-cfg-test --statements "${hfiles[@]}"); then
    gate_error "$(gate_name): the shared Rust reader could not build the statement view of the crate sources, so what it did not match is unknown — that is not a pass"
    exit 1
  fi
  hhits=$(printf '%s\n' "$hview" | gate_grep -E "$CERT_HOLDER_RE" |
    gate_interior_reader holders "$(printf '%s\n' "${CERT_HOLDER_EXCEPTIONS[@]}")")
  if [ -n "$hhits" ]; then
    printf '%s\n' "$hhits"
    gate_error "$(gate_name): a function takes a lane by value through its holder (impl …Of<T>, or the policy's T::…Lane) and names no bound that keeps a dual out (listed above), or an exception names a function that is no longer one (STALE). A function with a lane in hand runs certification: declare it over T: Certifies (geom-core/src/real.rs) and take the lane type itself, entered from the pass that holds Option<…> through the lane trait's method. A pass that may hold none takes Option<impl …Of<T>> or Option<T::…Lane>. See crates/geom-core/README.md, DL5.3"
    exit 1
  fi
  gate_ok "every generic item in the ${#CERT_FILES[@]} files listed whole names a bound that keeps a dual out, and no function in any crate takes a lane by value through its holder at a bound a dual meets (${#CERT_HOLDER_EXCEPTIONS[@]} argued exception)"
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
  # Each argued exception, as its file writes it.
  local entry
  for entry in "${CERT_HOLDER_EXCEPTIONS[@]}"; do
    home=${entry%% *}
    mkdir -p "$1/${home%/*}"
    printf 'pub fn %s<T: DoorScalar>(door: T::ShellDoor) {}\n' "${entry##* }" >> "$1/$home"
  done
}

plant_holder_impl_by_value() {
  mkdir -p "$1/crates/planted/src"
  printf 'fn new_interior<T: Decide>(_lane: impl QuadLaneOf<T>) {}\n' > "$1/crates/planted/src/lib.rs"
}
plant_holder_policy_by_value() {
  mkdir -p "$1/crates/planted/src"
  printf 'fn new_interior<T: AtRestPolicy>(\n    body: &Body<T>,\n    lane: T::RegionLane,\n) {\n}\n' > "$1/crates/planted/src/lib.rs"
}
plant_holder_exception_gone() {
  local entry=${CERT_HOLDER_EXCEPTIONS[0]}
  printf 'pub fn other<T: DoorScalar>(_door: Option<T::ShellDoor>) {}\n' > "$1/${entry%% *}"
}
plant_ok_holders() {
  mkdir -p "$1/crates/planted/src"
  {
    printf 'fn pass<T: Decide>(_lane: Option<impl QuadLaneOf<T>>) {}\n'
    printf 'fn pass2<T: AtRestPolicy>(_lane: Option<T::QuadLane>) {}\n'
    printf 'fn interior<T: geom_core::Certifies>(_lane: QuadLane<T>) {}\n'
    printf 'fn entry<T: Certifies>(_lane: impl QuadLaneOf<T>) {}\n'
    printf 'fn thunk<T: Decide, L: SectionLaneOf<T>>(_lane: impl FnOnce() -> L) {}\n'
  } > "$1/crates/planted/src/lib.rs"
}
plant_impl_iterator_item_bound() { plant_into "$1" 'fn h(_x: impl Iterator<Item: geom_core::Real>) {}'; }
plant_impl_intoiterator_item_bound() { plant_into "$1" 'fn h(_x: impl IntoIterator<Item: Decide>) {}'; }
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
  gate_selftest_case "$want" plant_impl_iterator_item_bound
  gate_selftest_case "$want" plant_impl_intoiterator_item_bound
  gate_selftest_case "takes a lane by value through its holder" plant_holder_impl_by_value
  gate_selftest_case "takes a lane by value through its holder" plant_holder_policy_by_value
  gate_selftest_case --also "STALE" "takes a lane by value through its holder" plant_holder_exception_gone
  gate_selftest_passes "a holder in an Option, a lane type at the certifying bound, a holder at the certifying bound, and a thunk that returns one" plant_ok_holders
  gate_selftest_passes "a sole certifying bracket door, on a fn and on an impl" plant_ok_sole_certified_bounds
  gate_selftest_passes "the bound in a rustfmt-wrapped where clause, beside a bracket door" plant_ok_where_clause
  gate_selftest_passes "the bound through a registered name" plant_ok_alias
  gate_selftest_passes "a lifetime-only list, a const-only list, a concrete impl and a non-generic fn" plant_ok_lifetime_and_const
  gate_selftest_passes "a method with no list of its own in a bounded impl" plant_ok_method_in_bounded_impl
  gate_selftest_passes "a generic item under cfg(test)" plant_ok_test_code
  gate_selftest_passes "an unbounded generic item in a file that is not listed" plant_ok_unlisted_file
  gate_selftest_homes --subject "$CERT_FILES_SUBJECT" "${CERT_FILES[@]}"
  printf '%s selftest OK: passes a clean fixture carrying every listed file; fires on a fn, an impl, a struct, an enum, a trait and a type alias whose generic list holds a type parameter and whose record names no certifying bound, whether the bound is absent, weaker (Decide, Real), spread over a rustfmt where block, present only in prose, or a longer name; fires on a method that declares its own type parameter inside a bounded impl (KNOWN GAP 2) and on `impl Trait` written as a type in a signature, bare or nested; passes the bound in a where clause, through a registered name, a sole certifying bracket door, a closure parameter, lifetime-only and const-only lists, test code and unlisted files; fires on a bound on an associated type inside an `impl Trait` argument; and over every crate, fires on a lane taken by value through `impl …Of<T>` or `T::…Lane` at a bound a dual meets and on a stale exception, and passes the holder in an `Option`, the lane type, and the argued exception\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
