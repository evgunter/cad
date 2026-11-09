#!/usr/bin/env bash
# bare-rounding-allowlist.sh — a bare `f64::EPSILON`, or a use of one of
# geom-core's rounding-charge doors, outside `geom-core` is a finding,
# and each is a listed site. ONE home; ci.yml's `lint` job runs every
# gate in this directory.
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
# geom-core's CHARGE DOORS, read from geom-core itself (`doors`): every
# `pub` item at the top of `geom_core::running`, wherever on its line it
# starts (`#[inline] pub fn` included), so a door declared there is
# counted the day it lands, and a `pub` item of a kind the gate cannot
# classify, or one inside a macro's body, reds UNRECOGNISED rather than
# going uncounted. The root's
# re-exports of them (`lib.rs`'s `pub use running::…`, an `as` rename
# included) are read the same way. A fn or constant counts by any path
# through `geom_core::running` (or a name `running` is imported or
# re-exported under), or through `geom_core` (or a crate alias of it,
# direct or through another crate's re-export) for a re-exported one;
# an alias resolves only inside its own block, or the file for a
# top-level one, and `running` alone, never imported, is another crate's
# module; a
# type (`Rounded`) by a construction, `Rounded::…` or `Rounded { … }`,
# or a qualified path `<Rounded<T>>::…`, never by naming it in a
# signature. Bare, a door counts where a `use` or a `type` alias in scope
# binds it (its block, or the file for a top-level one; an `as` rename
# and a glob bind too), and anywhere if no other item of the tree shares
# its name; a generic name (`dot`, `cross`) counts only where bound. The
# `use` itself is not a site. `BARE_ROUNDING_LINES=a.rs,b.rs` prints
# those files' sites. Each file's sites are
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
#   UNRECOGNISED  a `pub` item of `running`, or a root re-export under
#             it, that is no door kind the gate reads, and any `pub`
#             inside a macro's body in `running`;
#   REASON    a row whose reason (or one of its words) is not one of
#             the six, `oracle` on a `prod` row, or `fixture` on a `test`
#             row.
#
# The counts are compared with the tree, not with the merge base, so a
# change may raise a row beside the site it adds; only review holds that
# (`work/guard/the-bare-rounding-list-rises-in-the-diff-that-adds-its-site.md`).
#
# KNOWN GAPS. A token scan sees spellings, not values: a literal
# `2.220446049250313e-16`, `2f64.powi(-52)`, `f64::EPSILON` re-exported
# under another name, a crate-local constant, function, field or
# variable carrying a charge (its making is a site; its uses are not,
# so one `noise` read by three decisions is one site), a `Rounded` value
# carried through arithmetic or its methods (only the paths that make
# one are counted), a door re-exported by a crate other than geom-core,
# a bare generic door bound by an import in a parent file and reached
# through `use super::*`, a bare door fn passed point-free, a type alias
# naming a door through another alias, and a `pub(…)`-restricted item
# of `running` (which no other crate can name).
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


ITEM = (
    ("fn", re.compile(r"(?:(?:const|unsafe|async|extern\s*(?:\"[^\"]*\")?)\s+)*fn\s+(\w+)")),
    ("value", re.compile(r"(?:const|static(?:\s+mut)?)\s+(\w+)")),
    ("type", re.compile(r"(?:struct|enum|union|type|trait)\s+(\w+)")),
)


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


def lead(segs):
    """A `use` path without its leading `crate`, `self` or `super`s."""
    while segs and segs[0] in ("crate", "self", "super"):
        segs = segs[1:]
    return segs


