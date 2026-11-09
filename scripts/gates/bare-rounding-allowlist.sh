#!/usr/bin/env bash
# bare-rounding-allowlist.sh — a bare `f64::EPSILON` or `UNIT_ROUNDOFF`,
# or a call on geom-core's rounding charge, outside `geom-core` is a
# finding, and the list of today's sites only shrinks. ONE home; ci.yml's `lint` job runs every gate in this
# directory.
#
# THE RULE (DESIGN `[simple.conditioned]`; `crates/geom-core/README.md`,
# "Float rounding"). Every computation is posed well-conditioned where
# its result is decided, and at `f64` is then treated as exact. The one
# statement about rounding is the run's admission rule
# (`geom_core::tolerance::admissible`), and no site charges, pads or
# estimates rounding. A site that names the unit roundoff is therefore
# either a test's oracle or a formula or stored form owed a re-posing.
# This gate is the backstop: it cannot tell a charge from an oracle, so
# it makes every site a listed decision, and refuses a new one.
#
# THE SCAN is every `.rs` file under `crates/*/` (but `crates/geom-core/`),
# `demos/*/`, `tools/*/`, `benches/` and `interval-transcendentals/`,
# comments and string literals stripped. A site is `f64::EPSILON` (any
# path ending in it), a call of `carrier_resolution::resolution(`,
# geom-brep's charge for a carrier's stored terms, or a use of one of
# geom-core's CHARGE DOORS: every `pub` item at the top of
# `geom_core::running`, read from that file (`doors`), so a door declared
# there is counted the day it lands. A fn or constant counts by any path
# through `running` or, for one the root re-exports, through `geom_core`;
# a type (`Rounded`) by a construction, `Rounded::…` or `Rounded { … }`,
# never by naming it in a signature. Bare, a door counts where a `use`
# in scope binds it (its block, or the file for a top-level one; an `as`
# alias and a glob bind too), and anywhere if no other item of the tree
# shares its name; a generic name (`dot`, `cross`) counts only under its
# import. The `use` itself is not a site.
# `BARE_ROUNDING_LINES=a.rs,b.rs` prints those files' sites.
# stored terms. A charge read through a helper is the same decision as a
# bare one, so it is counted under the same ratchet. Each file's sites are
# counted in two scopes: `test`, for a file under a `tests/`, `benches/`
# or `examples/` directory, under `demos/`, `tools/` or `benches/`, a
# file a `#[cfg(test)] mod` mounts (`lib.sh`'s `gate_test_only_mounts`),
# or a site inside a `#[cfg(test)] mod { … }` block; and `prod`
# otherwise.
#
# THE LIST is `bare-rounding-sites.tsv` beside this file, one row per
# (file, scope): file, scope, count, reason. The reason is one word, or
# for a file whose sites are of more than one kind, its words joined by
# commas (`re-pose,far-centre`). Every `prod` row other than
# `exact-arith` and `fixture` is scheduled by
# `work/oneround/site-rounding-charges-owed-a-re-posing.md`, which lists
# its file (a `far-centre` site there points on to the far-centre conic
# row); `oracle` rows are a test's own reading of the unit roundoff,
# not a charge, and are not expected to shrink.
#
#   oracle      a test's expectation, a fixture's pose or a probe's
#               scale (`test` scope only);
#   re-pose     a production formula or stored form that charges its own
#               rounding, owed a re-posing (a distance form, a pose
#               decided first) so that the charge can go;
#   pose        a production pose decision reading rounding as its zero
#               (a sign of an exact value), owed a decision at the band;
#   far-centre  a production charge for a conic stored far from the arc
#               it serves, which waits on
#               `work/oneround/a-conics-point-is-evaluated-from-its-far-centre-not-its-vertex.md`;
#   exact-arith an error-free transformation (a compensated sum or
#               product), which reads the unit roundoff to compute
#               exactly, not to charge;
#   fixture     test-support code compiled into the library (`prod`
#               scope only).
#
# WHAT REDS:
#
#   UNLISTED  a (file, scope) holding more sites than its row says, or
#             holding sites with no row;
#   STALE     a row whose (file, scope) holds fewer sites than it says
#             (a site went: lower or drop the row in the same change, so
#             the ratchet holds);
#   REASON    a row whose reason (or one of its words) is not one of
#             the six, `oracle` on a `prod` row, or `fixture` on a `test`
#             row.
#
# KNOWN GAPS. A token scan sees spellings, not values: a literal
# `2.220446049250313e-16`, `2f64.powi(-52)`, `f64::EPSILON` re-exported
# under another name, a crate-local constant, function, field or
# variable carrying a charge (its making is a site; its uses are not,
# so one `noise` read by three decisions is one site), a `Rounded` value
# carried through arithmetic or its methods (only the paths that make
# one are counted), a door re-exported by a crate other than geom-core,
# a bare generic door bound by an import in a parent file and reached
# through `use super::*`, and a bare door fn passed point-free.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

