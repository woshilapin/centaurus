#ifndef __RENDER_HPP__
#define __RENDER_HPP__

#include "buffer.hpp"

namespace centaurus
{
	class render
	{
		private:
			buffer buffer_;
		public:
			render(void);
			render(const render &);
			~render(void);

			void run(void);
	};
}

#endif // __RENDER_HPP__