def pub_tokens(text):
    """Every `pub` token of `text`, with where it sits: `top` (outside
    every bracket: an item of the module, whatever precedes it on its
    line), `macro` (inside the body of a macro, `name! { … }` or
    `macro_rules! name { … }`, where an item is text the gate cannot
    read) or `nested` (in an `impl`, a fn or a struct: a method or a
    field, no door)."""
    pubs = {m.start() for m in re.finditer(r"\bpub\b", text)}
    out, stack = [], []
    for i, c in enumerate(text):
        if i in pubs:
            out.append((i, "macro" if any(stack) else "nested" if stack else "top"))
        if c in "{([":
            stack.append(bool(re.search(r"!\s*(?:\w+\s*)?$", text[max(0, i - 64):i])))
        elif c in "})]" and stack:
            stack.pop()
    return out


def doors():
    """The charge doors, read from geom-core itself.

    DOORS maps every `pub` item at the top of `geom_core::running` to its
    kind: `fn`, `value` (a const or static) or `type` (a struct, enum,
    union, type alias or trait). Every `pub` token is read
    (`pub_tokens`), wherever on its line it sits, so an attributed
    `#[inline] pub fn` is a door. A `pub(…)`-restricted item cannot be
    reached outside geom-core, so it is no door. Any other `pub` item
    (`pub mod`, `pub use`), and any `pub` inside a macro's body, is
    UNRECOGNISED and reds: the gate never skips a door it cannot
    classify.

    ROOT maps each name `geom_core`'s `lib.rs` re-exports from `running`
    (renamed with `as`, through `crate::` or `self::`, or by a glob) to
    its door, and RMODS holds the names it re-exports `running` itself
    under. A re-export of something under `running` that is no door is
    UNRECOGNISED."""
    for need in (RUNNING, LIB):
        if not os.path.isfile(need):
            sys.exit(f"REFUSED {need} is missing: the charge doors cannot be read")
    kinds, odd = {}, []
    text = read_stripped(RUNNING)
    for at, where in pub_tokens(text):
        m = re.match(r"pub\b(\s*\([^)]*\))?\s*([^\n]*)", text[at:])
        if where == "nested":
            continue
        if where == "macro":
            odd.append(f"UNRECOGNISED {RUNNING}: `pub {m.group(2).strip()}` sits in a macro's body, which this gate cannot read for a door")
            continue
        if m.group(1):
            continue
        for kind, shape in ITEM:
            got = shape.match(m.group(2))
            if got:
                kinds[got.group(1)] = kind
                break
        else:
            odd.append(f"UNRECOGNISED {RUNNING}: `pub {m.group(2).strip()}` is no fn, const, static or type")
    if not kinds:
        sys.exit(f"REFUSED {RUNNING} declares no `pub` item: the charge doors cannot be read")
    root, rmods = {}, set()
    lib = read_stripped(LIB)
    reexports = [re.match(r"pub\s+use\s+([^;]*);", lib[at:]) for at, where in pub_tokens(lib) if where == "top"]
    for m in filter(None, reexports):
        for segs, alias in use_items(m.group(1)):
            segs = lead(segs)
            if not segs or segs[0] != "running":
                continue
            rest = segs[1:]
            if rest in ([], ["self"]):
                rmods.add(alias or "running")
            elif rest == ["*"]:
                root.update({n: n for n in kinds})
            elif len(rest) == 1 and rest[0] in kinds:
                root[alias or rest[0]] = rest[0]
            else:
                odd.append(f"UNRECOGNISED {LIB}: `pub use {'::'.join(segs)}` re-exports no door of `running`")
    return kinds, root, rmods, odd


