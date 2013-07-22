#ifndef __OBJECT_HPP__
#define __OBJECT_HPP__

#include "ray.hpp"
#include "point.hpp"

namespace centaurus
{
	class object
	{
		private:
		public:
			point intersect(const ray &, const point &);
	};
}

#endif // __OBJECT_HPP__
