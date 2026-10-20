#!/usr/bin/env bash
# reporting-margin-door.sh — `MarginDiag::diagnostic_f64_for_error_text`,
# the reporting margin's one door to its numbers, is called from the
# ALLOWLISTED production sites only, at the count each pins. ONE home;
# ci.yml's `lint` job runs every gate in this directory.
#
# WHY IT EXISTS. Every outcome of the classify seam carries the margin it
# was decided on (`geom_core::MarginDiag`), for error reporting only
# (`geom_core/src/real.rs`'s `Bounds` scope rule, clause 2, as Ev ruled
# on `[ev]` PR 3402: "structurally obvious that it's not ok to use this
# number for anything but error reporting"). The type offers no number a
# comparison could read in passing — no variant, no field, no
# `PartialOrd`, no conversion — and computes the recourse wording
# itself. The one spelling that reaches the `f64`s is the door this gate
# counts, so a decision taken on the number is a named call at a file on
# the list below, and a new one is a red here.
#
# EQUALITY IS THE OTHER ROUTE, and it is counted too. The type derives
# `PartialEq` (identity of the reading, which every error enum carrying
# it derives in turn, for tests that pin a payload), so a production
# site could decide "exactly this number" by comparing against a
# reading it minted itself. A comparand has to be minted with
# `MarginDiag::value` or `MarginDiag::enclosure`, so the production MINT
# sites are a second list here: a new one is a red, and a review reads
# what it mints for.
#
# THE CURABILITY VERDICT IS HELD HERE TOO, rather than by the type.
# `Indeterminate::terminal_sliver` tells the subdivision driver that
# subdividing is futile; its fields are public, so a production literal
# `terminal_sliver: true` is pinned to no site at all.
#
# THE SENTENCE IS A THIRD ROUTE, and it is counted too.
# `MarginDiag::sized_recourse` chooses its words from the number, so a
# caller that asked it and searched the sentence for "tighten" would
# read the margin's side through it. So do the import door's two
# (D4 ¶1, as Ev ruled on `[ev]` PR 3380):
# `MarginDiag::sized_recourse_in_file` withholds the offer to tighten
# alone for a size whose nearer end lies at or below the file's ε_in,
# and `FileCoincidence::miss_recourse_in_file` names the set-ε-to-ε_in
# stopgap for a miss within it — a comparison against the margin that
# picks the words and nothing else, so a caller searching either
# sentence would read whether the margin lies within ε_in. Their
# production call sites are a third list here: the sized-decision table
# that owns the endings.
#
# THE DEFECT CHOICE IS A FOURTH LIST. A sized decision's caller may say
# a reading certainly on the refused side contradicts the stored data
# its decision reads (`RefusedReading::Defect`, with the culprit
# `geom_brep::recourse::ReadAt::culprit` names), which turns that reading's
# ending into a defect ending;
# and the sign-certain arm asks the refused class's ending without a
# reading (`SizedWords::sign_certain_recourse`); and whether a reading is
# certainly on the refused side at all (`MarginDiag::shows_no_size`, and
# `SizedDecision::certainly_refused` over it, crate-private to geom-brep)
# is the class those choices end, read without a sentence. All belong to
# the one table that picks how a definite reading ends from its class
# and the decision's refused side, so the routes there cannot drift: a
# spelling of any anywhere else is a second table. The list counts the variant's
# spellings and the method's calls, not what the site does with them.
#
# THE CULPRIT CHOICE IS A FIFTH LIST. Which culprit a defect ending names
# is chosen once, by the door (`geom_brep::recourse::ReadAt::culprit`,
# and the run doors' `Reading::culprit` it reads); only the import door
# names the file alone (`StoredCulprit::File`). A call to either
# chooser, or a spelling of that culprit, anywhere else chooses the
# culprit a second time: a site that names the wrong culprit for its
# door is invisible to every ending test that reads its words.
#
# WHAT A LISTED SITE OWES, and the gate checks none of it — it checks
# only the file and the count, so the list is where the argument lives:
# a door site puts the numbers into error text or a payload another
# renderer prints, and nothing branches on them; a mint site reports
# what its own classification saw; a sentence site returns the sentence
# as an ending and searches nothing in it. A review that finds the door or a
# mint anywhere else, or an entry whose site compares the numbers, has
# found the misuse.
#
# WHAT IS SKIPPED: test modules, test-only mounts, and the modules a
# crate mounts under `#[cfg(any(test, feature = "test-support"))]`
# (`lib.sh`'s test-only resolver refuses `any(…)` on purpose, so this
# gate resolves that one spelling itself): a test or a test-support
# sample pinning a payload's numbers decides nothing in a build.
#
# WHAT THIS GATE DOES NOT COVER, stated rather than implied. It reads
# production source text, so:
#   * a call reached through a macro, or a re-export or `use … as`
#     alias under another name, is invisible to it;
#   * so is a glob import of the stored-contradiction choice:
#     `use geom_core::RefusedReading::*;` followed by a bare
#     `Defect` escapes the definite-ending count, which
#     matches the variant only under its enum's name; the same holds of
#     a bare `File` under a glob import of `StoredCulprit`, and of
#     `DAMAGED_FILE_ENDING` spelled directly, which names the file
#     without choosing a culprit;
#   * so is a path spelled with whitespace inside it —
#     `MarginDiag :: value(`, or the path split across lines — since
#     the matcher reads each record's text as written;
#   * rendering the reading as text — `Display`, `LowerExp`, or the
#     derived `Debug` every payload carries — and parsing the text back
#     is a door it cannot see, as obviously wrong as it is long;
#   * a sentence asked through a public wrapper is invisible: a caller
#     searching the import door's text (`geom_brep::certify::recourse`,
#     `SizedDecision::recourse` or `topo::EulerOpError::render` read at
#     `ReadAt::File`) with `.contains(..)` reads the margin's side as
#     surely as one asking the sentence here, under a name the list does
#     not count;
#   * EQUALITY AGAINST A COMPARAND IT DID NOT MINT is invisible: a site
#     holding two readings the classifier minted — `sign_within(..)?`'s
#     or `decide_reported(..)`'s `.margin`, an escalation's — can ask
#     whether they are the same reading without writing a mint. What it
#     learns is identity of two classified quantities, not a threshold;
#     a review that sees `==` between two margins has found the misuse.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

