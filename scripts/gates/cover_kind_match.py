"""The region an entity occupies, read outside its cover: counted per file.

Read by `cover-kind-match.sh`. Reads `gate_rust_code`'s `path:line:code`
records on stdin (comments and literals already stripped, test code
skipped) and finds, per file, every site of these shapes:

- `arm`: a kind of `geom`'s `Surface` or `Curve3` in PATTERN position (a
  match arm, `if let`/`while let`, `let … else`, or `matches!`) whose
  governed code names the region vocabulary below. A kind is the enum's
  variant however the file names it (the names are RESOLVED, below). A
  match arm governs its body; an `if let` its block; a `let … else` the
  rest of its enclosing block; a `matches!` the rest of its statement (an
  `if` block included). A pattern nested in a struct or tuple pattern
  (`Self::Span { carrier: Curve3::Line(_), .. }`) counts at the outer
  pattern's arm: the counter climbs out of every bracket that is not a
  block until it meets the pattern's `=>`, `=` or its statement's end, so
  no nesting depth is assumed. One arm is one site, however many kinds
  its alternatives name.
- `ball`: a literal torus or sphere ball, read over the whole file's code
  (a sum rustfmt wraps is one sum): a sum of a major and a minor radius
  (`major_radius + minor_radius`, `t.major_radius() + t.minor_radius`,
  `R.abs() + r`, either order), or a distance plus a radius
  (`(c - p).norm() + radius`, `s.radius + c.distance(q.center())`, a bare
  `r` or `rho`, a conic's `speed_hi()` bound on its semi-axes, or a name
  the file binds to one of them by `let`, as the radius). Each operand is a whole term: not a factor
  of a product (`minor · cos v`, a chart metric). Not a ball, by
  structure: a difference of squares (`(R − r)·(R + r)`), a parenthesized
  sum added to another parenthesized group (two carriers' radii, not one
  ball: `(R1 + r1) + (R2 + r2)`), and a sum of the radii's perturbations
  (operands whose first word is `d` or `delta`).
- `whole`: a whole carrier's cover read (`.cover()` with no argument):
  the region of the whole carrier, where a consumed span or face reads
  its own (`Curve3::cover_over`, `topo::edge_cover`, `topo::face_cover`).
- `reader`: a reference to one of `topo`'s cover doors outside its homes,
  by whole token (a call, a turbofish, a path, a method call and a
  point-free use are each one; a `use` naming it is not a read), grouped
  by its file and enclosing fn.
  The doors are DERIVED: every free `fn` declared `pub` or `pub(…)` at
  the top level of `topo`'s `cover.rs` and `boxes.rs` (production code),
  less NOT_DOORS, so a new door is a new reader's subject the day it
  lands. A door's own `fn` is not a reference. Readers are not pinned
  but REGISTERED (`--readers`), each site with the direction it reads
  looseness in.

NAME RESOLUTION, per file. `Surface`, `Curve3` and `geom::`-qualified
paths to them; every `… as NAME` alias of either; the variants a `use`
brings into scope bare (`use Surface::{Plane, Sphere}`, `use Curve3::*`,
through an alias too); and `Self::` inside an `impl` whose self type is
either (geom's own `impl Curve3`). The variants are read off the enums'
own declarations (VARIANTS_FROM), so a new variant is seen. An alias or
an import counts in the whole file wherever it was declared (a `use`
inside one fn is read file-wide): over-counting, the safe direction.

THE REGION VOCABULARY, read by identifier: an identifier counts if one
of its words (split at `_` and at case changes) is a region word (box
other than `std`'s `Box`, aabb, bbox, ball, reach, extent, support,
hull, cover, covered; and a distance range's: near, nearest, far,
farthest, distance, clear, clearance), or it is one of the carrier
bounds a region is built of (REGION_IDENTS: the speed bounds, the
semi-axes, the control net, the cover's range readings). The enclosing
`fn`'s name counts the same way, and also on the words a pivot, floor
or lever of a region is named by (foot, floor, pivot, lever).

    cover_kind_match.py BASELINE [--base BASE_BASELINE] [--renames FILE]
                        [--readers REGISTRY] [--lines FILE]

Prints `ROSE kind file: was -> now`, `FELL kind file: was -> now`,
`RAISED kind file: base -> pin` against `--base`, `MALFORMED …`,
`DUPLICATE …`; for the readers `UNARGUED file fn: n`, `VANISHED file fn`,
`MOVED file fn: was -> now`, `BADREASON …`; then `TOTAL …`. `--lines
FILE` also prints `LINES kind FILE:LINE FN` for each of that file's sites.

A PIN FOLLOWS A MOVE. `--renames` lists `old new` path pairs (the shell
passes git's rename detection between the base and the tree), and a base
pin is read under its new path. A file split in two keeps its base pin
by citing it: a pin line ending `# from OLD_PATH` is held, summed with
every other line citing OLD_PATH and OLD_PATH's own line, to OLD_PATH's
base pin.

WHAT IT CANNOT SEE. A kind read through a method rather than a pattern
(`carrier.conic()`, `surface.kind() == …`, a `SurfaceKind`/`CurveKind`
match), so a site that dispatches on `kind()` is not counted. A region
built under a name the vocabulary does not hold (a `size`, a `radius`
returned bare, an `arm` built from a carrier's radii, a point set a lever
is the farthest of). A variant reached through a re-export under another
module's name (`pub use geom::Surface::Plane` elsewhere, then `use
that::Plane`), and an alias declared in another file. Macro bodies are
read as written. A ball whose radius is named none of those, and a
torus sum whose operands are not named `major`/`minor`. A
`whole` read of a cover whose receiver is not a carrier (a topo section
circle's `cover()`) is counted all the same, the safe direction.

COUNTS ARE PER FILE TOTALS, so a site deleted and another added in the
same file leaves the pin unmoved; a FELL forces the lowering in the same
change, which keeps that slack from standing.
"""
import bisect
import re
import sys

