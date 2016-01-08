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
