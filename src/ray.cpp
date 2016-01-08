#include "ray.hpp"

#include "vector.hpp"

using namespace centaurus;

ray::ray()
{
	this->dir_ = vector();
}

ray::ray(const vector &v)
{
	this->dir_ = v / v.norm();
}

ray::ray(const ray &src)
{
	this->dir_ = src.dir_;
}

ray::~ray()
{
}

vector ray::get_dir(void) const
{
	return this->dir_;
}

vector &ray::get_dir(void)
{
	return this->dir_;
}
