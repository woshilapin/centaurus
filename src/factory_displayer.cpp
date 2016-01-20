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
#include "factory_displayer.hpp"

#include "displayer_png.hpp"
#include "displayer_txt.hpp"

using namespace std;
using namespace centaurus;

displayer * factory_displayer::create(const displayer_type & type)
{
	switch (type)
	{
		case PNG:
			return new displayer_png();
		case TXT:
			return new displayer_txt();
		default:
			throw "Displayer factory: Unkown displayer type";
	}
}
