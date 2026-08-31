#!/usr/bin/env bash
# base-test-listing.sh — this run's test listing, the base run's listing, and
# the "what this PR adds" block that diffs them. ONE copy, called by both
# sharded test jobs.
#
# WHY IT IS A SCRIPT AND NOT TWO `run:` BLOCKS. The lookup is ~60 lines of
# shell and the default and interval lanes need the same 60 lines against a
# different archive. Written inline it was copied verbatim into both jobs and
# had already drifted in one string — and only ONE LANE RUNS PER RUN under
# configuration sampling, so the copy that did not run was untested by
# construction: every hosted green was a green for one of them.
#
# THE GUARDS LIVE HERE FOR THE SAME REASON, and it is the sharper half. The
# defect this file is shaped around actually happened hosted (run 33343519165):
# the artifacts API answered `403` to a token without `actions: read`, `gh` put
# `Resource not accessible by integration (HTTP 403)` where the command
# substitution picked it up, and that string was spliced into a URL and
# reported as "the artifact was found but could not be downloaded" — a report
# manufacturing a reading out of a failure, which is the one thing it may not
# do. In a `run:` block the fix for that was code no test could red. Here the
# `--selftest` below drives every one of those guards against stub `cargo`,
# `gh` and `curl` on PATH, so removing one turns this row red.
#
# THREE THINGS ARE NOT TRUSTED, each with its own case in the selftest:
#
#   * A TREE SHA THAT IS NOT ONE. `gh` writes its error text where a value was
#     expected; a capture that is not 40 hex characters is not a tree, and no
#     artifact name is built from it.
#   * A URL THAT IS NOT ONE. Same capture, same disposition: not `https://` is
#     not a URL, and nothing is fetched from it.
#   * AN HTTP ERROR THAT LOOKS LIKE A DOWNLOAD. `curl` without `-f` exits 0 on
#     a 404 and writes the error page to the output file, so the first layer
#     refuses rather than leaving it to `unzip` and the JSON parser downstream.
#
# AND THE LISTING IS NOT PUBLISHED UNLESS IT IS ONE. `cargo nextest list` can
# fail and leave an empty or truncated file behind. Published anyway, that file
# SQUATS the artifact name `test-list-<lane>-<tree>` for the retention window:
# every later run whose base is that tree finds it, fails to diff it, and
# prints a stated skip — a permanent skip for a week, caused by one bad run.
# So the listing is checked (exit status, non-empty, JSON with a non-empty
# `rust-suites` object) and the tree name is exported to `$GITHUB_ENV` only if
# it passes; a listing that fails is deleted, so the upload step has nothing to
# find even if its own condition is ever loosened.
#
# NOTHING HERE CAN FAIL A JOB. It exits 0 on every path — the report it prints
# gates nothing (issue 469) and a report that reds a job has broken something
# more important than itself. Every failure becomes a printed reason instead.
#
# Usage:
#     base-test-listing.sh --archive FILE --lane LANE --job LABEL --cost-dir DIR
#                          [--leg 'LABEL=PATH']...
#     base-test-listing.sh --selftest
#
# Reads BASE_SHA, GH_TOKEN, GITHUB_REPOSITORY and GITHUB_ENV from the
# environment. Writes the block to stdout; diagnostics to stderr.

set -uo pipefail

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# The listing is a listing: readable JSON with a non-empty `rust-suites`
# object, which is the one key the report's parser reads. A cheap probe in the
# interpreter that is going to parse it anyway, rather than a second opinion in
# another language.
listing_is_real() {
  [ -s "$1" ] || return 1
  python3 -c '
import json
import sys

with open(sys.argv[1], encoding="utf-8") as fh:
    doc = json.load(fh)
suites = doc.get("rust-suites") if isinstance(doc, dict) else None
raise SystemExit(0 if isinstance(suites, dict) and suites else 1)
' "$1" 2>/dev/null
}

