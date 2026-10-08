import sys, json, random
from oracle_a import *
from adv import est_flip
seed, ci, pi, eps = sys.argv[1], int(sys.argv[2]), int(sys.argv[3]), float(sys.argv[4])
m = json.load(open(f'meta_adv_{seed}.json'))[ci]
u, D, tag = m['P'][pi]
print('kind', m['kind'], 'hollow', m['hollow'], tag)
for j, s in enumerate(m['secs']):
    print(j, 'face', s['face'], 's', [f'{x:.6g}' for x in s['s']], 'e', [f'{x:.6g}' for x in s['e']], 'Ls %.3g Le %.3g' % (flen(s['fs']), flen(s['fe'])), 'h(D)=%.3e' % fdot(D, s['n']))
orc = OracleA(m['secs']); t = orc.cls(D); print('truth', t, 'est_flip/eps', est_flip(m['secs'], D)/eps)
for lab, s2, D2 in moves(m['secs'], D, eps):
    t2 = (orc if s2 is m['secs'] else OracleA(s2)).cls(D2)
    if t2 != t: print('  move', lab, '->', t2)
