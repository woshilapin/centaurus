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
 */

#include <config.h>
#include <iostream>

#include "localization.hpp"
#include "render.hpp"

using namespace std;
using namespace centaurus;

int main (const int argc, const char* const argv[])
{
	init_localization (PACKAGE, LOCALEDIR);
	cout << "Executing " << argv[0];
	cout << " with " << argc-1 << " argument(s)" << endl;
	cout << "This is " << PACKAGE_STRING << endl;

	render r;
	r.run();
	return EXIT_SUCCESS;
}
