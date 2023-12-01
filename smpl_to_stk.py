import sys

inp_lines = []
with open(sys.argv[1], 'r') as f:
    inp_lines = f.readlines()
inp_lines = list(map(lambda x: x.split(), inp_lines))

instrs = [("main",["push 1"])]

data_type_size = {"num": 1, "list": 0}

def dup_value_x_deep(index, x):
    # Get the value to the top
    instrs[index][1].append("push "+str(x))
    instrs[index][1].append("push -1")
    instrs[index][1].append("roll")

    instrs[index][1].append("dup")

    # put it back
    instrs[index][1].append("push "+str(x+1))
    instrs[index][1].append("push 1")
    instrs[index][1].append("roll")

def swap(index):
    instrs[index][1].append("push 2")
    instrs[index][1].append("push 1")
    instrs[index][1].append("roll")

def add(index, n):
    instrs[index][1].append("push "+str(n))
    instrs[index][1].append("add")

def sub(index, n):
    instrs[index][1].append("push "+str(n))
    instrs[index][1].append("sub")

def eq(index):
    instrs[index][1].append("sub")
    instrs[index][1].append("dup")
    instrs[index][1].append("mul")
    instrs[index][1].append("push 0")
    instrs[index][1].append("greater")
    instrs[index][1].append("not")

def binop(index,op):
    instrs[index][1].append("push 3")
    instrs[index][1].append("push 1")
    instrs[index][1].append("roll")
    instrs[index][1].append(op)
    swap(index)
    instrs[index][1].append("push 1")
    instrs[index][1].append("sub")

