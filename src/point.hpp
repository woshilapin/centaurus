#ifndef __POINT_HPP__
#define __POINT_HPP__

#include <boost/numeric/ublas/vector.hpp>
#include <boost/numeric/ublas/io.hpp>
#include "vector.hpp"

namespace centaurus
{
	using namespace boost::numeric;
	class point :
		public ublas::c_vector<float, 3>
	{
		static const unsigned int point_dim = 3;
		typedef float point_type;
		typedef point self_type;
		typedef ublas::c_vector<point_type, point_dim> base_type;

		public:
		BOOST_UBLAS_INLINE
			point ():
				base_type(point_dim)
		{}
		BOOST_UBLAS_INLINE
			point (
					const point_type x,
					const point_type y,
					const point_type z):
				base_type(point_dim)
		{
			(*this)[0] = x;
			(*this)[1] = y;
			(*this)[2] = z;
		}
		BOOST_UBLAS_INLINE
			point (
					const self_type &v):
				base_type(point_dim)
		{
			ublas::vector_assign<ublas::scalar_assign> ((*this), v);
		}
		template<class AE>
		BOOST_UBLAS_INLINE
			point (
					const ublas::vector_expression<AE> &ae):
				base_type(point_dim)
		{
			ublas::vector_assign<ublas::scalar_assign> ((*this), ae);
		}
	};
}

#endif // __POINT_HPP__
