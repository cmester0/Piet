def cmd_interpreter(cmd, stack):
    match cmd[0]:
        case "push":
            stack.append(int(cmd[1]))
        case "pop":
            if len(stack) >= 1:
                stack.pop()
        case "add":
            if len(stack) >= 2:
                a = stack.pop()
                b = stack.pop()
                stack.append(a + b)
        case "sub":
            if len(stack) >= 2:
                a = stack.pop()
                b = stack.pop()
                stack.append(b - a)
        case "mul":
            if len(stack) >= 2:
                a = stack.pop()
                b = stack.pop()
                stack.append(a * b)
        case "div":
            if len(stack) >= 2:
                a = stack.pop()
                b = stack.pop()
                stack.append(b // a) # TODO: Ignore if div by zero
        case "mod":
            if len(stack) >= 2:
                a = stack.pop()
                b = stack.pop()
                stack.append(b % a)
        case "not":
            if len(stack) >= 1:
                a = stack.pop()
                stack.append(1 if a == 0 else 0)
        case "greater":
            if len(stack) >= 2:
                a = stack.pop()
                b = stack.pop()
                stack.append(1 if b > a else 0)
        case "dup":
            if len(stack) >= 1:
                a = stack.pop()
                stack.append(a)
                stack.append(a)
        case "roll":
            if len(stack) >= 2:
                a = stack.pop()
                b = stack.pop() # TODO: Ignore b negative
                if b < 0:
                    print ("Error")
                else:
                    a = a % b
                    stack = stack[:-b] + stack[-a:] + stack[-b:-a]
        case default:
            print("Invalid cmd:", cmd)
    return stack
