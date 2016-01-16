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
#ifndef __LIGHT_SUN_HPP__
#define __LIGHT_SUN_HPP__

#include "light.hpp"
#include "point.hpp"
#include "vector.hpp"
#include "ray.hpp"

namespace centaurus
{
	class light_sun:
		public light
	{
		private:
			vector direction_;
		public:
			light_sun();
			light_sun(const vector &);
			light_sun(const light_sun &);
			~light_sun();

			ray get_ray(const point &);
	};
}

#endif // __LIGHT_SUN_HPP__
