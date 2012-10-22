/**
 * @file
 * @author  woshilapin <woshilapin@gmail.com>
 * @version 0.1
 *
 * @section LICENSE
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU General Public License as
 * published by the Free Software Foundation; either version 2 of
 * the License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
 * General Public License for more details at
 * http://www.gnu.org/copyleft/gpl.html
 *
 * @section This is a small description of the centaurus relativist ray-tracer.
 *
 * The time class represents a moment of time.
 */

# include <config.h>
# include <stdlib.h>
# include <stdio.h>

# include "localization.h"
# include "factorial.h"

int main (int argc, char** argv)
{
    init_localization (PACKAGE, LOCALEDIR);
    printf (_ ("Hello world!\n"));
    printf (_ ("The factorial of 5 is: 5! = %ld\n"), factorial (5));
    printf (_ ("This is %s.\n"), PACKAGE_STRING);
    return EXIT_SUCCESS;
}
