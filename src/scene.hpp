#ifndef __SCENE_HPP__
#define __SCENE_HPP__

#include <boost/container/map.hpp>

#include "camera.hpp"
#include "light.hpp"
#include "object.hpp"

namespace centaurus
{
	using namespace boost::container;
	class Scene
	{

		private:
			map<unsigned int, Camera &> cameras;
			map<unsigned int, Light &> lights;
			map<unsigned int, Object &> objects;
		public:
			Scene();
			Scene(const Scene &);
			~Scene();

			map<unsigned int, Camera &> & get_cameras();
			// Camera & get_camera(const unsigned int);
			map<unsigned int, Light &> & get_lights();
			// Light & get_light(const unsigned int);
			map<unsigned int, Object &> & get_objects();
			// Object & get_object(const unsigned int);
	};
}

#endif // __SCENE_HPP__
