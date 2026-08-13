#!/usr/bin/env python3
"""Tripwire: the `interval` cargo feature must stay PURELY ADDITIVE.

WHY. The interval CI run legs execute only the tests the feature adds
(scripts/interval-only-selection.py derives that set as an archive
difference). That is sound only while enabling `interval` cannot change
how the DEFAULT build behaves — i.e. while every
`#[cfg(feature = "interval")]` item only ADDS an `Interval`
implementation, module, alias or test, and nothing anywhere is compiled
`cfg(not(feature = "interval"))`. Rust has no specialization, so adding
impls cannot move f64 monomorphization; that is the whole argument, and
this script is what keeps its premise true.

If this ever fails, do NOT allowlist it to get green. A non-additive cfg
means the interval legs must go back to running the full suite — the
run-leg selection and this gate stand or fall together.

STATED RESIDUAL (this is a syntactic check, not a semantic one). It
cannot see through a gated `mod` whose contents are non-additive, nor
catch a gated impl of a trait with a blanket default method that an f64
path then calls. It narrows the hole; it does not close it. The
belt-and-braces is that the interval BUILD job still compiles the whole
graph with the feature on and runs clippy and the doctests over it, so a
non-additive change that breaks the f64 build is still caught there.

Usage:  check-interval-cfg-additive.py [--selftest]
Scans crates/*/src/**/*.rs relative to the repo root. Exit 1 on any
violation, naming file:line.
"""

import os
import re
import sys

FEATURE = "interval"

# A gated item may only be one of these. `#[test]`/`#[cfg(test)]` mean
# the gated thing is test-only and so cannot reach a production f64
# path. `impl`/`mod`/`use`/`type` are the additive item kinds.
ALLOWED_ITEM = re.compile(
    r"^\s*(?:pub(?:\s*\([^)]*\))?\s+)?(?:impl|mod|use|type)\b|^\s*#\[(?:test|cfg\(test\))\]"
)
GATE = re.compile(r"#\[cfg\(\s*feature\s*=\s*\"" + FEATURE + r"\"\s*\)\]")
# Any cfg that mentions the feature under a `not(...)`, or any
# `all(...)`/`any(...)` combination — all of these can make the default
# build differ, so none is allowed.
NEGATED = re.compile(r"cfg\(\s*(?:not|all|any)\s*\(.*\b" + FEATURE + r"\b")

# The one ratified statement-position gate. This is an extra
# `if let Some(v) = any.downcast_ref::<Interval>()` arm placed AFTER the
# f64 and Probe arms in `repr_bits_of`, so the f64 answer is returned
# before it is ever reached: additive by position. Ratified with the
# interval run-leg selection (see scripts/interval-only-selection.py).
ALLOWLIST = {
    ("crates/geom-core/src/bit_identity.rs", "if let Some(v) = any.downcast_ref"),
}


def sources(root):
    for crate in sorted(os.listdir(os.path.join(root, "crates"))):
        src = os.path.join(root, "crates", crate, "src")
        for dirpath, _dirs, files in os.walk(src):
            for f in sorted(files):
                if f.endswith(".rs"):
                    yield os.path.join(dirpath, f)


def next_item_line(lines, i):
    """The first line after index `i` that is not blank, not a comment,
    and not another attribute — i.e. the item the gate attaches to.
    Attribute lines are skipped, but `#[test]`/`#[cfg(test)]` short-
    circuit as allowed before we get here (see ALLOWED_ITEM)."""
    j = i + 1
    while j < len(lines):
        s = lines[j].strip()
        if not s or s.startswith("//"):
            j += 1
            continue
        if s.startswith("#[") and not ALLOWED_ITEM.match(lines[j]):
            j += 1
            continue
        return j
    return None


