#!/usr/bin/env bash
# view-cache-agrees.sh — the views `run-gates.sh`'s cache serves are the
# views the shared reader builds. ONE home; ci.yml's `lint` job runs every
# gate in this directory.
#
# THE RULE (`lib.sh`, "THE VIEW CACHE"). A plain run cuts each cached
# view of the tree once and every gate reads it from there, so a cache
# that served a view the reader would not have built would blind every
# gate at once while each printed its OK. Under `run-gates.sh` this gate
# reads the run's own cache (`GATE_VIEW_CACHE`); run alone, it builds one.
#
# THE SCAN: for every view the cache holds, the view of every file it
# indexes, served from the cache, is compared byte for byte with the same
# call read from the tree; and the cached test-only mount set with the
# resolver's own answer. Any difference is a red naming the view.
#
# THE SELF-TEST builds a cache over a fixture written to break the cut —
# nested and multi-line block comments, raw strings carrying quotes, a
# `#[cfg(test)]` module with braces in its literals, a statement left
# open at the end of a file, an empty file and a comment-only one, a path
# with a colon — and asks for the same agreement over the files in index
# order, reversed, with a file named twice in a row and with the
# unindexed path among them; it shows the cache is what answered by
# marking an entry, a file written after the build is read afresh, and
# each claim that does not hold — an entry gone, a builder that did not
# finish, a cache directory that is not there — is a refusal.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"
GATE_SCAN_NOUN="cached view"

# The flags each cached view was cut with, by its key.
declare -A VIEW_FLAGS=()
for flags in "${GATE_VIEW_CACHE_VIEWS[@]}"; do
  # shellcheck disable=SC2086
  VIEW_FLAGS[$(gate_view_cache_key $flags)]=$flags
done

# agree CACHE FILE... — prints one line per view or mount set the cache
# serves differently from the tree, over FILE...
agree() {
  local cache=$1 key out; shift
  out=$(mktemp -d)
  # Each view is compared in its own process, all at once: the reader's
  # side of each is a full read of the tree.
  for key in $(cat "$cache/complete"); do
    agree_one "$cache" "$key" "$@" > "$out/$key" &
  done
  wait
  cat "$out"/*
  rm -rf "$out"
}

agree_one() {
  local cache=$1 key=$2 served live; shift 2
  if [ "$key" = mounts ]; then
    [ -f "$cache/mounts.key" ] || return 0
    local -a src=()
    mapfile -t src < <(find crates/*/src -type f -name '*.rs' 2>/dev/null | sort)
    served=$(GATE_VIEW_CACHE=$cache gate_test_only_mounts "${src[@]}" | md5sum)
    live=$( (unset GATE_VIEW_CACHE; gate_test_only_mounts "${src[@]}") | md5sum)
  else
    if [ -z "${VIEW_FLAGS[$key]+set}" ]; then
      printf 'the cache holds a view keyed %s that no flags in lib.sh produce\n' "$key"
      return 0
    fi
    # shellcheck disable=SC2086
    served=$(GATE_VIEW_CACHE=$cache gate_rust_code ${VIEW_FLAGS[$key]} "$@" | md5sum)
    # shellcheck disable=SC2086
    live=$( (unset GATE_VIEW_CACHE; gate_rust_code ${VIEW_FLAGS[$key]} "$@") | md5sum)
  fi
  [ "$served" = "$live" ] || printf 'the cached %s view differs from the reader'"'"'s\n' "$key"
}

indexed() { cut -f3- "$1/index"; }

