/*
 * @file
 * @author  woshilapin <woshilapin@tuziwo.info>
 * @version 0.1
 *
 * @section LICENSE
 *
 * centaurus - a relativist ray-tracer
 * Copyright © 2012-2016 woshilapin <woshilapin@tuziwo.info>
 * 
 * centaurus is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 * 
 * centaurus is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 * 
 * You should have received a copy of the GNU General Public License
 * along with centaurus.  If not, see <http://www.gnu.org/licenses/>.
 *
 * @section This is a small description of the centaurus relativist ray-tracer.
 *
 */

#include <config.h>
#include <cstdlib>
#include <iostream>

#include "localization.hpp"
#include "render.hpp"

using namespace std;
using namespace centaurus;

int main (const int argc, const char* const argv[])
{
	init_localization (PACKAGE, LOCALEDIR);
	cout << PACKAGE_NAME << " [" << PACKAGE_VERSION << "]" << endl;
	cout << "Copyright \u00A9 2012-2016 woshilapin <woshilapin@tuziwo.info>" << endl;
	cout << "This program comes with ABSOLUTELY NO WARRANTY." << endl;
	cout << "This is free software, and you are welcome to redistribute it" << endl;
	cout << "under certain conditions." << endl;

	render r;
	r.run();
	return EXIT_SUCCESS;
}
