#include "basic.h"

extern int (*main)();

void _start() {
    exit(main());
}

