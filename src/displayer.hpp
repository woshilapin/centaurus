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
#ifndef __DISPLAYER_HPP__
#define __DISPLAYER_HPP__

#include <ostream>

#include <Magick++.h>

namespace centaurus
{
	class displayer
	{
		private:
		public:
			virtual ~displayer() {};
			virtual void display(const Magick::Image &, std::ostream &) = 0;
	};
}

#endif // __DISPLAYER_HPP__
