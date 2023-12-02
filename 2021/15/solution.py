from sys import maxsize
from heapq import heapify, heappop, heappush

dirs = [
    [-1, 0],
    [1, 0],
    [0, -1],
    [0, 1]
]

def print_graph(graph):
    print('\n'.join(map(lambda row: str(row), graph)))

def replace(pq, pos, new_dist):
    for i in range(len(pq)):
        if pq[i][1] == pos:
            pq[i] = (new_dist, pos)

    heapify(pq)

def part1(graph):
    visited = [[False] * len(graph[i]) for i in range(len(graph))]
    shortest_path = [[maxsize] * len(graph[i]) for i in range(len(graph))]
    shortest_path[0][0] = 0
    pq = []

    heappush(pq, (0, (0, 0)))

    while len(pq) > 0:
        curr_dist, pos = heappop(pq)
        i, j = pos
        visited[i][j] = True

        for dir in dirs:
            neighbour = (i + dir[0], j + dir[1])
            new_i, new_j = neighbour

            if new_i < 0 or new_i >= len(graph):
                continue
            if new_j < 0 or new_j >= len(graph[new_i]):
                continue
            if visited[new_i][new_j]:
                continue

            new_dist = curr_dist + graph[new_i][new_j]

            if shortest_path[new_i][new_j] == maxsize:
                heappush(pq, (new_dist, neighbour))
                shortest_path[new_i][new_j] = new_dist
            if shortest_path[new_i][new_j] > new_dist:
                replace(pq, neighbour, new_dist)
                shortest_path[new_i][new_j] = new_dist

    return shortest_path[len(graph) - 1][len(graph[0]) - 1]

def part2(graph):
    row_length = len(graph)
    col_length = len(graph[0])
    actual_graph = [[0] * col_length * 5 for _ in range(row_length * 5)]

    for i in range(row_length):
        for j in range(5):
            row = map(lambda x: x + j if x + j < 10 else (x + j) % 10 + 1, graph[i])

            for k in range(col_length):
                actual_graph[i + j * row_length][k] = next(row)
    for i in range(len(actual_graph)):
        for j in range(col_length):
            e = actual_graph[i][j]

            for k in range(1, 5):
                actual_graph[i][j + col_length * k] = e + k if e + k < 10 else (e + k) % 10 + 1

    return part1(actual_graph)

if __name__ == '__main__':
    graph = []

    with open('input', 'r') as f:
        for line in f:
            graph.append(list(map(lambda x: int(x), line.rstrip())))

    part_one_results = part1(graph)
    part_two_results = part2(graph)

    print(f'part1: {part_one_results}')
    print(f'part2: {part_two_results}')
