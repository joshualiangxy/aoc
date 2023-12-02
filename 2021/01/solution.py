def part1(arr):
    if len(arr) < 1:
        return

    prev = int(arr[0])
    increases = 0

    for i in range(1, len(arr)):
        curr = int(arr[i])

        if prev < curr:
            increases += 1

        prev = curr

    return increases

def part2(arr):
    if len(arr) < 3:
        return

    to_remove = int(arr[0])
    increases = 0

    for i in range(3, len(arr)):
        to_add = int(arr[i])

        if to_add > to_remove:
            increases += 1

        to_remove = int(arr[i - 2])

    return increases

if __name__ == '__main__':
    with open('input', 'r') as f:
        arr = f.readlines()

    part_one_results = part1(arr)
    part_two_results = part2(arr)

    print(f'part1: {part_one_results}')
    print(f'part2: {part_two_results}')
