def part1(lines):
    points = {
        ')': 3,
        ']': 57,
        '}': 1197,
        '>': 25137
    }
    opening_brackets = { '(', '[', '{', '<' }
    corresponding_brackets = {
        ')': '(',
        ']': '[',
        '}': '{',
        '>': '<'
    }
    total = 0

    for line in lines:
        stack = []

        for c in line:
            if c == '\n':
                continue
            if c in opening_brackets:
                stack.append(c)
            elif stack[-1] == corresponding_brackets[c]:
                stack.pop()
            else:
                total += points[c]
                break

    return total

def part2(lines):
    points = {
        '(': 1,
        '[': 2,
        '{': 3,
        '<': 4
    }
    opening_brackets = { '(', '[', '{', '<' }
    corresponding_brackets = {
        ')': '(',
        ']': '[',
        '}': '{',
        '>': '<'
    }
    totals = []

    for line in lines:
        stack = []
        invalid = False

        for c in line:
            if c == '\n':
                continue
            if c in opening_brackets:
                stack.append(c)
            elif stack[-1] == corresponding_brackets[c]:
                stack.pop()
            else:
                invalid = True
                break

        if not invalid:
            curr_total = 0

            while len(stack) > 0:
                curr_total *= 5
                curr_total += points[stack.pop()]

            totals.append(curr_total)

    totals.sort()

    return totals[len(totals) // 2]

if __name__ == '__main__':
    with open('input', 'r') as f:
        lines = f.readlines()

    part_one_results = part1(lines)
    part_two_results = part2(lines)

    print(f'part1: {part_one_results}')
    print(f'part2: {part_two_results}')