# The base listing, or the reason there is none. Sets BASE_LIST (a path, or
# empty) and REASON. Never exits nonzero.
lookup_base_listing() {
  local lane=$1 cost=$2
  BASE_LIST=""
  BASE_TREE=""
  if [ -z "${BASE_SHA:-}" ]; then
    REASON="this run is not a pull_request run, so it has no base tree to diff against"
    return 0
  fi
  # `:-` rather than a bare read, because this script runs under `set -u`: an
  # UNSET repository would abort it here, and an abort prints no block at all
  # — the silent absence every path in this file exists to replace.
  local repo="${GITHUB_REPOSITORY:-}"
  if [ -z "$repo" ]; then
    REASON="\$GITHUB_REPOSITORY is not set, so there is no repository to look a base listing up in — this lookup only means anything inside a GitHub Actions run"
    return 0
  fi
  # stderr discarded and the value shape-checked: see the header's three
  # untrusted captures.
  BASE_TREE=$(gh api "repos/$repo/commits/$BASE_SHA" --jq .commit.tree.sha 2>/dev/null)
  echo "$BASE_TREE" | grep -qE '^[0-9a-f]{40}$' || BASE_TREE=""
  if [ -z "$BASE_TREE" ]; then
    REASON="base commit $BASE_SHA could not be resolved to a tree — the API call failed or answered something that is not a tree SHA"
    return 0
  fi
  local name="test-list-$lane-$BASE_TREE"
  local url
  url=$(gh api "repos/$repo/actions/artifacts?name=$name&per_page=1" \
        --jq '.artifacts[] | select(.expired == false) | .archive_download_url' 2>/dev/null | head -n 1)
  case "$url" in https://*) ;; *) url="" ;; esac
  if [ -z "$url" ]; then
    REASON="no run has published a test listing named \`$name\`, so the base tree \`$BASE_TREE\` has never been listed in this lane — it was tested before this report existed, its listing's retention has expired, the run that tested it drew the other lane, or the lookup had no \`actions: read\`"
    return 0
  fi
  # `-f`: without it curl exits 0 on an HTTP error and writes the error body to
  # the output file, handing a decoy to `unzip` and the parser below.
  if curl -fsSL -H "Authorization: Bearer ${GH_TOKEN:-}" -o "$cost/base.zip" "$url" &&
     unzip -o -q "$cost/base.zip" -d "$cost/base" &&
     [ -f "$cost/base/test-list.json" ]; then
    BASE_LIST="$cost/base/test-list.json"
  else
    REASON="the listing artifact \`$name\` was found but could not be downloaded or unpacked"
  fi
  return 0
}

run() {
  local archive="" lane="" job="this job" cost=""
  local legs=()
  while [ $# -gt 0 ]; do
    case "$1" in
      --archive)  archive=$2; shift 2 ;;
      --lane)     lane=$2;    shift 2 ;;
      --job)      job=$2;     shift 2 ;;
      --cost-dir) cost=$2;    shift 2 ;;
      --leg)      legs+=(--leg "$2"); shift 2 ;;
      *) echo "base-test-listing.sh: unknown argument $1" >&2; return 0 ;;
    esac
  done
  if [ -z "$archive" ] || [ -z "$lane" ] || [ -z "$cost" ]; then
    echo "base-test-listing.sh: --archive, --lane and --cost-dir are required" >&2
    return 0
  fi
  mkdir -p "$cost"

  local list="$cost/test-list.json"
  local status=0
  rm -f "$list"
  cargo nextest list --archive-file "$archive" --workspace-remap . \
    --message-format json > "$list" || status=$?

  if [ "$status" -ne 0 ] || ! listing_is_real "$list"; then
    # A LISTING THAT IS NOT ONE PUBLISHES NOTHING. Deleted rather than left for
    # the upload step to find, and `$GITHUB_ENV` is not written, so the name
    # `test-list-$lane-<this tree>` stays free for a run that can fill it.
    rm -f "$list"
    python3 "$HERE/pr-added-tests.py" --job "$job" --head "$list" \
      --no-base "this run's own \`cargo nextest list\` did not produce a usable listing (exit $status), so there is nothing to diff against a base — and nothing was published under this tree's name, which would otherwise have made every future run based on this tree skip too"
    return 0
  fi

  local head_tree
  head_tree=$(git rev-parse "HEAD^{tree}" 2>/dev/null)
  echo "$head_tree" | grep -qE '^[0-9a-f]{40}$' || head_tree=""

  local BASE_LIST REASON="no base listing was looked for" BASE_TREE=""
  lookup_base_listing "$lane" "$cost"

  if [ -n "$BASE_LIST" ]; then
    python3 "$HERE/pr-added-tests.py" --job "$job" --head "$list" --base "$BASE_LIST" \
      --base-source "the published listing for base tree \`$BASE_TREE\`" \
      ${legs[@]+"${legs[@]}"}
  else
    python3 "$HERE/pr-added-tests.py" --job "$job" --head "$list" --no-base "$REASON"
  fi

  if [ -n "$head_tree" ] && [ -n "${GITHUB_ENV:-}" ]; then
    echo "TEST_LIST_TREE=$head_tree" >> "$GITHUB_ENV"
  fi
  return 0
}

