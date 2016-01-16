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
