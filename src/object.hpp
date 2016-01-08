#ifndef __OBJECT_HPP__
#define __OBJECT_HPP__

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
			virtual bool intersect(const point &, const ray &, point &) = 0;
			virtual ray reflect(const ray &) = 0;
	};
}

#endif // __OBJECT_HPP__
