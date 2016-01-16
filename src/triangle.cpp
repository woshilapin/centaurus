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
#include "triangle.hpp"


using namespace centaurus;

triangle::triangle()
{
	point p;
	vector v;
	matrix m;
	this->vertices_.push_back(p);
	this->vertices_.push_back(p);
	this->vertices_.push_back(p);
	this->normal_ = v;
	this->plan_offset_ = 0.0f;
	this->basis_ = m;
}

triangle::triangle(
		const point &p1,
		const point &p2,
		const point &p3)
{
	this->vertices_ = std::vector<point>(3);
	this->vertices_[0] = p1;
	this->vertices_[1] = p2;
	this->vertices_[2] = p3;
	this->normal_ = vector(p2-p1)^vector(p3-p1);
	this->normal_ = this->normal_.normalize();
	this->plan_offset_ = -(vector(p1)*this->normal_);
	this->set_basis(p1, p2, p3);
}

triangle::triangle(
		const point &p1,
		const point &p2,
		const point &p3,
		const vector &normal)
{
	this->vertices_ = std::vector<point>(3);
	this->vertices_[0] = p1;
	this->vertices_[1] = p2;
	this->vertices_[2] = p3;
	this->normal_ = normal.normalize();
	this->plan_offset_ = -(vector(p1)*this->normal_);
	this->set_basis(p1, p2, p3);
}

triangle::~triangle()
{
}

std::vector<point> triangle::get_vertices(void) const
{
	return this->vertices_;
}
std::vector<point> triangle::get_vertices(void)
{
	return this->vertices_;
}
point triangle::get_vertice(const unsigned int index) const
{
	return this->vertices_[index];
}
point triangle::get_vertice(const unsigned int index)
{
	return this->vertices_[index];
}
vector triangle::get_normal(void) const
{
	return this->normal_;
}
vector triangle::get_normal(void)
{
	return this->normal_;
}
float triangle::get_plan_offset(void) const
{
	return this->plan_offset_;
}
float triangle::get_plan_offset(void)
{
	return this->plan_offset_;
}
matrix triangle::get_basis(void) const
{
	return this->basis_;
}
matrix triangle::get_basis(void)
{
	return this->basis_;
}

void triangle::set_basis(const point &p1, const point &p2, const point &p3)
{
	this->basis_ = matrix(
			vector(p2-p1),
			vector(p3-p1),
			vector(0.0f,0.0f,1.0f));
	return;
}

bool triangle::intersect(const point &p, const ray &r, point &i)
{
	point i_proj;
	matrix inv;
	float t = 0.0f;
	float alpha = 0.0f, beta = 0.0f;
	t = r.get_dir()*this->normal_;
	if (t == 0.0f)
	{
		return this->OBJECT_NO_INTERSECTION;
	}
	t = (this->plan_offset_ + vector(p)*this->normal_) / t;
	i = p - r.get_dir()*t;
	inv = this->basis_.inverse();
	i_proj = vector(i-this->vertices_[0]);
	i_proj = prod(i_proj, inv);
	alpha = i_proj[0];
	beta = i_proj[1];
	if ( (0.0f <= alpha && alpha <= 1.0f)
			&& (0.0f <= beta && beta <= 1.0f)
			&& (alpha+beta <= 1.0f) )
	{
		return this->OBJECT_INTERSECTION;
	} else {
		return this->OBJECT_NO_INTERSECTION;
	}
}

ray triangle::reflect(const ray &r)
{
	ray reflect_ray(r.get_dir() + 2*(r.get_dir()*this->normal_)*this->normal_);
	return reflect_ray;
}
