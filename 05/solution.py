def is_horizontal_line(x1, x2):
    return x1 == x2

def is_vertical_line(y1, y2):
    return y1 == y2

def part1(coords, grid, results = 0):
    if len(coords) < 1:
        return

    for coord_one, coord_two in coords:
        x1, y1 = coord_one
        x2, y2 = coord_two

        if is_horizontal_line(x1, x2):
            for y in range(min(y1, y2), max(y1, y2) + 1):
                if grid[x1][y] == 1:
                    results += 1
                grid[x1][y] += 1

        if is_vertical_line(y1, y2):
            for x in range(min(x1, x2), max(x1, x2) + 1):
                if grid[x][y1] == 1:
                    results += 1
                grid[x][y1] += 1

    return results

def part2(coords, grid, results = 0):
    if len(coords) < 1:
        return

    for coord_one, coord_two in coords:
        x1, y1 = coord_one
        x2, y2 = coord_two

        if not is_horizontal_line(x1, x2) and not is_vertical_line(y1, y2):
            x = x1
            y = y1

            while x != x2 and y != y2:
                if grid[x][y] == 1:
                    results += 1

                grid[x][y] += 1
                x += 1 if x1 < x2 else -1
                y += 1 if y1 < y2 else -1

            if grid[x2][y2] == 1:
                results += 1

            grid[x2][y2] += 1

    return results

if __name__ == '__main__':
    coords = []
    grid = [[0] * 1000 for _ in range(1000)]

    with open('input', 'r') as f:
        for line in f:
            coord_one, coord_two = line.split(' -> ')
            x1, y1 = coord_one.split(',')
            x2, y2 = coord_two.split(',')
            coords.append(((int(x1), int(y1)), (int(x2), int(y2))))

    part_one_results = part1(coords, grid)
    part_two_results = part2(coords, grid, part_one_results)

    print(f'part1: {part_one_results}')
    print(f'part2: {part_two_results}')
