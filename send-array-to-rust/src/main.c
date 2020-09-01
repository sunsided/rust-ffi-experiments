#include <stdio.h>

extern int sum(const int* my_array, int length);

int main() {
    const int my_array[8] = {1, 2, 3, 4, 5, 6, 7, 8};
    const int total = sum(my_array, 8);
    printf("The total is %d (calculated in Rust)\n", total);
    return 0;
}
