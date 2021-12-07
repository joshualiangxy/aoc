from sys import maxsize

def part1(positions, largest):
    least = maxsize

    for aligned_position in range(largest + 1):
        curr_least = 0

        for position in positions:
            curr_least += abs(position - aligned_position)

        least = min(curr_least, least)

    return least

def get_fuel(fuel, moved_units):
    length = len(fuel)

    for i in range(length, moved_units + 1):
        fuel.append(fuel[i - 1] + i)

    return fuel[moved_units]

def part2(positions, largest):
    least = maxsize
    fuel = [0]

    for aligned_position in range(largest + 1):
        curr_least = 0

        for position in positions:
            moved_units = abs(position - aligned_position)

            try:
                curr_least += fuel[moved_units]
            except IndexError:
                curr_least += get_fuel(fuel, moved_units)

        least = min(curr_least, least)

    return least

if __name__ == '__main__':
    with open('input', 'r') as f:
        positions = list(map(lambda x: int(x), f.readline().split(',')))
        largest = 0

        for position in positions:
            largest = max(largest, position)

    part_one_results = part1(positions, largest)
    part_two_results = part2(positions, largest)

    print(f'part1: {part_one_results}')
    print(f'part2: {part_two_results}')
