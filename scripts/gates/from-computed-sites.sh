#!/usr/bin/env bash
# from-computed-sites.sh — `Real::from_computed` has no caller outside
# the admitting door(s) and the place step, but the sites a listed open
# work item owns. ONE home; ci.yml's `lint` job runs every gate here.
#
# THE SUBJECT (`crates/geom-core/README.md`, DL5.2). A computed `f64`
# re-entering `T` is sorted by what it is: a bound decides at `f64`,
# a place is handed to an evaluator as an `f64`, data is built at `T`,
# a constant enters by where it comes from, and a solved value the body
# stores enters through a door that runs its check at `T`
# (`geom_core::solved`). So `from_computed` is called by the door, by
# a scalar's own impl, and by the place step a place-taking evaluator
# keeps private; every other call is a re-entry some open work item
# still owes, listed here until it lands.
#
# THE SCAN is the production code (`lib.sh`'s reader, `#[cfg(test)]`
# dropped) of every crate under `crates/*/src` and every demo under
# `demos/*/src` (ordinary consumers of the public API), read in the
# statement view so a call rustfmt wraps over lines is one record. Every
# `from_computed` as a word is a site: a call `T::from_computed(x)`,
# a qualified `<Interval as Real>::from_computed(x)` and a point-free
# `.map(T::from_computed)` alike, except a definition
# (`fn from_computed`). No argument is exempt by its shape: a constant
# enters through `from_f64`, never here.
#
# THE KEY of a listed site is its file and its argument's text with
# whitespace collapsed (`<fn>` for a point-free use), and the entry
# carries how many such sites the file holds (`from-f64-allowlist.sh`'s
# keying: no line numbers).
#
# THE CLASSES (the TSV's third column), each a class of caller, never a
# function's name:
#
#   door          an admitting door: the check at `T` runs inside it
#                 and its caller cannot have the value without it
#                 (`geom_core::solved`);
#   scalar        a `Real` impl's own `from_computed`, forwarding to
#                 its value scalar's;
#   place-step    the one step from a place to the scalar inside a
#                 place-taking evaluator, private to it
#                 (`geom::places`'s `at_place`, and the evaluators
#                 that keep the step inside their own module);
#   owed(R)       a re-entry the open work item R owns: a stored value
#                 not yet behind a door, a bound, a place held at `T`,
#                 data to build at `T`, a fitted value.
#
# The owed class is the open count; the gate prints it.
#
# WHAT REDS, each with its own tag:
#
#   UNLISTED  a site whose key is not on the list, or a file holding
#             more such sites than its entry says;
#   STALE     an entry whose file holds fewer sites than it says (a site
#             moved behind a door: drop or lower the entry in the same
#             change);
#   CLASS     an entry whose class is not one of the above, whose reason
#             is empty, or whose owed class names a work item that is not
#             an open file under `work/`.
#
# KNOWN GAPS. What a token scan cannot see:
#   - An alias: `let lift = T::from_computed;` is one `<fn>` site and
#     the calls through `lift` are not read; every `<fn>` site in a
#     file shares one key, so fixing one listed lift while adding
#     another keeps the count.
#   - A helper that passes an `f64` through: a function taking an
#     `f64` and returning `T` by way of a listed site (`measured_foot`,
#     `computed_foot`) adds no site when a new caller hands it a value.
#     The door's own signature has the same shape: a caller can hand it
#     any candidate, and what holds is only that the candidate is
#     checked.
#   - A residual a caller writes that does not depend on the value the
#     door hands it: the door runs it, it cannot read what it computes.
#   - `from_f64` spelling a computed value (`from-f64-allowlist.sh`'s
#     subject), test code, `tools/`, `benches/`, and a macro body as
#     expanded.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

FROM_COMPUTED_LIST=scripts/gates/from-computed-sites.tsv
FROM_COMPUTED_RULES=(UNLISTED STALE CLASS)

from_computed_rule_message() {
  case "$1" in
    UNLISTED) printf '%s' "a from_computed site outside the admitting door(s), a scalar's impl and the place step, and not listed. A stored solved value enters through geom_core::solved::admit; a place goes to a place-taking evaluator as an f64; a bound decides at f64 (crates/geom-core/README.md, DL5.2). A re-entry an open work item owns goes on $FROM_COMPUTED_LIST with its class and reason" ;;
    STALE) printf '%s' "a listed from_computed site is gone (or fewer remain than listed): drop or lower its entry in $FROM_COMPUTED_LIST in the change that removed it" ;;
    CLASS) printf '%s' "an entry in $FROM_COMPUTED_LIST has an unknown class, an empty reason, or an owed class naming no open work item under work/" ;;
  esac
}