DEFINITION_SUBJECT='the file that DEFINES MarginDiag::diagnostic_f64_for_error_text, where a mention is the definition and not a call'
DEFINITION_HOMES=(
  crates/geom-core/src/predicate.rs
)

# `path count why` — one entry per file that may call the door, with the
# number of calls it makes.
DOOR_ALLOWLIST=(
  'crates/pncad-py/src/escalation.rs 1 the Python escalation payload: the numbers become the exception fields margin, margin_low and margin_high'
)
DOOR_RE='diagnostic_f64_for_error_text'

# `path count why` — one entry per production file outside the
# definition home that mints a valued reading.
MINT_ALLOWLIST=(
  'crates/geom-core/src/interval.rs 1 the interval classifier reports the enclosure it classified'
  'crates/geom-core/src/spline/knot_values.rs 1 a knot run order the structure refuses undecided (C6) reports the step enclosure it read, in the refusal and nothing else'
  'crates/geom-core/src/spline/weights.rs 1 a weight whose sign the structure refuses undecided (C6) reports the weight enclosure it read, in the refusal and nothing else'
  'crates/geom-core/src/spline/plan_inputs.rs 1 a knot parameter whose place over a wide breakpoint the structure refuses undecided (C6 S5b) reports its distance to the breakpoint'"'"'s bracket, in the refusal and nothing else'
  'crates/sweep/src/skin.rs 1 an arc sweep decision the structure refuses undecided (C6) reports the sweep'"'"'s distance to the breakpoint it read, in the refusal and nothing else'
  'crates/sweep/src/blend/battery.rs 2 the blend payload reports its companion quantities as its own scalar reads them (the M5 PR 12 seam)'
  'crates/topo/src/boolean/recl.rs 1 a grazing bisector between definite keys rides a record code that keeps no margin, so its On reports the zero band (residue on hand-minted-indeterminate-ratchet-gate)'
  'crates/topo/src/chart_region.rs 1 a definite deduction that cannot certify its outcome echoes the value it classified'
)
# `MarginDiag>?::` so the qualified-path form `<…MarginDiag>::value`
# is one spelling with the plain one.
MINT_RE='MarginDiag>?::(value|enclosure)([^A-Za-z0-9_]|$)'

