#ifndef __BUFFER_HPP__
#define __BUFFER_HPP__

#define CENTAURUS_BUFFER_OUTPUT 1

namespace centaurus
{
	class Buffer
	{
		private:
			unsigned int width;
			unsigned int height;
			unsigned int depth;
		public:
			Buffer();
			Buffer(const Buffer &);
			~Buffer();

			unsigned int get_width();
			unsigned int get_height();
			unsigned int get_depth();

			void set_width(unsigned int);
			void set_height(unsigned int);
			void set_depth(unsigned int);
			void set_size(unsigned int, unsigned int, unsigned int = 24);
	};
}

#endif // __BUFFER_HPP__
