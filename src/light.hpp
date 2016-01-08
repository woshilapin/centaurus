#ifndef __LIGHT_HPP__
#define __LIGHT_HPP__

#include "point.hpp"
#include "ray.hpp"

namespace centaurus
{
	class light
	{
		private:
		public:
			virtual ray get_ray(const point &) = 0;
	};
}

#endif // __LIGHT_HPP__
