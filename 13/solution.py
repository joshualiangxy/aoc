def format_paper(paper, spaces = ' '):
    return '\n'.join(list(map(lambda row: ''.join(list(map(lambda e: '#' if e else spaces, row))), paper)))

def fold_horizontally(paper, pos):
    i = 1

    while pos - i >= 0 and pos + i < len(paper):
        paper[pos - i] = list(map(lambda t: t[0] or t[1], zip(paper[pos - i], paper[pos + i])))
        i += 1
    
    return paper[:pos]

def fold_vertically(paper, pos):
    for i in range(len(paper)):
        row = paper[i]
        j = 1

        while pos - j >= 0 and pos + j < len(row):
            row[pos - j] = row[pos - j] or row[pos + j]
            j += 1

        paper[i] = row[:pos]

    return paper

def part1(paper, fold_instructions):
    total_dots = 0

    direction, pos = fold_instructions[0]

    if direction == 'x':
        paper = fold_vertically(paper, pos)
    if direction == 'y':
        paper = fold_horizontally(paper, pos)

    for row in paper:
        for dot in row:
            if dot:
                total_dots += 1

    return total_dots

def part2(paper, fold_instructions):
    for dir, pos in fold_instructions:
        if dir == 'x':
            paper = fold_vertically(paper, pos)
        if dir == 'y':
            paper = fold_horizontally(paper, pos)

    return paper

if __name__ == '__main__':
    paper = []
    fold_instructions = []
    dots = []
    length = 0
    width = 0

    with open('input', 'r') as f:
        for line in f:
            if line == '\n':
                break

            dot = tuple(map(lambda x: int(x), line.split(',')))
            length = max(dot[1], length)
            width = max(dot[0], width)
            dots.append(dot)
        
        for line in f:
            instruction = line.split()[-1].split('=')
            fold_instructions.append((instruction[0], int(instruction[1])))

    paper = [[False] * (width + 1) for _ in range(length + 1)]

    for y, x in dots:
        paper[x][y] = True

    part_one_results = part1(paper, fold_instructions)
    part_two_results = part2(paper, fold_instructions)

    print(f'part1: {part_one_results}')
    print(f'part2:\n{format_paper(part_two_results)}')
