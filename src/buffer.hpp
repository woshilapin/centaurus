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
