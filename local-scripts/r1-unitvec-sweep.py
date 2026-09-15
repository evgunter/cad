#!/usr/bin/env python3
"""R1's differently-shaped sweep: any `fn` (signature possibly spanning
lines) with a Vec3/Dual/SpanBox/[T;3] parameter, whose preceding doc
block (up to 20 lines, not just 8) says unit|normalized|normalised|
direction|orthonormal, non-test sources. Also struct fields with such
docs. Prints file:line fn-name and the doc word matched."""
import os, re, sys
ROOT = sys.argv[1]
WORD = re.compile(r'\b(unit|normali[sz]ed|orthonormal|direction)\b', re.I)
FN = re.compile(r'^\s*(pub(\([^)]*\))?\s+)?fn\s+([A-Za-z_][A-Za-z0-9_]*)')
FIELD = re.compile(r'^\s*(pub(\([^)]*\))?\s+)?([a-z_][a-z0-9_]*)\s*:\s*(Vec3|SpanBox|\[T; 3\]|\[f64; 3\])')
PARAM = re.compile(r'\b(Vec3|SpanBox)\s*<')
hits = []
for dp, dn, fns in os.walk(ROOT):
    if any(s in dp for s in ('/target', '/tests', '/examples', '/benches', '/.git')):
        continue
    for f in fns:
        if not f.endswith('.rs'):
            continue
        p = os.path.join(dp, f)
        lines = open(p, encoding='utf-8', errors='replace').read().split('\n')
        # find #[cfg(test)] tail and stop there
        end = len(lines)
        for i, l in enumerate(lines):
            if l.strip().startswith('#[cfg(test)]'):
                end = i
                break
        for i in range(end):
            m = FN.match(lines[i])
            if m:
                # gather signature up to '{' or ';'
                sig = ''
                j = i
                while j < end and j < i + 12:
                    sig += lines[j]
                    if '{' in lines[j] or lines[j].rstrip().endswith(';'):
                        break
                    j += 1
                if not PARAM.search(sig):
                    continue
                # doc block above, up to 20 lines
                doc = []
                k = i - 1
                while k >= 0 and k >= i - 20 and (lines[k].strip().startswith('///') or lines[k].strip().startswith('#[')):
                    doc.append(lines[k])
                    k -= 1
                # also param names hinting unit: dir, normal, axis, n, u_ref
                text = '\n'.join(doc)
                w = WORD.search(text)
                pname = re.search(r'\b(dir|normal|axis|unit|u_ref|d|n)\s*:\s*Vec3', sig)
                if w or pname:
                    hits.append((p, i + 1, m.group(3), (w.group(1) if w else '-') + ('/' + pname.group(1) if pname else ''), 'multiline' if j > i else ''))
            fm = FIELD.match(lines[i])
            if fm:
                doc = []
                k = i - 1
                while k >= 0 and k >= i - 12 and lines[k].strip().startswith('///'):
                    doc.append(lines[k]); k -= 1
                w = WORD.search('\n'.join(doc))
                if w:
                    hits.append((p, i + 1, 'FIELD ' + fm.group(3), w.group(1), ''))
for h in sorted(hits):
    print(f"{h[0]}:{h[1]} {h[2]} [{h[3]}] {h[4]}")
print(len(hits), "hits", file=sys.stderr)