COVER_HOMES = re.compile(r'^crates/(geom/src/cover\.rs|geom/src/cover/|topo/src/cover\.rs)')
READER_HOMES = ('crates/topo/src/boxes.rs', 'crates/topo/src/cover.rs')
# The homes' public free fns that are not region doors, each with why.
NOT_DOORS = {
    # The band's own tolerance a box is padded by: its reader reads the
    # box it pads, and that box's reader is the site.
    'sweep_pad',
    # A direction's three brackets, the narrow phase's key for telling
    # two axes apart: no region.
    'axis_key',
    # An edge's end vertex's point, which the edge's cover hulls: a
    # point of the region, not a bound on it.
    'edge_end_point',
}
VARIANTS_FROM = {
    'Surface': 'crates/geom/src/surfaces.rs',
    'Curve3': 'crates/geom/src/curves.rs',
}
PINNED_KINDS = ('arm', 'ball', 'whole')
DIRECTIONS = ('prunes', 'refuses', 'examines', 'two-sided', 'namesake')

IDENT = re.compile(r'[A-Za-z_]\w*')
REGION_WORDS = {'aabb', 'bbox', 'box', 'ball', 'reach', 'extent', 'support', 'hull',
                'cover', 'covered', 'near', 'nearest', 'far', 'farthest', 'distance',
                'clear', 'clearance'}
REGION_IDENTS = {'speed_lo', 'speed_hi', 'speed_range', 'speed_floor', 'speed_upper_bound',
                 'semi_axes', 'distance_range', 'range_along', 'range_about', 'control'}
FN_WORDS = REGION_WORDS | {'foot', 'floor', 'pivot', 'lever'}
FN = re.compile(r'\bfn\s+([A-Za-z_]\w*)')
WHOLE = re.compile(r'\.cover\(\s*\)')
PUB_FN = re.compile(r'\bpub(?:\s*\([^)]*\))?\s+(?:(?:const|async|unsafe)\s+)*fn\s+([A-Za-z_]\w*)')
# A struct pattern's or literal's `{` follows its type's path, whose last
# segment is capitalized (`Self::Span {`, `geom::Circle {`); every other
# `{` opens a block (`match self {`, `if x {`, `=> {`, `) {`, `|c| {`).
STRUCT_HEAD = re.compile(r'(?:^|[^\w])(?:[A-Za-z_]\w*::)*[A-Z]\w*\s*$')

OPEN, CLOSE = '([{', ')]}'


def words(ident):
    out = []
    for part in ident.split('_'):
        out += [w.lower() for w in re.findall(r'[A-Z]?[a-z0-9]+|[A-Z]+(?![a-z])', part)]
    return out


