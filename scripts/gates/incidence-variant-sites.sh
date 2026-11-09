#!/usr/bin/env bash
# incidence-variant-sites.sh — every file outside geom-brep that names an
# incidence refusal's variant (`CarrierOffChart`, or the check
# `ChartIncidence`) in code has a row in a site table, with its count
# and the reasons it names it. A row is meant only to be lowered; that
# is held in review (below). ONE home; ci.yml's `lint` job runs every
# gate in this directory.
#
# THE RULE (`work/onecopy/incidence-pins-in-three-rows-name-the-variant.md`).
# Whether a carrier was read off its chart, at a pose, or in band is the
# incidence classifier's question: `geom_brep::IncidenceRead::of` on a
# pcurve refusal, `geom_brep::CertifyError::incidence_read` on a
# certification refusal. A row that pins a miss or an in-band read by
# the variant's spelling (`CarrierOffChart { margin: Some(Past(_)), .. }`,
# `Escalated { check: ChartIncidence, .. }`) pins the shape of the
# refusal rather than its read, and the next re-shaping of the refusal
# moves it silently; one that pins the variant's `chart`/`carrier` kinds
# reads them off the refusal rather than off the body. Such a row reads
# the classifier, and the kinds on the body's own face and edge
# (`topo::test_support::incidence_kinds`). This gate is a backstop for
# that rule: a new naming anywhere outside geom-brep raises its file
# above its row and fails, and a naming that went away lowers it and
# fails until the row is lowered, so the slack cannot let one back in
# unseen.
#
# THE STRONGER FIX, filed: a typed incidence payload behind accessors,
# which would make a match by shape outside geom-brep fail to compile
# and retire this gate
# (`work/onecopy/incidence-variants-are-matchable-outside-geom-brep.md`).
#
# EXEMPT, not in the table: `crates/geom-brep/`, the variants' and the
# classifier's home, where reading the variant is the classification.
#
# THE REASONS. Each row names one or more, comma-separated; they say why
# the file names the variant and is not a classifier read:
#   ending    production code mapping a refusal to its ending, reason or
#             walk outcome arm by arm (topo's validator, the boolean's
#             walk, a kind discriminant's match)
#   fixture   a refusal BUILT as a test's input (a unit test's sample, a
#             rendered refusal, `test_support_samples`), never matched
#   door      a pin of a door's own field that no read answers: a
#             refusal's `decision()` naming the check it was decided
#             under (the incidence read, never the residual's meter)
# There is NO pin class: a test that tells a miss from an in-band read,
# or reads a refusal's chart and carrier kinds, reads the classifier and
# the body, and a row naming `pin` is refused.
#
# THE KEY is the file. WHAT THIS GATE DOES NOT ENFORCE, stated as
# `atan2-sites.sh` states its own: a raised count, a new row, or a swap
# within one file of a fixture for a pin passes; the table's diff is read
# in review, and that is where it is held.
#
# WHAT IS READ: the words `CarrierOffChart` and `ChartIncidence` in every
# `crates/*/{src,tests,examples,benches}/**/*.rs` outside geom-brep,
# comments and literals out (`gate_rust_code`), test modules IN — the
# pins this gate exists for are tests. BLIND SPOTS: a glob import
# (`use geom_brep::PcurveCertifyError::*`) or an alias (`use ... as C`
# then `C::CarrierOffChart` is read, `use ...::CarrierOffChart as Off`
# then `Off { .. }` is not); a naming inside a macro's expansion; the
# trees outside `crates/` (`demos/`, `tools/`, `benches/` at the root).
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

TABLE=scripts/gates/incidence-variant-sites.tsv