DOORS, ROOT, RMODS, ODD = doors()


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
    """What a file's imports and aliases bind, each with the span it is in
    scope over (its block, or the whole file at the top level):
    `names`, a local name of a door; `modules`, a local name of the
    `running` module (or of a root re-export of it); `crates`, a local
    name of `geom_core` itself, by default `geom_core` over the file, and
    any `as` name of a path ending in `geom_core`, direct or through
    another crate (`use pncad::geom_core as gc`). A `type NAME = PATH;`
    naming a door type binds NAME like an import. A path resolves
    through an alias only inside that alias's own scope."""
    n = len(text)
    spans = blocks(text)
    names, modules, crates = [], [], [("geom_core", (0, n))]

    def in_scope(pairs, at):
        return {name for name, (lo, hi) in pairs if lo <= at < hi}

    for m in re.finditer(r"\bextern\s+crate\s+geom_core\s+as\s+(\w+)", text):
        crates.append((m.group(1), scope_of(spans, m.start(), n)))
    for m in USE.finditer(text):
        for segs, alias in use_items(m.group(1)):
            if alias and lead(segs) and lead(segs)[-1] == "geom_core":
                crates.append((alias, scope_of(spans, m.start(), n)))
    for m in USE.finditer(text):
        scope = scope_of(spans, m.start(), n)
        crate_names = in_scope(crates, m.start())
        module_names = in_scope(modules, m.start())
        for segs, alias in use_items(m.group(1)):
            segs = lead(segs)
            if segs and segs[-1] == "self":
                segs = segs[:-1]
            if len(segs) < 2:
                continue
            head, last = segs[-2], segs[-1]
            if head in crate_names and last in RMODS | {"running"}:
                modules.append((alias or last, scope))
            elif last == "*" and head in module_names | ({"running"} if len(segs) >= 3 and segs[-3] in crate_names else set()):
                names += [(d, d, scope) for d in DOORS]
            elif last == "*" and head in crate_names and len(segs) == 2:
                names += [(r, d, scope) for r, d in ROOT.items()]
            elif last in DOORS and (head in module_names or (head in RMODS | {"running"} and len(segs) >= 3 and segs[-3] in crate_names)):
                names.append((alias or last, last, scope))
            elif head in crate_names and len(segs) == 2 and last in ROOT:
                names.append((alias or last, ROOT[last], scope))
    for m in re.finditer(r"\btype\s+(\w+)\s*(?:<[^=]*>)?\s*=\s*((?:\w+\s*::\s*)*\w+)", text):
        segs = lead(re.findall(r"\w+", m.group(2)))
        bound = {local: door for local, door, (lo, hi) in names if lo <= m.start() < hi}
        crate_names = in_scope(crates, m.start())
        mods_here = in_scope(modules, m.start())
        door = None
        if len(segs) >= 2 and segs[-1] in DOORS and (segs[-2] in mods_here or (segs[-2] in RMODS | {"running"} and len(segs) >= 3 and segs[-3] in crate_names)):
            door = segs[-1]
        elif len(segs) == 2 and segs[0] in crate_names and segs[1] in ROOT:
            door = ROOT[segs[1]]
        elif len(segs) == 1 and segs[0] in bound:
            door = bound[segs[0]]
        if door and DOORS[door] == "type":
            names.append((m.group(1), door, scope_of(spans, m.start(), n)))
    return names, modules, crates


def declared_elsewhere():
    """Door names (and root re-export names) some other file of the tree
    declares too: a bare one of those is read only where an import binds
    it."""
    alt = "|".join(map(re.escape, set(DOORS) | set(ROOT)))
    decl = re.compile(r"\b(?:fn|const|static|struct|enum|union|type|trait|mod)\s+(" + alt + r")\b")
    found = set()
    for path in glob.glob("crates/*/**/*.rs", recursive=True) + glob.glob("interval-transcendentals/**/*.rs", recursive=True):
        if path == RUNNING or "/target/" in path:
            continue
        found |= set(decl.findall(read_stripped(path)))
    return found


SHARED = declared_elsewhere()
SHAPE = {"fn": r"\s*(?:\(|::\s*<)", "value": r"\b", "type": r"\s*(?:::|\{)"}
GENERICS = r"(?:\s*<(?:[^<>]|<(?:[^<>]|<[^<>]*>)*>)*>)?"


