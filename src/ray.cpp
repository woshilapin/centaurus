#include "ray.hpp"

#include <boost/numeric/ublas/vector.hpp>

using namespace boost::numeric::ublas;

using namespace centaurus;

Ray::Ray()
{
	this->dir = vector<double>(3);
}

Ray::Ray(const Ray & src)
{
	this->dir = src.dir;
}

Ray::~Ray()
{
}

void Ray::set_dir(const vector<double> dir)
{
	this->dir = dir;
}

vector<double> Ray::get_dir(void)
{
	return this->dir;
}