LIST=scripts/gates/bare-rounding-sites.tsv

gate() {
  gate_require_file "$LIST"
  gate_require_crate_sources
  local mounts report
  mounts=$(mktemp)
  gate_test_only_mounts "${GATE_SOURCE_FILES[@]}" > "$mounts"
  if [ -e "$GATE_MATCHER_FAILED" ]; then
    rm -f "$GATE_MATCHER_FAILED" "$mounts"
    gate_error "$(gate_name): the test-only mounts were not decided (diagnosed above) — that is not a pass"
    exit 1
  fi
  if ! report=$(python3 - "$LIST" "$mounts" <<'PY'
import functools, glob, os, re, sys

list_path, mounts_path = sys.argv[1], sys.argv[2]
RUNNING, LIB = "crates/geom-core/src/running.rs", "crates/geom-core/src/lib.rs"
FIXED = re.compile(r"\bf64::EPSILON\b|\bcarrier_resolution::resolution\s*\(")
USE = re.compile(r"\buse\s+([^;]*);")
REASONS = {"oracle", "re-pose", "pose", "far-centre", "exact-arith", "fixture"}


def strip(text):
    out, i, n = [], 0, len(text)
    while i < n:
        c, two = text[i], text[i:i + 2]
        if two == "//":
            j = text.find("\n", i)
            i = n if j < 0 else j
        elif two == "/*":
            depth, j = 1, i + 2
            while j < n and depth:
                if text[j:j + 2] == "/*":
                    depth, j = depth + 1, j + 2
                elif text[j:j + 2] == "*/":
                    depth, j = depth - 1, j + 2
                else:
                    j += 1
            out.append("\n" * text.count("\n", i, j))
            i = j
        elif c == "r" and re.match(r'r#*"', text[i:i + 8]) and not (i and (text[i - 1].isalnum() or text[i - 1] == "_")):
            hashes = len(re.match(r"r(#*)", text[i:]).group(1))
            close = '"' + "#" * hashes
            j = text.find(close, i + 2 + hashes)
            j = n if j < 0 else j + len(close)
            out.append('""')
            i = j
        elif c == '"':
            j = i + 1
            while j < n and text[j] != '"':
                j += 2 if text[j] == "\\" else 1
            out.append('""')
            i = j + 1
        elif c == "'":
            m = re.match(r"'(?:\\(?:x[0-9a-fA-F]{2}|u\{[0-9a-fA-F]+\}|.)|[^'\\\n])'", text[i:])
            if m:
                out.append("' '")
                i += m.end()
            else:
                out.append(c)
                i += 1
        else:
            out.append(c)
            i += 1
    return "".join(out)


def matching(text, at):
    depth = 0
    for i in range(at, len(text)):
        if text[i] == "{":
            depth += 1
        elif text[i] == "}":
            depth -= 1
            if depth == 0:
                return i
    return len(text) - 1


mounts = [l.strip() for l in open(mounts_path, encoding="utf-8") if l.strip()]


def test_only(path):
    parts = path.split("/")
    if parts[0] in ("demos", "tools", "benches") or any(p in ("tests", "benches", "examples") for p in parts[1:]):
        return True
    return any(path.startswith(m) if m.endswith("/") else path == m for m in mounts)


@functools.lru_cache(maxsize=None)
def read_stripped(path):
    with open(path, encoding="utf-8") as fh:
        return strip(fh.read())


def doors():
    """The charge doors: every `pub` item at the top of `geom_core::running`,
    by kind, and the ones `geom_core`'s root re-exports."""
    for need in (RUNNING, LIB):
        if not os.path.isfile(need):
            sys.exit(f"REFUSED {need} is missing: the charge doors cannot be read")
    kinds = {}
    for kw, name in re.findall(r"^pub\s+(fn|const|static|struct|enum)\s+(\w+)", read_stripped(RUNNING), re.M):
        kinds[name] = "fn" if kw == "fn" else "value" if kw in ("const", "static") else "type"
    if not kinds:
        sys.exit(f"REFUSED {RUNNING} declares no `pub` item: the charge doors cannot be read")
    root = set()
    for m in re.finditer(r"\bpub\s+use\s+running::(\{[^}]*\}|\w+)\s*;", read_stripped(LIB)):
        root |= set(re.findall(r"\w+", m.group(1))) & set(kinds)
    return kinds, root


def use_items(tree):
    """`(segments, alias)` for each leaf of a `use` tree."""
    toks = re.findall(r"\w+|::|[{},*]", tree)
    out, i = [], 0

    def items(prefix):
        nonlocal i
        while i < len(toks) and toks[i] != "}":
            segs = []
            while i < len(toks) and toks[i] not in (",", "}", "{"):
                if toks[i] != "::":
                    segs.append(toks[i])
                i += 1
            alias = None
            if len(segs) >= 3 and segs[-2] == "as":
                segs, alias = segs[:-2], segs[-1]
            if i < len(toks) and toks[i] == "{":
                i += 1
                items(prefix + segs)
                i += 1
            elif segs:
                out.append((prefix + segs, alias))
            if i < len(toks) and toks[i] == ",":
                i += 1

    items([])
    return out


KINDS, ROOT = doors()


def blocks(text):
    """Every `{ … }` span of a file, as `(open, close)`."""
    out, stack = [], []
    for i, c in enumerate(text):
        if c == "{":
            stack.append(i)
        elif c == "}" and stack:
            out.append((stack.pop(), i))
    return out


def scope_of(spans, at, n):
    """The innermost block holding `at`, or the whole file."""
    inner = [(o, c) for o, c in spans if o < at < c]
    return max(inner, key=lambda oc: oc[0]) if inner else (0, n)


def bindings(text):
    """The local names a file's imports give the doors, each with the span
    its `use` is in scope over, and the local names of the `running`
    module itself."""
    names, modules, spans = [], {"running"}, blocks(text)
    for m in USE.finditer(text):
        scope = scope_of(spans, m.start(), len(text))
        for segs, alias in use_items(m.group(1)):
            segs = [x for x in segs if x not in ("crate", "self", "super") or x == segs[0]]
            if segs[-1] == "self":
                segs = segs[:-1]
            if len(segs) < 2:
                continue
            head, last = segs[-2], segs[-1]
            if last == "running" and head == "geom_core":
                modules.add(alias or last)
            elif last == "*" and head == "running":
                names += [(n, n, scope) for n in KINDS]
            elif last == "*" and segs == ["geom_core", "*"]:
                names += [(n, n, scope) for n in ROOT]
            elif head == "running" and last in KINDS:
                names.append((alias or last, last, scope))
            elif segs == ["geom_core", last] and last in ROOT:
                names.append((alias or last, last, scope))
    return names, modules


def declared_elsewhere():
    """Door names some other file of the tree declares too: a bare one of
    those is read only where an import binds it."""
    alt = "|".join(map(re.escape, KINDS))
    decl = re.compile(r"\b(?:fn|const|static|struct|enum|type|trait|mod)\s+(" + alt + r")\b")
    found = set()
    for path in glob.glob("crates/*/**/*.rs", recursive=True) + glob.glob("interval-transcendentals/**/*.rs", recursive=True):
        if path == RUNNING or "/target/" in path:
            continue
        found |= set(decl.findall(read_stripped(path)))
    return found


SHARED = declared_elsewhere()
SHAPE = {"fn": r"\s*(?:\(|::\s*<)", "value": r"\b", "type": r"\s*(?:::|\{)"}


def sites(text):
    """The offsets of one file's sites: a bare `f64::EPSILON` or a
    `carrier_resolution` call, and every use of a door — by a path through
    `running` (any shape for a fn or a value, a construction for a type),
    through the root for a re-exported one, or bare where an import in
    scope binds the name or no other item of the tree shares it. An import
    is not a site."""
    names, modules = bindings(text)
    code = USE.sub(lambda m: " " * len(m.group(0)), text)
    alts = [FIXED.pattern]
    mods = "|".join(map(re.escape, sorted(modules)))
    for name, kind in KINDS.items():
        shape = SHAPE[kind] if kind == "type" else r"\b"
        alts.append(r"\b(?:" + mods + r")::" + re.escape(name) + shape)
        if name in ROOT:
            alts.append(r"\bgeom_core::" + re.escape(name) + shape)
    names += [(n, n, (0, len(code))) for n in KINDS if n not in SHARED]
    at = {m.start() for m in re.finditer("|".join(alts), code)}
    for name, door, (lo, hi) in names:
        bare = re.compile(r"(?<![\w.:])" + re.escape(name) + SHAPE[KINDS[door]])
        at |= {m.start() for m in bare.finditer(code, lo, hi)}
    return sorted(at)


def test_spans(text):
    """The spans of a file's `#[cfg(test)] mod { … }` blocks."""
    out, at = [], 0
    for m in re.finditer(r"#\[cfg\(test\)\]\s*(?:#\[[^\]]*\]\s*)*(?:pub(?:\([^)]*\))?\s+)?mod\s+\w+\s*\{", text):
        if m.start() < at:
            continue
        at = matching(text, m.end() - 1) + 1
        out.append((m.start(), at))
    return out


paths = set()
for pattern in ("crates/*/**/*.rs", "demos/*/**/*.rs", "tools/*/**/*.rs", "benches/**/*.rs",
                "interval-transcendentals/**/*.rs"):
    paths.update(glob.glob(pattern, recursive=True))
counts = {}
for path in sorted(paths):
    if path.startswith("crates/geom-core/") or "/target/" in path:
        continue
    text = read_stripped(path)
    found = sites(text)
    if not found:
        continue
    if path in os.environ.get("BARE_ROUNDING_LINES", "").split(","):
        for k in found:
            print(f"LINES {path}: {text[k:k + 40].splitlines()[0]}")
    tests = [] if test_only(path) else test_spans(text)
    for k in found:
        scope = "test" if test_only(path) or any(lo <= k < hi for lo, hi in tests) else "prod"
        counts[(path, scope)] = counts.get((path, scope), 0) + 1

rows, bad = {}, []
for k, line in enumerate(open(list_path, encoding="utf-8"), 1):
    if not line.strip() or line.startswith("#"):
        continue
    cols = line.rstrip("\n").split("\t")
    if len(cols) != 4 or not cols[2].isdigit():
        bad.append(f"REASON {list_path}:{k}: not `file<TAB>scope<TAB>count<TAB>reason`: {line.strip()}")
        continue
    path, scope, n, reason = cols
    words = reason.split(",")
    if any(w not in REASONS for w in words) or scope not in ("prod", "test") or ("oracle" in words and scope == "prod") or ("fixture" in words and scope == "test"):
        bad.append(f"REASON {path} ({scope}): `{reason}` is not a reason this scope may carry")
    rows[(path, scope)] = int(n)

for key in sorted(set(counts) | set(rows)):
    now, was = counts.get(key, 0), rows.get(key, 0)
    if now > was:
        bad.append(f"UNLISTED {key[0]} ({key[1]}): {now} rounding sites, listed {was}")
    elif now < was:
        bad.append(f"STALE {key[0]} ({key[1]}): {now} rounding sites, listed {was}")
for b in bad:
    print(b)
print(f"TOTAL {sum(counts.values())} {sum(v for (p, s), v in counts.items() if s == 'prod')}")
PY
  ); then
    rm -f "$mounts"
    gate_error "$(gate_name): the counter failed under $PWD (its own message is above, if it had one) — the gate could not read its subject, which is not a pass"
    exit 1
  fi
  rm -f "$mounts"
  grep '^LINES ' <<<"$report" || true
  if grep -q '^UNLISTED \|^REASON ' <<<"$report"; then
    grep '^UNLISTED \|^REASON ' <<<"$report"
    gate_error "a bare f64::EPSILON / UNIT_ROUNDOFF or a rounding charge outside geom-core: no site charges rounding ([simple.conditioned]). Re-pose the formula (a distance form, a pose decided first), or, for a test's oracle, list it in $LIST with its one-word reason"
    exit 1
  fi
  if grep -q '^STALE ' <<<"$report"; then
    grep '^STALE ' <<<"$report"
    gate_error "bare rounding sites fell — lower or drop the row in $LIST, so the list only shrinks"
    exit 1
  fi
  local total prod
  read -r total prod < <(sed -n 's/^TOTAL //p' <<<"$report")
  gate_ok "bare f64::EPSILON / UNIT_ROUNDOFF and rounding charges outside geom-core at their listed counts ($total sites, $prod in production code)"
}

