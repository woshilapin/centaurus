#include "buffer.hpp"

#include <iostream>
#include <cmath>
#include <limits>

using namespace std;
using namespace centaurus;

Buffer::Buffer()
{
	this->width = 80;
	this->height = 36;
	this->depth = 24;
	this->buffer = new double *[this->height];
	for (unsigned int h=0; h<this->height; h++)
	{
		this->buffer[h] = new double[this->width];
	}
}

Buffer::Buffer(const Buffer & src)
{
	this->width = src.width;
	this->height = src.height;
	this->depth = src.depth;
}

Buffer::~Buffer()
{
	for (unsigned int i=0; i<this->height; i++)
	{
		delete this->buffer[i];
	}
	delete this->buffer;
}

unsigned int Buffer::get_width()
{
	return this->width;
}

unsigned int Buffer::get_height()
{
	return this->height;
}

unsigned int Buffer::get_depth()
{
	return this->depth;
}

void Buffer::set_width(const unsigned int w)
{
	this->width = w;
}

void Buffer::set_height(const unsigned int h)
{
	this->height = h;
}

void Buffer::set_depth(const unsigned int d)
{
	this->depth = d;
}

void Buffer::set_size(
		const unsigned int w,
		const unsigned int h,
		const unsigned int d)
{
	this->width = w;
	this->height = h;
	this->depth = d;
}

double & Buffer::operator()(
					const unsigned int h,
					const unsigned int w)
{
	return this->buffer[h][w];
}

void Buffer::display(void)
{
	const char * colormap[4];
	colormap[0] = " ";
	colormap[1] = "\u2591";
	colormap[2] = "\u2592";
	colormap[3] = "\u258C";
	unsigned int color_idx = 0;

	// Upper border
	cout << "+";
	for (unsigned int w=0; w<this->get_width(); w++)
	{
		cout << "-";
	}
	cout << "+" << endl;
	for (unsigned int h=0; h<this->get_height(); h++)
	{
		cout << "|";
		for (unsigned int w=0; w<this->get_width(); w++)
		{
			color_idx = get_color_from_value((*this)(h,w));
			cout << colormap[color_idx];
		}
		cout << "|" << endl;
	}
	// Lower border
	cout << "+";
	for (unsigned int w=0; w<this->get_width(); w++)
	{
		cout << "-";
	}
	cout << "+" << endl;
}

unsigned int Buffer::get_color_from_value(const double value)
{
	const unsigned int colormap_size = 4;
	// If 'value=1', then the result will be 4 which is not a valid index [0..3]
	if (value == 1.0)
	{
		return 3;
	} else {
		return (unsigned int)(floor(value*double(colormap_size)));
	}
}
