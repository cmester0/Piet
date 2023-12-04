import sys
import png

from generate_number import * # optimize number

def stk_to_piet(i_file, o_file, optim=True):
    # ⚪

    # ❤🧡💛💚💙💜
    # 🔴🟠🟡🟢🔵🟣
    # 🟥🟧🟨🟩🟦🟪

    # ⚫

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

    stack_optimizer = StackOptimizer()

    def make_block(block):
        output = ["🔴"]
        previous_c = 0
        previous_r = 1
        for inp in block:
            cmd = inp.split()
            match cmd[0]:
                case "push":
                    for _ in range(int(cmd[1])-1):
                        output.append(colors[previous_r][previous_c])
                    previous_r = (previous_r + 1) % 3
                    output.append(colors[previous_r][previous_c])
                case "pop":
                    previous_r = (previous_r - 1) % 3
                    output.append(colors[previous_r][previous_c])
                case "add":
                    previous_c = (previous_c + 1) % 6
                    output.append(colors[previous_r][previous_c])
                case "sub":
                    previous_c = (previous_c + 1) % 6
                    previous_r = (previous_r + 1) % 3
                    output.append(colors[previous_r][previous_c])
                case "mul":
                    previous_c = (previous_c + 1) % 6
                    previous_r = (previous_r - 1) % 3
                    output.append(colors[previous_r][previous_c])
                case "div":
                    previous_c = (previous_c + 2) % 6
                    output.append(colors[previous_r][previous_c])
                case "mod":
                    previous_c = (previous_c + 2) % 6
                    previous_r = (previous_r + 1) % 3
                    output.append(colors[previous_r][previous_c])
                case "not":
                    previous_c = (previous_c + 2) % 6
                    previous_r = (previous_r - 1) % 3
                    output.append(colors[previous_r][previous_c])
                case "greater":
                    previous_c = (previous_c + 3) % 6
                    output.append(colors[previous_r][previous_c])
                case "dup":
                    previous_c = (previous_c + 4) % 6
                    output.append(colors[previous_r][previous_c])
                case "roll":
                    previous_c = (previous_c + 4) % 6
                    previous_r = (previous_r + 1) % 3
                    output.append(colors[previous_r][previous_c])
                case "inN":
                    previous_c = (previous_c + 4) % 6
                    previous_r = (previous_r - 1) % 3
                    output.append(colors[previous_r][previous_c])
                case "inC":
                    previous_c = (previous_c + 5) % 6
                    output.append(colors[previous_r][previous_c])
                case "outN":
                    previous_c = (previous_c + 5) % 6
                    previous_r = (previous_r + 1) % 3
                    output.append(colors[previous_r][previous_c])
                case "outC":
                    previous_c = (previous_c + 5) % 6
                    previous_r = (previous_r - 1) % 3
                    output.append(colors[previous_r][previous_c])
                case "branch":
                    # pointer
                    previous_c = (previous_c + 3) % 6
                    previous_r = (previous_r + 1) % 3
                    output.append(colors[previous_r][previous_c])

                    return output, [cmd[1], cmd[2]]
                case "goto":
                    return output, [cmd[1]]
                case "term":
                    return output, ["term"]
                case default:
                    print("Invalid cmd:", cmd)
        return output, []

    inp_lines = []
    with open(i_file, 'r') as f:
        inp_lines = f.readlines()
    lines = []
    for x in inp_lines:
        if "#" in x:
            lines.append(x[:x.find("#")])
        else:
            lines.append(x)

    blocks = dict()
    for l in [x.split("\n") for x in "".join(lines).split("\n\n")]:
        cmds = []
        for x in l[1:]:
            cmd = x.split()
            if len(cmd) == 2 and cmd[0] == "push":
                push_val = int(cmd[1])
                cmds = cmds + (stack_optimizer.optimize_number(int(cmd[1])) if optim else (["push "+str(push_val)] if push_val > 0 else ["push 1", "not", "push "+str(push_val), "sub"]))
            else:
                if len(x) > 0:
                    cmds.append(x)
        blocks[l[0].split()[1]] = make_block(cmds)
    blocks["term"] = ([], [])

    print ("Reading from:", i_file)
    print ("Saving to:", o_file)

    with open(o_file, 'w') as f:
        final_output = []

        for _ in range(30):
            final_output.append(["⚪"]*150)

        def block_line_left(yi, xi, gap=2):
            final_output[yi][xi-3] = "⚫"
            final_output[yi-2][xi-2] = "⚫"
            final_output[yi-1][xi-1] = "⚫"
            final_output[yi+gap+2][xi-2] = "⚫"
            final_output[yi+gap+1][xi-4] = "⚫"
            final_output[yi+gap][xi-3] = "⚫"

        def block_line_right(yi, xi, gap=2):
            final_output[yi][xi+1] = "⚫"
            final_output[yi+gap+2][xi] = "⚫"

        # for x in blocks:
        #     # print (x, blocks[x][0])

        x = "main"
        j_width = len(blocks[x][0]) // 7

        start_index = 10
        
        final_output[0][start_index+1] = "⚫"

        print (len(blocks[x][0]))
        li = 2 # line index

        final_output[li+1][start_index] = "⚫"
        final_output[li][start_index-2] = "⚫"
        final_output[li-1][start_index-1] = "⚫"

        offset = 0
        do_break = False
        right_line_gap = 2
        left_line_gap = 1
        going_right = True
        while True:
            if going_right:
                for j in range(j_width):
                    if offset + j >= len(blocks[x][0]):
                        do_break = True
                        break
                    if offset + j_width < len(blocks[x][0]):
                        if len(set(blocks[x][0][offset+j:offset+j_width])) == 2:
                            offset += j - 1
                            break
                    final_output[li][start_index+j] = blocks[x][0][offset+j]
                else:
                    offset += j_width - 1
                if do_break:
                    break
                block_line_right(li, start_index+j_width, gap=right_line_gap)
                li = li + right_line_gap + 1
            else: # going left
                for j in range(j_width):
                    if offset + j >= len(blocks[x][0]):
                        do_break = True
                        break
                    if offset + j_width < len(blocks[x][0]):
                        if len(set(blocks[x][0][offset+j:offset+j_width])) == 2:
                            offset += j - 1
                            break
                    final_output[li][j_width+start_index-1-j] = blocks[x][0][offset+j]
                else:
                    offset += j_width - 1
                if do_break:
                    break
                block_line_left(li, start_index, gap=left_line_gap)
                li = li + left_line_gap + 1
            going_right = not going_right

        if going_right:
            block_line_right(li, start_index+j_width, gap=5)
            li = li + 5 + 1

            final_output[li-1][start_index] = "🔴"
            final_output[li][start_index] = "🔴"
            final_output[li+1][start_index] = "🔴"
            final_output[li][start_index-1] = "🔴"

            final_output[li-1][start_index+1] = "⚫"
            final_output[li-2][start_index] = "⚫"
            final_output[li-1][start_index-1] = "⚫"
            final_output[li][start_index-2] = "⚫"
            final_output[li+1][start_index-1] = "⚫"
            final_output[li+2][start_index] = "⚫"
            final_output[li+1][start_index+1] = "⚫"

        else:
            block_line_left(li, start_index, gap=5)
            li = li + 5 + 1

            final_output[li-1][start_index+j_width] = "🔴"
            final_output[li][start_index+j_width] = "🔴"
            final_output[li+1][start_index+j_width] = "🔴"
            final_output[li][start_index+1+j_width] = "🔴"

            final_output[li-1][start_index-1+j_width] = "⚫"
            final_output[li-2][start_index+j_width] = "⚫"
            final_output[li-1][start_index+1+j_width] = "⚫"
            final_output[li][start_index+2+j_width] = "⚫"
            final_output[li+1][start_index+1+j_width] = "⚫"
            final_output[li+2][start_index+j_width] = "⚫"
            final_output[li+1][start_index-1+j_width] = "⚫"

        for x in final_output:
            f.write("".join(list(x)) + "\n")