# The fixture's geom-core: the doors as `running` declares them, a
# generic `dot` declared again outside it, and the root's re-exports.
gate_plant_clean() {
  gate_plant_clean_sources "$1"
  mkdir -p "$1/scripts/gates" "$1/crates/geom-core/src"
  printf '# file\tscope\tcount\treason\n' > "$1/$LIST"
  {
    printf 'pub const UNIT_ROUNDOFF: f64 = f64::EPSILON * 0.5;\npub const DIRECTION_UNITS: f64 = 4.0;\n'
    printf 'pub fn rounding_charge(t: f64) -> f64 { t }\npub fn direction_rounding(t: f64) -> f64 { t }\n'
    printf 'pub fn exact_vec(v: f64) -> f64 { v }\npub fn dot(a: f64, b: f64) -> f64 { a * b }\n'
    printf 'pub struct Rounded { pub value: f64 }\nfn private_helper() {}\n'
  } > "$1/crates/geom-core/src/running.rs"
  printf 'pub mod running;\npub use running::{Rounded, UNIT_ROUNDOFF, rounding_charge};\n' > "$1/crates/geom-core/src/lib.rs"
  printf 'pub fn dot(a: f64, b: f64) -> f64 { a + b }\npub fn use_it() -> f64 { dot(1.0, 2.0) }\n' >> "$1/crates/clean/src/lib.rs"
}

