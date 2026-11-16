#!/usr/bin/env python3
"""The nightly's attribution of each newly red slow test to its merge
(`work/ciw/dependents-slow-sets-skip-a-kernel-pr.md`, the policy Ev ruled on
PR 6731, 2026-11-12).

A slow test red on tonight's nightly and green on the previous one is bisected
over the first-parent commits between the two nightly heads, alone, at the
nightly's build profile; the first red commit is filed as a tracker row. It
costs no PR anything and prevents nothing: it replaces days of unattributed
red with a named PR by morning.

    nightly-attribute.py newly-red --tonight DIR --last DIR --out FILE
        the newly red slow tests, as JSON. DIR holds one subdirectory per
        junit artifact (`full-suite-junit-<eps>[-<k>of<n>]/junit.xml`, as
        actions/download-artifact lays a pattern out); shards of one ε row
        are one row. A test is newly red in a row when it failed there
        tonight and passed there last night, and it is a SLOW test
        (`slow_set.entries`). One entry per test: its first red row in
        default, 1e-6, 1e-12 order.

    nightly-attribute.py bisect --tests FILE --base SHA --head SHA
                                --worktree DIR --out FILE [--cap N] [--minutes M]
        bisects the first CAP tests over `base..head`'s first-parent
        commits, skipping a commit whose own diff is TIER=docs
        (scripts/ci-filter.py: it cannot move a test). Each probe checks the
        commit out in DIR (a `git worktree`, so this checkout keeps running
        the script) and runs the one test, alone, at its ε. The endpoints
        are probed first: a test that passes alone at HEAD did not
        reproduce, and one that fails at the base was not newly red; neither
        is filed. A commit that does not build, or no longer holds the test,
        is skipped, and the answer is then a range. Past M minutes the
        remaining tests and the one in flight are listed, not filed.

    nightly-attribute.py write --results FILE [--root DIR]
        files each attributed test under `work/` (the policy's "What the
        attribution writes"): a dated `## Nightly attribution` section on
        every LIVE row that names the test, or else one new row
        `work/<program>/main-red-<leaf>.md`. It writes nothing else; the
        caller lints and commits. Closed rows are not reopened by a mention:
        a red that a closed row once fixed comes back as a new row.

    nightly-attribute.py summary --results FILE   markdown for the run summary
    nightly-attribute.py --selftest

Stdlib only; reads the slow set through `slow_set.py`, its one reader.
"""

from __future__ import annotations

import argparse
import datetime as dt
import json
import os
import re
import subprocess
import sys
import tempfile
import time
import xml.etree.ElementTree as ET
from collections.abc import Callable
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import slow_set

EPS_ORDER = ("default", "1e-6", "1e-12")
CAP = 5
ARTIFACT_PREFIX = "full-suite-junit-"
PANIC_LINES = 4
# nextest's exit codes (`nextest-metadata`'s NextestExitCode).
EXIT_TEST_FAILED = 100
EXIT_NO_TESTS = 4
EXIT_BUILD_FAILED = 101
LIVE_EXCLUDES = ("closed",)
MERGE_RE = re.compile(r"^Merge pull request #(\d+) from [^/\s]+/(\S+)")
NARRATIVE = {"plan.md", "log.md", "process-observations.md", "program.md"}

Key = tuple[str, str]  # (binary id, test name)


# --------------------------------------------------------------------------
# newly red
# --------------------------------------------------------------------------


def eps_row(artifact: str) -> str:
    """`full-suite-junit-1e-12-2of3` → `1e-12`: the shards of a row are one row."""
    return re.sub(r"-\d+of\d+$", "", artifact.removeprefix(ARTIFACT_PREFIX))


def _panic(case: ET.Element) -> str:
    bad = case.find("failure")
    if bad is None:
        bad = case.find("error")
    text = (bad.text if bad is not None else "") or ""
    lines = [ln for ln in text.strip().splitlines() if ln.strip()]
    for i, ln in enumerate(lines):
        if "panicked at" in ln:
            return "\n".join(lines[i : i + PANIC_LINES])
    if bad is not None:
        return bad.get("message") or bad.get("type") or "failed"
    return ""


