# include "factorial.h"

# include <stdio.h>
# include <limits.h>

unsigned long long int factorial (const unsigned long long int n)
{
    unsigned long long int out = 1;
    unsigned long long int i = n;
    if (n == 0)
    {
        return (unsigned long long int) 1;
    }
    for (i = n; i > 0; i--)
    {
        if (ULLONG_MAX / i < out)
        {
            return 0;
        }
        out = out * i;
    }
    return out;
}