# ---------------------------------------------------------------- selftest

FIXTURES="$HERE/fixtures"

# The stubs. `cargo`, `gh` and `curl` are the three producers this script has
# no way to fail on purpose otherwise; each reads its behaviour out of the
# environment so a case is a variable assignment rather than a second stub.
make_stubs() {
  local bin=$1
  mkdir -p "$bin"
  cat > "$bin/cargo" <<'STUB'
#!/usr/bin/env bash
# `cargo nextest list` writes $STUB_LIST_FILE (if set) and exits $STUB_LIST_STATUS.
[ "${1:-}" = nextest ] || exit 0
[ -n "${STUB_LIST_FILE:-}" ] && cat "$STUB_LIST_FILE"
exit "${STUB_LIST_STATUS:-0}"
STUB
  cat > "$bin/gh" <<'STUB'
#!/usr/bin/env bash
# `gh api PATH …`. The commits call answers $STUB_TREE_OUT / $STUB_TREE_STATUS,
# the artifacts call $STUB_URL_OUT / $STUB_URL_STATUS. An ERROR BODY ON STDOUT
# with a nonzero exit is the real 403 shape from run 33343519165.
case "${2:-}" in
  */commits/*)         printf '%s\n' "${STUB_TREE_OUT:-}"; exit "${STUB_TREE_STATUS:-0}" ;;
  *actions/artifacts*) printf '%s\n' "${STUB_URL_OUT:-}";  exit "${STUB_URL_STATUS:-0}" ;;
esac
exit 1
STUB
  cat > "$bin/curl" <<'STUB'
#!/usr/bin/env bash
# With $STUB_ZIP set, copies it to the `-o` target. Without, refuses the way
# `curl -f` refuses an HTTP error — which is the whole point of the flag.
out=""
while [ $# -gt 0 ]; do
  case "$1" in -o) out=$2; shift 2 ;; *) shift ;; esac
done
if [ -n "${STUB_ZIP:-}" ] && [ -n "$out" ]; then cp "$STUB_ZIP" "$out"; exit 0; fi
echo "curl: (22) The requested URL returned error: 404" >&2
exit 22
STUB
  chmod +x "$bin/cargo" "$bin/gh" "$bin/curl"
}

SELFTEST_FAILURES=0

fail() {
  echo "SELFTEST FAILED: $1" >&2
  SELFTEST_FAILURES=$((SELFTEST_FAILURES + 1))
}

want() {
  case "$2" in *"$1"*) ;; *) fail "$3 — expected to find: $1"$'\n'"$2" ;; esac
}

reject() {
  case "$2" in *"$1"*) fail "$3 — must NOT contain: $1"$'\n'"$2" ;; esac
}

SELFTEST_TMP=""
# The trap outlives the function, so the directory it removes cannot be a
# `local` — under `set -u` that is an unbound variable at exit time.
cleanup_selftest() { [ -n "$SELFTEST_TMP" ] && rm -rf "$SELFTEST_TMP"; return 0; }

selftest() {
  local tmp
  SELFTEST_TMP=$(mktemp -d)
  tmp=$SELFTEST_TMP
  trap cleanup_selftest EXIT
  make_stubs "$tmp/bin"
  PATH="$tmp/bin:$PATH"
  export PATH

  # The base listing, zipped the way the artifacts API serves one.
  python3 -c '
import sys
import zipfile

with zipfile.ZipFile(sys.argv[1], "w") as z:
    z.write(sys.argv[2], "test-list.json")
' "$tmp/base.zip" "$FIXTURES/nextest-list-base.json"

  local out env_file cost
  local n=0
  # `case NAME` sets up one case: a fresh cost dir and GITHUB_ENV, the stub
  # variables, and then `out` and `env_file` for the assertions.
  case_run() {
    n=$((n + 1))
    cost="$tmp/case-$n"
    env_file="$tmp/env-$n"
    : > "$env_file"
    out=$(GITHUB_ENV="$env_file" GITHUB_REPOSITORY="owner/repo" GH_TOKEN=x \
          run --archive nextest-x.tar.zst --lane default --job "test (eps = default, 1/2)" \
              --cost-dir "$cost" --leg "run archived tests=$FIXTURES/nextest-run-head.txt" 2>/dev/null)
  }

  # 1 — THE LISTING FAILED. Nothing is published, and the skip says why.
  STUB_LIST_STATUS=101 STUB_LIST_FILE="" BASE_SHA=deadbeef case_run
  want "Skipped, and here is why" "$out" "case 1 (listing exit 101)"
  want "did not produce a usable listing (exit 101)" "$out" "case 1 (listing exit 101)"
  reject "TEST_LIST_TREE" "$(cat "$env_file")" "case 1 squatted the artifact name"
  [ -f "$cost/test-list.json" ] && fail "case 1 left a broken listing for the upload step to find"

  # 2 — THE LISTING SUCCEEDED AND IS EMPTY. Same disposition: exit status alone
  # is not the check, because a truncated write exits 0.
  STUB_LIST_STATUS=0 STUB_LIST_FILE=/dev/null BASE_SHA=deadbeef case_run
  want "did not produce a usable listing" "$out" "case 2 (empty listing)"
  reject "TEST_LIST_TREE" "$(cat "$env_file")" "case 2 published an empty listing"

  # 3 — VALID JSON, NO SUITES. `{}` parses and lists nothing; a diff against it
  # says "this PR adds every test in the suite", which is the sentence the
  # whole report exists not to manufacture.
  echo '{"rust-suites": {}}' > "$tmp/no-suites.json"
  STUB_LIST_STATUS=0 STUB_LIST_FILE="$tmp/no-suites.json" BASE_SHA=deadbeef case_run
  want "did not produce a usable listing" "$out" "case 3 (no suites)"
  reject "TEST_LIST_TREE" "$(cat "$env_file")" "case 3 published a listing with no suites"

  # From here the listing is real, so every case exercises a lookup guard.

  # 4 — THE 403 THAT ACTUALLY HAPPENED. `gh` writes its error text to stdout and
  # exits nonzero; that text is not a tree SHA, so no artifact name is built
  # from it and none of it reaches the report.
  STUB_LIST_STATUS=0 STUB_LIST_FILE="$FIXTURES/nextest-list-head.json" BASE_SHA=deadbeef \
    STUB_TREE_OUT="Resource not accessible by integration (HTTP 403)" STUB_TREE_STATUS=1 case_run
  want "could not be resolved to a tree" "$out" "case 4 (403 on the commits call)"
  reject "HTTP 403" "$out" "case 4 spliced gh's error text into the report"
  reject "test-list-default-Resource" "$out" "case 4 built an artifact name out of an error"
  want "TEST_LIST_TREE=" "$(cat "$env_file")" "case 4 — a failed LOOKUP must not stop this run's own listing being published"

  # 5 — THE ARTIFACTS CALL ANSWERS AN ERROR BODY. Not `https://`, so not a URL,
  # and the report says the listing was never published rather than inventing a
  # download.
  STUB_LIST_STATUS=0 STUB_LIST_FILE="$FIXTURES/nextest-list-head.json" BASE_SHA=deadbeef \
    STUB_TREE_OUT=1109b0184d1218a3cf0e8435e7ce713b855d0a1c \
    STUB_URL_OUT="Resource not accessible by integration (HTTP 403)" STUB_URL_STATUS=1 case_run
  want "no run has published a test listing named" "$out" "case 5 (error body for a URL)"
  want "test-list-default-1109b0184d1218a3cf0e8435e7ce713b855d0a1c" "$out" "case 5 names the listing it looked for"
  reject "found but could not be downloaded" "$out" "case 5 read a failed lookup as a found artifact"

  # 6 — THE URL IS REAL AND THE FETCH FAILS. `curl -f` refuses the error body;
  # without the flag this is a 200-shaped zero-byte "download".
  STUB_LIST_STATUS=0 STUB_LIST_FILE="$FIXTURES/nextest-list-head.json" BASE_SHA=deadbeef \
    STUB_TREE_OUT=1109b0184d1218a3cf0e8435e7ce713b855d0a1c \
    STUB_URL_OUT="https://api.github.com/artifacts/1/zip" case_run
  want "was found but could not be downloaded or unpacked" "$out" "case 6 (curl refuses an HTTP error)"

  # 7 — THE WHOLE PATH. A real base listing arrives as a zip and the priced
  # diff is printed: the fixtures' one rename plus one addition.
  STUB_LIST_STATUS=0 STUB_LIST_FILE="$FIXTURES/nextest-list-head.json" BASE_SHA=deadbeef \
    STUB_TREE_OUT=1109b0184d1218a3cf0e8435e7ce713b855d0a1c \
    STUB_URL_OUT="https://api.github.com/artifacts/1/zip" STUB_ZIP="$tmp/base.zip" case_run
  want "adds 2 tests costing 0.038 cpu-s per run" "$out" "case 7 (the happy path)"
  want "the published listing for base tree" "$out" "case 7 names its base"
  reject "Skipped, and here is why" "$out" "case 7 skipped a lookup that succeeded"
  grep -qE '^TEST_LIST_TREE=[0-9a-f]{40}$' "$env_file" ||
    fail "case 7 — a good listing must export a 40-hex tree: $(cat "$env_file")"

  # 8 — NOT A pull_request RUN. No base to look for, and that is a stated skip
  # rather than a lookup against an empty SHA.
  STUB_LIST_STATUS=0 STUB_LIST_FILE="$FIXTURES/nextest-list-head.json" BASE_SHA="" case_run
  want "not a pull_request run" "$out" "case 8 (no base sha)"

  # 9 — RUN OUTSIDE ACTIONS, with the repository variable genuinely UNSET
  # rather than empty. Under `set -u` a bare `$GITHUB_REPOSITORY` aborts the
  # script mid-report, and an abort prints NO block at all — which is the
  # silent absence, and worse than any stated one. `env -u` is the only way to
  # stage that: an assignment to `""` does not trip `set -u`.
  n=$((n + 1))
  cost="$tmp/case-$n"
  out=$(env -u GITHUB_REPOSITORY GH_TOKEN=x BASE_SHA=deadbeef \
        STUB_LIST_STATUS=0 STUB_LIST_FILE="$FIXTURES/nextest-list-head.json" \
        bash "$0" --archive nextest-x.tar.zst --lane default --job "j" \
                  --cost-dir "$cost" 2>/dev/null)
  want "GITHUB_REPOSITORY is not set" "$out" "case 9 (no repository in the environment)"
  want "What this PR adds to the test suite" "$out" "case 9 printed no block at all"

  if [ "$SELFTEST_FAILURES" -ne 0 ]; then
    echo "base-test-listing selftest FAILED ($SELFTEST_FAILURES)" >&2
    return 1
  fi
  echo "base-test-listing selftest ok: a failed/empty/suite-less listing publishes nothing, gh's" \
       "403 body is refused as a tree and as a URL, curl -f refuses an HTTP error, the whole" \
       "path prices a real base listing served as a zip, and neither a missing base SHA nor a" \
       "missing repository can turn a stated skip into a silent one"
  return 0
}

if [ "${1:-}" = --selftest ]; then
  selftest
  exit $?
fi
run "$@"
exit 0
