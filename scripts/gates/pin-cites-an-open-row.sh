#!/usr/bin/env bash
# pin-cites-an-open-row.sh — every pinned or disabled red cites a `work/`
# row that exists and is not closed. ONE home; ci.yml's `lint` job runs
# every gate in this directory.
#
# A red that GREEN pins (`test_utils::pinned_red::row`,
# `test_utils::kernel_limit::pinned*`), disables (`#[ignore = "disabled
# red: work/…"]`) or holds out of a workflow row names the row that owns
# its fix. The pin is only honest while that row is open: a closed row
# leaves a pin nobody is going to lift, and a missing one leaves a pin
# that points nowhere. So a row's close takes its pins with it, and this
# gate is what makes that so.
#
# WHAT IS A PIN CITATION. A `work/<program>/<id>` path in
#   - a comment block (consecutive `//` lines in Rust, `#` lines in a
#     workflow), or a single line, that says `pinned red` or `disabled
#     red` — the `#[ignore = "…"]` reasons, the "Pinned red since PR N"
#     comments, nightly.yml's pinned k-lint step — or, in a workflow
#     only, `held out` (the filters that keep a test out of a row);
#   - a comment between a pin call (`pinned_red::row(`,
#     `kernel_limit::pinned(`, `_at`, `_row`, `_row_at`) and the `fn`
#     line enclosing it, or in that fn's own doc and attribute block.
# Scanned: `*.rs` under crates/, demos/, tools/ and
# interval-transcendentals/ (any `target/` skipped), and
# .github/workflows/*.yml.
#
# WHAT IT CANNOT SEE: a citation in a helper fn the pin call reaches
# through an argument, one in a `/* … */` block, one in a module doc
# that does not say `pinned red`, and a pin site that cites no row at
# all. A workflow citation outside a `held out`/`pinned red` block is
# read as a reference, not a pin, and is not checked.
#
# A ROW THAT MOVED is reported at its new path: the id is the file name,
# so the same id under another program is the row the pin means.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

GATE_SCAN_NOUN="pin citation"

PIN_PHRASE_RE='[Pp]inned red|[Dd]isabled red'
WORKFLOW_PHRASE_RE="$PIN_PHRASE_RE|[Hh]eld out"
PIN_CALL_RE='pinned_red::row\(|kernel_limit::pinned(_at|_row|_row_at)?\('

