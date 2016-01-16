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
#ifndef __BUFFER_HPP__
#define __BUFFER_HPP__

#define CENTAURUS_BUFFER_OUTPUT 1

namespace centaurus
{
	class buffer
	{
		private:
			unsigned int width_;
			unsigned int height_;
			unsigned int depth_;

			double ** buffer_;
		public:
			buffer();
			buffer(const buffer &);
			~buffer();

			unsigned int get_width();
			unsigned int get_height();
			unsigned int get_depth();

			void set_width(const unsigned int);
			void set_height(const unsigned int);
			void set_depth(const unsigned int);
			void set_size(const unsigned int,
					const unsigned int,
					const unsigned int = 24);

			double & operator()(
					const unsigned int,
					const unsigned int);

			void display(void);

		private:
			unsigned int get_color_from_value(const double);
	};
}

#endif // __BUFFER_HPP__
