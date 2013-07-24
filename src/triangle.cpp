#include "triangle.hpp"

#include <vector>
#include "vector.hpp"

using namespace centaurus;
using namespace std;

triangle::triangle()
{
}

triangle::triangle(
		const point &P1,
		const point &P2,
		const point &P3)
{
	point O(0.0, 0.0, 0.0);
	this->vertices_ = std::vector<point>(3);
	this->vertices_[0] = P1;
	this->vertices_[1] = P2;
	this->vertices_[2] = P3;
	this->normal_ = vector(P2-P1)^vector(P3-P1);
	this->plan_offset_ = -(vector(P1-O)*this->normal_);
	this->basis_ = matrix(
			vector(P2-P1),
			vector(P3-P1),
			vector(0.0f,0.0f,1.0f));
}

triangle::triangle(
		const point &P1,
		const point &P2,
		const point &P3,
		const vector &normal)
{
	point O(0.0, 0.0, 0.0);
	this->vertices_ = std::vector<point>(3);
	this->vertices_[0] = P1;
	this->vertices_[1] = P2;
	this->vertices_[2] = P3;
	this->normal_ = normal;
	this->plan_offset_ = -(vector(P1-O)*this->normal_);
}

triangle::~triangle()
{
}

#include <boost/numeric/ublas/matrix.hpp>
#include <boost/numeric/ublas/vector.hpp>
#include <boost/numeric/ublas/vector_expression.hpp>
#include <boost/numeric/ublas/matrix_expression.hpp>
using namespace boost::numeric;
bool triangle::intersect(const point &P, const ray &ray, point &I)
{
	point I_proj;
	matrix inv;
	float t = 0.0f;
	float alpha = 0.0f, beta = 0.0f;
	t = ray.get_dir()*this->normal_;
	if (t == 0.0f)
	{
		return this->OBJECT_NO_INTERSECTION;
	}
	t = (this->plan_offset_ + vector(P)*this->normal_) / t;
	I = P - ray.get_dir()*t;
	inv = this->basis_.inverse();
	I_proj = vector(I-this->vertices_[0]);
	I_proj = prod(I_proj, inv);
	alpha = I_proj[0];
	beta = I_proj[1];
	if ( (0.0f <= alpha && alpha <= 1.0f)
			&& (0.0f <= beta && beta <= 1.0f)
			&& (alpha+beta <= 1.0f) )
	{
		return this->OBJECT_INTERSECTION;
	} else {
		return this->OBJECT_NO_INTERSECTION;
	}
}
