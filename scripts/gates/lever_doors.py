"""The lever doors that take a caller's number or point, counted per file.

Read by `lever-doors.sh`. Reads `gate_rust_code --skip-cfg-test`'s
`path:line:code` records on stdin (production code: comments, string
literals and `#[cfg(test)]` items already stripped) and counts every
REFERENCE to a held door outside that door's home: a call, a fn pointer
and a `use` of a free fn each spell the name once.

    lever_doors.py PINS [--base BASE_PINS] [--renames FILE] [--lines PATH]

Prints, then `TOTAL n`:

    ROSE path: pin -> now      more references than the file's pin
    FELL path: pin -> now      fewer: lower the pin
    RAISED path: base -> pin   a pin above the merge base's (a file git
                               moved keeps its pin: `--renames` is
                               `OLD NEW` per line; a file split from
                               another cites it, `# from OLD_PATH`)
    ALIAS path:line            one of the three types renamed or aliased,
                               which would take a qualified door's
                               references out of the count
    IMPL path:line             an impl block of one of the three types
                               outside its home, whose `Self::door(…)`
                               the count cannot see
    MALFORMED text             a pin line that is not `PATH COUNT`
    SITE path:line name        each counted reference in `--lines PATH`
                               (`ALL` for every file)
"""
import re
import sys

CURVATURE_HOME = 'crates/geom-brep/src/curvature_arm.rs'
EXTENT_HOME = 'crates/geom-brep/src/extent.rs'
LEVER_HOME = 'crates/geom-core/src/predicate.rs'

# door -> the one file that may reference it uncounted.
DOORS = {
    'curvature_radius': CURVATURE_HOME,
    'within_curvature': CURVATURE_HOME,
    'ExtentBall::point': EXTENT_HOME,
    'ExtentBall::of_box': EXTENT_HOME,
    'ExtentBall::box_diagonal': EXTENT_HOME,
    'points_lever_from': EXTENT_HOME,
    'points_diameter': EXTENT_HOME,
    'of_cover_within': EXTENT_HOME,
    'Reach::of_face': EXTENT_HOME,
    'lever_from_within': EXTENT_HOME,
    'of_crossing_cylinders': EXTENT_HOME,
    'hinge_lever': EXTENT_HOME,
    'turn_lever': EXTENT_HOME,
    'ruled_lever': EXTENT_HOME,
    'generator_lever': EXTENT_HOME,
}
TYPES = r'(?:Lever|ExtentBall|Reach)'
word = re.compile(r'(?<![\w:])(?:\w+::)*(' + '|'.join(map(re.escape, DOORS)) + r')\b')
# A door's own declaration is not a reference to it.
declared = re.compile(r'\bfn\s+(\w+)')
turbofish = re.compile(r'\b(' + TYPES + r')::<(?:[^<>]|<[^<>]*>)*>::')
# `Lever`'s tuple field is crate-visible, so geom-core can write the
# struct literal anywhere in its own sources.
literal = re.compile(r'(?<![\w:])Lever\(')
alias = re.compile(r'\b' + TYPES + r'\s+as\s+\w|\btype\s+\w+[^=;]*=\s*(?:\w+::)*' + TYPES + r'\b')
impl = re.compile(r'\bimpl\b[^{;]*\b' + TYPES + r'\s*<')
trait_impl = re.compile(r'\bfor\s+(?:\w+::)*(?:Lever|ExtentBall)\s*<')


origins = {}


def read_pins(path, problems):
    pins = {}
    with open(path, encoding='utf-8') as fh:
        for raw in fh:
            line, _, note = raw.partition('#')
            line = line.strip()
            if not line:
                continue
            split_from = re.match(r'\s*from\s+(\S+)', note)
            if split_from:
                origins[line.split()[0]] = split_from.group(1)
            parts = line.split()
            if len(parts) != 2 or not parts[1].isdigit() or parts[0] in pins:
                problems.append(f'MALFORMED {raw.rstrip()}')
                continue
            pins[parts[0]] = int(parts[1])
    return pins


def option(name):
    return sys.argv[sys.argv.index(name) + 1] if name in sys.argv else None


out = []
pins = read_pins(sys.argv[1], out)
lines_for = option('--lines')
counts = {}
for rec in sys.stdin:
    path, ln, code = rec.rstrip('\n').split(':', 2)
    code = turbofish.sub(r'\1::', code)
    own = set(declared.findall(code))
    hits = [m.group(1) for m in word.finditer(code)
            if DOORS[m.group(1)] != path and m.group(1).split('::')[-1] not in own]
    if path.startswith('crates/geom-core/src/') and path != LEVER_HOME:
        hits += ['Lever(…)'] * len(literal.findall(code))
        if impl.search(code) and 'Lever' in code:
            out.append(f'IMPL {path}:{ln}')
    if path.startswith('crates/geom-brep/src/') and path != EXTENT_HOME:
        if impl.search(code) and re.search(r'\b(ExtentBall|Reach)\s*<', code):
            out.append(f'IMPL {path}:{ln}')
    if path not in (EXTENT_HOME, LEVER_HOME):
        if trait_impl.search(code):
            out.append(f'IMPL {path}:{ln}')
        if alias.search(code):
            out.append(f'ALIAS {path}:{ln}')
    if hits:
        counts[path] = counts.get(path, 0) + len(hits)
        if lines_for in (path, 'ALL'):
            out += [f'SITE {path}:{ln} {h}' for h in hits]

for key in sorted(set(counts) | set(pins)):
    now, was = counts.get(key, 0), pins.get(key, 0)
    if now > was:
        out.append(f'ROSE {key}: {was} -> {now}')
    elif now < was:
        out.append(f'FELL {key}: {was} -> {now}')

base_path = option('--base')
if base_path:
    base = read_pins(base_path, [])
    moved = {}
    renames = option('--renames')
    if renames:
        with open(renames, encoding='utf-8') as fh:
            for line in fh:
                old, new = line.split()
                moved[new] = old
    for key, pin in sorted(pins.items()):
        was = sum(base.get(k, 0) for k in {key, moved.get(key), origins.get(key)} if k)
        if pin > was:
            out.append(f'RAISED {key}: {was} -> {pin}')

print('\n'.join([*out, f'TOTAL {sum(counts.values())}']))