def outcomes(root: Path) -> dict[str, dict[Key, tuple[bool, str]]]:
    """Per ε row, each test's (passed, first panic lines)."""
    rows: dict[str, dict[Key, tuple[bool, str]]] = {}
    for report in sorted(root.glob(f"{ARTIFACT_PREFIX}*/junit.xml")):
        row = rows.setdefault(eps_row(report.parent.name), {})
        for case in ET.parse(report).getroot().iter("testcase"):  # noqa: S314 — written by this workflow
            if case.find("skipped") is not None:
                continue
            failed = case.find("failure") is not None or case.find("error") is not None
            row[(case.get("classname") or "", case.get("name") or "")] = (not failed, _panic(case))
    return rows


def newly_red(tonight: dict, last: dict, slow: set[Key]) -> list[dict]:
    order = [e for e in EPS_ORDER if e in tonight] + sorted(set(tonight) - set(EPS_ORDER))
    found: dict[Key, dict] = {}
    for eps in order:
        before = last.get(eps, {})
        for key, (passed, panic) in sorted(tonight[eps].items()):
            if passed or key not in slow or key in found:
                continue
            if before.get(key, (False, ""))[0]:
                found[key] = {"binary": key[0], "test": key[1], "eps": eps, "panic": panic}
    return list(found.values())


def slow_keys() -> set[Key]:
    with open(slow_set.CONFIG, encoding="utf-8") as fh:
        return set(slow_set.entries(slow_set.ci_filter(fh.read())))


# --------------------------------------------------------------------------
# bisect
# --------------------------------------------------------------------------


def git(*args: str, cwd: str | None = None) -> str:
    return subprocess.run(["git", *args], cwd=cwd, check=True, capture_output=True, text=True).stdout


def is_docs(sha: str, cwd: str | None = None) -> bool:
    """Whether `sha`'s own diff against its first parent is TIER=docs."""
    files = git("diff", "--name-only", f"{sha}^1", sha, cwd=cwd)
    if not files.strip():
        return True
    tier = subprocess.run(
        [sys.executable, str(Path(__file__).resolve().parent / "ci-filter.py"), "--files", "-"],
        input=files, capture_output=True, text=True, cwd=cwd, check=False,
    ).stdout
    return "TIER=docs" in tier.split()


def candidates(base: str, head: str, cwd: str | None = None,
               docs: Callable[[str], bool] | None = None) -> list[str]:
    """`base..head`'s first-parent commits, oldest first, that can move a test."""
    shas = git("rev-list", "--first-parent", "--reverse", f"{base}..{head}", cwd=cwd).split()
    docs = docs or (lambda s: is_docs(s, cwd))
    return [s for s in shas if not docs(s)]


def bisect(base: str, cands: list[str], probe: Callable[[str], str],
           out_of_time: Callable[[], bool] = lambda: False) -> dict:
    """Bisect `cands` (oldest first, after `base`) for the first that fails.

    `probe(sha)` answers `pass`, `fail` or `skip`. The verdict is `attributed`
    (with `first_red` and `last_green`, and `skipped` commits between them,
    when the answer is a range), `not-reproduced`, `red-at-base`,
    `out-of-time`, or `no-candidates`."""
    probes: list[tuple[str, str]] = []

    def p(sha: str) -> str:
        r = probe(sha)
        probes.append((sha, r))
        return r

    if not cands:
        return {"verdict": "no-candidates", "probes": probes}
    head = p(cands[-1])
    if head != "fail":
        return {"verdict": "not-reproduced", "probes": probes, "head": head}
    if out_of_time():
        return {"verdict": "out-of-time", "probes": probes}
    at_base = p(base)
    if at_base != "pass":
        return {"verdict": "red-at-base", "probes": probes, "base": at_base}
    lo, hi, skipped = -1, len(cands) - 1, set()
    while hi - lo > 1:
        if out_of_time():
            return {"verdict": "out-of-time", "probes": probes,
                    "last_green": cands[lo] if lo >= 0 else base, "first_red": cands[hi]}
        mid = (lo + hi) // 2
        open_ = [i for i in range(lo + 1, hi) if i not in skipped]
        if not open_:
            break
        i = min(open_, key=lambda j: (abs(j - mid), j))
        r = p(cands[i])
        if r == "fail":
            hi = i
        elif r == "pass":
            lo = i
        else:
            skipped.add(i)
    return {
        "verdict": "attributed",
        "probes": probes,
        "last_green": cands[lo] if lo >= 0 else base,
        "first_red": cands[hi],
        "skipped": [cands[i] for i in sorted(skipped) if lo < i < hi],
    }


