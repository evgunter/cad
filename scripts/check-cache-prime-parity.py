#!/usr/bin/env python3
"""A build job and the job that primes its cache must compute the SAME cache key.

WHY THIS EXISTS. `Swatinem/rust-cache` composes its key from the prefix, the
`shared-key`, os + arch, a hash of the CARGO_*/RUST* environment, and a hash of
the lockfiles. `cache-prime` / `cache-prime-interval` in
`.github/workflows/ci.yml` exist only to write that entry in the DEFAULT
BRANCH's scope, which is the only scope every branch can read; `build` /
`build-interval` restore it. The two sides agree today because their `env:`
blocks are the same text and their `shared-key`s are the same string.

THE FAILURE MODE THIS CATCHES IS SILENT. Change `CARGO_PROFILE_TEST_OPT_LEVEL`
on the build job and not on the primer, or rename one `shared-key`, and the two
keys stop matching. Nothing goes red: the primer writes an entry nobody reads,
every branch's first build job reports `No cache found` and recompiles ~225
dependency crates, and the only symptom is a bill. The measurement that says
what that costs — 82 % of build jobs missing, ~200 s each, ~45 billed minutes
an hour — is the 2026-09-03 entry in `docs/CI-MINUTES-2026-08.md`.

WHAT IT PROVES, no wider:

  For each (consumer, primer) pair below, both jobs exist in ci.yml, their
  `env:` blocks are identical once comments and blank lines are dropped, their
  `runs-on:` lines are the same text, and the `shared-key` on each one's
  `Swatinem/rust-cache@v2` step is the same string; the two pairs' shared keys
  differ from each other; and no OTHER job in any workflow under
  .github/workflows/ uses either of those two keys.

`runs-on` is in that list because os + arch are IN the key: a primer moved to a
different runner class would satisfy every other claim here and still write an
entry the build job cannot restore. The comparison is textual, so the two must
spell the runner the same way — today both say
`${{ vars.BUILD_RUNNER || 'ubuntu-latest' }}`, and two spellings of one runner
would red this rather than pass, which is the direction that fails safe.

WHAT IT DOES NOT PROVE. Not that the keys match at run time — the toolchain,
the runner image and the lockfiles are inputs this cannot read, and a label
that resolves to two different images at two different times is outside it. Not
that the primer builds the right thing; `--workspace` versus a scoped build is
a comment's job, not this one. Not that the entry survives eviction. And not
anything about a THIRD job that grows its own `shared-key`: the exclusivity
claim covers these two values only.

Stdlib only, and a line recogniser rather than a YAML parser — the same posture
as `scripts/check-ci-mirror-parity.py`, whose header argues it. Anything it
cannot recognise raises `Bail` and fails, rather than being read as agreement.

  check-cache-prime-parity.py [--selftest] [--root DIR]
"""

from __future__ import annotations

import os
import re
import shutil
import sys
import tempfile

WORKFLOW_DIR = ".github/workflows"
HOSTED = ".github/workflows/ci.yml"

# (consumer job, primer job). The consumer restores; the primer writes the
# entry under the default branch's scope.
PAIRS = (("build", "cache-prime"), ("build-interval", "cache-prime-interval"))

JOB_RE = re.compile(r"^  ([A-Za-z0-9_-]+):\s*$")
SHARED_KEY_RE = re.compile(r"^\s*shared-key:\s*(\S+)\s*$")


class Bail(Exception):
    """The file is not in a shape this recogniser can read."""


def _jobs(text: str) -> dict[str, list[str]]:
    """Job name -> its block's lines, for the `jobs:` mapping of one workflow."""
    lines = text.split("\n")
    try:
        start = next(i for i, line in enumerate(lines) if line.rstrip() == "jobs:")
    except StopIteration:
        raise Bail("no top-level `jobs:` key") from None
    out: dict[str, list[str]] = {}
    name = None
    for line in lines[start + 1 :]:
        if line and not line.startswith(" ") and not line.startswith("#"):
            break  # a later top-level key ends the mapping
        m = JOB_RE.match(line)
        if m:
            if m.group(1) in out:
                raise Bail(f"job `{m.group(1)}` defined twice")
            name = m.group(1)
            out[name] = []
        elif name is not None:
            out[name].append(line)
    if not out:
        raise Bail("`jobs:` mapping is empty or unreadable")
    return out


