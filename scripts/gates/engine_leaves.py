"""`[topo.homes]`'s engine-door census, read by `engine-leaves.sh`.

Reads `gate_rust_code`'s `path:line:code` records of `crates/topo/src` on
stdin (comments and literals already stripped) and counts, per file and
KIND (the block below): every site naming an engine from outside it —
a path through its module, and every occurrence of an item DECLARED
under it, BY NAME, however the name reached scope (a hub's re-export,
`use super::X`, a glob, the root's own `pub use`) — the crate-wide
qualifiers declared inside an engine, and the constructs it refuses.

    engine_leaves.py BASELINE CARGO_TOML

Prints `ROSE kind file: was -> now (the file's counted sites: L…)`,
`FELL kind file: was -> now`, `AMBIGUOUS-NEW name: …`, `AMBIGUOUS-GONE
name`, `MALFORMED …`, then `TOTAL n`; `REFUSED …` on stderr when it
cannot read its subject. The gate decides from them.

BY NAME IS SOUND ONLY IF EVERY ITEM IS SPELLED BY ITS OWN NAME, so a
second name is REFUSED (`refused_bindings`, kind `refused-binding`):
`use crate as C`, `use topo as C`, `use crate::{self as C}`, `use
super::{self as C}` reaching the root, `extern crate self as …`, a
leading `::topo::` path, an `as` rename of an engine item or module in a
`use` (lib.rs's top-level re-exports included), a lib.rs re-export of an
engine MODULE (`crate::boxes::X` would hide an ambiguous X), `#[macro_use]`
on an engine's `mod` or on any `mod` inside one (it exports macros past
the visibility check), and — at any depth, fn bodies included — a `type`
alias or a `macro_rules!` body outside the engine spelling an engine
item. An associated `type` in an `impl` or `trait` is not an alias; its
right side counts as a site.
A macro is refused rather than counted because its expansion sites are
invisible: one count could not stand for its uses.

WHICH NAMES. The items an engine declares at module level (a file's top
level or an inline `mod`'s — never a method, field or local) that are
visible outside it: `pub`, a qualifier reaching the crate, or
`#[macro_export]` for a macro. A name also declared at module level
outside the engines, or in both, is AMBIGUOUS: counted only where a path
or an import through the engine spells it, where it resolves (per inline
module, through `use super::*`) to such an import, or as `crate::NAME`
for a NAME lib.rs re-exports from the engine — `crate::Name::…` too, at
any length, for a type. `$crate`, `topo` (this crate's own name) and a
`super` chain that climbs to the root read as `crate`. The baseline lists every
such name, so the exclusion is read rather than silent.

WHAT AN OCCURRENCE IS. A token of the name, not after `.` (a method or
field), not qualified by an external crate, and not resolving to an
external crate's import. Under a capitalised head (`H::name`) it counts
unless H is `Self`, a type parameter, a std type, a type declared
outside the engines or imported from elsewhere, or an engine name
(counted itself). A lowercase name counts only called (`f(`, `f::<`),
as a macro (`f!`), path-qualified, or imported, and never as `f::…`
(a module of that name).

ENGINES is fixed at the two operation directories `[topo.homes]` names;
a third is a change to this file.

COUNTS ARE PER FILE TOTALS, so they move for reasons that are not a
move: a spelling deleted as redundant, `Self` or inference in place of
a name lowers a count, and the slack that leaves lets a new reference
ride in until the line is lowered. A FELL forces the lowering in the
same change, which is what keeps the slack from standing.

WHAT IT CANNOT SEE. A method an engine declares on a type outside it
(`impl Body` under `boolean/`) is not a name it declares. A lowercase
name passed point-free and unqualified (`map(f)`). An ambiguous name
through a non-engine hub or glob (`use crate::hub::X`, `use crate::*`).
An associated `type` in an `impl` (only its own right side is a site).
OVER-COUNTS, the safe direction: a bare name imported by a glob from an
external crate, an unknown capitalised head, a local closure called like
an engine function, a root fn or method sharing a lowercase engine name
called bare, and a root enum's variants under `use K::*` sharing an
engine type's name. Not expanded: macros. Not read: callers outside
`topo`, which are consumers of the doors.
"""
import os
import re
import sys
import tomllib
from collections import defaultdict

