#!/usr/bin/env bash
# Drives scripts/k_probe_sweep.sh against a stub `cargo` and a stub probe
# census in a scratch tree, and checks its exit and its census input in
# five states: a red plain suite, two red plain suites, a plain run that
# crashed with no `test result:` line, a red census with every suite
# green, and all green.
#
#   scripts/k_probe_sweep_selftest.sh
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
tree=$(mktemp -d)
trap 'rm -rf "$tree"' EXIT
mkdir -p "$tree/scripts/gates" "$tree/bin" "$tree/demos/tour"
cp "$here/k_probe_sweep.sh" "$tree/scripts/"

cat >"$tree/scripts/gates/probe-suite-census.sh" <<'E'
#!/usr/bin/env bash
case "$1" in
  --executed) printf 'plain\tcrateA\tmodA\nplain\tcrateB\tmodB\n' ;;
  --check-executed) cat >"$CENSUS_SEEN"; exit "${CENSUS_RC:-0}" ;;
esac
E
cat >"$tree/bin/cargo" <<'E'
#!/usr/bin/env bash
args="$*"
if [ "$1" = run ]; then printf 'h\nr\n' >"${*: -1}"; exit 0; fi
if [[ "$args" == *--ignored* ]]; then
  printf 'h\nr\n' >"$CAD_K_REPORT_OUT"
  echo "test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out"
  exit 0
fi
for m in ${STUB_CRASH:-}; do [[ "$args" == *"$m::"* ]] && { echo "running 2 tests"; exit 101; }; done
for m in ${STUB_RED:-}; do
  [[ "$args" == *"$m::"* ]] && { echo "test result: FAILED. 1 passed; 1 failed; 1 ignored; 0 measured; 0 filtered out"; exit 101; }
done
echo "test result: ok. 2 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out"
E
chmod +x "$tree/bin/cargo" "$tree/scripts/gates/probe-suite-census.sh"

bad=0
# case NAME WANT_RC WANT_CSVS WANT_LOG_TEXT [VAR=VALUE...]
case_() {
  local name=$1 want_rc=$2 want_csvs=$3 want_text=$4 rc=0 csvs
  shift 4
  rm -rf "$tree/out" "$tree/seen.tsv" "$tree/log"
  (cd "$tree" && env PATH="$tree/bin:$PATH" CENSUS_SEEN="$tree/seen.tsv" "$@" \
    bash scripts/k_probe_sweep.sh "$tree/out" >"$tree/log" 2>&1) || rc=$?
  csvs=$(find "$tree/out" -maxdepth 1 -name 'k-eps-*.csv' 2>/dev/null | wc -l)
  if [ "$rc" != "$want_rc" ] || [ "$csvs" != "$want_csvs" ] || ! grep -qF -- "$want_text" "$tree/log"; then
    echo "FAIL $name: exit $rc (want $want_rc), $csvs merged CSVs (want $want_csvs), log lacks '$want_text'" >&2
    sed 's/^/    /' "$tree/log" >&2
    bad=1
  fi
  # A red plain suite still counts its executed tests (1 passed + 1 failed).
  if [ "$want_csvs" = 3 ] && ! grep -qP '^plain\tcrateA\tmodA\t2\t1$' "$tree/seen.tsv"; then
    echo "FAIL $name: the census did not read crateA::modA as 2 executed, 1 skipped" >&2
    bad=1
  fi
}
case_ "a red plain suite" 1 3 "1 probe suite(s) red" STUB_RED=modA
case_ "two red plain suites" 1 3 "crateA::modA crateB::modB" STUB_RED="modA modB"
case_ "a crashed plain run" 1 0 "without a test result" STUB_CRASH=modB
case_ "a red census, every suite green" 3 3 "=== k-probe sweep @ eps=1e-12" CENSUS_RC=3
case_ "all green" 0 3 "wrote " STUB_RED=
[ "$bad" = 0 ] && echo "k_probe_sweep selftest OK: 5 cases"
exit "$bad"