listed_at() { printf 'crates/clean/src/lib.rs\tprod\t%s\tre-pose\n' "$2" >> "$1/$LIST"; }
add_clean() { printf '%s\n' "$2" >> "$1/crates/clean/src/lib.rs"; }

plant_new_site() {
  printf 'pub fn pad(x: f64) -> f64 { x * (1.0 + 4.0 * f64::EPSILON) }\n' >> "$1/crates/clean/src/lib.rs"
}

plant_new_roundoff() {
  printf 'pub fn pad(x: f64) -> f64 { x * geom_core::UNIT_ROUNDOFF }\n' >> "$1/crates/clean/src/lib.rs"
}

plant_listed() {
  printf 'crates/clean/src/lib.rs\tprod\t1\tre-pose\n' >> "$1/$LIST"
  plant_new_site "$1"
}

plant_stale() {
  printf 'crates/clean/src/lib.rs\tprod\t2\tre-pose\n' >> "$1/$LIST"
  plant_new_site "$1"
}

plant_oracle_in_prod() {
  printf 'crates/clean/src/lib.rs\tprod\t1\toracle\n' >> "$1/$LIST"
  plant_new_site "$1"
}

plant_test_module_as_prod() {
  printf 'crates/clean/src/lib.rs\tprod\t1\tre-pose\n' >> "$1/$LIST"
  printf '#[cfg(test)]\nmod tests {\n    const U: f64 = f64::EPSILON;\n}\n' >> "$1/crates/clean/src/lib.rs"
}

