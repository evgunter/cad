#!/usr/bin/env python3
"""The two halves of CI run the same checks, the gates that must fire on
docs-tier inputs are SITED where they can, and the prune exception stays one job.

WHY THIS IS PYTHON AND NOT A `scripts/gates/*.sh` MEMBER. Its subject is the
STRUCTURE of `.github/workflows/*.yml` — which job a step belongs to, whether
that job carries an `if:`, whether a prune step precedes a step that reads the
pruned tree. The first version of this check was bash, and read that structure
with `awk '/^  [a-z0-9_-]+:$/'`. A reviewer planted a job named `buildXtra:`
with a checkout and a read of `local-scripts/`: the matcher did not see an
uppercase letter as a job start, folded the steps into the PREVIOUS job, and the
gate exited 0. The claim was stated as total and was approximately total, which
is the failure mode this whole track is about. `scripts/gates/`'s own roster
argument (see `gate-roster.sh`) is that the directory means `lib.sh`'s two-mode
bash contract, and that a python check either reimplements it or meets none of
it — so this lives in `scripts/` beside its sibling
`scripts/check-interval-cfg-additive.py`, is named by hand in BOTH halves like
every other check out here, and is covered by its own claim 1.

Stdlib only, no YAML library: the runner image is not asked for one, matching
the posture of every other cheap tripwire here. What replaces a parser is
FAILING CLOSED — the reader below Bails on any structure it does not recognise,
so an unparsed workflow is an error and never a quiet pass.

WHAT IT CANNOT SEE, stated because a disclosed blind spot is a work order.
Claims 1-4 are about PATHS, so a hosted row that runs work inline — `cargo` in a
`run:` block, with no `scripts/` or `demos/` path to match — is outside them.
That is the residue **S127 / D71** records at this file's `interval_backend`
row: `ci.yml`'s `oracle-certify` job has no local mirror, and nothing enforces
ci.yml <-> ci-local.sh JOB parity. Claim 5 closes the `tools/` corner of it by
name; the rest is a bigger claim than this file makes. Claim 6 reads
`.github/workflows/*.yml` and nothing else that can trigger a checkout, so a
composite action under `.github/actions/` is outside it. And, as everywhere,
wiring is not execution: a step disabled by an `if:` on the STEP still satisfies
claims 7 and 8 — only job-level `if:` is read.

  check-ci-mirror-parity.py [--selftest] [--root DIR]
"""

from __future__ import annotations

import os
import re
import subprocess
import sys
import tempfile

HOSTED_HALF = ".github/workflows/ci.yml"
LOCAL_HALF = "local-scripts/ci-local.sh"
WORKFLOW_DIR = ".github/workflows"

# The one job allowed to keep `local-scripts/` after checkout, because the
# agreement between the halves is what it checks.
SITING_JOB = "mirror"

# THE RULE THIS FILE ENFORCES (Evan, 2026-08-20, on S61): *a gate must be sited
# where it can fire on its own inputs.* Each entry is an invocation whose inputs
# are prose, documentation or `local-scripts/` — file classes that make a change
# set TIER=docs, on which every `if: run_build` job is skipped. Each must be
# invoked from a ci.yml job carrying NO `if:`, and from the local half BEFORE
# its docs-tier early exit.
#
# Without this claim the rule is prose in three script headers: hollowing the
# `mirror` job and moving its three steps back into `discipline` restores the
# exact state S61 recorded, and nothing fires.
TIER_BLIND = (
    "scripts/gates/gate-roster.sh",
    "scripts/gates/probe-suite-census.sh --citations",
    "scripts/check-ci-mirror-parity.py",
)

# Declared asymmetries in claim 1. `path: (half, reason)`. An entry is a
# confession, not a disposition: it says a check runs in one half only.
MIRROR_EXEMPT = {
    "demos/render-uv.sh": (
        "local",
        "the committed UV sheet drift row. Its hosted mirror was retired "
        "2026-08-17 when render.yml's uv lane started re-baselining itself; "
        "ci-local.sh says so at the row",
    ),
}

# Declared asymmetries in claim 2 (gate MODES). Empty, and that is the point:
# every flagged gate invocation is spelled in both halves today.
GATE_MODE_EXEMPT: dict[str, tuple[str, str]] = {}

