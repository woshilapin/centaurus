#ifndef __RENDER_HPP__
#define __RENDER_HPP__

#include <boost/container/map.hpp>

#include "buffer.hpp"

namespace centaurus
{
	class Render
	{
		private:
			boost::container::map<unsigned short, Buffer> buffers;
		public:
			Render();
			Render(const Render &);
			~Render();
	};
}

#endif // __RENDER_HPP__
