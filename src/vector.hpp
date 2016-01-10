#ifndef __VECTOR_HPP__
#define __VECTOR_HPP__

#include <boost/numeric/ublas/vector.hpp>

namespace centaurus
{
	using namespace boost::numeric;
	class vector :
		public ublas::c_vector<float, 3>
	{
		static const unsigned int vector_dim = 3;
		typedef float vector_type;
		typedef vector self_type;
		typedef ublas::c_vector<vector_type, vector_dim> base_type;

		public:
		BOOST_UBLAS_INLINE
			vector ():
				base_type(vector_dim)
		{
			(*this)[0] = 0.0f;
			(*this)[1] = 0.0f;
			(*this)[2] = 0.0f;
		}
		BOOST_UBLAS_INLINE
			vector (
					const vector_type x,
					const vector_type y,
					const vector_type z):
				base_type(vector_dim)
		{
			(*this)[0] = x;
			(*this)[1] = y;
			(*this)[2] = z;
		}
		BOOST_UBLAS_INLINE
			vector (const self_type &v):
				base_type(vector_dim)
		{
			ublas::vector_assign<ublas::scalar_assign> ((*this), v);
		}
		template<class AE>
		BOOST_UBLAS_INLINE
			vector (const ublas::vector_expression<AE> &ae):
				base_type(vector_dim)
		{
			ublas::vector_assign<ublas::scalar_assign> ((*this), ae);
		}

		BOOST_UBLAS_INLINE
			vector_type operator* (const self_type &v) const
			{
				return ublas::inner_prod((*this), v);
			}
		BOOST_UBLAS_INLINE
			vector_type operator* (const self_type &v)
			{
				return ublas::inner_prod((*this), v);
			}
		BOOST_UBLAS_INLINE
			self_type operator^ (const self_type &v) const
			{
				self_type n;
				n[0] = (*this)[1]*v[2] - (*this)[2]*v[1];
				n[1] = (*this)[2]*v[0] - (*this)[0]*v[2];
				n[2] = (*this)[0]*v[1] - (*this)[1]*v[0];
				return n;
			}
		BOOST_UBLAS_INLINE
			self_type operator^ (const self_type &v)
			{
				self_type n;
				n[0] = (*this)[1]*v[2] - (*this)[2]*v[1];
				n[1] = (*this)[2]*v[0] - (*this)[0]*v[2];
				n[2] = (*this)[0]*v[1] - (*this)[1]*v[0];
				return n;
			}
		BOOST_UBLAS_INLINE
			bool operator== (const self_type &v) const
			{
				if (v.size() != vector_dim) return false;
				for (unsigned int idx=0; idx<vector_dim; idx++)
				{
					// TODO: Do a real comparison of float numbers
					if ((*this)[idx] != v[idx]) return false;
				}
				return true;
			}
		BOOST_UBLAS_INLINE
			bool operator== (const self_type &v)
			{
				const vector copy = vector(*this);
				return copy == v;
			}
		BOOST_UBLAS_INLINE
			bool operator!= (const self_type &v) const
			{
				return !((*this) == v);
			}
		BOOST_UBLAS_INLINE
			bool operator!= (const self_type &v)
			{
				return !((*this) == v);
			}
		BOOST_UBLAS_INLINE
			vector_type norm () const
			{
				return ublas::norm_2((*this));
			}
		BOOST_UBLAS_INLINE
			vector_type norm ()
			{
				return ublas::norm_2((*this));
			}
		BOOST_UBLAS_INLINE
			self_type normalize() const
			{
				return (*this)/(*this).norm();
			}
		BOOST_UBLAS_INLINE
			self_type normalize()
			{
				return (*this)/(*this).norm();
			}
	};
}

#endif // __VECTOR_HPP__
