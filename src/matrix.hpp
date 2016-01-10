#ifndef __MATRIX_HPP__
#define __MATRIX_HPP__

#include <boost/numeric/ublas/matrix.hpp>
#include "point.hpp"
#include "vector.hpp"

namespace centaurus
{
	using namespace boost::numeric;
	class matrix :
		public ublas::matrix<float>
	{
		static const unsigned int matrix_dim = 3;
		typedef float matrix_type;
		typedef matrix self_type;
		typedef ublas::matrix<matrix_type> base_type;

		public:
		BOOST_UBLAS_INLINE
			matrix ():
				base_type(matrix_dim, matrix_dim)
		{}

		// TODO: Implement a generic version (independent of dimension)
		BOOST_UBLAS_INLINE
			matrix (
					const vector &v1,
					const vector &v2,
					const vector &v3):
				base_type(matrix_dim, matrix_dim)
			{
				(*this)(0,0) = v1[0];
				(*this)(0,1) = v1[1];
				(*this)(0,2) = v1[2];
				(*this)(1,0) = v2[0];
				(*this)(1,1) = v2[1];
				(*this)(1,2) = v2[2];
				(*this)(2,0) = v3[0];
				(*this)(2,1) = v3[1];
				(*this)(2,2) = v3[2];
			}
		BOOST_UBLAS_INLINE
			matrix (const self_type &m):
				base_type(matrix_dim, matrix_dim)
		{
			ublas::matrix_assign<ublas::scalar_assign> ((*this), m);
		}
		template<class AE>
			BOOST_UBLAS_INLINE
			matrix (const ublas::matrix_expression<AE> &ae):
			base_type (matrix_dim, matrix_dim)
		{
			ublas::matrix_assign<ublas::scalar_assign> ((*this), ae);
        }

		// TODO: Implement a generic version (independent of dimension)
		BOOST_UBLAS_INLINE
			matrix_type determinant()
			{
				matrix_type det =
					(*this)(0,0)*(*this)(1,1)*(*this)(2,2)
					+ (*this)(0,1)*(*this)(1,2)*(*this)(2,0)
					+ (*this)(0,2)*(*this)(1,0)*(*this)(2,1)
					- (*this)(0,0)*(*this)(1,2)*(*this)(2,1)
					- (*this)(0,1)*(*this)(1,0)*(*this)(2,2)
					- (*this)(0,2)*(*this)(1,1)*(*this)(2,0);
				return det;
			}

		// TODO: Implement a generic version (independent of dimension)
		BOOST_UBLAS_INLINE
			self_type inverse()
			{
				self_type inv(*this);
				inv(0,0) = (*this)(1,1)*(*this)(2,2) - 
					(*this)(1,2)*(*this)(2,1);
				inv(0,1) = (*this)(0,2)*(*this)(2,1) - 
					(*this)(0,1)*(*this)(2,2);
				inv(0,2) = (*this)(0,1)*(*this)(1,2) - 
					(*this)(0,2)*(*this)(1,1);
				inv(1,0) = (*this)(1,2)*(*this)(2,0) - 
					(*this)(1,0)*(*this)(2,2);
				inv(1,1) = (*this)(0,0)*(*this)(2,2) - 
					(*this)(0,2)*(*this)(2,0);
				inv(1,2) = (*this)(0,2)*(*this)(1,0) - 
					(*this)(0,0)*(*this)(1,2);
				inv(2,0) = (*this)(1,0)*(*this)(2,1) - 
					(*this)(1,1)*(*this)(2,0);
				inv(2,1) = (*this)(0,1)*(*this)(2,0) - 
					(*this)(0,0)*(*this)(2,1);
				inv(2,2) = (*this)(0,0)*(*this)(1,1) - 
					(*this)(0,1)*(*this)(1,0);
				return inv/this->determinant();
			}
	};
}

#endif // __MATRIX_HPP__
