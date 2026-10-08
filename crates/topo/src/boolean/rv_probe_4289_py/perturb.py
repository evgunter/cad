import sys, json, random, math, copy
sys.path.insert(0, '.')
from gen import *
from cases import write_case
src, seed, K, amp = sys.argv[1], int(sys.argv[2]), int(sys.argv[3]), float(sys.argv[4])
random.seed(seed)
meta = json.load(open(f'meta_{src}.json'))
def jit(v, a):
    while True:
        r = [random.uniform(-1, 1) for _ in range(3)]
        if dot(r, r) <= 1: return [x + a*y for x, y in zip(v, r)]
groups = []
with open(f'cases_p{seed}.txt', 'w') as f:
    cid = 0
    for gi, m in enumerate(meta):
        for k in range(K):
            secs = copy.deepcopy(m['secs'])
            if k:
                # perturb shared far points consistently: key by rounded unit vector
                cache = {}
                def pf(far):
                    key = tuple(round(x, 12) for x in far)
                    if key not in cache: cache[key] = jit(far, amp)
                    return cache[key]
                bis = {}
                for s in secs:
                    for b, fk, ek in (('s', 'fs', 'es'), ('e', 'fe', 'ee')):
                        if s[ek]:
                            s[fk] = pf(s[fk]); s[b] = norm(s[fk])
                        else:
                            key = tuple(round(x, 12) for x in s[b])
                            if key not in bis: bis[key] = norm(jit(s[b], amp / s['arm']))
                            s[b] = bis[key]
                    s['n'] = norm(jit(s['n'], amp / 2))
            P = []
            for u, far, tag in m['P']:
                if k:
                    far2 = jit(far, amp); P.append((norm(far2), far2, tag))
                else:
                    P.append((u, far, tag))
            write_case(f, cid, secs, P); cid += 1
        groups.append(m['name'])
json.dump(dict(K=K, names=groups), open(f'pgroups_{seed}.json', 'w'))
