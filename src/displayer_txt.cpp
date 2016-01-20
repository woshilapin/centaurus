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
#include "displayer_txt.hpp"

#include <cmath>

using namespace std;
using namespace centaurus;

displayer_txt::~displayer_txt()
{
}

void displayer_txt::display(const buffer & b, ostream & out)
{
	const char * colormap[5];
	colormap[0] = " ";
	colormap[1] = "\u2591";
	colormap[2] = "\u2592";
	colormap[3] = "\u2593";
	colormap[4] = "\u2588";
	unsigned int color_idx = 0;

	// Upper border
	out << "\u250C";
	for (unsigned int w=0; w<b.get_width(); w++)
	{
		out << "\u2500";
	}
	out << "\u2510" << endl;
	for (unsigned int h=0; h<b.get_height(); h++)
	{
		out << "\u2502";
		for (unsigned int w=0; w<b.get_width(); w++)
		{
			color_idx = get_color_from_value(b(h,w));
			out << colormap[color_idx];
		}
		out << "\u2502" << endl;
	}
	// Lower border
	out << "\u2514";
	for (unsigned int w=0; w<b.get_width(); w++)
	{
		out << "\u2500";
	}
	out << "\u2518" << endl;
}

unsigned int displayer_txt::get_color_from_value(const double value)
{
	const unsigned int colormap_size = 5;
	// If 'value=1', then the result will be 5 which is not a valid index [0..4]
	if (value >= 1.0)
	{
		return 4;
	} else {
		return (unsigned int)(floor(value*double(colormap_size)));
	}
}
