#!/usr/bin/env python3
"""Join closed items to their PRs and compute per-item effort proxies.

Inputs (same dir): prs.json (collect.py), ghp_*.json (GitHub REST PR pages),
logs/*.md (last version of every work/<program>/log.md), and the repo's
docs/dual-review-log/rows/PR-*.md.
Outputs: items.csv (one row per closed priced item), prs.csv, summary.json.
"""
import json, glob, os, re, csv, statistics as st, collections, datetime as dt, subprocess

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = '/home/user/cad'
REDEF = '2026-09-27'
THRESHOLDS = [30, 60, 90, 120, 240]   # minutes; gap above = pause
PRIMARY_T = 90

prs = {p['pr']: p for p in json.load(open(os.path.join(HERE, 'prs.json')))}
gh = {}
for f in glob.glob(os.path.join(HERE, 'ghp_*.json')):
    for r in json.load(open(f)):
        gh[r['n']] = r

def ts(s):
    return dt.datetime.fromisoformat(s.replace('Z', '+00:00')).timestamp()

def kind_of(path):
    if path == 'work/STATUS.md':
        return 'skip'
    if path.startswith('work/'):
        return 'work'
    ext = os.path.splitext(path)[1]
    if ext in ('.rs', '.py', '.pyi', '.sh', '.yml', '.toml'):
        return 'code'
    if ext == '.md':
        return 'prose'
    return 'data'   # goldens, fixtures, svg, csv, ...

# ---- review evidence from program logs -------------------------------------
PRREF = re.compile(r'(?:\bPR\s*#?|#)(\d{4})\b')
entries = []   # (program, text)
for f in glob.glob(os.path.join(HERE, 'logs', '*.md')):
    prog = os.path.basename(f)[:-3]
    cur = []
    for line in open(f, encoding='utf-8', errors='replace'):
        if re.match(r'^(- |\* |#+ )', line) and cur:
            entries.append((prog, ''.join(cur))); cur = []
        cur.append(line)
    if cur:
        entries.append((prog, ''.join(cur)))

FIND = re.compile(r'\*\*\(?(?:MAJOR|MINOR|NIT|NOTE|M\d|N\d)|\b(?:MAJOR|MINOR)\s*\d|\bMAJOR\b|\bMINOR\b')
TIER_RANK = {'none': 0, 'read': 1, 'style': 2, 'full': 3, 'dual': 4}
branch_pr = {p['branch']: n for n, p in prs.items() if '/' in p['branch'] and not p['branch'].startswith('claude/')}
BR = re.compile(r'[a-z0-9]+/[a-z0-9][a-z0-9./_-]+')
rev = collections.defaultdict(lambda: dict(log_entries=0, review_entries=0, review_dispatches=0, fix_passes=0,
                                           findings=0, log_words=0, review_tier='none'))
for prog, text in entries:
    nums = set(int(n) for n in PRREF.findall(text))
    nums = {n for n in nums if n in prs}
    for b in BR.findall(text):
        b = b.rstrip('.,);`')
        if b in branch_pr:
            nums.add(branch_pr[b])
    if len(nums) != 1:
        continue
    n = nums.pop()
    r = rev[n]
    r['log_entries'] += 1
    r['log_words'] += len(text.split())
    low = text.lower()
    if re.search(r'review(ed)?\b[^\n]{0,60}(approve|mergeable|merge\b|changes|fix-first|fixes|verdict)', low):
        r['review_entries'] += 1
    if 'dispatch' in low and 'review' in low:
        r['review_dispatches'] += 1
    r['fix_passes'] += len(re.findall(r'fix pass', low))
    r['findings'] += len(FIND.findall(text))
    t = 'none'
    if re.search(r"orchestrator'?s read|no review lane", low): t = 'read'
    if re.search(r'style review|\bstyle\b tier|tier:? style', low): t = 'style'
    if re.search(r'full review|tier:? full', low): t = 'full'
    if re.search(r'dual review|dual tier|tier:? dual|\bdr-?\d', low): t = 'dual'
    if TIER_RANK[t] > TIER_RANK[r['review_tier']]:
        r['review_tier'] = t

# dual-review rows: MAJOR/MINOR/NOTE triples
dual = {}
for f in glob.glob(os.path.join(REPO, 'docs/dual-review-log/rows/PR-*.md')):
    n = int(re.search(r'PR-(\d+)', f).group(1))
    t = open(f).read()
    trip = re.findall(r'(?:APPROVE|CHANGES|MERGE|FIX|REQUEST|BLOCK|REJECT|SOUND|NOT)[-A-Z ]*,?\s*(\d+)/(\d+)/(\d+)(?:\s*\(\+(\d+) style\))?', t)
    tot = sum(int(a) + int(b) + int(c) + int(s or 0) for a, b, c, s in trip[:2])
    fp = len(re.findall(r'fix pass', t.lower()))
    dual[n] = dict(dual_findings=tot, dual_fixpass_mentions=fp, dual_letter=(re.search(r'^letter:\s*(\S+)', t, re.M) or [None, None])[1])

# ---- per PR metrics -----------------------------------------------------------
def active_minutes(times, T):
    times = sorted(times)
    return sum(min(b - a, 10**9) for a, b in zip(times, times[1:]) if b - a <= T * 60) / 60

