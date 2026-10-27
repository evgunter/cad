#!/usr/bin/env python3
"""Distinct Claude-Session trailers and authors per PR branch -> sessions.json."""
import json, subprocess, re, os
from concurrent.futures import ThreadPoolExecutor
HERE = os.path.dirname(os.path.abspath(__file__))
prs = json.load(open(os.path.join(HERE, 'prs.json')))
def one(p):
    out = subprocess.run(['git', '-C', '/home/user/cad', 'log', '--no-merges', '--format=%B%x00', f"{p['p1']}..{p['p2']}"],
                         capture_output=True, text=True).stdout
    s = set(re.findall(r'Claude-Session: \S*?(session_\w+)', out))
    return p['pr'], len(s)
with ThreadPoolExecutor(16) as ex:
    res = dict(ex.map(one, prs))
json.dump(res, open(os.path.join(HERE, 'sessions.json'), 'w'))
import collections; print(collections.Counter(res.values()).most_common(10))