def qualified(code, path, lo, hi, prefix=r"(?:\w+\s*::\s*)*"):
    """Offsets of `<PATH<…> as …>::` or `<PATH<…>>::` in `code[lo:hi]`, a
    door type read through a qualified path: the `<` must open the path,
    not open a generic list (`Vec<PATH>`, `W::<PATH>::new` name a type)."""
    out = set()
    for m in re.compile(r"<\s*" + prefix + path + GENERICS + r"\s*(?:>\s*::|as\b)").finditer(code, lo, hi):
        before = code[:m.start()].rstrip()
        if not before or not (before[-1].isalnum() or before[-1] in "_>:"):
            out.add(m.start())
    return out


def sites(text):
    """The offsets of one file's sites: a bare `f64::EPSILON` or a
    `carrier_resolution` call, and every use of a door — by a path through
    `running` (any shape for a fn or a value, a construction or a
    qualified path for a type), through the crate root for a re-exported
    one, or bare where an import or a type alias in scope binds the name
    or no other item of the tree shares it. An import is not a site."""
    names, modules, crates = bindings(text)
    code = USE.sub(lambda m: " " * len(m.group(0)), text)
    n = len(code)
    at = {m.start() for m in FIXED.finditer(code)}
    # A path is a site at its door's name, so a path read two ways (a
    # crate alias's `gc::running::dot` and a module's `running::dot`)
    # is one site.
    via = [(r"\b" + re.escape(c) + r"\s*::\s*(?:" + "|".join(map(re.escape, sorted(RMODS | {"running"}))) + r")\s*::\s*", scope, DOORS)
           for c, scope in crates]
    via += [(r"\b" + re.escape(mname) + r"\s*::\s*", scope, DOORS) for mname, scope in modules]
    via += [(r"\b" + re.escape(c) + r"\s*::\s*", scope, ROOT) for c, scope in crates]
    for prefix, (lo, hi), table in via:
        for name in table:
            door = table[name] if table is ROOT else name
            kind = DOORS[door]
            pat = re.compile("(" + prefix + ")" + re.escape(name) + (SHAPE[kind] if kind == "type" else r"\b"))
            at |= {m.end(1) for m in pat.finditer(code, lo, hi)}
            if kind == "type":
                at |= qualified(code, prefix + re.escape(name), lo, hi, prefix="")
    names += [(d, d, (0, n)) for d in DOORS if d not in SHARED]
    names += [(r, d, (0, n)) for r, d in ROOT.items() if r not in SHARED and r not in DOORS]
    for name, door, (lo, hi) in names:
        local = r"(?<![\w.:])" + re.escape(name)
        at |= {m.start() for m in re.compile(local + SHAPE[DOORS[door]]).finditer(code, lo, hi)}
        if DOORS[door] == "type":
            at |= qualified(code, local, lo, hi, prefix="")
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
for b in ODD + bad:
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
  if grep -q '^UNRECOGNISED ' <<<"$report"; then
    grep '^UNRECOGNISED ' <<<"$report"
    gate_error "geom_core::running declares or re-exports an item this gate cannot classify as a door (above): a door it cannot classify would go uncounted, so teach \`doors\` its kind"
    exit 1
  fi
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
    printf 'pub struct Rounded { pub value: f64 }\nfn private_helper() {}\npub(crate) mod hidden {}\n'
  } > "$1/crates/geom-core/src/running.rs"
  printf 'pub mod running;\npub use running::{Rounded, UNIT_ROUNDOFF, rounding_charge, dot, exact_vec as rexact};\n' > "$1/crates/geom-core/src/lib.rs"
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
  add_clean "$1" 'pub fn a(x: f64, v: V) -> f64 { use geom_core::running::{dot, exact_vec}; v.dot(x); dot(exact_vec(x), x) }'
  add_clean "$1" 'pub fn b(x: f64, v: V) -> f64 { v.dot(x); dot(x, x) }'
}
# An import is not a site, nor a door type named in a signature or a
# generic list, nor a restricted item of `running`.
plant_import_only() {
  add_clean "$1" 'use geom_core::{UNIT_ROUNDOFF, running::Rounded};'
  add_clean "$1" 'pub fn f(r: Rounded, v: Vec<geom_core::running::Rounded>) -> Vec<Rounded> { let _ = <Vec<geom_core::running::Rounded>>::new(); W::<geom_core::running::Rounded>::new() }'
}
# Each binding arm, planted alone.
plant_self_as() { add_clean "$1" 'use geom_core::running::{self as r};'; add_clean "$1" 'pub fn pad(x: f64) -> f64 { r::dot(x, x) }'; }
plant_root_glob() { add_clean "$1" 'mod g { use geom_core::*; pub fn pad(x: f64) -> f64 { dot(x, x) } }'; }
plant_running_alias() { add_clean "$1" 'mod g { use geom_core::running::dot as rd; pub fn pad(x: f64) -> f64 { rd(x, x) } }'; }
plant_crate_alias() { add_clean "$1" 'use geom_core as gc;'; add_clean "$1" 'pub fn pad(x: f64) -> f64 { gc::rounding_charge(x) }'; }
plant_type_alias() { add_clean "$1" 'type R = geom_core::Rounded;'; add_clean "$1" 'pub fn pad() -> f64 { R::exact(0.0).value }'; }
plant_qualified() { add_clean "$1" 'pub fn pad() -> f64 { <geom_core::running::Rounded>::exact(0.0).value }'; }
# Doors of each kind `running` may declare, and the root's spellings.
plant_const_fn_door() {
  listed_at "$1" 1
  printf 'pub const fn fresh(t: f64) -> f64 { t }\n' >> "$1/crates/geom-core/src/running.rs"
  add_clean "$1" 'pub fn pad(x: f64) -> f64 { geom_core::running::fresh(x) }'
}
plant_type_door() {
  printf 'pub type Charge = Rounded;\n' >> "$1/crates/geom-core/src/running.rs"
  add_clean "$1" 'pub fn pad() -> f64 { geom_core::running::Charge::exact(0.0).value }'
}
plant_renamed_root() { add_clean "$1" 'pub fn pad(x: f64) -> f64 { geom_core::rexact(x) }'; }
plant_crate_path_root() {
  printf 'pub use crate::running::DIRECTION_UNITS as UNITS;\n' >> "$1/crates/geom-core/src/lib.rs"
  add_clean "$1" 'pub fn pad(x: f64) -> f64 { x * geom_core::UNITS }'
}
# An attributed one-line door is a door; one a macro makes reds.
plant_attributed_door() {
  printf '#[inline] pub fn fresh_inline(t: f64) -> f64 { t }\n' >> "$1/crates/geom-core/src/running.rs"
  add_clean "$1" 'pub fn pad(x: f64) -> f64 { geom_core::running::fresh_inline(x) }'
}
plant_macro_rules_door() {
  printf 'macro_rules! doors { () => { pub fn made(t: f64) -> f64 { t } }; }\n' >> "$1/crates/geom-core/src/running.rs"
}
plant_macro_call_door() {
  printf 'doors! {\n    pub fn made(t: f64) -> f64 { t }\n}\n' >> "$1/crates/geom-core/src/running.rs"
}
# An alias resolves inside its own scope only, and `running` unimported is
# not geom-core's: neither call here is a site.
plant_alias_out_of_scope() {
  add_clean "$1" 'mod a { use geom_core as gc; }'
  add_clean "$1" 'mod n { use other as gc; pub fn pad(x: f64) -> f64 { gc::rounding_charge(x) } }'
  add_clean "$1" 'mod b { use geom_core::running as r; }'
  add_clean "$1" 'mod m { use other::stuff as r; pub fn pad(x: f64) -> f64 { r::exact_vec(x) } }'
}
plant_running_unbound() { add_clean "$1" 'pub fn pad(x: f64) -> f64 { crate::running::exact_vec(x) }'; }
# A crate alias through another crate's re-export of geom_core.
plant_indirect_crate_alias() { add_clean "$1" 'use pncad::geom_core as gc;'; add_clean "$1" 'pub fn pad(x: f64) -> f64 { gc::rounding_charge(x) }'; }
# A path read two ways is one site: listed at 1, it passes.
plant_path_read_twice() {
  listed_at "$1" 1
  add_clean "$1" 'pub fn pad(x: f64) -> f64 { use geom_core::running; geom_core::running::exact_vec(x) }'
}

