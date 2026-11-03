"""`[topo.homes]`'s engine-door census, read by `engine-leaves.sh`.

Reads `gate_rust_code`'s `path:line:code` records of `crates/topo/src` on
stdin (comments and literals already stripped) and counts, per file:

- `names-<engine>`: in a file outside both engines, every site naming
  that engine — a path through its module (`crate::boolean::…`, any
  alias of `crate` before it, a `use` leaf through it, `use
  crate::boolean as b`), and every occurrence of an item DECLARED under
  it, by name, however the name got into scope (an alias, a hub's
  re-export, `use super::X`, a glob, the root's `pub use`);
- `<engine>-names-<other>`: the same, in one engine's files, for the
  other engine;
- `crate-visible`: in an engine's files, every visibility qualifier that
  reaches the whole crate without being `pub`.

    engine_leaves.py BASELINE CARGO_TOML

Prints `NEW kind file: now (allowed was) at L…`, `STALE kind file:
allowed was, now now`, `AMBIGUOUS-NEW name: …` / `AMBIGUOUS-GONE name`,
`MALFORMED …`, `REFUSED …`, then `TOTAL n`. The gate decides from them.

WHICH NAMES. The items an engine declares at module level (a file's top
level or an inline `mod`'s — never a method, field or local) that are
visible outside it: `pub`, or a qualifier reaching the crate. A name also
declared at module level in a file outside the engines, or in both
engines, cannot be told apart by name: it is AMBIGUOUS, counted only
where a path through the engine spells it or its occurrence resolves (per
inline module, through `use super::*`) to an import through the engine,
and the baseline lists every such name so the exclusion is read rather
than silent.

WHAT IT CANNOT SEE. A method an engine declares on a type outside it
(`impl Body { … }` under `boolean/`) is not a name it declares. A
lowercase name passed point-free (`map(f)`), unqualified, is not read.
An ambiguous name reached through a hub or a glob that is not the
engine's own (`use crate::hub::X`, `use crate::*`) cannot be told from
its namesake and is not counted. A bare name imported by a glob from an
external crate (`use geom_brep::*`) is read as the engine's. Macros are
not expanded. Callers outside `topo` are consumers of the doors and are
not read.

WHAT AN OCCURRENCE IS. A token of the name, not after `.` (a method or
field), not qualified by an external crate or a capitalised segment
(`geom_brep::X`, `Self::x`, `T::x`), and not in a file that imports the
name from an external crate. A lowercase name (a function) counts only
called (`f(`, `f::<`), as a macro (`f!`), path-qualified, or imported,
so a local or field of the same spelling is not read as it. `lib.rs`'s
top-level `pub use` declarations are the doors' re-exports and are not
read.
"""
import os
import re
import sys
import tomllib
from collections import defaultdict

ENGINES = ('boolean', 'splitting')
SRC = 'crates/topo/src/'
KINDS = ('names-boolean', 'names-splitting', 'boolean-names-splitting',
         'splitting-names-boolean', 'crate-visible')
ITEM_KW = {'fn', 'struct', 'enum', 'trait', 'type', 'const', 'static'}
QUALIFIERS = {'const', 'async', 'unsafe', 'extern', 'default'}
TOKEN = re.compile(r'[A-Za-z_][A-Za-z0-9_]*|\d[\w.]*|::|\S')


def externs(cargo_toml):
    """The crate names a `topo` file can name a path from."""
    with open(cargo_toml, 'rb') as fh:
        manifest = tomllib.load(fh)
    names = {'std', 'core', 'alloc', 'proc_macro'}
    for table in ('dependencies', 'dev-dependencies', 'build-dependencies'):
        names |= {n.replace('-', '_') for n in manifest.get(table, {})}
    names.discard('topo')
    return names


def tokenize(lines):
    """`(text, line)` tokens of one file's code records."""
    out = []
    for ln, code in lines:
        out.extend((m.group(), ln) for m in TOKEN.finditer(code))
    return out


def home(rel):
    head = rel.split('/', 1)
    return head[0] if len(head) == 2 and head[0] in ENGINES else None


def module_depth(rel):
    parts = rel.split('/')
    return len(parts) - 1 if rel.endswith('/mod.rs') or rel == 'lib.rs' else len(parts)


def is_ident(t):
    return bool(t) and (t[0].isalpha() or t[0] == '_')


