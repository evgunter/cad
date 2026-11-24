#!/usr/bin/env python3
"""The census of production reads of a conic's centre doors, taken by clippy.

Reads cargo's JSON diagnostics on stdin, from

    cargo clippy <scope> --all-targets --all-features --message-format=json \
        -- -D warnings --force-warn clippy::disallowed_methods

`--force-warn` reports every use of a door `clippy.toml` lists whatever
attribute sits over it (an allow, an expect, a group, a retired name, a
module mounted by path), so no spelling of an attribute hides a read.

WHICH READS ARE PRODUCTION: under `--all-targets` cargo checks a library
or a binary twice, once as itself and once as its test harness. A read
in production code is reported by both; one under `#[cfg(test)]` by the
harness only; one in `tests/`, `examples/` or `benches/` by a target of
another kind. So a production read is a span reported at least twice by
a `lib` or `bin` target. (Measured equal, span for span, to the census
of `--lib --bins` alone, which is a second build.)

WHICH FILES ARE HELD: those of the packages this run selected (a scoped
run selects some crates only), known by the test harness among a
package's artifacts; a pin of a crate checked only as a dependency, or
not at all, is not read. The conic's own module is not counted.

`--check` compares the count per file with the pins and exits 1 where
they differ; `--write` rewrites the pins from the census (every package
must have been checked); `--selftest` runs the comparison on planted
diagnostics.
"""
import collections
import json
import pathlib
import sys

HOME = "crates/geom/src/curves/conic.rs"
LINT = "clippy::disallowed_methods"
HEADER = (
    "# Production reads of a conic's centre doors per file, as clippy\n"
    "# reports them (scripts/conic_centre_census.py). A pin only falls.\n"
    "# A read that lands from another lane is converted to a reading of the\n"
    "# conic, or pinned here with an `#[expect]` at its function and its site\n"
    "# on a `work/conic/` row, in the PR that finds it.\n"
)


def census(lines):
    """(reads per file, crate directories checked) from cargo's JSON lines."""
    seen = collections.Counter()
    crates = set()
    for line in lines:
        line = line.strip()
        if not line.startswith("{"):
            continue
        try:
            msg = json.loads(line)
        except json.JSONDecodeError:
            continue
        manifest = msg.get("manifest_path", "")
        # A package this run selected has a test harness among its
        # artifacts; one checked only as another's dependency has not,
        # and its reads are reported once or not at all.
        selected = msg.get("reason") == "compiler-artifact" and msg.get("profile", {}).get("test")
        if selected and "/crates/" in manifest:
            crates.add("crates/" + manifest.split("/crates/")[1].split("/")[0] + "/")
        if msg.get("reason") != "compiler-message":
            continue
        diag = msg["message"]
        if (diag.get("code") or {}).get("code") != LINT:
            continue
        kind = msg["target"]["kind"][0]
        if kind not in ("lib", "bin", "rlib", "cdylib", "proc-macro"):
            continue
        for span in diag["spans"]:
            if span["is_primary"]:
                seen[(span["file_name"], span["line_start"], span["column_start"])] += 1
    reads = collections.Counter()
    for (file, _, _), times in seen.items():
        if times >= 2 and file != HOME and file.startswith("crates/"):
            reads[file] += 1
    return reads, crates


def read_pins(path):
    pins = {}
    for line in pathlib.Path(path).read_text().splitlines():
        if line and not line.startswith("#"):
            file, count = line.rsplit(" ", 1)
            pins[file] = int(count)
    return pins


def compare(reads, crates, pins):
    """The lines that say where the census and the pins differ."""
    def held(file):
        return any(file.startswith(c) for c in crates)

    out = []
    for file in sorted(set(reads) | set(pins)):
        if not held(file):
            continue
        got, want = reads.get(file, 0), pins.get(file, 0)
        if got != want:
            out.append(f"{file}: {got} production read(s) of a conic's centre doors, pinned at {want}")
    return out


def selftest():
    def message(file, line, kind):
        return json.dumps({
            "reason": "compiler-message",
            "manifest_path": "/r/crates/geom/Cargo.toml",
            "target": {"kind": [kind]},
            "message": {"code": {"code": LINT}, "spans": [
                {"is_primary": True, "file_name": file, "line_start": line, "column_start": 5}]},
        })
    def twice(file, line):
        return [message(file, line, "lib")] * 2

    harness = json.dumps({"reason": "compiler-artifact", "manifest_path": "/r/crates/geom/Cargo.toml", "profile": {"test": True}})
    clean = [
        harness,
        *twice("crates/geom/src/a.rs", 3),
        message("crates/geom/src/a.rs", 90, "lib"),
        *[message("crates/geom/tests/t.rs", 4, "test")] * 2,
        *twice(HOME, 7),
    ]
    pins = {"crates/geom/src/a.rs": 1, "crates/topo/src/unchecked.rs": 4}
    cases = [
        ("a clean census, a test-only read, the home module and an unchecked crate's pin", clean, pins, False),
        ("a new production read", [*clean, *twice("crates/geom/src/a.rs", 40)], pins, True),
        ("a read in a file with no pin", [*clean, *twice("crates/geom/src/b.rs", 1)], pins, True),
        ("a pinned read that has gone", [clean[0], *clean[3:]], pins, True),
    ]
    for name, lines, pins_, red in cases:
        reads, crates = census(lines)
        if bool(compare(reads, crates, pins_)) != red:
            print(f"conic_centre_census selftest: {name}: expected {'red' if red else 'green'}", file=sys.stderr)
            return 1
    print("conic_centre_census selftest OK: passes a clean census with a test-only read, the home module and an unchecked crate's pin; fires on a new production read, a read in an unpinned file and a pinned read that has gone")
    return 0


def main(argv):
    if "--selftest" in argv:
        return selftest()
    pins_path = argv[argv.index("--pins") + 1] if "--pins" in argv else "scripts/gates/conic-centre-reads.counts"
    reads, crates = census(sys.stdin)
    if not crates:
        print("conic_centre_census: no package in the diagnostics read, so nothing was counted, which is not a pass", file=sys.stderr)
        return 1
    if "--write" in argv:
        body = "".join(f"{f} {n}\n" for f, n in sorted(reads.items()))
        pathlib.Path(pins_path).write_text(HEADER + body)
        print(f"conic_centre_census: wrote {sum(reads.values())} reads in {len(reads)} files")
        return 0
    bad = compare(reads, crates, read_pins(pins_path))
    for line in bad:
        print(line)
    if bad:
        print("ERROR: a conic's centre door is read in production where the pins do not hold it (above): read where the conic is through its readings (crates/geom/src/curves/conic.rs), and lower the pin of a read that has gone", file=sys.stderr)
        return 1
    print(f"conic_centre_census OK: {sum(reads.values())} production reads of a conic's centre doors in {len(reads)} files, each at its pin ({len(crates)} crates checked)")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