# Claim 8's floor. A hand-maintained number inside a gate whose thesis is that
# hand-maintained rosters drift — kept deliberately, for the reason
# `probe-suite-census.sh`'s CENSUS_FLOOR is kept: what a floor pins is that the
# population cannot silently SHRINK, and there is nothing to derive it from. A
# marker is a sentence someone chose to write; no file lists which sentences
# ought to exist. Lowering it is a decision, and reads as one in a diff.
MIRROR_MARKER_FLOOR = 13

SCRIPT_RE = re.compile(r"(?:^|[^A-Za-z0-9_/.-])((?:scripts|demos)/[A-Za-z0-9_/.-]+\.(?:sh|py))")
COMMENT_RE = re.compile(r"^\s*#")
MARKER_RE = re.compile(r"#\s*HOSTED MIRROR:\s*(.*?)\s*$")


class Bail(Exception):
    """Structure this reader does not recognise. Never a pass."""


class Step:
    def __init__(self, name: str | None) -> None:
        self.name = name
        self.lines: list[str] = []


class Job:
    def __init__(self, name: str, line: int) -> None:
        self.name = name
        self.line = line
        self.has_if = False
        self.uses: str | None = None
        self.steps: list[Step] = []

    @property
    def text(self) -> str:
        return "\n".join(l for s in self.steps for l in s.lines)


def read_workflow(path: str) -> list[Job]:
    """Jobs, their `if:`, and their steps' raw lines. Bails on anything unknown.

    The job-name pattern is deliberately WIDE (`[A-Za-z0-9_-]+`, any case) and
    anything else at job indent is an error rather than a skip: the bug this
    replaced came from a narrow pattern treating an unmatched line as "not a
    job" instead of as "I do not understand this file".
    """
    jobs: list[Job] = []
    in_jobs = False
    job: Job | None = None
    step: Step | None = None
    in_steps = False
    with open(path, encoding="utf-8") as fh:
        raw = fh.read().splitlines()
    for n, line in enumerate(raw, 1):
        if not line.strip() or COMMENT_RE.match(line):
            continue
        if not line.startswith(" "):
            in_jobs = line.split(":", 1)[0] == "jobs"
            job = step = None
            in_steps = False
            continue
        if not in_jobs:
            continue
        indent = len(line) - len(line.lstrip(" "))
        if indent == 2:
            m = re.fullmatch(r"  ([A-Za-z0-9_-]+):\s*", line)
            if not m:
                raise Bail(f"{path}:{n}: not a job name at job indent: {line!r}")
            job = Job(m.group(1), n)
            jobs.append(job)
            step = None
            in_steps = False
            continue
        if job is None:
            raise Bail(f"{path}:{n}: content inside jobs: before any job: {line!r}")
        if indent == 4:
            key = line.strip().split(":", 1)[0]
            in_steps = key == "steps"
            step = None
            if key == "if":
                job.has_if = True
            elif key == "uses":
                job.uses = line.strip().split(":", 1)[1].strip()
            continue
        if in_steps and re.match(r"^      - ", line):
            step = Step(None)
            job.steps.append(step)
            m = re.match(r"^      - name:\s*(.*?)\s*$", line)
            if m:
                step.name = m.group(1)
        if step is None:
            # Job-level mapping content (env:, strategy:, with:, …). Kept out of
            # the step scan on purpose: it carries no invocations.
            continue
        m = re.match(r"^        name:\s*(.*?)\s*$", line)
        if m:
            step.name = m.group(1)
        step.lines.append(line)
    if not jobs:
        raise Bail(f"{path}: no jobs found — the reader scanned nothing, which is not a pass")
    return jobs


def non_comment(path: str) -> list[str]:
    with open(path, encoding="utf-8") as fh:
        return [l for l in fh.read().splitlines() if not COMMENT_RE.match(l)]


def invocations(lines: list[str]) -> set[str]:
    """Every scripts/** or demos/** path named. The leading boundary in
    SCRIPT_RE keeps `local-scripts/ci-local.sh` from being read as an
    invocation of `scripts/ci-local.sh`."""
    return {m for l in lines for m in SCRIPT_RE.findall(l)}


def gate_modes(lines: list[str]) -> set[str]:
    """`scripts/gates/X.sh --flag` pairs, ignoring `--selftest` (every half runs
    that for every gate) and `--root` (a fixture argument, never a mode)."""
    out = set()
    for l in lines:
        for m in re.finditer(r"(scripts/gates/[A-Za-z0-9_-]+\.sh)\s+(--[a-z-]+)", l):
            if m.group(2) not in ("--selftest", "--root"):
                out.add(f"{m.group(1)} {m.group(2)}")
    return out


