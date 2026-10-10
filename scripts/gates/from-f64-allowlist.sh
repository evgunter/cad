#!/usr/bin/env bash
# from-f64-allowlist.sh — every `from_f64` in the kernel's generic
# crates is a constant, or is on a list that says what it is. ONE home;
# ci.yml's `lint` job runs every gate in this directory.
#
# THE SUBJECT. `Real::from_f64` embeds an `f64` exactly, and at the
# symbolic scalar (`geom_core::Sym`) that embedding is a `Lit`: an exact
# rational CONSTANT. A value the kernel computed, read out to `f64` and
# re-entered here, is therefore a different function of the variables
# than the one it came from, and a symbolic `Zero` over it is a theorem
# about the wrong function (ERROR-DESIGN E12; FORK-S4F, fork log row
# 93). A computed value stays in `T`, or enters through
# `Real::from_computed`, which is one unknown per call at `Sym`. Whether
# an argument is computed is semantic, so a token scan cannot decide it:
# it can only make every site a reviewer's decision once, and refuse a
# new site nobody decided. The decision is the list,
# `from-f64-sites.tsv` beside this file, one entry per site with its
# class and its reason. What CATCHES a laundered value is the
# re-valuation row, `crates/editor-core/tests/revalue_corpus.rs`.
#
# THE SCAN is the production code (`lib.sh`'s reader, `#[cfg(test)]`
# dropped) of the crates in FROM_F64_CRATES, read in the statement view
# so a call rustfmt wraps over lines is one record. Every `from_f64` as
# a word is a site — a call `T::from_f64(x)` and a point-free use
# `.map(T::from_f64)` alike — except a definition (`fn from_f64`).
#
# EXEMPT BY SHAPE, and nothing else: an argument built only from
# numeric literals (`0.5`, `-1e-3`, `2_f64`), SCREAMING_CASE constant
# paths (`f64::consts::PI`, `ARC_SAMPLES`, `geom_core::UNIT_ROUNDOFF`),
# `as f64`, parentheses and `+ - * /` — `8.0 * f64::EPSILON`,
# `ARC_SAMPLES as f64`. Each is a constant at compile time, so it cannot
# be a laundered value.
#
# THE KEY of a listed site is its file and its argument's text with
# whitespace collapsed (`<fn>` for a point-free use), and the entry
# carries how many such sites the file holds. No line numbers: they
# move with every edit above them, and a list keyed by them would churn
# on changes that touch no site.
#
# THE CLASSES (the TSV's third column):
#
#   constant      a constant of the model or the algorithm: a count, a
#                 sample schedule's fraction, a tolerance or band, a
#                 float bit pattern chosen by the code;
#   fixture       test-support code compiled into the library;
#   bracket       a concrete non-symbolic scalar (`Interval::from_f64`,
#                 a certification bracket's endpoint), which no `Sym`
#                 lane reaches;
#   launders(R)   a computed value re-entered as a constant, owed to the
#                 work item R, which owns its fix;
#   stored-carrier-data(R)
#                 a carrier's `f64` field (a NURBS weight or knot) read
#                 back into `T`: laundering by TYPE, owed to R.
#
# The two owed classes are the audit's open count; the gate prints it.
#
# WHAT REDS, each with its own tag:
#
#   UNLISTED  a site whose key is not on the list, or a file holding
#             more such sites than its entry says;
#   STALE     an entry whose file holds fewer sites than it says (a fix
#             landed: drop or lower the entry in the same change);
#   CLASS     an entry whose class is not one of the above, whose reason
#             is empty, or whose owed class names a work item that is not
#             a file under `work/`.
#
# KNOWN GAPS: a value that leaves `T` and comes back by another door
# (`T::zero() + …` built from `f64` arithmetic on `lo()`, a crate outside
# FROM_F64_CRATES) is not a `from_f64` and this scan does not see it;
# the re-valuation row does, wherever the corpus reaches it. A
# point-free use bound to a name (`let f = T::from_f64;`) is one listed
# `<fn>` site, and the calls through `f` are not read. The statement
# view cuts a record at `{`, so an argument holding a block (`if … {`)
# is keyed by its text up to the brace. A macro body is read as written.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

FROM_F64_CRATES=(geom geom-brep sweep topo)
FROM_F64_LIST=scripts/gates/from-f64-sites.tsv

FROM_F64_RULES=(UNLISTED STALE CLASS)

