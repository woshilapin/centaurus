#ifndef __POINT_HPP__
#define __POINT_HPP__

#include <boost/numeric/ublas/vector.hpp>

using namespace boost::numeric::ublas;

namespace centaurus
{
	class Point
	{
		private:
			vector<double> pos;
		public:
			Point();
			Point(const Point &);
			~Point();

			void set_pos(const vector<double>);

			vector<double> get_pos(void);
	};
}

#endif // __POINT_HPP__
