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
#include "ray.hpp"

#include "vector.hpp"

using namespace centaurus;

ray::ray()
{
	this->dir_ = vector();
}

ray::ray(const vector &v)
{
	this->dir_ = v / v.norm();
}

ray::ray(const ray &src)
{
	this->dir_ = src.dir_;
}

ray::~ray()
{
}

vector ray::get_dir(void) const
{
	return this->dir_;
}

vector &ray::get_dir(void)
{
	return this->dir_;
}
