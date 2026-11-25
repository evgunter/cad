"""embedwalk-splitground: the row's tables from the summary's per-ask rows."""
import json, os, sys, collections, statistics as st, re
S = os.path.dirname(os.path.abspath(__file__))
exec(open(S + '/sg-summ.py').read().split("print('\\n== splits of refusing asks")[0].replace("print(", "(lambda *a, **k: None)("))
pop = json.load(open(S + '/population.json'))
dear = {t for _, t in pop['dear']}
grp = lambda t: 'dear' if t in dear else 'floor'
def rclass(r):
    w = r['rwhat']
    m = re.search(r'\[(.*)\]$', w); why = m.group(1) if m else ''
    if w.startswith('Uncertified') and why.startswith('stuck-both'):
        return 'Uncertified/stuck-both (' + '+'.join(sorted(s.split(':')[1] for s in why.split(':',1)[1].split('|'))) + ')'
    if w.startswith('Uncertified') and r['rstop'] == 'depth':
        return 'Uncertified at the depth floor (' + r['rreason'] + ')'
    if w.startswith('Budget') and 'Depth' in w:
        return 'Budget(Depth) at the depth floor (' + r['rreason'] + ')'
    return w.split(' ')[0] + ' ' + why[:40]
T = collections.defaultdict(lambda: [0, 0.0, 0, [], 0.0, []])
for r in rows:
    k = (grp(r['test']), rclass(r))
    t = T[k]; t[0] += 1; t[1] += r['secs']; t[2] += r['splits']; t[3].append(r['rdepth']); t[4] += r['walk']; t[5].append(r['rt'] / r['secs'] if r['secs'] else 1)
print('group\trefusal\tasks\tsecs\twalk secs\tsplits\tmedian refusal depth\tmin share of time before refusal')
for k, t in sorted(T.items(), key=lambda kv: (kv[0][0], -kv[1][1])):
    print(f"{k[0]}\t{k[1]}\t{t[0]}\t{t[1]:.2f}\t{t[4]:.2f}\t{t[2]}\t{st.median(t[3])}\t{min(t[5]):.3f}")
print()
print('group\tasks\tsplits\tsecs\tpair splits after d_rep\tsecs after d_rep\tmedian d_rep')
for g in ('dear', 'floor'):
    R = [r for r in rows if grp(r['test']) == g]
    print(f"{g}\t{len(R)}\t{sum(r['splits'] for r in R)}\t{sum(r['secs'] for r in R):.2f}\t{sum(r['rep_after'][0] for r in R)}\t{sum(r['rep_after'][1] for r in R):.3f}\t{st.median(r['d_rep'] for r in R) if R else '-'}")
print()
print('tests per group', collections.Counter(grp(v['test']) for v in ref.values()), 'distinct tests', len({v['test'] for v in ref.values()}))
allt = collections.defaultdict(float)
for v in asks.values(): allt[grp(v['test'])] += v['secs']
print('all gate asks secs by group', dict(allt))