# A comment, a string and geom-core itself are not sites.
plant_new_charge() {
  printf 'pub fn pad(x: f64) -> f64 { x + geom_core::rounding_charge(x) }\n' >> "$1/crates/clean/src/lib.rs"
}

plant_new_running() {
  printf 'pub fn pad(x: f64) -> f64 { geom_core::running::Rounded::exact(x).value }\n' >> "$1/crates/clean/src/lib.rs"
}

plant_new_carrier_charge() {
  printf 'pub fn pad(c: &C) -> f64 { geom_brep::carrier_resolution::resolution(c, 0.0, 1.0) }\n' >> "$1/crates/clean/src/lib.rs"
}

plant_compound_reason() {
  printf 'crates/clean/src/lib.rs\tprod\t2\tre-pose,far-centre\n' >> "$1/$LIST"
  plant_new_site "$1"
  plant_new_charge "$1"
}

plant_bad_compound_reason() {
  printf 'crates/clean/src/lib.rs\tprod\t1\tre-pose,oracle\n' >> "$1/$LIST"
  plant_new_site "$1"
}

plant_not_sites() {
  printf '// f64::EPSILON in a comment\npub const S: &str = "UNIT_ROUNDOFF";\n' >> "$1/crates/clean/src/lib.rs"
  printf 'pub const OWN: f64 = f64::EPSILON * UNIT_ROUNDOFF;\n' >> "$1/crates/geom-core/src/lib.rs"
}

