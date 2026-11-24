---
id: third-party-fetches-on-the-critical-path-are-unretried
kind: issue
title: two workflow downloads carry no retry, and three pipe a retried curl into tar, which a retry cannot make sound
status: open
opened: 2026-09-11
priority: P2
cost: E
---

Found by the class sweep of the (since deleted) install-wrapper guard, and
re-read against the tree on 2026-09-28.

## The two halves

**No retry at all:** `.github/workflows/render.yml`'s FreeCAD pair
(`install freecad (AppImage 1.1.2, checksum-verified)`, `curl -sLO` for
the AppImage and its SHA256 file). It sits behind a cache restore, so
only the cold path pays, but a cold path on a stranger's bad hour is a
red indistinguishable from a real one. The same pair in `ci.yml`'s
`step-import` job and `nightly.yml`'s carries `--retry 5` already.

**A retry that cannot be sound:** ruff in `ci.yml`'s `lint` job, and
maturin in `ci.yml`'s `python` job and `nightly.yml`'s `python` job, are
each `curl -LsSf --retry 5 … | tar zxf -`. `install-nextest`'s own
header states why that shape is wrong: *"a retried `curl | tar` has
already fed tar the bytes of the failed attempt, so [a retry] is only
sound when the sink is a file curl can truncate and rewrite."* The
repair is the shape that action already uses — download to
`$RUNNER_TEMP`, then extract.

## Retries done right, the pattern to copy

`.github/actions/install-nextest/action.yml` and
`.github/actions/install-sccache/action.yml`: `curl -sSfL --retry 5
--retry-all-errors --retry-max-time 120 --connect-timeout 15 --max-time
300 -o <file>`, behind a content-keyed cache.

GitHub API calls (`nightly.yml`'s `interaction-limit` job, `render.yml`'s dispatch
checks) are not part of this: they talk to the host the run is on, and a
retry buys nothing a re-run does not.

## Open question

Whether the answer is a parameterised composite action for a pinned
prebuilt download, a `scripts/fetch-prebuilt.sh`, or three restructured
sinks plus one `--retry` pair. `install-nextest` takes only a `version`
input and hardcodes its URL, so it is a door for one tool, not for this
class.

- 2026-11-22 — **Re-banded P4 → P2 by CIW triage.** Runner tool downloads (actionlint and the python suite's tools) returned 504 "several times on 2026-11-09", failing lint and python before any check ran. Each cost a fresh push, because the integration cannot re-run jobs (403) (`work/boxes/log.md:538`). The fetches are still unsound: `ci.yml:280` (actionlint, not in the row), `ci.yml:562` and `nightly.yml:938` (maturin) are `curl --retry 5 | tar` with no fallback, and `render.yml:1101-1102` has no retry. ruff (`ci.yml:259`) now has a PyPI fallback. The repair costs nothing per PR. The pattern to copy is a version-keyed cache plus download-to-file (`.github/actions/install-nextest`), which takes the network off the warm path.
