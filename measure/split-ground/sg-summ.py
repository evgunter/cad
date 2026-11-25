"""embedwalk-splitground: summarize CAD_SPLIT_GROUND logs (ASK/CODE/PAIR/N rows)."""
import collections, glob, sys, re

D = sys.argv[1]
asks, codes, pairs, nodes = {}, collections.defaultdict(dict), collections.defaultdict(dict), collections.defaultdict(dict)
for f in glob.glob(D + '/*.log'):
    for l in open(f, errors='replace'):
        p = l.rstrip('\n').split('\t')
        if p[0] == 'ASK':
            asks[p[1]] = dict(test=p[2], door=p[3], verdict=p[4], cand=int(p[5]), splits=int(p[6]), secs=float(p[7]),
                              rnode=int(p[8]), rdepth=int(p[9]), rk=int(p[10]), rfaces=p[11], rwhat=p[12],
                              rt=float(p[13]) if p[13] != 'NaN' else float('nan'), nnodes=int(p[14]))
        elif p[0] == 'CODE':
            codes[p[1]][int(p[2])] = p[3]
        elif p[0] == 'PAIR':
            pairs[p[1]][int(p[2])] = (p[3], p[4], p[5])
        elif p[0] == 'N':
            nodes[p[1]][int(p[2])] = dict(parent=int(p[3]), depth=int(p[4]), k=int(p[5]), reason=int(p[6]),
                                          secs=float(p[7]), stop=p[8], lo=float(p[9]), hi=float(p[10]))

ref = {a: v for a, v in asks.items() if v['verdict'].startswith('Refused')}
print(f"asks {len(asks)}: refusing {len(ref)}, crossing {sum(v['verdict']=='Crosses' for v in asks.values())}, embedded {sum(v['verdict']=='Embedded' for v in asks.values())}")
print('refusing verdicts', collections.Counter(v['verdict'] for v in ref.values()).most_common())
print('refusing doors', collections.Counter(v['door'] for v in ref.values()).most_common())
tests = collections.Counter(v['test'] for v in ref.values())
print('tests with a refusing ask', len(tests))


def cls(code_s):
    """reason class without site/shared."""
    if code_s is None:
        return 'leaf'
    head = code_s.split('#')[0]
    return head


def coarse(code_s):
    if code_s is None:
        return 'leaf'
    h = code_s.split('#')[0]
    if h.startswith('distance:'):
        return h
    # touch:Departure:<kind>:states=...
    parts = h.split(':', 2)
    rest = parts[2] if len(parts) > 2 else ''
    if rest.startswith('both-presented'):
        m = rest.split(':')[1]
        return 'touch:both-presented:' + m
    if rest.startswith('unpresented'):
        st = rest.split(':', 1)[1].split('|')
        tags = sorted(s.split(':', 1)[1].split('@')[0] for s in st)
        return 'touch:unpresented:' + '+'.join(tags)
    return 'touch:' + parts[1] + ':' + rest.split(':')[0].split('=')[0]


def strict(code_s):
    """a reason no smaller cell changes, read off the code: a Structural
    part, or a margin wholly in band / decided zero."""
    if code_s is None:
        return False
    return 'Structural' in code_s or 'wholly-in-band' in code_s or 'decided-zero' in code_s or '@13945' in code_s


def strict_touch(code_s):
    return code_s is not None and code_s.startswith('touch:') and strict(code_s)


