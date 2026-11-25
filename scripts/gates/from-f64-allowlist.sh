#!/usr/bin/env bash
# from-f64-allowlist.sh — `Real::from_f64` is called by the doors a
# constant enters by, by a scalar's own impl, with a literal, or at a
# site a listed work item still owes. ONE home; ci.yml's `lint` job
# runs every gate in this directory.
#
# THE SUBJECT (`crates/geom-core/README.md`, DL5.2). `Real::from_f64`
# embeds an `f64` exactly, and at the symbolic scalar (`geom_core::Sym`)
# that embedding is a `Lit`: the claim that the number is the same at
# every valuation of the document's unknowns. A constant of the model
# is, and it enters by where it comes from, through a door whose
# signature takes no bare `f64`:
#
#   a number the source wrote   `Real::lit(geom_core::lit!(..))`
#   a count                     `Real::count(n)`, `Real::count_ratio(i, n)`
#   a band                      `geom_core::BandWidth::at`
#   an authored value           the document's parameter door
#                               (`editor_core::expr::authored`), and the
#                               programmatic author's (`pncad::real`)
#
# The first three take no `f64`. The two authored doors do, and check
# nothing, so THEIR CALLERS ARE LISTED TOO: every use of
# `expr::authored` in `editor-core` and of `pncad`'s `real` wherever it
# is in scope is a site (`<call authored>`, `<call real>`), counted per
# file against the list, and a new caller reds as a new bare call does.
#
# Anything else that reaches `from_f64` is one of the things DL5.2
# sorts elsewhere (a bound, a place, data built at `f64`, a solved
# value) or a constant not yet on its door, and is listed here against
# the work item that owns it until it lands. What CATCHES a laundered
# value is the re-valuation row,
# `crates/editor-core/tests/revalue_corpus.rs`.
#
# THE SCAN is the production code (`lib.sh`'s reader, `#[cfg(test)]`
# dropped) of every crate under `crates/*/src` and every demo under
# `demos/*/src`, read in the statement view so a call rustfmt wraps
# over lines is one record. Every `from_f64` as a word is a site (a
# call `T::from_f64(x)`, a qualified `<Interval as Real>::from_f64(x)`
# and a point-free `.map(T::from_f64)` alike) except a definition
# (`fn from_f64`).
#
# EXEMPT BY SHAPE, and nothing else: an argument built only from
# numeric literals (`0.5`, `-1e-3`, `2_f64`), SCREAMING_CASE constant
# paths (`f64::consts::PI`, `geom_core::UNIT_ROUNDOFF`), `as f64`,
# grouping parentheses and binary `+ - * /`. Each is a constant at
# compile time; each is a site `Real::lit` will take, and the gate
# prints how many are left. A dereference (`*LAZY`) and a call
# (`SCREAMING_FN()`) are not constant shapes and are listed. An argument
# the statement view cut at a `{` (a block, an `if`) is never exempt: it
# is keyed `<block> <prefix>` and listed.
#
# THE KEY of a listed site is its file, the `fn` it stands in (the last
# `fn` the file declares before it, `-` before any) and its argument's
# text with whitespace collapsed (`<fn>` for a point-free use); the
# entry carries how many such sites that `fn` holds. No line numbers:
# they move with every edit above them.
#
# THE CLASSES (the TSV's third column). Five are no debt. What the
# gate holds of each, and what stays a reviewer's label:
#
#   door          a door's one step to the scalar. HELD: the file, the
#                 function's name and the argument's text are one of
#                 FROM_F64_DOORS, and the entry counts one site; a door
#                 that takes an `f64` (the document's) is crate-private,
#                 the one public exception being the programmatic
#                 author's (FROM_F64_PUBLIC_DOORS);
#   scalar        a `Real` impl's own embed, forwarding to its value
#                 scalar's. HELD: the site stands in a method written
#                 `fn from_f64(x: f64) -> Self` (or `from_computed`),
#                 which no free function can be, and the last `impl`
#                 the file opened before it is `impl .. Real for ..`;
#   caller        a use of an authored door (`<call authored>`,
#                 `<call real>`). HELD: the count per file. A LABEL:
#                 that what each caller hands the door is authored;
#   concrete      a point of the concrete `Interval`, the enclosure
#                 arithmetic's own constructor, which is never `Sym` and
#                 carries no tangent. HELD: the call's receiver is
#                 spelled `Interval` (`Interval::from_f64`,
#                 `<Interval as Real>::from_f64`, with or without the
#                 `geom_core::` path). A LABEL: everything about the
#                 number; a computed bound passes;
#   fixture       test-support code compiled into the library. HELD: the
#                 file's path names `test_support`, or the function is
#                 one FROM_F64_FIXTURES names. A LABEL: that the code is
#                 behind `cfg(test)` or the `test-support` feature (the
#                 gate does not read the `cfg`), and what the number is;
#   owed(R)       a site the work item R owns: a place, a bound, data
#                 built at `f64`, an edge's parameter end, or a constant
#                 of one of the four sources that is not yet on its
#                 door. The hand-kept part of the list, and each entry
#                 goes when its site does.
#
# R owns its sites while its file has a `status:` line that is not
# `closed`: `open`, `dispatched`, and `parked` or `deferred` too (the
# debt is scheduled, not gone). An item with no status line owns nothing.
#
# WHAT REDS, each with its own tag:
#
#   UNLISTED  a site whose key is not on the list, or a `fn` holding
#             more such sites than its entry says;
#   STALE     an entry whose `fn` holds fewer sites than it says (a site
#             moved onto a door: drop or lower the entry in the same
#             change);
#   CLASS     an entry whose class is not one of the above, whose reason
#             is empty, or whose owed class names a work item that does
#             not own it (missing, no status line, or closed);
#   WHERE     a class that is no debt, listed where the gate holds it
#             may not stand, or a door taking an `f64` whose function is
#             public.
#
# KNOWN GAPS. What a token scan cannot see; the re-valuation row sees
# each wherever the corpus reaches it.
#   - The bare door is still a trait method: nothing in the type stops a
#     new caller, only this list. It closes when the literal sites have
#     moved onto `Real::lit` and the owed list is empty.
#   - A door that takes an `f64`: `editor_core::expr::authored` and
#     `pncad::real` admit whatever a listed caller hands them, and a
#     caller outside the scan (a user's program) is not listed. The
#     caller scan reads `authored` bare or as `expr::authored` in
#     `editor-core`, and `real` as `authoring::real`, `prelude::real`,
#     `pncad::real`, or bare in a file that imports it or a `pncad`
#     glob; a re-export under another name, and an alias's later calls,
#     are not read. A local closure named `real` in such a file reads as
#     a caller.
#   - The other doors, each admitting a computed number the type cannot
#     see: an integer cast from a computed float into `Real::count` or
#     `Real::count_ratio` (`(x * 1e9) as i64`), which is silent here;
#     a `Band` built or padded from a computed `f64` (`Band::new`,
#     `Band::padded`) into a `BandWidth`; a hand-written `impl Constant`
#     whose value reads the build environment (`env!`).
#   - `scalar` reads the last `impl` the file opened and not where it
#     closed (the view holds no braces): an inherent method of that
#     exact signature in an `impl` with no generics that the reader
#     takes for the same block is not told apart.
#   - A helper that passes an `f64` through to a listed site
#     (`Knots::lift(T::from_f64)`, `scale_pow2(k)`, `apart_along(.., pad,
#     ..)`): a new caller handing it a computed value adds no site.
#   - A name: a point-free use bound to a local (`let f = T::from_f64;`)
#     is one `<fn>` site and the calls through `f` are not read; one
#     site swapped for another in the same `fn` with the same argument
#     text keeps the count. A SCREAMING_CASE local
#     (`#[allow(non_snake_case)] let N = x.lo();`) reads as a constant
#     path.
#   - The receiver is text: a type alias of `Interval` is not read as
#     concrete, which cries wolf; a local type NAMED `Interval` would be,
#     which does not.
#   - A concrete `Interval` point is no claim at `Sym` or at a dual, and
#     the gate does not ask what the number was: a bound entered as a
#     point and then handed to generic code as `T = Interval` is not
#     seen.
#   - A `Lit` picked at run time from a table or a `const`-generic
#     marker (a selection among constants the source wrote); a count
#     past 2^53 in a wide integer (the nearest `f64`).
#   - `Real::from_computed` (`from-computed-sites.sh`'s subject), test
#     code, a `paste!`-built name and a macro body as expanded, and
#     every root outside `crates/*/src` and `demos/*/src`: `tools/`,
#     `benches/`, `examples/`, `build.rs`, and top-level crates such as
#     `interval-transcendentals/`.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

