import sys,collections,re
STEPS=[('0','shipped walk (Sqrt/Abs/Min/Max, depth 8)'),('1','+ Sin/Cos/Tan/Asin/Acos/Atan/Atan2'),('3','+ Select'),('7','+ Hull'),('15','+ depth 32'),('47','+ frozen nodes opened (op over children)'),('111','[alt, sampled] + node values')]
def norm(c): return re.sub(r'@\d+','',c)
def load(files,ctxre=None):
    rows=[]
    for f in files:
        for l in open(f,errors='replace'):
            p=l.rstrip('\n').split('\t')
            if p[0]!="ASK2" or len(p)<12: continue
            if ctxre and not re.search(ctxre,p[1]): continue
            cells={}
            for c in p[10].split(';'):
                q=c.split('|')
                if len(q)==6: cells[q[0]]=dict(cls=q[1],form=q[2],lo=q[3],hi=q[4],ns=int(q[5]))
            rows.append(dict(ctx=p[1],pred=p[2],today=p[3],vlo=p[4],vhi=p[5],bz=p[6],be=p[7],plain_ns=int(p[8]),ladder_ns=int(p[9]),cells=cells,members=p[11]))
    return rows
def kind(c):
    if c in('Positive','Negative'): return 'sign'
    if c=='Zero': return 'zero-in-band'
    if c in('sliver','straddles','EMPTY'): return c
    return 'declines'
def main(ctxre,files):
    rows=load(files,ctxre)
    und=[r for r in rows if r['today']=='undecided']
    print('== %s: asks %d; today %s'%(ctxre,len(rows),dict(collections.Counter(r['today'] for r in rows))))
    if und:
        print('   undecided today: %d; plain build %.1f us/ask; ladder after it %.2f ms/ask (%.1f s total)'%(len(und),sum(r['plain_ns'] for r in und)/1e3/len(und),sum(r['ladder_ns'] for r in und)/1e6/len(und),sum(r['ladder_ns'] for r in und)/1e9))
    for st,name in STEPS:
        g=[r for r in und if st in r['cells']]
        if not g: continue
        k=collections.Counter(kind(r['cells'][st]['cls']) for r in g)
        why=collections.Counter(norm(r['cells'][st]['cls']) for r in g if kind(r['cells'][st]['cls'])=='declines')
        ns=sum(r['cells'][st]['ns'] for r in g)/len(g)/1e3
        print('  step %-2s %-45s sign %6d  zero %5d  sliver %4d  straddles %6d  declines %6d  EMPTY %d  | %.1f us/ask'%(st,name,k['sign'],k['zero-in-band'],k['sliver'],k['straddles'],k['declines'],k['EMPTY'],ns))
        print('          declines by reason:',dict(why.most_common(9)))
    # ladder-zero agreement
    lz=[r for r in rows if r['today']!='undecided']
    k=collections.Counter(norm(r['cells']['15']['cls']) if '15' in r['cells'] else 'no-cell' for r in lz)
    print('  asks today proves/reads Zero (%d), reading at step 15:'%len(lz),dict(k.most_common(8)))
    emp=[(r,st) for r in rows for st,c in r['cells'].items() if c['cls']=='EMPTY']
    print('  EMPTY intersections:',len(emp))
    for r,st in emp[:25]: print('     ',r['ctx'],r['pred'],r['today'],'value [%s,%s]'%(r['vlo'],r['vhi']),'step',st,'form [%s,%s]'%(r['cells'][st]['lo'],r['cells'][st]['hi']),r['cells'][st]['form'])
    print('  by predicate at step 15 (undecided today):')
    bp=collections.defaultdict(list)
    for r in und: bp[r['pred']].append(r)
    for pred,g in sorted(bp.items(),key=lambda kv:-len(kv[1]))[:12]:
        k0=collections.Counter(kind(r['cells']['0']['cls']) for r in g); k=collections.Counter(kind(r['cells']['15']['cls']) for r in g)
        why=collections.Counter(norm(r['cells']['15']['cls']) for r in g if kind(r['cells']['15']['cls'])=='declines')
        print('     %-32s %6d  %s  %s'%(pred,len(g),dict(k.most_common()),dict(why.most_common(3))))
    # item 1
    ind=collections.Counter(); rep={}
    for r in und:
        c=norm(r['cells']['0']['cls'])
        if 'indet' in c: ind[c]+=1; rep.setdefault(c,r['pred'])
    print('  step-0 indet declines by kind (best of plain/early):',[(c,n,rep[c]) for c,n in ind.most_common(12)])
    # soundness
    n=bad=na=0; diffs=collections.Counter(); ex={}
    for r in rows:
        if not r['members'] or '15' not in r['cells']: continue
        want=r['cells']['15']['cls']
        ms=r['members'].split(',')
        if want not in('Positive','Negative','Zero'):
            continue
        if any(m.startswith('NA') for m in ms): na+=1; continue
        n+=1
        if any(m.split(':')[0]!=want for m in ms):
            bad+=1; diffs[(r['pred'],want)]+=1; ex.setdefault((r['pred'],want),(r['ctx'],r['bz'],r['be'],r['cells']['15']['lo'],r['cells']['15']['hi'],r['members']))
    print('  soundness: %d sampled asks x5 members (reading decides at step 15); differing %d; not evaluable %d'%(n,bad,na))
    for k,v in diffs.items(): print('     DIFF',k,v,ex[k])
    # straddles with members
    st=collections.Counter(r['pred'] for r in und if r['cells'].get('15',{}).get('cls')=='straddles')
    print('  still straddling at step 15, by predicate:',dict(st.most_common(8)))