# Doors no spelling of the gate names: derived from `running`.
plant_direction_charge() { add_clean "$1" 'pub fn pad(x: f64) -> f64 { geom_core::running::direction_rounding(x) }'; }
plant_door_declared() {
  printf 'pub fn fresh_charge(t: f64) -> f64 { t }\n' >> "$1/crates/geom-core/src/running.rs"
  add_clean "$1" 'pub fn pad(x: f64) -> f64 { geom_core::running::fresh_charge(x) }'
}
plant_door_constant() { add_clean "$1" 'pub fn pad(x: f64) -> f64 { x * geom_core::running::DIRECTION_UNITS }'; }
plant_alias() { add_clean "$1" 'use geom_core::rounding_charge as rc;'; add_clean "$1" 'pub fn pad(x: f64) -> f64 { rc(x) }'; }
plant_module_alias() { add_clean "$1" 'use geom_core::running as r;'; add_clean "$1" 'pub fn pad(x: f64) -> f64 { r::exact_vec(x) }'; }
plant_glob() { add_clean "$1" 'mod g { use geom_core::running::*; pub fn pad(x: f64) -> f64 { dot(x, x) } }'; }
# A generic door name counts only under an import in scope: `a`'s two,
# not `b`'s, a method or the file's own `dot`.
plant_scoped_import() {
  listed_at "$1" 2
  add_clean "$1" 'pub fn a(x: f64) -> f64 { use geom_core::running::{dot, exact_vec}; dot(exact_vec(x), x) }'
  add_clean "$1" 'pub fn b(x: f64, v: V) -> f64 { v.dot(x); dot(x, x) }'
}
# An import is not a site, nor a type named in a signature.
plant_import_only() { add_clean "$1" 'use geom_core::{UNIT_ROUNDOFF, running::Rounded};'; add_clean "$1" 'pub fn f(r: Rounded) {}'; }

gate_selftest() {
  gate_selftest_clean
  gate_selftest_case "UNLISTED" plant_new_site
  gate_selftest_case "UNLISTED" plant_new_roundoff
  gate_selftest_case "UNLISTED" plant_new_charge
  gate_selftest_case "UNLISTED" plant_new_running
  gate_selftest_case "UNLISTED" plant_new_carrier_charge
  gate_selftest_case "UNLISTED" plant_direction_charge
  gate_selftest_case "UNLISTED" plant_door_declared
  gate_selftest_case "UNLISTED" plant_door_constant
  gate_selftest_case "UNLISTED" plant_alias
  gate_selftest_case "UNLISTED" plant_module_alias
  gate_selftest_case "UNLISTED" plant_glob
  gate_selftest_passes "a generic door name under its import, at its count" plant_scoped_import
  gate_selftest_passes "an import and a type in a signature" plant_import_only
  gate_selftest_case "REASON" plant_bad_compound_reason
  gate_selftest_passes "a compound reason" plant_compound_reason
  gate_selftest_case "STALE" plant_stale
  gate_selftest_case "REASON" plant_oracle_in_prod
  gate_selftest_case "UNLISTED crates/clean/src/lib.rs (test)" plant_test_module_as_prod
  gate_selftest_passes "a listed site" plant_listed
  gate_selftest_passes "a comment, a string and geom-core's own" plant_not_sites
  printf '%s selftest OK: passes a clean fixture, a listed site, and comments, strings and geom-core; fires on an unlisted site of each spelling (bare, a rounding_charge call, a Rounded path, a carrier_resolution call, a door the gate never names — direction_rounding, a constant, one declared in the fixture running module — an aliased door, an aliased running module, a glob), passes a generic door name counted only under its import in scope and an import or signature type alone, an oracle inside a compound reason, a stale row, an oracle in production and a test module counted as production\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