# A one-line attributed re-export is read like any other.
plant_attributed_reexport() {
  printf '#[doc(hidden)] pub use running::DIRECTION_UNITS as HIDDEN_UNITS;\n' >> "$1/crates/geom-core/src/lib.rs"
  add_clean "$1" 'pub fn pad(x: f64) -> f64 { x * geom_core::HIDDEN_UNITS }'
}

plant_unrecognised_item() { printf 'pub mod inner {}\n' >> "$1/crates/geom-core/src/running.rs"; }
plant_unrecognised_root() { printf 'pub use running::private_helper;\n' >> "$1/crates/geom-core/src/lib.rs"; }

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
  local p
  for p in plant_self_as plant_root_glob plant_running_alias plant_crate_alias plant_type_alias plant_qualified \
      plant_type_door plant_renamed_root plant_crate_path_root; do
    gate_selftest_case "UNLISTED" "$p"
  done
  gate_selftest_passes "a const fn door at its count" plant_const_fn_door
  gate_selftest_case "UNLISTED" plant_attributed_door
  gate_selftest_case "UNRECOGNISED" plant_macro_rules_door
  gate_selftest_case "UNRECOGNISED" plant_macro_call_door
  gate_selftest_passes "an alias used outside its own scope" plant_alias_out_of_scope
  gate_selftest_passes "another crate's running module" plant_running_unbound
  gate_selftest_case "UNLISTED" plant_indirect_crate_alias
  gate_selftest_case "UNLISTED" plant_attributed_reexport
  gate_selftest_passes "a path read two ways, at its count of one" plant_path_read_twice
  gate_selftest_case "UNRECOGNISED" plant_unrecognised_item
  gate_selftest_case "UNRECOGNISED" plant_unrecognised_root
  gate_selftest_passes "a generic door name under its import, at its count" plant_scoped_import
  gate_selftest_passes "an import and a type in a signature" plant_import_only
  gate_selftest_case "REASON" plant_bad_compound_reason
  gate_selftest_passes "a compound reason" plant_compound_reason
  gate_selftest_case "STALE" plant_stale
  gate_selftest_case "REASON" plant_oracle_in_prod
  gate_selftest_case "UNLISTED crates/clean/src/lib.rs (test)" plant_test_module_as_prod
  gate_selftest_passes "a listed site" plant_listed
  gate_selftest_passes "a comment, a string and geom-core's own" plant_not_sites
  printf '%s selftest OK: passes a clean fixture, a listed site, a compound reason, comments, strings and geom-core itself, a generic door name counted only under its import in scope (a method of that name inside the scope not counted), and an import, a restricted item or a door type named in a signature, a generic list or <Vec<Rounded>>:: or a turbofish alone; fires on an unlisted site of each spelling (bare f64::EPSILON and UNIT_ROUNDOFF, a rounding_charge call, a Rounded path, a carrier_resolution call, direction_rounding and a constant the gate never names, a door declared in the fixture running module as a fn or a type alias), passing a const fn door at its exact count and a path read two ways at a count of one, an alias used outside its scope and the running module of another crate not counted, through each binding (a door alias, a running module alias, {self as r}, a running glob, a root glob, a crate alias, a type alias, a qualified path), through each root spelling (a re-export, renamed, through crate::), on an attributed one-line door and re-export, an indirect crate alias, on an unrecognised running item or root re-export, a pub item made by macro_rules! or a macro call, an oracle inside a compound reason, a stale row, an oracle in production and a test module counted as production\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