ENGINES = ('boolean', 'splitting')
SRC = 'crates/topo/src/'
# THE KINDS, stated once (the counts file's header points here):
#   names-boolean, names-splitting  a file outside both engines naming one
#   boolean-names-splitting         a boolean/ file naming splitting
#   splitting-names-boolean         a splitting/ file naming boolean
#   crate-visible                   a crate-wide qualifier that is not `pub`,
#                                   declared inside an engine
#   refused-binding                 a second name bound to an engine item or
#                                   module, or to this crate (refused_bindings)
KINDS = ('names-boolean', 'names-splitting', 'boolean-names-splitting',
         'splitting-names-boolean', 'crate-visible', 'refused-binding')
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
        self.in_impl, self.block_kind = [False] * n, []
        stack = []
        for i, (t, _) in enumerate(toks):
            self.mods[i] = sum(1 for s in stack if s != 'other')
            self.module[i] = all(s != 'other' for s in stack)
            self.scope[i] = tuple(s for s in stack if s != 'other')
            self.in_impl[i] = bool(stack) and stack[-1] == 'other' and self.block_kind[-1:] == ['impl']
            if t == '{':
                is_mod = i >= 2 and toks[i - 2][0] == 'mod' and is_ident(toks[i - 1][0])
                stack.append(i if is_mod else 'other')
                self.block_kind.append('impl' if self.opens_impl(i) else 'x')
            elif t == '}' and stack:
                stack.pop()
                self.block_kind.pop()
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

    def opens_impl(self, brace):
        """Whether the `{` at `brace` opens an `impl` or `trait` body."""
        j = brace - 1
        while j >= 0 and self.toks[j][0] not in (';', '{', '}'):
            j -= 1
        head = [t for t, _ in self.toks[j + 1:brace]]
        while head[:1] == ['#']:                      # attributes before the item
            k = head.index(']') if ']' in head else len(head)
            head = head[k + 1:]
        while head and head[0] in ('pub', 'unsafe', 'default', 'auto', '(', ')', 'crate', 'in', 'super', 'self', '::'):
            head = head[1:]
        return head[:1] in (['impl'], ['trait'])

    def depth_at(self, at):
        """The module depth below the crate root at token `at`."""
        return module_depth(self.rel) + self.mods[at]

    def rooted(self, segs, at):
        """`segs` with a leading alias of the crate root spelled `crate`:
        `$crate`, `topo` (this crate's own name), and a `super` chain
        that climbs to the root."""
        if segs and segs[0] in ('$crate', 'topo'):
            return ['crate', *segs[1:]]
        ups = 0
        while ups < len(segs) and segs[ups] == 'super':
            ups += 1
        if ups and ups >= self.depth_at(at):
            return ['crate', *segs[ups:]]
        return segs

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
                exported = i >= 4 and [x for x, _ in toks[i - 4:i]] == ['#', '[', 'macro_export', ']']
                out.append((toks[i + 2][0], ['pub'] if exported else [], i))
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

    def type_params(self):
        """Every name declared as a type parameter anywhere in the file
        (`<T: Real, const N: usize>` after an item keyword or `impl`)."""
        toks, out = self.toks, {'Self'}
        for i, (t, _) in enumerate(toks):
            j = i + 1 if t == 'impl' else i + 2
            if t not in ('fn', 'struct', 'enum', 'trait', 'type', 'union', 'impl') or j >= len(toks) \
                    or toks[j][0] != '<':
                continue
            depth, k = 0, j
            while k < len(toks):
                x = toks[k][0]
                depth += x == '<'
                depth -= x == '>'
                if depth == 0:
                    break
                if depth == 1 and is_ident(x) and toks[k - 1][0] in ('<', ',', 'const') and x != 'const':
                    out.add(x)
                k += 1
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


STD_HEADS = {'Self', 'Vec', 'Option', 'Result', 'Box', 'String', 'Some', 'None', 'Ok', 'Err',
             'Default', 'Iterator', 'IntoIterator', 'From', 'Into', 'TryFrom', 'TryInto', 'Clone',
             'Copy', 'PartialEq', 'PartialOrd', 'Ord', 'Eq', 'Hash', 'Debug', 'ToString',
             'AsRef', 'Fn', 'FnMut', 'FnOnce', 'Send', 'Sync', 'Sized', 'Drop'}


