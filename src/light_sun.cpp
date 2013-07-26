#include "light_sun.hpp"

using namespace centaurus;

light_sun::light_sun()
{
	vector v(0.0f, -1.0f, 0.0f);
	this->direction_ = v;
}

light_sun::light_sun(const vector &v)
{
	this->direction_ = v.normalize();
}

light_sun::light_sun(const light_sun &src)
{
	this->direction_ = src.direction_;
}

light_sun::~light_sun()
{
}

ray light_sun::get_ray(const point &p)
{
	ray r(-this->direction_);
	return r;
}