def region(text):
    for ident in IDENT.findall(text):
        if ident in REGION_IDENTS:
            return True
        if ident == 'Box':
            continue
        if REGION_WORDS.intersection(words(ident)):
            return True
    return False


def scan_to(text, i, stops):
    """From `i`, the first of `stops` at depth 0 — `(index, stop)` — or
    `(index, 'neg')` where a closer leaves the depth `i` started at."""
    depth, n = 0, len(text)
    while i < n:
        if depth == 0:
            for stop in stops:
                if text.startswith(stop, i):
                    if stop == '=' and (text.startswith('==', i) or text.startswith('=>', i)
                                        or text[i - 1] in '!<>=+-*/%&|^'):
                        continue
                    return i, stop
        c = text[i]
        if c in OPEN:
            depth += 1
        elif c in CLOSE:
            depth -= 1
            if depth < 0:
                return i, 'neg'
        i += 1
    return n, 'eof'


def opener_of(text, k):
    """The index of the opener the closer at `text[k]` closes."""
    depth = 0
    for j in range(k - 1, -1, -1):
        c = text[j]
        if c in CLOSE:
            depth += 1
        elif c in OPEN:
            if depth == 0:
                return j
            depth -= 1
    return 0


def statement_start(text, i, exits=0):
    """Where the statement holding `text[i]` starts, passing out of
    `exits` enclosing brackets first (a nested pattern's)."""
    depth = 0
    for j in range(i - 1, -1, -1):
        c = text[j]
        if c in CLOSE:
            if c == '}' and depth == 0 and exits == 0:
                return j + 1
            depth += 1
        elif c in OPEN:
            if depth > 0:
                depth -= 1
            elif exits > 0:
                exits -= 1
            else:
                return j + 1
        elif c in ';,' and depth == 0 and exits == 0:
            return j + 1
    return 0


def is_block(text, opener):
    """Whether the `{` at `opener` opens a block rather than a struct
    pattern or literal."""
    return not STRUCT_HEAD.search(text[statement_start(text, opener):opener])


def block(text, i):
    """The block opening at `text[i] == '{'`."""
    k, _ = scan_to(text, i + 1, [])
    return text[i:k + 1]


def arm_body(text, i):
    while i < len(text) and text[i] in ' \n\t':
        i += 1
    if i < len(text) and text[i] == '{':
        return block(text, i)
    k, _ = scan_to(text, i, [','])
    return text[i:k]


def governed(text, start, end):
    """Where the kind at `text[start:end]` stands in a pattern, its key
    (the place its pattern ends) and the code it governs; else None."""
    pos, exits = end, 0
    while True:
        k, tok = scan_to(text, pos, ['=>', ';', ',', '='])
        if tok == ',':
            k, tok = scan_to(text, k + 1, [])
        if tok != 'neg':
            break
        opener = opener_of(text, k)
        if text[opener] == '(' and text[:opener].rstrip().endswith('matches!'):
            at = statement_start(text, opener)
            k4, _ = scan_to(text, at, [';'])
            return k, text[at:k4]
        if text[opener] == '{' and is_block(text, opener):
            return None
        exits += 1
        pos = k + 1
    if tok == '=>':
        return k, arm_body(text, k + 2)
    if tok != '=':
        return None
    before = text[statement_start(text, start, exits):start]
    if not re.search(r'\blet\b', before):
        return None
    k2, tok2 = scan_to(text, k + 1, ['{', ';'])
    if tok2 == '{' and re.search(r'\b(if|while)\s+let\b', before):
        return k, block(text, k2)
    if tok2 == '{' and re.search(r'\belse\s*$', text[k + 1:k2]):
        k3, _ = scan_to(text, k2, [])
        return k, text[k2:k3]
    return None


def assemble(lines):
    parts, starts, pos = [], [], 0
    for i in range(1, max(lines) + 1):
        starts.append(pos)
        t = lines.get(i, '')
        parts.append(t + '\n')
        pos += len(t) + 1
    return ''.join(parts), starts