def _env_block(block: list[str]) -> list[str]:
    """The job-level `env:` mapping, comments and blank lines dropped.

    Job-level means indent 4; a step's own `env:` is deeper and is not this.
    """
    body: list[str] = []
    seen = False
    for i, line in enumerate(block):
        if line.rstrip() == "    env:":
            if seen:
                raise Bail("two job-level `env:` blocks in one job")
            seen = True
            for nxt in block[i + 1 :]:
                if nxt.strip() == "" or nxt.lstrip().startswith("#"):
                    continue
                if not nxt.startswith("      "):
                    break
                body.append(nxt.strip())
    if not seen:
        raise Bail("no job-level `env:` block")
    return body


def _runs_on(block: list[str]) -> str:
    """The job-level `runs-on:` line, as text. os + arch ride the cache key."""
    hits = [line.strip() for line in block if line.startswith("    runs-on:")]
    if len(hits) != 1:
        raise Bail(f"expected exactly one job-level `runs-on:`, found {len(hits)}")
    return hits[0]


def _shared_keys(block: list[str]) -> list[str]:
    return [m.group(1) for m in map(SHARED_KEY_RE.match, block) if m]


def check(root: str) -> list[str]:
    errs: list[str] = []
    path = os.path.join(root, HOSTED)
    if not os.path.exists(path):
        raise Bail(f"{HOSTED} is not there")
    jobs = _jobs(open(path, encoding="utf-8").read())

    claimed: dict[str, str] = {}
    for consumer, primer in PAIRS:
        for name in (consumer, primer):
            if name not in jobs:
                errs.append(
                    f"{HOSTED} has no job `{name}`. The pair ({consumer}, {primer}) is what "
                    "makes a branch's first build job restore anything at all; if one half is "
                    "gone on purpose, retire the pair from this script in the same diff"
                )
        if consumer not in jobs or primer not in jobs:
            continue

        a, b = _env_block(jobs[consumer]), _env_block(jobs[primer])
        if a != b:
            only_a = [x for x in a if x not in b]
            only_b = [x for x in b if x not in a]
            errs.append(
                f"{HOSTED} jobs `{consumer}` and `{primer}` have different job-level `env:` "
                f"blocks (only in {consumer}: {only_a or '-'}; only in {primer}: {only_b or '-'}). "
                "rust-cache hashes CARGO_*/RUST* into its cache key, so the primer would write an "
                "entry the build job cannot restore, and nothing would go red — see the header"
            )

        ra, rb = _runs_on(jobs[consumer]), _runs_on(jobs[primer])
        if ra != rb:
            errs.append(
                f"{HOSTED} job `{consumer}` is `{ra}` and `{primer}` is `{rb}`. os and arch are "
                "components of rust-cache's key, so a primer on another runner class writes an "
                "entry the build job cannot restore — and every other claim here would still pass"
            )

        ka, kb = _shared_keys(jobs[consumer]), _shared_keys(jobs[primer])
        for name, ks in ((consumer, ka), (primer, kb)):
            if len(ks) != 1:
                errs.append(
                    f"{HOSTED} job `{name}` names {len(ks)} `shared-key` values, expected exactly "
                    "one. The key is the whole coupling; a job with none has fallen back to the "
                    "job-id default, which no other job can spell"
                )
        if len(ka) == 1 and len(kb) == 1:
            if ka[0] != kb[0]:
                errs.append(
                    f"{HOSTED} job `{consumer}` restores `shared-key: {ka[0]}` but `{primer}` "
                    f"writes `{kb[0]}`. Two names, two cache entries, and the one on main is the "
                    "one nobody reads"
                )
            else:
                if ka[0] in claimed:
                    errs.append(
                        f"the {claimed[ka[0]]} pair and the {consumer}/{primer} pair both use "
                        f"`shared-key: {ka[0]}`. `shared-key` REPLACES the job component of "
                        "rust-cache's key, so one value across two feature graphs is exactly the "
                        "thrash `build-interval`'s CACHE SEPARATION note prevents"
                    )
                claimed[ka[0]] = f"{consumer}/{primer}"

    # Exclusivity: no other job anywhere may claim these two keys.
    wf_dir = os.path.join(root, WORKFLOW_DIR)
    for fname in sorted(os.listdir(wf_dir)):
        if not fname.endswith((".yml", ".yaml")):
            continue
        rel = f"{WORKFLOW_DIR}/{fname}"
        for name, block in _jobs(open(os.path.join(wf_dir, fname), encoding="utf-8").read()).items():
            if rel == HOSTED and name in {n for p in PAIRS for n in p}:
                continue
            for k in _shared_keys(block):
                if k in claimed:
                    errs.append(
                        f"{rel} job `{name}` uses `shared-key: {k}`, which belongs to the "
                        f"{claimed[k]} pair. A third job on that key shares one cache entry with "
                        "two feature graphs' worth of jobs, which is the eviction thrash the "
                        "separate keys exist to prevent"
                    )
    return errs


