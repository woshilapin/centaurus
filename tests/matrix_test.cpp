#include "matrix.hpp"
#define BOOST_TEST_DYN_LINK
#define BOOST_TEST_MODULE `matrix` test suite
#define TOLERANCE 0.0001f
#include <boost/test/unit_test.hpp>

using namespace centaurus;

const vector v0 = vector(0.0f, 0.0f, 0.0f);
const vector v1 = vector(1.0f, 2.0f, 3.0f);
const vector v2 = vector(4.0f, 6.0f, 5.0f);
const vector v3 = vector(9.0f, 8.0f, 7.0f);

BOOST_AUTO_TEST_CASE( matrix_constructor )
{
	matrix m0 = matrix();
	matrix m1 = matrix(v0, v0, v0);
	BOOST_CHECK_EQUAL( m0.size1(), m1.size1() );
	BOOST_CHECK_EQUAL( m0.size2(), m1.size2() );
	BOOST_CHECK_CLOSE( m1(0, 0), 0.0f, TOLERANCE );
	BOOST_CHECK_CLOSE( m1(0, 1), 0.0f, TOLERANCE );
	BOOST_CHECK_CLOSE( m1(0, 2), 0.0f, TOLERANCE );
	BOOST_CHECK_CLOSE( m1(1, 0), 0.0f, TOLERANCE );
	BOOST_CHECK_CLOSE( m1(1, 1), 0.0f, TOLERANCE );
	BOOST_CHECK_CLOSE( m1(1, 2), 0.0f, TOLERANCE );
	BOOST_CHECK_CLOSE( m1(2, 0), 0.0f, TOLERANCE );
	BOOST_CHECK_CLOSE( m1(2, 1), 0.0f, TOLERANCE );
	BOOST_CHECK_CLOSE( m1(2, 2), 0.0f, TOLERANCE );

	matrix m2 = matrix(m1);
	BOOST_CHECK_EQUAL( m1.size1(), m2.size1() );
	BOOST_CHECK_EQUAL( m1.size2(), m2.size2() );
	BOOST_CHECK_CLOSE( m1(0, 0), m2(0, 0), TOLERANCE );
	BOOST_CHECK_CLOSE( m1(0, 1), m2(0, 1), TOLERANCE );
	BOOST_CHECK_CLOSE( m1(0, 2), m2(0, 2), TOLERANCE );
	BOOST_CHECK_CLOSE( m1(1, 0), m2(1, 0), TOLERANCE );
	BOOST_CHECK_CLOSE( m1(1, 1), m2(1, 1), TOLERANCE );
	BOOST_CHECK_CLOSE( m1(1, 2), m2(1, 2), TOLERANCE );
	BOOST_CHECK_CLOSE( m1(2, 0), m2(2, 0), TOLERANCE );
	BOOST_CHECK_CLOSE( m1(2, 1), m2(2, 1), TOLERANCE );
	BOOST_CHECK_CLOSE( m1(2, 2), m2(2, 2), TOLERANCE );
}

BOOST_AUTO_TEST_CASE( matrix_determinant )
{
	matrix m0 = matrix(v0, v0, v0);
	BOOST_CHECK_CLOSE( m0.determinant(), 0.0f, TOLERANCE );

	matrix m1 = matrix(v1, v2, v3);
	BOOST_CHECK_CLOSE( m1.determinant(), -30.0f, TOLERANCE );
}

BOOST_AUTO_TEST_CASE( matrix_inverse )
{
	matrix m0 = matrix(v0, v0, v0);
	matrix mi0 = m0.inverse();
	BOOST_CHECK( isnan(mi0(0, 0)) );
	BOOST_CHECK( isnan(mi0(0, 1)) );
	BOOST_CHECK( isnan(mi0(0, 2)) );
	BOOST_CHECK( isnan(mi0(1, 0)) );
	BOOST_CHECK( isnan(mi0(1, 1)) );
	BOOST_CHECK( isnan(mi0(1, 2)) );
	BOOST_CHECK( isnan(mi0(2, 0)) );
	BOOST_CHECK( isnan(mi0(2, 1)) );
	BOOST_CHECK( isnan(mi0(2, 2)) );

	matrix m1 = matrix(v1, v2, v3);
	matrix mi1 = m1.inverse();
	BOOST_CHECK_CLOSE( mi1(0, 0), -0.0666667f, TOLERANCE );
	BOOST_CHECK_CLOSE( mi1(0, 1), -0.3333333f, TOLERANCE );
	BOOST_CHECK_CLOSE( mi1(0, 2),  0.2666667f, TOLERANCE );
	BOOST_CHECK_CLOSE( mi1(1, 0), -0.5666667f, TOLERANCE );
	BOOST_CHECK_CLOSE( mi1(1, 1),  0.6666667f, TOLERANCE );
	BOOST_CHECK_CLOSE( mi1(1, 2), -0.2333333f, TOLERANCE );
	BOOST_CHECK_CLOSE( mi1(2, 0),  0.7333333f, TOLERANCE );
	BOOST_CHECK_CLOSE( mi1(2, 1), -0.3333333f, TOLERANCE );
	BOOST_CHECK_CLOSE( mi1(2, 2),  0.0666667f, TOLERANCE );
}