class File:
    """One file's tokens, its brace nesting and its `use` declarations."""

    def __init__(self, rel, toks):
        self.rel, self.toks = rel, toks
        self.home = home(rel)
        n = len(toks)
        # mods[i]: how many inline `mod {}` enclose token i; module[i]:
        # whether every brace enclosing it is a `mod`'s.
        # scope[i]: the open braces of the inline `mod`s enclosing token
        # i, outermost first — the module a name at i is resolved in.
        self.mods, self.module, self.scope = [0] * n, [True] * n, [()] * n
        stack = []
        for i, (t, _) in enumerate(toks):
            self.mods[i] = sum(1 for s in stack if s != 'other')
            self.module[i] = all(s != 'other' for s in stack)
            self.scope[i] = tuple(s for s in stack if s != 'other')
            if t == '{':
                is_mod = i >= 2 and toks[i - 2][0] == 'mod' and is_ident(toks[i - 1][0])
                stack.append(i if is_mod else 'other')
            elif t == '}' and stack:
                stack.pop()
        self.uses = []          # (start, end, visibility tokens)
        self.in_use = [False] * n
        i = 0
        while i < n:
            if toks[i][0] == 'use' and not (i + 1 < n and toks[i + 1][0] == '<') \
                    and not (i >= 1 and toks[i - 1][0] == '#'):
                j = i
                while j < n and toks[j][0] != ';':
                    j += 1
                self.uses.append((i, j, self.visibility(i)))
                for k in range(i, min(j + 1, n)):
                    self.in_use[k] = True
                i = j
            i += 1

    def visibility(self, kw):
        """The visibility tokens before the item keyword at `kw`
        (`['pub']`, `['pub', 'crate']`, `['pub', 'in', 'super', …]`), or
        `[]` for private."""
        j = kw - 1
        while j >= 0 and (self.toks[j][0] in QUALIFIERS or self.toks[j][0] == '"'):
            j -= 1
        if j >= 0 and self.toks[j][0] == ')':
            k = j
            while k >= 0 and self.toks[k][0] != '(':
                k -= 1
            if k >= 1 and self.toks[k - 1][0] == 'pub':
                return ['pub'] + [t for t, _ in self.toks[k + 1:j] if t != '::']
            return []
        return ['pub'] if j >= 0 and self.toks[j][0] == 'pub' else []

    def reaches_outside(self, vis, at):
        """Whether `vis` at token `at` of an engine file is visible
        outside the engine. `crate_wide` is the same less bare `pub`."""
        if vis == ['pub']:
            return True
        return self.crate_wide(vis, at)

    def crate_wide(self, vis, at):
        scope = vis[1:]
        if scope[:1] == ['in']:
            scope = scope[1:]
        if not scope:
            return False
        if scope[0] == 'crate':
            return scope[1:2] != [self.home]
        if scope[0] == 'self':
            return False
        if scope[0] == 'super':
            ups = len([s for s in scope if s == 'super'])
            return ups >= module_depth(self.rel) + self.mods[at]
        raise SystemExit(f'REFUSED {SRC}{self.rel}: a visibility this census cannot read: pub({" ".join(scope)})')

    def items(self):
        """`(name, visibility, token index)` of every module-level item."""
        toks, out = self.toks, []
        for i, (t, _) in enumerate(toks):
            if not self.module[i] or self.in_use[i]:
                continue
            nxt = toks[i + 1][0] if i + 1 < len(toks) else ''
            if t == 'macro_rules' and nxt == '!' and i + 2 < len(toks):
                out.append((toks[i + 2][0], ['pub'], i))
            elif t == 'union' and is_ident(nxt) and i + 2 < len(toks) and toks[i + 2][0] in '{<':
                out.append((nxt, self.visibility(i), i))
            elif t in ITEM_KW:
                if t == 'const' and nxt in ('fn', 'unsafe', 'async', 'extern'):
                    continue
                if t == 'static' and nxt == 'mut':
                    nxt = toks[i + 2][0] if i + 2 < len(toks) else ''
                if t in ('struct', 'enum', 'trait', 'type', 'fn', 'const', 'static') \
                        and is_ident(nxt) and nxt != '_' and not (i >= 1 and toks[i - 1][0] in ('impl', 'dyn', '&', '<', ',', '(')):
                    out.append((nxt, self.visibility(i), i))
        return out

    def prefix(self, i):
        """The `::`-joined segments before token `i`, innermost last;
        `None` when a leading `::` makes the path an external crate's."""
        toks, segs, j = self.toks, [], i
        while j >= 2 and toks[j - 1][0] == '::' and is_ident(toks[j - 2][0]):
            segs.insert(0, toks[j - 2][0])
            j -= 2
        if j >= 1 and toks[j - 1][0] == '::':
            return None
        return segs

    def use_leaves(self):
        """`(segments, bound name, token index, exempt)` of every leaf of
        every `use` declaration; a glob's bound name is `*`."""
        out = []
        for start, end, vis in self.uses:
            exempt = self.rel == 'lib.rs' and vis == ['pub'] and self.mods[start] == 0
            out.extend((s, b, k, exempt) for s, b, k in self.tree(start + 1, end, []))
        return out

    def tree(self, i, end, head):
        toks, segs, leaves = self.toks, list(head), []
        if i < end and toks[i][0] == '::':
            segs.append('::')
            i += 1
        while i < end:
            t = toks[i][0]
            if t == '{':
                depth, j, start = 0, i, i + 1
                while j < end:
                    if toks[j][0] == '{':
                        depth += 1
                    elif toks[j][0] == '}':
                        depth -= 1
                        if depth == 0:
                            break
                    elif toks[j][0] == ',' and depth == 1:
                        leaves += self.tree(start, j, segs)
                        start = j + 1
                    j += 1
                leaves += self.tree(start, j, segs)
                return leaves
            if t == '*':
                return [(segs, '*', i)]
            if is_ident(t):
                segs = [*segs, t]
                if i + 1 < end and toks[i + 1][0] == 'as' and i + 2 < end:
                    return [(segs, toks[i + 2][0], i)]
                if i + 1 >= end or toks[i + 1][0] != '::':
                    return [(segs, t, i)]
            i += 1
        return leaves


