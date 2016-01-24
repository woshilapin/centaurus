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
#include "color.hpp"
#define BOOST_TEST_DYN_LINK
#define BOOST_TEST_MODULE `color` test suite
#include <Magick++.h>
using namespace Magick;
const double TOLERANCE = 100.0/MaxRGB;
#include <boost/test/unit_test.hpp>

using namespace centaurus;

BOOST_AUTO_TEST_CASE( color_constructor )
{
	color c0 = color();
	color c1 = color(0.5);
	color c2 = color(0.5, 0.0);
	color c3 = color(0.25, 0.5, 0.75);
	color c4 = color(0.25, 0.5, 0.75, 0.0);

	BOOST_CHECK_NE( c0.red(), c1.red() );
	BOOST_CHECK_NE( c0.green(), c1.green() );
	BOOST_CHECK_NE( c0.blue(), c1.blue() );
	BOOST_CHECK_EQUAL( c0.alpha(), c1.alpha() );

	BOOST_CHECK_EQUAL( c1.red(), c2.red() );
	BOOST_CHECK_EQUAL( c1.green(), c2.green() );
	BOOST_CHECK_EQUAL( c1.blue(), c2.blue() );
	BOOST_CHECK_NE( c1.alpha(), c2.alpha() );

	BOOST_CHECK_NE( c2.red(), c3.red() );
	BOOST_CHECK_EQUAL( c2.green(), c3.green() );
	BOOST_CHECK_NE( c2.blue(), c3.blue() );
	BOOST_CHECK_NE( c2.alpha(), c3.alpha() );

	BOOST_CHECK_EQUAL( c3.red(), c4.red() );
	BOOST_CHECK_EQUAL( c3.green(), c4.green() );
	BOOST_CHECK_EQUAL( c3.blue(), c4.blue() );
	BOOST_CHECK_NE( c3.alpha(), c4.alpha() );
}

BOOST_AUTO_TEST_CASE( color_getset )
{
	color c = color();

	c.red(0.5);
	BOOST_CHECK_CLOSE( c.red(), 0.5, TOLERANCE );
	c.red(1.0);
	BOOST_CHECK_CLOSE( c.red(), 1.0, TOLERANCE );
	c.red(42.0);
	BOOST_CHECK_CLOSE( c.red(), 1.0, TOLERANCE );
	c.red(0.0);
	BOOST_CHECK_CLOSE( c.red(), 0.0, TOLERANCE );
	c.red(-42.0);
	BOOST_CHECK_CLOSE( c.red(), 0.0, TOLERANCE );
	c.red(0);
	BOOST_CHECK_CLOSE( c.red(), 0.0, TOLERANCE );
	c.red(MaxRGB);
	BOOST_CHECK_CLOSE( c.red(), 1.0, TOLERANCE );

	c.green(0.5);
	BOOST_CHECK_CLOSE( c.green(), 0.5, TOLERANCE );
	c.green(1.0);
	BOOST_CHECK_CLOSE( c.green(), 1.0, TOLERANCE );
	c.green(42.0);
	BOOST_CHECK_CLOSE( c.green(), 1.0, TOLERANCE );
	c.green(0.0);
	BOOST_CHECK_CLOSE( c.green(), 0.0, TOLERANCE );
	c.green(-42.0);
	BOOST_CHECK_CLOSE( c.green(), 0.0, TOLERANCE );
	c.green(0);
	BOOST_CHECK_CLOSE( c.green(), 0.0, TOLERANCE );
	c.green(MaxRGB);
	BOOST_CHECK_CLOSE( c.green(), 1.0, TOLERANCE );

	c.blue(0.5);
	BOOST_CHECK_CLOSE( c.blue(), 0.5, TOLERANCE );
	c.blue(1.0);
	BOOST_CHECK_CLOSE( c.blue(), 1.0, TOLERANCE );
	c.blue(42.0);
	BOOST_CHECK_CLOSE( c.blue(), 1.0, TOLERANCE );
	c.blue(0.0);
	BOOST_CHECK_CLOSE( c.blue(), 0.0, TOLERANCE );
	c.blue(-42.0);
	BOOST_CHECK_CLOSE( c.blue(), 0.0, TOLERANCE );
	c.blue(0);
	BOOST_CHECK_CLOSE( c.blue(), 0.0, TOLERANCE );
	c.blue(MaxRGB);
	BOOST_CHECK_CLOSE( c.blue(), 1.0, TOLERANCE );

	c.alpha(0.5);
	BOOST_CHECK_CLOSE( c.alpha(), 0.5, TOLERANCE );
	c.alpha(1.0);
	BOOST_CHECK_CLOSE( c.alpha(), 1.0, TOLERANCE );
	c.alpha(42.0);
	BOOST_CHECK_CLOSE( c.alpha(), 1.0, TOLERANCE );
	c.alpha(0.0);
	BOOST_CHECK_CLOSE( c.alpha(), 0.0, TOLERANCE );
	c.alpha(-42.0);
	BOOST_CHECK_CLOSE( c.alpha(), 0.0, TOLERANCE );
	c.alpha(0);
	BOOST_CHECK_CLOSE( c.alpha(), 0.0, TOLERANCE );
	c.alpha(MaxRGB);
	BOOST_CHECK_CLOSE( c.alpha(), 1.0, TOLERANCE );
}

BOOST_AUTO_TEST_CASE( color_equal )
{
	color c1 = color(0.25, 0.5, 0.75);
	color c2 = color(0.75, 0.5, 0.25);
	color c3 = color(0.25, 0.5, 0.75, 0.5);

	BOOST_CHECK( c1 != c2 );
	BOOST_CHECK( c1 != c3 );
	BOOST_CHECK( c2 != c3 );
	c3.alpha(1.0);
	BOOST_CHECK( c1 == c3 );
}

BOOST_AUTO_TEST_CASE( color_operator )
{
	color c0 = color(0.0, 0.0, 0.0, 0.0);
	color c1 = color(0.1, 0.2, 0.3, 1.0);
	color c2 = color(0.2, 0.4, 0.6, 1.0);
	color c3 = color(0.2, 0.3, 0.4, 1.0);
	color c4 = color(0.2, 0.3, 0.4, 0.1);
	color c5 = color(0.9, 0.8, 0.7, 1.0);
	Quantum tenth = color::to_quantum(0.1);

	// negative
	BOOST_CHECK( c1.negative() == c5 );
	BOOST_CHECK( -c1 == c5 );

	// operator+
	BOOST_CHECK( (c1 + c1) == c2 );
	BOOST_CHECK( (c1 + 0.1) == c3 );
	BOOST_CHECK( (c1 + tenth) == c4 );

	// operator-
	BOOST_CHECK( (c1 - c1) == c0 );
	BOOST_CHECK( (c3 - 0.1) == c1 );
	BOOST_CHECK( (c3 - tenth) == c1 );

	// operator*
	BOOST_CHECK( (c0 * c1) == c0 );
	BOOST_CHECK( (c1 * 2.0) == c2 );
}
