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
#define BOOST_TEST_DYN_LINK
#define BOOST_TEST_MODULE `point` test suite
#define TOLERANCE 0.0001
#include <boost/test/unit_test.hpp>

using namespace centaurus;

BOOST_AUTO_TEST_CASE( point_constructor )
{
	point p1 = point();
	point p2 = point(0.0f, 0.0f, 0.0f);
	BOOST_CHECK_EQUAL( p1.size(), p2.size() );
	BOOST_CHECK_CLOSE( p1[0], p2[0], TOLERANCE );
	BOOST_CHECK_CLOSE( p1[1], p2[1], TOLERANCE );
	BOOST_CHECK_CLOSE( p1[2], p2[2], TOLERANCE );

	point p3 = point(p2);
	BOOST_CHECK_EQUAL( p1.size(), p3.size() );
	BOOST_CHECK_CLOSE( p1[0], p3[0], TOLERANCE );
	BOOST_CHECK_CLOSE( p1[1], p3[1], TOLERANCE );
	BOOST_CHECK_CLOSE( p1[2], p3[2], TOLERANCE );
}

BOOST_AUTO_TEST_CASE( point_equal )
{
	point p0 = point(0.0f, 0.0f, 0.0f);
	point p1 = point(1.0f, 2.0f, 3.0f);
	BOOST_CHECK( p0 == p0 );
	BOOST_CHECK( p1 == p1 );
	BOOST_CHECK( p0 != p1 );
}
