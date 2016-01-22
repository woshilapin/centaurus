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
#include "render.hpp"

#include <iostream>
#include <fstream>

#include "vector.hpp"
#include "point.hpp"
#include "triangle.hpp"
#include "light_point.hpp"
#include "factory_displayer.hpp"

using namespace centaurus;
using namespace Magick;

render::render(void)
{
	Geometry g(80, 36);
	this->image_.size(g);
}

render::render(const render &src)
{
	this->image_ = src.image_;
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
	point pl(0.25f, 0.25f, 0.25f);
	light_point l(pl);
	unsigned int width = this->image_.columns();
	unsigned int height = this->image_.rows();
	float color = 0.0f;
	point pixel, I;
	ray r;
	ray reflect_ray, light_ray;
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
			Color background(0, 0, 0);
			this->image_.pixelColor(w, h, background);
			if (is_intersect == true)
			{
				reflect_ray = o.reflect(r);
				light_ray = l.get_ray(I);
				color = o.get_normal() * light_ray.get_dir();
				Color c(color*MaxRGB, color*MaxRGB, color*MaxRGB);
				this->image_.pixelColor(w, h, c);
			}
		}
	}
	displayer * txt = factory_displayer::create(factory_displayer::TXT);
	std::ofstream txt_file("image.txt", std::ofstream::out);
	txt->display(this->image_, txt_file);
	txt->display(this->image_, std::cout);
	displayer * png = factory_displayer::create(factory_displayer::PNG);
	std::ofstream png_file("image.png", std::ofstream::out | std::ofstream::binary);
	png->display(this->image_, png_file);
	return;
}
