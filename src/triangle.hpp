#ifndef __TRIANGLE_HPP__
#define __TRIANGLE_HPP__

#include <vector>
#include "vector.hpp"
#include "matrix.hpp"
#include "object.hpp"
#include "ray.hpp"
#include "point.hpp"

namespace centaurus
{
	class triangle : public object
	{
		private:
			std::vector<point> vertices_;
			vector normal_;
			double plan_offset_;
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

			point intersect(const ray &, const point &);
	};
}

#endif // __TRIANGLE_HPP__