def reachable(root: str, seeds: set[str]) -> set[str]:
    """Transitive closure of scripts/** and demos/** paths named from a seed
    set, following file contents. This is what turns claim 4 from a one-hop
    guess into a total statement: `scripts/step_import_check.py` is named by no
    half and is not an orphan — `scripts/check_step.sh` runs it."""
    seen = set(seeds)
    stack = list(seeds)
    while stack:
        cur = stack.pop()
        full = os.path.join(root, cur)
        if not os.path.isfile(full):
            continue
        try:
            with open(full, encoding="utf-8", errors="replace") as fh:
                body = [l for l in fh.read().splitlines() if not COMMENT_RE.match(l)]
        except OSError:
            continue
        # A script names its siblings by BASENAME as often as by path
        # (`demos/render.sh` runs `strip_png_stamps.py` from its own directory),
        # so both spellings count.
        here = os.path.dirname(cur)
        named = invocations(body)
        for l in body:
            # `name.sh`, and `./name.sh` — the spelling a script uses to source
            # a sibling (`. ./hosted-render-guard.sh`), which a path-shaped
            # matcher does not see.
            for m in (re.findall(r"(?:^|[^A-Za-z0-9_/.-])([A-Za-z0-9_.-]+\.(?:sh|py))", l)
                      + re.findall(r"\./([A-Za-z0-9_.-]+\.(?:sh|py))", l)):
                cand = os.path.join(here, m)
                if os.path.isfile(os.path.join(root, cand)):
                    named.add(cand)
        for p in named:
            if p not in seen:
                seen.add(p)
                stack.append(p)
    return seen


def local_docs_exit_line(lines: list[str]) -> int:
    """The line index of the local half's docs-tier early exit. Claim 7 needs
    it: a row placed after it does not run on the tier it is about."""
    for i, l in enumerate(lines):
        if re.search(r'"\$TIER"\s*=\s*docs', l):
            for j in range(i, min(i + 12, len(lines))):
                if re.match(r'\s*exit (0|"?\$[A-Za-z_]\w*"?)\s*$', lines[j]):
                    return j
            raise Bail("the local half's docs-tier branch no longer exits — re-read claim 7")
    raise Bail("the local half has no docs-tier branch — re-read claim 7")


def local_call_sites(lines: list[str], want: str) -> list[int]:
    """Line indices at which the local half actually RUNS `want`.

    Not "mentions": a row wrapped in a shell function is run where the
    FUNCTION is called, and a definition placed above the docs-tier exit says
    nothing about when it executes. So an occurrence inside a function body
    resolves to that function's call sites at top level. Without this the
    claim is satisfied by moving a definition, which is not the property.
    """
    funcs: dict[str, tuple[int, int]] = {}
    open_at: tuple[str, int] | None = None
    for i, l in enumerate(lines):
        m = re.match(r"^([a-z_][a-z0-9_]*)\(\)\s*\{\s*$", l)
        if m and open_at is None:
            open_at = (m.group(1), i)
        elif re.match(r"^\}\s*$", l) and open_at is not None:
            funcs[open_at[0]] = (open_at[1], i)
            open_at = None

    def enclosing(i: int) -> str | None:
        for name, (a, b) in funcs.items():
            if a < i < b:
                return name
        return None

    sites: list[int] = []
    for i, l in enumerate(lines):
        if want not in l:
            continue
        host = enclosing(i)
        if host is None:
            sites.append(i)
            continue
        sites += [j for j, m in enumerate(lines)
                  if re.search(rf"(^|[^A-Za-z0-9_-]){re.escape(host)}([^A-Za-z0-9_-]|$)", m)
                  and enclosing(j) is None and not (funcs[host][0] <= j <= funcs[host][1])]
    return sorted(set(sites))


