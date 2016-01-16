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