gate() {
  local cache=${GATE_VIEW_CACHE:-} own= files=() diffs
  if [ -z "$cache" ]; then
    own=$(mktemp -d)
    cache=$own
    if ! (gate_view_cache_build "$cache"); then
      rm -rf "$own"
      gate_error "$(gate_name): the view cache could not be built under $PWD, so there is nothing to compare — that is not a pass"
      exit 1
    fi
  fi
  ( GATE_VIEW_CACHE=$cache gate_view_cache_applies ) || {
    [ -z "$own" ] || rm -rf "$own"
    gate_error "$(gate_name): GATE_VIEW_CACHE=$cache was not built for $PWD, so this gate compared nothing — that is not a pass"
    exit 1
  }
  mapfile -t files < <(indexed "$cache")
  GATE_SCAN_FILES=$(wc -w < "$cache/complete")
  diffs=$(agree "$cache" ${files[@]+"${files[@]}"})
  # THE LOOKUP, not only the cut: the whole index in index order is the
  # one list the lookup can answer by reading entries in order, so two
  # real files are also asked for reversed, with one named twice and
  # with one spelled as the index does not spell it.
  local -a probe=()
  mapfile -t probe < <(printf '%s\n' ${files[@]+"${files[@]}"} | grep -E '^crates/[^/]+/src/lib\.rs$' | head -2)
  if [ "${#probe[@]}" -eq 2 ]; then
    diffs+=$'\n'$(agree "$cache" "${probe[1]}" "${probe[0]}")
    diffs+=$'\n'$(agree "$cache" "${probe[0]}" "${probe[0]}" "${probe[1]}")
    diffs+=$'\n'$(agree "$cache" "./${probe[0]}" "${probe[1]}")
  else
    diffs+=$'\nno two crates/*/src/lib.rs in the index to probe the lookup with'
  fi
  diffs=$(printf '%s\n' "$diffs" | sed '/^$/d')
  [ -z "$own" ] || rm -rf "$own"
  if [ -n "$diffs" ]; then
    printf '%s\n' "$diffs"
    gate_error "the view cache serves what the shared Rust reader would not build (above): every gate reading it read the wrong tree. Fix the cut or the lookup in lib.sh's VIEW CACHE, never the gates"
    exit 1
  fi
  gate_ok "every cached view of ${#files[@]} files, and the cached mount set, is the reader's own"
}

plant_fixture() {
  local d=$1
  mkdir -p "$d/crates/clean/src" "$d/crates/odd/src"
  printf '%s\n' \
    'pub fn a(x: f64) -> f64 { /* one /* nested */ still */ x + 1.0 }' \
    '/* a block' '   across lines { */' \
    'pub fn b() -> &'"'"'static str { r#"a "quoted" } brace"# }' \
    'pub fn c(c: char) -> bool { c == '"'"'{'"'"' || c == '"'"'\'"'"''"'"' }' \
    'fn d<'"'"'a>(s: &'"'"'a str) -> &'"'"'a str { s } // trailing { comment' \
    '#[cfg(test)]' 'mod t;' \
    '#[cfg(test)]' 'mod inline_tests {' '    const S: &str = "}}}";' '    fn f() { let _ = S; }' '}' \
    'pub fn e() {' '    let open = 1' > "$d/crates/clean/src/lib.rs"
  printf 'fn t() { let _ = "{"; }\n' > "$d/crates/clean/src/t.rs"
  : > "$d/crates/clean/src/empty.rs"
  printf '// only a comment\n/* and a block */\n' > "$d/crates/clean/src/comments.rs"
  printf '    + 2; }\nfn g() { 1 }\n' > "$d/crates/odd/src/lib.rs"
  printf 'fn colon() {}\n' > "$d/crates/odd/src/a:b.rs"
}

# expect_refusal WHAT WANT CMD... — CMD, run in its own subshell with its
# own marker, must fail with WANT in a diagnosis and leave the marker.
expect_refusal() {
  local what=$1 want=$2 out; shift 2
  local marker
  marker=$(mktemp -u)
  if out=$(GATE_MATCHER_FAILED=$marker "$@" 2>&1); then
    printf 'SELFTEST FAILED (%s): it did not refuse:\n%s\n' "$what" "$out" >&2
    exit 1
  fi
  case "$out" in
    *"$want"*) ;;
    *) printf 'SELFTEST FAILED (%s): wanted `%s`, got:\n%s\n' "$what" "$want" "$out" >&2; exit 1 ;;
  esac
  if [ ! -e "$marker" ]; then
    printf 'SELFTEST FAILED (%s): it refused without the marker gate_ok reads\n' "$what" >&2
    exit 1
  fi
  rm -f "$marker"
}