from_f64_rule_message() {
  case "$1" in
    UNLISTED) printf '%s' "a from_f64 site that is neither a literal nor a listed site. Keep a computed value in T, or enter it through Real::from_computed (one unknown per call at Sym); a genuine constant goes on $FROM_F64_LIST with its class and reason" ;;
    STALE) printf '%s' "a listed from_f64 site is gone (or fewer remain than listed) — drop or lower its entry in $FROM_F64_LIST in the change that removed it" ;;
    CLASS) printf '%s' "an entry in $FROM_F64_LIST has an unknown class, an empty reason, or an owed class naming no work item under work/" ;;
  esac
}

# The sites, one `file<TAB>argument` per line, over the statement view
# on stdin. The argument is read to its balancing parenthesis inside the
# record (the statement view holds a whole call).
from_f64_sites() {
  gate_record_awk '
    {
      if (!gate_record_split($0)) next
      t = GR_TEXT
      while (match(t, /(^|[^A-Za-z0-9_])from_f64([^A-Za-z0-9_]|$)/)) {
        pre = substr(t, 1, RSTART)
        s = RSTART + (substr(t, RSTART, 1) ~ /[A-Za-z0-9_]/ ? 0 : 1) + 8
        rest = substr(t, s)
        t = rest
        if (pre ~ /fn[ \t]+$/ || pre ~ /fn[ \t]+f$/) continue
        sub(/^[ \t]+/, "", rest)
        if (substr(rest, 1, 1) != "(") { print GR_FILE "\t<fn>"; continue }
        d = 0; a = ""
        for (i = 1; i <= length(rest); i++) {
          c = substr(rest, i, 1)
          if (c == "(") { d++; if (d == 1) continue }
          else if (c == ")") { d--; if (d == 0) break }
          a = a c
        }
        # rustfmt leaves a trailing comma on a call it wraps.
        gsub(/[ \t]+/, " ", a); sub(/^ /, "", a); sub(/ ?,? ?$/, "", a)
        print GR_FILE "\t" a
      }
    }'
}

# One token of an exempt argument, once `as f64`, the operators and the
# parentheses are spaces: a numeric literal or a constant path.
FROM_F64_TOKEN_RE='^([0-9][0-9_]*(\.[0-9_]*)?([eE][0-9]+)?(_?f64)?|([A-Za-z_][A-Za-z0-9_]*::)*[A-Z][A-Z0-9_]*)$'
FROM_F64_CLASS_RE='^(constant|fixture|bracket|launders\([A-Za-z0-9_.-]+\)|stored-carrier-data\([A-Za-z0-9_.-]+\))$'

