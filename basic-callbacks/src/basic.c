#include <time.h>

typedef void (*Progress)(float);

void sleep_ms(unsigned int ms);

double long_computation(double n, Progress progress) {
    for(int i = 0; i < n; i++) {
        sleep_ms(50);
        progress((double)i/n * 100.0);
    }

    progress(100.0);
    return n*n;
}

void sleep_ms(unsigned int ms) {
    struct timespec tim;
    tim.tv_sec = 0;
    tim.tv_nsec = ms * 1000 * 1000;
    nanosleep(&tim, NULL);
}