served_view() { local c=$1; shift; GATE_VIEW_CACHE=$c gate_rust_code "$@"; }
served_mounts() { local c=$1; shift; GATE_VIEW_CACHE=$c gate_test_only_mounts "$@"; }

gate_selftest() {
  local tmp cache diffs files=() rev=() entry out
  tmp=$(mktemp -d)
  plant_fixture "$tmp"
  cd "$tmp"
  cache=$tmp.cache
  mkdir "$cache"
  gate_view_cache_build "$cache"
  mapfile -t files < <(indexed "$cache")
  case " ${files[*]} " in *" crates/odd/src/a:b.rs "*)
    printf 'SELFTEST FAILED: a path with a colon was indexed, so the cut at the first colon would misplace its records\n' >&2; exit 1 ;;
  esac
  [ -s "$cache/mounts.key" ] || { printf 'SELFTEST FAILED: the fixture'"'"'s cfg(test) mount set was not cached\n' >&2; exit 1; }
  for ((i = ${#files[@]} - 1; i >= 0; i--)); do rev+=("${files[$i]}"); done
  for list in "files" "rev" "dup" "colon"; do
    case $list in
      files) set -- "${files[@]}" ;;
      rev) set -- "${rev[@]}" ;;
      dup) set -- crates/clean/src/lib.rs crates/clean/src/lib.rs "${files[@]}" ;;
      colon) set -- "${files[@]}" crates/odd/src/a:b.rs ;;
    esac
    diffs=$(agree "$cache" "$@")
    if [ -n "$diffs" ]; then
      printf 'SELFTEST FAILED (%s): %s\n' "$list" "$diffs" >&2; exit 1
    fi
  done
  # The cache is what answered: a mark in an entry reaches the caller.
  entry=$cache/c0l0-lines0/$(awk -F '\t' '$3 == "crates/clean/src/lib.rs" { print $1 }' "$cache/index")
  printf 'MARK\n' >> "$entry"
  out=$(served_view "$cache" crates/clean/src/lib.rs)
  case "$out" in *MARK*) ;; *) printf 'SELFTEST FAILED: the view did not come from the cache, so nothing above exercised it\n' >&2; exit 1 ;; esac
  # A file written after the build is read afresh.
  sleep 0.01
  printf 'fn later() {}\n' >> crates/clean/src/lib.rs
  out=$(served_view "$cache" crates/clean/src/lib.rs)
  case "$out" in
    *MARK*|*"fn later"*"fn later"*) printf 'SELFTEST FAILED: a file written after the build was served from the cache\n%s\n' "$out" >&2; exit 1 ;;
    *"fn later"*) ;;
    *) printf 'SELFTEST FAILED: the rewritten file was not read at all\n%s\n' "$out" >&2; exit 1 ;;
  esac
  # Each claim that does not hold is a refusal.
  rm -f "$cache/c0l0-lines0/1"
  expect_refusal "an entry gone" "missing an entry" served_view "$cache" crates/clean/src/comments.rs
  rm -f "$cache/complete"
  expect_refusal "a builder that did not finish" "names no complete view cache" served_view "$cache" crates/clean/src/comments.rs
  expect_refusal "the mount set of a builder that did not finish" "names no complete view cache" \
    served_mounts "$cache" crates/clean/src/lib.rs
  rm -rf "$cache"
  expect_refusal "a cache that is not there" "names no complete view cache" served_view "$cache" crates/clean/src/comments.rs
  cd /
  rm -rf "$tmp"
  printf '%s selftest OK: a cache built over a fixture that breaks the cut — nested and multi-line comments, raw strings with quotes, a cfg(test) module with braces in its literals, a statement open at the end of a file, an empty and a comment-only file — serves every view and the mount set as the reader builds them, over its files in order, reversed, with one named twice in a row and beside an unindexed path with a colon; the mark in an entry reaches the caller, a file written after the build is read afresh, and an entry gone, a builder that did not finish and a missing cache are each a refusal through the marker\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