def check(root: str) -> list[str]:
    errs: list[str] = []

    def err(msg: str) -> None:
        errs.append(msg)

    os.chdir(root)
    for required in (HOSTED_HALF, LOCAL_HALF):
        if not os.path.isfile(required):
            raise Bail(
                f"{required} does not exist under {root} — half of this check's subject is "
                "missing. This runs in the one job that does NOT prune local-scripts/; if a "
                "runner reports this, the prune has spread to that job"
            )

    hosted_lines = non_comment(HOSTED_HALF)
    local_lines = non_comment(LOCAL_HALF)
    hosted = invocations(hosted_lines)
    local = invocations(local_lines)
    if not hosted or not local:
        raise Bail("one half names no scripts/ or demos/ invocation at all — the matcher scanned nothing")

    # CLAIM 1 — invocation parity outside scripts/gates/, both directions.
    for path in sorted((hosted | local) - {p for p in hosted | local if p.startswith("scripts/gates/")}):
        in_h, in_l = path in hosted, path in local
        if in_h and in_l:
            continue
        side = "hosted" if in_h else "local"
        if path in MIRROR_EXEMPT:
            want, reason = MIRROR_EXEMPT[path]
            if want != side:
                err(f"{path} is declared {want}-only in MIRROR_EXEMPT but is invoked by the {side} half. "
                    f'The exemption now describes the opposite of the tree — re-read the reason ("{reason}") '
                    "and fix whichever side moved")
            continue
        other = LOCAL_HALF if in_h else HOSTED_HALF
        why = ("A check that runs only on a developer's machine gates nothing on merge"
               if side == "local" else
               "a row added to one side is invisible on the other until someone reads for it")
        err(f"{HOSTED_HALF if in_h else LOCAL_HALF} invokes {path} and {other} does not. Every check "
            f"outside scripts/gates/ is named by hand in both halves, so {why} — mirror it, or declare it "
            "in MIRROR_EXEMPT with the reason it is one-sided")

    # CLAIM 2 — gate MODE parity. Claim 1 excludes scripts/gates/ because both
    # halves take that roster from the directory, but the directory says
    # nothing about the FLAGS a gate is run with, and a gate's flagged mode is
    # a different check. `--citations` was hosted-only when this claim was
    # written, and nothing could see it.
    h_modes, l_modes = gate_modes(hosted_lines), gate_modes(local_lines)
    for mode in sorted(h_modes ^ l_modes):
        side = "hosted" if mode in h_modes else "local"
        if mode in GATE_MODE_EXEMPT:
            want, reason = GATE_MODE_EXEMPT[mode]
            if want != side:
                err(f"`{mode}` is declared {want}-only in GATE_MODE_EXEMPT but is invoked by the {side} "
                    f'half ("{reason}")')
            continue
        err(f"`{mode}` is invoked by the {side} half only. A gate's flagged mode is a separate check: the "
            "directory loop in the local half runs every gate in DEFAULT mode and sees no flag, so a mode "
            "wired on one side only runs on one side only — mirror it, or declare it in GATE_MODE_EXEMPT")

    # CLAIM 3 — a mirrored path exists on disk. `gate-roster.sh` makes exactly
    # this check for its own directory; without it a typo present in both
    # halves is perfect parity over a file that is not there.
    for path in sorted(hosted | local):
        if not os.path.isfile(path):
            err(f"both halves name {path} and no such file exists — a renamed or deleted check leaves "
                "rows invoking a stale path in perfect agreement with each other")

    # CLAIM 4 — no orphan executables under scripts/ or demos/. The seed is
    # BOTH halves plus every workflow file: `render.yml` is reached through a
    # `uses:` job and runs the render entry points, so a script owned only by
    # it is owned, not orphaned.
    wf_seeds: set[str] = set()
    for wf in sorted(os.listdir(WORKFLOW_DIR)):
        if wf.endswith((".yml", ".yaml")):
            wf_seeds |= invocations(non_comment(f"{WORKFLOW_DIR}/{wf}"))
    owned = reachable(root, hosted | local | wf_seeds)
    for d in ("scripts", "demos"):
        for name in sorted(os.listdir(d)):
            p = f"{d}/{name}"
            if not os.path.isfile(p) or not name.endswith((".sh", ".py")):
                continue
            if p.startswith("scripts/gates/") or p in owned:
                continue
            err(f"{p} is an executable check under {d}/ that NEITHER half names and no named script "
                "reaches. Outside scripts/gates/ there is no roster property at all, so a check can be "
                "written, committed and never run by anything — wire it into both halves, or move it")

    # CLAIM 5 — the `tools/` crates. They are checked by `cd tools/X && cargo …`
    # rows, which carry no scripts/ or demos/ path for claim 1 to match; this is
    # the smallest total statement about that directory.
    for name in sorted(os.listdir("tools")):
        if not os.path.isdir(f"tools/{name}"):
            continue
        in_h = any(f"tools/{name}" in l for l in hosted_lines)
        in_l = any(f"tools/{name}" in l for l in local_lines)
        if not (in_h and in_l):
            side = "the hosted half only" if in_h else ("the local half only" if in_l else "neither half")
            err(f"tools/{name} is a workspace-excluded crate named by {side}. Nothing else "
                "gates that tree — scripts/doc-gate.sh is `cargo doc --workspace` and cannot see it — so "
                "a tool checked on one side only is checked on one side only")

    # CLAIM 6 — the prune exception is EXACTLY ONE JOB, across every workflow
    # file. `render.yml` runs four checked-out lanes of its own through a
    # `uses:` job, and a hole there is the same hole. Both trees are checked,
    # and the ORDER is checked: a job that reads local-scripts/ and then
    # deletes it has read it.
    siting_job_seen = False
    for wf in sorted(os.listdir(WORKFLOW_DIR)):
        if not wf.endswith((".yml", ".yaml")):
            continue
        path = f"{WORKFLOW_DIR}/{wf}"
        for job in read_workflow(path):
            steps = job.steps
            checkout = next((i for i, st in enumerate(steps)
                             if any("actions/checkout" in l for l in st.lines)), None)
            if checkout is None:
                continue
            reads = next((i for i, st in enumerate(steps)
                          if any(("local-scripts" in l or ".claude" in l) and "rm -rf" not in l
                                 for l in st.lines)), None)
            pruned = {t for i, st in enumerate(steps) for l in st.lines
                      if "rm -rf" in l and i > checkout
                      for t in ("local-scripts", ".claude") if t in l}
            first_prune = next((i for i, st in enumerate(steps)
                                if any("rm -rf" in l and "local-scripts" in l for l in st.lines)), None)
            if path == HOSTED_HALF and job.name == SITING_JOB:
                siting_job_seen = True
                if pruned:
                    err(f"job `{SITING_JOB}` prunes {' and '.join(sorted(pruned))}, but it is the one job "
                        "whose subject is the agreement between the halves — with the tree deleted it can "
                        "only check the hosted side, and the gates sited there pass for the wrong reason")
                continue
            missing = {"local-scripts", ".claude"} - pruned
            if missing:
                err(f"{path} job `{job.name}` checks the repo out and does not delete "
                    f"{' and '.join(sorted(missing))}. That deletion is what makes `scripts/ci-filter.py` "
                    "right to classify a change under either tree as non-triggering for the build rows; a "
                    "job that keeps them can couple the hosted gate to a developer's machine or an agent's "
                    "container without anything saying so")
            elif reads is not None and first_prune is not None and reads < first_prune:
                err(f"{path} job `{job.name}` reads local-only tooling at step {reads + 1} and prunes it at "
                    f"step {first_prune + 1}. The prune is only structural while it comes FIRST")
    if not siting_job_seen:
        err(f"{HOSTED_HALF} has no job `{SITING_JOB}` that checks the repo out. That job is where the "
            "checks whose inputs are docs, prose and local-scripts/ are sited; without it they are back in "
            "a job that skips on exactly the change class they are about")

    # CLAIM 7 — THE SITING RULE ITSELF. See TIER_BLIND's comment.
    ci_jobs = {j.name: j for j in read_workflow(HOSTED_HALF)}
    local_exit = local_docs_exit_line(non_comment(LOCAL_HALF))
    for want in TIER_BLIND:
        hosts = [j for j in ci_jobs.values() if any(want in l for st in j.steps for l in st.lines)]
        if not hosts:
            err(f"no ci.yml job runs `{want}`, which is one of the checks whose INPUTS are the docs tier. "
                "A check nobody runs cannot fire anywhere")
            continue
        if all(j.has_if for j in hosts):
            err(f"`{want}` is run only by job(s) {', '.join(sorted(j.name for j in hosts))}, and every one "
                "of them carries an `if:`. Its inputs are prose, documentation or local-scripts/ — file "
                "classes that make a change set TIER=docs, on which every `if: run_build` job is skipped, "
                "so it cannot fire on the only change class that breaks it. That is the exact state S61 "
                "recorded. Site it in a job with no `if:`")
        # The local half is half the pair, and the same rule binds it.
        at = local_call_sites(non_comment(LOCAL_HALF), want)
        if not at:
            err(f"{LOCAL_HALF} never runs `{want}`. The rule is not hosted-side-only: a check the local "
                "half cannot run is a check a developer cannot run before pushing")
        elif min(at) > local_exit:
            err(f"{LOCAL_HALF} RUNS `{want}` only after its docs-tier `exit 0` — a definition above the "
                "exit is not a run — so on a docs-only or "
                "local-scripts-only change the local half runs it not at all — S61's defect, in the other "
                "half of the pair. Move the row above the early exit")

    # CLAIM 8 — mirror citations resolve, job AND step.
    steps = {(j.name, st.name) for j in ci_jobs.values() for st in j.steps if st.name}
    # Markers ARE comments, so they are read from the raw file.
    with open(LOCAL_HALF, encoding="utf-8") as fh:
        markers = sorted({m.group(1) for l in fh.read().splitlines()
                          for m in [MARKER_RE.search(l)] if m})
    for marker in markers:
        if " / " not in marker:
            err(f"{LOCAL_HALF} has a HOSTED MIRROR marker `{marker}` that is not `<job> / <step name>`. The "
                "job half is the part that drifted last time — the prose named the wrong job for a step "
                "that existed")
            continue
        job_name, step_name = marker.split(" / ", 1)
        if (job_name, step_name) not in steps:
            err(f"{LOCAL_HALF} cites a hosted mirror `{marker}`, and ci.yml has no step named "
                f"`{step_name}` in job `{job_name}`. Renaming a step or moving it between jobs leaves "
                "every sentence citing it quietly false")
    if len(markers) < MIRROR_MARKER_FLOOR:
        err(f"{LOCAL_HALF} carries {len(markers)} HOSTED MIRROR marker(s), below the "
            f"{MIRROR_MARKER_FLOOR} it had when this floor was set. Deleting a marker is how a citation "
            "check becomes vacuous; if a row genuinely lost its hosted mirror, lower the floor deliberately")

    return errs


