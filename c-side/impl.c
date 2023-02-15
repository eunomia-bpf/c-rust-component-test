#include "c_side_binding.h"

int64_t c_side_binding_gcd4(int64_t a, int64_t b, int64_t c, int64_t d) {
    return gcd2_outer_gcd2(
        gcd2_outer_gcd2(gcd2_outer_gcd2(a, b), c), d);
}