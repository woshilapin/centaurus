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

			void set_width(const unsigned int);
			void set_height(const unsigned int);
			void set_depth(const unsigned int);
			void set_size(const unsigned int,
					const unsigned int,
					const unsigned int = 24);
	};
}

#endif // __BUFFER_HPP__
