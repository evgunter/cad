"""Unblind the four raters' grades and ranks against the correspondence key."""
import json, sys, random, statistics as st
S = sys.argv[1]
UNITS = {'u6972': 'PR-A', 'box-region': 'PR-B', 'anchored-conic': 'PR-C'}
RATERS = {0: 'Opus', 1: 'Sonnet', 2: 'Opus', 3: 'Sonnet'}
G = {'MAJOR': 3, 'MINOR': 2, 'NOTE': 1, 'STYLE': 0}
key = json.load(open(f'{S}/key.json'))
master = json.load(open(f'{S}/master_key.json'))
krow = {(u, r['row']): r for u, rs in key.items() for r in rs}
rat = {k: json.load(open(f'{S}/rater{k}/ratings.json')) for k in RATERS}

items = []
for oid, m in master.items():
    u, row = m['unit'], m['row']
    k = krow[(u, row)]
    pr = UNITS[u]
    grades, pcts, acts = {}, {}, {}
    for r in RATERS:
        R = rat[r][pr]
        grades[r] = G[R['grades'][oid]]
        n = len(R['rank'])
        pcts[r] = R['rank'].index(oid) / (n - 1)       # 0 = most severe, 1 = least
        acts[r] = R['act'][oid] == 'yes'
    src = {'Fable': 'Fable only', 'Opus': 'Opus only', 'both': 'both'}[k['src_model']]
    items.append(dict(id=oid, unit=u, row=row, src=src,
                      lab_f=k['label_fable'] if src != 'Opus only' else None,
                      lab_o=k['label_opus'] if src != 'Fable only' else None,
                      grade=st.mean(grades.values()), grade_opus_r=st.mean([grades[r] for r in RATERS if RATERS[r] == 'Opus']),
                      grade_sonnet_r=st.mean([grades[r] for r in RATERS if RATERS[r] == 'Sonnet']),
                      pct=st.mean(pcts.values()), act=st.mean(acts.values()), grades=grades, pcts=pcts))

out = {}
SRC = ['Fable only', 'both', 'Opus only']
def summ(xs):
    return dict(n=len(xs), mean=round(st.mean(xs), 3) if xs else None)
for s in SRC:
    I = [i for i in items if i['src'] == s]
    out[s] = dict(n=len(I),
                  mean_grade=round(st.mean(i['grade'] for i in I), 2),
                  mean_grade_by_opus_raters=round(st.mean(i['grade_opus_r'] for i in I), 2),
                  mean_grade_by_sonnet_raters=round(st.mean(i['grade_sonnet_r'] for i in I), 2),
                  mean_rank_pct=round(st.mean(i['pct'] for i in I), 3),
                  share_bottom_third=round(sum(i['pct'] > 2 / 3 for i in I) / len(I), 3),
                  share_top_third=round(sum(i['pct'] < 1 / 3 for i in I) / len(I), 3),
                  act_share=round(st.mean(i['act'] for i in I), 3),
                  consensus_grade_counts={g: sum(1 for i in I if round(i['grade']) == v) for g, v in G.items()},
                  per_unit={u: len([i for i in I if i['unit'] == u]) for u in UNITS})

# permutation test within unit: Fable-only vs Opus-only mean rank pct
def perm(metric):
    rng = random.Random(7)
    def diff(lab):
        f = [i[metric] for i, l in zip(items, lab) if l == 'Fable only']
        o = [i[metric] for i, l in zip(items, lab) if l == 'Opus only']
        return st.mean(f) - st.mean(o)
    obs = diff([i['src'] for i in items])
    cnt = 0; N = 20000
    for _ in range(N):
        lab = [i['src'] for i in items]
        for u in UNITS:
            idx = [j for j, i in enumerate(items) if i['unit'] == u and i['src'] != 'both']
            vals = [lab[j] for j in idx]; rng.shuffle(vals)
            for j, v in zip(idx, vals): lab[j] = v
        if abs(diff(lab)) >= abs(obs) - 1e-12: cnt += 1
    return dict(observed_fable_minus_opus=round(obs, 3), p_two_sided=round(cnt / N, 4))
out['test: rank pct, Fable-only minus Opus-only (within-unit permutation)'] = perm('pct')
out['test: grade, Fable-only minus Opus-only (within-unit permutation)'] = perm('grade')

# harshness: the reviewer's own label vs the blind consensus grade, on the items it raised with a label
harsh = {}
for side, lk in (('Fable', 'lab_f'), ('Opus', 'lab_o')):
    for scope in ('unilateral', 'bilateral', 'all'):
        I = [i for i in items if i[lk] in G and
             (scope == 'all' or (scope == 'bilateral') == (i['src'] == 'both'))]
        d = [G[i[lk]] - i['grade'] for i in I]
        harsh[f'{side} reviewer, {scope}'] = dict(n=len(I), own_label_mean=round(st.mean(G[i[lk]] for i in I), 2) if I else None,
                                                  blind_grade_mean=round(st.mean(i['grade'] for i in I), 2) if I else None,
                                                  inflation=round(st.mean(d), 2) if d else None)
out['harshness (own label minus blind consensus, grade points; MAJOR=3 .. STYLE=0)'] = harsh

# bilateral items labelled by both: who was nearer the blind grade
bl = [i for i in items if i['src'] == 'both' and i['lab_f'] in G and i['lab_o'] in G]
out['bilateral, labelled by both'] = dict(
    n=len(bl),
    fable_higher=sum(G[i['lab_f']] > G[i['lab_o']] for i in bl),
    opus_higher=sum(G[i['lab_o']] > G[i['lab_f']] for i in bl),
    same=sum(G[i['lab_o']] == G[i['lab_f']] for i in bl),
    fable_abs_err=round(st.mean(abs(G[i['lab_f']] - i['grade']) for i in bl), 2),
    opus_abs_err=round(st.mean(abs(G[i['lab_o']] - i['grade']) for i in bl), 2),
    fable_signed=round(st.mean(G[i['lab_f']] - i['grade'] for i in bl), 2),
    opus_signed=round(st.mean(G[i['lab_o']] - i['grade'] for i in bl), 2))

# rater agreement: Spearman of rank orders between raters, per unit
def spearman(a, b):
    n = len(a); ra = {x: i for i, x in enumerate(a)}; rb = {x: i for i, x in enumerate(b)}
    return 1 - 6 * sum((ra[x] - rb[x]) ** 2 for x in a) / (n * (n * n - 1))
agree = {}
for u, pr in UNITS.items():
    for a in RATERS:
        for b in RATERS:
            if a < b:
                agree[f'{pr} r{a}({RATERS[a][0]}) vs r{b}({RATERS[b][0]})'] = round(spearman(rat[a][pr]['rank'], rat[b][pr]['rank']), 2)
out['rater agreement (Spearman on ranks)'] = agree
out['grade agreement: all four raters within one grade'] = round(
    sum(max(i['grades'].values()) - min(i['grades'].values()) <= 1 for i in items) / len(items), 3)

# the bottom of each unit's consensus order
tails = {}
for u, pr in UNITS.items():
    I = sorted([i for i in items if i['unit'] == u], key=lambda i: i['pct'])
    k3 = len(I) // 3
    tails[pr] = dict(n=len(I), top_third=[i['src'] for i in I[:k3]], bottom_third=[i['src'] for i in I[-k3:]])
out['tails by unit'] = tails
json.dump(dict(summary=out, items=items), open(f'{S}/sev_results.json', 'w'), indent=1)
print(json.dumps(out, indent=1))
