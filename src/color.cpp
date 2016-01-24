/*
 * centaurus - a relativist ray-tracer
 * Copyright © 2012-2016 woshilapin <woshilapin@tuziwo.info>
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

#include <boost/math/special_functions/round.hpp>

using namespace centaurus;
using namespace Magick;
using namespace boost::math;

color::color() : Color()
{
}

color::color(const double gray)
: Color(
	Quantum(color::to_quantum(gray)),
	Quantum(color::to_quantum(gray)),
	Quantum(color::to_quantum(gray)),
	Quantum(color::to_quantum(1.0)))
{
}

color::color(const double gray, const double alpha)
: Color(
	Quantum(color::to_quantum(gray)),
	Quantum(color::to_quantum(gray)),
	Quantum(color::to_quantum(gray)),
	Quantum(color::to_quantum(alpha)))
{
}

color::color(const double red, const double green, const double blue)
: Color(
	Quantum(color::to_quantum(red)),
	Quantum(color::to_quantum(green)),
	Quantum(color::to_quantum(blue)),
	Quantum(color::to_quantum(1.0)))
{
}

color::color(const double red, const double green, const double blue, const double alpha)
: Color(
	Quantum(color::to_quantum(red)),
	Quantum(color::to_quantum(green)),
	Quantum(color::to_quantum(blue)),
	Quantum(color::to_quantum(alpha)))
{
}

color::~color()
{
}

double color::red() const
{
	return color::to_double(this->redQuantum());
}
double color::red()
{
	return color::to_double(this->redQuantum());
}
void color::red(const double red)
{
	this->redQuantum(color::to_quantum(color::saturate(red)));
}

double color::green() const
{
	return color::to_double(this->greenQuantum());
}
double color::green()
{
	return color::to_double(this->greenQuantum());
}
void color::green(const double green)
{
	this->greenQuantum(color::to_quantum(color::saturate(green)));
}

double color::blue() const
{
	return color::to_double(this->blueQuantum());
}
double color::blue()
{
	return color::to_double(this->blueQuantum());
}
void color::blue(const double blue)
{
	this->blueQuantum(color::to_quantum(color::saturate(blue)));
}

double color::alpha() const
{
	return color::to_double(this->alphaQuantum());
}
double color::alpha()
{
	return color::to_double(this->alphaQuantum());
}
void color::alpha(const double alpha)
{
	this->alphaQuantum(color::to_quantum(color::saturate(alpha)));
}

bool color::operator==(const color & c)
{
	if(
		fabs(this->redQuantum() - c.redQuantum()) <= color::quantum_epsilon
		&& fabs(this->greenQuantum() - c.greenQuantum()) <= color::quantum_epsilon
		&& fabs(this->blueQuantum() - c.blueQuantum()) <= color::quantum_epsilon
		&& fabs(this->alphaQuantum() - c.alphaQuantum()) <= color::quantum_epsilon)
	{
		return true;
	}
	return false;
}

bool color::operator!=(const color & c)
{
	return !(this->operator==(c));
}


color color::negative()
{
	return Quantum(MaxRGB) - (*this);
}
color color::operator-()
{
	return this->negative();
}
color color::operator+(const color & c2)
{
	color c = color();
	c.red(this->red() + c2.red());
	c.green(this->green() + c2.green());
	c.blue(this->blue() + c2.blue());
	c.alpha(this->alpha() + c2.alpha());
	return c;
}
color color::operator+(const double d2)
{
	color c = color();
	c.red(this->red() + d2);
	c.green(this->green() + d2);
	c.blue(this->blue() + d2);
	c.alpha(this->alpha() + d2);
	return c;
}
color color::operator+(const Quantum q2)
{
	color c = color();
	c.redQuantum(this->redQuantum() + q2);
	c.greenQuantum(this->greenQuantum() + q2);
	c.blueQuantum(this->blueQuantum() + q2);
	c.alphaQuantum(this->alphaQuantum() + q2);
	return c;
}

color color::operator-(const color & c2)
{
	color c = color();
	c.red(this->red() - c2.red());
	c.green(this->green() - c2.green());
	c.blue(this->blue() - c2.blue());
	c.alpha(this->alpha() - c2.alpha());
	return c;
}
color color::operator-(const double d2)
{
	color c = color();
	c.red(this->red() - d2);
	c.green(this->green() - d2);
	c.blue(this->blue() - d2);
	c.alpha(this->alpha());
	return c;
}
color color::operator-(const Quantum q2)
{
	color c = color();
	c.redQuantum(this->redQuantum() - q2);
	c.greenQuantum(this->greenQuantum() - q2);
	c.blueQuantum(this->blueQuantum() - q2);
	c.alphaQuantum(this->alphaQuantum());
	return c;
}

color color::operator*(const color & c2)
{
	color c = color();
	c.red(this->red() * c2.red());
	c.green(this->green() * c2.green());
	c.blue(this->blue() * c2.blue());
	c.alpha(this->alpha() * c2.alpha());
	return c;
}
color color::operator*(const double d2)
{
	color c = color();
	c.red(this->red() * d2);
	c.green(this->green() * d2);
	c.blue(this->blue() * d2);
	c.alpha(this->alpha() * d2);
	return c;
}

namespace centaurus
{
	color operator+(const double d1, const color & c2)
	{
		color c = color();
		c.red(d1 + c2.red());
		c.green(d1 + c2.green());
		c.blue(d1 + c2.blue());
		c.alpha(d1 + c2.alpha());
		return c;
	}
	color operator+(const Quantum q1, const color & c2)
	{
		color c = color();
		c.redQuantum(q1 + c2.redQuantum());
		c.greenQuantum(q1 + c2.greenQuantum());
		c.blueQuantum(q1 + c2.blueQuantum());
		c.alphaQuantum(q1 + c2.alphaQuantum());
		return c;
	}

	color operator-(const double d1, const color & c2)
	{
		color c = color();
		c.red(d1 - c2.red());
		c.green(d1 - c2.green());
		c.blue(d1 - c2.blue());
		c.alpha(c2.alpha());
		return c;
	}
	color operator-(const Quantum q1, const color & c2)
	{
		color c = color();
		c.redQuantum(q1 - c2.redQuantum());
		c.greenQuantum(q1 - c2.greenQuantum());
		c.blueQuantum(q1 - c2.blueQuantum());
		c.alphaQuantum(c2.alphaQuantum());
		return c;
	}

	color operator*(const double d1, const color & c2)
	{
		color c = color();
		c.red(d1 * c2.red());
		c.green(d1 * c2.green());
		c.blue(d1 * c2.blue());
		c.alpha(d1 * c2.alpha());
		return c;
	}
}

// private
double color::saturate(const double d)
{
	double s = d;
	s = (s > 1.0) ? 1.0 : s;
	s = (s < 0.0) ? 0.0 : s;
	return s;
}
double color::to_double(const Quantum q)
{
	return double(q) / double(MaxRGB);
}
Quantum color::to_quantum(const double d)
{
	return round(d * double(MaxRGB));
}
