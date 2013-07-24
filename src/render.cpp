#include "render.hpp"

#include "vector.hpp"
#include "point.hpp"
#include "triangle.hpp"

using namespace centaurus;

render::render(void)
{
}

render::render(const render &src)
{
	this->buffer_ = src.buffer_;
}

render::~render(void)
{
}

void render::run(void)
{
	point cam(0.0, 0.0, 1.0);
	point p1(-1.0,  0.0,  0.0);
	point p2( 1.0,  0.0,  0.0);
	point p3( 0.0,  1.0, -1.0);
	triangle o(p1, p2, p3);
	unsigned int width = this->buffer_.get_width();
	unsigned int height = this->buffer_.get_height();
	point pixel, I;
	ray r;
	for(unsigned int h=0; h<height; h++)
	{
		for(unsigned int w=0; w<width; w++)
		{
			pixel = point(
					float(2 * w - 1) / width - 1.0,
					float(2 * (height-h+1) - 1) / height - 1.0,
					0.0);
			r = ray(cam-pixel);
			unsigned int is_intersect = o.intersect(cam, r, I);
			this->buffer_(h,w) = 0.0;
			if (is_intersect == true)
			{
				this->buffer_(h,w) = 1.0;
			}
		}
	}
	this->buffer_.display();
	return;
}