# --------------------------------------------------------------------------
# self-test: a clean fixture must pass, and each planted drift must fire.

def _fixture(dst: str) -> None:
    os.makedirs(os.path.join(dst, WORKFLOW_DIR), exist_ok=True)
    shutil.copy(os.path.join(_root_of_record(), HOSTED), os.path.join(dst, HOSTED))


def _root_of_record() -> str:
    return os.path.dirname(os.path.dirname(os.path.abspath(__file__)))


def _selftest() -> int:
    failures = []

    def case(label, mutate, want_fire):
        with tempfile.TemporaryDirectory() as d:
            _fixture(d)
            p = os.path.join(d, HOSTED)
            if mutate is not None:
                # Read fully, THEN write: `open(p, "w")` in the same expression
                # truncates before the read that feeds it runs.
                before = open(p, encoding="utf-8").read()
                with open(p, "w", encoding="utf-8") as fh:
                    fh.write(mutate(before))
            try:
                errs = check(d)
                fired = bool(errs)
            except Bail as e:
                fired = True
                errs = [f"Bail: {e}"]
            if fired != want_fire:
                failures.append(f"{label}: fired={fired}, wanted={want_fire} ({errs[:1]})")

    def drift_env(t):
        # Move one profile knob on the primer only.
        i = t.index("  cache-prime:")
        head, tail = t[:i], t[i:]
        return head + tail.replace(
            'CARGO_PROFILE_TEST_OPT_LEVEL: "1"', 'CARGO_PROFILE_TEST_OPT_LEVEL: "2"', 1
        )

    def rename_key(t):
        i = t.index("  cache-prime:")
        return t[:i] + t[i:].replace("shared-key: build-default", "shared-key: primed", 1)

    def drop_key(t):
        i = t.index("  cache-prime:")
        return t[:i] + t[i:].replace("          shared-key: build-default\n", "", 1)

    def one_key_both_lanes(t):
        return t.replace("shared-key: build-interval", "shared-key: build-default")

    def third_job(t):
        return t.replace(
            "\njobs:\n",
            "\njobs:\n  interloper:\n    runs-on: ubuntu-latest\n    steps:\n"
            "      - uses: Swatinem/rust-cache@v2\n        with:\n"
            "          shared-key: build-default\n",
            1,
        )

    def drift_runs_on(t):
        i = t.index("  cache-prime:")
        return t[:i] + t[i:].replace(
            "    runs-on: ${{ vars.BUILD_RUNNER || 'ubuntu-latest' }}\n",
            "    runs-on: ubuntu-24.04\n",
            1,
        )

    def lose_primer(t):
        return t.replace("  cache-prime:", "  cache-prime-renamed-away:", 1)

    def no_jobs(t):
        return t.replace("\njobs:\n", "\nnot-jobs:\n", 1)

    case("clean tree passes", None, False)
    case("an env knob moved on one side only", drift_env, True)
    case("the primer's shared-key renamed", rename_key, True)
    case("the primer's shared-key deleted", drop_key, True)
    case("one shared-key across both lanes", one_key_both_lanes, True)
    case("the primer moved to another runner class", drift_runs_on, True)
    case("a third job claiming the key", third_job, True)
    case("the primer job renamed away", lose_primer, True)
    case("a workflow with no jobs: mapping", no_jobs, True)

    if failures:
        for f in failures:
            print(f"check-cache-prime-parity SELFTEST FAILED: {f}", file=sys.stderr)
        return 1
    print(
        "check-cache-prime-parity selftest OK: passes a clean tree; fires on an env knob moved on "
        "one side, a renamed or deleted `shared-key`, one key across both lanes, a primer moved "
        "to another runner class, a third job claiming a key, a primer renamed away, and a "
        "workflow it cannot read"
    )
    return 0


def main(argv: list[str]) -> int:
    root = "."
    if "--root" in argv:
        root = argv[argv.index("--root") + 1]
    if "--selftest" in argv:
        return _selftest()
    try:
        errs = check(root)
    except Bail as e:
        print(f"check-cache-prime-parity: I do not understand this file — {e}", file=sys.stderr)
        return 1
    for e in errs:
        print(f"check-cache-prime-parity: {e}", file=sys.stderr)
    if errs:
        return 1
    pairs = ", ".join(f"{c}/{p}" for c, p in PAIRS)
    print(
        f"check-cache-prime-parity OK: {pairs} each agree on their job-level env block, their "
        "`runs-on:` line and one `shared-key`, the two lanes' keys differ, and no other job "
        "claims either"
    )
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