def nextest_probe(test: dict, worktree: str, log: Callable[[str], None]) -> Callable[[str], str]:
    pkg = test["binary"].split("::")[0]
    env = dict(os.environ)
    env.pop("CAD_TOLERANCE_EPS", None)
    if test["eps"] != "default":
        env["CAD_TOLERANCE_EPS"] = test["eps"]

    def probe(sha: str) -> str:
        git("checkout", "--quiet", "--detach", "--force", sha, cwd=worktree)
        cmd = ["cargo", "nextest", "run", "--profile", "ci", "--ignore-default-filter",
               "--no-fail-fast", "-p", pkg, "-E",
               f"binary_id({test['binary']}) & test(={test['test']})"]
        t0 = time.monotonic()
        rc = subprocess.run(cmd, cwd=worktree, env=env, check=False,
                            stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL).returncode
        verdict = "pass" if rc == 0 else "fail" if rc == EXIT_TEST_FAILED else "skip"
        log(f"{sha[:10]} {verdict} (rc {rc}, {time.monotonic() - t0:.0f} s)")
        return verdict

    return probe


# --------------------------------------------------------------------------
# write
# --------------------------------------------------------------------------


def merge_of(sha: str, cwd: str | None = None) -> dict:
    subject = git("log", "-1", "--format=%s", sha, cwd=cwd).strip()
    m = MERGE_RE.match(subject)
    return {"sha": sha, "subject": subject,
            "pr": int(m.group(1)) if m else None, "branch": m.group(2) if m else None}


def _front(text: str) -> dict[str, str]:
    if not text.startswith("---\n"):
        return {}
    head = text[4:].split("\n---\n", 1)[0]
    return dict(ln.split(":", 1) for ln in head.splitlines() if ":" in ln)


def live_rows_naming(root: Path, test: str) -> list[Path]:
    """Every live (not closed) item under work/ whose text names the test:
    its full path, or its leaf as a whole word when the leaf is distinctive."""
    leaf = test.split("::")[-1]
    pats = [re.compile(re.escape(test))]
    if len(leaf) >= 20:
        pats.append(re.compile(rf"(?<![A-Za-z0-9_]){re.escape(leaf)}(?![A-Za-z0-9_])"))
    hits = []
    for path in sorted((root / "work").glob("*/*.md")):
        if path.name in NARRATIVE:
            continue
        text = path.read_text(encoding="utf-8")
        if _front(text).get("status", "").strip() in LIVE_EXCLUDES:
            continue
        if any(p.search(text) for p in pats):
            hits.append(path)
    return hits


def _ref(m: dict) -> str:
    pr = f"PR {m['pr']} " if m.get("pr") else ""
    branch = f", `{m['branch']}`" if m.get("branch") else ""
    return f"{pr}(`{m['sha'][:10]}`{branch})"


