#include <assert.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

struct int_array
{
    size_t len;
    int data[];
};

static struct int_array * int_array_copy(const struct int_array *arr)
{
    const size_t banks_size = sizeof(*arr) + arr->len * sizeof(*arr->data);
    struct int_array *result = malloc(banks_size);
    assert(result);
    result->len = arr->len;
    for (size_t i = 0; i < arr->len; ++i)
        result->data[i] = arr->data[i];
    return result;
}

static bool int_array_equal(struct int_array *arr1, struct int_array *arr2)
{
    if (arr1->len != arr2->len)
        return false;
    for (size_t i = 0; i < arr1->len; ++i)
        if (arr1->data[i] != arr2->data[i])
            return false;
    return true;
}

static struct int_array * read_input(const char *filename)
{
    FILE *fp = fopen(filename, "r");
    assert(fp);
    size_t cap = 20;
    struct int_array *result = malloc(sizeof(*result) + cap * sizeof(*result->data));
    result->len = 0;
    while (true)
    {
        assert(result->len < cap);
        int rc = fscanf(fp, " %d", result->data + result->len);
        if (rc != 1)
            break;
        ++result->len;
    }
    return result;
}

static struct int_array * step(struct int_array *banks)
{
    size_t max_idx = 0;
    for (size_t i = 1; i < banks->len; ++i)
        if (banks->data[i] > banks->data[max_idx])
            max_idx = i;
    int blocks = banks->data[max_idx];
    banks->data[max_idx] = 0;
    for (size_t i = max_idx + 1; blocks --> 0; ++i)
    {
        if (i == banks->len)
            i = 0;
        ++banks->data[i];
    }
    return banks;
}

int main(void)
{
    //struct int_array *banks = read_input("day06.example.txt");
    struct int_array *banks = read_input("day06.txt");
    struct int_array *slow = int_array_copy(banks);
    struct int_array *fast = int_array_copy(banks);

    // Floyd's loop detection for
    while (true)
    {
        step(slow);
        step(step(fast));
        if (int_array_equal(slow, fast))
            break;
    }
    // at this point slow and fast represent the same node in the cycle

    // to find the start of the cycle we restart with the slow pointer
    // at the beginning and more the fast pointer along the cycle in slow pace
    for (size_t i = 0; i < banks->len; ++i)
        slow->data[i] = banks->data[i];
    int path_len = 0;
    for (; !int_array_equal(slow, fast); step(slow), step(fast))
        ++path_len;

    // find the length of the cycle
    int cycle_len = 1;
    for (step(slow); !int_array_equal(slow, fast); step(slow))
        ++cycle_len;

    // cleanup and print result
    free(fast);
    free(slow);
    printf("part 1: %d\n", path_len + cycle_len);
    printf("part 2: %d\n", cycle_len);
}
