import sys,collections,re
def load(files,ctxre):
    rows=[];inv=collections.Counter();deff=[]
    for f in files:
        for l in open(f,errors='replace'):
            p=l.rstrip('\n').split('\t')
            if ctxre and len(p)>1 and not re.search(ctxre,p[1]): continue
            if p[0]=='INVALID': inv[(p[2],p[3])]+=1
            elif p[0]=='DEF': deff.append(p)
            elif p[0]=='ASK2' and len(p)>=13:
                cells={}
                for c in p[10].split(';'):
                    q=c.split('|')
                    if len(q)==6: cells[q[0]]=dict(cls=q[1],lo=q[3],hi=q[4])
                rows.append(dict(ctx=p[1],pred=p[2],today=p[3],vlo=float(p[4]),vhi=float(p[5]),bz=float(p[6]),be=float(p[7]),cells=cells,members=p[11],p=p[12]))
    return rows,inv,deff
def best(r):
    for st in ('47','15'):
        c=r['cells'].get(st)
        if c and c['cls'] in('Positive','Negative','Zero'): return c
    return None
def main(ctxre,files):
    rows,inv,deff=load(files,ctxre)
    und=[r for r in rows if r['today']=='undecided']
    print('==',ctxre,'ASK2 rows',len(rows),'undecided',len(und))
    print('  INVALID margins (never reach the tier):',sum(inv.values()),dict(collections.Counter({k:v for k,v in inv.items()}).most_common(8)))
    for name,grp in (('undecided today, reading decides',und),('ladder/value Zero today (sampled)',[r for r in rows if r['today']!='undecided'])):
        tot=collections.Counter(); byp=collections.defaultdict(collections.Counter); term=0; leaves=set(); tp=collections.Counter(); ex={}
        for r in grp:
            b=best(r)
            if not b or not r['p']: continue
            q=r['p'].split('|')
            if q[0].startswith('declines'): k='P declines'
            elif q[0]==b['cls']: k='P agrees'
            else:
                k='P straddles/differs'
                w=float(q[2])-float(q[1]); lo=float(b['lo']); hi=float(b['hi'])
                if b['cls']=='Positive': t= r['be']-w<=lo and hi<=r['be']+w
                elif b['cls']=='Negative': t= -r['be']-w<=lo and hi<=-r['be']+w
                else: t=(r['bz']-w<=lo and hi<=r['bz']+w) or (-r['bz']-w<=lo and hi<=-r['bz']+w)
                if t: term+=1; leaves.add(r['ctx']); tp[r['pred']]+=1
                ex.setdefault(r['pred'],(r['ctx'],'reading',b['cls'],b['lo'],b['hi'],'P',q,'band',r['bz'],r['be']))
            tot[k]+=1; byp[r['pred']][k]+=1
        print('  --',name,': with a point reading',sum(tot.values()),dict(tot),'| held back, terminal:',term,'in',len(leaves),'leaves',dict(tp))
        for pred,k in sorted(byp.items(),key=lambda kv:-sum(kv[1].values()))[:10]:
            print('       %-32s %s'%(pred,dict(k)))
        for pred,v in list(ex.items())[:6]: print('       e.g. differs',pred,v)
    # containment
    n=out=na=0; outs=[]
    for p in deff:
        ms=p[5].split(',')
        if any(m.startswith('NA') for m in ms): na+=1; continue
        n+=1
        if any(m.startswith('OUT') for m in ms): out+=1; outs.append((p[1],p[2],p[3],p[4],p[5]))
    print('  value-signed asks sampled: %d (x5 members); with a member outside the value enclosure: %d; not evaluable %d'%(n,out,na))
    oc=collections.Counter(o[1] for o in outs); print('     by predicate',dict(oc))
    seen=set()
    for o in outs:
        if o[1] in seen: continue
        seen.add(o[1]); print('     OUT',o)
    n=out=0; outs=[]
    for r in rows:
        if not r['members']: continue
        ms=r['members'].split(',')
        if any(m.startswith('NA') for m in ms): continue
        n+=1
        vals=[float(m.split(':')[1]) for m in ms]
        if any(v<r['vlo'] or v>r['vhi'] for v in vals): out+=1; outs.append((r['ctx'],r['pred'],r['vlo'],r['vhi'],vals))
    print('  unsigned asks with members: %d; with a member outside the value enclosure: %d'%(n,out), dict(collections.Counter(o[1] for o in outs)))
    seen=set()
    for o in outs:
        if o[1] in seen: continue
        seen.add(o[1]); print('     OUT',o)
main(sys.argv[1],sys.argv[2:])