def facts(r: dict) -> str:
    t = r["test"]
    lines = [
        f"- **Test:** `{t['binary']}` `{t['test']}`, red in the nightly's ε `{t['eps']}` row.",
        f"- **First red merge:** {_ref(r['first_red_merge'])}.",
        f"- **Last green:** {_ref(r['last_green_merge'])}.",
    ]
    if r["result"].get("skipped"):
        lines.append("- **A range, not one merge:** these commits between them did not build or no "
                     "longer held the test, so they were not probed to an answer: "
                     + ", ".join(f"`{s[:10]}`" for s in r["result"]["skipped"]) + ".")
    lines.append(f"- **Nightly run:** {r['run']}.")
    panic = t.get("panic") or "(no panic text in the junit report)"
    lines.append("- **The panic** (its first lines, verbatim):\n\n  ```\n" + "\n".join(f"  {ln}" for ln in panic.splitlines()) + "\n  ```")
    lines.append("- **Probes** (alone, at the nightly's build profile):")
    lines.extend(f"  - `{sha[:10]}`: {v}" for sha, v in r["result"]["probes"])
    lines.append("")
    lines.append("Found by the nightly attribution; not triaged.")
    return "\n".join(lines) + "\n"


def write_one(root: Path, r: dict, today: str, work_py: Path) -> list[str]:
    t = r["test"]
    holders = live_rows_naming(root, t["test"])
    if holders:
        for path in holders:
            text = path.read_text(encoding="utf-8").rstrip("\n")
            path.write_text(f"{text}\n\n## Nightly attribution ({today})\n\n{facts(r)}", encoding="utf-8")
        return [str(p.relative_to(root)) for p in holders]
    first = r["first_red_merge"]
    prefix = (first.get("branch") or "").split("/")[0]
    program = prefix if prefix and (root / "work" / prefix / "program.md").is_file() else "green"
    leaf = t["test"].split("::")[-1].replace("_", "-").lower()
    item = f"main-red-{leaf}"
    if (root / "work" / program / f"{item}.md").exists():
        item = f"{item}-{today}"
    since = f"PR {first['pr']}" if first.get("pr") else first["sha"][:10]
    rel = subprocess.run(
        [sys.executable, str(work_py), "--root", str(root), "new", item, "--kind", "issue",
         "--title", f"main red: {t['binary']} {t['test']} since {since}", "--program", program,
         "--set", "priority=P1", "--set", "cost=E"],
        check=True, capture_output=True, text=True).stdout.strip()
    path = root / rel
    text = path.read_text(encoding="utf-8").rstrip("\n")
    path.write_text(f"{text}\n\n## What\n\n{facts(r)}", encoding="utf-8")
    return [rel]


def summary(results: list[dict]) -> str:
    out = ["### Nightly attribution", ""]
    if not results:
        out.append("No slow test is newly red tonight.")
    for r in results:
        t, res = r["test"], r["result"]
        name = f"`{t['binary']}` `{t['test']}` (ε {t['eps']})"
        v = res["verdict"]
        if v == "attributed":
            out.append(f"- {name}: first red {_ref(r['first_red_merge'])}, last green "
                       f"{_ref(r['last_green_merge'])}; {len(res['probes'])} probes; filed in "
                       + ", ".join(f"`{w}`" for w in r.get("written", [])) + ".")
        else:
            why = {
                "not-reproduced": "passes alone at tonight's head: not filed",
                "red-at-base": "fails alone at last night's head too: not newly red, not filed",
                "out-of-time": "the night's attribution budget ran out: not filed",
                "over-cap": f"past the night's cap of {CAP} bisects: not filed",
                "no-candidates": "no commit since last night can move a test: not filed",
            }.get(v, v)
            out.append(f"- {name}: {why}.")
    return "\n".join(out) + "\n"


# --------------------------------------------------------------------------
# commands
# --------------------------------------------------------------------------


