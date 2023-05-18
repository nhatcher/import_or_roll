// gcc -o metal metal.c -O3 -lm

#include <stdio.h>
#include <math.h>

int count_numbers(double N, double T) {
    int count = 0;
    for (double i = 0.0; i < N; i += 1.0) {
        if (sin(i) < T) {
            count++;
        }
    }
    return count;
}


int main() {
    int count = count_numbers(100000000.0, 0.223);
    printf("%i", count);
}
