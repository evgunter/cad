"""The region an entity occupies, read outside its cover: counted per file.

Read by `cover-kind-match.sh`. Reads `gate_rust_code`'s `path:line:code`
records on stdin (comments and literals already stripped, test code
skipped) and counts, per KIND and file, every site of these shapes:

- `arm`: a `Surface::`/`Curve3::` kind in PATTERN position (a match arm,
  `if let`/`while let`, `let … else`, or `matches!`) whose governed code
  names the region vocabulary below. A match arm governs its body; an
  `if let` its block; a `let … else` the rest of its enclosing block; a
  `matches!` the rest of its statement (an `if` block included). A
  pattern nested in another (`Self::Span { carrier: Curve3::Line(_), .. }`)
  counts at the outer pattern's arm. One arm is one site, however many
  kinds its alternatives name.
- `ball`: a literal torus or sphere ball: a sum of a major and a minor
  radius (`major_radius + minor_radius`, `R.abs() + r`, either order),
  or a distance plus a radius (`(c - p).norm() + radius`), with the
  radius term a whole operand (not `minor · cos v`, a chart metric).
- `whole`: a whole carrier's cover read (`.cover()` with no argument):
  the region of the whole carrier, where a consumed span or face reads
  its own (`Curve3::cover_over`, `topo::edge_cover`, `topo::face_cover`).
- `reader`: a reference to one of `topo`'s cover doors (READERS below)
  outside its homes (`topo`'s `boxes.rs` and `cover.rs`), by whole token:
  a call, a turbofish, a path and a point-free use (`.map(face_box)`) are
  each one reference. A door's own `fn` is not one.

THE REGION VOCABULARY, read by identifier: an identifier counts if one
of its words (split at `_` and at case changes) is a region word
(`aabb`, `bbox`, `box` other than `std`'s `Box`, `ball`, `reach`,
`extent`, `support`, `hull`, `cover`, `covered`), or it is one of the
carrier bounds a region is built of (`speed_lo`, `speed_hi`,
`speed_range`, `speed_floor`, `speed_upper_bound`, `semi_axes`,
`distance_range`, `range_along`, `range_about`, `control`, the control
net). The enclosing `fn`'s name counts the same way, and also on the
words a pivot, floor or lever of a region is named by (`foot`, `floor`,
`pivot`, `lever`).

    cover_kind_match.py BASELINE [--base BASE_BASELINE] [--lines FILE]

Prints `ROSE kind file: was -> now`, `FELL kind file: was -> now`,
`RAISED kind file: base -> pin` against `--base`, `MALFORMED …`, then
`TOTAL n`. `--lines FILE` also prints `LINES kind FILE:LINE` for each of
that file's counted sites. The gate decides from them.

WHAT IT CANNOT SEE. A kind read through a method rather than a pattern
(`carrier.conic()`, `surface.kind() == …`, a `SurfaceKind`/`CurveKind`
match), so a site that dispatches on `kind()` is not counted. A region
built under a name the vocabulary does not hold (a `size`, a `radius`
returned bare, an `arm` built from a carrier's radii). A `let` binding
a kind pattern with no `else` (irrefutable only for a one-variant enum,
which neither is). Macro bodies are read as written. A `whole` read of a
cover whose receiver is not a carrier (a topo section circle's
`cover()`) is counted all the same, the safe direction.

COUNTS ARE PER FILE TOTALS, so a site deleted and another added in the
same file leaves the pin unmoved; a FELL forces the lowering in the same
change, which keeps that slack from standing.
"""
import bisect
import re
import sys

COVER_HOMES = re.compile(r'^crates/(geom/src/cover\.rs|geom/src/cover/|topo/src/cover\.rs)')
READER_HOMES = ('crates/topo/src/boxes.rs', 'crates/topo/src/cover.rs')
READERS = (
    'face_box', 'cover_box', 'face_cover', 'face_reach_from', 'face_range_along',
    'face_axial_range', 'cone_face_reach', 'face_cover_toward', 'face_nearest',
    'meet_within', 'hull_of_covers', 'boundary_cover', 'edge_box', 'edge_cover',
    'edge_cover_linked', 'edge_cover_of',
)
KINDS = ('arm', 'ball', 'whole', 'reader')

KIND = re.compile(r'\b(?:geom::)?(?:Surface|Curve3)::[A-Z]\w*')
IDENT = re.compile(r'[A-Za-z_]\w*')
REGION_WORDS = {'aabb', 'bbox', 'box', 'ball', 'reach', 'extent', 'support', 'hull',
                'cover', 'covered'}
REGION_IDENTS = {'speed_lo', 'speed_hi', 'speed_range', 'speed_floor', 'speed_upper_bound',
                 'semi_axes', 'distance_range', 'range_along', 'range_about', 'control'}
