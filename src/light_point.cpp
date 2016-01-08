#include "light_point.hpp"

using namespace centaurus;

light_point::light_point()
{
	point p(0.0f, 1.0f, 0.0f);
	this->position_ = p;
}

light_point::light_point(const point &p)
{
	this->position_ = p;
}

light_point::light_point(const light_point &src)
{
	this->position_ = src.position_;
}

light_point::~light_point()
{
}

ray light_point::get_ray(const point &p)
{
	ray r(vector(this->position_ - p));
	return r;
}