index = 0
label_count = 0
var_list = []
for l in inp_lines:
    if len(l) == 0 or l[0] == "#":
        continue
    match l[0]:
        case "label":
            index = len(instrs)
            instrs.append((l[1], []))

        case "push":
            instrs[index][1].append("#+push" + l[1])
            instrs[index][1].append("push " + l[1])
            swap(index)
            add(index,1)
            instrs[index][1].append("#-push" + l[1])

        case "pop":
            instrs[index][1].append("#+pop")
            swap(index)
            instrs[index][1].append("pop")
            sub(index,1)
            instrs[index][1].append("#-pop")

        case "eq":
            instrs[index][1].append("#+eq")
            swap(index)
            instrs[index][1].append("push "+str(n))
            eq(index)
            swap(index)
            instrs[index][1].append("#-eq")

        case "add":
            instrs[index][1].append("#+add")
            binop(index,"add")
            instrs[index][1].append("#-add")

        case "sub":
            instrs[index][1].append("#+sub")
            binop(index,"sub")
            instrs[index][1].append("#-sub")

        case "div":
            instrs[index][1].append("#+div")
            binop(index,"div")
            instrs[index][1].append("#-div")

        case "mod":
            instrs[index][1].append("#+mod")
            binop(index,"mod")
            instrs[index][1].append("#-mod")

        case "mul":
            instrs[index][1].append("#+mul")
            binop(index,"mul")
            instrs[index][1].append("#-mul")

        case "dup":
            instrs[index][1].append("#+dup")
            swap(index)
            instrs[index][1].append("dup")
            instrs[index][1].append("push 3")
            instrs[index][1].append("push -1")
            instrs[index][1].append("roll")
            instrs[index][1].append("push 1")
            instrs[index][1].append("add")
            instrs[index][1].append("#-dup")

        case "inN":
            instrs[index][1].append("#+inN")
            label_index = len(instrs)
            new_label = "l" + str(label_index)
            instrs[index][1].append("goto " + new_label)

            instrs.append((new_label, []))
            instrs[label_index][1].append("push -2")
            instrs[label_index][1].append("push -3")
            instrs[label_index][1].append("inN")
            swap(label_index)

            # Is it -3 ?
            instrs[label_index][1].append("push -3")
            eq(label_index)

            succ_label_index = len(instrs)
            succ_new_label = "l" + str(succ_label_index)
            instrs.append((succ_new_label, []))

            fail_label_index = len(instrs)
            fail_new_label = "l" + str(fail_label_index)
            instrs.append((fail_new_label, []))
            instrs[label_index][1].append("branch " + succ_new_label + " " + fail_new_label)

            continue_label_index = len(instrs)
            continue_new_label = "l" + str(continue_label_index)
            instrs.append((continue_new_label, []))

            instrs[succ_label_index][1].append("push 3")
            instrs[succ_label_index][1].append("push 1")
            instrs[succ_label_index][1].append("roll")
            instrs[succ_label_index][1].append("pop")
            instrs[succ_label_index][1].append("push 1")
            instrs[succ_label_index][1].append("add")
            instrs[succ_label_index][1].append("goto " + continue_new_label)

            instrs[fail_label_index][1].append("pop")
            instrs[fail_label_index][1].append("goto " + continue_new_label)

            instrs[fail_label_index][1].append("#-inN")

            index = continue_label_index

        case "inC":
            instrs[index][1].append("#+inC")
            label_index = len(instrs)
            new_label = "l" + str(label_index)
            instrs[index][1].append("goto " + new_label)

            instrs.append((new_label, []))
            instrs[label_index][1].append("push -2")
            instrs[label_index][1].append("push -3")
            instrs[label_index][1].append("inC")
            swap(label_index)

            # Is it -3 ?
            instrs[label_index][1].append("push -3")
            eq(label_index)

            succ_label_index = len(instrs)
            succ_new_label = "l" + str(succ_label_index)
            instrs.append((succ_new_label, []))

            fail_label_index = len(instrs)
            fail_new_label = "l" + str(fail_label_index)
            instrs.append((fail_new_label, []))
            instrs[label_index][1].append("branch " + succ_new_label + " " + fail_new_label)

            continue_label_index = len(instrs)
            continue_new_label = "l" + str(continue_label_index)
            instrs.append((continue_new_label, []))

            instrs[succ_label_index][1].append("push 3")
            instrs[succ_label_index][1].append("push 1")
            instrs[succ_label_index][1].append("roll")
            instrs[succ_label_index][1].append("pop")
            instrs[succ_label_index][1].append("push 1")
            instrs[succ_label_index][1].append("add")
            instrs[succ_label_index][1].append("goto " + continue_new_label)

            instrs[fail_label_index][1].append("pop")
            instrs[fail_label_index][1].append("goto " + continue_new_label)

            instrs[fail_label_index][1].append("#-inC")

            index = continue_label_index

        case "goto":
            instrs[index][1].append("#+goto " + l[1])
            instrs[index][1].append("goto " + l[1])
            instrs[index][1].append("#-goto " + l[1])

        case "branch":
            instrs[index][1].append("#+branch" + l[1] + " " + l[2])
            instrs[index][1].append("push 1")
            instrs[index][1].append("sub")
            swap(index)
            instrs[index][1].append("branch " + l[1] + " " + l[2])
            instrs[index][1].append("#-branch " + l[1] + " " + l[2])

        case "var":
            instrs[index][1].append("#+var " + l[1])
            var_list = [l[1]] + var_list
            print (var_list)

            # Allocate empty variable
            instrs[index][1].append("push " + str(data_type_size[l[2]]))
            for _ in range(data_type_size[l[2]]):
                instrs[index][1].append("push 0")
            instrs[index][1].append("push " + str(data_type_size[l[2]]))
            new_data_size = 2 + data_type_size[l[2]]

            ####################
            # Fetch stack size #
            ####################

            instrs[index][1].append("push " + str(new_data_size+1))
            instrs[index][1].append("push -1")
            instrs[index][1].append("roll")

            # Update size
            instrs[index][1].append("push " + str(new_data_size))
            instrs[index][1].append("add")

            instrs[index][1].append("dup")

            instrs[index][1].append("push " + str(new_data_size+2))
            instrs[index][1].append("push 1")
            instrs[index][1].append("roll")

            ######################
            # Rotate into bottom #
            ######################

            instrs[index][1].append("push " + str(new_data_size))
            instrs[index][1].append("roll")

            instrs[index][1].append("#-var " + l[1])

        case "set":
            instrs[index][1].append("#+set " + l[1])
            var_index = 0
            for i, x in enumerate(var_list):
                if x == l[1]:
                    var_index = i
                    break
            else:
                print ("Variable", l[1], "was not defined")
                exit(1)

            if var_index == 0:
                instrs[index][1].append("dup")
                instrs[index][1].append("push -2") # TODO: change to index / varsize
                instrs[index][1].append("roll")
                instrs[index][1].append("pop")
                instrs[index][1].append("push 3")
                instrs[index][1].append("push -2")
                instrs[index][1].append("roll")
                instrs[index][1].append("dup")
                instrs[index][1].append("push 4")
                instrs[index][1].append("push 1")
                instrs[index][1].append("roll")
                instrs[index][1].append("push 2") # TODO: change to index / varsize
                instrs[index][1].append("roll")
                instrs[index][1].append("push 1")
                instrs[index][1].append("sub")

                instrs[index][1].append("#-set " + l[1])

            elif len(var_list) < (var_index+2):
                print ("TODO:", len(var_list), "<", (var_index+2))

                # Var index
                instrs[index][1].append("push " + str(var_index))

                label_index = len(instrs)
                new_label = "l" + str(label_index)
                instrs[index][1].append("goto " + new_label)

                instrs.append((new_label, []))

                instrs[label_index][1].append("dup")

                less_branch_index = len(instrs)
                new_label_1 = "l" + str(less_branch_index)
                instrs.append((new_label_1, []))

                greater_branch_index = len(instrs)
                new_label_2 = "l" + str(greater_branch_index)
                instrs.append((new_label_2, []))

                instrs[label_index][1].append("branch " + new_label_1 + " " + new_label_2)

                swap(less_branch_index)

                instrs[less_branch_index][1].append("dup")

                instrs[less_branch_index][1].append("push 3")
                instrs[less_branch_index][1].append("push 1")
                instrs[less_branch_index][1].append("roll")

                # TODO: handle case of lists
                instrs[less_branch_index][1].append("push 1")
                instrs[less_branch_index][1].append("add")
                instrs[less_branch_index][1].append("push -3")
                instrs[less_branch_index][1].append("roll")

                instrs[less_branch_index][1].append("push 6")
                instrs[less_branch_index][1].append("push -3")
                instrs[less_branch_index][1].append("roll")

                instrs[less_branch_index][1].append("push 1")
                instrs[less_branch_index][1].append("sub")

                instrs[less_branch_index][1].append("goto " + new_label)

                instrs[greater_branch_index][1].append("pop")
                instrs[greater_branch_index][1].append("dup")
                # TODO: handle case of lists
                instrs[greater_branch_index][1].append("push -2")
                instrs[greater_branch_index][1].append("roll")
                instrs[greater_branch_index][1].append("pop")

                instrs[greater_branch_index][1].append("push 3")
                instrs[greater_branch_index][1].append("push -1")
                instrs[greater_branch_index][1].append("roll")

                instrs[greater_branch_index][1].append("push 3")
                instrs[greater_branch_index][1].append("push -1")
                instrs[greater_branch_index][1].append("roll")
                instrs[greater_branch_index][1].append("dup")
                instrs[greater_branch_index][1].append("push 4")
                instrs[greater_branch_index][1].append("push 1")
                instrs[greater_branch_index][1].append("roll")

                swap(greater_branch_index)
                instrs[greater_branch_index][1].append("push 1")
                instrs[greater_branch_index][1].append("sub")

                instrs[greater_branch_index][1].append("#-set " + l[1])

                index = greater_branch_index
            else:
                instrs[index][1].append("push 0") # Heap size
                instrs[index][1].append("push " + str(len(var_list) - (var_index+2))) # Second round
                instrs[index][1].append("push " + str(var_index+1)) # First round
                label_index = len(instrs)
                new_label = "l" + str(label_index)
                instrs[index][1].append("goto " + new_label)

                instrs.append((new_label, []))

                ##################
                # Fetch stk size #
                ##################

                instrs[label_index][1].append("push 4")
                instrs[label_index][1].append("push -1")
                instrs[label_index][1].append("roll")

                # Add 3
                instrs[label_index][1].append("push 3")
                instrs[label_index][1].append("add")
                instrs[label_index][1].append("dup")

                # Roll stack one backwards
                instrs[label_index][1].append("push -1")
                instrs[label_index][1].append("roll")

                # Fetch size
                instrs[label_index][1].append("dup")

                instrs[label_index][1].append("push 3")
                instrs[label_index][1].append("push -1")
                instrs[label_index][1].append("roll")

                instrs[label_index][1].append("dup")

                instrs[label_index][1].append("push 3")
                instrs[label_index][1].append("push -1")
                instrs[label_index][1].append("roll")

                instrs[label_index][1].append("push -1")
                swap(label_index)
                instrs[label_index][1].append("sub")

                instrs[label_index][1].append("roll")

                # Fetch stack size
                instrs[label_index][1].append("dup")
                instrs[label_index][1].append("push 2")
                instrs[label_index][1].append("add")
                instrs[label_index][1].append("push -1")
                instrs[label_index][1].append("roll")
                instrs[label_index][1].append("push 3")
                instrs[label_index][1].append("sub")

                swap(label_index)
                instrs[label_index][1].append("dup")
                instrs[label_index][1].append("push 6")
                instrs[label_index][1].append("add")
                instrs[label_index][1].append("push -3")
                instrs[label_index][1].append("roll")
                instrs[label_index][1].append("push 4")
                instrs[label_index][1].append("push -1")
                instrs[label_index][1].append("roll")

                # Increment heap size counter
                instrs[label_index][1].append("dup")
                instrs[label_index][1].append("push 5")
                instrs[label_index][1].append("push -1")
                instrs[label_index][1].append("roll")
                instrs[label_index][1].append("add")
                instrs[label_index][1].append("push 2")
                instrs[label_index][1].append("add")
                instrs[label_index][1].append("push 4")
                instrs[label_index][1].append("push 1")
                instrs[label_index][1].append("roll")

                # Continue
                instrs[label_index][1].append("push 5")
                instrs[label_index][1].append("push 1")
                instrs[label_index][1].append("roll")

                instrs[label_index][1].append("push 1")
                instrs[label_index][1].append("sub")

                instrs[label_index][1].append("dup")
                instrs[label_index][1].append("push 0")
                instrs[label_index][1].append("greater")
                next_label_index = len(instrs)
                next_new_label = "l" + str(next_label_index)

                instrs[label_index][1].append("branch " + new_label + " " + next_new_label)

                instrs.append((next_new_label, []))
                instrs[next_label_index][1].append("pop")

                instrs[next_label_index][1].append("push 3")
                instrs[next_label_index][1].append("push -1")
                instrs[next_label_index][1].append("roll")
                instrs[next_label_index][1].append("dup")
                instrs[next_label_index][1].append("push 4")
                instrs[next_label_index][1].append("push 1")
                instrs[next_label_index][1].append("roll")

                instrs[next_label_index][1].append("push 3")
                instrs[next_label_index][1].append("push -1")
                instrs[next_label_index][1].append("roll")
                instrs[next_label_index][1].append("dup")
                instrs[next_label_index][1].append("push 4")
                instrs[next_label_index][1].append("push 1")
                instrs[next_label_index][1].append("roll")

                instrs[next_label_index][1].append("sub")
                instrs[next_label_index][1].append("push 2")
                instrs[next_label_index][1].append("add")
                instrs[next_label_index][1].append("push -1")
                instrs[next_label_index][1].append("roll")

                instrs[next_label_index][1].append("push 6")
                instrs[next_label_index][1].append("push -1")
                instrs[next_label_index][1].append("roll")
                instrs[next_label_index][1].append("pop")

                instrs[next_label_index][1].append("push 5")
                instrs[next_label_index][1].append("push 1")
                instrs[next_label_index][1].append("roll")

                instrs[next_label_index][1].append("push 3")
                instrs[next_label_index][1].append("push -1")
                instrs[next_label_index][1].append("roll")

                instrs[next_label_index][1].append("push 1")
                instrs[next_label_index][1].append("sub")

                instrs[next_label_index][1].append("push 3")
                instrs[next_label_index][1].append("push 1")
                instrs[next_label_index][1].append("roll")

                # Continue

                final_label_index = len(instrs)
                final_new_label = "l" + str(final_label_index)
                instrs[next_label_index][1].append("goto " + final_new_label)

                instrs.append((final_new_label, []))

                ##################
                # Fetch stk size #
                ##################

                instrs[final_label_index][1].append("push 3")
                instrs[final_label_index][1].append("push -1")
                instrs[final_label_index][1].append("roll")

                instrs[final_label_index][1].append("push 2")
                instrs[final_label_index][1].append("add")
                instrs[final_label_index][1].append("dup")

                # Roll stack one backwards
                instrs[final_label_index][1].append("push -1")
                instrs[final_label_index][1].append("roll")

                # fetch size
                instrs[final_label_index][1].append("dup")

                instrs[final_label_index][1].append("push 3")
                instrs[final_label_index][1].append("push -1")
                instrs[final_label_index][1].append("roll")

                instrs[final_label_index][1].append("dup")

                instrs[final_label_index][1].append("push 3")
                instrs[final_label_index][1].append("push 1")
                instrs[final_label_index][1].append("roll")

                swap(final_label_index)

                instrs[final_label_index][1].append("push -1")
                swap(final_label_index)
                instrs[final_label_index][1].append("sub")

                instrs[final_label_index][1].append("roll")

                # Increase heap size counter
                instrs[final_label_index][1].append("dup")
                instrs[final_label_index][1].append("dup")
                instrs[final_label_index][1].append("push 7")
                instrs[final_label_index][1].append("add")
                instrs[final_label_index][1].append("dup")
                instrs[final_label_index][1].append("push -1")
                instrs[final_label_index][1].append("roll")
                instrs[final_label_index][1].append("push 3")
                instrs[final_label_index][1].append("push -1")
                instrs[final_label_index][1].append("roll")
                instrs[final_label_index][1].append("add")
                instrs[final_label_index][1].append("push 2")
                instrs[final_label_index][1].append("add")
                swap(final_label_index)
                instrs[final_label_index][1].append("push 2")
                instrs[final_label_index][1].append("sub")
                instrs[final_label_index][1].append("push 1")
                instrs[final_label_index][1].append("roll")

                # Fetch stack size
                instrs[final_label_index][1].append("dup")
                instrs[final_label_index][1].append("push 2")
                instrs[final_label_index][1].append("add")
                instrs[final_label_index][1].append("push -1")
                instrs[final_label_index][1].append("roll")

                swap(final_label_index)

                instrs[final_label_index][1].append("dup")
                instrs[final_label_index][1].append("push 5")
                instrs[final_label_index][1].append("add")
                instrs[final_label_index][1].append("push -2")
                instrs[final_label_index][1].append("roll")
                instrs[final_label_index][1].append("push 4")
                instrs[final_label_index][1].append("push -1")
                instrs[final_label_index][1].append("roll")

                instrs[final_label_index][1].append("push 1")
                instrs[final_label_index][1].append("sub")

                instrs[final_label_index][1].append("push 3")
                instrs[final_label_index][1].append("push 1")
                instrs[final_label_index][1].append("roll")

                instrs[final_label_index][1].append("push 1")
                instrs[final_label_index][1].append("sub")

                instrs[final_label_index][1].append("dup")
                instrs[final_label_index][1].append("push 0")

                instrs[final_label_index][1].append("greater")

                index = len(instrs)
                new_label = "l" + str(index)
                instrs[final_label_index][1].append("branch " + final_new_label + " " + new_label)

                instrs.append((new_label, []))

                # TODO ROTATE HERE!
                instrs[index][1].append("pop")
                instrs[index][1].append("dup")
                instrs[index][1].append("push 3")
                instrs[index][1].append("push -1")
                instrs[index][1].append("roll")
                instrs[index][1].append("dup")
                instrs[index][1].append("push 3")
                instrs[index][1].append("push 1")
                instrs[index][1].append("roll")
                swap(index)
                instrs[index][1].append("push 3")
                instrs[index][1].append("add")
                instrs[index][1].append("push 1")
                instrs[index][1].append("roll")
                swap(index)
                instrs[index][1].append("roll")

                instrs[index][1].append("push 1")
                instrs[index][1].append("sub")

                instrs[index][1].append("#-set " + l[1])

        case "get":
            instrs[index][1].append("#+get " + l[1])
            var_index = 0
            for i, x in enumerate(var_list):
                if x == l[1]:
                    var_index = i
                    break
            else:
                print ("Variable", l[1], "was not defined")
                exit(1)

            if var_index == 0:
                instrs[index][1].append("dup")
                instrs[index][1].append("push -2") # TODO: change to index / varsize
                instrs[index][1].append("roll")
                instrs[index][1].append("dup")
                instrs[index][1].append("push 4")
                instrs[index][1].append("push 1")
                instrs[index][1].append("roll")
                instrs[index][1].append("push 3")
                instrs[index][1].append("push -1")
                instrs[index][1].append("roll")
                instrs[index][1].append("push 1")
                instrs[index][1].append("add")
                instrs[index][1].append("dup")
                instrs[index][1].append("push 4")
                instrs[index][1].append("push 1")
                instrs[index][1].append("roll")
                instrs[index][1].append("push 2") # TODO: change to index / varsize
                instrs[index][1].append("roll")

                instrs[index][1].append("#-get " + l[1])

            elif len(var_list) < (var_index+2):
                print ("TODO")
                instrs[index][1].append("#-get " + l[1])
            else:
                instrs[index][1].append("push 0") # Heap size
                instrs[index][1].append("push " + str(len(var_list) - (var_index+2))) # Second round
                instrs[index][1].append("push " + str(var_index+1)) # First round
                label_index = len(instrs)
                new_label = "l" + str(label_index)
                instrs[index][1].append("goto " + new_label)

                instrs.append((new_label, []))

                ##################
                # Fetch stk size #
                ##################

                instrs[label_index][1].append("push 4")
                instrs[label_index][1].append("push -1")
                instrs[label_index][1].append("roll")

                # Add 3
                instrs[label_index][1].append("push 3")
                instrs[label_index][1].append("add")
                instrs[label_index][1].append("dup")

                # Roll stack one backwards
                instrs[label_index][1].append("push -1")
                instrs[label_index][1].append("roll")

                # Fetch size
                instrs[label_index][1].append("dup")

                instrs[label_index][1].append("push 3")
                instrs[label_index][1].append("push -1")
                instrs[label_index][1].append("roll")

                instrs[label_index][1].append("dup")

                instrs[label_index][1].append("push 3")
                instrs[label_index][1].append("push -1")
                instrs[label_index][1].append("roll")

                instrs[label_index][1].append("push -1")
                swap(label_index)
                instrs[label_index][1].append("sub")

                instrs[label_index][1].append("roll")

                # Fetch stack size
                instrs[label_index][1].append("dup")
                instrs[label_index][1].append("push 2")
                instrs[label_index][1].append("add")
                instrs[label_index][1].append("push -1")
                instrs[label_index][1].append("roll")
                instrs[label_index][1].append("push 3")
                instrs[label_index][1].append("sub")

                instrs[label_index][1].append("push 2")
                instrs[label_index][1].append("push 1")
                instrs[label_index][1].append("roll")
                instrs[label_index][1].append("dup")
                instrs[label_index][1].append("push 6")
                instrs[label_index][1].append("add")
                instrs[label_index][1].append("push -3")
                instrs[label_index][1].append("roll")
                instrs[label_index][1].append("push 4")
                instrs[label_index][1].append("push -1")
                instrs[label_index][1].append("roll")

                # Increment heap size counter
                instrs[label_index][1].append("dup")
                instrs[label_index][1].append("push 5")
                instrs[label_index][1].append("push -1")
                instrs[label_index][1].append("roll")
                instrs[label_index][1].append("add")
                instrs[label_index][1].append("push 2")
                instrs[label_index][1].append("add")
                instrs[label_index][1].append("push 4")
                instrs[label_index][1].append("push 1")
                instrs[label_index][1].append("roll")

                # Continue
                instrs[label_index][1].append("push 5")
                instrs[label_index][1].append("push 1")
                instrs[label_index][1].append("roll")

                instrs[label_index][1].append("push 1")
                instrs[label_index][1].append("sub")

                instrs[label_index][1].append("dup")
                instrs[label_index][1].append("push 0")
                instrs[label_index][1].append("greater")
                next_label_index = len(instrs)
                next_new_label = "l" + str(next_label_index)

                instrs[label_index][1].append("branch " + new_label + " " + next_new_label)

                instrs.append((next_new_label, []))
                instrs[next_label_index][1].append("pop")

                # Get Value HERE # TODO: FOR LIST
                instrs[next_label_index][1].append("push 4")
                instrs[next_label_index][1].append("push -1")
                instrs[next_label_index][1].append("roll")
                instrs[next_label_index][1].append("dup")
                instrs[next_label_index][1].append("push 5")
                instrs[next_label_index][1].append("push 1")
                instrs[next_label_index][1].append("roll")
                instrs[next_label_index][1].append("push 5")
                instrs[next_label_index][1].append("add")
                instrs[next_label_index][1].append("dup")
                instrs[next_label_index][1].append("push -1")
                instrs[next_label_index][1].append("roll")
                instrs[next_label_index][1].append("dup")
                instrs[next_label_index][1].append("push 3")
                instrs[next_label_index][1].append("push -1")
                instrs[next_label_index][1].append("roll")
                instrs[next_label_index][1].append("push 1")
                instrs[next_label_index][1].append("roll")

                instrs[next_label_index][1].append("push 4")
                instrs[next_label_index][1].append("push -1")
                instrs[next_label_index][1].append("roll")
                instrs[next_label_index][1].append("dup")
                instrs[next_label_index][1].append("push 5")
                instrs[next_label_index][1].append("push 1")
                instrs[next_label_index][1].append("roll")

                instrs[next_label_index][1].append("push 4")
                instrs[next_label_index][1].append("push -1")
                instrs[next_label_index][1].append("roll")
                instrs[next_label_index][1].append("dup")
                instrs[next_label_index][1].append("push 5")
                instrs[next_label_index][1].append("push 1")
                instrs[next_label_index][1].append("roll")
                instrs[next_label_index][1].append("sub")
                instrs[next_label_index][1].append("push 2")
                instrs[next_label_index][1].append("add")
                instrs[next_label_index][1].append("push 1")
                instrs[next_label_index][1].append("roll")

                # CONTINUE ?

                instrs[next_label_index][1].append("push 3")
                instrs[next_label_index][1].append("push -1")
                instrs[next_label_index][1].append("roll")

                instrs[next_label_index][1].append("push 1")
                instrs[next_label_index][1].append("add")

                instrs[next_label_index][1].append("push 3")
                instrs[next_label_index][1].append("push 1")
                instrs[next_label_index][1].append("roll")

                # instrs[next_label_index][1].append("goto term")

                # Continue

                final_label_index = len(instrs)
                final_new_label = "l" + str(final_label_index)
                instrs[next_label_index][1].append("goto " + final_new_label)

                instrs.append((final_new_label, []))

                ##################
                # Fetch stk size #
                ##################

                instrs[final_label_index][1].append("push 3")
                instrs[final_label_index][1].append("push -1")
                instrs[final_label_index][1].append("roll")

                instrs[final_label_index][1].append("push 2")
                instrs[final_label_index][1].append("add")
                instrs[final_label_index][1].append("dup")

                # Roll stack one backwards
                instrs[final_label_index][1].append("push -1")
                instrs[final_label_index][1].append("roll")

                # fetch size
                instrs[final_label_index][1].append("dup")

                instrs[final_label_index][1].append("push 3")
                instrs[final_label_index][1].append("push -1")
                instrs[final_label_index][1].append("roll")

                instrs[final_label_index][1].append("dup")

                instrs[final_label_index][1].append("push 3")
                instrs[final_label_index][1].append("push 1")
                instrs[final_label_index][1].append("roll")

                swap(final_label_index)

                instrs[final_label_index][1].append("push -1")
                swap(final_label_index)
                instrs[final_label_index][1].append("sub")

                instrs[final_label_index][1].append("roll")

                # Increase heap size counter
                instrs[final_label_index][1].append("dup")
                instrs[final_label_index][1].append("dup")
                instrs[final_label_index][1].append("push 7")
                instrs[final_label_index][1].append("add")
                instrs[final_label_index][1].append("dup")
                instrs[final_label_index][1].append("push -1")
                instrs[final_label_index][1].append("roll")
                instrs[final_label_index][1].append("push 3")
                instrs[final_label_index][1].append("push -1")
                instrs[final_label_index][1].append("roll")
                instrs[final_label_index][1].append("add")
                instrs[final_label_index][1].append("push 2")
                instrs[final_label_index][1].append("add")
                instrs[final_label_index][1].append("push 2")
                instrs[final_label_index][1].append("push 1")
                instrs[final_label_index][1].append("roll")
                instrs[final_label_index][1].append("push 2")
                instrs[final_label_index][1].append("sub")
                instrs[final_label_index][1].append("push 1")
                instrs[final_label_index][1].append("roll")

                # Fetch stack size
                instrs[final_label_index][1].append("dup")
                instrs[final_label_index][1].append("push 2")
                instrs[final_label_index][1].append("add")
                instrs[final_label_index][1].append("push -1")
                instrs[final_label_index][1].append("roll")

                instrs[final_label_index][1].append("push 2")
                instrs[final_label_index][1].append("push 1")
                instrs[final_label_index][1].append("roll")

                instrs[final_label_index][1].append("dup")
                instrs[final_label_index][1].append("push 5")
                instrs[final_label_index][1].append("add")
                instrs[final_label_index][1].append("push -2")
                instrs[final_label_index][1].append("roll")
                instrs[final_label_index][1].append("push 4")
                instrs[final_label_index][1].append("push -1")
                instrs[final_label_index][1].append("roll")

                instrs[final_label_index][1].append("push 1")
                instrs[final_label_index][1].append("sub")

                instrs[final_label_index][1].append("push 3")
                instrs[final_label_index][1].append("push 1")
                instrs[final_label_index][1].append("roll")

                instrs[final_label_index][1].append("push 1")
                instrs[final_label_index][1].append("sub")

                instrs[final_label_index][1].append("dup")
                instrs[final_label_index][1].append("push 0")

                instrs[final_label_index][1].append("greater")

                index = len(instrs)
                new_label = "l" + str(index)
                instrs[final_label_index][1].append("branch " + final_new_label + " " + new_label)

                instrs.append((new_label, []))

                # TODO ROTATE HERE!
                instrs[index][1].append("pop")
                instrs[index][1].append("dup")
                instrs[index][1].append("push 3")
                instrs[index][1].append("push -1")
                instrs[index][1].append("roll")
                instrs[index][1].append("dup")
                instrs[index][1].append("push 3")
                instrs[index][1].append("push 1")
                instrs[index][1].append("roll")
                swap(index)
                instrs[index][1].append("push 3")
                instrs[index][1].append("add")
                instrs[index][1].append("push 1")
                instrs[index][1].append("roll")
                swap(index)
                instrs[index][1].append("roll")

                instrs[index][1].append("push 1")
                instrs[index][1].append("sub")
                instrs[index][1].append("#-get " + l[1])

        case "del":
            instrs[index][1].append("#+del " + l[1])
            var_index = 0
            for i, x in enumerate(var_list):
                if x == l[1]:
                    var_index = i
                    break
            else:
                print ("Variable", l[1], "was not defined")
                exit(1)
            del var_list[var_index]

            if var_index == 0:
                instrs[index][1].append("push 2")
                instrs[index][1].append("sub")
                instrs[index][1].append("dup")
                instrs[index][1].append("push 1")
                instrs[index][1].append("add")
                instrs[index][1].append("dup")
                instrs[index][1].append("push 2")
                instrs[index][1].append("add")
                instrs[index][1].append("push -1")
                instrs[index][1].append("roll")
                instrs[index][1].append("push -1")
                swap(index)
                instrs[index][1].append("sub")
                instrs[index][1].append("roll")

                instrs[index][1].append("dup")
                instrs[index][1].append("dup")
                instrs[index][1].append("push 2")
                instrs[index][1].append("add")
                instrs[index][1].append("push 1")
                instrs[index][1].append("roll")

                instrs[index][1].append("dup")
                pop_label_index = len(instrs)
                pop_new_label = "l" + str(pop_label_index)
                instrs.append((pop_new_label, []))

                done_label_index = len(instrs)
                done_new_label = "l" + str(done_label_index)
                instrs.append((done_new_label, []))

                instrs[index][1].append("branch " + pop_new_label + " " + done_new_label)

                # Pop while top is less than one

                swap(pop_label_index)
                instrs[pop_label_index][1].append("pop")
                instrs[pop_label_index][1].append("push 1")
                instrs[pop_label_index][1].append("sub")
                instrs[pop_label_index][1].append("dup")
                instrs[pop_label_index][1].append("branch " + pop_new_label + " " + done_new_label)

                instrs[done_label_index][1].append("pop")
                instrs[done_label_index][1].append("sub")

                index = done_label_index

                instrs[done_label_index][1].append("#-del " + l[1])

            elif len(var_list) < (var_index + 1):
                instrs[done_label_index][1].append("#-del " + l[1])
                print("TODO")
            else:
                instrs[index][1].append("push 0") # Heap size
                instrs[index][1].append("push " + str(len(var_list) - (var_index + 1))) # Second round
                instrs[index][1].append("push " + str(var_index+1)) # First round
                label_index = len(instrs)
                new_label = "l" + str(label_index)
                instrs[index][1].append("goto " + new_label)

                instrs.append((new_label, []))

                ##################
                # Fetch stk size #
                ##################

                instrs[label_index][1].append("push 4")
                instrs[label_index][1].append("push -1")
                instrs[label_index][1].append("roll")

                # Add 3
                instrs[label_index][1].append("push 3")
                instrs[label_index][1].append("add")
                instrs[label_index][1].append("dup")

                # Roll stack one backwards
                instrs[label_index][1].append("push -1")
                instrs[label_index][1].append("roll")

                # Fetch size
                instrs[label_index][1].append("dup")

                instrs[label_index][1].append("push 3")
                instrs[label_index][1].append("push -1")
                instrs[label_index][1].append("roll")

                instrs[label_index][1].append("dup")

                instrs[label_index][1].append("push 3")
                instrs[label_index][1].append("push -1")
                instrs[label_index][1].append("roll")

                instrs[label_index][1].append("push -1")
                swap(label_index)
                instrs[label_index][1].append("sub")

                instrs[label_index][1].append("roll")

                # Fetch stack size
                instrs[label_index][1].append("dup")
                instrs[label_index][1].append("push 2")
                instrs[label_index][1].append("add")
                instrs[label_index][1].append("push -1")
                instrs[label_index][1].append("roll")
                instrs[label_index][1].append("push 3")
                instrs[label_index][1].append("sub")

                instrs[label_index][1].append("push 2")
                instrs[label_index][1].append("push 1")
                instrs[label_index][1].append("roll")
                instrs[label_index][1].append("dup")
                instrs[label_index][1].append("push 6")
                instrs[label_index][1].append("add")
                instrs[label_index][1].append("push -3")
                instrs[label_index][1].append("roll")
                instrs[label_index][1].append("push 4")
                instrs[label_index][1].append("push -1")
                instrs[label_index][1].append("roll")

                # Increment heap size counter
                instrs[label_index][1].append("dup")
                instrs[label_index][1].append("push 5")
                instrs[label_index][1].append("push -1")
                instrs[label_index][1].append("roll")
                instrs[label_index][1].append("add")
                instrs[label_index][1].append("push 2")
                instrs[label_index][1].append("add")
                instrs[label_index][1].append("push 4")
                instrs[label_index][1].append("push 1")
                instrs[label_index][1].append("roll")

                # Continue
                instrs[label_index][1].append("push 5")
                instrs[label_index][1].append("push 1")
                instrs[label_index][1].append("roll")

                instrs[label_index][1].append("push 1")
                instrs[label_index][1].append("sub")

                instrs[label_index][1].append("dup")
                instrs[label_index][1].append("push 0")
                instrs[label_index][1].append("greater")
                next_label_index = len(instrs)
                next_new_label = "l" + str(next_label_index)

                instrs[label_index][1].append("branch " + new_label + " " + next_new_label)

                instrs.append((next_new_label, []))
                instrs[next_label_index][1].append("pop")

                instrs[next_label_index][1].append("push 4")
                instrs[next_label_index][1].append("push -1")
                instrs[next_label_index][1].append("roll")

                # Decrement Heap size counter (by deleted element size)
                instrs[next_label_index][1].append("dup")
                instrs[next_label_index][1].append("push 4")
                instrs[next_label_index][1].append("push -1")
                instrs[next_label_index][1].append("roll")
                swap(next_label_index)
                instrs[next_label_index][1].append("sub")
                instrs[next_label_index][1].append("push 2")
                instrs[next_label_index][1].append("sub")
                instrs[next_label_index][1].append("push 3")
                instrs[next_label_index][1].append("push 1")
                instrs[next_label_index][1].append("roll")

                # Continue
                instrs[next_label_index][1].append("push 1")
                instrs[next_label_index][1].append("add")
                pop_label_index = len(instrs)
                pop_new_label = "l" + str(pop_label_index)
                instrs[next_label_index][1].append("goto " + pop_new_label)

                instrs.append((pop_new_label, []))
                instrs[pop_label_index][1].append("push 5")
                instrs[pop_label_index][1].append("push -1")
                instrs[pop_label_index][1].append("roll")
                instrs[pop_label_index][1].append("pop")
                instrs[pop_label_index][1].append("push 1")
                instrs[pop_label_index][1].append("sub")
                instrs[pop_label_index][1].append("push 4")
                instrs[pop_label_index][1].append("push -1")
                instrs[pop_label_index][1].append("roll")
                instrs[pop_label_index][1].append("push 1")
                instrs[pop_label_index][1].append("sub")
                instrs[pop_label_index][1].append("push 4")
                instrs[pop_label_index][1].append("push 1")
                instrs[pop_label_index][1].append("roll")
                instrs[pop_label_index][1].append("dup")
                instrs[pop_label_index][1].append("push 0")
                instrs[pop_label_index][1].append("greater")
                reset_label_index = len(instrs)
                reset_new_label = "l" + str(reset_label_index)
                instrs[pop_label_index][1].append("branch " + pop_new_label + " "  + reset_new_label)

                instrs.append((reset_new_label, []))
                instrs[reset_label_index][1].append("pop")

                final_label_index = len(instrs)
                final_new_label = "l" + str(final_label_index)
                instrs[reset_label_index][1].append("goto " + final_new_label)

                instrs.append((final_new_label, []))

                ##################
                # Fetch stk size #
                ##################

                instrs[final_label_index][1].append("push 3")
                instrs[final_label_index][1].append("push -1")
                instrs[final_label_index][1].append("roll")

                instrs[final_label_index][1].append("push 1")
                instrs[final_label_index][1].append("add")
                instrs[final_label_index][1].append("dup")

                # Roll stack one backwards
                instrs[final_label_index][1].append("push -1")
                instrs[final_label_index][1].append("roll")

                # fetch size
                instrs[final_label_index][1].append("dup")

                instrs[final_label_index][1].append("push 3")
                instrs[final_label_index][1].append("push -1")
                instrs[final_label_index][1].append("roll")

                instrs[final_label_index][1].append("dup")

                instrs[final_label_index][1].append("push 3")
                instrs[final_label_index][1].append("push 1")
                instrs[final_label_index][1].append("roll")

                swap(final_label_index)

                instrs[final_label_index][1].append("push -1")
                instrs[final_label_index][1].append("push 2")
                instrs[final_label_index][1].append("push 1")
                instrs[final_label_index][1].append("roll")
                instrs[final_label_index][1].append("sub")

                instrs[final_label_index][1].append("roll")

                # Increase heap size counter
                instrs[final_label_index][1].append("dup")
                instrs[final_label_index][1].append("dup")
                instrs[final_label_index][1].append("push 7")
                instrs[final_label_index][1].append("add")
                instrs[final_label_index][1].append("dup")
                instrs[final_label_index][1].append("push -1")
                instrs[final_label_index][1].append("roll")
                instrs[final_label_index][1].append("push 3")
                instrs[final_label_index][1].append("push -1")
                instrs[final_label_index][1].append("roll")
                instrs[final_label_index][1].append("add")
                instrs[final_label_index][1].append("push 2")
                instrs[final_label_index][1].append("add")
                instrs[final_label_index][1].append("push 2")
                instrs[final_label_index][1].append("push 1")
                instrs[final_label_index][1].append("roll")
                instrs[final_label_index][1].append("push 2")
                instrs[final_label_index][1].append("sub")
                instrs[final_label_index][1].append("push 1")
                instrs[final_label_index][1].append("roll")

                # Fetch stack size
                instrs[final_label_index][1].append("dup")
                instrs[final_label_index][1].append("push 2")
                instrs[final_label_index][1].append("add")
                instrs[final_label_index][1].append("push -1")
                instrs[final_label_index][1].append("roll")

                swap(final_label_index)

                instrs[final_label_index][1].append("dup")
                instrs[final_label_index][1].append("push 5")
                instrs[final_label_index][1].append("add")
                instrs[final_label_index][1].append("push -2")
                instrs[final_label_index][1].append("roll")
                instrs[final_label_index][1].append("push 4")
                instrs[final_label_index][1].append("push -1")
                instrs[final_label_index][1].append("roll")

                instrs[final_label_index][1].append("push 1")
                instrs[final_label_index][1].append("sub")

                instrs[final_label_index][1].append("push 3")
                instrs[final_label_index][1].append("push 1")
                instrs[final_label_index][1].append("roll")

                instrs[final_label_index][1].append("push 1")
                instrs[final_label_index][1].append("sub")

                instrs[final_label_index][1].append("dup")
                instrs[final_label_index][1].append("push 0")

                instrs[final_label_index][1].append("greater")

                index = len(instrs)
                new_label = "l" + str(index)
                instrs[final_label_index][1].append("branch " + final_new_label + " " + new_label)

                instrs.append((new_label, []))

                # TODO ROTATE HERE!
                instrs[index][1].append("pop")
                instrs[index][1].append("dup")
                instrs[index][1].append("push 3")
                instrs[index][1].append("push -1")
                instrs[index][1].append("roll")
                instrs[index][1].append("dup")
                instrs[index][1].append("push 3")
                instrs[index][1].append("push 1")
                instrs[index][1].append("roll")
                swap(index)
                instrs[index][1].append("push 3")
                instrs[index][1].append("add")
                instrs[index][1].append("push 1")
                instrs[index][1].append("roll")
                swap(index)
                instrs[index][1].append("roll")

                instrs[index][1].append("push 1")
                instrs[index][1].append("sub")

                instrs[index][1].append("#-del " + l[1])

        case "outC":
            instrs[index][1].append("#+outC")
            swap(index)
            instrs[index][1].append("outC")
            instrs[index][1].append("push 1")
            instrs[index][1].append("sub")
            instrs[index][1].append("#-outC")

        case "outN":
            instrs[index][1].append("#+outN")
            swap(index)
            instrs[index][1].append("outN")
            instrs[index][1].append("push 1")
            instrs[index][1].append("sub")
            instrs[index][1].append("#-outN")

# print (var_list)
# print ("\n".join(instrs))

with open(sys.argv[2], 'w') as f:
    f.write("\n\n".join(["\n".join(["label " + label] + instr) for label, instr in instrs]))