def cmd_bisect(a: argparse.Namespace) -> None:
    tests = json.loads(Path(a.tests).read_text())
    deadline = time.monotonic() + a.minutes * 60
    late = lambda: time.monotonic() > deadline  # noqa: E731
    cands = candidates(a.base, a.head)
    run = (f"{os.environ.get('GITHUB_SERVER_URL', '')}/{os.environ.get('GITHUB_REPOSITORY', '')}"
           f"/actions/runs/{os.environ.get('GITHUB_RUN_ID', '')}")
    print(f"{len(cands)} commits since {a.base[:10]} can move a test; {len(tests)} newly red")
    results = []
    for i, t in enumerate(tests):
        if i >= a.cap:
            res = {"verdict": "over-cap", "probes": []}
        elif late():
            res = {"verdict": "out-of-time", "probes": []}
        else:
            print(f"bisecting {t['binary']} {t['test']} at ε {t['eps']}", flush=True)
            res = bisect(a.base, cands, nextest_probe(t, a.worktree, lambda s: print("  " + s, flush=True)),
                         late)
        r = {"test": t, "result": res, "run": f"{run} (ε `{t['eps']}`)"}
        if res["verdict"] == "attributed":
            r["first_red_merge"] = merge_of(res["first_red"])
            r["last_green_merge"] = merge_of(res["last_green"])
        results.append(r)
        Path(a.out).write_text(json.dumps(results, indent=1))


def cmd_write(a: argparse.Namespace) -> None:
    root = Path(a.root)
    results = json.loads(Path(a.results).read_text())
    today = dt.date.today().isoformat()
    work_py = Path(__file__).resolve().parent / "work.py"
    for r in results:
        if r["result"]["verdict"] == "attributed":
            r["written"] = write_one(root, r, today, work_py)
            print("filed: " + ", ".join(r["written"]))
    Path(a.results).write_text(json.dumps(results, indent=1))


def main(argv: list[str]) -> int:
    if argv == ["--selftest"]:
        selftest()
        return 0
    ap = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    sub = ap.add_subparsers(dest="cmd", required=True)
    n = sub.add_parser("newly-red")
    n.add_argument("--tonight", required=True)
    n.add_argument("--last", required=True)
    n.add_argument("--out", required=True)
    b = sub.add_parser("bisect")
    for k in ("--tests", "--base", "--head", "--worktree", "--out"):
        b.add_argument(k, required=True)
    b.add_argument("--cap", type=int, default=CAP)
    b.add_argument("--minutes", type=float, default=240)
    w = sub.add_parser("write")
    w.add_argument("--results", required=True)
    w.add_argument("--root", default=".")
    s = sub.add_parser("summary")
    s.add_argument("--results", required=True)
    a = ap.parse_args(argv)
    if a.cmd == "newly-red":
        red = newly_red(outcomes(Path(a.tonight)), outcomes(Path(a.last)), slow_keys())
        Path(a.out).write_text(json.dumps(red, indent=1))
        print(f"{len(red)} slow tests newly red")
        for t in red:
            print(f"  {t['binary']} {t['test']} (ε {t['eps']})")
    elif a.cmd == "bisect":
        cmd_bisect(a)
    elif a.cmd == "write":
        cmd_write(a)
    else:
        print(summary(json.loads(Path(a.results).read_text())))
    return 0


# --------------------------------------------------------------------------
# selftest
# --------------------------------------------------------------------------


def _junit(cases: list[tuple[str, str, str]]) -> str:
    body = []
    for binary, name, state in cases:
        inner = {"pass": "", "skip": "<skipped/>",
                 "fail": f"<failure type=\"test failure\">thread '{name}' panicked at a.rs:1:2:\n"
                         "assertion failed\n  left: 18\n right: 10\nstack backtrace:</failure>"}[state]
        body.append(f'<testcase classname="{binary}" name="{name}">{inner}</testcase>')
    return f'<testsuites><testsuite name="x">{"".join(body)}</testsuite></testsuites>'


