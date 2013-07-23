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

point triangle::intersect(const ray &ray, const point &P)
{
	point O(0.0, 0.0, 0.0);
	double t = (this->plan_offset_ + vector(P-O)*this->normal_) / 
		(ray.get_dir()*this->normal_);
	point I(P - ray.get_dir()*t);
	matrix inv(this->basis_.inverse());
	point I_proj(vector(I-this->vertices_[0]));
	float alpha = I_proj[0];
	float beta = I_proj[1];
	if ( (0.0f <= alpha && alpha <= 1.0f)
			&& (0.0f <= beta && beta <= 1.0f)
			&& (alpha+beta <= 1.0f) )
	{
		return I;
	} else {
		return P;
	}
}
