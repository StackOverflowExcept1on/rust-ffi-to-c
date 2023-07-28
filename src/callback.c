#include <stdio.h>
#include <stdint.h>

int32_t add(int32_t a, int32_t b, void(*callback)(int32_t)) {
    puts("calling rust code...");
    callback(a * b);
    puts("done");
    return a * b;
}
