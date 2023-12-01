# Spec: https://www.dangermouse.net/esoteric/piet.html
# TODO

import sys
import png

import time

# ⚪

# ❤🧡💛💚💙💜
# 🔴🟠🟡🟢🔵🟣
# 🟥🟧🟨🟩🟦🟪

# ⚫

# FFFFFF

# FFC0C0 FFFFC0 C0FFC0 C0FFFF C0C0FF FFC0FF
# FF0000 FFFF00 00FF00 00FFFF 0000FF FF00FF
# C00000 C0C000 00C000 00C0C0 0000C0 C000C0

# 000000

debug = True # False
gif_saved = True

colors = [["❤","🧡","💛","💚","💙","💜"],
          ["🔴","🟠","🟡","🟢","🔵","🟣"],
          ["🟥","🟧","🟨","🟩","🟦","🟪"]]

rev_map = {
    "❤": (0,0),
    "🔴": (0,1),
    "🟥": (0,2),
    "🧡": (1,0),
    "🟠": (1,1),
    "🟧": (1,2),
    "💛": (2,0),
    "🟡": (2,1),
    "🟨": (2,2),
    "💚": (3,0),
    "🟢": (3,1),
    "🟩": (3,2),
    "💙": (4,0),
    "🔵": (4,1),
    "🟦": (4,2),
    "💜": (5,0),
    "🟣": (5,1),
    "🟪": (5,2)}

# with open(sys.argv[1], 'rb') as f:
reader = png.Reader(filename=sys.argv[1])
(width, height, rows, info) = reader.read()

pxls = []
for x in rows:
    l = list(map(int,x))
    row = []
    for y in range(0,len(l),3):
        match l[y:y+3]:
            case [255, 255, 255]:
                row.append("⚪")

            case [255, 192, 192]:
                row.append("❤")
            case [255, 255, 192]:
                row.append("🧡")
            case [192, 255, 192]:
                row.append("💛")
            case [192, 255, 255]:
                row.append("💚")
            case [192, 192, 255]:
                row.append("💙")
            case [255, 192, 255]:
                row.append("💜")

            # FF0000 FFFF00 00FF00 00FFFF 0000FF FF00FF
            case [255,   0,   0]:
                row.append("🔴")
            case [255, 255,   0]:
                row.append("🟠")
            case [  0, 255,   0]:
                row.append("🟡")
            case [  0, 255, 255]:
                row.append("🟢")
            case [  0,   0, 255]:
                row.append("🔵")
            case [255,   0, 255]:
                row.append("🟣")

            # C00000 C0C000 00C000 00C0C0 0000C0 C000C0
            case [192,   0,   0]:
                row.append("🟥")
            case [192, 192,   0]:
                row.append("🟧")
            case [  0, 192,   0]:
                row.append("🟨")
            case [  0, 192, 192]:
                row.append("🟩")
            case [  0,   0, 192]:
                row.append("🟦")
            case [192,   0, 192]:
                row.append("🟪")

            case [  0,   0,   0]:
                row.append("⚫")

    pxls.append(row)

blobs = {}

blobs["⚪"] = set()
blobs["❤"] = set()
blobs["🧡"] = set()
blobs["💛"] = set()
blobs["💚"] = set()
blobs["💙"] = set()
blobs["💜"] = set()
blobs["🔴"] = set()
blobs["🟠"] = set()
blobs["🟡"] = set()
blobs["🟢"] = set()
blobs["🔵"] = set()
blobs["🟣"] = set()
blobs["🟥"] = set()
blobs["🟧"] = set()
blobs["🟨"] = set()
blobs["🟩"] = set()
blobs["🟦"] = set()
blobs["🟪"] = set()
blobs["⚫"] = set()

for ci, c in enumerate(pxls):
    for pi, p in enumerate(c):
        blobs[p].add((pi,ci))

all_blobs = []
for l in blobs:
    if (l == "⚫" or l == "⚪"):
        continue
    seperate_blobs = []
    for _ in range(len(blobs[l])):
        for x, y in blobs[l]:
            blobs_in = []
            for i, v in enumerate(seperate_blobs):
                if ((x, y) in v or
                    (x+1, y) in v or
                    (x, y+1) in v or
                    (x-1, y) in v or
                    (x, y-1) in v):
                    blobs_in.append(i)
            match blobs_in:
                case []:
                    seperate_blobs.append(set([(x,y)]))
                case [vi]:
                    seperate_blobs[vi].add((x,y))
                case vs:
                    merge_blob = set()
                    for vi in reversed(vs):
                        merge_blob = merge_blob.union(seperate_blobs[vi])
                        seperate_blobs = seperate_blobs[:vi] + seperate_blobs[vi+1:]
                    seperate_blobs.append(merge_blob)
    for x in seperate_blobs:
        all_blobs.append((l, x))

pix_to_blob = []
for ci, c in enumerate(pxls):
    pix_to_blob.append([])
    for pi in range(len(c)):
        pix_to_blob[ci].append(-1)

