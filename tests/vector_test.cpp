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
#include "vector.hpp"
#define BOOST_TEST_DYN_LINK
#define BOOST_TEST_MODULE `vector` test suite
#define TOLERANCE 0.0001
#include <boost/test/unit_test.hpp>

using namespace centaurus;

BOOST_AUTO_TEST_CASE( vector_constructor )
{
	vector v1 = vector();
	vector v2 = vector(0.0f, 0.0f, 0.0f);
	BOOST_CHECK_EQUAL( v1.size(), v2.size() );
	BOOST_CHECK_CLOSE( v1[0], v2[0], TOLERANCE );
	BOOST_CHECK_CLOSE( v1[1], v2[1], TOLERANCE );
	BOOST_CHECK_CLOSE( v1[2], v2[2], TOLERANCE );

	vector v3 = vector(v2);
	BOOST_CHECK_EQUAL( v1.size(), v3.size() );
	BOOST_CHECK_CLOSE( v1[0], v3[0], TOLERANCE );
	BOOST_CHECK_CLOSE( v1[1], v3[1], TOLERANCE );
	BOOST_CHECK_CLOSE( v1[2], v3[2], TOLERANCE );
}

BOOST_AUTO_TEST_CASE( vector_dot_product )
{
	const vector v0 = vector(0.0f, 0.0f, 0.0f);
	vector v1 = vector(1.0f, 2.0f, 3.0f);
	vector v2 = vector(2.0f, 3.0f, 4.0f);
	BOOST_CHECK_CLOSE( (v0 * v1), 0.0f, TOLERANCE );
	BOOST_CHECK_CLOSE( (v1 * v1), 14.0f, TOLERANCE );
	BOOST_CHECK_CLOSE( (v1 * v2), 20.0f, TOLERANCE );
}

BOOST_AUTO_TEST_CASE( vector_cross_product )
{
	const vector v0 = vector(0.0f, 0.0f, 0.0f);
	vector v1 = vector(1.0f, 2.0f, 3.0f);
	vector v2 = vector(2.0f, 3.0f, 4.0f);
	vector vr1 = v0 ^ v1;
	BOOST_CHECK_CLOSE( vr1[0], 0.0f, TOLERANCE );
	BOOST_CHECK_CLOSE( vr1[1], 0.0f, TOLERANCE );
	BOOST_CHECK_CLOSE( vr1[2], 0.0f, TOLERANCE );

	vector vr2 = v1 ^ v1;
	BOOST_CHECK_CLOSE( vr2[0], 0.0f, TOLERANCE );
	BOOST_CHECK_CLOSE( vr2[1], 0.0f, TOLERANCE );
	BOOST_CHECK_CLOSE( vr2[2], 0.0f, TOLERANCE );

	vector vr3 = v1 ^ v2;
	BOOST_CHECK_CLOSE( vr3[0], -1.0f, TOLERANCE );
	BOOST_CHECK_CLOSE( vr3[1], 2.0f, TOLERANCE );
	BOOST_CHECK_CLOSE( vr3[2], -1.0f, TOLERANCE );
}

BOOST_AUTO_TEST_CASE( vector_equals )
{
	const vector v0 = vector(0.0f, 0.0f, 0.0f);
	vector v1 = vector(1.0f, 2.0f, 3.0f);
	vector v2 = vector(2.0f, 2.0f, 2.0f);
	BOOST_CHECK( v0 == v0 );
	BOOST_CHECK( v1 == v1 );
	BOOST_CHECK( v0 != v1 );
	BOOST_CHECK( v1 != v2 );
}

BOOST_AUTO_TEST_CASE( vector_norm )
{
	const vector v0 = vector(0.0f, 0.0f, 0.0f);
	vector v1 = vector(3.0f, 4.0f, 0.0f);
	BOOST_CHECK_CLOSE( v0.norm(), 0.0f, TOLERANCE );
	BOOST_CHECK_CLOSE( v1.norm(), 5.0f, TOLERANCE );
}

BOOST_AUTO_TEST_CASE( vector_normalize )
{
	const vector v0 = vector(0.0f, 0.0f, 0.0f);
	vector vn0 = v0.normalize();
	BOOST_CHECK( isnan(vn0[0]) );
	BOOST_CHECK( isnan(vn0[1]) );
	BOOST_CHECK( isnan(vn0[2]) );

	vector v1 = vector(3.0f, 4.0f, 0.0f);
	vector vn1 = v1.normalize();
	BOOST_CHECK_CLOSE( vn1[0], 0.6f, TOLERANCE );
	BOOST_CHECK_CLOSE( vn1[1], 0.8f, TOLERANCE );
	BOOST_CHECK_CLOSE( vn1[2], 0.0f, TOLERANCE );
}
