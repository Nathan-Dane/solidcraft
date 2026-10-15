from lib import *
ICONS={}
def icon(fn): ICONS[fn.__name__]=fn; return fn
def pt(ic,x,y,c=A3,r=3.07): ic.dot(x,y,r,c)
def arrowhead(x,y,dx,dy,L=9,W=8):
    n=math.hypot(dx,dy); dx,dy=dx/n,dy/n
    bx,by=x-dx*L,y-dy*L; px,py=-dy,dx
    return poly([(x,y),(bx+px*W/2,by+py*W/2),(bx-px*W/2,by-py*W/2)])
def arrow(ic,x0,y0,x1,y1,c=A3,w=AW,L=9,W=9):
    n=math.hypot(x1-x0,y1-y0); dx,dy=(x1-x0)/n,(y1-y0)/n
    ic.stroke(poly([(x0,y0),(x1-dx*(L-2),y1-dy*(L-2))],False),c,w)
    ic.fill(arrowhead(x1,y1,dx,dy,L,W),c)
def poly_reg(n,cx,cy,r,rot=-90): return [(cx+r*math.cos(math.radians(rot+360*i/n)),cy+r*math.sin(math.radians(rot+360*i/n))) for i in range(n)]

@icon
def point():
    ic=Icon('point','Point')
    for d in ((0,-1),(0,1),(-1,0),(1,0)):
        ic.stroke(poly([(32+d[0]*10,32+d[1]*10),(32+d[0]*22,32+d[1]*22)],False),A3,AW)
    pt(ic,32,32,A3,6); return ic
@icon
def text():
    ic=Icon('text','Text')
    ic.stroke('M13 17L51 17M32 17L32 49M25 49L39 49',A3,AW)
    ic.stroke('M13 17L13 23M51 17L51 23',A3,AW)
    return ic
@icon
def construction():
    ic=Icon('construction','Construction'); 
    ic.stroke('M13 51L19 45M25 39L31 33M37 27L43 21M49 15L51 13',GUIDE,AW)
    ic.stroke('M10 54L12 52',GUIDE,AW)
    pt(ic,12,52,GUIDE); pt(ic,52,12,GUIDE)
    return ic
@icon
def dimension():
    ic=Icon('dimension','Dimension')
    ic.stroke('M10 12L10 52M54 12L54 52',INK,AW)                          # extension lines, as heavy as the other sketch strokes
    ic.stroke('M19 32L45 32',A3,AW)
    ic.fill(arrowhead(13,32,-1,0,11,12),A3); ic.fill(arrowhead(51,32,1,0,11,12),A3)
    return ic
def dimension():
    ic=Icon('dimension','Dimension')
    ic.stroke('M10 14L10 50M54 14L54 50',INK,OW)
    ic.stroke('M18 32L46 32',A3,AW)
    ic.fill(arrowhead(10,32,-1,0,10,10),A3); ic.fill(arrowhead(54,32,1,0,10,10),A3)
    return ic
@icon
def dots():
    ic=Icon('dots','More')
    for x in (13,32,51): ic.dot(x,32,5,INK)
    return ic
@icon
def axis():
    ic=Icon('axis','Axis'); ic.stroke('M10 48L32 48L48 22',INK,OW) if False else None
    ic.stroke('M12 52L12 12',A3,AW); ic.stroke('M12 52L52 52',A3,AW)
    ic.fill(arrowhead(12,8,0,-1,9,9),A3); ic.fill(arrowhead(56,52,1,0,9,9),A3)
    pt(ic,12,52,INK,4); return ic
@icon
def origin():
    ic=Icon('origin','Origin')
    ic.stroke('M32 32L32 8',A3,AW); ic.stroke('M32 32L56 32',A3,AW); ic.stroke('M32 32L16 48',A3,AW)
    ic.fill(arrowhead(32,5,0,-1,8,8),A3); ic.fill(arrowhead(59,32,1,0,8,8),A3); ic.fill(arrowhead(12,52,-1,1,8,8),A3)
    pt(ic,32,32,INK,5); return ic

def dashes(ic,x0,y0,x1,y1,c,w=AW,n=3,fr=0.55):
    """n dashes along a segment"""
    for i in range(n):
        a=i/n; b=a+fr/n
        ic.stroke(poly([(x0+(x1-x0)*a,y0+(y1-y0)*a),(x0+(x1-x0)*b,y0+(y1-y0)*b)],False),c,w)
