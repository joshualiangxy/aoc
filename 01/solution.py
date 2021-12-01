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

    print(f'part1: {increases}')

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

    print(f'part2: {increases}')

if __name__ == '__main__':
    with open('input', 'r') as f:
        arr = f.readlines()

    part1(arr)
    part2(arr)

