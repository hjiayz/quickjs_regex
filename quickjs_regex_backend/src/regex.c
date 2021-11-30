#include <stddef.h>
#include <stdint.h>

#include "libunicode.h"
#include "libunicode.c"

#include "cutils.h"
#include "cutils.c"

#include "libregexp.h"
#include "libregexp.c"

void *lre_realloc(void *opaque, void *ptr, size_t size)
{
    return realloc(ptr, size);
}

BOOL lre_check_stack_overflow(void *opaque, size_t alloca_size)
{
    return FALSE;
}
