from copy import deepcopy

def flash(octopi, visited, i, j):
    if visited[i][j]:
        return

    visited[i][j] = True
    dirs = [
        [-1, -1],
        [-1, 0],
        [-1, 1],
        [0, -1],
        [0, 1],
        [1, -1],
        [1, 0],
        [1, 1]
    ]

    for dir in dirs:
        row_idx = i + dir[0]
        col_idx = j + dir[1]

        if row_idx < 0 or row_idx >= len(octopi):
            continue
        if col_idx < 0 or col_idx >= len(octopi[row_idx]):
            continue

        octopi[row_idx][col_idx] += 1

        if octopi[row_idx][col_idx] > 9:
            flash(octopi, visited, row_idx, col_idx)

def step(octopi):
    num_flashes = 0
    visited = [[False] * len(octopi[i]) for i in range(len(octopi))]

    for row in octopi:
        for i in range(len(row)):
            row[i] += 1
    
    for i in range(len(octopi)):
        row = octopi[i]

        for j in range(len(row)):
            if row[j] > 9:
                flash(octopi, visited, i, j)

    for row in octopi:
        for i in range(len(row)):
            if row[i] > 9:
                row[i] = 0
                num_flashes += 1

    return num_flashes

def part1(octopi):
    num_flashes = 0

    for _ in range(100):
        num_flashes += step(octopi)

    return num_flashes

def part2(octopi):
    i = 0

    while True:
        i += 1

        if step(octopi) == 100:
            return i

if __name__ == '__main__':
    octopi = []

    with open('input', 'r') as f:
        for line in f:
            row = []

            for c in line:
                if c == '\n':
                    continue

                row.append(int(c))

            octopi.append(row)

    part_one_results = part1(deepcopy(octopi))
    part_two_results = part2(deepcopy(octopi))

    print(f'part1: {part_one_results}')
    print(f'part2: {part_two_results}')