def selftest() -> None:
    assert eps_row("full-suite-junit-1e-12-2of3") == "1e-12"
    assert eps_row("full-suite-junit-default") == "default"
    with tempfile.TemporaryDirectory() as tmp:
        tmp_p = Path(tmp)
        for night, rows in {
            "last": {"default": [("s::all", "m::slow", "pass"), ("s::all", "m::was_red", "fail"),
                                 ("s::all", "m::fast", "pass"), ("g", "k::deep", "pass")],
                     "1e-12-1of3": [("g", "k::deep", "pass")]},
            "tonight": {"default": [("s::all", "m::slow", "fail"), ("s::all", "m::was_red", "fail"),
                                    ("s::all", "m::fast", "fail"), ("s::all", "m::new", "fail"),
                                    ("g", "k::deep", "pass")],
                        "1e-12-1of3": [("g", "k::deep", "fail")],
                        "1e-12-2of3": [("s::all", "m::skipped", "skip")]},
        }.items():
            for art, cases in rows.items():
                d = tmp_p / night / f"{ARTIFACT_PREFIX}{art}"
                d.mkdir(parents=True)
                (d / "junit.xml").write_text(_junit(cases))
        slow = {("s::all", "m::slow"), ("s::all", "m::was_red"), ("s::all", "m::new"), ("g", "k::deep")}
        red = newly_red(outcomes(tmp_p / "tonight"), outcomes(tmp_p / "last"), slow)
        got = [(t["binary"], t["test"], t["eps"]) for t in red]
        assert got == [("s::all", "m::slow", "default"), ("g", "k::deep", "1e-12")], (
            f"only slow tests green last night in the same row: {got}")
        assert red[0]["panic"].startswith("thread 'm::slow' panicked at a.rs:1:2:"), red[0]["panic"]
        assert "stack backtrace" not in red[0]["panic"]

    # Bisect over a scripted history: index 5 is the first red.
    cands = [f"c{i}" for i in range(9)]

    def scripted(first_red: int, skip: frozenset[int] = frozenset()) -> Callable[[str], str]:
        def probe(sha: str) -> str:
            if sha == "base":
                return "pass"
            i = int(sha[1:])
            return "skip" if i in skip else "fail" if i >= first_red else "pass"
        return probe

    r = bisect("base", cands, scripted(5))
    assert (r["verdict"], r["first_red"], r["last_green"], r["skipped"]) == ("attributed", "c5", "c4", []), r
    assert len(r["probes"]) <= 2 + 4, f"head, base and log2(9) probes: {r['probes']}"
    r = bisect("base", cands, scripted(0))
    assert (r["first_red"], r["last_green"]) == ("c0", "base"), r
    r = bisect("base", cands, scripted(5, frozenset({4})))
    assert (r["first_red"], r["last_green"], r["skipped"]) == ("c5", "c3", ["c4"]), f"a range: {r}"
    r = bisect("base", cands, lambda s: "pass")
    assert r["verdict"] == "not-reproduced", r
    r = bisect("base", cands, lambda s: "fail")
    assert r["verdict"] == "red-at-base", r
    r = bisect("base", [], lambda s: "fail")
    assert r["verdict"] == "no-candidates", r
    calls = iter([False, False, True])
    r = bisect("base", cands, scripted(5), lambda: next(calls, True))
    assert r["verdict"] == "out-of-time", r

    # First-parent candidates over a real history, docs-only commits dropped.
    with tempfile.TemporaryDirectory() as repo:
        def g(*args: str) -> str:
            return git(*args, cwd=repo)
        g("init", "-q", "-b", "main")
        g("config", "user.email", "t@example.invalid")
        g("config", "user.name", "t")
        Path(repo, "a.rs").write_text("0\n")
        g("add", ".")
        g("commit", "-qm", "base")
        base = g("rev-parse", "HEAD").strip()
        for i, name in enumerate(("a.rs", "notes.md", "a.rs")):
            g("checkout", "-qb", f"onecopy/b{i}")
            Path(repo, name).write_text(f"{i + 1}\n")
            g("add", ".")
            g("commit", "-qm", f"side {i}")
            g("checkout", "-q", "main")
            g("merge", "-q", "--no-ff", f"onecopy/b{i}", "-m",
              f"Merge pull request #{100 + i} from evgunter/onecopy/b{i}")
        head = g("rev-parse", "HEAD").strip()
        got = candidates(base, head, cwd=repo, docs=lambda s: "notes.md" in git(
            "diff", "--name-only", f"{s}^1", s, cwd=repo))
        subjects = [merge_of(s, cwd=repo)["pr"] for s in got]
        assert subjects == [100, 102], f"first-parent merges, the docs-only one dropped: {subjects}"
        m = merge_of(got[-1], cwd=repo)
        assert (m["pr"], m["branch"]) == (102, "onecopy/b2"), m

    # Writing: a live row that names the test gets a section; otherwise a new
    # row under the merge's program, or `green`; a closed row is not reopened.
    with tempfile.TemporaryDirectory() as root:
        rp = Path(root)
        work_py = Path(__file__).resolve().parent / "work.py"
        for prog in ("onecopy", "green"):
            (rp / "work" / prog).mkdir(parents=True)
            (rp / "work" / prog / "program.md").write_text(f"---\nid: {prog}\nkind: program\n---\n")
        leaf = "a_lofted_arc_wall_is_a_function_of_the_sections_bulge"
        (rp / "work/onecopy/holder.md").write_text(
            f"---\nid: holder\nkind: issue\ntitle: t\nstatus: open\n---\n\n`{leaf}` reads 18.\n")
        (rp / "work/onecopy/old.md").write_text(
            f"---\nid: old\nkind: issue\ntitle: t\nstatus: closed\n---\n\n`{leaf}` was fixed.\n")
        res = {"verdict": "attributed", "probes": [("f" * 40, "fail"), ("b" * 40, "pass")],
               "skipped": []}
        first = {"sha": "f" * 40, "pr": 6484, "branch": "onecopy/x", "subject": ""}
        last = {"sha": "b" * 40, "pr": 6480, "branch": "onehome/y", "subject": ""}

        def r_of(test: str, first_m: dict) -> dict:
            return {"test": {"binary": "sweep::all", "test": test, "eps": "default",
                             "panic": "thread 'x' panicked at a.rs:1:2:\n  left: 18"},
                    "result": res, "run": "https://example.invalid/run (ε `default`)",
                    "first_red_merge": first_m, "last_green_merge": last}

        written = write_one(rp, r_of(f"a_loft_computes_its_walls_in_t::{leaf}", first), "2026-11-16", work_py)
        assert written == ["work/onecopy/holder.md"], f"the live holder only: {written}"
        text = (rp / "work/onecopy/holder.md").read_text()
        assert "## Nightly attribution (2026-11-16)" in text and "PR 6484" in text, text
        assert "Found by the nightly attribution; not triaged." in text
        assert "## Nightly attribution" not in (rp / "work/onecopy/old.md").read_text()
        written = write_one(rp, r_of("m::a_test_no_row_has_named_yet", first), "2026-11-16", work_py)
        assert written == ["work/onecopy/main-red-a-test-no-row-has-named-yet.md"], written
        row = (rp / written[0]).read_text()
        for want in ("kind: issue", "status: open", "priority: P1", "cost: E",
                     "title: main red: sweep::all m::a_test_no_row_has_named_yet since PR 6484",
                     "## What", "`ffffffffff`: fail"):
            assert want in row, f"{want!r} not in the new row:\n{row}"
        other = dict(first, branch="nosuchprogram/z")
        written = write_one(rp, r_of("m::a_second_test_no_row_names", other), "2026-11-16", work_py)
        assert written == ["work/green/main-red-a-second-test-no-row-names.md"], written
        written = write_one(rp, r_of("m::a_second_test_no_row_names", other), "2026-11-16", work_py)
        assert written == ["work/green/main-red-a-second-test-no-row-names.md"], (
            f"the row just written is live and names the test, so it takes the next night's section: {written}")
    s = summary([{"test": {"binary": "b", "test": "t", "eps": "default"},
                  "result": {"verdict": "over-cap", "probes": []}, "run": ""}])
    assert "past the night's cap" in s, s
    print("nightly-attribute: selftest ok")


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