FN_WORDS = REGION_WORDS | {'foot', 'floor', 'pivot', 'lever'}
FN = re.compile(r'\bfn\s+([A-Za-z_]\w*)')
WHOLE = re.compile(r'\.cover\(\s*\)')
READER = re.compile(r'\b(' + '|'.join(READERS) + r')\b')
READER_DECL = re.compile(r'\bfn\s+(' + '|'.join(READERS) + r')\b')
# A literal ball: `<major> + <minor>` or `<minor> + <major>`, or a
# distance plus a radius; each operand a whole term.
OPERAND = r'\*?[A-Za-z_][\w.]*(?:\(\))?(?:\.abs\(\))?'
TORUS = re.compile(r'(?<![\w.*/])(' + OPERAND + r')\s*\+\s*(' + OPERAND + r')(?!\s*[*/.(\w])')
SPHERE = re.compile(r'(?:\.norm\(\)|\.distance\([^()]*\))\s*\+\s*\*?([A-Za-z_]\w*)(?:\.abs\(\))?(?!\s*[*/.(\w])')

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
    """Where the kind at `text[start:end]` stands in a pattern, the code it
    governs, keyed by the pattern's own place; else None."""
    k, tok = scan_to(text, end, ['=>', ';', ',', '='])
    climbs = 0
    # A pattern nested in a struct or tuple pattern: climb out of it.
    while tok in (',', 'neg') and climbs < 4:
        if tok == ',':
            k, tok = scan_to(text, k + 1, [])
        k, tok = scan_to(text, k + 1, ['=>', ';', ',', '='])
        climbs += 1
    before = text[max(0, start - 200):start]
    if tok == '=>':
        return k, arm_body(text, k + 2)
    if tok == '=' and re.search(r'\blet\s+[^;{}]*$', before):
        k2, tok2 = scan_to(text, k + 1, ['{', ';'])
        if tok2 == '{' and re.search(r'\b(if|while)\s+let\s+[^;{}]*$', before):
            return k, block(text, k2)
        if tok2 == '{' and re.search(r'\belse\s*$', text[k + 1:k2]):
            k3, _ = scan_to(text, k2, [])
            return k, text[k2:k3]
        return None
    opened = re.search(r'\bmatches!\s*\([^;]*$', before[-160:])
    if opened:
        at = start - len(before[-160:]) + opened.start()
        k4, _ = scan_to(text, at, [';'])
        return k, text[at:k4]
    return None


def assemble(lines):
    parts, starts, pos = [], [], 0
    for i in range(1, max(lines) + 1):
        starts.append(pos)
        t = lines.get(i, '')
        parts.append(t + '\n')
        pos += len(t) + 1
    return ''.join(parts), starts


def sites(path, lines):
    """`(kind, line)` for each counted site of one file."""
    text, starts = assemble(lines)
    line = lambda pos: bisect.bisect_right(starts, pos)  # noqa: E731
    fns = [(m.start(), m.group(1)) for m in FN.finditer(text)]
    fn_at = lambda pos: fns[bisect.bisect_right(fns, (pos, '￿')) - 1][1] if fns and fns[0][0] <= pos else ''  # noqa: E731
    out = []
    if not COVER_HOMES.match(path):
        seen = set()
        for m in KIND.finditer(text):
            got = governed(text, m.start(), m.end())
            if got is None or got[0] in seen:
                continue
            key, body = got
            if region(body) or FN_WORDS.intersection(words(fn_at(m.start()))):
                seen.add(key)
                out.append(('arm', line(m.start())))
        for n, code in sorted(lines.items()):
            for m in TORUS.finditer(code):
                a, b = (words(x.split('.')[0].lstrip('*')) for x in m.groups())
                if ('major' in a and 'minor' in b) or ('minor' in a and 'major' in b):
                    out.append(('ball', n))
            for m in SPHERE.finditer(code):
                if 'radius' in words(m.group(1)):
                    out.append(('ball', n))
            out += [('whole', n)] * len(WHOLE.findall(code))
    if path not in READER_HOMES:
        for n, code in sorted(lines.items()):
            hits = len(READER.findall(code)) - len(READER_DECL.findall(code))
            out += [('reader', n)] * hits
    return out


def read_pins(path):
    pins, bad = {}, []
    with open(path, encoding='utf-8') as fh:
        for n, raw in enumerate(fh, 1):
            line = raw.split('#', 1)[0].strip()
            if not line:
                continue
            parts = line.split()
            if len(parts) != 3 or parts[0] not in KINDS or not parts[1].isdigit():
                bad.append(f'{path}:{n}: {raw.rstrip()}')
                continue
            pins[(parts[0], parts[2])] = int(parts[1])
    return pins, bad


def main():
    args = sys.argv[1:]
    base_file = lines_for = None
    if '--base' in args:
        i = args.index('--base')
        base_file = args[i + 1]
        del args[i:i + 2]
    if '--lines' in args:
        i = args.index('--lines')
        lines_for = args[i + 1]
        del args[i:i + 2]
    pins, bad = read_pins(args[0])
    for b in bad:
        print(f'MALFORMED {b}')
    files = {}
    for rec in sys.stdin:
        path, ln, code = rec.rstrip('\n').split(':', 2)
        files.setdefault(path, {})[int(ln)] = code
    counts = {}
    for path in sorted(files):
        for kind, n in sites(path, files[path]):
            counts[(kind, path)] = counts.get((kind, path), 0) + 1
            if path == lines_for:
                print(f'LINES {kind} {path}:{n}')
    for key in sorted(set(counts) | set(pins), key=lambda k: (k[1], k[0])):
        now, was = counts.get(key, 0), pins.get(key, 0)
        if now > was:
            print(f'ROSE {key[0]} {key[1]}: {was} -> {now}')
        elif now < was:
            print(f'FELL {key[0]} {key[1]}: {was} -> {now}')
    if base_file is not None:
        base, _ = read_pins(base_file)
        for key, pin in sorted(pins.items(), key=lambda kv: (kv[0][1], kv[0][0])):
            if pin > base.get(key, 0):
                print(f'RAISED {key[0]} {key[1]}: {base.get(key, 0)} -> {pin}')
    print(f'TOTAL {sum(counts.values())}')


main()