gate() {
  gate_require_file "$TABLE"
  gate_require_crate_sources
  local files report
  mapfile -t files < <(find crates/*/src crates/*/tests crates/*/examples crates/*/benches \
    -type f -name '*.rs' -not -path 'crates/geom-brep/*' 2>/dev/null | sort)
  if ! report=$(gate_rust_code "${files[@]}" | python3 -c '
import re, sys

table_path = sys.argv[1]
WORD = re.compile(r"\b(?:CarrierOffChart|ChartIncidence)\b")
REASONS = {"ending", "fixture", "door"}
counts = {}
for line in sys.stdin:
    path, _, rest = line.partition(":")
    _, _, code = rest.partition(":")
    n = len(WORD.findall(code))
    if n:
        counts[path] = counts.get(path, 0) + n

table, bad = {}, []
with open(table_path, encoding="utf-8") as fh:
    for number, line in enumerate(fh, 1):
        line = line.split("#", 1)[0].strip()
        if not line:
            continue
        fields = line.split()
        if len(fields) != 3 or not fields[1].isdigit():
            bad.append(f"{table_path}:{number}: a row is `path count reasons`")
            continue
        path, n, reasons = fields
        for r in reasons.split(","):
            if r == "pin":
                bad.append(f"{table_path}:{number}: {path}: a pin of a miss, an in-band read or the kinds reads geom_brep::IncidenceRead::of / CertifyError::incidence_read and topo::test_support::incidence_kinds, never a table row")
            elif r not in REASONS:
                bad.append(f"{table_path}:{number}: {path}: `{r}` is not a reason (the header lists them)")
        if path.startswith("crates/geom-brep/"):
            bad.append(f"{table_path}:{number}: {path}: geom-brep is the home, exempt, never a row")
        if path in table:
            bad.append(f"{table_path}:{number}: {path}: one row per file")
        table[path] = int(n)

for b in bad:
    print("BAD " + b)
for key in sorted(set(counts) | set(table)):
    now, was = counts.get(key, 0), table.get(key, 0)
    if now > was:
        print(f"ROSE {key}: {was} -> {now}")
    elif now < was:
        print(f"FELL {key}: {was} -> {now}")
print(f"TOTAL {sum(counts.values())}")
' "$TABLE"); then
    gate_error "$(gate_name): the counter failed under $PWD (its own message is above, if it had one) — the gate could not read its subject, which is not a pass"
    exit 1
  fi
  if grep -q '^BAD ' <<<"$report"; then
    sed -n 's/^BAD //p' <<<"$report"
    gate_error "$TABLE has a row this gate refuses (above)"
    exit 1
  fi
  if grep -q '^ROSE ' <<<"$report"; then
    grep '^ROSE ' <<<"$report"
    gate_error "an incidence variant was named outside geom-brep — read a miss or an in-band read through geom_brep::IncidenceRead::of / CertifyError::incidence_read, and the kinds on the body (topo::test_support::incidence_kinds), not a new table row: a row is meant only to be lowered"
    exit 1
  fi
  if grep -q '^FELL ' <<<"$report"; then
    grep '^FELL ' <<<"$report"
    gate_error "incidence variant namings went away — lower (or delete) their rows in $TABLE, so the ratchet holds them there"
    exit 1
  fi
  gate_ok "incidence variant namings outside geom-brep at their table ($(sed -n 's/^TOTAL //p' <<<"$report") in all)"
}

gate_plant_clean() {
  gate_plant_clean_sources "$1"
  mkdir -p "$1/scripts/gates"
  printf '# path count reasons\n' > "$1/$TABLE"
}

plant_new_pin() {
  mkdir -p "$1/crates/clean/tests"
  printf 'fn f(e: &E) -> bool { matches!(e, E::CarrierOffChart { .. }) }\n' > "$1/crates/clean/tests/rows.rs"
}

plant_fell() {
  printf 'crates/clean/src/lib.rs 1 ending\n' >> "$1/$TABLE"
}

plant_pin_row() {
  printf 'crates/clean/tests/rows.rs 1 pin\n' >> "$1/$TABLE"
  plant_new_pin "$1"
}

plant_unknown_reason() {
  printf 'crates/clean/tests/rows.rs 1 because\n' >> "$1/$TABLE"
  plant_new_pin "$1"
}

plant_at_table() {
  printf 'crates/clean/tests/rows.rs 1 fixture\n' >> "$1/$TABLE"
  plant_new_pin "$1"
}

# A comment, a string and the home are not namings.
plant_not_sites() {
  printf '// E::CarrierOffChart\npub const S: &str = "ChartIncidence";\n' >> "$1/crates/clean/src/lib.rs"
  mkdir -p "$1/crates/geom-brep/src"
  printf 'fn f(e: &E) -> bool { matches!(e, E::CarrierOffChart { .. }) }\n' > "$1/crates/geom-brep/src/lib.rs"
}

gate_selftest() {
  gate_selftest_clean
  gate_selftest_case "named outside geom-brep" plant_new_pin
  gate_selftest_case "went away" plant_fell
  gate_selftest_case "never a table row" plant_pin_row
  gate_selftest_case "is not a reason" plant_unknown_reason
  gate_selftest_passes "a naming at its row" plant_at_table
  gate_selftest_passes "a comment, a string and the home" plant_not_sites
  printf '%s selftest OK: passes a clean fixture, a naming at its row, and namings in comments, strings and the home; fires on a new naming in a test, an unrecorded fall, a pin row and an unknown reason\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