# Emits `FILE<TAB>LINE<TAB>work/<program>/<id>` for every pin citation in
# the files named, one per (line, path). Rust and workflow files differ
# only in their comment leader and their phrase.
extract_awk='
function flush_block(   k) {
  if (blk_n > 0 && blk_hit) {
    for (k = 1; k <= blk_n; k++) cites(blk_file, blk_first + k - 1, blk[k])
  }
  blk_n = 0; blk_hit = 0
}
function cites(file, n, s) {
  while (match(s, /work\/[a-z0-9-]+\/[A-Za-z0-9_.-]*[A-Za-z0-9]/)) {
    print file "\t" n "\t" substr(s, RSTART, RLENGTH)
    s = substr(s, RSTART + RLENGTH)
  }
}
function is_comment(s) {
  return wf ? (s ~ /^[[:space:]]*#/) : (s ~ /^[[:space:]]*\/\//)
}
function walk_back(i,   j) {
  for (j = i - 1; j >= 1 && line[j] !~ ENVIRON["FN_RE"]; j--)
    if (is_comment(line[j])) cites(FILENAME, j, line[j])
  for (j = j - 1; j >= 1 && (is_comment(line[j]) || line[j] ~ /^[[:space:]]*#\[/); j--)
    cites(FILENAME, j, line[j])
}
FNR == 1 {
  flush_block(); delete line
  wf = (FILENAME ~ /\.ya?ml$/)
  phrase = wf ? ENVIRON["WORKFLOW_PHRASE_RE"] : ENVIRON["PIN_PHRASE_RE"]
}
{
  line[FNR] = $0
  if (is_comment($0)) {
    if (blk_n == 0) { blk_first = FNR; blk_file = FILENAME }
    blk[++blk_n] = $0
    if ($0 ~ phrase) blk_hit = 1
  } else {
    flush_block()
    if ($0 ~ phrase) cites(FILENAME, FNR, $0)
    if (!wf && $0 ~ ENVIRON["PIN_CALL_RE"]) walk_back(FNR)
  }
}
END { flush_block() }
'

# `work/<program>/<file>.md<TAB>status` for every row, `-` where the
# header carries none (plan.md, log.md, …). The header is the first
# dozen lines; nothing past it is read.
row_index() {
  awk 'FNR == 1 { st[FILENAME] = "-" }
       /^status:/ { st[FILENAME] = $2; nextfile }
       FNR > 12 { nextfile }
       END { for (f in st) print f "\t" st[f] }' work/*/*.md \
    || gate_reader_died_refusal "the work/ row index (awk)" "$?"
}

gate() {
  gate_require_crate_sources
  local rows=(work/*/*.md)
  if [ ! -f "${rows[0]}" ]; then
    gate_error "$(gate_name): no work/<program>/<id>.md under $PWD, so no citation can be resolved — that is not a pass"
    exit 1
  fi
  local roots=() r
  for r in crates demos tools interval-transcendentals; do
    [ -d "$r" ] && roots+=("$r")
  done
  local files=()
  mapfile -t files < <(gate_grep -rlE --include='*.rs' --exclude-dir=target \
    "$PIN_PHRASE_RE|$PIN_CALL_RE" "${roots[@]}" | sort)
  local workflows=(.github/workflows/*.yml)
  if [ -f "${workflows[0]}" ]; then
    mapfile -t -O "${#files[@]}" files < <(gate_grep -lE "$WORKFLOW_PHRASE_RE" \
      "${workflows[@]}" | sort)
  fi
  local index cites=""
  index=$(row_index)
  if [ "${#files[@]}" -gt 0 ]; then
    cites=$(FN_RE='^[[:space:]]*(pub(\([^)]*\))?[[:space:]]+)?((const|async|unsafe|extern)[[:space:]]+)*fn[[:space:]]' \
      PIN_PHRASE_RE="$PIN_PHRASE_RE" WORKFLOW_PHRASE_RE="$WORKFLOW_PHRASE_RE" \
      PIN_CALL_RE="$PIN_CALL_RE" awk "$extract_awk" "${files[@]}" | sort -u -t$'\t' -k1,1 -k2,2n -k3,3) \
      || gate_reader_died_refusal "the pin-citation reader (awk)" "$?"
  fi
  GATE_SCAN_FILES=0
  if [ -n "$cites" ]; then GATE_SCAN_FILES=$(printf '%s\n' "$cites" | wc -l); fi
  local bad
  bad=$(printf '%s\n' "$index" | awk -F'\t' '
    NR == FNR { st[$1] = $2; n = split($1, p, "/"); id = p[n]; at[id] = at[id] (at[id] ? " " : "") $1; next }
    $0 == "" { next }
    {
      path = $3; sub(/\.md$/, "", path); row = path ".md"
      if (row in st) {
        if (st[row] == "closed") print $1 ":" $2 ": cites " $3 " — that row is closed; lift the pin or point it at the open row that owns the fix"
        next
      }
      n = split(row, p, "/"); id = p[n]; live = ""; dead = ""
      m = split(at[id], hits, " ")
      for (k = 1; k <= m; k++) { if (st[hits[k]] == "closed") dead = hits[k]; else live = hits[k] }
      if (live != "") print $1 ":" $2 ": cites " $3 " — that row moved to " live "; cite it there"
      else if (dead != "") print $1 ":" $2 ": cites " $3 " — that row moved to " dead ", where it is closed; lift the pin or point it at the open row that owns the fix"
      else print $1 ":" $2 ": cites " $3 " — that row does not exist under work/; point it at the open row that owns the fix, or file one"
    }' - <(printf '%s\n' "$cites")) \
    || gate_reader_died_refusal "the row resolver (awk)" "$?"
  if [ -n "$bad" ]; then
    printf '%s\n' "$bad" >&2
    gate_error "$(gate_name): $(printf '%s\n' "$bad" | wc -l) pin citation(s) above name a row that is closed, moved or missing — a pin is honest only while the row that owns its fix is open"
    exit 1
  fi
  gate_ok "every pinned or disabled red cites an open work/ row"
}

# --- self-test ---------------------------------------------------------

gate_plant_clean() {
  gate_plant_clean_sources "$1"
  mkdir -p "$1/work/live" "$1/work/done"
  printf -- '---\nid: owner\nkind: issue\ntitle: t\nstatus: open\nopened: 2026-10-30\n---\n' > "$1/work/live/owner.md"
  printf -- '---\nid: gone\nkind: issue\ntitle: t\nstatus: closed\nopened: 2026-10-30\nclosed: 2026-10-30\n---\n' > "$1/work/done/gone.md"
  printf '# plan\n' > "$1/work/live/plan.md"
  mkdir -p "$1/crates/clean/tests"
  printf '#[test]\n#[ignore = "disabled red: work/live/owner"]\nfn t() {}\n' > "$1/crates/clean/tests/pin.rs"
}

plant_ignore_closed() {
  printf '#[test]\n#[ignore = "pinned red: work/done/gone"]\nfn u() {}\n' > "$1/crates/clean/tests/closed.rs"
}

plant_comment_missing() {
  printf 'fn v() {\n    // Pinned red since PR 1: it refuses\n    // (`work/live/no-such-row.md`).\n    f();\n}\n' \
    > "$1/crates/clean/tests/missing.rs"
}

plant_moved() {
  mkdir -p "$1/demos/tour/tests"
  printf '/// Pinned red (`work/elsewhere/owner.md`).\nfn w() {}\n' > "$1/demos/tour/tests/moved.rs"
}

plant_moved_and_closed() {
  mkdir -p "$1/tools/t/src"
  printf '// disabled red: work/live/gone\nfn x() {}\n' > "$1/tools/t/src/lib.rs"
}

plant_call_site_doc() {
  mkdir -p "$1/interval-transcendentals/src"
  printf '/// The stem, past the kernel limit\n/// (`work/done/gone.md`).\n#[test]\npub(crate) fn y() {\n    let a = 1;\n    test_utils::kernel_limit::pinned(b, eps, &[1e-12], "s");\n}\n' \
    > "$1/interval-transcendentals/src/lib.rs"
}

plant_call_site_body() {
  printf 'fn z() {\n    // the owner: work/done/gone\n    let a = 1;\n    test_utils::pinned_red::row(\n        eps, &[1e-6], "w", &[], f);\n}\n' \
    > "$1/crates/clean/tests/body.rs"
}

plant_workflow_held_out() {
  mkdir -p "$1/.github/workflows"
  printf 'jobs:\n  a:\n    steps:\n      - run: |\n          # Held out at 1e-12 only\n          # (work/live/missing-row.md).\n          cargo nextest run -E "not test(=x)"\n' \
    > "$1/.github/workflows/nightly.yml"
}

plant_near_misses() {
  mkdir -p "$1/.github/workflows" "$1/crates/clean/src"
  # A closed row cited where nothing is pinned: history, not a pin.
  printf '// The census closed with work/done/gone.\nfn a() {}\n' > "$1/crates/clean/src/history.rs"
  # A pin call whose enclosing fn sits below the citation of another fn.
  printf '/// see work/done/gone\nfn a() {}\n\nfn b() {\n    test_utils::pinned_red::row(eps, &[], "w", &[], f);\n}\n' \
    > "$1/crates/clean/src/two_fns.rs"
  # A workflow reference that is no hold-out.
  printf '# The selection and its evidence: work/done/gone.md\n' > "$1/.github/workflows/ci.yml"
  # The live row, by path with `.md`, from a call site's doc.
  printf '/// (`work/live/owner.md`)\nfn c() {\n    test_utils::kernel_limit::pinned_row_at(s, e, &[], "w", f);\n}\n' \
    > "$1/crates/clean/src/live.rs"
}

plant_no_rows() { rm -rf "$1/work"; }

gate_selftest() {
  gate_selftest_clean
  gate_selftest_without_tool grep "it is grep saying it could not search"
  gate_selftest_case --also "work/done/gone" "that row is closed" plant_ignore_closed
  gate_selftest_case --also "missing.rs:3:" "does not exist under work/" plant_comment_missing
  gate_selftest_case --also "moved.rs:1:" "that row moved to work/live/owner.md" plant_moved
  gate_selftest_case "moved to work/done/gone.md, where it is closed" plant_moved_and_closed
  gate_selftest_case --also "lib.rs:2:" "that row is closed" plant_call_site_doc
  gate_selftest_case --also "body.rs:2:" "that row is closed" plant_call_site_body
  gate_selftest_case --also "nightly.yml:6:" "does not exist under work/" plant_workflow_held_out
  gate_selftest_case "no work/<program>/<id>.md" plant_no_rows
  gate_selftest_passes "a closed row cited as history, behind another fn, or as a workflow reference" plant_near_misses
  printf '%s selftest OK: passes a live citation and closed rows cited outside any pin; fires, at file:line, on a pin citing a closed row (an #[ignore] reason, a pin call'"'"'s doc block and its body comment), a missing row, a moved row (live or closed at its new home), a workflow hold-out citing a missing row, and a tree with no work/ rows; and stays RED when grep cannot run\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