def census(files, ext):
    """Counts by (kind, file), each with its lines, and the ambiguous names."""
    declared = {e: {} for e in ENGINES}
    outside = defaultdict(set)
    for f in files.values():
        for name, vis, at in f.items():
            if f.home is None:
                outside[name].add(f.rel)
            elif f.reaches_outside(vis, at):
                declared[f.home].setdefault(name, f.rel)
    for e in ENGINES:
        if not declared[e]:
            raise SystemExit(f'REFUSED no item under {SRC}{e}/ reaches outside it: the census read nothing')
    both = set(declared['boolean']) & set(declared['splitting'])
    ambiguous = {n: sorted(outside[n]) or ['both engines'] for e in ENGINES for n in declared[e]
                 if n in outside or n in both}
    names = {e: set(declared[e]) - set(ambiguous) for e in ENGINES}
    shared = {e: set(declared[e]) & set(ambiguous) for e in ENGINES}
    sites = defaultdict(list)
    for f in files.values():
        for i, (t, _) in enumerate(f.toks):
            if t == 'mod' and i + 1 < len(f.toks) and f.toks[i + 1][0] in ENGINES and f.rel != 'lib.rs':
                raise SystemExit(f'REFUSED {SRC}{f.rel} declares a module named {f.toks[i + 1][0]}: '
                                 'this census reads that name as the engine\'s')
        for e in ENGINES:
            if f.home == e:
                continue
            kind = f'{f.home}-names-{e}' if f.home else f'names-{e}'
            sites[(kind, f.rel)] += names_engine(f, e, names[e], shared[e], ext)
        if f.home:
            n = []
            for i, (t, ln) in enumerate(f.toks):
                if t == 'pub' and i + 1 < len(f.toks) and f.toks[i + 1][0] == '(':
                    j = i + 1
                    while f.toks[j][0] != ')':
                        j += 1
                    vis = ['pub'] + [x for x, _ in f.toks[i + 2:j] if x != '::']
                    if f.crate_wide(vis, i):
                        n.append(ln)
            sites[('crate-visible', f.rel)] += n
    return {k: v for k, v in sites.items() if v}, ambiguous


