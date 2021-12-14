from sys import maxsize

def difference(elem_count):
    largest = -1
    smallest = maxsize

    for count in elem_count.values():
        largest = max(largest, count)
        smallest = min(smallest, count)

    return largest - smallest

def step(rules, pair_count, elem_count):
    next_pair_count = {}

    for pair, count in pair_count.items():
        if pair not in rules:
            next_pair_count[pair] = count
            continue

        to_insert = rules[pair]
        left_pair = pair[0] + to_insert
        right_pair = to_insert + pair[1]

        if left_pair not in next_pair_count:
            next_pair_count[left_pair] = 0
        if right_pair not in next_pair_count:
            next_pair_count[right_pair] = 0
        if to_insert not in elem_count:
            elem_count[to_insert] = 0

        next_pair_count[left_pair] += count
        next_pair_count[right_pair] += count
        elem_count[to_insert] += count

    return next_pair_count

def main(template, rules):
    elem_count = {}
    pair_count = {}

    for c in template:
        if c not in elem_count:
            elem_count[c] = 0
        elem_count[c] += 1

    for i in range(0, len(template) - 1):
        pair = template[i:i + 2]

        if pair not in pair_count:
            pair_count[pair] = 0

        pair_count[pair] += 1

    for i in range(40):
        if i == 10:
            part1 = difference(elem_count)

        pair_count = step(rules, pair_count, elem_count)

    part2 = difference(elem_count)

    return part1, part2

if __name__ == '__main__':
    rules = {}

    with open('input', 'r') as f:
        template = f.readline().rstrip();

        for line in f:
            if line == '\n':
                continue

            key, val = line.rstrip().split(' -> ')
            rules[key] = val

    part_one_results, part_two_results = main(template, rules)

    print(f'part1: {part_one_results}')
    print(f'part2: {part_two_results}')
