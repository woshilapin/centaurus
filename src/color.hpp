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
#ifndef __COLOR_HPP__
#define __COLOR_HPP__

#include <Magick++.h>

namespace centaurus
{
	class color : public Magick::Color
	{
		private:
			// Max delta between 2 Quantums due to the conversion to/from double (round)
			static const Magick::Quantum quantum_epsilon = 1;

		public:
			color();
			color(const double);
			color(const double, const double);
			color(const double, const double, const double);
			color(const double, const double, const double, const double);
			~color();

			double red() const;
			double red();
			void red(const double);
			double green() const;
			double green();
			void green(const double);
			double blue() const;
			double blue();
			void blue(const double);
			double alpha() const;
			double alpha();
			void alpha(const double);

			bool operator==(const color &);
			bool operator!=(const color &);

			color negative();
			color operator-();
			color operator+(const color &);
			color operator+(const double);
			color operator+(const Magick::Quantum);
			color operator-(const color &);
			color operator-(const double);
			color operator-(const Magick::Quantum);
			color operator*(const color &);
			color operator*(const double);

			friend color operator+(const double, const color &);
			friend color operator+(const Magick::Quantum, const color &);
			friend color operator-(const double, const color &);
			friend color operator-(const Magick::Quantum, const color &);
			friend color operator*(const double, const color &);

			static double saturate(const double);
			static double to_double(const Magick::Quantum);
			static Magick::Quantum to_quantum(const double);
	};
}

#endif // __COLOR_HPP__
