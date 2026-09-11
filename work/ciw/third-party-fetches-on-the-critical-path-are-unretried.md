---
id: third-party-fetches-on-the-critical-path-are-unretried
kind: issue
title: six hosted downloads from a third-party host carry no retry, beside two composite actions that do
status: open
opened: 2026-09-11
refs: [apt-preamble-bypass-is-unguarded]
---


Found by the class sweep of `apt-preamble-bypass-is-unguarded` (the guard's
own head-word census over every `run:` body and every tracked shell file).
The subject there is *an install idiom re-spelled inline where a wrapper
exists*; this is the sibling where the wrapper does NOT exist, so it is a
finding and not a bypass.

## The asymmetry

Two composite actions fetch a pinned prebuilt from a third-party host and
both retry it:

- `.github/actions/install-nextest/action.yml:89` — `curl -sSfL --retry 5
  --retry-all-errors --retry-max-time 120 --connect-timeout 15 --max-time
  300 -o <file>`, behind a content-keyed cache.
- `.github/actions/install-sccache/action.yml:91` — the same rules.

Six other downloads from the same class of host carry none of it:

- `.github/workflows/ci.yml:1242` — ruff, `curl -LsSf … | tar zxf -`
- `.github/workflows/ci.yml:3979` and `:3980` — the FreeCAD AppImage and its
  SHA256 file, `curl -sLO`
- `.github/workflows/ci.yml:4154` — maturin, `curl -LsSf … | tar zxf -`
- `.github/workflows/nightly.yml:694` — maturin again, the same line
- `.github/workflows/render.yml:1118` and `:1119` — the FreeCAD pair again

`ci.yml:1242` and `:4154` sit in the `mirror` and `python-suite` jobs, which
run on every code-tier run; the FreeCAD pairs are behind a cache restore, so
they are the cold path only.

## Why the two `curl | tar` sites are the sharper half

`install-nextest`'s own header argues the point and then two rows walk past
it: *"a retried `curl | tar` has already fed tar the bytes of the failed
attempt, so [a retry] is only sound when the sink is a file curl can
truncate and rewrite."* `ci.yml:4154` and `nightly.yml:694` are
`curl … | tar zxf -`, so adding `--retry` to them as they stand would be
unsound; the repair is the shape that action already uses — download to
`$RUNNER_TEMP`, then extract.

## Not taken in the guard's unit

That unit's subject is a wrapper that exists (`scripts/apt-install.sh`) and
a population that must go through it. There is no wrapper for a pinned
prebuilt download: `install-nextest` takes only a `version` input and
hardcodes its URL, so it is a door for one tool and not for this class.
Deciding whether the answer here is a parameterised composite action, a
`scripts/fetch-prebuilt.sh`, or six inline `--retry` flags plus two
restructured sinks is a design question and not a lane's.

Note the shape of the cost: this is the same failure as the apt preamble —
a red gate on every branch during a stranger's bad hour, indistinguishable
from a real red — with the host changed. The apt half is fixed and now
guarded; this half is not.
