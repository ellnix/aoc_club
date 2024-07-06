#include <stdio.h>
#include "parsers/int_list.h"
#include "types/int_matrix.h"

typedef struct {
  int loop_start;
  int loop_end;
} looped_instruction;

int part1(void);
int part2(void);

int main() {
  int p1 = part1();
  int p2 = part2();
  printf("Part 1: %d, Part 2: %d.\n", p1, p2);
}

int max_idx(IntArray* list) {
  int max_index = 0;
  for (int i = 1; i < list->len; i++) {
    if (list->items[i] > list->items[max_index])
      max_index = i;
  }

  return max_index;
}

looped_instruction find_looped_instruction() {
  IntArray* memory_banks = parse_int_list(6);
  IntMatrix* prev_banks = new_int_matrix(0);

  int loop_end = 0;
  int loop_start;

  do {
    int_matrix_push(&prev_banks, deep_copy_array(memory_banks));

    int idx = max_idx(memory_banks);
    int popped_blocks = memory_banks->items[idx];
    memory_banks->items[idx] = 0;

    for (idx++; popped_blocks > 0; idx++, popped_blocks--)
      memory_banks->items[idx % memory_banks->len]++;

    loop_end++;
  } while ((loop_start = find_matrix_row(prev_banks, memory_banks)) == NOT_FOUND);

  free_matrix(prev_banks);

  return (looped_instruction) { loop_start, loop_end };
}

int part1() {
  return find_looped_instruction().loop_end;
}

int part2() {
  looped_instruction ins = find_looped_instruction();
  return ins.loop_end - ins.loop_start;
}

