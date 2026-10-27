#!/usr/bin/env python3
"""Collect per-PR metrics and item closures from first-parent merges on origin/main."""
import subprocess, re, json, sys, os
from concurrent.futures import ThreadPoolExecutor

REPO = '/home/user/cad'
OUT = os.path.dirname(os.path.abspath(__file__))
SINCE = sys.argv[1] if len(sys.argv) > 1 else '2026-09-20'

def git(*a, inp=None):
    return subprocess.run(['git', '-C', REPO, *a], capture_output=True, text=True, input=inp).stdout

ITEM_RE = re.compile(r'^work/([^/]+)/([^/]+)\.md$')
SKIP = {'log', 'program', 'README', 'STATUS'}

def header(text):
    if not text.startswith('---'):
        return None
    end = text.find('\n---', 3)
    if end < 0:
        return None
    h = {}
    for line in text[3:end].splitlines():
        m = re.match(r'^([a-z_]+):\s*(.*?)\s*(#.*)?$', line)
        if m:
            h[m.group(1)] = m.group(2)
    return h

def show(sha, path):
    r = subprocess.run(['git', '-C', REPO, 'show', f'{sha}:{path}'], capture_output=True, text=True)
    return r.stdout if r.returncode == 0 else None

merges = []
for line in git('log', 'origin/main', '--first-parent', '--merges', f'--since={SINCE}',
                '--format=%H%x09%P%x09%cI%x09%s').splitlines():
    sha, parents, date, subj = line.split('\t', 3)
    m = re.match(r'Merge pull request #(\d+) from evgunter/(\S+)', subj)
    if not m:
        continue
    p = parents.split()
    if len(p) < 2:
        continue
    merges.append(dict(sha=sha, p1=p[0], p2=p[1], merged=date, pr=int(m.group(1)), branch=m.group(2)))

def per_pr(mg):
    sha, p1, p2 = mg['sha'], mg['p1'], mg['p2']
    commits = []
    for l in git('log', '--no-merges', '--format=%at%x09%ct%x09%an', f'{p1}..{p2}').splitlines():
        at, ct, an = l.split('\t')
        commits.append((int(at), int(ct), an))
    nmerge = len(git('rev-list', '--merges', f'{p1}..{p2}').split())
    files = []
    for l in git('diff', '--numstat', '--no-renames', p1, sha).splitlines():
        a, d, path = l.split('\t', 2)
        a = 0 if a == '-' else int(a); d = 0 if d == '-' else int(d)
        files.append((path, a, d))
    # item closures in this diff
    work = [f[0] for f in files if ITEM_RE.match(f[0]) and ITEM_RE.match(f[0]).group(2) not in SKIP]
    before, after = {}, {}
    for path in work:
        prog, iid = ITEM_RE.match(path).groups()
        b = show(p1, path); a = show(sha, path)
        if b:
            hb = header(b)
            if hb: before[iid] = (prog, hb)
        if a:
            ha = header(a)
            if ha: after[iid] = (prog, ha, a)
    items = []
    for iid, (prog, ha, body) in after.items():
        hb = before.get(iid, (None, None))[1]
        bstat = hb.get('status') if hb else None
        items.append(dict(id=iid, program=prog, status_after=ha.get('status'), status_before=bstat,
                          cost=ha.get('cost'), cost_before=hb.get('cost') if hb else None,
                          kind=ha.get('kind'), opened=ha.get('opened'), closed=ha.get('closed'),
                          pr_hdr=ha.get('pr'), branch_hdr=ha.get('branch'), parent=ha.get('parent'),
                          refs=ha.get('refs'), priority=ha.get('priority'), design=ha.get('design'),
                          new=hb is None, body_len=len(body)))
    mg = dict(mg)
    mg.update(commits=commits, n_merge_commits=nmerge, files=files, items=items)
    return mg

with ThreadPoolExecutor(16) as ex:
    res = list(ex.map(per_pr, merges))
json.dump(res, open(os.path.join(OUT, 'prs.json'), 'w'))
print(len(res))
