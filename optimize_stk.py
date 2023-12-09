def optimize_stk(i_file, o_file):
    inp_lines = []
    with open(i_file, 'r') as f:
        inp_lines = f.readlines()

    lines = []
    for x in inp_lines:
        if "#" in x:
            lines.append(x[:x.find("#")])
        else:
            lines.append(x)

    instrs = [(l[0], l[1:]) for l in list(filter(lambda x: len(x) != 0, [list(filter(lambda x: len(x) != 0, x.split("\n"))) for x in ("\n".join(lines)).split("label ")]))]

    def find_match(start, instr, matches):
        j = start
        match_indexes = []
        for m in matches:
            if j >= len(instr):
                return False, []
            while instr[j][0] == "#":
                j += 1
                if j >= len(instr):
                    return False, []
            sources = instr[j].split()
            targets = m.split()
            if sources[0] != targets[0]:
                return False, []
            if len(sources) != len(targets):
                return False, []
            if len(sources) >= 2 and not (targets[1] == "_" or sources[1] == targets[1]):
                return False, []
            match_indexes.append(j)
            j+=1
        return True, match_indexes

    for instr_index in range(len(instrs)):
        label, instr = instrs[instr_index]

        j = 0
        while j < len(instr):
            success, indexes = find_match(j, instr, [
                "push _", "push _", "push 2", "push -1", "roll"])
            if success:
                add0 = int(instr[indexes[0]].split()[1])
                add1 = int(instr[indexes[1]].split()[1])

                instr[indexes[0]] = "push "+str(add1)
                instr[indexes[1]] = "push "+str(add0)
                instr[indexes[2]] = "#delete push 2"
                instr[indexes[3]] = "#delete push -1"
                instr[indexes[4]] = "#delete roll"

                j = 0
                continue

            binop_succeed = False
            binops = [("add",lambda x, y:x+y),
                      ("mul",lambda x, y:x*y),
                      ("mod",lambda x, y:x%y),
                      ("div",lambda x, y:x/y)]
            for (binop, lambda_op) in binops:
                success, indexes = find_match(j, instr, [
                    "push _", "push _", binop])
                if success:
                    add0 = int(instr[indexes[0]].split()[1])
                    add1 = int(instr[indexes[1]].split()[1])

                    instr[indexes[0]] = "#delete push _"
                    instr[indexes[1]] = "push "+str(lambda_op(add0,add1))
                    instr[indexes[2]] = "#delete " + binop

                    binop_succeed = True
                    break
            if binop_succeed:
                j = 0
                continue

            roll_succeed = False
            for k in range(2,25):
                success, indexes = find_match(j, instr, [
                    "push _"] * k + ["push "+str(k),"push _","roll"])
                if success:
                    old_indexes = []
                    a = int(instr[indexes[-2]].split()[1])
                    for i in range(k):
                        indx = (i-a)%k
                        old_indexes.append(instr[indexes[indx]])
                    for i in range(k):
                        instr[indexes[i]] = old_indexes[i]

                    instr[indexes[-3]] = "#delete push _"
                    instr[indexes[-2]] = "#delete push _"
                    instr[indexes[-1]] = "#delete roll"

                    roll_succeed = True
                    break

                success, indexes = find_match(j, instr, ["push "+str(k),"push _","roll"] + ["push "+str(k),"push _","roll"])
                if success:
                    a1 = int(instr[indexes[1]].split()[1])
                    a2 = int(instr[indexes[4]].split()[1])

                    instr[indexes[0]] = "#delete push _"
                    instr[indexes[1]] = "#delete push _"
                    instr[indexes[2]] = "#delete roll"

                    instr[indexes[4]] = "push "+str(a1+a2)

                    roll_succeed = True
                    break

                success, indexes = find_match(j, instr, ["push _", "push _", "roll"])
                if success:
                    b = int(instr[indexes[0]].split()[1])
                    a = int(instr[indexes[1]].split()[1])

                    if a % b == 0:
                        instr[indexes[0]] = "#delete push _"
                        instr[indexes[1]] = "#delete push _"
                        instr[indexes[2]] = "#delete roll"
                        roll_succeed = True
                        break

            if roll_succeed:
                j = 0
                continue

            success, indexes = find_match(j, instr, [
                    "push _", "sub"])
            if success:
                v = int(instr[indexes[0]].split()[1])
                instr[indexes[0]] = "push "+str(-v)
                instr[indexes[1]] = "add"
                j = 0
                continue

            success, indexes = find_match(j, instr, [
                    "push _", "add", "push _", "add"])
            if success:
                v0 = int(instr[indexes[0]].split()[1])
                v1 = int(instr[indexes[2]].split()[1])
                instr[indexes[0]] = "push "+str(v0)
                instr[indexes[1]] = "push "+str(v1)
                instr[indexes[2]] = "add"
                instr[indexes[3]] = "add"
                j = 0
                continue

            success, indexes = find_match(j, instr, [
                    "push 0", "add"])
            if success:
                instr[indexes[0]] = "# push 0"
                instr[indexes[1]] = "# add"
                j = 0
                continue

            success, indexes = find_match(j, instr, [
                    "push _", "dup"])
            if success:
                v = int(instr[indexes[0]].split()[1])
                instr[indexes[0]] = "push "+str(v)
                instr[indexes[1]] = "push "+str(v)
                j = 0
                continue


            j += 1

        j = 0
        while j < len(instr):
            success, indexes = find_match(j, instr, [
                    "push _", "add"])
            if success:
                v = int(instr[indexes[0]].split()[1])
                if v < 0:
                    instr[indexes[0]] = "push "+str(-v)
                    instr[indexes[1]] = "sub"
                    j = 0
                    continue

            success, indexes = find_match(j, instr, [
                    "push 2", "push -1", "roll"])
            if success:
                instr[indexes[0]] = "push 2"
                instr[indexes[1]] = "push 1"
                instr[indexes[2]] = "roll"
                j = 0
                continue

            success, indexes = find_match(j, instr, [
                    "push _", "push _"])
            if success:
                v0 = int(instr[indexes[0]].split()[1])
                v1 = int(instr[indexes[1]].split()[1])
                if v0 == v1:
                    instr[indexes[0]] = "push "+str(v0)
                    instr[indexes[1]] = "dup"
                    j = 0
                    continue

            j += 1

        # instrs[instr_index] = (label, instr)
        instrs[instr_index] = label, list(filter(lambda x: x[0] != "#", instr))
        # instrs[instr_index] = label, list(filter(lambda x: x[:8] != "#delete ", instr))

    with open(o_file, 'w') as f:
        f.write("\n\n".join(["\n".join(["label " + label] + instr) for label, instr in instrs]))
