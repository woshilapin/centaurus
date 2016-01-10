TESTS = \
		matrix_test \
		point_test \
		vector_test \
		triangle_test
check_PROGRAMS = \
				 matrix_test \
				 point_test \
				 vector_test \
				 triangle_test

AM_CPPFLAGS = -I$(top_builddir)/src/
LDADD = \
		$(top_builddir)/src/ray.o \
		$(top_builddir)/src/triangle.o \
		-lboost_unit_test_framework
matrix_test_SOURCES = matrix_test.cpp $(top_builddir)/src/matrix.hpp
point_test_SOURCES = point_test.cpp $(top_builddir)/src/point.hpp
vector_test_SOURCES = vector_test.cpp $(top_builddir)/src/vector.hpp
triangle_test_SOURCES = triangle_test.cpp $(top_builddir)/src/triangle.hpp
