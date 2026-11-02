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

rc=0
for g in "$(dirname "$0")"/gates/*.sh; do
  name=$(basename "$g" .sh)
  [ "$name" = lib ] && continue
  start=$SECONDS
  if [ "$mode" = plain ]; then
    group "$g"
    bash "$g" || fail "$g failed"
  else
    group "$g --selftest"
    out=$(bash "$g" --selftest 2>&1)
    status=$?
    printf '%s\n' "$out"
    if [ "$status" -ne 0 ]; then
      fail "$g --selftest failed (exit $status)"
    elif ! grep -qF "$name selftest OK" <<<"$out"; then
      fail "$g --selftest exited 0 without printing '$name selftest OK': it did not run its self-test"
    fi
  fi
  printf '%s: %s s\n' "$g" "$((SECONDS - start))"
  endgroup
done
exit "$rc"
