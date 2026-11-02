#!/usr/bin/env bash
# run-gates.sh [--selftest] — run every gate in scripts/gates/ (each
# `*.sh` but lib.sh), each in its own log group, and exit 1 if any was
# red. ci.yml's `lint` job runs it plain; nightly.yml's `gate-selftests`
# job runs it with --selftest.
#
# Under --selftest a gate passes only when its output carries its own
# `<gate> selftest OK` line, which lib.sh's harness prints after the last
# case. A gate that ignored the flag would scan the repo plainly and
# exit 0, and that is not a self-test.
#
# The gates run GATE_JOBS at a time (default: one per CPU). Each is a
# single-threaded scan of the tree that shares no file with another (its
# scratch is mktemp or PID-keyed), so they are independent. Each gate's
# output is held until it finishes and then printed whole, in name order,
# so the log reads as a serial run would.
set -u

mode=plain
case "${1:-}" in
  "") ;;
  --selftest) mode=selftest ;;
  *) printf 'usage: %s [--selftest]\n' "$0" >&2; exit 2 ;;
esac

group() { [ -n "${GITHUB_ACTIONS:-}" ] && printf '::group::%s\n' "$*"; true; }
endgroup() { [ -n "${GITHUB_ACTIONS:-}" ] && printf '::endgroup::\n'; true; }
fail() {
  if [ -n "${GITHUB_ACTIONS:-}" ]; then printf '::error::%s\n' "$*"; else printf 'ERROR: %s\n' "$*"; fi
  rc=1
}

jobs_max=${GATE_JOBS:-$(nproc 2>/dev/null || echo 1)}
case "$jobs_max" in ''|*[!0-9]*|0) printf 'GATE_JOBS must be a positive integer, not %s\n' "$jobs_max" >&2; exit 2 ;; esac
held=$(mktemp -d) || exit 2
trap 'rm -rf "$held"' EXIT

# One gate, its output and exit status written under $held/<name>.
run_one() {
  local g=$1 name=$2 start=$SECONDS status=0
  if [ "$mode" = plain ]; then
    bash "$g" >"$held/$name.out" 2>&1 || status=$?
  else
    bash "$g" --selftest >"$held/$name.out" 2>&1 || status=$?
  fi
  printf '%s\n' "$((SECONDS - start))" >"$held/$name.secs"
  printf '%s\n' "$status" >"$held/$name.status"
}

gates=()
for g in "$(dirname "$0")"/gates/*.sh; do
  [ "$(basename "$g" .sh)" = lib ] && continue
  gates+=("$g")
done

for g in "${gates[@]}"; do
  while [ "$(jobs -rp | wc -l)" -ge "$jobs_max" ]; do wait -n; done
  run_one "$g" "$(basename "$g" .sh)" &
done
wait

rc=0
for g in "${gates[@]}"; do
  name=$(basename "$g" .sh)
  if [ "$mode" = plain ]; then group "$g"; else group "$g --selftest"; fi
  cat "$held/$name.out" 2>/dev/null
  status=$(cat "$held/$name.status" 2>/dev/null || echo missing)
  if [ "$mode" = plain ]; then
    [ "$status" = 0 ] || fail "$g failed"
  elif [ "$status" != 0 ]; then
    fail "$g --selftest failed (exit $status)"
  elif ! grep -qF "$name selftest OK" "$held/$name.out"; then
    fail "$g --selftest exited 0 without printing '$name selftest OK': it did not run its self-test"
  fi
  printf '%s: %s s\n' "$g" "$(cat "$held/$name.secs" 2>/dev/null || echo '?')"
  endgroup
done
exit "$rc"
