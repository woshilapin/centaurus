#ifndef __POINT_HPP__
#define __POINT_HPP__

#include <boost/numeric/ublas/vector.hpp>
#include <boost/numeric/ublas/matrix_expression.hpp>

namespace centaurus
{
	using namespace boost::numeric;
	class matrix;
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
		{
			(*this)[0] = 0.0f;
			(*this)[1] = 0.0f;
			(*this)[2] = 0.0f;
		}
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
			point (const self_type &v):
				base_type(point_dim)
		{
			ublas::vector_assign<ublas::scalar_assign> ((*this), v);
		}
		template<class AE>
		BOOST_UBLAS_INLINE
			point (const ublas::vector_expression<AE> &ae):
				base_type(point_dim)
		{
			ublas::vector_assign<ublas::scalar_assign> ((*this), ae);
		}
		template<class AE>
		BOOST_UBLAS_INLINE
			point (const ublas::matrix_expression<AE> &ae):
				base_type(point_dim)
		{
			ublas::vector_assign<ublas::scalar_assign> ((*this), ae);
		}

		BOOST_UBLAS_INLINE
			bool operator==(const self_type &p) const
			{
				if (p.size() != point_dim) return false;
				for (unsigned int idx=0; idx<point_dim; idx++)
				{
					if ((*this)[idx] != p[idx]) return false;
				}
				return true;
			}
		BOOST_UBLAS_INLINE
			bool operator==(const self_type &p)
			{
				const point copy = point(*this);
				return copy == p;
			}
		
		BOOST_UBLAS_INLINE
			bool operator!=(const self_type &p) const
			{
				return !((*this) == p);
			}
		
		BOOST_UBLAS_INLINE
			bool operator!=(const self_type &p)
			{
				return !((*this) == p);
			}
	};
}

#endif // __POINT_HPP__
