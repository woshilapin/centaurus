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
#include "scene.hpp"

using namespace centaurus;
using namespace boost::container;

Scene::Scene()
{
	this->cameras = map<unsigned int, Camera &>();
}

Scene::Scene(const Scene & src)
{
	this->cameras = src.cameras;
	this->lights = src.lights;
	this->objects = src.objects;
}

Scene::~Scene()
{
}

map<unsigned int, Camera &> & Scene::get_cameras()
{
	return this->cameras;
}

// Camera & Scene::get_camera(const unsigned int key)
// {
// 	return this->cameras[key];
// }

map<unsigned int, Light &> & Scene::get_lights()
{
	return this->lights;
}

// Light & Scene::get_light(const unsigned int key)
// {
// 	return this->lights[key];
// }

map<unsigned int, Object &> & Scene::get_objects()
{
	return this->objects;
}

// Object & Scene::get_object(const unsigned int key)
// {
// 	return this->objects[key];
// }
