from copy import deepcopy

class Grid:
    def __init__(self):
        self.__grid = []
        self.__coords = {}
        self.__score_sum = 0

        for _ in range(5):
            self.__grid.append([False] * 5)

    def add_row(self, row, row_index):
        arr = row.split()

        for col_index in range(5):
            number = int(arr[col_index])
            self.__coords[number] = (row_index, col_index)
            self.__score_sum += number

    def draw_number(self, number):
        try:
            row, col = self.__coords[number]
        except KeyError:
            return False

        self.__grid[row][col] = True
        self.__score_sum -= number

        return self.__check_row(row) or self.__check_col(col)

    def get_score(self, number):
        return self.__score_sum * number

    def __check_row(self, row):
        for col in range(5):
            if not self.__grid[row][col]:
                return False

        return True

    def __check_col(self, col):
        for row in range(5):
            if not self.__grid[row][col]:
                return False

        return True

def part1(grids, drawn_numbers):
    for number in drawn_numbers:
        for grid in grids:
            if grid.draw_number(number):
                return grid.get_score(number)

    return 0

def part2(grids, drawn_numbers):
    num_grids = len(grids)
    won = [False] * num_grids
    left = num_grids

    for number in drawn_numbers:
        for i in range(num_grids):
            if won[i]:
                continue

            grid = grids[i]

            if grid.draw_number(number):
                won[i] = True
                left -= 1
            if left == 0:
                return grid.get_score(number)

    return 0

if __name__ == '__main__':
    with open('input', 'r') as f:
        drawn_numbers = map(lambda x: int(x), f.readline().split(','))
        grids = []
        row = 0

        for line in f:
            if line == '\n':
                grids.append(Grid())
                row = 0
            else:
                grids[-1].add_row(line, row)
                row += 1

    part_one_results = part1(deepcopy(grids), deepcopy(drawn_numbers))
    part_two_results = part2(deepcopy(grids), deepcopy(drawn_numbers))

    print(f'part1: {part_one_results}')
    print(f'part2: {part_two_results}')