# `path count why` — one entry per production file outside the
# definition home that asks the margin for a sized recourse sentence, or
# for either of the import door's.
SIZED_ALLOWLIST=(
  'crates/geom-brep/src/recourse.rs 3 the sized-decision table: a sized decision ends its Zero and Undecided arms in one sentence at a build or at rest and in the import door one, and a residual ends every arm in the one door miss sentence, each returned whole'
)
SIZED_RE='(sized_recourse|sized_recourse_in_file|miss_recourse_in_file)([^A-Za-z0-9_]|$)'

# `path count why` — production files outside the definition home that
# spell the stored-contradiction choice or ask the sign-certain ending.
DEFINITE_ALLOWLIST=(
  'crates/geom-brep/src/recourse.rs 4 the sized-decision table: SizedDecision::words picks the stored-contradiction choice once for every route, its culprit named by ReadAt::culprit, the sign-certain arm asks the refused class ending through those words, and SizedDecision::certainly_refused (its definition, for the doors that refuse a span as their input) reads the class once through MarginDiag::shows_no_size'
  'crates/geom-brep/src/certify.rs 1 every public certification and re-certification door asks whether a span check refusal is certainly on its refused side, to refuse it as the door input (StatedSpan::of) rather than end it'
)
DEFINITE_RE='(RefusedReading::Defect|sign_certain_recourse|shows_no_size|certainly_refused)([^A-Za-z0-9_]|$)'

# `path count why` — production files outside the definition home that
# call a culprit chooser, or spell the file culprit.
CULPRIT_ALLOWLIST=(
  'crates/geom-brep/src/recourse.rs 8 the culprit chooser: the definitions of ReadAt::culprit and Reading::culprit, ReadAt::culprit reading the run door and naming the file at the import door, defect_ending reading the door, unreadable_margin_note reading the door and naming the file note at the import door, and SizedDecision::words for a stored contradiction or an unminted side'
)
CULPRIT_RE='(culprit\(|StoredCulprit::File([^A-Za-z0-9_]|$))'

# `path count why` — production files that may write the classifier's
# verdict `terminal_sliver: true` on an escalation of their own. None:
# only the interval classifier decides that an escalation's subdivision
# is futile, inside the definition home's crate (`interval.rs` computes
# the flag, and never spells the literal). The field is public like its
# neighbours, so this list is what holds it — a forged `true` on a
# straddle would make the subdivision driver refuse terminally.
FORGE_ALLOWLIST=()
FORGE_RE='terminal_sliver[[:space:]]*:[[:space:]]*true'

