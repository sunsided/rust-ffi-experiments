#include <stdio.h>

typedef struct Point {
    double x;
    double y;
} Point;

extern Point add_points(Point, Point);

int main() {
    const Point first = { .x=-1.2, .y=3.4 };
    const Point second = { .x=4.341592653589793, .y=38.6 };

    const Point sum = add_points(first, second);

    printf("First Point is:  {x: %f, y: %f}\n", first.x, first.y);
    printf("Second Point is: {x: %f, y: %f}\n", second.x, second.y);
    printf("Sum is Point:    {x: %f, y: %f} (calculated in Rust)\n", sum.x, sum.y);

    return 0;
}
