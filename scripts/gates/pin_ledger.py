"""A ledger of pins that only fall: `KIND COUNT FILE` lines, one per (kind, file).

Read by `cover_kind_match.py` and `lever_doors.py`, so both gates read a
pin, follow a moved file and credit a split file the same way.

A PIN FOLLOWS A MOVE. `renames` lists `old new` path pairs (git's own
rename detection against the merge base): a base pin on `old` is held as
one on `new`.

A SPLIT FILE CITES ITS ORIGIN: `# from OLD_PATH` at the end of a line.
Every line that cites one origin is summed WITH that origin's own line,
and the sum is held to the origin's base pin. So a citation moves a pin
between files and never raises the total: a new site that cites an
unchanged file is red.
"""
import re


def read_pins(path, kinds=None):
    """`(pins, origins, malformed, duplicate)`; pins are `{(kind, file): count}`."""
    pins, origins, bad, dup = {}, {}, [], []
    with open(path, encoding='utf-8') as fh:
        for n, raw in enumerate(fh, 1):
            code, _, note = raw.partition('#')
            line = code.strip()
            if not line:
                continue
            parts = line.split()
            known = kinds is None or (len(parts) == 3 and parts[0] in kinds)
            if len(parts) != 3 or not known or not parts[1].isdigit():
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


def by_file(key):
    return (key[1], key[0])


def moved(pins, counts):
    """`ROSE`/`FELL` lines: the counted sites against their pins."""
    out = []
    for key in sorted(set(counts) | set(pins), key=by_file):
        now, was = counts.get(key, 0), pins.get(key, 0)
        if now > was:
            out.append(f'ROSE {key[0]} {key[1]}: {was} -> {now}')
        elif now < was:
            out.append(f'FELL {key[0]} {key[1]}: {was} -> {now}')
    return out


def raised(pins, origins, base_file, renames_file=None, kinds=None):
    """`RAISED` lines: the pins, summed per origin, against the base's,
    summed the same way, so a split the base already carries is no rise."""
    base_pins, base_origins, _, _ = read_pins(base_file, kinds)
    base = {}
    for key, pin in base_pins.items():
        target = (key[0], base_origins.get(key, key[1]))
        base[target] = base.get(target, 0) + pin
    if renames_file:
        with open(renames_file, encoding='utf-8') as fh:
            went = dict(line.split()[:2] for line in fh if len(line.split()) >= 2)
        base = {(k, went.get(p, p)): n for (k, p), n in base.items()}
    held = {}
    for key, pin in pins.items():
        target = (key[0], origins.get(key, key[1]))
        held[target] = held.get(target, 0) + pin
    return [
        f'RAISED {target[0]} {target[1]}: {base.get(target, 0)} -> {total}'
        for target, total in sorted(held.items(), key=lambda kv: by_file(kv[0]))
        if total > base.get(target, 0)
    ]