@icon
def trim():
    ic=Icon('trim','Trim')
    ic.stroke('M22 10L22 54M42 10L42 54',P3,AW)
    ic.stroke('M7 32L22 32M42 32L57 32',A3,AW)
    ic.stroke('M28 32L36 32',P3,AW)
    return ic
@icon
def break_():
    ic=Icon('break','Break')
    ic.stroke('M7 32L23 32M41 32L57 32',A3,AW); pt(ic,23,32); pt(ic,41,32)
    ic.stroke('M32 22L32 42',INK,OW)
    return ic
@icon
def offset():
    ic=Icon('offset','Offset')
    ic.stroke('M8 56L8 28A20 20 0 0 1 28 8L56 8',P3,AW)
    ic.stroke('M24 56L24 32A8 8 0 0 1 32 24L56 24',A3,AW)
    ic.stroke('M44 8L44 24',INK,OW) if False else None
    return ic
def tri_pair(ic,fill_l,fill_r,sl,sr,w):
    L=[(8,50),(28,50),(28,16)]; R=[(56,50),(36,50),(36,16)]
    ic.fill(poly(L),fill_l); ic.fill(poly(R),fill_r)
    ic.stroke(poly(L),sl,w); ic.stroke(poly(R),sr,w)
@icon
def mirror():
    ic=Icon('mirror','Mirror')
    ic.stroke('M32 6L32 12M32 20L32 26M32 34L32 40M32 48L32 54',GUIDE,2.0); tri_pair(ic,P2,A2,INK,INK,OW)
    return ic
@icon
def sketch_mirror():
    ic=Icon('sketch_mirror','Sketch mirror')
    ic.stroke('M32 6L32 12M32 20L32 26M32 34L32 40M32 48L32 54',GUIDE,AW)
    ic.stroke(poly([(7,50),(25,50),(25,16)]),P3,AW); ic.stroke(poly([(57,50),(39,50),(39,16)]),A3,AW)
    return ic
@icon
def sketch_fillet():
    ic=Icon('sketch_fillet','Sketch fillet')
    ic.stroke('M12 34L12 12L34 12',GUIDE,OW)
    ic.stroke('M12 54L12 34A22 22 0 0 1 34 12L54 12',A3,AW)
    pt(ic,12,54); pt(ic,54,12); return ic
@icon
def sketch_chamfer():
    ic=Icon('sketch_chamfer','Sketch chamfer')
    ic.stroke('M12 32L12 12L32 12',GUIDE,OW)
    ic.stroke('M12 54L12 32L32 12L54 12',A3,AW)
    pt(ic,12,54); pt(ic,54,12); return ic
def arc_pts(cx,cy,r,deg): a=math.radians(deg); return cx+r*math.cos(a),cy+r*math.sin(a)
@icon
def spun():
    ic=Icon('spun','Spun profile')
    ic.stroke('M32 4L32 60',INK,OW,dash='4 5')                                 # the axis, dashed like the others
    ic.stroke('M37 8L51 8C60 18 47 30 57 44L37 44Z',A3,AW,fill=A1)               # the profile that is spun, large
    cx,cy,rx,ry=32,50,22,8
    P=lambda t:(cx+rx*math.cos(math.radians(t)),cy+ry*math.sin(math.radians(t)))
    t_tip=142; tip=P(t_tip); L,W=11,12
    t_b=t_tip
    while math.hypot(P(t_b)[0]-tip[0],P(t_b)[1]-tip[1])<L: t_b-=0.5
    base=P(t_b)                                                                # the arc ends exactly in the middle of the head's base
    dx,dy=tip[0]-base[0],tip[1]-base[1]; n=math.hypot(dx,dy); dx,dy=dx/n,dy/n  # the head points from the end of the shaft to its tip, along the arc
    arc=arcpts(cx,cy,rx,ry,0,t_b)
    head=poly([tip,(base[0]-dy*W/2,base[1]+dx*W/2),(base[0]+dy*W/2,base[1]-dx*W/2)])
    ic.stroke(arc,GAP,AW+3.6); ic.stroke(head,GAP,3.6,fill=GAP)               # punch-out around the arrow
    ic.stroke(arc,A3,AW); ic.fill(head,A3)
    return ic
