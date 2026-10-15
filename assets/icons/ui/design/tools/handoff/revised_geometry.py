# Original geometry revisions; executed inside create_complete.py before export.
# Square-profile sweep around a modest S, with a visible dotted path.
def sweep_geometry():
 def point(t):
  u=1-t
  return (u**3*14+3*u*u*t*39+3*u*t*t*27+t**3*53,u**3*47+3*u*u*t*50+3*u*t*t*17+t**3*18)
 def rail(t,sign,depth=False):
  dt=.0001;a=point(t-dt);b=point(t+dt);dx=b[0]-a[0];dy=b[1]-a[1];n=math.hypot(dx,dy);x,y=point(t)
  return (x-sign*6.8*dy/n+(-6 if depth else 0),y+sign*6.8*dx/n+(-5 if depth else 0))
 ts=[j/60 for j in range(61)]
 upper=[rail(t,-1) for t in ts];lower=[rail(t,1) for t in ts];back=[rail(t,-1,True) for t in ts]
 out=[p(poly(back+list(reversed(upper))),B[0]),p(poly(upper+list(reversed(lower))),B[1]),p(poly([rail(0,-1),rail(0,1),rail(0,1,True),rail(0,-1,True)]),G[1])]
 track=[point(t) for t in [-.12]+ts+[1.12]]
 out.append(l(track,'white',3.2,{'stroke-dasharray':'0.1 5.2'}))
 out.append(l(track,B[2],1.8,{'stroke-dasharray':'0.1 5.2'}))
 return out
D['sweep']=sweep_geometry()
# Exactly the fillet's dimensions, projection and corner endpoints, with a flat bevel.
def matching_chamfer():
 P=m['P'];W=Dd=28;H=26;Rr=16;curve=[(W,H-Rr),(W-Rr,H)]
 top=[P(0,0,H),P(W-Rr,0,H),P(W-Rr,Dd,H),P(0,Dd,H)]
 side=[P(W,0,0),P(W,Dd,0),P(W,Dd,H-Rr),P(W,0,H-Rr)]
 ribbon=[P(x,0,z) for x,z in curve]+[P(x,Dd,z) for x,z in reversed(curve)]
 cap=[P(0,0,0),P(W,0,0)]+[P(x,0,z) for x,z in curve]+[P(0,0,H)]
 out=[p(poly(top),G[0],None),p(poly(side),G[2],None),p(poly(ribbon),B[1],None),p(poly(cap),G[1],None)]
 outer=[P(0,0,0),P(W,0,0),P(W,Dd,0),P(W,Dd,H-Rr),P(W-Rr,Dd,H),P(0,Dd,H),P(0,0,H)]
 out += [p(poly(outer)),l([P(x,0,z) for x,z in curve]),l([P(W,0,0),P(W,0,H-Rr)]),l([P(W,0,H-Rr),P(W,Dd,H-Rr)]),l([P(W-Rr,0,H),P(W-Rr,Dd,H)]),l([P(0,0,H),P(W-Rr,0,H)])]
 return out
D['chamfer']=matching_chamfer()
# Two offset cubes meet along part of a face; the union has a stepped peanut silhouette.
def combined_cubes():
 def Q(x,y,z):return (5+.85*math.sqrt(3)/2*(x+y),40+.85*(.5*x-.5*y-z))
 footprint=[(0,0),(20,0),(20,12),(40,12),(40,32),(20,32),(20,20),(0,20)]
 out=[]
 for a,b in [((0,0),(20,0)),((20,0),(20,12)),((20,12),(40,12)),((40,12),(40,32))]:
  tone=B[1] if a[1]==b[1] else B[2]
  out.append(p(poly([Q(*a,24),Q(*b,24),Q(*b,0),Q(*a,0)]),tone))
 out.append(p(poly([Q(x,y,24) for x,y in footprint]),B[0]))
 return out
D['combine']=combined_cubes()
# Both cubes share exactly the bottom-front-left vertex (10, 43).
def scale_cubes():
 pts=[(32,6),(8,20),(56,20),(8,46),(32,60),(56,46)]
 def anchored(s):return [(10+(x-8)*s,43+(y-46)*s) for x,y in pts]
 back,left,right,bl,bf,br=anchored(.95);ft=(left[0]+right[0]-back[0],left[1]+right[1]-back[1])
 ghost=[p(poly([left,back,right,br,bf,bl]),stroke=B[2],attrs={'stroke-dasharray':'2.4 4.2'})]
 ghost.extend(l([ft,z],B[2],E,{'stroke-dasharray':'2.4 4.2'}) for z in [left,right,bf])
 return cube(*anchored(.43),G,E)+ghost+arrow(33,32,15,-14)
