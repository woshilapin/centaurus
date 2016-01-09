TESTS = \
		vector_test \
		triangle_test
check_PROGRAMS = \
				 vector_test \
				 triangle_test

AM_CPPFLAGS = -I$(top_builddir)/src/
LDADD = \
		$(top_builddir)/src/ray.o \
		$(top_builddir)/src/triangle.o \
		-lboost_unit_test_framework
vector_test_SOURCES = vector_test.cpp $(top_builddir)/src/vector.hpp
triangle_test_SOURCES = triangle_test.cpp $(top_builddir)/src/triangle.hpp
