#include "render.hpp"

using namespace centaurus;

Render::Render()
{
}

Render::Render(const Render & src)
{
	this->buffers = src.buffers;
}

Render::~Render()
{
}
