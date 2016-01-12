#include "ray.hpp"
#define BOOST_TEST_DYN_LINK
#define BOOST_TEST_MODULE `ray` test suite
#define TOLERANCE 0.0001f
#include <boost/test/unit_test.hpp>

using namespace centaurus;

const vector v0 = vector(0.0f, 0.0f, 1.0f);

BOOST_AUTO_TEST_CASE( ray_constructor )
{
	const ray r0 = ray();
	ray r1 = ray(v0);
	BOOST_CHECK( r0.get_dir() != r1.get_dir() );
	BOOST_CHECK( r1.get_dir() == v0 );

	ray r2 = ray(r1);
	BOOST_CHECK( r1.get_dir() == r2.get_dir() );
}