SESS = json.load(open(os.path.join(HERE, 'sessions.json')))
prrow = {}
for n, p in prs.items():
    times = [c[0] for c in p['commits']]
    lines = collections.Counter()
    for path, a, d in p['files']:
        lines[kind_of(path)] += a + d
    g = gh.get(n, {})
    created = ts(g['c']) if g.get('c') else None
    merged = ts(p['merged'])
    # timeline: commits + PR open + merge (merge closes the last gap if short)
    tl = times + ([created] if created else []) + [merged]
    row = dict(pr=n, branch=p['branch'], merged=p['merged'][:10], n_commits=len(times),
               n_main_merges=p['n_merge_commits'],
               lines_code=lines['code'], lines_prose=lines['prose'], lines_work=lines['work'], lines_data=lines['data'],
               files=len(p['files']),
               raw_elapsed_h=round((merged - min(tl)) / 3600, 2),
               open_to_merge_h=round((merged - created) / 3600, 2) if created else None)
    for T in THRESHOLDS:
        row[f'active_min_T{T}'] = round(active_minutes(tl, T), 1)
        row[f'commit_active_min_T{T}'] = round(active_minutes(times, T), 1)
    row.update(rev.get(n, dict(log_entries=0, review_entries=0, review_dispatches=0, fix_passes=0, findings=0, log_words=0, review_tier='none')))
    row['sessions'] = SESS.get(str(n))
    row.update(dual.get(n, dict(dual_findings=None, dual_fixpass_mentions=None, dual_letter=None)))
    prrow[n] = row

# ---- item closures ----------------------------------------------------------
closures = {}
for n, p in sorted(prs.items(), key=lambda kv: kv[1]['merged']):
    for it in p['items']:
        if it['status_after'] == 'closed' and it['status_before'] != 'closed':
            it = dict(it, closing_pr=n, closing_branch=p['branch'], closed_at=p['merged'][:10])
            closures[(it['program'], it['id'])] = it   # last closure wins (reopen/close)

# all items ever created per (pr) and parent/refs for follow-up counting
created_in = collections.defaultdict(list)   # pr -> item ids newly added
all_items = {}
for n, p in prs.items():
    for it in p['items']:
        all_items[it['id']] = it
        if it['new'] and it['status_after'] != 'closed':
            created_in[n].append(it['id'])
# latest header of every item seen in the window; children = items naming it as parent or in refs
latest = {}
for p in sorted(prs.values(), key=lambda p: p['merged']):
    for it in p['items']:
        latest[it['id']] = it
parent_of = collections.defaultdict(set)
for iid, it in latest.items():
    tg = set()
    if it.get('parent'):
        tg.add(it['parent'].strip())
    tg |= set(re.findall(r'[A-Za-z0-9][\w-]+', it.get('refs') or ''))
    for t in tg - {iid}:
        parent_of[t].add(iid)
WCHILD = dict(E=1, M=2.5, H=5, D=2.5)

def impl_pr(it):
    h = (it.get('pr_hdr') or '').strip().strip('[]').split(',')[0].strip()
    if h.isdigit() and int(h) in prs:
        return int(h), 'pr_header'
    b = (it.get('branch_hdr') or '').strip()
    for n, p in prs.items():
        if b and p['branch'] == b:
            return n, 'branch_header'
    if not it['closing_branch'].startswith('claude/'):
        return it['closing_pr'], 'closing_pr'
    return None, None

rows = []
for (prog, iid), it in closures.items():
    cost = it['cost']
    if cost not in ('E', 'M', 'H', 'D'):
        continue
    n, how = impl_pr(it)
    rows.append(dict(item=iid, program=prog, kind=it['kind'], cost=cost, cost_before=it['cost_before'],
                     repriced=bool(it['cost_before'] and it['cost_before'] != cost),
                     priority=it['priority'], design=it['design'], opened=it['opened'], closed=it['closed_at'],
                     post_redef=(it['opened'] or '') >= REDEF, impl_pr=n, join=how,
                     closing_pr=it['closing_pr'], status_before=it['status_before']))

# bundling: items sharing an implementing PR
bundle = collections.Counter(r['impl_pr'] for r in rows if r['impl_pr'])
wsum = collections.Counter()
W = dict(E=1, M=2.5, H=5, D=2.5)
for r in rows:
    if r['impl_pr']:
        wsum[r['impl_pr']] += W[r['cost']]
for r in rows:
    n = r['impl_pr']
    r['bundle_size'] = bundle[n] if n else None
    if n:
        p = prrow[n]
        r['impl_branch'] = p['branch']
        share = W[r['cost']] / wsum[n]   # weight-proportional share of a bundled PR
        r['share'] = round(share, 3)
        for k, v in p.items():
            if k in ('pr', 'branch', 'merged', 'dual_letter', 'review_tier'):
                continue
            r['pr_' + k] = v
        r['followups_parent_refs'] = len(parent_of.get(r['item'], ()))
        r['followup_points'] = sum(WCHILD.get(latest[c].get('cost'), 2.5) for c in parent_of.get(r['item'], ()))
        r['followups_new_in_pr'] = len(created_in.get(n, ()))
        r['dual_letter'] = p['dual_letter']
        r['review_tier'] = p['review_tier']

fields = sorted({k for r in rows for k in r}, key=lambda k: (not k[0].isalpha(), k))
lead = ['item', 'program', 'kind', 'cost', 'cost_before', 'repriced', 'priority', 'design', 'opened', 'closed',
        'post_redef', 'impl_pr', 'impl_branch', 'join', 'closing_pr', 'status_before', 'bundle_size', 'share']
fields = lead + [f for f in fields if f not in lead]
with open(os.path.join(HERE, 'items.csv'), 'w', newline='') as f:
    w = csv.DictWriter(f, fields); w.writeheader(); w.writerows(rows)
with open(os.path.join(HERE, 'prs.csv'), 'w', newline='') as f:
    pf = list(next(iter(prrow.values())).keys())
    w = csv.DictWriter(f, pf); w.writeheader(); w.writerows(prrow.values())
print(len(rows), 'closed priced items;', sum(1 for r in rows if r['impl_pr']), 'joined to a PR')
print(collections.Counter(r['join'] for r in rows))
