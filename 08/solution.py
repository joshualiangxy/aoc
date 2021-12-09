def part1(arr):
    total = 0
    easy_digit_lengths = set([2, 3, 4, 7])

    for _, output in arr:
        for digit in output:
            if len(digit) in easy_digit_lengths:
                total += 1

    return total

def part2(arr):
    total = 0
    easy_digit_lengths = set([2, 3, 4, 7])

    for signal, output in arr:
        dc = {}

        # 1, 4, 7, 8
        for digit in signal:
            length = len(digit)

            if length not in easy_digit_lengths:
                continue

            if length == 2:
                dc[1] = set(digit)
            if length == 3:
                dc[7] = set(digit)
            if length == 4:
                dc[4] = set(digit)
            if length == 7:
                dc[8] = set(digit)

        nine_without_g = dc[7].union(dc[4])
        bd = dc[4].difference(dc[1])

        for digit in signal:
            # 0, 6, 9
            if len(digit) == 6:
                lines = set(digit)

                if lines.issuperset(nine_without_g):
                    dc[9] = lines
                    continue
                if lines.issuperset(dc[1]):
                    dc[0] = lines
                else:
                    dc[6] = lines

            # 2, 3, 5
            if len(digit) == 5:
                lines = set(digit)

                if lines.issuperset(bd):
                    dc[5] = lines
                    continue
                if lines.issuperset(dc[1]):
                    dc[3] = lines
                else:
                    dc[2] = lines

        output_value = 0

        for digit in output:
            lines = set(digit)

            for number, number_lines in dc.items():
                if number_lines.issubset(lines) and number_lines.issuperset(lines):
                    output_value *= 10
                    output_value += number

        total += output_value

    return total

if __name__ == '__main__':
    arr = []

    with open('input', 'r') as f:
        for line in f:
            result = tuple(map(lambda signal: signal.split(), line.split(' | ')))
            arr.append(result)

    part_one_results = part1(arr)
    part_two_results = part2(arr)

    print(f'part1: {part_one_results}')
    print(f'part2: {part_two_results}')
