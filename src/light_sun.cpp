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
#include "light_sun.hpp"

using namespace centaurus;

light_sun::light_sun()
{
	vector v(0.0f, -1.0f, 0.0f);
	this->direction_ = v;
}

light_sun::light_sun(const vector &v)
{
	this->direction_ = v.normalize();
}

light_sun::light_sun(const light_sun &src)
{
	this->direction_ = src.direction_;
}

light_sun::~light_sun()
{
}

ray light_sun::get_ray(const point &p)
{
	ray r(-this->direction_);
	return r;
}