# The `mod` files a crate mounts under
# `#[cfg(any(test, feature = "test-support"))]`, one path per line: the
# attribute, any further attributes, then `mod name;`, resolved beside
# the declaring file as `name.rs` or `name/mod.rs`.
test_support_mounts() {
  local f
  for f in "$@"; do
    awk -v F="$f" '
      /^[[:space:]]*#\[cfg\(any\(test, feature = "test-support"\)\)\]/ { armed = 1; next }
      armed && /^[[:space:]]*#\[/ { next }
      armed && match($0, /^[[:space:]]*(pub(\([a-z]+\))? )?mod [a-z_][a-z0-9_]*;/) {
        name = $0; sub(/^.*mod /, "", name); sub(/;.*$/, "", name)
        dir = F; sub(/[^\/]*$/, "", dir)
        base = F; sub(/^.*\//, "", base)
        if (base != "lib.rs" && base != "mod.rs" && base != "main.rs") {
          stem = base; sub(/\.rs$/, "", stem); dir = dir stem "/"
        }
        print dir name ".rs"; print dir name "/mod.rs"
      }
      { armed = 0 }
    ' "$f"
  done
}

# check_list WHAT RE ENTRY... — the hits of RE over production code,
# outside the definition home, against the `path count why` entries.
# Prints the offending records, cut to 200 columns, and returns 1
# (outside) or 2 (a moved count); 0 when every hit is on the list at
# its count. On 1, CHECK_LIST_FIRED holds the names RE matched in the
# offending records, read before the cut, so a call past column 200
# is still named.
check_list() {
  local what=$1 re=$2
  shift 2
  local full hits
  full=$(gate_rust_code --skip-cfg-test "${GATE_SCAN[@]}" \
    | gate_grep -E "$re" \
    | gate_grep -vE "$(gate_record_anchor_any "${DEFINITION_HOMES[@]}")")
  hits=$(printf '%s\n' "$full" | cut -c1-200)
  CHECK_LIST_FIRED=
  local entry path want why have bad=
  local -a listed=()
  for entry in ${@+"$@"}; do
    read -r path want why <<<"$entry"
    listed+=("$path")
    [ -f "$path" ] || bad+="  [$path]: the allowlisted file is gone"$'\n'
    have=$(printf '%s\n' "$hits" | gate_grep -cE "$(gate_record_anchor "$path")" || true)
    if [ "$have" != "$want" ]; then
      bad+="  [$path]: $have $what(s), the entry pins $want ($why)"$'\n'
    fi
  done
  local outside full_outside
  if [ "${#listed[@]}" -gt 0 ]; then
    outside=$(printf '%s\n' "$hits" | gate_grep -vE "$(gate_record_anchor_any "${listed[@]}")" || true)
    full_outside=$(printf '%s\n' "$full" | gate_grep -vE "$(gate_record_anchor_any "${listed[@]}")" || true)
  else
    outside=$hits
    full_outside=$full
  fi
  if [ -n "$outside" ]; then
    printf '%s\n' "$outside"
    CHECK_LIST_FIRED=$(printf '%s\n' "$full_outside" | gate_grep -oE "$re" \
      | sed -E 's/[^A-Za-z0-9_]$//' | sort -u | paste -sd, - | sed 's/,/, /g')
    return 1
  fi
  if [ -n "$bad" ]; then
    printf '%s' "$bad"
    return 2
  fi
}

gate() {
  gate_require_crate_sources
  gate_production_sources
  gate_require_homes "$DEFINITION_SUBJECT" "${DEFINITION_HOMES[@]}"
  local -a support=()
  mapfile -t support < <(test_support_mounts "${GATE_PRODUCTION_FILES[@]}")
  GATE_SCAN=()
  local f m skip
  for f in "${GATE_PRODUCTION_FILES[@]}"; do
    skip=false
    for m in ${support[@]+"${support[@]}"}; do
      if [ "$f" = "$m" ]; then skip=true; break; fi
    done
    [ "$skip" = true ] || GATE_SCAN+=("$f")
  done
  local rc=0
  check_list call "$DOOR_RE" "${DOOR_ALLOWLIST[@]}" || rc=$?
  case $rc in
    1) gate_error "$(gate_name): a call to MarginDiag::diagnostic_f64_for_error_text outside the allowlisted sites (this file's header says what a site owes). The reporting margin is for error text only; a comparison on its numbers is a decision the classifier did not take. Render it through its own Display, or add the site here with its reason"
       exit 1 ;;
    2) gate_error "$(gate_name): an allowlisted site's call count moved. A site that GAINED a call reads the numbers somewhere nobody argued; one that LOST one has an entry claiming a read it no longer makes. Move the pin in the change that carries the argument"
       exit 1 ;;
  esac
  check_list mint "$MINT_RE" "${MINT_ALLOWLIST[@]}" || rc=$?
  case $rc in
    1) gate_error "$(gate_name): a valued MarginDiag minted outside the allowlisted sites (this file's header says what a site owes). A reading minted in production code is a comparand an equality can decide on; report what the classifier saw instead, or add the site here with its reason"
       exit 1 ;;
    2) gate_error "$(gate_name): an allowlisted mint site's count moved. Move the pin in the change that carries the argument"
       exit 1 ;;
  esac
  check_list verdict "$FORGE_RE" ${FORGE_ALLOWLIST[@]+"${FORGE_ALLOWLIST[@]}"} || rc=$?
  case $rc in
    1) gate_error "$(gate_name): terminal_sliver: true written outside the classifier (this file's header says why). Whether subdivision is futile is the interval classifier's verdict, recorded when it mints the escalation; an escalation minted anywhere else carries false"
       exit 1 ;;
  esac
  check_list sentence "$SIZED_RE" "${SIZED_ALLOWLIST[@]}" || rc=$?
  case $rc in
    1) gate_error "$(gate_name): $CHECK_LIST_FIRED asked outside the allowlisted sites (this file's header says what a site owes). Its sentence is chosen from the number, so reading it is reading the margin; end a sized decision through geom_brep::recourse::SizedDecision instead, or add the site here with its reason"
       exit 1 ;;
    2) gate_error "$(gate_name): an allowlisted sentence site's count moved. Move the pin in the change that carries the argument"
       exit 1 ;;
  esac
  check_list "definite-ending choice" "$DEFINITE_RE" "${DEFINITE_ALLOWLIST[@]}" || rc=$?
  case $rc in
    1) gate_error "$(gate_name): $CHECK_LIST_FIRED spelled outside the allowlisted sites (this file's header says why). How a definite refused reading ends is chosen once, in geom_brep::recourse::SizedDecision, for both arms that route it; end a sized decision through it instead, or add the site here with its reason"
       exit 1 ;;
    2) gate_error "$(gate_name): an allowlisted definite-ending site's count moved. Move the pin in the change that carries the argument"
       exit 1 ;;
  esac
  check_list "culprit choice" "$CULPRIT_RE" "${CULPRIT_ALLOWLIST[@]}" || rc=$?
  case $rc in
    1) gate_error "$(gate_name): $CHECK_LIST_FIRED spelled outside the allowlisted sites (this file's header says why). Which culprit a defect ending names is chosen once, in geom_brep::recourse::ReadAt::culprit; end the refusal through it instead, or add the site here with its reason"
       exit 1 ;;
    2) gate_error "$(gate_name): an allowlisted culprit site's count moved. Move the pin in the change that carries the argument"
       exit 1 ;;
  esac
  gate_ok "the reporting margin's door is called only at the ${#DOOR_ALLOWLIST[@]} allowlisted site(s), a valued reading minted only at the ${#MINT_ALLOWLIST[@]} allowlisted mint site(s), its sized sentence asked only at the ${#SIZED_ALLOWLIST[@]} allowlisted site(s), the definite-ending choice spelled only at the ${#DEFINITE_ALLOWLIST[@]} allowlisted site(s), the culprit chosen only at the ${#CULPRIT_ALLOWLIST[@]} allowlisted site(s), no forged terminal-sliver verdict, at the counts they pin ($((${#support[@]} / 2)) test-support mount(s) skipped)"
}

