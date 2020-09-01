#include <stdio.h>

// Callback type and the generator method are defined in Rust.
typedef void (*Process)(void *data, int value);
extern void generate_numbers(int upper, Process process, void *data);

// The state our callback requires to work.
typedef struct Statistics {
    int count;
    float average;
} Statistics;

// Our callback method; implements the Process signature.
void increment_statistics(void *data, int value) {
    Statistics *const stats = (Statistics*)data;
    printf("received %d\n", value);

    float total = stats->average * stats->count;
    total += value;

    stats->count += 1;
    stats->average = total / stats->count;
}

int main() {
    int upper_limit = 10;
    Statistics stats = {0, 0};

    printf("Generating %d numbers\n", upper_limit);

    // Call out to the Rust function, providing our callback as a function pointer,
    // as well as our state as a *void pointer.
    generate_numbers(upper_limit, increment_statistics, &stats);

    printf("Statistics:\n");
    printf("\tCount: %d\n", stats.count);
    printf("\tAverage: %g\n", stats.average);

    return 0;
}