def names_engine(f, e, names, shared, ext):
    """The lines of every site in `f` naming engine `e`.

    A name's binding is resolved the way rustc resolves a single
    segment, per inline module: the module's own `use` leaves and
    item declarations, then its parent's only through `use super::*`.
    A `shared` (ambiguous) name counts where it resolves to an import
    through the engine, or follows a segment that does; any other engine
    name counts unless it resolves to an external crate's import."""
    toks, out = f.toks, []
    through = set()                         # names bound through `e`
    binds = defaultdict(dict)               # name -> scope -> 'e' | 'ext' | 'other'
    globs_e, globs_super = set(), set()
    for segs, bound, k, exempt in f.use_leaves():
        sc = f.scope[k]
        if segs and (segs[0] == '::' or segs[0] in ext):
            kind = 'ext'
        elif e in segs or (segs and segs[0] in through):
            kind = 'e'
        else:
            kind = 'other'
        if bound == '*':
            if kind == 'e':
                globs_e.add(sc)
            if segs == ['super']:
                globs_super.add(sc)
        else:
            binds[bound].setdefault(sc, kind)
            if kind == 'e':
                through.add(bound)
        if exempt:
            continue
        if kind == 'e' or (kind == 'other' and segs and segs[-1] in names):
            out.append(toks[k][1])
    for name, _, at in f.items():
        binds[name].setdefault(f.scope[at], 'other')

    def resolve(name, at):
        sc = f.scope[at]
        while True:
            if sc in binds.get(name, {}):
                return binds[name][sc]
            if sc in globs_e:
                return 'e'
            if sc and sc in globs_super:
                sc = sc[:-1]
                continue
            return None

    for i, (t, ln) in enumerate(toks):
        if f.in_use[i] or (i >= 1 and toks[i - 1][0] == '.'):
            continue
        if t != e and t not in names and t not in shared:
            continue
        pre = f.prefix(i)
        if pre is None or (pre and (pre[0] in ext or pre[0][0].isupper())):
            continue
        nxt = toks[i + 1][0] if i + 1 < len(toks) else ''
        if t == e:
            before = toks[i - len(pre) * 2 - 1][0] if i - len(pre) * 2 >= 1 else ''
            if before in ('mod', 'in') or not (pre or nxt == '::'):
                continue
            out.append(ln)
            continue
        if e in pre:
            continue
        head = resolve(pre[0], i - 2 * len(pre)) if pre else resolve(t, i)
        if t in shared and head != 'e':
            continue
        if not pre and head == 'ext':
            continue
        if t[0].islower() or t[0] == '_':
            after = toks[i + 2][0] if i + 2 < len(toks) else ''
            if not (pre or nxt in ('(', '!') or (nxt == '::' and after == '<')):
                continue
        out.append(ln)
    return out


def baseline(path):
    allowed, ambiguous, bad = {}, set(), []
    with open(path, encoding='utf-8') as fh:
        for n, line in enumerate(fh, 1):
            cols = line.split('#', 1)[0].split()
            if not cols:
                continue
            if cols[0] == 'ambiguous' and len(cols) == 2:
                ambiguous.add(cols[1])
                continue
            ok = len(cols) == 3 and cols[0] in KINDS and cols[1].isdigit() and int(cols[1]) > 0
            if not ok or (cols[0], cols[2]) in allowed:
                bad.append(f'MALFORMED {path}:{n}: {line.strip()!r} — a line is `KIND COUNT FILE` '
                           f'(count ≥ 1, one line per kind and file) or `ambiguous NAME`; '
                           f'kinds: {" ".join(KINDS)}')
                continue
            allowed[(cols[0], cols[2])] = int(cols[1])
    return allowed, ambiguous, bad


def main():
    base_path, cargo = sys.argv[1], sys.argv[2]
    ext = externs(cargo)
    recs = defaultdict(list)
    for rec in sys.stdin:
        path, ln, code = rec.rstrip('\n').split(':', 2)
        if path.startswith(SRC):
            recs[path[len(SRC):]].append((int(ln), code))
    if 'lib.rs' not in recs or not any(home(r) for r in recs):
        raise SystemExit(f'REFUSED no {SRC}lib.rs or no engine file among the records: the census read nothing')
    files = {r: File(r, tokenize(lines)) for r, lines in recs.items()}
    sites, ambiguous = census(files, ext)
    allowed, listed, bad = baseline(base_path)
    for line in bad:
        print(line)
    for key in sorted(set(sites) | set(allowed)):
        now, was = len(sites.get(key, [])), allowed.get(key, 0)
        if now > was:
            at = ' '.join(map(str, sorted(set(sites[key]))))
            print(f'NEW {key[0]} {key[1]}: {now} (allowed {was}) at L{at}')
        elif now < was:
            print(f'STALE {key[0]} {key[1]}: allowed {was}, now {now}')
    for name in sorted(set(ambiguous) - listed):
        print(f'AMBIGUOUS-NEW {name}: declared under an engine and in {", ".join(ambiguous[name])}')
    for name in sorted(listed - set(ambiguous)):
        print(f'AMBIGUOUS-GONE {name}')
    if os.environ.get('ENGINE_LEAVES_TABLE'):
        for (kind, rel), lines in sorted(sites.items()):
            print(f'TABLE {kind:<24} {len(lines):>4}  {rel}')
        for name in sorted(ambiguous):
            print(f'TABLE ambiguous {name}  # {", ".join(ambiguous[name])}')
    print(f'TOTAL {sum(len(v) for v in sites.values())}')


main()
