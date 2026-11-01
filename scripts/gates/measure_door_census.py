"""The measurement doors' test-only names, counted per crate.

Read by `raw-measure-door-sites.sh` (the raw doors). Reads `gate_rust_code`'s
`path:line:code` records on stdin (comments and string literals already
stripped) and counts every REFERENCE to one of the names: a call, a
turbofish, a fn pointer, a `(path)(…)` call and a `use … as` rename
all spell the name once, so none of them is a way around the count. The
names' own declarations (`topo::test_support`'s `pub use … as NAME;`)
are not references.

    measure_door_census.py BASELINE NAME... [--reasons]

Prints `ROSE key: was -> now` / `FELL key: was -> now` per key that
moved, then `TOTAL n`. With `--reasons`, also prints `UNREASONED
path:line` for each reference with no `// Raw door:` comment on its own
line or above it in the same statement (read from the file itself, since
the records carry no comments). The walk up from a reference passes
comment-only lines and the lines of its own statement, and stops at the
first line that ends another one: a blank line, a `fn` or an attribute,
code ending in `;`, `{` or `}`, or a match arm ending in `,`. So a
reason covers the statement it heads, and never a later statement, a
later arm or another fn.
"""
import re
import sys

args = [a for a in sys.argv[1:] if not a.startswith('--')]
reasons = '--reasons' in sys.argv
baseline, names = args[0], args[1:]
word = re.compile(r'\b(' + '|'.join(map(re.escape, names)) + r')\b')
decl = re.compile(r'\bas\s+(' + '|'.join(map(re.escape, names)) + r')\s*;')

base = {}
with open(baseline, encoding='utf-8') as fh:
    for line in fh:
        line = line.split('#', 1)[0].strip()
        if line:
            key, n = line.split()
            base[key] = int(n)


def key_of(path):
    parts = path.split('/')
    if parts[0] in ('crates', 'demos') and len(parts) > 2:
        return '/'.join(parts[:2])
    return parts[0]


counts = {}
refs = []
for rec in sys.stdin:
    path, ln, code = rec.rstrip('\n').split(':', 2)
    if path == 'crates/topo/src/lib.rs' and decl.search(code):
        continue
    hits = len(word.findall(code))
    if hits:
        counts[key_of(path)] = counts.get(key_of(path), 0) + hits
        refs.append((path, int(ln)))

for key in sorted(set(counts) | set(base)):
    now, was = counts.get(key, 0), base.get(key, 0)
    if now > was:
        print(f'ROSE {key}: {was} -> {now}')
    elif now < was:
        print(f'FELL {key}: {was} -> {now}')

if reasons:
    cache = {}
    stop = re.compile(r'^\s*($|(pub(\([^)]*\))?\s+)?((const|async|unsafe|extern\s+"[^"]*")\s+)*fn\s|#!?\[)')

    def ends_a_statement(line):
        code = line.split('//', 1)[0].rstrip()
        if not code:
            return False
        return code[-1] in ';{}' or (code[-1] == ',' and '=>' in code)

    for path, ln in refs:
        lines = cache.setdefault(path, open(path, encoding='utf-8').read().split('\n'))
        ok = '// Raw door:' in lines[ln - 1]
        k = ln - 2
        while not ok and k >= 0 and not stop.match(lines[k]):
            if '// Raw door:' in lines[k]:
                ok = True
            elif ends_a_statement(lines[k]):
                break
            k -= 1
        if not ok:
            print(f'UNREASONED {path}:{ln}')

print(f'TOTAL {sum(counts.values())}')
