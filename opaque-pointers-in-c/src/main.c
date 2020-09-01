#include <stdio.h>
#include "my_vec.h"

int main() {
    MyVec *const vec = my_vec_new();
    if (vec == NULL) {
        printf("Creation failed!\n");
        return 1;
    }

    printf("The initial length is %d\n", my_vec_len(vec));

    my_vec_push(vec, 42);
    printf("After pushing 42, the length is %d\n", my_vec_len(vec));

    my_vec_push(vec, 123);
    printf("After pushing 123, the length is %d\n", my_vec_len(vec));

    printf("Iterating over the items in my vec:\n");
    const int *const numbers = my_vec_contents(vec);
    const size_t length = my_vec_len(vec);

    for(size_t i = 0; i < length; ++i) {
        printf("my_vec[%ld] = %d\n", i, numbers[i]);
    }

    my_vec_destroy(vec);
    return 0;
}
