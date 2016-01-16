/*
 * centaurus - a relativist ray-tracer
 * Copyright © 2012-2016 woshilapin<woshilapin@tuziwo.info>
 * 
 * centaurus is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 * 
 * centaurus is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 * 
 * You should have received a copy of the GNU General Public License
 * along with centaurus.  If not, see <http://www.gnu.org/licenses/>.
 */
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
