#ifndef __RAY_HPP__
#define __RAY_HPP__

#include "vector.hpp"

namespace centaurus
{
	class ray
	{
		private:
			vector dir_;
		public:
			ray();
			ray(const vector &);
			ray(const ray &);
			~ray();

			vector get_dir(void) const;
			vector &get_dir(void);
	};
}

#endif // __RAY_HPP__