class Engines:
    """What every file is read against: the names each engine declares,
    which of them are ambiguous, and what the root re-exports."""

    def __init__(self, files, ext):
        self.ext = ext
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
        self.ambiguous = {n: sorted(outside[n]) or ['both engines'] for e in ENGINES for n in declared[e]
                          if n in outside or n in both}
        self.declared = {e: set(declared[e]) for e in ENGINES}
        self.names = {e: self.declared[e] - set(self.ambiguous) for e in ENGINES}
        self.outside = set(outside)
        # The modules each engine declares, by name: a lib.rs re-export of
        # one would let `crate::MODULE::X` hide an ambiguous X.
        self.modules = {e: {e} for e in ENGINES}
        for f in files.values():
            if f.home:
                for i, (t, _) in enumerate(f.toks):
                    if t == 'mod' and i + 1 < len(f.toks) and is_ident(f.toks[i + 1][0]):
                        self.modules[f.home].add(f.toks[i + 1][0])
        # The root's re-export table: a name lib.rs's top-level `pub use`
        # binds, and the engine it binds it through.
        self.root = {}
        lib = files['lib.rs']
        for segs, bound, _, exempt in lib.use_leaves():
            hit = next((e for e in ENGINES if e in segs), None)
            if exempt and hit and bound not in ('*', '_'):
                self.root[bound] = hit

    def engine_of(self, segs, through, f=None, at=0):
        """The engine a `use` leaf's or a path's segments reach, if any:
        through its module, through a name already bound through it, or
        as `crate::NAME…` (any root alias, any length) where the root
        re-exports NAME from it."""
        if segs and (segs[0] == '::' or segs[0] in self.ext):
            return None
        if f is not None:
            segs = f.rooted(segs, at)
        for e in ENGINES:
            if e in segs or (segs and segs[0] in through[e]):
                return e
        # `crate::NAME` for any NAME, `crate::Name::…` only for a type: a
        # lowercase NAME with more segments after it is a module.
        if len(segs) >= 2 and segs[0] == 'crate' and segs[1] in self.root \
                and (len(segs) == 2 or segs[1][0].isupper()):
            return self.root[segs[1]]
        if segs and segs[-1] in self.names['boolean']:
            return 'boolean'
        if segs and segs[-1] in self.names['splitting']:
            return 'splitting'
        return None


def census(files, ext):
    """Counts by (kind, file), each with its lines; the ambiguous names."""
    eng = Engines(files, ext)
    sites = defaultdict(list)
    for f in files.values():
        for i, (t, _) in enumerate(f.toks):
            if t == 'mod' and i + 1 < len(f.toks) and f.toks[i + 1][0] in ENGINES and f.rel != 'lib.rs':
                raise SystemExit(f'REFUSED {SRC}{f.rel} declares a module named {f.toks[i + 1][0]}: '
                                 'this census reads that name as the engine\'s')
        sites[('refused-binding', f.rel)] += refused_bindings(f, eng)
        for e in ENGINES:
            if f.home == e:
                continue
            kind = f'{f.home}-names-{e}' if f.home else f'names-{e}'
            sites[(kind, f.rel)] += names_engine(f, e, eng)
        if f.home:
            n = []
            for i, (t, ln) in enumerate(f.toks):
                if t == 'pub' and i + 1 < len(f.toks) and f.toks[i + 1][0] == '(':
                    j = i + 1
                    while j < len(f.toks) and f.toks[j][0] != ')':
                        j += 1
                    vis = ['pub'] + [x for x, _ in f.toks[i + 2:j] if x != '::']
                    if f.crate_wide(vis, i):
                        n.append(ln)
            sites[('crate-visible', f.rel)] += n
    return {k: v for k, v in sites.items() if v}, eng.ambiguous


def refused_bindings(f, eng):
    """The lines of every construct in `f` that binds a second name to an
    engine item or module, or to this crate, so that a count by name
    could no longer see the item's uses."""
    toks, out = f.toks, []
    other = [e for e in ENGINES if e != f.home]
    through = defaultdict(set)
    for segs, bound, k, _exempt in f.use_leaves():
        ln = toks[k][1]
        rooted = f.rooted(segs, k)
        orig = segs[-2] if segs[-1:] == ['self'] and len(segs) > 1 else (segs[-1] if segs else '')
        if rooted and all(s in ('crate', 'self') for s in rooted) and bound not in ('*', '_', orig):
            out.append(ln)                                     # use crate as C, use topo as T, …
            continue
        e = eng.engine_of(segs, through, f, k)
        if e in other:
            through[e].add(bound)
            if bound not in ('*', '_', segs[-1]):
                out.append(ln)                                 # use …engine… as Q
            elif f.rel == 'lib.rs' and segs[-1] in eng.modules[e] and segs[-1] not in eng.declared[e]:
                out.append(ln)                                 # lib.rs re-exports an engine module
    for i, (t, ln) in enumerate(toks):
        if t == 'extern' and [x for x, _ in toks[i + 1:i + 3]] == ['crate', 'self']:
            out.append(ln)
        if t == '::' and i + 1 < len(toks) and toks[i + 1][0] == 'topo' \
                and not (i >= 1 and (is_ident(toks[i - 1][0]) or toks[i - 1][0] == '>')):
            out.append(ln)
        if t == 'macro_use' and i >= 2 and toks[i - 1][0] == '[' and toks[i - 2][0] == '#':
            j = i
            while j < len(toks) and toks[j][0] not in ('mod', ';', '{'):
                j += 1
            name = toks[j + 1][0] if j + 1 < len(toks) and toks[j][0] == 'mod' else ''
            if f.home or name in ENGINES:
                out.append(ln)                                 # #[macro_use] past the visibility check
        alias = t == 'type' and not f.in_impl[i] and i + 1 < len(toks) and is_ident(toks[i + 1][0])
        rule = t == 'macro_rules' and i + 1 < len(toks) and toks[i + 1][0] == '!'
        if not (alias or rule):
            continue
        j, depth = i + 1, 0
        while j < len(toks):
            x = toks[j][0]
            if x in '{([':
                depth += 1
            elif x in '})]':
                depth -= 1
                if depth == 0 and rule:
                    break
            elif x == ';' and depth == 0 and alias:
                break
            j += 1
        body = toks[i + 2:j]
        rooted_root = any(x in eng.root and eng.root[x] in other and m >= 2
                          and body[m - 1][0] == '::' and body[m - 2][0] in ('crate', '$crate', 'topo')
                          for m, (x, _) in enumerate(body))
        if rooted_root or any(x in other or any(x in eng.names[e] for e in other) for x, _ in body):
            out.append(ln)                                     # type Q = …engine…; a macro body
    return out


