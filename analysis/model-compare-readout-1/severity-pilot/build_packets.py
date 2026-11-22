import json,random,sys,os
S=sys.argv[1]
UNITS={'u6972':'PR-A','box-region':'PR-B','anchored-conic':'PR-C'}
DROP={('u6972','1')}  # class row; its instances 1a/1b are rated
key=json.load(open(f'{S}/key.json'))
master={}  # opaque id -> (unit,row)
rng=random.Random(20261122)
pool=list(range(100,1000)); rng.shuffle(pool)
units={}
for u,name in UNITS.items():
    nj=json.load(open(f'{S}/{u}/neutral.json'))
    krows={r['row'] for r in key[u]}
    items=[]
    for it in nj['items']:
        row=str(it['row'])
        if (u,row) in DROP: continue
        assert row in krows,(u,row)
        oid=f'F{pool.pop()}'
        master[oid]=dict(unit=u,row=row)
        items.append(dict(id=oid,title=it['title'],where=it['where'],what=it['what'],evidence=it['evidence'],consequence=it['consequence']))
    missing=krows-{str(i['row']) for i in nj['items']}-{r for (uu,r) in DROP if uu==u}
    assert not missing,(u,missing)
    units[name]=dict(context=nj['context'],items=items)
json.dump(master,open(f'{S}/master_key.json','w'),indent=1)
for k in range(4):
    r=random.Random(1000+k)
    pk={}
    names=list(units); r.shuffle(names)
    for n in names:
        its=list(units[n]['items']); r.shuffle(its)
        pk[n]=dict(context=units[n]['context'],findings=its)
    os.makedirs(f'{S}/rater{k}',exist_ok=True)
    json.dump(pk,open(f'{S}/rater{k}/packet.json','w'),indent=1)
print({n:len(units[n]['items']) for n in units})
