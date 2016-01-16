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
#ifndef __TRIANGLE_HPP__
#define __TRIANGLE_HPP__

#include <vector>
#include "vector.hpp"
#include "ray.hpp"
#include "point.hpp"
#include "object.hpp"
#include "matrix.hpp"

namespace centaurus
{
	class triangle : public object
	{
		private:
			std::vector<point> vertices_;
			vector normal_;
			float plan_offset_;
			matrix basis_;
		public:
			triangle();
			triangle(
					const point &,
					const point &,
					const point &);
			triangle(
					const point &,
					const point &,
					const point &,
					const vector &);
			~triangle();

			std::vector<point> get_vertices(void) const;
			std::vector<point> get_vertices(void);
			point get_vertice(const unsigned int) const;
			point get_vertice(const unsigned int);
			vector get_normal(void) const;
			vector get_normal(void);
			float get_plan_offset(void) const;
			float get_plan_offset(void);
			matrix get_basis(void) const;
			matrix get_basis(void);

			void set_basis(const point &, const point &, const point &);

			bool intersect(const point &, const ray &, point &);
			ray reflect(const ray &);
	};
}

#endif // __TRIANGLE_HPP__