# The clean fixture: the definition home and every allowlisted file,
# each carrying its door calls and mints the way the matcher reads them,
# plus a test module that reads and mints (skipped) and prose naming
# both.
gate_plant_clean() {
  local entry path want why i
  mkdir -p "$1/crates/geom-core/src" "$1/crates/topo/src"
  cat > "$1/crates/geom-core/src/predicate.rs" <<'RS'
impl MarginDiag {
    pub fn diagnostic_f64_for_error_text(self) -> ErrorTextReading {
        todo()
    }
}
fn f(m: f64) -> MarginDiag { MarginDiag::value(m) }
RS
  for entry in "${DOOR_ALLOWLIST[@]}" "${MINT_ALLOWLIST[@]}" "${SIZED_ALLOWLIST[@]}" "${DEFINITE_ALLOWLIST[@]}" "${CULPRIT_ALLOWLIST[@]}"; do
    read -r path want why <<<"$entry"
    mkdir -p "$1/${path%/*}"
    : > "$1/$path"
  done
  for entry in "${SIZED_ALLOWLIST[@]}"; do
    read -r path want why <<<"$entry"
    for ((i = 0; i < want; i++)); do
      printf 'fn s%d(d: MarginDiag, b: Band, w: SizedWords) -> String { d.sized_recourse(b, w) }\n' "$i" >> "$1/$path"
    done
  done
  for entry in "${DEFINITE_ALLOWLIST[@]}"; do
    read -r path want why <<<"$entry"
    for ((i = 0; i < want; i++)); do
      printf 'fn c%d(w: SizedWords) -> String { w.sign_certain_recourse() }\n' "$i" >> "$1/$path"
    done
  done
  for entry in "${CULPRIT_ALLOWLIST[@]}"; do
    read -r path want why <<<"$entry"
    for ((i = 0; i < want; i++)); do
      printf 'fn k%d(at: ReadAt) -> Culprit { at.culprit() }\n' "$i" >> "$1/$path"
    done
  done
  # A test-support mount, whose samples mint: skipped.
  mkdir -p "$1/crates/topo/src"
  cat > "$1/crates/topo/src/lib.rs" <<'RS'
#[cfg(any(test, feature = "test-support"))]
#[doc(hidden)]
mod samples;
RS
  printf 'fn m() -> MarginDiag { MarginDiag::value(5e-9) }\n' > "$1/crates/topo/src/samples.rs"
  for entry in "${DOOR_ALLOWLIST[@]}"; do
    read -r path want why <<<"$entry"
    for ((i = 0; i < want; i++)); do
      printf 'fn d%d(d: MarginDiag) { let _ = d.diagnostic_f64_for_error_text(); }\n' "$i" >> "$1/$path"
    done
  done
  for entry in "${MINT_ALLOWLIST[@]}"; do
    read -r path want why <<<"$entry"
    for ((i = 0; i < want; i++)); do
      printf 'fn m%d(v: f64) -> MarginDiag { MarginDiag::enclosure(v, v) }\n' "$i" >> "$1/$path"
    done
  done
  cat > "$1/crates/topo/src/validate.rs" <<'RS'
// Never call `m.diagnostic_f64_for_error_text()` here to decide, nor
// compare against `MarginDiag::value(0.0)`.
pub const WHY: &str = "m.diagnostic_f64_for_error_text() == MarginDiag::value(0.0)";
pub fn poison() -> MarginDiag { MarginDiag::INVALID }
#[cfg(test)]
mod tests {
    fn pin(d: MarginDiag) { let _ = d.diagnostic_f64_for_error_text() == MarginDiag::value(0.0); }
}
RS
}

