#include "render.hpp"

using namespace centaurus;

Render::Render(void)
{
}

Render::Render(const Render & src)
{
	this->buffer = src.buffer;
}

Render::~Render(void)
{
}

void Render::run(void)
{
	for(unsigned int h=0; h<this->buffer.get_height(); h++)
	{
		for(unsigned int w=0; w<this->buffer.get_width(); w++)
		{
			this->buffer(h,w) = 0.0;
			if (h>10 && h<30 && w>10 && w<20)
			{
				this->buffer(h,w) = 1.0;
			}
		}
	}
	this->buffer.display();
	return;
}