# ---------------------------------------------------------------- self-test
#
# THE FIXTURE IS A MINIATURE REPO. Every case runs the checker AS A SUBPROCESS,
# not as an in-process call: the bash gates' shared harness runs its subject
# inside `if out=$(…)`, where bash suppresses errexit, and that is exactly the
# condition under which a `set -e` script dies before printing its own error.
# A self-test that cannot reproduce the real invocation cannot see that.
def plant_clean(t: str) -> None:
    for d in ("scripts/gates", "demos", "local-scripts", ".github/workflows", "tools/toolcrate"):
        os.makedirs(os.path.join(t, d), exist_ok=True)
    open(os.path.join(t, "tools/toolcrate/Cargo.toml"), "w").close()
    names = [f"check-{i}.sh" for i in range(MIRROR_MARKER_FLOOR)]
    for n in names:
        open(os.path.join(t, "scripts", n), "w").close()
    open(os.path.join(t, "scripts/gates/probe-suite-census.sh"), "w").close()
    open(os.path.join(t, "scripts/check-ci-mirror-parity.py"), "w").close()
    open(os.path.join(t, "scripts/gates/gate-roster.sh"), "w").close()
    with open(os.path.join(t, HOSTED_HALF), "w") as fh:
        fh.write("jobs:\n")
        fh.write(f"  {SITING_JOB}:\n    steps:\n      - uses: actions/checkout@v4\n")
        for want in TIER_BLIND:
            fh.write(f"      - name: sited {want}\n        run: {want}\n")
        fh.write("  discipline:\n    if: needs.filter.outputs.run_build == 'true'\n    steps:\n")
        fh.write("      - uses: actions/checkout@v4\n")
        fh.write("      - name: prune local-only tooling\n        run: rm -rf local-scripts .claude\n")
        for i, n in enumerate(names):
            fh.write(f"      - name: mirrored step {i}\n        run: scripts/{n}\n")
        fh.write("      - name: tools\n        run: cd tools/toolcrate && cargo test\n")
    with open(os.path.join(t, LOCAL_HALF), "w") as fh:
        fh.write("#!/usr/bin/env bash\n")
        for want in TIER_BLIND:
            fh.write(f"{want}\n")
        fh.write('if [ "$TIER" = docs ]; then\n  exit 0\nfi\n')
        for i, n in enumerate(names):
            fh.write(f"# HOSTED MIRROR: discipline / mirrored step {i}\nscripts/{n}\n")
        fh.write("demos/render-uv.sh\ncd tools/toolcrate && cargo test\n")
    open(os.path.join(t, "demos/render-uv.sh"), "w").close()