def names_engine(f, e, eng):
    """The lines of every site in `f` naming engine `e`.

    A name's binding is resolved the way rustc resolves a single
    segment, per inline module: the module's own `use` leaves and
    item declarations, then its parent's only through `use super::*`.
    An ambiguous name counts where it resolves to an import through the
    engine, follows a segment that does, or is `crate::NAME` for a NAME
    the root re-exports from the engine; any other engine name counts
    unless it resolves to an external crate's import."""
    toks, out, ext = f.toks, [], eng.ext
    names, shared = eng.names[e], eng.declared[e] & set(eng.ambiguous)
    through = {x: set() for x in ENGINES}
    binds = defaultdict(dict)               # name -> scope -> 'e' | 'ext' | 'other'
    globs_e, globs_super = set(), set()
    for segs, bound, k, exempt in f.use_leaves():
        sc = f.scope[k]
        if segs and (segs[0] == '::' or segs[0] in ext):
            kind = 'ext'
        elif eng.engine_of(segs, through, f, k) == e:
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
                through[e].add(bound)
        if not exempt and kind == 'e':
            out.append(toks[k][1])
    for name, _, at in f.items():
        binds[name].setdefault(f.scope[at], 'other')
    params = f.type_params()

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
        if pre is None or (pre and pre[0] in ext):
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
        root_pre = f.rooted(pre, i)
        if pre and pre[0][0].isupper():
            # An associated item of a type: the engine's only when the
            # type is not a parameter, `Self`, a std type, one declared
            # outside the engines, or one imported from elsewhere — and
            # not already counted as an engine name itself.
            h = pre[0]
            if h in params or h in STD_HEADS or h in eng.outside or head in ('ext', 'other') \
                    or any(h in eng.declared[x] for x in ENGINES):
                continue
        if t in shared and head != 'e' and not (root_pre == ['crate'] and eng.root.get(t) == e):
            continue
        if not pre and head == 'ext':
            continue
        if t[0].islower() or t[0] == '_':
            after = toks[i + 2][0] if i + 2 < len(toks) else ''
            if nxt == '::' and after != '<':
                continue                    # a module of the same name
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
    args = sys.argv[1:]
    lines_for = None
    if '--lines' in args:
        k = args.index('--lines')
        lines_for = args[k + 1]
        del args[k:k + 2]
    base_path, cargo = args[0], args[1]
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
            print(f'ROSE {key[0]} {key[1]}: {was} -> {now} (run engine-leaves.sh --lines {key[1]} '
                  'to list its counted sites)')
        elif now < was:
            print(f'FELL {key[0]} {key[1]}: {was} -> {now}')
    for name in sorted(set(ambiguous) - listed):
        print(f'AMBIGUOUS-NEW {name}: declared under an engine and in {", ".join(ambiguous[name])}')
    for name in sorted(listed - set(ambiguous)):
        print(f'AMBIGUOUS-GONE {name}')
    for (kind, rel), lines in sorted(sites.items()):
        if rel == lines_for:
            print(f'LINES {kind} {rel}: L{" ".join(map(str, sorted(set(lines))))}')
    if os.environ.get('ENGINE_LEAVES_TABLE'):
        for (kind, rel), lines in sorted(sites.items()):
            print(f'TABLE {kind:<24} {len(lines):>4}  {rel}')
        for name in sorted(ambiguous):
            print(f'TABLE ambiguous {name}  # {", ".join(ambiguous[name])}')
    print(f'TOTAL {sum(len(v) for v in sites.values())}')


if __name__ == '__main__':
    main()