all_blobs_indexed = []
for i, (c, blob) in enumerate(all_blobs):
    r_u = max(list(blob), key = lambda x: (x[0], -x[1]))
    r_d = max(list(blob), key = lambda x: (x[0], x[1]))
    d_r = max(list(blob), key = lambda x: (x[1], x[0]))
    d_l = max(list(blob), key = lambda x: (x[1], -x[0]))
    l_d = max(list(blob), key = lambda x: (-x[0], x[1]))
    l_u = max(list(blob), key = lambda x: (-x[0], -x[1]))
    u_l = max(list(blob), key = lambda x: (-x[1], -x[0]))
    u_r = max(list(blob), key = lambda x: (-x[1], x[0]))

    abi = []
    abi.append(r_u)
    abi.append(r_d)
    abi.append(d_r)
    abi.append(d_l)
    abi.append(l_d)
    abi.append(l_u)
    abi.append(u_l)
    abi.append(u_r)
    abi.append(len(blob))
    abi.append(c)
    all_blobs_indexed.append(abi)

    for x, y in blob:
        pix_to_blob[y][x] = i

# 0 = right
# 1 = down
# 2 = left
# 3 = up
dp = 0

# 0 left
# 1 right
cc = 0

last_color = "⚪"
last_bs = 0
input_eof = False
stack = []

def next_step_from_dp(k):
    global dp
    (cx, cy) = k
    match dp:
        case 0:
            next_pos = (cx + 1, cy)
        case 1:
            next_pos = (cx, cy + 1)
        case 2:
            next_pos = (cx - 1, cy)
        case 3:
            next_pos = (cx, cy - 1)
    return next_pos, (not next_pos in blobs["⚫"] and ((0 <= next_pos[1] and next_pos[1] < len(pix_to_blob)) and (0 <= next_pos[0] and next_pos[0] < len(pix_to_blob[next_pos[1]]))))