def _run(root: str) -> tuple[int, str]:
    r = subprocess.run([sys.executable, os.path.abspath(__file__), "--root", root],
                       capture_output=True, text=True)
    return r.returncode, r.stdout + r.stderr


def _case(want: str, plant) -> None:
    with tempfile.TemporaryDirectory() as t:
        plant_clean(t)
        plant(t)
        rc, out = _run(t)
        if rc == 0:
            raise SystemExit(f"SELFTEST FAILED: passed a planted violation ({plant.__name__})\n{out}")
        if want not in out:
            raise SystemExit(f"SELFTEST FAILED ({plant.__name__}): unexpected message\n{out}")


def _append(path: str, text: str):
    def go(t: str) -> None:
        with open(os.path.join(t, path), "a") as fh:
            fh.write(text)
    return go


def selftest() -> None:
    with tempfile.TemporaryDirectory() as t:
        plant_clean(t)
        rc, out = _run(t)
        if rc != 0:
            raise SystemExit(f"SELFTEST FAILED: the checker FAILED on a clean fixture\n{out}")

    def hosted_only(t):        _append(HOSTED_HALF, "      - name: new\n        run: scripts/check-new.sh\n")(t); open(os.path.join(t, "scripts/check-new.sh"), "w").close()
    def local_only(t):         _append(LOCAL_HALF, "scripts/check-local.sh\n")(t); open(os.path.join(t, "scripts/check-local.sh"), "w").close()
    def gate_mode_one_side(t): _append(HOSTED_HALF, "      - name: cit\n        run: scripts/gates/probe-suite-census.sh --crates\n")(t)
    def ghost_path(t):         _append(HOSTED_HALF, "      - name: g\n        run: scripts/gone.sh\n")(t); _append(LOCAL_HALF, "scripts/gone.sh\n")(t)
    def orphan(t):             open(os.path.join(t, "scripts/orphan.sh"), "w").close()
    def tools_one_side(t):     os.makedirs(os.path.join(t, "tools/lonely"))
    def unpruned_job(t):       _append(HOSTED_HALF, "  extra:\n    steps:\n      - uses: actions/checkout@v4\n      - run: echo hi\n")(t)
    def uppercase_job(t):      _append(HOSTED_HALF, "  buildXtra:\n    steps:\n      - uses: actions/checkout@v4\n      - run: cat local-scripts/ci-local.sh\n")(t)
    def second_workflow(t):
        with open(os.path.join(t, ".github/workflows/render.yml"), "w") as fh:
            fh.write("jobs:\n  tour:\n    steps:\n      - uses: actions/checkout@v4\n      - run: echo hi\n")
    def prune_after_read(t):   _append(HOSTED_HALF, "  late:\n    steps:\n      - uses: actions/checkout@v4\n      - run: cat local-scripts/ci-local.sh\n      - run: rm -rf local-scripts .claude\n")(t)
    def unparseable(t):        _append(HOSTED_HALF, "  not a job name\n")(t)
    def marker_wrong_job(t):   _sub(t, LOCAL_HALF, "# HOSTED MIRROR: discipline / mirrored step 0", "# HOSTED MIRROR: k-lint / mirrored step 0")
    def marker_step_renamed(t): _sub(t, HOSTED_HALF, "- name: mirrored step 0", "- name: mirrored step zero")
    def markers_deleted(t):    _sub(t, LOCAL_HALF, "# HOSTED MIRROR: ", "# was: ")
    def exemption_inverted(t): _sub(t, LOCAL_HALF, "demos/render-uv.sh\n", ""); _append(HOSTED_HALF, "      - name: uv\n        run: demos/render-uv.sh\n")(t)

    _case("and local-scripts/ci-local.sh does not", hosted_only)
    _case("and .github/workflows/ci.yml does not", local_only)
    _case("is invoked by the hosted half only", gate_mode_one_side)
    _case("and no such file exists", ghost_path)
    _case("NEITHER half names", orphan)
    _case("named by neither half", tools_one_side)
    _case("checks the repo out and does not delete", unpruned_job)
    _case("job `buildXtra` checks the repo out", uppercase_job)
    _case("render.yml job `tour` checks the repo out", second_workflow)
    _case("prunes it at step", prune_after_read)
    _case("but it is the one job", _resite_prune)
    _case("not a job name at job indent", unparseable)
    _case("has no step named", marker_wrong_job)
    _case("has no step named", marker_step_renamed)
    _case("below the", markers_deleted)
    _case("declared local-only in MIRROR_EXEMPT", exemption_inverted)
    # THE SITING RULE, both halves. These are the cases the reviewer's
    # experiment plants: hollow `mirror`, move the steps back into a job that
    # skips on docs tier; and, locally, move the row below the docs exit.
    _case("carries an `if:`", _hollow_siting_job)
    _case("a definition above the exit is not a run", _local_row_below_exit)
    _case("never runs", _local_row_deleted)
    print("check-ci-mirror-parity selftest OK: passes a clean fixture; fires on a one-sided row, a "
          "one-sided gate MODE, a path both halves name that does not exist, an orphan script, a "
          "one-sided tools/ crate, a checked-out job that keeps either tree, an UPPERCASE job name doing "
          "the same, a second workflow file growing one, a prune that comes after the read, the siting job "
          "pruning, an unparseable workflow, a marker naming the wrong job or a renamed step, the markers "
          "deleted, an inverted exemption, the sited steps moved back into an `if:` job, and the local "
          "half's rows wrapped in a function called below its docs-tier exit, or deleted")


