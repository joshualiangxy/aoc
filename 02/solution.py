def part1(arr):
    if len(arr) < 1:
        return

    directions = {
        'down': 1,
        'up': -1
    }

    horizontal = 0
    depth = 0

    for line in arr:
        direction, magnitude = line.split(' ')
        magnitude = int(magnitude)

        if direction == 'forward':
            horizontal += magnitude
        else:
            depth += directions[direction] * magnitude

    return horizontal * depth

def part2(arr):
    if len(arr) < 1:
        return

    directions = {
        'down': 1,
        'up': -1
    }

    horizontal = 0
    depth = 0
    aim = 0

    for line in arr:
        direction, magnitude = line.split(' ')
        magnitude = int(magnitude)

        if direction == 'forward':
            horizontal += magnitude
            depth += aim * magnitude
        else:
            aim += directions[direction] * magnitude

    return horizontal * depth

if __name__ == '__main__':
    with open('input', 'r') as f:
        arr = f.readlines()

    part_one_results = part1(arr)
    part_two_results = part2(arr)

    print(f'part1: {part_one_results}')
    print(f'part2: {part_two_results}')
