from heapq import heappush, heappop

def part1(grid):
    total = 0

    for i in range(len(grid)):
        row = grid[i]

        for j in range(len(row)):
            cell = row[j]

            if i > 0 and grid[i - 1][j] <= cell:
                continue
            if i < len(grid) - 1 and grid[i + 1][j] <= cell:
                continue
            if j > 0 and grid[i][j - 1] <= cell:
                continue
            if j < len(row) - 1 and grid[i][j + 1] <= cell:
                continue

            total += cell + 1

    return total

def find_size(grid, visited, i, j):
    if i < 0 or i >= len(grid):
        return 0
    if j < 0 or j >= len(grid[i]):
        return 0
    if visited[i][j]:
        return 0

    visited[i][j] = True

    if grid[i][j] == 9:
        return 0

    return 1 + find_size(grid, visited, i - 1, j)\
        + find_size(grid, visited, i + 1, j)\
        + find_size(grid, visited, i, j - 1)\
        + find_size(grid, visited, i, j + 1)

def part2(grid):
    visited = []
    pq = []

    for i in range(len(grid)):
        row = grid[i]

        visited.append([False] * len(row))

    for i in range(len(grid)):
        row = grid[i]

        for j in range(len(row)):
            size = find_size(grid, visited, i, j)

            if size > 0:
                heappush(pq, (-size, size))

    output = 1

    for _ in range(3):
        output *= heappop(pq)[1]

    return output

if __name__ == '__main__':
    grid = []

    with open('input', 'r') as f:
        for line in f:
            row = []

            for digit in line:
                if digit == '\n':
                    continue

                row.append(int(digit))

            grid.append(row)

    part_one_results = part1(grid)
    part_two_results = part2(grid)

    print(f'part1: {part_one_results}')
    print(f'part2: {part_two_results}')