def _sub(t: str, path: str, a: str, b: str) -> None:
    full = os.path.join(t, path)
    with open(full, encoding="utf-8") as fh:
        s = fh.read()
    with open(full, "w", encoding="utf-8") as fh:
        fh.write(s.replace(a, b))


def _resite_prune(t: str) -> None:
    _sub(t, HOSTED_HALF, f"  {SITING_JOB}:\n    steps:\n      - uses: actions/checkout@v4\n",
         f"  {SITING_JOB}:\n    steps:\n      - uses: actions/checkout@v4\n      - run: rm -rf local-scripts .claude\n")


def _hollow_siting_job(t: str) -> None:
    """The reviewer's experiment: hollow `mirror`, move its steps into the
    `if:`-guarded job."""
    moved = "".join(f"      - name: sited {w}\n        run: {w}\n" for w in TIER_BLIND)
    _sub(t, HOSTED_HALF, moved, "      - run: echo hollow\n")
    _sub(t, HOSTED_HALF, "      - name: prune local-only tooling\n        run: rm -rf local-scripts .claude\n",
         "      - name: prune local-only tooling\n        run: rm -rf local-scripts .claude\n" + moved)


def _local_row_below_exit(t: str) -> None:
    """The rows wrapped in a function DEFINED above the docs exit and CALLED
    below it — the shape that satisfies a "mentioned before the exit" check
    while running nothing on the tier the rows are about."""
    for w in TIER_BLIND:
        _sub(t, LOCAL_HALF, f"{w}\n", "")
    body = "".join(f"  {w}\n" for w in TIER_BLIND)
    _sub(t, LOCAL_HALF, 'if [ "$TIER" = docs ]; then\n  exit 0\nfi\n',
         f"tier_blind_rows() {{\n{body}}}\n" + 'if [ "$TIER" = docs ]; then\n  exit 0\nfi\n'
         "tier_blind_rows\n")


