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

        push_max = len(blocks)*2
        pre = len(blocks)*2

        final_output.append("⚪"*(pre+push_max+1))
        for i in range(len(blocks)+1):
            final_output.append("⚪"*(pre+push_max+1))

        final_output[1] = final_output[1][:pre-1] + "⚫⚫" + final_output[i+1][pre-1+2:]

        for i in range(1,len(blocks)):
            j = i
            final_output[i+1] = final_output[i+1][:pre-1-(j*2)] + "⚫" + final_output[i+1][pre-1-(j*2)+1:]
            final_output[i+1] = final_output[i+1][:pre-1+(i*2)] + "⚫" + final_output[i+1][pre-1+(i*2)+1:]

        final_output[len(blocks)+1] = final_output[len(blocks)+1][:pre-1+(len(blocks)*2)] + "⚫" + final_output[i+1][pre-1+(len(blocks)*2)+1:]

        key_to_index = dict()
        prev_size = 0
        prev_clearance = 0
        for i, l in enumerate(sorted(blocks, key=lambda x: len(blocks[x][0]))):
            key_to_index[l] = i
            curr_size = len(blocks[l][0])

            clearance = 2
            if curr_size >= 2:
                ac, ar = rev_map[blocks[l][0][-1]]
                bc, br = rev_map[blocks[l][0][-2]]
                clearance = 3 # if ((ac - bc) % 6 == 3 and (ar - br) % 3 == 1) else 2
            else:
                clearance = 2

            if curr_size <= prev_size + clearance:
                blocks[l] = (["⚪"] * (prev_size + (0 if clearance == prev_clearance else -1) + clearance - curr_size) + blocks[l][0], blocks[l][1])

            prev_size = len(blocks[l][0])
            prev_clearance = clearance


        mid_section = []
        max_block_len = max([len(blocks[l][0]) for l in blocks])
        print (max_block_len)
        for xi in range(max_block_len + 2):
            xvals = ["⚪" for _ in range(pre + push_max + 1)]
            mid_section = ["".join(xvals)] + mid_section

        final_output_offset = len(final_output)
        final_output = final_output + mid_section

        for l in blocks:
            i = key_to_index[l]
            for xi, x in enumerate(blocks[l][0]):
                final_output[xi + final_output_offset] = final_output[xi+final_output_offset][:pre + (i*2)] + x + final_output[xi+final_output_offset][pre + (i*2) + 1:]
            if len(blocks[l][1]) == 1:
                j = key_to_index[blocks[l][1][0]]
                final_output[len(blocks[l][0])-1+final_output_offset] = final_output[len(blocks[l][0])-1+final_output_offset][:pre-(j * 2+2)] + "⚫" + final_output[len(blocks[l][0])-1+final_output_offset][pre-(j * 2+2)+1:]
                final_output[len(blocks[l][0])+final_output_offset] = final_output[len(blocks[l][0])+final_output_offset][:pre+(i*2)] + "⚫" + final_output[len(blocks[l][0])+final_output_offset][pre+i*2+1:]
            if len(blocks[l][1]) == 2:
                j = key_to_index[blocks[l][1][0]]
                final_output[len(blocks[l][0])-1+final_output_offset] = final_output[len(blocks[l][0])-1+final_output_offset][:pre-(j * 2+2)] + "⚫" + final_output[len(blocks[l][0])-1+final_output_offset][pre-(j * 2+2)+1:]

                j = key_to_index[blocks[l][1][1]]
                final_output[len(blocks[l][0])+final_output_offset] = final_output[len(blocks[l][0])+final_output_offset][:pre-(j * 2+2)] + "⚫" + final_output[len(blocks[l][0])+final_output_offset][pre-(j * 2+2)+1:]
                final_output[len(blocks[l][0])+1+final_output_offset] = final_output[len(blocks[l][0])+1+final_output_offset][:pre+i*2] + "⚫" + final_output[len(blocks[l][0])+1+final_output_offset][pre+i*2+1:]

        i = key_to_index["term"]
        final_output.append("⚪" * (pre+(i*2)-2) + "⚪⚫⚪⚫⚪" + "⚪" * (push_max-(i*2)-2))
        final_output.append("⚪" * (pre+(i*2)-2) + "⚫🔴🔴🔴⚫" + "⚪" * (push_max-(i*2)-2))
        final_output.append("⚪" * (pre+(i*2)-2) + "⚪⚫🔴⚫⚪" + "⚪" * (push_max-(i*2)-2))
        final_output.append("⚪" * (pre+(i*2)-2) + "⚪⚪⚫⚪⚪" + "⚪" * (push_max-(i*2)-2))

        final_output.append("⚪" * (pre  + push_max + 1))
        main_goto = key_to_index["main"]
        final_output[-1] = final_output[-1][:pre-(main_goto * 2+2)] + "⚫" + final_output[-1][pre-(main_goto * 2+2)+1:]

        while (final_output[len(blocks)+3] == "⚪" * len(final_output[len(blocks)+3])):
            final_output = final_output[:len(blocks)+3] + final_output[len(blocks)+4:]

        for x in final_output:
            f.write("".join(list(x)) + "\n")