def step(k):
    global dp
    global cc
    global stack
    global last_color
    global last_bs
    global input_eof
    (cx, cy) = k
    curr_pos = (cx, cy)
    next_pos, valid_pix = next_step_from_dp(curr_pos)
    if not valid_pix:
        block = 0
        while not valid_pix:
            if block % 2 == 0:
                cc = (cc + 1) % 2
            else:
                dp = (dp + 1) % 4

            block += 1
            if block >= 8:
                # for _ in sys.stdin:
                #     pass
                print ("Should term", block)
                # exit()
                return (0,0), True

            if not (cx,cy) in blobs["⚪"]:
                r_u, r_d, d_r, d_l, l_d, l_u, u_l, u_r, bs, c = all_blobs_indexed[pix_to_blob[cy][cx]]

                match dp, cc:
                    case (0, 0):
                        curr_pos = r_u
                    case (0, 1):
                        curr_pos = r_d
                    case (1, 0):
                        curr_pos = d_r
                    case (1, 1):
                        curr_pos = d_l
                    case (2, 0):
                        curr_pos = l_d
                    case (2, 1):
                        curr_pos = l_u
                    case (3, 0):
                        curr_pos = u_l
                    case (3, 1):
                        curr_pos = u_r

            next_pos, valid_pix = next_step_from_dp(curr_pos)

        return curr_pos, False
        # return cx, cy
    elif next_pos in blobs["⚪"]:
        last_color = "⚪"
        last_pos = next_pos
        while next_pos in blobs["⚪"]:
            last_pos = next_pos
            next_pos, valid_pix = next_step_from_dp(next_pos)
            # if not valid_pix:
            #     break
        return last_pos, False
    else:
        r_u, r_d, d_r, d_l, l_d, l_u, u_l, u_r, bs, c = all_blobs_indexed[pix_to_blob[next_pos[1]][next_pos[0]]]

        if last_color != "⚪":
            lc_c, lc_r = rev_map[last_color]
            cc_c, cc_r = rev_map[c]

            match (cc_c - lc_c) % 6, (cc_r - lc_r) % 3:
                case (0,1): # push
                    if debug:
                        print("push",last_bs)
                    stack.append(last_bs)
                case (0,2): # pop
                    if debug:
                        print("pop")
                    if len(stack) >= 1:
                        stack.pop()
                case (1,0): # add
                    if debug:
                        print("add")
                    if len(stack) >= 2:
                        a = stack.pop()
                        b = stack.pop()
                        stack.append(a + b)
                case (1,1): # sub
                    if debug:
                        print("sub")
                    if len(stack) >= 2:
                        a = stack.pop()
                        b = stack.pop()
                        stack.append(b - a)
                case (1,2): # mul
                    if debug:
                        print("mul")
                    if len(stack) >= 2:
                        a = stack.pop()
                        b = stack.pop()
                        stack.append(a * b)
                case (2,0): # div
                    if debug:
                        print("div")
                    if len(stack) >= 2:
                        a = stack.pop()
                        b = stack.pop()
                        if a != 0:
                            stack.append(b // a) # TODO: Ignore if div by zero
                case (2,1): # mod
                    if debug:
                        print("mod")
                    if len(stack) >= 2:
                        a = stack.pop()
                        b = stack.pop()
                        stack.append(b % a)
                case (2,2): # not
                    if debug:
                        print("not")
                    if len(stack) >= 1:
                        a = stack.pop()
                        stack.append(1 if a == 0 else 0)
                case (3,0): # greater
                    if debug:
                        print("greater")
                    if len(stack) >= 2:
                        a = stack.pop()
                        b = stack.pop()
                        stack.append(1 if b > a else 0)
                case (3,1): # pointer
                    if debug:
                        print("pointer", str(len(stack) >= 1))
                    if len(stack) >= 1:
                        a = stack.pop()
                        dp = (dp + a) % 4
                case (3,2): # switch
                    if debug:
                        print("switch")
                    if len(stack) >= 1:
                        a = stack.pop()
                        cc = (cc + abs(a)) % 2
                case (4,0): # dup
                    if debug:
                        print("dup")
                    if len(stack) >= 1:
                        a = stack.pop()
                        stack.append(a)
                        stack.append(a)
                case (4,1): # roll
                    if debug:
                        print("roll")
                    # TODO: more clever
                    def roll(l, n):
                        if n > 0:
                            return roll([l[-1]] + l[:-1], n-1)
                        elif n < 0:
                            return roll(l[1:] + [l[0]], n+1)
                        else:
                            return l

                    if len(stack) >= 2:
                        a = stack.pop()
                        b = stack.pop() # TODO: Ignore b negative
                        stack = stack[:-b] + roll(stack[-b:], a)
                case (4,2): # inN
                    if debug:
                        print("inN")
                    if not input_eof:
                        value = sys.stdin.buffer.peek(1)
                        if value == b"":
                            stack.append(-1)
                            input_eof = True
                        else:
                            for x in reversed(range(len(value))):
                                if "".join(map(chr, value[:x+1])).isnumeric():
                                    stack.append(int("".join(map(chr, sys.stdin.buffer.read(x+1)))))
                                    break
                    else:
                        stack.append(-1)
                case (5,0): # inC
                    if debug:
                        print("inC")
                    if not input_eof:
                        value = sys.stdin.buffer.peek(1)
                        if value == b"":
                            stack.append(-1)
                            input_eof = True
                        else:
                            stack.append(ord(chr(sys.stdin.buffer.read(1)[0])))
                    else:
                        stack.append(-1)
                case (5,1): # outN
                    if debug:
                        print("outN\n")
                    if len(stack) > 0:
                        print(stack.pop(),end="")
                    if debug:
                        print("\n")
                case (5,2): # outC
                    if debug:
                        print("outC\n")
                    if len(stack) > 0:
                        print(chr(stack.pop()),end="")
                    if debug:
                        print("\n")

        last_color = c
        last_bs = bs

        match dp, cc:
            case (0, 0):
                return r_u, False
            case (0, 1):
                return r_d, False
            case (1, 0):
                return d_r, False
            case (1, 1):
                return d_l, False
            case (2, 0):
                return l_d, False
            case (2, 1):
                return l_u, False
            case (3, 0):
                return u_l, False
            case (3, 1):
                return u_r, False

pos = (0, 0)
r_u, r_d, d_r, d_l, l_d, l_u, u_l, u_r, bs, c = all_blobs_indexed[pix_to_blob[0][0]]
last_color = c
last_bs = bs

from PIL import Image
max_count = -1
frames = []
gif_name = sys.argv[2] if len(sys.argv) > 2 else ""

if gif_name != "":
    reader = png.Reader(filename=sys.argv[1])
    (width, height, rows, info) = reader.read()
    img = Image.new("RGBA", (width, height), "white")
    for xi, x in enumerate(list(rows)):
        l = list(map(int,x))
        row = []
        for y in range(0,len(l),3):
            img.putpixel((y // 3, xi), tuple(l[y:y+3] + [255]))

def save_image():
    print ("SAVING GIF TO:", gif_name)
    frames[0].save(gif_name, format='GIF', append_images=frames[1:], save_all=True, duration=60, loop=0)
    print ("DONE SAVING GIF")
    gif_saved = True

total_steps = 0
while True:
    pos, terminate = step(pos)
    if terminate:
        break

    if debug:
        if last_color != "⚪":
            # time.sleep(0.01)
            print (input_eof, pos, stack, len(frames))

    if gif_name != "" and not gif_saved and (max_count == -1 or len(frames) < max_count):
        img_changed = img.copy()
        img_changed.putpixel(pos, (30, 30, 30, 50))
        img_changed.putpixel(pos, (150 + (100 if dp == 0 else (-100 if dp == 2 else 0)),
                                   150 + (100 if dp == 1 else (-100 if dp == 3 else 0)),
                                   150 + (100 if cc == 0 else -100)))
        frames.append(img_changed)

        if max_count != -1 and len(frames) == max_count:
            save_image()

    total_steps += 1

print ("Total steps: ", total_steps)
print ("Total steps: ", total_steps)
print ("Total steps: ", total_steps)

if gif_name != "" and not gif_saved:
    save_image()

print (gif_saved)
