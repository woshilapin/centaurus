#include "point.hpp"

#include <boost/numeric/ublas/vector.hpp>

using namespace boost::numeric::ublas;

using namespace centaurus;

Point::Point()
{
	this->pos = vector<double>(3);
}

Point::Point(const Point & src)
{
	this->pos = src.pos;
}

Point::~Point()
{
}

void Point::set_pos(const vector<double> pos)
{
	this->pos = pos;
}

vector<double> Point::get_pos(void)
{
	return this->pos;
}