rows = []
by_reason = collections.Counter()
same_parent = collections.Counter()
straddle = collections.Counter()
straddle_spend = collections.Counter()
for a, v in sorted(ref.items(), key=lambda kv: -kv[1]['secs']):
    N, C = nodes[a], codes[a]
    rs = lambda n: C.get(N[n]['reason']) if N[n]['reason'] else None
    split_nodes = [n for n in N if N[n]['reason'] and N[n]['stop'] == '' and any(True for _ in [0])]
    # a node with a reason and no stop split (children exist)
    has_child = collections.Counter(N[n]['parent'] for n in N)
    splits = [n for n in N if has_child[n]]
    for n in splits:
        c = rs(n)
        by_reason[coarse(c)] += 1
        par = N[n]['parent']
        if par and has_child[par]:
            pc = rs(par)
            same_parent[(coarse(c), 'same-as-parent' if c == pc else ('same-class' if coarse(c) == coarse(pc) else 'changed'))] += 1
        else:
            same_parent[(coarse(c), 'root')] += 1
    # nearest touching ancestor per node
    def touch_anc(n):
        g = 0
        p = N[n]['parent']
        while p:
            g += 1
            c = rs(p)
            if c and c.startswith('touch:'):
                return g, coarse(c)
            p = N[p]['parent']
        return None, None
    for n in splits:
        c = rs(n)
        if c and c.startswith('distance:'):
            g, tc = touch_anc(n)
            share = c.split(':')[2]
            key = (share, 'no-touching-ancestor' if g is None else f'touching-ancestor', N[n]['k'] == v['rk'])
            straddle[key] += 1
            straddle_spend[key] += N[n]['secs']
    # per refusing face pair (the one refusing pair)
    rk = v['rk']
    mine = [n for n in N if N[n]['k'] == rk]
    pair_splits = sum(1 for n in mine if has_child[n])
    pair_secs = sum(N[n]['secs'] for n in mine)
    walk_secs = sum(N[n]['secs'] for n in N)
    d_strict = min((N[n]['depth'] for n in mine if has_child[n] and strict(rs(n))), default=None)
    # repeat depth: below it every split of this pair repeats its parent's reason exactly
    changed = [N[n]['depth'] for n in mine if has_child[n] and N[n]['parent'] and has_child[N[n]['parent']] and rs(n) != rs(N[n]['parent'])]
    d_rep = (max(changed) + 1) if changed else 1
    after = lambda d: (sum(1 for n in mine if has_child[n] and N[n]['depth'] > d),
                       sum(N[n]['secs'] for n in mine if N[n]['depth'] > d))
    rn = v['rnode']
    rows.append(dict(ask=a, test=v['test'], verdict=v['verdict'], secs=v['secs'], walk=walk_secs, splits=v['splits'],
                     rdepth=v['rdepth'], rt=v['rt'], pair=pairs[a].get(rk, ('?', '?', '?')), pair_splits=pair_splits,
                     pair_secs=pair_secs, d_strict=d_strict, strict_after=after(d_strict) if d_strict is not None else None,
                     d_rep=d_rep, rep_after=after(d_rep), rwhat=v['rwhat'],
                     rreason=coarse(rs(rn)) if rn in N and N[rn]['reason'] else 'none', rsite=(rs(rn) or '').split('#')[0],
                     rstop=N[rn]['stop'] if rn in N else ''))

print('\n== splits of refusing asks by reason (coarse)')
tot = sum(by_reason.values())
for k, n in by_reason.most_common(40):
    print(f'{n:9d} {100*n/tot:5.1f}%  {k}')
print('total', tot)
print('\n== child vs parent (coarse reason, relation)')
for k, n in sorted(same_parent.items(), key=lambda kv: -kv[1])[:40]:
    print(f'{n:9d}  {k}')
print('\n== straddling distance splits: (faces share, lineage, on the refusing pair) -> splits, secs')
for k, n in straddle.most_common():
    print(f'{n:9d} {straddle_spend[k]:9.2f}s  {k}')

print('\n== per refusing ask (dearest first)')
print('secs\twalk\tsplits\tpair_splits\tpair_secs\trdepth\tt_refusal/secs\td_strict\tafter_strict\td_rep\tafter_rep\tshare\trefusal\treason_at_refusal\tstop\ttest')
for r in rows:
    frac = r['rt'] / r['secs'] if r['secs'] else float('nan')
    print(f"{r['secs']:.3f}\t{r['walk']:.3f}\t{r['splits']}\t{r['pair_splits']}\t{r['pair_secs']:.3f}\t{r['rdepth']}\t{frac:.3f}\t{r['d_strict']}\t{r['strict_after']}\t{r['d_rep']}\t{r['rep_after'][0]}:{r['rep_after'][1]:.3f}\t{r['pair'][2]}\t{r['rwhat'][:60]}\t{r['rreason']}\t{r['rstop']}\t{r['test'][:70]}")