# A deciding crate reaching for the door.
plant_call_outside() {
  mkdir -p "$1/crates/topo/src"
  printf 'fn f(d: MarginDiag) -> bool { matches!(d.diagnostic_f64_for_error_text(), ErrorTextReading::Value(m) if m > 0.0) }\n' \
    > "$1/crates/topo/src/props.rs"
}

# The path form of the same call.
plant_path_call_outside() {
  mkdir -p "$1/crates/sweep/src"
  printf 'fn f(d: Option<MarginDiag>) { let _ = d.map(MarginDiag::diagnostic_f64_for_error_text); }\n' \
    > "$1/crates/sweep/src/blend.rs"
}

# A second call in an allowlisted file: the count moves.
plant_second_call_in_a_listed_file() {
  local entry path want why
  entry=${DOOR_ALLOWLIST[0]}
  read -r path want why <<<"$entry"
  printf 'fn extra(d: MarginDiag) { let _ = d.diagnostic_f64_for_error_text(); }\n' >> "$1/$path"
}

# Deciding by equality: a minted comparand.
plant_equality_outside() {
  mkdir -p "$1/crates/topo/src"
  printf 'fn f(d: MarginDiag) -> bool { d == MarginDiag::value(0.0) }\n' \
    > "$1/crates/topo/src/props.rs"
}