gate() {
  gate_require_crate_sources
  gate_production_sources
  gate_require_file "$FROM_F64_LIST"
  local files=() f c
  for f in "${GATE_PRODUCTION_FILES[@]}"; do
    for c in "${FROM_F64_CRATES[@]}"; do
      case "$f" in "crates/$c/src/"*) files+=("$f") ;; esac
    done
  done
  if [ "${#files[@]}" -eq 0 ]; then
    gate_error "$(gate_name): no production source under crates/{$(IFS=,; echo "${FROM_F64_CRATES[*]}")}/src — the scan decided nothing, which is not a pass"
    exit 1
  fi
  local view
  if ! view=$(gate_rust_code --skip-cfg-test --statements "${files[@]}"); then
    gate_error "$(gate_name): the shared Rust reader could not build the statement view, so what it did not match is unknown — that is not a pass"
    exit 1
  fi
  local sites status=0
  sites=$(printf '%s\n' "$view" | from_f64_sites) || status=$?
  [ "$status" -eq 0 ] || gate_reader_died_refusal "the site walk over the statement view" "$status" \
    "What it did not read is unknown."
  local verdicts
  status=0
  verdicts=$(printf '%s\n' "$sites" | awk -F'\t' \
    -v LIST="$FROM_F64_LIST" \
    -v TOKEN_RE="$FROM_F64_TOKEN_RE" \
    -v CLASS_RE="$FROM_F64_CLASS_RE" '
    function constant_shape(a,    k, n, tok) {
      if (a == "<fn>") return 0
      gsub(/[ \t]as[ \t]+f64/, " ", a)
      gsub(/[eE][-+]/, "e", a)
      gsub(/[-+*\/()]/, " ", a)
      n = split(a, tok, /[ \t]+/)
      for (k = 1; k <= n; k++) if (tok[k] != "" && tok[k] !~ TOKEN_RE) return 0
      return 1
    }
    BEGIN {
      while ((getline line < LIST) > 0) {
        if (line ~ /^#/ || line ~ /^[ \t]*$/) continue
        k = split(line, col, "\t")
        key = col[1] "\t" col[4]
        want[key] += col[2] + 0
        cls = col[3]; why = (k >= 5) ? col[5] : ""
        if (cls !~ CLASS_RE || why ~ /^[ \t]*$/) { print "CLASS " line; continue }
        if (match(cls, /\(.*\)/)) {
          row = substr(cls, RSTART + 1, RLENGTH - 2)
          print "OWED " row "\t" line
          owed++
          owed_n += col[2]
        }
        listed_n += col[2]
      }
    }
    NF >= 2 {
      if (constant_shape($2)) { lit++; next }
      have[$1 "\t" $2]++
    }
    END {
      for (k in have) if (have[k] > want[k] + 0) {
        split(k, p, "\t"); print "UNLISTED " p[1] ": from_f64(" p[2] ") x" have[k] " (listed " want[k] + 0 ")"
      }
      for (k in want) if (want[k] > have[k] + 0) {
        split(k, p, "\t"); print "STALE " p[1] ": from_f64(" p[2] ") listed x" want[k] ", found " have[k] + 0
      }
      printf "COUNT %d %d %d\n", lit + 0, listed_n + 0, owed_n + 0
    }') || status=$?
  [ "$status" -eq 0 ] || gate_reader_died_refusal "the list walk" "$status" "What it did not compare is unknown."
  # An owed class names a work item: an item file under `work/`.
  local owed row entry
  owed=$(printf '%s\n' "$verdicts" | sed -n 's/^OWED //p' | sort -u)
  while IFS=$'\t' read -r row entry; do
    [ -n "$row" ] || continue
    if [ -z "$(find work -name "$row.md" -type f 2>/dev/null | head -1)" ]; then
      verdicts=$(printf '%s\nCLASS %s (no work item %s)' "$verdicts" "$entry" "$row")
    fi
  done < <(printf '%s\n' "$owed" | awk -F'\t' '{ r = $1; $1 = ""; sub(/^\t/, ""); print r "\t" $0 }' OFS='\t')
  local rule line hit any=false
  for rule in "${FROM_F64_RULES[@]}"; do
    hit=false
    while IFS= read -r line; do
      case "$line" in
        "$rule "*) printf 'FROM-F64 %s: %s\n' "$rule" "${line#"$rule "}"; hit=true ;;
      esac
    done <<< "$verdicts"
    if [ "$hit" = true ]; then
      any=true
      gate_error "$(gate_name) $rule: $(from_f64_rule_message "$rule")"
    fi
  done
  [ "$any" = false ] || exit 1
  local lit listed owed
  read -r lit listed owed < <(printf '%s\n' "$verdicts" | sed -n 's/^COUNT //p')
  GATE_SCAN_FILES=${#files[@]}
  gate_ok "every from_f64 in crates/{$(IFS=,; echo "${FROM_F64_CRATES[*]}")}/src production code is a literal ($lit) or a listed site ($listed), and $owed listed sites are still owed (launders / stored-carrier-data)"
}

# --- SELF-TEST ---------------------------------------------------------
#
# The clean fixture is a tree of its own: one crate in the scan, a list
# naming its sites, and the work item an owed class names.
fixture_file=crates/geom/src/lib.rs

from_f64_fixture_list() {
  printf '# file\tcount\tclass\targument\treason\n'
  printf '%s\t2\tconstant\tn as f64\ta sample count\n' "$fixture_file"
  printf '%s\t1\tlaunders(owed-row)\tfoot.x\ta projection foot, owed\n' "$fixture_file"
  printf '%s\t1\tfixture\t<fn>\ta point-free fixture lift\n' "$fixture_file"
}

gate_plant_clean() {
  gate_plant_clean_sources "$1"
  mkdir -p "$1/crates/geom/src" "$1/scripts/gates" "$1/work/geom"
  {
    printf 'pub fn f<T: Real>(n: usize, foot: P) -> T {\n'
    printf '    let a = T::from_f64(n as f64) + T::from_f64(0.5) + T::from_f64(f64::consts::PI);\n'
    printf '    let b = T::from_f64(\n        n as f64,\n    );\n'
    printf '    let c = T::from_f64(1.0 / 3.0) * T::from_f64(foot.x);\n'
    printf '    let _ = v.map(T::from_f64);\n'
    printf '    a + b + c\n}\n'
    printf '#[cfg(test)]\nmod tests {\n    fn t() { let _ = T::from_f64(x.lo()); }\n}\n'
  } > "$1/$fixture_file"
  from_f64_fixture_list > "$1/$FROM_F64_LIST"
  printf -- '---\nid: owed-row\n---\n' > "$1/work/geom/owed-row.md"
}

plant_unlisted() { printf 'pub fn g<T: Real>(r: R) -> T {\n    T::from_f64(r.lo())\n}\n' >> "$1/$fixture_file"; }
plant_unlisted_wrapped() {
  printf 'pub fn g<T: Real>(r: R) -> T {\n    T::from_f64(\n        r.hi(),\n    )\n}\n' >> "$1/$fixture_file"
}
plant_unlisted_point_free() { printf 'pub fn g() {\n    let _ = w.map_scalar(T::from_f64);\n}\n' >> "$1/$fixture_file"; }
plant_one_more_listed() { printf 'pub fn g<T: Real>(foot: P) -> T {\n    T::from_f64(foot.x)\n}\n' >> "$1/$fixture_file"; }
plant_unlisted_other_crate() {
  mkdir -p "$1/crates/topo/src"
  printf 'pub fn g<T: Real>(u1: f64, u0: f64) -> T {\n    T::from_f64(u1 - u0)\n}\n' > "$1/crates/topo/src/lib.rs"
}
plant_stale() { sed -i 's/T::from_f64(foot.x)/foot_t/' "$1/$fixture_file"; }
plant_bad_class() { printf '%s\t1\tfine\tz\tit is fine\n' "$fixture_file" >> "$1/$FROM_F64_LIST"; }
plant_empty_reason() { printf '%s\t1\tconstant\tz\t\n' "$fixture_file" >> "$1/$FROM_F64_LIST"; }
plant_missing_row() { sed -i 's/launders(owed-row)/launders(no-such-row)/' "$1/$FROM_F64_LIST"; }
plant_near_misses() {
  mkdir -p "$1/crates/planted/src"
  {
    printf '// T::from_f64(x.lo()) in a comment\n'
    printf 'pub const WHY: &str = "T::from_f64(x.lo())";\n'
    printf 'pub fn g<T: Real>() -> T { T::from_f64x(1.0) + T::from_f64_bits(2) }\n'
    printf 'impl Real for X {\n    fn from_f64(x: f64) -> Self { X(x) }\n}\n'
    printf 'pub fn h<T: Real>() -> T { T::from_f64(-2.5e-3) + T::from_f64(3_f64) + T::from_f64(-K::MAX_TURN) }\n'
  } >> "$1/$fixture_file"
  printf 'pub fn g<T: Real>(x: R) -> T {\n    T::from_f64(x.lo())\n}\n' > "$1/crates/planted/src/lib.rs"
}

from_f64_case() {
  local rule=$1; shift
  gate_selftest_case "$(gate_name) $rule: " "$@"
}

gate_selftest() {
  gate_selftest_clean
  gate_selftest_without_tool awk "could not build the statement view"
  from_f64_case UNLISTED plant_unlisted
  from_f64_case UNLISTED plant_unlisted_wrapped
  from_f64_case UNLISTED plant_unlisted_point_free
  from_f64_case UNLISTED plant_one_more_listed
  from_f64_case UNLISTED plant_unlisted_other_crate
  from_f64_case STALE plant_stale
  from_f64_case CLASS plant_bad_class
  from_f64_case CLASS plant_empty_reason
  from_f64_case CLASS plant_missing_row
  gate_selftest_passes "a comment, a string literal, longer names, a definition, literals and a SCREAMING_CASE constant in a listed file, and a computed site in a crate outside the scan" \
    plant_near_misses
  printf '%s selftest OK: passes a clean fixture (literals, a literal ratio, a constant path, a wrapped listed call, a point-free listed use, a cfg(test) site); fires UNLISTED on a computed call, one wrapped over lines, a point-free use, one site more than listed and a site in another scanned crate; STALE on a listed site that went away; CLASS on an unknown class, an empty reason and an owed class naming no work item; passes every near miss; and stays RED when awk cannot run\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