def spun():
    ic=Icon('spun','Spun profile')
    ic.stroke('M32 4L32 60',INK,OW,dash='4 5')                                 # the axis, dashed like the others
    ic.stroke('M37 8L51 8C60 18 47 30 57 44L37 44Z',A3,AW,fill=A1)               # the profile that is spun, large
    cx,cy,rx,ry=32,51,22,7
    P=lambda t:(cx+rx*math.cos(math.radians(t)),cy+ry*math.sin(math.radians(t)))
    t_tip=172; tip=P(t_tip)
    dx,dy=-rx*math.sin(math.radians(t_tip)),ry*math.cos(math.radians(t_tip)); n=math.hypot(dx,dy); dx,dy=dx/n,dy/n   # heading at the tip
    L,W=11,12; base=(tip[0]-dx*L,tip[1]-dy*L)
    # the shaft stops at the middle of the head's base, so the head sits on the end of the arc
    t_b=t_tip
    while math.hypot(P(t_b)[0]-tip[0],P(t_b)[1]-tip[1])<L-1.5: t_b-=1
    arc=arcpts(cx,cy,rx,ry,0,t_b)
    head=poly([tip,(base[0]-dy*W/2,base[1]+dx*W/2),(base[0]+dy*W/2,base[1]-dx*W/2)])
    ic.stroke(arc,GAP,AW+3.6); ic.stroke(head,GAP,3.6,fill=GAP)               # punch-out around the arrow
    ic.stroke(arc,A3,AW); ic.fill(head,A3)
    return ic
def spun():
    ic=Icon('spun','Spun profile')
    ic.stroke('M32 4L32 60',INK,OW,dash='4 5')                                 # the axis, dashed like the others
    ic.stroke('M37 8L51 8C60 18 47 30 57 44L37 44Z',A3,AW,fill=A1)               # the profile that is spun, large
    cx,cy,rx,ry=32,51,22,7
    t0,t1=-10,165
    p=lambda t:(cx+rx*math.cos(math.radians(t)),cy+ry*math.sin(math.radians(t)))
    e=p(t1); dx,dy=-rx*math.sin(math.radians(t1)),ry*math.cos(math.radians(t1)); n=math.hypot(dx,dy); dx,dy=dx/n,dy/n   # heading at the end: left and a little up
    arc=arcpts(cx,cy,rx,ry,t0,t1); head=poly([(e[0]+dx*3,e[1]+dy*3),(e[0]-dx*8+dy*6,e[1]-dy*8-dx*6),(e[0]-dx*8-dy*6,e[1]-dy*8+dx*6)])
    ic.stroke(arc,GAP,AW+3.6); ic.stroke(head,GAP,3.6,fill=GAP)               # punch-out around the arrow
    ic.stroke(arc,A3,AW); ic.fill(head,A3)
    return ic
@icon
def constrainer():
    ic=Icon('constrainer','Constrainer'); d=poly([(10,16),(54,16),(54,48),(10,48)])
    ic.stroke(d,A3,AW,fill=A1)
    ic.stroke('M23 27L41 27M23 37L41 37',A3,AW)                      # the constraint it applies, drawn as the equal glyph
    for x,y in ((10,16),(54,48)):
        ic.dot(x,y,5.8,GAP); ic.dot(x,y,3.4,A3)
    return ic
def constrainer():
    ic=Icon('constrainer','Constrainer')
    d=poly([(14,16),(50,16),(50,48),(14,48)]); ic.stroke(d,A3,AW)
    ic.stroke('M26 10L38 10M26 55L38 55',INK,OW) if False else None
    ic.stroke('M24 29L40 29M24 35L40 35',INK,OW)
    pt(ic,14,16); pt(ic,50,48); return ic
@icon
def finish():
    ic=Icon('finish','Finish')
    ic.stroke('M12 34L26 48L52 16',A3,NW); return ic
@icon
def svg():
    ic=Icon('svg','Insert SVG'); doc='M14 6L38 6L52 20L52 58L14 58Z'
    ic.fill(doc,P1); ic.stroke(doc,INK,OW); ic.stroke('M38 6L38 20L52 20',INK,OW)
    ic.stroke('M21 46C23 30 33 30 34 38C35 44 42 40 45 30',A3,AW) if False else None
    ic.stroke('M20 46C22 32 30 30 33 38C36 46 42 42 46 30',A3,AW*0.8); pt(ic,20,46,A3,2.6); pt(ic,46,30,A3,2.6)
    return ic
@icon
def fit_section():
    ic=Icon('fit_section','Fit curves to section')
    for x in (14,32,50): ic.stroke(poly([(x,10),(x,54)],False),GUIDE,OW)
    ic.stroke('M6 42C20 42 24 20 32 28C40 36 44 22 58 22',A3,AW)
    return ic
