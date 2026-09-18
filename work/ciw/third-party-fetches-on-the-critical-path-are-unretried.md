---
id: third-party-fetches-on-the-critical-path-are-unretried
kind: issue
title: seven hosted downloads from a third-party host carry no retry, beside three callers that do
status: open
opened: 2026-09-11
refs: [apt-preamble-bypass-is-unguarded]
---


Found by the class sweep of `apt-preamble-bypass-is-unguarded` (the guard's
own head-word census over every `run:` body and every tracked shell file).
The subject there is *an install idiom re-spelled inline where a wrapper
exists*; this is the sibling where the wrapper does NOT exist, so it is a
finding and not a bypass.

## Every `curl` in that population, with its disposition

Fourteen commands. Three retry, seven are third-party downloads that do not,
and four are GitHub API calls that are a deliberate narrowing rather than a
gap. Line numbers are as of the branch that filed this
(`ciw/apt-preamble-guard`).

**Retries, and they are the pattern to copy:**

- `.github/actions/install-nextest/action.yml:89` — `curl -sSfL --retry 5
  --retry-all-errors --retry-max-time 120 --connect-timeout 15 --max-time
  300 -o <file>`, behind a content-keyed cache.
- `.github/actions/install-sccache/action.yml:91` — the same rules.
- `.claude/hooks/session-start.sh:270` — `--retry 3 --retry-delay 2`, into a
  file. Not a CI row, but the same shape and already solved.

**Third-party downloads with no retry — the finding:**

- `.github/workflows/ci.yml:1242` — ruff, `curl -LsSf … | tar zxf -`
- `.github/workflows/ci.yml:4181` — maturin, `curl -LsSf … | tar zxf -`
- `.github/workflows/nightly.yml:694` — maturin again, the same line
- `.github/workflows/ci.yml:4006`, `:4007` — the FreeCAD AppImage and its
  SHA256 file, `curl -sLO`
- `.github/workflows/render.yml:1118`, `:1119` — the FreeCAD pair again

`ci.yml:1242` sits in the `mirror` job and `:4181` in `python-suite`, both of
which run on every code-tier run; the FreeCAD pairs sit behind a cache
restore, so they are the cold path only.

**GitHub API calls, NOT part of this finding, and here so the list is a
receipt rather than a claim:**

- `.github/workflows/nightly.yml:2003`, `.github/workflows/render.yml:430`,
  `:439` — authenticated `api.github.com` calls that read or dispatch. They
  talk to the host the run is already hosted on; if it is down the run is
  down, and a retry buys nothing a re-run does not.
- `scripts/base-test-listing.sh:117` — the same, and that file is S-TCOST's
  by this program's `keep_out`, so it is reported and not edited here.

## Why the two `curl | tar` sites are the sharper half

`install-nextest`'s own header argues the point and then three rows walk past
it: *"a retried `curl | tar` has already fed tar the bytes of the failed
attempt, so [a retry] is only sound when the sink is a file curl can truncate
and rewrite."* `ci.yml:1242`, `ci.yml:4181` and `nightly.yml:694` are all
`curl … | tar zxf -`, so adding `--retry` to them as they stand would be
unsound; the repair is the shape that action already uses — download to
`$RUNNER_TEMP`, then extract.

## Not taken in the guard's unit

That unit's subject is a wrapper that exists (`scripts/apt-install.sh`) and a
population that must go through it. There is no wrapper for a pinned prebuilt
download: `install-nextest` takes only a `version` input and hardcodes its
URL, so it is a door for one tool and not for this class. Deciding whether
the answer is a parameterised composite action, a `scripts/fetch-prebuilt.sh`,
or seven inline `--retry` flags plus three restructured sinks is a design
question and not a lane's.

Note the shape of the cost: this is the same failure as the apt preamble — a
red gate on every branch during a stranger's bad hour, indistinguishable from
a real red — with the host changed. The apt half is fixed and now guarded;
this half is not.
