TESTS = \
		check_factorial \
		check_vector \
		check_triangle
check_PROGRAMS = \
				 check_factorial \
				 check_vector \
				 check_triangle

AM_CPPFLAGS = -I$(top_srcdir)/src/
LDADD = @CHECKLIBS@  /opt/local/lib/libboost_unit_test_framework-mt.a

check_factorial_SOURCES = check_factorial.c factorial.h
check_factorial_LDADD = \
	@CHECK_LIBS@ \
	$(top_srcdir)/src/factorial.o


check_vector_SOURCES = check_vector.cpp
check_triangle_SOURCES = check_triangle.cpp
check_triangle_LDADD = \
					   $(LDADD) \
					   $(top_srcdir)/src/ray.o \
					   $(top_srcdir)/src/triangle.o
