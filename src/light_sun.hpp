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
