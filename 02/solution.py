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

    print(f'part1: {horizontal * depth}')

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

    print(f'part2: {horizontal * depth}')

if __name__ == '__main__':
    with open('input', 'r') as f:
        arr = f.readlines()

    part1(arr)
    part2(arr)