def variants_of(files):
    """Each kind enum's variants, read off its declaration."""
    out = {}
    for enum, path in VARIANTS_FROM.items():
        if path not in files:
            continue
        text, _ = assemble(files[path])
        m = re.search(r'\bpub\s+enum\s+' + enum + r'\b[^{]*\{', text)
        if not m:
            continue
        body = block(text, m.end() - 1)[1:-1]
        names, depth = [], 0
        for tok in re.finditer(r'[{}()\[\]]|\b[A-Z]\w*\b', body):
            t = tok.group(0)
            if t in OPEN:
                depth += 1
            elif t in CLOSE:
                depth -= 1
            elif depth == 0:
                names.append(t)
        out[enum] = sorted(set(names))
    return out


def kind_pattern(text, variants):
    """The regex of every spelling of a kind in this file, the spans of
    the `impl` blocks of a kind enum (where `Self::` is one), and the
    `Self::` regex."""
    heads = {enum: {enum} for enum in variants}
    for enum in variants:
        for m in re.finditer(r'\b' + enum + r'\s+as\s+([A-Za-z_]\w*)', text):
            heads[enum].add(m.group(1))
    bare = set()
    for m in re.finditer(r'\buse\b[^;]*;', text):
        stmt = m.group(0)
        for enum, names in heads.items():
            for head in names:
                for g in re.finditer(r'\b' + head + r'::(\{[^}]*\}|\*|[A-Z]\w*)', stmt):
                    sel = g.group(1)
                    if sel == '*':
                        bare.update(variants[enum])
                    else:
                        bare.update(v for v in re.findall(r'\b[A-Z]\w*\b', sel)
                                    if v in variants[enum])
    alts = []
    for enum, names in heads.items():
        if not variants[enum]:
            continue
        alts.append(r'\b(?:geom::)?(?:' + '|'.join(sorted(names)) + r')::(?:'
                    + '|'.join(variants[enum]) + r')\b')
    if bare:
        alts.append(r'(?<![:\w])(?:' + '|'.join(sorted(bare)) + r')\b(?!\s*::)')
    impls = []
    for m in re.finditer(r'\bimpl\b', text):
        k, tok = scan_to(text, m.end(), ['{', ';'])
        if tok != '{':
            continue
        header = text[m.end():k].strip()
        if header.startswith('<'):
            close, depth = 0, 0
            for j, c in enumerate(header):
                depth += {'<': 1, '>': -1}.get(c, 0)
                if depth == 0:
                    close = j
                    break
            header = header[close + 1:]
        self_ty = re.split(r'\bfor\b', header)[-1]
        self_ty = re.split(r'\bwhere\b', self_ty)[0]
        base = re.sub(r'<.*', '', self_ty, flags=re.S).split('::')[-1].strip()
        if any(base in names for names in heads.values()):
            impls.append((k, k + len(block(text, k))))
    every = sorted({v for vs in variants.values() for v in vs})
    selfs = re.compile(r'\bSelf::(?:' + '|'.join(every) + r')\b') if every else None
    return (re.compile('|'.join(alts)) if alts else None), impls, selfs


def fn_spans(text):
    spans = []
    for m in FN.finditer(text):
        k, tok = scan_to(text, m.end(), ['{', ';'])
        if tok == '{':
            spans.append((m.start(), k + len(block(text, k)), m.group(1)))
    return spans


def fn_at(spans, pos):
    best = None
    for a, b, name in spans:
        if a <= pos < b and (best is None or a > best[0]):
            best = (a, name)
    return best[1] if best else '-'


# A ball's operands: a whole term, optionally a receiver chain's field
# or method, optionally `.abs()`.
TERM = r'\*?(?:[A-Za-z_]\w*(?:\(\))?\.)*[A-Za-z_]\w*(?:\(\))?(?:\.abs\(\))?'
TORUS = re.compile(r'(?<![\w.])(' + TERM + r')\s*\+\s*(' + TERM + r')')
DIST = r'(?:\.norm\(\)|\.distance\((?:[^()]|\([^()]*\))*\))'
RECEIVER = r'(?:\((?:[^()]|\([^()]*\))*\)|[A-Za-z_][\w.]*(?:\([^()]*\))?)'
SPHERE = re.compile(DIST + r'\s*\+\s*(' + TERM + r')(?![\w.(])|(?<![\w.])(' + TERM
                    + r')\s*\+\s*' + RECEIVER + DIST)


def last_words(term):
    seg = re.sub(r'\(\)|\.abs\(\)', '', term.lstrip('*')).split('.')[-1]
    return words(seg)


def radius_of(term):
    w = last_words(term)
    if not w or w[0] in ('d', 'delta'):
        return None
    return 'major' if 'major' in w else 'minor' if 'minor' in w else None


