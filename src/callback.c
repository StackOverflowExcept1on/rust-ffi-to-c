#include <stdint.h>

int add(int32_t a, int32_t b, void(*callback)(int32_t)) {
    callback(a * b);
    return a * b;
}