# The path form of a mint, handed on as a function.
plant_path_mint_outside() {
  mkdir -p "$1/crates/sweep/src"
  printf 'fn f(v: Option<f64>) -> Option<MarginDiag> { v.map(MarginDiag::value) }\n' \
    > "$1/crates/sweep/src/blend.rs"
}

# The qualified-path form of a mint.
plant_qualified_mint_outside() {
  mkdir -p "$1/crates/topo/src"
  printf 'fn f(d: MarginDiag) -> bool { d == <geom_core::MarginDiag>::value(0.0) }\n' \
    > "$1/crates/topo/src/props.rs"
}

# A plain (not test-support) mount of the same samples: counted.
plant_mint_in_a_plain_mount() {
  mkdir -p "$1/crates/topo/src"
  printf 'mod extra;\n' >> "$1/crates/topo/src/lib.rs"
  printf 'fn m() -> MarginDiag { MarginDiag::value(5e-9) }\n' > "$1/crates/topo/src/extra.rs"
}

# The import door's sentence used as an ε_in oracle, from the door.
plant_in_file_oracle_outside() {
  mkdir -p "$1/crates/step-import/src"
  printf 'fn f(d: MarginDiag, e: FileCoincidence) -> bool { e.miss_recourse_in_file(MissReading::Definite(d), MissSource::File, "", "").contains("stopgap") }\n' \
    > "$1/crates/step-import/src/error.rs"
}

# The door's sized sentence used as an ε_in oracle.
plant_sized_in_file_oracle_outside() {
  mkdir -p "$1/crates/step-import/src"
  printf 'fn f(d: MarginDiag, b: Band, w: SizedWords, e: FileCoincidence) -> bool { d.sized_recourse_in_file(b, w, e).contains("re-export") }\n' \
    > "$1/crates/step-import/src/error.rs"
}

# The door's sized sentence asked past column 200, where the printed
# record is cut: the gate still names it.
plant_long_in_file_oracle_outside() {
  mkdir -p "$1/crates/step-import/src"
  printf 'fn f(d: MarginDiag, b: Band, w: SizedWords, e: FileCoincidence) -> bool { let padding_that_pushes_the_call_past_the_two_hundredth_column_of_the_record_the_gate_prints_and_then_some_more_padding = 0; d.sized_recourse_in_file(b, w, e).contains("re-export") }\n' \
    > "$1/crates/step-import/src/error.rs"
}

# The sentence used as a sign oracle.
plant_sentence_oracle_outside() {
  mkdir -p "$1/crates/topo/src"
  printf 'fn f(d: MarginDiag, b: Band, w: SizedWords) -> bool { d.sized_recourse(b, w).contains("tighten") }\n' \
    > "$1/crates/topo/src/props.rs"
}

# The stored-contradiction choice made outside the sized-decision table.
plant_definite_choice_outside() {
  mkdir -p "$1/crates/topo/src"
  printf 'fn f(w: SizedWords) -> SizedWords { SizedWords { refused: RefusedReading::Defect(Culprit::KernelOrFile), ..w } }\n' \
    > "$1/crates/topo/src/props.rs"
}

# The culprit chosen outside the chooser: a call to it, and the file
# culprit spelled directly.
plant_culprit_call_outside() {
  mkdir -p "$1/crates/topo/src"
  printf 'fn f(at: ReadAt) -> &'"'"'static str { at.culprit().ending() }\n' \
    > "$1/crates/topo/src/props.rs"
}
plant_file_culprit_outside() {
  mkdir -p "$1/crates/step-import/src"
  printf 'fn f() -> &'"'"'static str { StoredCulprit::File.ending() }\n' \
    > "$1/crates/step-import/src/error.rs"
}

# Whether a reading is certainly refused, asked outside the table.
plant_certainly_refused_outside() {
  mkdir -p "$1/crates/topo/src"
  printf 'fn f(d: MarginDiag, b: Band) -> bool { d.shows_no_size(b, SizedPass::Positive) }\n' \
    > "$1/crates/topo/src/props.rs"
}