def check_file(path, rel, violations):
    with open(path, encoding="utf-8") as f:
        lines = f.read().splitlines()
    for i, line in enumerate(lines):
        if NEGATED.search(line):
            violations.append(
                "{}:{}: cfg negates or combines the `{}` feature — the "
                "default build's behaviour must not depend on it\n      {}".format(
                    rel, i + 1, FEATURE, line.strip()
                )
            )
        if not GATE.search(line):
            continue
        j = next_item_line(lines, i)
        if j is None:
            violations.append(
                "{}:{}: `#[cfg(feature = \"{}\")]` attaches to nothing".format(
                    rel, i + 1, FEATURE
                )
            )
            continue
        if ALLOWED_ITEM.match(lines[j]):
            continue
        if any(rel == f and frag in lines[j] for f, frag in ALLOWLIST):
            continue
        violations.append(
            "{}:{}: `#[cfg(feature = \"{}\")]` gates a non-additive item. "
            "Only impl/mod/use/type items and test-only code may be gated, "
            "so that enabling the feature cannot change the default build.\n"
            "      {}".format(rel, j + 1, FEATURE, lines[j].strip())
        )


def selftest(root):
    """Prove the guard fires. Synthetic sources, nothing on disk."""
    import tempfile

    bad = [
        ('#[cfg(feature = "interval")]\nfn helper() {}\n', "gated fn"),
        ('#[cfg(not(feature = "interval"))]\nfn helper() {}\n', "negated cfg"),
        ('#[cfg(all(feature = "interval", unix))]\nimpl Foo for Bar {}\n', "combined cfg"),
        ('#[cfg(feature = "interval")]\nconst K: u8 = 1;\n', "gated const"),
    ]
    good = [
        '#[cfg(feature = "interval")]\nimpl Lane for Interval {}\n',
        '#[cfg(feature = "interval")]\npub mod interval;\n',
        '#[cfg(feature = "interval")]\npub use interval::Interval;\n',
        '#[cfg(feature = "interval")]\npub type DualInterval = Dual<Interval>;\n',
        '#[cfg(feature = "interval")]\n#[test]\nfn t() {}\n',
        '#[cfg(feature = "interval")]\n// a comment first\nimpl Sealed for Interval {}\n',
    ]
    failures = []
    with tempfile.TemporaryDirectory() as d:
        for n, (text, label) in enumerate(bad):
            p = os.path.join(d, "bad{}.rs".format(n))
            with open(p, "w") as f:
                f.write(text)
            v = []
            check_file(p, "bad{}.rs".format(n), v)
            if not v:
                failures.append("selftest: {} was NOT caught".format(label))
        for n, text in enumerate(good):
            p = os.path.join(d, "good{}.rs".format(n))
            with open(p, "w") as f:
                f.write(text)
            v = []
            check_file(p, "good{}.rs".format(n), v)
            if v:
                failures.append("selftest: false positive on:\n{}\n{}".format(text, v))
    if failures:
        for f in failures:
            print(f, file=sys.stderr)
        raise SystemExit("selftest FAILED")
    print("selftest ok: {} violations caught, {} clean cases pass".format(len(bad), len(good)))
    return 0


def main(argv):
    root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    if len(argv) > 1 and argv[1] == "--selftest":
        return selftest(root)
    violations = []
    n = 0
    for path in sources(root):
        n += 1
        check_file(path, os.path.relpath(path, root), violations)
    if violations:
        print(
            "error: the `interval` feature is not purely additive:\n", file=sys.stderr
        )
        for v in violations:
            print("  " + v, file=sys.stderr)
        print(
            "\nThe interval CI run legs execute ONLY the tests this feature "
            "adds, which is sound only while it cannot change the default "
            "build. Read scripts/check-interval-cfg-additive.py's header "
            "before touching this — allowlisting to get green is the wrong "
            "move; if the feature is genuinely non-additive now, the "
            "interval run legs must go back to the full suite.",
            file=sys.stderr,
        )
        return 1
    print("interval cfgs are additive ({} source files scanned)".format(n))
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