FROM_F64_LIST=scripts/gates/from-f64-sites.tsv
FROM_F64_RULES=(UNLISTED STALE CLASS WHERE)

from_f64_rule_message() {
  case "$1" in
    UNLISTED) printf '%s' "a from_f64 site that is neither a literal nor a listed site, or a new use of an authored door (expr::authored, pncad's real: read what it hands the door, then raise its file's caller entry). A number the source wrote enters through Real::lit(geom_core::lit!(..)), a count through Real::count, a band through BandWidth::at, an authored value through the document's parameter door; a place goes to a place-taking evaluator as an f64, a bound decides at f64, data is built at T. A site that cannot move yet goes on $FROM_F64_LIST as owed(<work item>)" ;;
    STALE) printf '%s' "a listed from_f64 site is gone (or fewer remain than listed): drop or lower its entry in $FROM_F64_LIST in the change that removed it" ;;
    CLASS) printf '%s' "an entry in $FROM_F64_LIST has an unknown class, an empty reason, or an owed class naming no work item under work/ that owns it (a status line, not closed)" ;;
    WHERE) printf '%s' "an entry in $FROM_F64_LIST is classed where its class may not stand: a door only as the one site FROM_F64_DOORS names by file, function and argument (crate-private if it takes an f64, but for FROM_F64_PUBLIC_DOORS), a scalar impl only in a fn from_f64 or fn from_computed under an impl of Real, a caller only as <call authored> or <call real>, a concrete point only where the receiver is spelled Interval, a fixture only in a test_support file or at a function FROM_F64_FIXTURES names; anything else is owed(<work item>)" ;;
  esac
}

