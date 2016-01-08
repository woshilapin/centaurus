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
