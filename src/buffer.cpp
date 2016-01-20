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
#include "buffer.hpp"

#include <limits>

using namespace std;
using namespace centaurus;

buffer::buffer()
{
	this->width_ = 80;
	this->height_ = 36;
	this->depth_ = 24;
	this->buffer_ = new double *[this->height_];
	for (unsigned int h=0; h<this->height_; h++)
	{
		this->buffer_[h] = new double[this->width_];
	}
}

buffer::buffer(const buffer & src)
{
	this->width_ = src.width_;
	this->height_ = src.height_;
	this->depth_ = src.depth_;
}

buffer::~buffer()
{
	for (unsigned int i=0; i<this->height_; i++)
	{
		delete this->buffer_[i];
	}
	delete this->buffer_;
}

unsigned int buffer::get_width() const
{
	return this->width_;
}

unsigned int buffer::get_width()
{
	return this->width_;
}

unsigned int buffer::get_height() const
{
	return this->height_;
}

unsigned int buffer::get_height()
{
	return this->height_;
}

unsigned int buffer::get_depth() const
{
	return this->depth_;
}

unsigned int buffer::get_depth()
{
	return this->depth_;
}

void buffer::set_width(const unsigned int w)
{
	this->width_ = w;
}

void buffer::set_height(const unsigned int h)
{
	this->height_ = h;
}

void buffer::set_depth(const unsigned int d)
{
	this->depth_ = d;
}

void buffer::set_size(
		const unsigned int w,
		const unsigned int h,
		const unsigned int d)
{
	this->width_ = w;
	this->height_ = h;
	this->depth_ = d;
}

double buffer::operator()(
					const unsigned int h,
					const unsigned int w) const
{
	return this->buffer_[h][w];
}

double & buffer::operator()(
					const unsigned int h,
					const unsigned int w)
{
	return this->buffer_[h][w];
}
