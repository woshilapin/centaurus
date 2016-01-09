#include "point.hpp"
#include "ray.hpp"
#include "triangle.hpp"
#define BOOST_TEST_DYN_LINK
#define BOOST_TEST_MODULE `triangle` test suite
#include <boost/test/unit_test.hpp>

using namespace centaurus;

BOOST_AUTO_TEST_CASE( triangle_void_constructor )
{
	triangle t;
	point p;
	vector n;
	BOOST_CHECK( t.get_vertices().size() == 3 );
	BOOST_CHECK( t.get_vertice(0) == p );
	BOOST_CHECK( t.get_vertice(1) == p );
	BOOST_CHECK( t.get_vertice(2) == p );
	BOOST_CHECK( t.get_normal() == n );
	BOOST_CHECK( t.get_plan_offset() == 0.0f );
}
BOOST_AUTO_TEST_CASE( triangle_constructor_points )
{
	point p1(-1.0f,  0.0f, 0.0f);
	point p2( 1.0f,  0.0f, 0.0f);
	point p3( 0.0f,  1.0f, 0.0f);
	vector n(0.0f, 0.0f, 1.0f);
	triangle t(p1, p2, p3);
	BOOST_CHECK( t.get_vertices().size() == 3 );
	BOOST_CHECK( t.get_vertice(0) == p1 );
	BOOST_CHECK( t.get_vertice(1) == p2 );
	BOOST_CHECK( t.get_vertice(2) == p3 );
	BOOST_CHECK( t.get_normal() == n );
	BOOST_CHECK( t.get_plan_offset() == 0.0f );
}
BOOST_AUTO_TEST_CASE( triangle_constructor_points_normal )
{
	point p1(-1.0f,  0.0f, 0.0f);
	point p2( 1.0f,  0.0f, 0.0f);
	point p3( 0.0f,  1.0f, 0.0f);
	vector n(0.0f, 0.0f, 1.0f);
	triangle t(p1, p2, p3, n);
	BOOST_CHECK( t.get_vertices().size() == 3 );
	BOOST_CHECK( t.get_vertice(0) == p1 );
	BOOST_CHECK( t.get_vertice(1) == p2 );
	BOOST_CHECK( t.get_vertice(2) == p3 );
	BOOST_CHECK( t.get_normal() == n );
	BOOST_CHECK( t.get_plan_offset() == 0.0f );
}
BOOST_AUTO_TEST_CASE( triangle_intersect_in_triangle )
{
	point cam(0.0f, 0.0f, 1.0f);
	point o(0.0f, 0.0f, 0.0f);
	point p1(-1.0f,  0.0f, 0.0f);
	point p2( 1.0f,  0.0f, 0.0f);
	point p3( 0.0f,  1.0f, 0.0f);
	triangle t(p1, p2, p3);
	ray r;
	r = ray(o-cam);
	point I;
	bool is_intersect = t.intersect(cam, r, I);
	BOOST_CHECK( is_intersect == true );
	BOOST_CHECK( I[0] == 0.0f );
	BOOST_CHECK( I[1] == 0.0f );
	BOOST_CHECK( I[2] == 0.0f );
}
BOOST_AUTO_TEST_CASE( triangle_intersect_out_triangle )
{
	point cam(0.0f, 0.0f, 1.0f);
	point o(0.0f, 0.0f, 0.0f);
	point p1(-1.0f, -1.0f, 0.0f);
	point p2( 1.0f, -1.0f, 0.0f);
	point p3( 0.0f, -0.5f, 0.0f);
	triangle t(p1, p2, p3);
	ray r;
	r = ray(o-cam);
	point I;
	bool is_intersect = t.intersect(cam, r, I);
	BOOST_CHECK( is_intersect == false );
	BOOST_CHECK( I[0] == 0.0f );
	BOOST_CHECK( I[1] == 0.0f );
	BOOST_CHECK( I[2] == 0.0f );
}
BOOST_AUTO_TEST_CASE( triangle_no_intersect )
{
	point cam(0.0f, 0.0f, 1.0f);
	point o(0.0f, 0.0f, 0.0f);
	point p1(-1.0f, -1.0f,  0.0f);
	point p2( 1.0f, -1.0f,  0.0f);
	point p3( 0.0f, -1.0f, -1.0f);
	triangle t(p1, p2, p3);
	ray r;
	r = ray(o-cam);
	point I;
	bool is_intersect = t.intersect(cam, r, I);
	BOOST_CHECK( is_intersect == false );
	BOOST_CHECK( I[0] == 0.0f );
	BOOST_CHECK( I[1] == 0.0f );
	BOOST_CHECK( I[2] == 0.0f );
}