def difference_of_squares(a, b, outer_left, outer_right):
    for x, y in ((a, b), (b, a)):
        diff = r'\(\s*' + re.escape(x) + r'\s*-\s*' + re.escape(y) + r'\s*\)'
        if re.match(r'\*\s*' + diff, outer_right) or re.search(diff + r'\s*\*$', outer_left):
            return True
    return False


def balls(text):
    out = []
    for m in TORUS.finditer(text):
        a, b = m.group(1), m.group(2)
        if {radius_of(a), radius_of(b)} != {'major', 'minor'}:
            continue
        left, right = text[:m.start()].rstrip(), text[m.end():].lstrip()
        if left.endswith(('*', '/')) or re.match(r'[*/.(\w]', right):
            continue
        if left.endswith('(') and right.startswith(')'):
            outer_left, outer_right = left[:-1].rstrip(), right[1:].lstrip()
            if difference_of_squares(a, b, outer_left, outer_right):
                continue
            if re.match(r'\+\s*\(', outer_right) or re.search(r'\)\s*\+$', outer_left):
                continue
        out.append(m.start())
    # A radius: named one, or a conic's bound on its semi-axes, or a
    # `let` binding of either in the file.
    bound = set(re.findall(r'\blet\s+(?:mut\s+)?([a-z_]\w*)\s*=[^;=]*?(?:radius|speed_hi)\b[^;]*;', text))
    for m in SPHERE.finditer(text):
        term = m.group(1) or m.group(2)
        w = last_words(term)
        name = re.sub(r'\(\)|\.abs\(\)', '', term.lstrip('*')).split('.')[-1]
        if 'radius' in w or w in (['r'], ['rho']) or name in ('speed_hi',) or name in bound:
            out.append(m.start())
    return out


def door_names(files):
    names = set()
    for path in READER_HOMES:
        if path not in files:
            continue
        text, _ = assemble(files[path])
        depth, last = 0, 0
        for m in PUB_FN.finditer(text):
            seg = text[last:m.start()]
            depth += seg.count('{') - seg.count('}')
            last = m.start()
            if depth == 0:
                names.add(m.group(1))
    return sorted(names - NOT_DOORS)


def sites(path, lines, variants, doors):
    """`(kind, line, fn)` for each counted site of one file."""
    text, starts = assemble(lines)
    line = lambda pos: bisect.bisect_right(starts, pos)  # noqa: E731
    spans = fn_spans(text)
    out = []
    if not COVER_HOMES.match(path):
        kinds, impls, selfs = kind_pattern(text, variants)
        hits = list(kinds.finditer(text)) if kinds else []
        if selfs:
            hits += [m for m in selfs.finditer(text) if any(a <= m.start() < b for a, b in impls)]
        seen = set()
        for m in sorted(hits, key=lambda m: m.start()):
            got = governed(text, m.start(), m.end())
            if got is None or got[0] in seen:
                continue
            key, body = got
            fn = fn_at(spans, m.start())
            if region(body) or FN_WORDS.intersection(words(fn)):
                seen.add(key)
                out.append(('arm', line(m.start()), fn))
        out += [('ball', line(p), fn_at(spans, p)) for p in balls(text)]
        out += [('whole', line(m.start()), fn_at(spans, m.start())) for m in WHOLE.finditer(text)]
    if path not in READER_HOMES and doors:
        ref = re.compile(r'(?<![\w])(?:' + '|'.join(doors) + r')\b')
        uses = [(u.start(), u.end()) for u in re.finditer(r'\buse\b[^;]*;', text)]
        for m in ref.finditer(text):
            if re.search(r'\bfn\s+$', text[max(0, m.start() - 8):m.start()]):
                continue
            if any(a <= m.start() < b for a, b in uses):
                continue
            out.append(('reader', line(m.start()), fn_at(spans, m.start())))
    return out


def read_pins(path):
    pins, origins, bad, dup = {}, {}, [], []
    with open(path, encoding='utf-8') as fh:
        for n, raw in enumerate(fh, 1):
            code, _, note = raw.partition('#')
            line = code.strip()
            if not line:
                continue
            parts = line.split()
            if len(parts) != 3 or parts[0] not in PINNED_KINDS or not parts[1].isdigit():
                bad.append(f'{path}:{n}: {raw.rstrip()}')
                continue
            key = (parts[0], parts[2])
            if key in pins:
                dup.append(f'{path}:{n}: {raw.rstrip()}')
            pins[key] = int(parts[1])
            m = re.match(r'\s*from\s+(\S+)', note)
            if m:
                origins[key] = m.group(1)
    return pins, origins, bad, dup


