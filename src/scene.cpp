#include "scene.hpp"

using namespace centaurus;
using namespace boost::container;

Scene::Scene()
{
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

map<unsigned int, Camera> & Scene::get_cameras()
{
	return this->cameras;
}

Camera & Scene::get_camera(const unsigned int key)
{
	return this->cameras[key];
}

map<unsigned int, Light> & Scene::get_lights()
{
	return this->lights;
}

Light & Scene::get_light(const unsigned int key)
{
	return this->lights[key];
}

map<unsigned int, Object> & Scene::get_objects()
{
	return this->objects;
}

Object & Scene::get_object(const unsigned int key)
{
	return this->objects[key];
}
