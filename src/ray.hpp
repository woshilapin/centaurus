#ifndef __RAY_HPP__
#define __RAY_HPP__

#include <boost/numeric/ublas/vector.hpp>

using namespace boost::numeric::ublas;

namespace centaurus
{
	class Ray
	{
		private:
			vector<double> dir;
		public:
			Ray();
			Ray(const Ray &);
			~Ray();

			void set_dir(const vector<double>);

			vector<double> get_dir(void);
	};
}

#endif // __RAY_HPP__