D['scale']=scale_cubes()
# A drawn mathematical f(x), independent of installed fonts.
D['params']=[p('M6 56 C11 61 17 59 18.5 50 L24 14 C25.5 5 31.5 4 36 9',stroke=I,w=4.2),p('M14 24 H28',stroke=I,w=4.2),p('M37 26 C30 29 26 36 26 42 C26 50 31 56 37 58',stroke=I,w=4.2),p('M49 26 C56 29 60 36 60 42 C60 50 55 56 49 58',stroke=I,w=4.2),p('M37 38 L48 49 M48 38 L37 49',stroke=B[2],w=4.2)]
def short_block():return cube((32,26),(8,40),(56,40),(8,47),(32,61),(56,47),G,E)
def dotted_plane(points):return p(poly(points),B[0],B[2],E,{'stroke-dasharray':'0.1 5','stroke-linecap':'round'})
D['plane']=short_block()+[dotted_plane([(8,28),(32,14),(56,28),(32,42)])]
# Hinge is exactly the grey block's top-right edge: (32,54) -> (56,40).
D['plane_angle']=short_block()+[dotted_plane([(12,21),(36,7),(56,40),(32,54)])]
D['measure']=[p('M8 43 L44 7 L57 20 L21 56 Z',G[0]),l([(14,41),(20,47)]),l([(23,32),(27,36)]),l([(32,23),(38,29)]),l([(41,14),(45,18)]),l([(5,40),(41,4)],B[2],2.55,{'stroke-dasharray':'0.1 5.1'})]
# Thin crescent, using a real cutout so it works on any background.
moon_defs=el('defs');moon_mask=el('mask',{'id':'thin-crescent','maskUnits':'userSpaceOnUse','x':'0','y':'0','width':'64','height':'64'})
moon_mask.append(rect(0,0,64,64,'white',None));moon_mask.append(disk(39,25,25,'black',None));moon_defs.append(moon_mask)
moon=disk(32,32,24,I,None);moon.set('mask','url(#thin-crescent)');flat['moon']=[moon_defs,moon]
flat['home']=[p('M7 30 L32 11 L57 30 H49 V52 H37 V39 H27 V52 H15 V30 Z',I,I,1.2)]
flat['pan']=[p('M32 5 L42 18 H36 V28 H46 V22 L59 32 L46 42 V36 H36 V46 H42 L32 59 L22 46 H28 V36 H18 V42 L5 32 L18 22 V28 H28 V18 H22 Z',I,I,1.2)]
# Flattened rotation arrows around the vertical axis; each crossing is punched out behind its front line.
def spin_geometry():
 def point(t):
  a=math.radians(t);return(32+23*math.cos(a),37+9*math.sin(a))
 def arc(lo,hi):return [point(lo+(hi-lo)*i/50) for i in range(51)]
 back=arc(195,325);front=arc(15,145)
 defs=el('defs')
 for name,d in [('spin-back', 'M32 9 V58'),('spin-axis','M'+' L'.join(f'{x} {y}' for x,y in front))]:
  mask=el('mask',{'id':name,'maskUnits':'userSpaceOnUse','x':'0','y':'0','width':'64','height':'64'})
  mask.append(rect(0,0,64,64,'white',None));mask.append(p(d,stroke='black',w=10));defs.append(mask)
 a=l(back,I,5.6);a.set('mask','url(#spin-back)');axis=p('M32 9 V58',stroke=I,w=5.6);axis.set('mask','url(#spin-axis)')
 out=[defs,a,axis,l(front,I,5.6)]
 for theta in [350,170]:
  tip=point(theta);base=point(theta-25);dx,dy=tip[0]-base[0],tip[1]-base[1];z=math.hypot(dx,dy);nx,ny=-dy/z,dx/z
  out.append(p(poly([tip,(base[0]+4*nx,base[1]+4*ny),(base[0]-4*nx,base[1]-4*ny)]),I,I,1.2))
 return out
flat['spin']=spin_geometry()
# Front arrow with a broad head; the rear rectangle is truly punched out.
def look_at_geometry():
 head='M25 32 L39 23 L39 41 Z';shaft='M39 32 H58'
 defs=el('defs');mask=el('mask',{'id':'look-at-rear','maskUnits':'userSpaceOnUse','x':'0','y':'0','width':'64','height':'64'})
 mask.append(rect(0,0,64,64,'white',None));mask.append(p(head,'black','black',5));mask.append(p(shaft,stroke='black',w=10));defs.append(mask)
 rear=rect(10,13,31,39,'none',I,5.6,3);rear.set('mask','url(#look-at-rear)')
 return [defs,rear,p(shaft,stroke=I,w=5.6),p(head,I,I,1.2)]
flat['lookat']=look_at_geometry()
# Increase the weight of the whole navigation family, with subtly rounded outer corners.
navkeys=['orbit','pan','fit','lookat','home','perspective','style']
for key in navkeys:
 for node in flat[key]:
  for n in node.iter():
   if n.get('stroke')==I and float(n.get('stroke-width','0'))>=4:n.set('stroke-width','5.6')
   if n.get('fill')==I and n.tag.endswith('path') and not n.get('stroke'):
    n.set('stroke',I);n.set('stroke-width','1.2');n.set('stroke-linejoin','round')
   if n.get('stroke'):n.set('stroke-linejoin','round');n.set('stroke-linecap','round')