# The sites, one `file<TAB>argument<TAB>fn<TAB>visibility<TAB>receiver`
# per line, over the statement view on stdin. The receiver is `Interval`
# where the call names that type, `-` anywhere else.
from_f64_sites() {
  gate_record_awk '
    {
      if (!gate_record_split($0)) next
      t = GR_TEXT
      # The `fn` a site stands in: the last one the file declared before
      # it, and whether that declaration is bare `pub` (public API) or
      # crate-private (`pub(crate)`, `pub(super)` or none).
      if (GR_FILE != file) { file = GR_FILE; fn = "-"; vis = "-"; imp = "-"; under = 0 }
      # The embed of a `Real` impl: the last `impl` the file opened is one
      # of `Real`, and the `fn` is the trait method as an impl writes it
      # (`fn from_f64(x: f64) -> Self`, which no free function can be).
      # The view holds no braces, so where an impl ends is not read.
      if (t ~ /(^|[^A-Za-z0-9_])impl([ \t]|<)/) under = (t ~ /Real[ \t]+for[ \t]/)
      if (t ~ /(^|[^A-Za-z0-9_])fn[ \t]/) imp = (under && t ~ /^[ \t]*fn[ \t]+(from_f64|from_computed)[ \t]*[(][a-z_]+:[ \t]*f64[)][ \t]*->[ \t]*Self[ \t]*$/) ? "real-impl" : "-"
      callers(t)
      if (match(t, /(^|[^A-Za-z0-9_])fn[ \t]+[A-Za-z_][A-Za-z0-9_]*/)) {
        decl = substr(t, RSTART, RLENGTH); head = substr(t, 1, RSTART)
        sub(/^.*fn[ \t]+/, "", decl); fn = decl
        vis = (head ~ /(^|[^A-Za-z0-9_])pub[ \t]*$/) ? "pub" : "private"
      }
      while (match(t, /(^|[^A-Za-z0-9_])from_f64([^A-Za-z0-9_]|$)/)) {
        pre = substr(t, 1, RSTART)
        s = RSTART + (substr(t, RSTART, 1) ~ /[A-Za-z0-9_]/ ? 0 : 1) + 8
        rest = substr(t, s)
        t = rest
        if (pre ~ /fn[ \t]+$/) continue
        recv = "-"
        if (pre ~ /(^|[^A-Za-z0-9_:])(geom_core::)?Interval::$/) recv = "Interval"
        if (pre ~ /<[ \t]*(geom_core::)?Interval[ \t]+as[ \t]+(geom_core::)?Real[ \t]*>::$/) recv = "Interval"
        sub(/^[ \t]+/, "", rest)
        if (substr(rest, 1, 1) != "(") { print GR_FILE "\t<fn>\t" fn "\t" vis "\t" recv "\t" imp; continue }
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
        print GR_FILE "\t" a "\t" fn "\t" vis "\t" recv "\t" imp
      }
    }
    # The uses of the two doors that take an `f64`, one site each, keyed
    # by file alone (`<call authored>`, `<call real>`).
    function callers(t,    s, pre, post, qual) {
      if (GR_FILE ~ /^crates\/editor-core\/src\//) {
        u = t
        while (match(u, /(^|[^A-Za-z0-9_])authored([^A-Za-z0-9_]|$)/)) {
          s = RSTART + (substr(u, RSTART, 1) ~ /[A-Za-z0-9_]/ ? 0 : 1)
          pre = substr(u, 1, s - 1); post = substr(u, s + 8); u = post
          if (pre ~ /fn[ \t]+$/ || pre ~ /[.][ \t]*$/) continue
          qual = (pre ~ /(^|[^A-Za-z0-9_])expr::$/)
          if (pre ~ /::$/ && !qual) continue
          if (qual || post ~ /^[ \t]*([(]|::<)/) print GR_FILE "\t<call authored>\t-\t-\t-\t-"
        }
      }
      if (GR_FILE == "crates/pncad/src/authoring.rs") reach[GR_FILE] = 1
      if (t ~ /(pncad::prelude|authoring)::[*]/ || t ~ /(authoring|prelude)::([{][^}]*[^A-Za-z0-9_])?real([^A-Za-z0-9_]|$)/) reach[GR_FILE] = 1
      u = t
      while (match(u, /(^|[^A-Za-z0-9_])real([^A-Za-z0-9_]|$)/)) {
        s = RSTART + (substr(u, RSTART, 1) ~ /[A-Za-z0-9_]/ ? 0 : 1)
        pre = substr(u, 1, s - 1); post = substr(u, s + 4); u = post
        if (pre ~ /fn[ \t]+$/ || pre ~ /[.][ \t]*$/) continue
        qual = (pre ~ /(^|[^A-Za-z0-9_])(authoring|prelude|pncad)::$/)
        if (pre ~ /::$/ && !qual) continue
        if (qual && t !~ /(^|[^A-Za-z0-9_])use[ \t]/) print GR_FILE "\t<call real>\t-\t-\t-\t-"
        else if (!qual && post ~ /^[ \t]*([(]|::<)/) bare[GR_FILE]++
      }
    }
    END {
      for (f in bare) if (f in reach) for (i = 0; i < bare[f]; i++) print f "\t<call real>\t-\t-\t-\t-"
    }'
}

# One token of an exempt argument, once `as f64`, the operators and the
# parentheses are spaces: a numeric literal or a constant path.
# Bracket classes, never `\.` or `\(`: the patterns reach `awk` through
# `-v`, which processes backslash escapes, and gawk and mawk do not agree
# on what is left (`certification-doors.sh`, CERT_GLOB_RE).
FROM_F64_TOKEN_RE='^([0-9][0-9_]*([.][0-9_]*)?([eE][0-9]+)?(_?f64)?|([A-Za-z_][A-Za-z0-9_]*::)*[A-Z][A-Z0-9_]*)$'
FROM_F64_CLASS_RE='^(door|scalar|caller|concrete|fixture|owed[(][A-Za-z0-9_.-]+[)])$'
# Where a class that is no debt may stand.
# A door is its file, its function and its argument, `:`-joined, the
# argument's parentheses and spaces written `_`.
FROM_F64_DOORS=(
  crates/geom-core/src/constant.rs:lit:c.0
  crates/geom-core/src/constant.rs:count:n.to_f64__
  crates/geom-core/src/constant.rs:count_ratio:num.to_f64___/_den.to_f64__
  crates/geom-core/src/predicate.rs:at:self.0
  crates/editor-core/src/expr.rs:authored:value
  crates/pncad/src/authoring.rs:real:x
)
FROM_F64_PUBLIC_DOORS=(
  crates/geom-core/src/predicate.rs:at
  crates/pncad/src/authoring.rs:real
)
FROM_F64_FIXTURE_PATH_RE='(^|/)test_support'
FROM_F64_FIXTURES=(
  crates/geom-brep/src/pcurve_cache.rs:iso_line_for_tests
  crates/profile/src/validate.rs:lift
  crates/sweep/src/loft.rs:f64_body_held_as_points
)

gate() {
  gate_require_crate_sources
  gate_production_sources
  gate_require_file "$FROM_F64_LIST"
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
  sites=$(printf '%s\n' "$view" | from_f64_sites) || status=$?
  [ "$status" -eq 0 ] || gate_reader_died_refusal "the site walk over the statement view" "$status" \
    "What it did not read is unknown."
  if [ "${FROM_F64_DUMP:-}" = 1 ]; then printf '%s\n' "$sites" | sort | uniq -c; exit 0; fi
  local verdicts
  status=0
  verdicts=$(printf '%s\n' "$sites" | awk -F'\t' \
    -v LIST="$FROM_F64_LIST" \
    -v TOKEN_RE="$FROM_F64_TOKEN_RE" \
    -v CLASS_RE="$FROM_F64_CLASS_RE" \
    -v DOORS="${FROM_F64_DOORS[*]}" \
    -v PUBLIC_DOORS="${FROM_F64_PUBLIC_DOORS[*]}" \
    -v FIXTURE_RE="$FROM_F64_FIXTURE_PATH_RE" \
    -v FIXTURES="${FROM_F64_FIXTURES[*]}" '
    function constant_shape(a,    k, n, tok, b) {
      if (a ~ /^</) return 0
      # A call, and a dereference (a `*` with no operand before it), are
      # not constant shapes.
      if (a ~ /[A-Za-z0-9_][ \t]*[(]/) return 0
      b = a; gsub(/[ \t]/, "", b)
      if (b ~ /(^|[-+*\/(])[*]/) return 0
      gsub(/[ \t]as[ \t]+f64/, " ", a)
      gsub(/[eE][-+]/, "e", a)
      gsub(/[-+*\/()]/, " ", a)
      n = split(a, tok, /[ \t]+/)
      for (k = 1; k <= n; k++) if (tok[k] != "" && tok[k] !~ TOKEN_RE) return 0
      return 1
    }
    BEGIN {
      n = split(DOORS, st, " "); for (i = 1; i <= n; i++) door[st[i]] = 1
      n = split(PUBLIC_DOORS, st, " "); for (i = 1; i <= n; i++) public_door[st[i]] = 1
      n = split(FIXTURES, st, " "); for (i = 1; i <= n; i++) fixture[st[i]] = 1
      while ((getline line < LIST) > 0) {
        if (line ~ /^#/ || line ~ /^[ \t]*$/) continue
        k = split(line, col, "\t")
        key = col[1] "\t" col[4] "\t" col[5]
        want[key] += col[2] + 0
        cls = col[3]; why = (k >= 6) ? col[6] : ""
        if (cls !~ CLASS_RE || why ~ /^[ \t]*$/) { print "CLASS " line; continue }
        class[key] = cls; entry[key] = line
        # A class that is no debt says where it may stand, and the gate
        # holds it there.
        at = col[1] ":" col[4]
        item = col[5]; gsub(/[() ]/, "_", item)
        if (cls == "door" && (!((at ":" item) in door) || col[2] != 1)) print "WHERE " line " (not one site of a door the gate names by file, function and argument: FROM_F64_DOORS)"
        if (cls == "scalar" && col[4] != "from_f64" && col[4] != "from_computed") print "WHERE " line " (a scalar impl stands only in its own fn from_f64 or fn from_computed)"
        if (cls == "caller" && col[5] !~ /^<call (authored|real)>$/) print "WHERE " line " (a caller entry is <call authored> or <call real>)"
        if (cls != "caller" && col[5] ~ /^<call /) print "WHERE " line " (a use of an authored door is classed caller)"
        if (cls == "caller") { callers_n += col[2]; listed_n -= col[2] }
        if (cls == "fixture" && col[1] !~ FIXTURE_RE && !(at in fixture)) print "WHERE " line " (a fixture stands only in a test_support file or at a function FROM_F64_FIXTURES names)"
        if (match(cls, /\(.*\)/)) {
          row = substr(cls, RSTART + 1, RLENGTH - 2)
          print "OWED " row "\t" line
          owed_n += col[2]
        }
        listed_n += col[2]
      }
    }
    NF >= 5 {
      if (constant_shape($2)) { lit++; next }
      key = $1 "\t" $3 "\t" $2
      have[key]++
      if (class[key] == "door" && $4 == "pub" && !(($1 ":" $3) in public_door)) print "WHERE " $1 ": " $3 " is public: a door that takes an f64 is private to its crate"
      if (class[key] == "scalar" && $6 != "real-impl") print "WHERE " entry[key] " (a scalar impl stands only under an impl of Real)"
      if (class[key] == "concrete" && $5 != "Interval") print "WHERE " entry[key] " (a concrete point stands only where the receiver is spelled Interval)"
    }
    END {
      for (k in have) if (have[k] > want[k] + 0) {
        split(k, p, "\t"); print "UNLISTED " p[1] ": " p[2] ": from_f64(" p[3] ") x" have[k] " (listed " want[k] + 0 ")"
      }
      for (k in want) if (want[k] > have[k] + 0) {
        split(k, p, "\t"); print "STALE " p[1] ": " p[2] ": from_f64(" p[3] ") listed x" want[k] ", found " have[k] + 0
      }
      printf "COUNT %d %d %d %d\n", lit + 0, listed_n + 0, owed_n + 0, callers_n + 0
    }') || status=$?
  [ "$status" -eq 0 ] || gate_reader_died_refusal "the list walk" "$status" "What it did not compare is unknown."
  # An owed class names a work item that still owns the site: an item
  # file under `work/` with a `status:` line that is not `closed`.
  local owed row entry item
  owed=$(printf '%s\n' "$verdicts" | sed -n 's/^OWED //p' | sort -u)
  while IFS=$'\t' read -r row entry; do
    [ -n "$row" ] || continue
    item=$(find work -name "$row.md" -type f 2>/dev/null | head -1)
    if [ -z "$item" ]; then
      verdicts=$(printf '%s\nCLASS %s (no work item %s)' "$verdicts" "$entry" "$row")
    elif ! grep -q '^status:' "$item"; then
      verdicts=$(printf '%s\nCLASS %s (work item %s has no status line)' "$verdicts" "$entry" "$row")
    elif grep -q '^status: closed' "$item"; then
      verdicts=$(printf '%s\nCLASS %s (work item %s is closed)' "$verdicts" "$entry" "$row")
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
  local lit listed owed_n callers_n
  read -r lit listed owed_n callers_n < <(printf '%s\n' "$verdicts" | sed -n 's/^COUNT //p')
  GATE_SCAN_FILES=${#files[@]}
  gate_ok "every from_f64 in crates/*/src and demos/*/src production code is a literal ($lit, each one Real::lit will take) or a listed site ($listed): the doors, a scalar's impl, a concrete Interval point, a fixture, and $owed_n sites still owed to open work items; and the two authored doors have $callers_n listed uses"
}

# --- SELF-TEST ---------------------------------------------------------
#
# The clean fixture is a tree of its own: a door, a scalar impl, a
# concrete point, a fixture, owed sites, literals, and the work item an
# owed class names.
fixture_file=crates/geom/src/lib.rs
door_file=crates/geom-core/src/constant.rs
authored_file=crates/editor-core/src/expr.rs
support_file=crates/geom/src/test_support.rs

gate_plant_clean() {
  gate_plant_clean_sources "$1"
  mkdir -p "$1/crates/geom/src" "$1/crates/geom-core/src" "$1/crates/editor-core/src" "$1/scripts/gates" "$1/work/geom"
  printf 'pub(crate) fn count<T: Real, N: Count>(n: N) -> T {\n    T::from_f64(n.to_f64())\n}\n' > "$1/$door_file"
  printf 'pub(crate) fn authored<T: Real>(value: f64) -> T {\n    T::from_f64(value)\n}\nfn leaf<T: Real>(q: Q) -> T {\n    authored(q.value) + q.coord.authored() + Coord::authored(q)\n}\n' > "$1/$authored_file"
  printf 'pub fn lifted<T: Real>(p: [f64; 3]) -> [T; 3] {\n    p.map(T::from_f64)\n}\n' > "$1/$support_file"
  {
    printf 'pub fn f<T: Real>(foot: P) -> T {\n'
    printf '    let a = T::from_f64(0.5) + T::from_f64(f64::consts::PI) + T::from_f64(2.0 * K as f64);\n'
    printf '    let b = T::from_f64(\n        foot.x,\n    );\n'
    printf '    let _ = v.map(T::from_f64);\n'
    printf '    a + b\n}\n'
    printf 'impl Real for X {\n    fn from_f64(x: f64) -> Self {\n        X(Y::from_f64(x))\n    }\n}\n'
    printf 'fn hull(lo: f64) -> Interval {\n    Interval::from_f64(lo) + <Interval as Real>::from_f64(lo) + <geom_core::Interval as geom_core::Real>::from_f64(lo)\n}\n'
    printf '#[cfg(test)]\nmod tests {\n    fn t() { let _ = T::from_f64(x.lo()); }\n}\n'
  } > "$1/$fixture_file"
  {
    printf '# file\tcount\tclass\tfn\targument\treason\n'
    printf '%s\t1\tdoor\tcount\tn.to_f64()\tthe count door\n' "$door_file"
    printf '%s\t1\tdoor\tauthored\tvalue\tthe parameter door\n' "$authored_file"
    printf '%s\t1\tcaller\t-\t<call authored>\ta quantity literal\n' "$authored_file"
    printf '%s\t1\tfixture\tlifted\t<fn>\ta fixture lift\n' "$support_file"
    printf '%s\t1\towed(owed-row)\tf\tfoot.x\ta foot, owed\n' "$fixture_file"
    printf '%s\t1\towed(owed-row)\tf\t<fn>\ta point-free lift, owed\n' "$fixture_file"
    printf '%s\t1\tscalar\tfrom_f64\tx\tthe impl forwards\n' "$fixture_file"
    printf '%s\t3\tconcrete\thull\tlo\ta point of the concrete Interval\n' "$fixture_file"
  } > "$1/$FROM_F64_LIST"
  printf -- '---\nid: owed-row\nstatus: open\n---\n' > "$1/work/geom/owed-row.md"
}

plant_unlisted() { printf 'pub fn g<T: Real>(r: R) -> T {\n    T::from_f64(r.lo())\n}\n' >> "$1/$fixture_file"; }
plant_unlisted_wrapped() {
  printf 'pub fn g<T: Real>(r: R) -> T {\n    T::from_f64(\n        r.hi(),\n    )\n}\n' >> "$1/$fixture_file"
}
plant_unlisted_point_free() { printf 'pub fn g() {\n    let _ = w.map_scalar(T::from_f64);\n}\n' >> "$1/$fixture_file"; }
plant_unlisted_qualified() { printf 'pub fn g(r: R) -> Interval {\n    <Interval as Real>::from_f64(r.root)\n}\n' >> "$1/$fixture_file"; }
plant_one_more_listed() { printf 'pub fn g<T: Real>(foot: P) -> T {\n    T::from_f64(foot.x)\n}\n' >> "$1/$fixture_file"; }
plant_one_more_in_listed_fn() { sed -i 's/^pub fn f<T: Real>(foot: P) -> T {$/&\n    let _ = T::from_f64(foot.x);/' "$1/$fixture_file"; }
plant_unlisted_other_crate() {
  mkdir -p "$1/crates/topo/src"
  printf 'pub fn g<T: Real>(u1: f64, u0: f64) -> T {\n    T::from_f64(u1 - u0)\n}\n' > "$1/crates/topo/src/lib.rs"
}
plant_unlisted_unwalked_crate() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn g<T: Real>(x: R) -> T {\n    T::from_f64(x.lo())\n}\n' > "$1/crates/planted/src/lib.rs"
}
plant_unlisted_demo() {
  mkdir -p "$1/demos/tour/src"
  printf 'pub fn g<S: Real>(r: f64) -> S {\n    S::from_f64(r)\n}\n' > "$1/demos/tour/src/main.rs"
}
plant_unlisted_block() { printf 'pub fn g<T: Real>(r: R) -> T {\n    T::from_f64({ r.lo() })\n}\n' >> "$1/$fixture_file"; }
plant_unlisted_block_constant_prefix() {
  printf 'pub fn g<T: Real>(r: R) -> T {\n    T::from_f64(SCALE * {\n        let m = r.lo();\n        m\n    })\n}\n' >> "$1/$fixture_file"
}
plant_relabel() {
  printf 'pub fn h<T: Real>(r: R) -> T {\n    T::from_f64(r.root)\n}\n' >> "$1/$fixture_file"
  printf '%s\t1\t%s\th\tr.root\tsays it is fine\n' "$fixture_file" "$2" >> "$1/$FROM_F64_LIST"
}
plant_relabel_door() { plant_relabel "$1" door; }
plant_unlisted_deref() { printf 'pub fn g<T: Real>() -> T {\n    T::from_f64(*LAZY)\n}\n' >> "$1/$fixture_file"; }
plant_unlisted_call() { printf 'pub fn g<T: Real>() -> T {\n    T::from_f64(-SCREAMING_FN() * 2.0)\n}\n' >> "$1/$fixture_file"; }
plant_free_scalar() {
  printf 'pub fn from_computed<T: Real>(x: f64) -> T {\n    T::from_f64(x)\n}\n' >> "$1/$fixture_file"
  printf '%s\t1\tscalar\tfrom_computed\tx\tsays it is an impl\n' "$fixture_file" >> "$1/$FROM_F64_LIST"
}
plant_second_door() {
  printf 'impl Other {\n    pub fn count<T: Real>(x: f64) -> T {\n        T::from_f64(x)\n    }\n}\n' >> "$1/$door_file"
  printf '%s\t1\tdoor\tcount\tx\tsays it is the door\n' "$door_file" >> "$1/$FROM_F64_LIST"
}
plant_new_authored_caller() {
  printf 'pub fn g<T: Real>(x: f64) -> T {\n    crate::expr::authored::<T>(x.sqrt())\n}\n' > "$1/crates/editor-core/src/ratio.rs"
}
plant_one_more_authored_caller() { printf 'fn twig<T: Real>(x: f64) -> T {\n    let f = authored::<T>;\n    f(x)\n}\n' >> "$1/$authored_file"; }
plant_new_real_caller() {
  mkdir -p "$1/demos/tour/src"
  printf 'use pncad::prelude::*;\npub fn g<S: Real>(r: f64) -> S {\n    real(r.sqrt())\n}\n' > "$1/demos/tour/src/main.rs"
}
plant_qualified_real_caller() { printf 'pub fn g<T: Real>(r: f64) -> T {\n    pncad::authoring::real(r)\n}\n' >> "$1/$fixture_file"; }
plant_real_namesake() { printf 'pub fn g(r: f64) -> P {\n    let real = |x: f64| P(x);\n    real(r) + other.real(r)\n}\n' >> "$1/$fixture_file"; }
plant_relabel_scalar() { plant_relabel "$1" scalar; }
plant_relabel_concrete() { plant_relabel "$1" concrete; }
plant_relabel_fixture() { plant_relabel "$1" fixture; }
plant_concrete_beside_generic() { sed -i 's/^fn hull(lo: f64) -> Interval {$/&\n    let _ = T::from_f64(lo);/; s/\t3\tconcrete\thull/\t4\tconcrete\thull/' "$1/$fixture_file" "$1/$FROM_F64_LIST"; }
plant_public_door() { sed -i 's/^pub(crate) fn authored/pub fn authored/' "$1/$authored_file"; }
plant_stale() { sed -i 's/T::from_f64($/foot_t(/' "$1/$fixture_file"; }
plant_bad_class() { printf '%s\t1\tconstant\tf\tz\tit is a constant\n' "$fixture_file" >> "$1/$FROM_F64_LIST"; }
plant_empty_reason() { printf '%s\t1\tdoor\tcount\tz\t\n' "$door_file" >> "$1/$FROM_F64_LIST"; }
plant_missing_row() { sed -i 's/owed(owed-row)/owed(no-such-row)/' "$1/$FROM_F64_LIST"; }
plant_closed_row() { printf -- '---\nid: owed-row\nstatus: closed\n---\n' > "$1/work/geom/owed-row.md"; }
plant_statusless_row() { printf -- '---\nid: owed-row\n---\n' > "$1/work/geom/owed-row.md"; }
plant_parked_row() { printf -- '---\nid: owed-row\nstatus: parked\n---\n' > "$1/work/geom/owed-row.md"; }
plant_near_misses() {
  {
    printf '// T::from_f64(x.lo()) in a comment\n'
    printf 'pub const WHY: &str = "T::from_f64(x.lo())";\n'
    printf 'pub fn k<T: Real>() -> T { T::from_f64x(1.0) + T::from_f64_bits(2) }\n'
    printf 'impl Real for Z {\n    fn from_f64(x: f64) -> Self { Z(x) }\n}\n'
    printf 'pub fn m<T: Real>() -> T { T::from_f64(-2.5e-3) + T::from_f64(3_f64) + T::from_f64(-K::MAX_TURN) }\n'
  } >> "$1/$fixture_file"
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
  from_f64_case UNLISTED plant_unlisted_qualified
  from_f64_case UNLISTED plant_one_more_listed
  from_f64_case UNLISTED plant_one_more_in_listed_fn
  from_f64_case UNLISTED plant_unlisted_other_crate
  from_f64_case UNLISTED plant_unlisted_unwalked_crate
  from_f64_case UNLISTED plant_unlisted_demo
  from_f64_case UNLISTED plant_unlisted_block
  from_f64_case UNLISTED plant_unlisted_block_constant_prefix
  from_f64_case UNLISTED plant_unlisted_deref
  from_f64_case UNLISTED plant_unlisted_call
  from_f64_case UNLISTED plant_new_authored_caller
  from_f64_case UNLISTED plant_one_more_authored_caller
  from_f64_case UNLISTED plant_new_real_caller
  from_f64_case UNLISTED plant_qualified_real_caller
  from_f64_case WHERE plant_free_scalar
  from_f64_case WHERE plant_second_door
  from_f64_case WHERE plant_relabel_door
  from_f64_case WHERE plant_relabel_scalar
  from_f64_case WHERE plant_relabel_concrete
  from_f64_case WHERE plant_relabel_fixture
  from_f64_case WHERE plant_concrete_beside_generic
  from_f64_case WHERE plant_public_door
  from_f64_case STALE plant_stale
  from_f64_case CLASS plant_bad_class
  from_f64_case CLASS plant_empty_reason
  from_f64_case CLASS plant_missing_row
  from_f64_case CLASS plant_closed_row
  from_f64_case CLASS plant_statusless_row
  gate_selftest_passes "a parked work item still owns its site" plant_parked_row
  gate_selftest_passes "a closure named real, and a method of that name, in a file that imports no pncad door" plant_real_namesake
  gate_selftest_passes "a comment, a string literal, longer names, a definition, literals and a SCREAMING_CASE constant in a listed file" \
    plant_near_misses
  printf '%s selftest OK: passes a clean fixture (a door, the parameter door and its one listed caller beside two namesakes, a scalar impl, three spellings of a concrete Interval point, a fixture lift, a wrapped and a point-free owed call, literals, a cfg(test) site) and a parked owner; fires UNLISTED on a computed call, one wrapped over lines, a point-free use, a qualified call, a site in another fn with a listed argument, one more in a listed fn, a site in another crate, in a crate the old list did not walk and in a demo, and a block argument whether its prefix is empty or reads as a constant; a dereference and a call that read as constants, a new use of expr::authored in another file, one more (an alias) in a listed file, a bare use of real under a pncad glob and a qualified one; WHERE on a site relabelled door, scalar, concrete or fixture where that class may not stand, on a free fn from_computed labelled scalar, on a second fn with a door name labelled door, on a generic call sharing a concrete entry, and on a public door that takes an f64; STALE on a listed site that went away; CLASS on the retired class constant, an empty reason, an owed class naming no work item, a closed one and one with no status; passes every near miss; and stays RED when awk cannot run\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