main(sys.argv[1],sys.argv[2:])

def extra(ctxre,files,best='47'):
    rows=load(files,ctxre)
    und=[r for r in rows if r['today']=='undecided']
    print('  by predicate at step',best)
    bp=collections.defaultdict(list)
    for r in und: bp[r['pred']].append(r)
    for pred,g in sorted(bp.items(),key=lambda kv:-len(kv[1]))[:12]:
        k=collections.Counter(kind(r['cells'][best]['cls']) for r in g if best in r['cells'])
        why=collections.Counter(norm(r['cells'][best]['cls']) for r in g if best in r['cells'] and kind(r['cells'][best]['cls'])=='declines')
        print('     %-32s %6d  %s  %s'%(pred,len(g),dict(k.most_common()),dict(why.most_common(3))))
    print('  straddling at step',best,'with sampled members:')
    sp=collections.defaultdict(lambda: collections.Counter()); ex={}
    for r in und:
        c=r['cells'].get(best)
        if not c or c['cls']!='straddles' or not r['members']: continue
        ms=r['members'].split(',')
        if any(m.startswith('NA') for m in ms): sp[r['pred']]['members not evaluable']+=1; continue
        signs=set(m.split(':')[0] for m in ms)
        vals=[float(m.split(':')[1]) for m in ms]
        key='members all '+signs.pop() if len(signs)==1 else 'members mixed '+'/'.join(sorted(signs))
        sp[r['pred']][key]+=1
        ex.setdefault((r['pred'],key),(r['ctx'],'band',r['bz'],r['be'],'form',c['lo'],c['hi'],'value',r['vlo'],r['vhi'],'members %.3e..%.3e'%(min(vals),max(vals))))
    for pred,k in sorted(sp.items(),key=lambda kv:-sum(kv[1].values()))[:8]:
        print('     %-30s %s'%(pred,dict(k)))
        for (p2,key),v in ex.items():
            if p2==pred: print('        e.g.',key,v)
    # soundness at best
    n=bad=na=0; diffs=collections.Counter(); exd={}
    for r in rows:
        c=r['cells'].get(best) or r['cells'].get('15')
        if not r['members'] or not c or c['cls'] not in('Positive','Negative','Zero'): continue
        ms=r['members'].split(',')
        if any(m.startswith('NA') for m in ms): na+=1; continue
        n+=1
        if any(m.split(':')[0]!=c['cls'] for m in ms):
            bad+=1; diffs[(r['pred'],c['cls'])]+=1; exd.setdefault((r['pred'],c['cls']),(r['ctx'],r['bz'],r['be'],c['lo'],c['hi'],r['members']))
    print('  soundness at step %s: %d sampled x5 members; differing %d; not evaluable %d'%(best,n,bad,na))
    for k,v in diffs.items(): print('     DIFF',k,v,exd[k])
if len(sys.argv)>2: extra(sys.argv[1],sys.argv[2:])