# The sites, one `file<TAB>argument` per line, over the statement view
# on stdin.
from_computed_sites() {
  gate_record_awk '
    {
      if (!gate_record_split($0)) next
      t = GR_TEXT
      while (match(t, /(^|[^A-Za-z0-9_])from_computed([^A-Za-z0-9_]|$)/)) {
        pre = substr(t, 1, RSTART)
        s = RSTART + (substr(t, RSTART, 1) ~ /[A-Za-z0-9_]/ ? 0 : 1) + 13
        rest = substr(t, s)
        t = rest
        if (pre ~ /fn[ \t]+$/) continue
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
        # The record ended inside the call: the view cut it at a `{`.
        if (d != 0 || a == "") a = "<block> " a
        print GR_FILE "\t" a
      }
    }'
}

FROM_COMPUTED_CLASS_RE='^(door|scalar|place-step|owed[(][A-Za-z0-9_.-]+[)])$'

gate() {
  gate_require_crate_sources
  gate_production_sources
  gate_require_file "$FROM_COMPUTED_LIST"
  local files=("${GATE_PRODUCTION_FILES[@]}") f
  if [ -d demos ]; then
    while IFS= read -r f; do files+=("$f"); done < <(find demos/*/src -type f -name '*.rs' 2>/dev/null | sort)
  fi
  local view
  if ! view=$(gate_rust_code --skip-cfg-test --statements "${files[@]}"); then
    gate_error "$(gate_name): the shared Rust reader could not build the statement view, so what it did not match is unknown — that is not a pass"
    exit 1
  fi
  local sites status=0
  sites=$(printf '%s\n' "$view" | from_computed_sites) || status=$?
  [ "$status" -eq 0 ] || gate_reader_died_refusal "the site walk over the statement view" "$status" \
    "What it did not read is unknown."
  if [ "${FROM_COMPUTED_DUMP:-}" = 1 ]; then printf '%s\n' "$sites" | sort | uniq -c; exit 0; fi
  local verdicts
  status=0
  verdicts=$(printf '%s\n' "$sites" | awk -F'\t' \
    -v LIST="$FROM_COMPUTED_LIST" \
    -v CLASS_RE="$FROM_COMPUTED_CLASS_RE" '
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
          owed_n += col[2]
        }
        listed_n += col[2]
      }
    }
    NF >= 2 { have[$1 "\t" $2]++ }
    END {
      for (k in have) if (have[k] > want[k] + 0) {
        split(k, p, "\t"); print "UNLISTED " p[1] ": from_computed(" p[2] ") x" have[k] " (listed " want[k] + 0 ")"
      }
      for (k in want) if (want[k] > have[k] + 0) {
        split(k, p, "\t"); print "STALE " p[1] ": from_computed(" p[2] ") listed x" want[k] ", found " have[k] + 0
      }
      printf "COUNT %d %d\n", listed_n + 0, owed_n + 0
    }') || status=$?
  [ "$status" -eq 0 ] || gate_reader_died_refusal "the list walk" "$status" "What it did not compare is unknown."
  local owed row entry item
  owed=$(printf '%s\n' "$verdicts" | sed -n 's/^OWED //p' | sort -u)
  while IFS=$'\t' read -r row entry; do
    [ -n "$row" ] || continue
    item=$(find work -name "$row.md" -type f 2>/dev/null | head -1)
    if [ -z "$item" ]; then
      verdicts=$(printf '%s\nCLASS %s (no work item %s)' "$verdicts" "$entry" "$row")
    elif grep -q '^status: closed' "$item"; then
      verdicts=$(printf '%s\nCLASS %s (work item %s is closed)' "$verdicts" "$entry" "$row")
    fi
  done < <(printf '%s\n' "$owed" | awk -F'\t' '{ r = $1; $1 = ""; sub(/^\t/, ""); print r "\t" $0 }' OFS='\t')
  local rule line hit any=false
  for rule in "${FROM_COMPUTED_RULES[@]}"; do
    hit=false
    while IFS= read -r line; do
      case "$line" in
        "$rule "*) printf 'FROM-COMPUTED %s: %s\n' "$rule" "${line#"$rule "}"; hit=true ;;
      esac
    done <<< "$verdicts"
    if [ "$hit" = true ]; then
      any=true
      gate_error "$(gate_name) $rule: $(from_computed_rule_message "$rule")"
    fi
  done
  [ "$any" = false ] || exit 1
  local listed owed_n
  read -r listed owed_n < <(printf '%s\n' "$verdicts" | sed -n 's/^COUNT //p')
  GATE_SCAN_FILES=${#files[@]}
  gate_ok "every from_computed in crates/*/src and demos/*/src production code is a listed site ($listed): the admitting door(s), a scalar's impl, the place step, and $owed_n re-entries still owed to open work items"
}

# --- SELF-TEST ---------------------------------------------------------
fixture_file=crates/geom/src/lib.rs

gate_plant_clean() {
  gate_plant_clean_sources "$1"
  mkdir -p "$1/crates/geom/src" "$1/scripts/gates" "$1/work/geom"
  {
    printf 'pub fn admit<T: Certifies>(c: f64) -> T {\n    T::from_computed(c)\n}\n'
    printf 'pub fn f<T: Real>(foot: P) -> T {\n    T::from_computed(\n        foot.x,\n    )\n}\n'
    printf 'pub fn g() { let _ = v.map(T::from_computed); }\n'
    printf '#[cfg(test)]\nmod tests {\n    fn t() { let _ = T::from_computed(x.lo()); }\n}\n'
  } > "$1/$fixture_file"
  {
    printf '# file\tcount\tclass\targument\treason\n'
    printf '%s\t1\tdoor\tc\tthe door\n' "$fixture_file"
    printf '%s\t1\towed(owed-row)\tfoot.x\ta foot, owed\n' "$fixture_file"
    printf '%s\t1\tplace-step\t<fn>\ta place step\n' "$fixture_file"
  } > "$1/$FROM_COMPUTED_LIST"
  printf -- '---\nid: owed-row\n---\n' > "$1/work/geom/owed-row.md"
}

plant_unlisted() { printf 'pub fn h<T: Real>(r: R) -> T {\n    T::from_computed(r.root)\n}\n' >> "$1/$fixture_file"; }
plant_unlisted_qualified() { printf 'pub fn h(r: R) -> Interval {\n    <Interval as Real>::from_computed(r.root)\n}\n' >> "$1/$fixture_file"; }
plant_unlisted_literal() { printf 'pub fn h<T: Real>() -> T {\n    T::from_computed(0.5)\n}\n' >> "$1/$fixture_file"; }
plant_one_more_listed() { printf 'pub fn h<T: Real>(foot: P) -> T {\n    T::from_computed(foot.x)\n}\n' >> "$1/$fixture_file"; }
plant_unlisted_demo() {
  mkdir -p "$1/demos/tour/src"
  printf 'pub fn h<S: Real>(p: P) -> P2 {\n    p.map(S::from_computed)\n}\n' > "$1/demos/tour/src/main.rs"
}
plant_unlisted_other_crate() {
  mkdir -p "$1/crates/topo/src"
  printf 'pub fn h<T: Real>(t: f64) -> T {\n    T::from_computed(t)\n}\n' > "$1/crates/topo/src/lib.rs"
}
plant_stale() { sed -i 's/T::from_computed($/foot_t(/' "$1/$fixture_file"; }
plant_bad_class() { printf '%s\t1\tfine\tz\tit is fine\n' "$fixture_file" >> "$1/$FROM_COMPUTED_LIST"; }
plant_empty_reason() { printf '%s\t1\tdoor\tz\t\n' "$fixture_file" >> "$1/$FROM_COMPUTED_LIST"; }
plant_missing_row() { sed -i 's/owed(owed-row)/owed(no-such-row)/' "$1/$FROM_COMPUTED_LIST"; }
plant_closed_row() { printf -- '---\nid: owed-row\nstatus: closed\n---\n' > "$1/work/geom/owed-row.md"; }
plant_near_misses() {
  {
    printf '// T::from_computed(x.lo()) in a comment\n'
    printf 'pub const WHY: &str = "T::from_computed(x.lo())";\n'
    printf 'pub fn k<T: Real>() -> T { T::from_computedx(1.0) }\n'
    printf 'impl Real for X {\n    fn from_computed(x: f64) -> Self { X(x) }\n}\n'
  } >> "$1/$fixture_file"
}

from_computed_case() {
  local rule=$1; shift
  gate_selftest_case "$(gate_name) $rule: " "$@"
}

gate_selftest() {
  gate_selftest_clean
  gate_selftest_without_tool awk "could not build the statement view"
  from_computed_case UNLISTED plant_unlisted
  from_computed_case UNLISTED plant_unlisted_qualified
  from_computed_case UNLISTED plant_unlisted_literal
  from_computed_case UNLISTED plant_one_more_listed
  from_computed_case UNLISTED plant_unlisted_demo
  from_computed_case UNLISTED plant_unlisted_other_crate
  from_computed_case STALE plant_stale
  from_computed_case CLASS plant_bad_class
  from_computed_case CLASS plant_empty_reason
  from_computed_case CLASS plant_missing_row
  from_computed_case CLASS plant_closed_row
  gate_selftest_passes "a comment, a string literal, a longer name and a definition in a listed file" \
    plant_near_misses
  printf '%s selftest OK: passes a clean fixture (a door, a wrapped owed call, a point-free place step, a cfg(test) site); fires UNLISTED on a computed call, a qualified call, a literal, one site more than listed, a demo site and a site in another crate; STALE on a listed site that went away; CLASS on an unknown class, an empty reason, a missing and a closed work item; passes every near miss; and stays RED when awk cannot run\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