# The sized decision's own question, whether a reading is certainly
# refused, asked outside the table.
plant_certainly_refused_call_outside() {
  mkdir -p "$1/crates/topo/src"
  printf 'fn f(d: SizedDecision, a: RefusedArm) -> bool { d.certainly_refused(a) }\n' \
    > "$1/crates/topo/src/props.rs"
}

# The sign-certain ending asked outside the sized-decision table.
plant_sign_certain_outside() {
  mkdir -p "$1/crates/topo/src"
  printf 'fn f(w: SizedWords) -> String { w.sign_certain_recourse() }\n' \
    > "$1/crates/topo/src/props.rs"
}

# A forged curability verdict on an escalation minted outside the
# classifier.
plant_forged_sliver() {
  mkdir -p "$1/crates/topo/src"
  printf 'fn f(b: Band) -> Indeterminate { Indeterminate { margin: MarginDiag::INVALID, band: b, predicate: None, terminal_sliver: true } }\n' \
    > "$1/crates/topo/src/props.rs"
}

gate_selftest() {
  local outside="a call to MarginDiag::diagnostic_f64_for_error_text outside the allowlisted sites"
  local minted="a valued MarginDiag minted outside the allowlisted sites"
  gate_selftest_clean
  gate_selftest_without_tool grep "it is grep saying it could not search"
  gate_selftest_case "$outside" plant_call_outside
  gate_selftest_case "$outside" plant_path_call_outside
  gate_selftest_case "an allowlisted site's call count moved" plant_second_call_in_a_listed_file
  gate_selftest_case "$minted" plant_equality_outside
  gate_selftest_case "$minted" plant_path_mint_outside
  gate_selftest_case "$minted" plant_qualified_mint_outside
  gate_selftest_case "$minted" plant_mint_in_a_plain_mount
  gate_selftest_case "sized_recourse asked outside the allowlisted sites" plant_sentence_oracle_outside
  gate_selftest_case "miss_recourse_in_file asked outside the allowlisted sites" plant_in_file_oracle_outside
  gate_selftest_case "sized_recourse_in_file asked outside the allowlisted sites" plant_sized_in_file_oracle_outside
  gate_selftest_case ": sized_recourse_in_file asked outside the allowlisted sites" plant_long_in_file_oracle_outside
  gate_selftest_case "terminal_sliver: true written outside the classifier" plant_forged_sliver
  gate_selftest_case "RefusedReading::Defect spelled outside the allowlisted sites" plant_definite_choice_outside
  gate_selftest_case "sign_certain_recourse spelled outside the allowlisted sites" plant_sign_certain_outside
  gate_selftest_case "shows_no_size spelled outside the allowlisted sites" plant_certainly_refused_outside
  gate_selftest_case "certainly_refused spelled outside the allowlisted sites" plant_certainly_refused_call_outside
  gate_selftest_case "culprit spelled outside the allowlisted sites" plant_culprit_call_outside
  gate_selftest_case "StoredCulprit::File spelled outside the allowlisted sites" plant_file_culprit_outside
  gate_selftest_passes "the definition, the allowlisted calls and mints, a test module's, the poison constant and prose" gate_plant_clean
  gate_selftest_homes --narrowed --subject "$DEFINITION_SUBJECT" "${DEFINITION_HOMES[@]}"
  printf '%s selftest OK: passes a clean fixture carrying the definition, every allowlisted call and mint, a test module that reads and mints, the poison constant and prose naming both; fires on a method call and a path call from another crate, on a moved count, on an equality against a minted reading, a path mint, a qualified-path mint and a mint in a plain mount, on the sized sentence or an import-door sentence asked outside its table, on the stored-contradiction choice, the sign-certain ending or whether a reading is certainly refused outside its table, on a culprit chooser called, or the file culprit spelled, outside the chooser, and on a forged terminal-sliver verdict; skips a test-support mount; and stays RED, with a diagnosis, when grep itself cannot run\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