def _local_row_deleted(t: str) -> None:
    _sub(t, LOCAL_HALF, f"{TIER_BLIND[0]}\n", "")


def main() -> int:
    root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    args = sys.argv[1:]
    if "--selftest" in args:
        selftest()
        return 0
    if "--root" in args:
        root = os.path.abspath(args[args.index("--root") + 1])
    try:
        errs = check(root)
    except Bail as exc:
        if os.environ.get("GITHUB_ACTIONS"):
            print(f"::error::check-ci-mirror-parity: {exc}")
        print(f"ERROR: check-ci-mirror-parity: {exc}", file=sys.stderr)
        return 1
    for e in errs:
        # BOTH FORMS. `::error::` is what puts the message on the failing step
        # in the Actions UI; the plain line on stderr is what a `gh run view
        # --log` and a piped local run carry. Neither subsumes the other, and
        # the cost of printing both is one line.
        if os.environ.get("GITHUB_ACTIONS"):
            print(f"::error::check-ci-mirror-parity: {e}")
        print(f"ERROR: check-ci-mirror-parity: {e}", file=sys.stderr)
    if errs:
        return 1
    print("check-ci-mirror-parity OK: both halves name the same checks and the same gate modes, no orphan "
          "or missing check under scripts/ or demos/, both tools/ crates' halves agree, every checked-out "
          f"job in {WORKFLOW_DIR}/ but `{SITING_JOB}` prunes local-only tooling before reading it, every "
          "tier-blind check is sited in a job with no `if:` and above the local half's docs exit, and all "
          "hosted-mirror citations resolve")
    return 0


if __name__ == "__main__":
    sys.exit(main())