def read_registry(path):
    reg, bad = {}, []
    with open(path, encoding='utf-8') as fh:
        for n, raw in enumerate(fh, 1):
            if not raw.strip() or raw.lstrip().startswith('#'):
                continue
            parts = raw.rstrip('\n').split('\t')
            if (len(parts) != 5 or not parts[2].isdigit() or parts[3] not in DIRECTIONS
                    or len(parts[4].split()) < 6):
                bad.append(f'{path}:{n}: {raw.rstrip()}')
                continue
            key = (parts[0], parts[1])
            if key in reg:
                bad.append(f'{path}:{n}: a second entry for {parts[0]} {parts[1]}')
            reg[key] = int(parts[2])
    return reg, bad


def option(args, name):
    if name in args:
        i = args.index(name)
        value = args[i + 1]
        del args[i:i + 2]
        return value
    return None


def main():
    args = sys.argv[1:]
    base_file = option(args, '--base')
    renames_file = option(args, '--renames')
    registry = option(args, '--readers')
    lines_for = option(args, '--lines')
    pins, origins, bad, dup = read_pins(args[0])
    for b in bad:
        print(f'MALFORMED {b}')
    for d in dup:
        print(f'DUPLICATE {d}')
    files = {}
    for rec in sys.stdin:
        path, ln, code = rec.rstrip('\n').split(':', 2)
        files.setdefault(path, {})[int(ln)] = code
    variants = variants_of(files)
    missing = sorted(set(VARIANTS_FROM) - {e for e, vs in variants.items() if vs})
    if missing:
        print(f'REFUSED no variants read for {", ".join(missing)} from '
              f'{", ".join(VARIANTS_FROM[e] for e in missing)}: the kinds are unknown',
              file=sys.stderr)
        sys.exit(2)
    doors = door_names(files)
    counts, readers = {}, {}
    for path in sorted(files):
        for kind, n, fn in sites(path, files[path], variants, doors):
            if kind == 'reader':
                readers[(path, fn)] = readers.get((path, fn), 0) + 1
            else:
                counts[(kind, path)] = counts.get((kind, path), 0) + 1
            if path == lines_for:
                print(f'LINES {kind} {path}:{n} {fn}')
    for key in sorted(set(counts) | set(pins), key=lambda k: (k[1], k[0])):
        now, was = counts.get(key, 0), pins.get(key, 0)
        if now > was:
            print(f'ROSE {key[0]} {key[1]}: {was} -> {now}')
        elif now < was:
            print(f'FELL {key[0]} {key[1]}: {was} -> {now}')
    if base_file is not None:
        base, _, _, _ = read_pins(base_file)
        if renames_file:
            with open(renames_file, encoding='utf-8') as fh:
                moved = dict(line.split()[:2] for line in fh if len(line.split()) >= 2)
            base = {(k, moved.get(p, p)): n for (k, p), n in base.items()}
        held = {}
        for key, pin in pins.items():
            target = (key[0], origins.get(key, key[1]))
            held[target] = held.get(target, 0) + pin
        for target, total in sorted(held.items(), key=lambda kv: (kv[0][1], kv[0][0])):
            if total > base.get(target, 0):
                print(f'RAISED {target[0]} {target[1]}: {base.get(target, 0)} -> {total}')
    if registry is not None:
        reg, bad_reg = read_registry(registry)
        for b in bad_reg:
            print(f'BADREASON {b}')
        for key in sorted(set(reg) | set(readers)):
            now, was = readers.get(key, 0), reg.get(key)
            if was is None:
                print(f'UNARGUED {key[0]} {key[1]}: {now}')
            elif now == 0:
                print(f'VANISHED {key[0]} {key[1]}')
            elif now != was:
                print(f'MOVED {key[0]} {key[1]}: {was} -> {now}')
    print(f'TOTAL {sum(counts.values())} sites pinned; {sum(readers.values())} reads of {len(doors)} doors registered')


if __name__ == '__main__':
    main()
