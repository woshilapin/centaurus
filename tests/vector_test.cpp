#include "vector.hpp"
#define BOOST_TEST_DYN_LINK
#define BOOST_TEST_MODULE `vector' test suite
#include <boost/test/unit_test.hpp>

using namespace centaurus;

BOOST_AUTO_TEST_CASE( vector_constructor )
{
	vector v1(0.0f, 0.0f, 0.0f);
	vector v2(1.0f, 1.0f, 1.0f);
	BOOST_CHECK( v1[0] != v2[0] );
}
