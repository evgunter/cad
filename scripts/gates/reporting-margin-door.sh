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
# WHAT A LISTED SITE OWES, and the gate checks none of it — it checks
# only the file and the count, so the list is where the argument lives:
# a door site puts the numbers into error text or a payload another
# renderer prints, and nothing branches on them; a mint site reports
# what its own classification saw. A review that finds the door or a
# mint anywhere else, or an entry whose site compares the numbers, has
# found the misuse.
#
# WHAT THIS GATE DOES NOT COVER, stated rather than implied: it reads
# production source text (test modules and test-only mounts are skipped:
# a test pinning a payload's numbers decides nothing), so a call reached
# through a macro or a re-export under another name is invisible to it;
# and rendering the reading as text (`Display`/`LowerExp`) and parsing
# the text back is a door it cannot see — as obviously wrong as it is
# long.
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
  'crates/sweep/src/blend/battery.rs 2 the blend payload reports its companion quantities as its own scalar reads them (the M5 PR 12 seam)'
  'crates/topo/src/boolean/sectors.rs 1 a bisector read On between definite bounds reports what is known of it, the zero band'
  'crates/topo/src/chart_region.rs 1 a definite deduction that cannot certify its outcome echoes the value it classified'
  'crates/topo/src/test_support_samples.rs 8 the refusal samples the coverage rows render'
)
MINT_RE='MarginDiag::(value|enclosure)([^A-Za-z0-9_]|$)'

# check_list WHAT RE ENTRY... — the hits of RE over production code,
# outside the definition home, against the `path count why` entries.
# Prints the offending records and returns 1 (outside) or 2 (a moved
# count); 0 when every hit is on the list at its count.
check_list() {
  local what=$1 re=$2
  shift 2
  local hits
  hits=$(gate_rust_code --skip-cfg-test "${GATE_PRODUCTION_FILES[@]}" \
    | gate_grep -E "$re" \
    | gate_grep -vE "$(gate_record_anchor_any "${DEFINITION_HOMES[@]}")" \
    | cut -c1-200)
  local entry path want why have bad=
  local -a listed=()
  for entry in "$@"; do
    read -r path want why <<<"$entry"
    listed+=("$path")
    [ -f "$path" ] || bad+="  [$path]: the allowlisted file is gone"$'\n'
    have=$(printf '%s\n' "$hits" | gate_grep -cE "$(gate_record_anchor "$path")" || true)
    if [ "$have" != "$want" ]; then
      bad+="  [$path]: $have $what(s), the entry pins $want ($why)"$'\n'
    fi
  done
  local outside
  outside=$(printf '%s\n' "$hits" | gate_grep -vE "$(gate_record_anchor_any "${listed[@]}")" || true)
  if [ -n "$outside" ]; then
    printf '%s\n' "$outside"
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
  gate_ok "the reporting margin's door is called only at the ${#DOOR_ALLOWLIST[@]} allowlisted site(s), and a valued reading minted only at the ${#MINT_ALLOWLIST[@]} allowlisted mint site(s), at the counts they pin"
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
  for entry in "${DOOR_ALLOWLIST[@]}" "${MINT_ALLOWLIST[@]}"; do
    read -r path want why <<<"$entry"
    mkdir -p "$1/${path%/*}"
    : > "$1/$path"
  done
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
  gate_selftest_passes "the definition, the allowlisted calls and mints, a test module's, the poison constant and prose" gate_plant_clean
  gate_selftest_homes --narrowed --subject "$DEFINITION_SUBJECT" "${DEFINITION_HOMES[@]}"
  printf '%s selftest OK: passes a clean fixture carrying the definition, every allowlisted call and mint, a test module that reads and mints, the poison constant and prose naming both; fires on a method call and a path call from another crate, on a moved count, on an equality against a minted reading and on a path mint; and stays RED, with a diagnosis, when grep itself cannot run\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
