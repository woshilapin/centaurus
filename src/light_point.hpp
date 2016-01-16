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
#ifndef __LIGHT_POINT_HPP__
#define __LIGHT_POINT_HPP__

#include "light.hpp"
#include "point.hpp"
#include "vector.hpp"
#include "ray.hpp"

namespace centaurus
{
	class light_point:
		public light
	{
		private:
			point position_;
		public:
			light_point();
			light_point(const point &);
			light_point(const light_point &);
			~light_point();

			ray get_ray(const point &);
	};
}

#endif // __LIGHT_POINT_HPP__
