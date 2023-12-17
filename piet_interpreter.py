# Spec: https://www.dangermouse.net/esoteric/piet.html
# TODO

import sys
import png

import time

from stk_execute import *

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
stack = []

def piet_interpreter(i_file, o_file = "",debug=False,max_count = -1,gif_speed=1):
    global dp
    global cc
    global last_color
    global last_bs
    global stack

    gif_saved=False

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
    stack = []

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

    reader = png.Reader(filename=i_file)
    (width, height, rows, info) = reader.read()

    if debug:
        print ("Read image of:",width,height)

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

    total_coords = 0
    for ci, c in enumerate(pxls):
        for pi, p in enumerate(c):
            blobs[p].add((pi,ci))
            total_coords += 1

    if debug:
        print ("Get all blobs")
    coords_checked = 0
    all_blobs = []

    # Watershed (image processing)
    for l in blobs:
        if (l == "⚫" or l == "⚪"):
            continue
        checked_coord = set()
        seperate_blobs = []
        for x, y in blobs[l]:
            sep_blob = set()
            queue = [(x,y)]
            while len(queue) > 0:
                xi, yi = queue.pop()
                if (xi, yi) in checked_coord:
                    continue
                checked_coord.add((xi,yi))
                sep_blob.add((xi,yi))
                for xn,yn in [(xi+1, yi), (xi, yi+1), (xi, yi-1), (xi-1, yi)]:
                    if (xn, yn) in blobs[l]:
                        queue.append((xn,yn))
            if len(sep_blob) > 0:
                seperate_blobs.append(sep_blob)
        for x in seperate_blobs:
            all_blobs.append((l, x))

    if debug:
        print ("Pix to blob")
    pix_to_blob = []
    for ci, c in enumerate(pxls):
        pix_to_blob.append([])
        for pi in range(len(c)):
            pix_to_blob[ci].append(-1)

    if debug:
        print ("Index all blobs")
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
        global last_color
        global last_bs
        global stack
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
                    if debug:
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

                cmd = []
                match (cc_c - lc_c) % 6, (cc_r - lc_r) % 3:
                    case (0,1): # push
                        if debug:
                            print("push",last_bs)
                        cmd = ["push", str(last_bs)]
                    case (0,2): # pop
                        if debug:
                            print("pop")
                        cmd = ["pop"]
                    case (1,0): # add
                        if debug:
                            print("add")
                        cmd = ["add"]
                    case (1,1): # sub
                        if debug:
                            print("sub")
                        cmd = ["sub"]
                    case (1,2): # mul
                        if debug:
                            print("mul")
                        cmd = ["mul"]
                    case (2,0): # div
                        if debug:
                            print("div")
                        cmd = ["div"]
                    case (2,1): # mod
                        if debug:
                            print("mod")
                        cmd = ["mod"]
                    case (2,2): # not
                        if debug:
                            print("not")
                        cmd = ["not"]
                    case (3,0): # greater
                        if debug:
                            print("greater")
                        cmd = ["greater"]
                    case (3,1): # pointer
                        if debug:
                            print("pointer", str(len(stack) >= 1))
                        if len(stack) >= 1:
                            a = stack.pop()
                            dp = (dp + a) % 4
                            if debug:
                                print("dp", dp, a)
                    case (3,2): # switch
                        if debug:
                            print("switch")
                        if len(stack) >= 1:
                            a = stack.pop()
                            cc = (cc + abs(a)) % 2
                    case (4,0): # dup
                        if debug:
                            print("dup")
                        cmd = ["dup"]
                    case (4,1): # roll
                        if debug:
                            print("roll")
                        cmd = ["roll"]
                    case (4,2): # inN
                        if debug:
                            print("inN")
                        cmd = ["inN"]
                    case (5,0): # inC
                        if debug:
                            print("inC")
                        cmd = ["inC"]
                    case (5,1): # outN
                        if debug:
                            print("outN\n")
                        cmd = ["outN"]
                    case (5,2): # outC
                        if debug:
                            print("outC\n")
                        cmd = ["outC"]

                if cmd:
                    cmd_interpreter(cmd, stack)

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
    frames = []
    gif_name = o_file

    if gif_name != "":
        reader = png.Reader(filename=i_file)
        (width, height, rows, info) = reader.read()
        img = Image.new("RGBA", (width, height), "white")
        for xi, x in enumerate(list(rows)):
            l = list(map(int,x))
            row = []
            for y in range(0,len(l),3):
                img.putpixel((y // 3, xi), tuple(l[y:y+3] + [255]))

    def save_image():
        if debug:
            print ("SAVING GIF TO:", gif_name)
        frames[0].save(gif_name, format='GIF', append_images=frames[1:], save_all=True, duration=60, loop=0)
        if debug:
            print ("DONE SAVING GIF")
        gif_saved = True

    if debug:
        print ("Start running..")
    total_steps = 0
    while True:
        pos, terminate = step(pos)
        if terminate:
            break

        # if total_steps % 100 == 0:
        #     print ("steps", total_steps)
        #     input()

        if debug:
            if last_color != "⚪":
                time.sleep(0.01)
                print (pos, stack, len(frames))

        if gif_name != "" and not gif_saved and (max_count == -1 or len(frames)//gif_speed < max_count):
            img_changed = img.copy()
            img_changed.putpixel(pos, (30, 30, 30, 50))
            img_changed.putpixel(pos, (150 + (100 if dp == 0 else (-100 if dp == 2 else 0)),
                                       150 + (100 if dp == 1 else (-100 if dp == 3 else 0)),
                                       150 + (100 if cc == 0 else -100)))

            for i in range(gif_speed):
                frames.append(img_changed)

            if max_count != -1 and len(frames)//gif_speed == max_count:
                save_image()

        total_steps += 1

    if debug:
        print ("Total steps: ", total_steps)
        print ("Total steps: ", total_steps)
        print ("Total steps: ", total_steps)

    if gif_name != "" and not gif_saved:
        save_image()

    if debug:
        print (gif_saved)
