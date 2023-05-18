// clang --target=wasm32 --no-standard-libraries -Wl,--export-all -Wl,--no-entry -o import.wasm import.c

__attribute__((import_module("Math"), import_name("sin")))
double Sin(double x);

__attribute__((visibility("default")))
int count_numbers(double N, double T) {
    int count = 0;
    for (double i = 0.0; i < N; i += 1.0) {
        if (Sin(i) < T) {
            count++;
        }
    }
    return count;
}
