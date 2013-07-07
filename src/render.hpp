#ifndef __RENDER_HPP__
#define __RENDER_HPP__

#include "buffer.hpp"

namespace centaurus
{
	class Render
	{
		private:
			Buffer buffer;
		public:
			Render(void);
			Render(const Render &);
			~Render(void);

			void run(void);
	};
}

#endif // __RENDER_HPP__
