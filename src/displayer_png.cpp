/*
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
 */
#include "displayer_png.hpp"

using namespace std;
using namespace centaurus;
using namespace Magick;

displayer_png::~displayer_png()
{
}

void displayer_png::display(const Image & i, ostream & out)
{
	Image image(i);
	Blob blob;
	image.magick("PNG");
	image.write(&blob);
	out.write((const char *)blob.data(), (long unsigned int)blob.length());
}
