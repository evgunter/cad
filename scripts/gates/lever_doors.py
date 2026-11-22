"""The lever doors that take a caller's number or point, counted per file and door.

Read by `lever-doors.sh`. Reads `gate_rust_code --skip-cfg-test
--statements`' `path:line:statement` records on stdin (production code,
one record per statement, so a call split over lines is one text) and
counts every REFERENCE to a held door outside that door's home: a call,
a fn pointer and a `use` of a free fn each spell the name once.

    lever_doors.py PINS [--base BASE_PINS] [--renames FILE] [--lines PATH]

The pins are `pin_ledger`'s: `DOOR COUNT FILE`, one line per door and
file, so a site swapped for another door's in one file moves two pins.

Each statement is NORMALISED before it is matched: the space about `::`
goes, a turbofish on one of the three types goes (`ExtentBall::<f64>::`),
and a qualified path to one is read as the type (`<ExtentBall<f64>>::`).

Prints, then `TOTAL n`:

    ROSE door path: pin -> now    more references than the pin
    FELL door path: pin -> now    fewer: lower the pin
    RAISED door path: base -> pin the pins of one door and origin, summed,
                                  above the merge base's (`pin_ledger`)
    ALIAS path:line               one of the three types, or a held free
                                  fn, renamed or aliased, which would take
                                  a door's references out of the count
    IMPL path:line                an impl block of one of the three types
                                  outside its home, whose `Self::door(…)`
                                  the count cannot see
    MALFORMED / DUPLICATE text    a pin line that is not `DOOR COUNT FILE`,
                                  or a second line for one door and file
    SITE path:line door           each counted reference in `--lines PATH`
                                  (`ALL` for every file)
"""
import re
import sys

import pin_ledger

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
    'Reach::Measured': EXTENT_HOME,
    'Reach::Face': EXTENT_HOME,
    'lever_from_within': EXTENT_HOME,
    'lever_from_point_and_plane': EXTENT_HOME,
    'of_crossing_cylinders': EXTENT_HOME,
    'hinge_lever': EXTENT_HOME,
    'turn_lever': EXTENT_HOME,
    'ruled_lever': EXTENT_HOME,
    'generator_lever': EXTENT_HOME,
}
FREE = [d for d in DOORS if '::' not in d]
TYPES = r'(?:Lever|ExtentBall|Reach)'
GENERICS = r'<(?:[^<>]|<[^<>]*>)*>'
word = re.compile(r'(?<![\w:])(?:\w+::)*(' + '|'.join(map(re.escape, DOORS)) + r')\b')
# A door's own declaration is not a reference to it.
declared = re.compile(r'\bfn\s+(\w+)')
turbofish = re.compile(r'\b(' + TYPES + r')::' + GENERICS + '::')
qualified = re.compile(r'<\s*(?:\w+::)*(' + TYPES + r')\s*(?:' + GENERICS + r')?\s*>::')
alias = re.compile(
    r'\b(?:' + TYPES + '|' + '|'.join(FREE) + r')\s+as\s+\w'
    r'|\btype\s+\w+[^=;]*=\s*(?:\w+::)*' + TYPES + r'\b'
)
# An impl block's header is a statement of its own, and starts it.
impl = re.compile(r'^\s*(?:unsafe\s+)?impl\b[^{;(]*\b' + TYPES + r'\s*<')
trait_impl = re.compile(r'^\s*(?:unsafe\s+)?impl\b[^{;(]*\bfor\s+(?:\w+::)*(?:Lever|ExtentBall)\s*<')


def normalised(code):
    code = re.sub(r'\s*::\s*', '::', code)
    return qualified.sub(r'\1::', turbofish.sub(r'\1::', code))


def option(name):
    return sys.argv[sys.argv.index(name) + 1] if name in sys.argv else None


out = []
pins, origins, bad, dup = pin_ledger.read_pins(sys.argv[1], DOORS)
out += [f'MALFORMED {b}' for b in bad] + [f'DUPLICATE {d}' for d in dup]
lines_for = option('--lines')
counts = {}
for rec in sys.stdin:
    path, ln, code = rec.rstrip('\n').split(':', 2)
    code = normalised(code)
    own = set(declared.findall(code))
    for m in word.finditer(code):
        door = m.group(1)
        if DOORS[door] == path or door.split('::')[-1] in own:
            continue
        counts[(door, path)] = counts.get((door, path), 0) + 1
        if lines_for in (path, 'ALL'):
            out.append(f'SITE {path}:{ln} {door}')
    if path.startswith('crates/geom-core/src/') and path != LEVER_HOME:
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

out += pin_ledger.moved(pins, counts)
base = option('--base')
if base:
    out += pin_ledger.raised(pins, origins, base, option('--renames'), DOORS)

print('\n'.join([*out, f'TOTAL {sum(counts.values())}']))
