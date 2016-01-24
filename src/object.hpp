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
#ifndef __OBJECT_HPP__
#define __OBJECT_HPP__

#include <Magick++.h>

#include "ray.hpp"
#include "point.hpp"

namespace centaurus
{
	class object
	{
		private:
		protected:
			static const bool OBJECT_INTERSECTION = true;
			static const bool OBJECT_NO_INTERSECTION = false;
		public:
			Magick::Color material;
			Magick::Color diffuse;
			Magick::Color specular;
			double shininess;

			virtual bool intersect(const point &, const ray &, point &) = 0;
			virtual ray reflect(const ray &) = 0;
	};
}

#endif // __OBJECT_HPP__
