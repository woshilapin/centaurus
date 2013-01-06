#include "buffer.hpp"

using namespace centaurus;

Buffer::Buffer()
{
	this->width = 100;
	this->height = 100;
	this->depth = 24;
}

Buffer::Buffer(const Buffer & src)
{
	this->width = src.width;
	this->height = src.height;
	this->depth = src.depth;
}

Buffer::~Buffer()
{
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