print('\n== straddling distance splits on refusing asks: generations since the nearest touching split, and hi by depth')
gap = collections.Counter(); hi_by = collections.defaultdict(list); sib = collections.Counter()
for a, v in ref.items():
    N, C = nodes[a], codes[a]
    has_child = collections.Counter(N[n]['parent'] for n in N)
    rs = lambda n: C.get(N[n]['reason']) if N[n]['reason'] else None
    kids = collections.defaultdict(list)
    for n in N: kids[N[n]['parent']].append(n)
    for n in N:
        c = rs(n)
        if not (has_child[n] and c and c.startswith('distance:')): continue
        g = 0; p = N[n]['parent']; found = None
        while p:
            g += 1
            pc = rs(p)
            if pc and pc.startswith('touch:'):
                found = g; break
            p = N[p]['parent']
        share = c.split(':')[2]
        gap[(share, found if found is None or found <= 3 else '>3')] += 1
        # does a sibling (same parent) itself split touching?
        par = N[n]['parent']
        sibt = any(s != n and rs(s) and rs(s).startswith('touch:') and has_child[s] for s in kids[par])
        sib[(share, 'a sibling splits touching' if sibt else 'no sibling splits touching')] += 1
        d = N[n]['depth']
        hi_by[(share, (d // 5) * 5)].append(N[n]['hi'])
for k, n in sorted(gap.items(), key=lambda kv: -kv[1]): print(f'{n:8d}  generations-since-touching {k}')
for k, n in sib.most_common(): print(f'{n:8d}  {k}')
import statistics
for k in sorted(hi_by):
    xs = hi_by[k]
    print(f'{k[0]:15s} depth {k[1]:2d}-{k[1]+4:2d}: n={len(xs):6d} median hi {statistics.median(xs):.2e} max hi {max(xs):.2e}')

print('\n== touching splits by site pair (Local lines), refusing asks')
site = collections.Counter()
for a, v in ref.items():
    N, C = nodes[a], codes[a]
    has_child = collections.Counter(N[n]['parent'] for n in N)
    for n in N:
        if has_child[n] and N[n]['reason']:
            c = C[N[n]['reason']]
            if c.startswith('touch:'):
                site[c.split('#')[0].split(':', 2)[2]] += 1
for k, n in site.most_common(25): print(f'{n:8d}  {k}')

print('\n== the refusing node at the floor: its reason with sites')
fl = collections.Counter(r['rsite'] for r in rows if r['rstop'] == 'depth')
for k, n in fl.most_common(15): print(f'{n:6d}  {k}')
print('\n== touching splits on refusing pairs at depth >= 30, by site of either side')
deep = collections.Counter()
for a, v in ref.items():
    N, C = nodes[a], codes[a]
    has_child = collections.Counter(N[n]['parent'] for n in N)
    for n in N:
        if has_child[n] and N[n]['reason'] and N[n]['k'] == v['rk'] and N[n]['depth'] >= 30:
            c = C[N[n]['reason']].split('#')[0]
            for side in re.findall(r'Local@(\d+)|(P-\w+)|(Structural)', c):
                deep[next(x for x in side if x)] += 1
for k, n in deep.most_common(20): print(f'{n:6d}  {k}')

print('\n== a reason no smaller cell changes (Structural part, margin wholly in band or decided zero, a plane corner lean decided reflex or undecided): first depth, and what follows it')
def fixed(c):
    return c is not None and c.startswith('touch:') and (strict(c) or '@23945' in c)
agg = collections.Counter(); per = []
for a, v in ref.items():
    N, C = nodes[a], codes[a]
    has_child = collections.Counter(N[n]['parent'] for n in N)
    rs = lambda n: C.get(N[n]['reason']) if N[n]['reason'] else None
    fx = [N[n]['depth'] for n in N if N[n]['reason'] and fixed(rs(n))]
    fxp = [N[n]['depth'] for n in N if N[n]['reason'] and N[n]['k'] == v['rk'] and fixed(rs(n))]
    tot_s = sum(1 for n in N if has_child[n]); tot_t = sum(N[n]['secs'] for n in N)
    if not fx:
        agg['asks with none'] += 1; agg['splits in asks with none'] += tot_s; continue
    d = min(fx)
    agg['asks with one'] += 1
    agg['splits'] += tot_s; agg['secs'] += tot_t
    agg['splits after'] += sum(1 for n in N if has_child[n] and N[n]['depth'] > d)
    agg['secs after'] += sum(N[n]['secs'] for n in N if N[n]['depth'] > d)
    per.append((d, min(fxp) if fxp else None, v['rdepth']))
    agg['on the refusing pair'] += bool(fxp)
for k, n in agg.items(): print(f'  {k}: {n:.3f}' if isinstance(n, float) else f'  {k}: {n}')
print('  first depth (ask), first depth (refusing pair), refusal depth:', collections.Counter(per).most_common(12))

print('\n== the refusing floor node: does its own lineage carry a fixed reason, and from what depth?')
lin = collections.Counter()
for a, v in ref.items():
    N, C = nodes[a], codes[a]
    rs = lambda n: C.get(N[n]['reason']) if N[n]['reason'] else None
    n = v['rnode']
    if n not in N or N[n]['stop'] != 'depth': continue
    ds = []; p = n
    while p:
        if fixed(rs(p)): ds.append(N[p]['depth'])
        p = N[p]['parent']
    chain = []; p = n
    while p:
        chain.append(coarse(rs(p))); p = N[p]['parent']
    lin[('fixed in lineage from depth %s' % min(ds)) if ds else 'no fixed reason in lineage'] += 1
for k, n in sorted(lin.items(), key=lambda kv: -kv[1]): print(f'{n:5d}  {k}')
