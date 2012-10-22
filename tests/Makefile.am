TESTS = check_factorial
check_PROGRAMS = check_factorial
check_factorial_SOURCES = check_factorial.c factorial.h
check_factorial_CPPFLAGS = \
	-I$(top_srcdir)/src/
check_factorial_LDADD = \
	@CHECK_LIBS@ \
	$(top_srcdir)/src/factorial.o
