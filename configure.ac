AC_PREREQ([2.69])
AC_INIT([centaurus], [0.1], [woshilapin@tuziwo.info])
AC_COPYRIGHT([Copyright 2012 woshilapin])
AC_CONFIG_AUX_DIR([build-aux])
AC_CONFIG_MACRO_DIR([m4])
AC_CONFIG_SRCDIR([src/centaurus.cpp])
AC_CONFIG_HEADERS([config.h])

AM_INIT_AUTOMAKE([gnu 1.9 --warnings=all --warnings=error dist-bzip2 parallel-tests color-tests])

# To include MacPorts directories
CFLAGS="-I/opt/local/include $CFLAGS"
CXXFLAGS="-I/opt/local/include $CXXFLAGS"
LDFLAGS="-I/opt/local/lib $LDFLAGS"

# Checks for programs.
AC_PROG_AWK
AC_PROG_SED
AC_PROG_CXX
AC_PROG_CC
AM_PROG_CC_C_O
AC_PROG_CPP
AC_GNU_SOURCE
AC_PROG_INSTALL
AC_PROG_LN_S
AC_PROG_MAKE_SET

# Checks for libraries.
PKG_CHECK_MODULES([CHECK], [check >= 0.9.4])

# Checks for header files.
AC_STDC_HEADERS
AC_CHECK_HEADERS([libintl.h locale.h stdlib.h string.h])

# Checks for typedefs, structures, and compiler characteristics.
AC_C_INLINE
AC_TYPE_SIZE_T

# Checks for library functions.
AC_FUNC_MALLOC
AC_CHECK_FUNCS([setlocale])

# Check for gettext internationalization
AM_GNU_GETTEXT_VERSION([0.18.1])
AM_GNU_GETTEXT([external])

# activate all debug informations
AC_MSG_CHECKING([whether debug options should be activated])	
AC_ARG_ENABLE(debug,
	[AS_HELP_STRING([--enable-debug],
		[enable debug flags (coverage, profiling...)])
	],
	[enable_debug=$enableval],
	[enable_debug='no'])
AC_MSG_RESULT($enable_debug)

# gcov coverageereporting
AC_ARG_ENABLE(gcov,
	[AS_HELP_STRING([--enable-gcov],
		[enable 'gcov' coverage support])
	],
	[enable_gcov=$enableval],
	[enable_gcov='no'])

# lcov coverage visualization
AC_ARG_ENABLE(lcov,
	[AS_HELP_STRING([--enable-lcov],
		[enable 'lcov' profiling visualization])
	],
	[enable_lcov=$enableval],
	[enable_lcov='no'])

# gprof profiling
AC_ARG_ENABLE(gprof,
	[AS_HELP_STRING([--enable-gprof],
		[enable 'gprof' profiling support])
	],
	[enable_gprof=$enableval],
	[enable_gprof='no'])

# valgrind profiling
AC_ARG_ENABLE(valgrind,
	[AS_HELP_STRING([--enable-valgrind],
		[enable 'valgrind' profiling support])
	],
	[enable_valgrind=$enableval],
	[enable_valgrind='no'])

# kcachegrind GUI profiling
AC_ARG_ENABLE(kcachegrind,
	[AS_HELP_STRING([--enable-kcachegrind],
		[enable 'kcachegrind' profiling support])
	],
	[enable_kcachegrind=$enableval],
	[enable_kcachegrind='no'])

if test "$enable_debug" = 'yes';
then
	CFLAGS="-Wall -Wextra $CFLAGS"
	CXXFLAGS="-Wall -Wextra $CXXFLAGS"
	LDFLAGS="-Wall -Wextra $LDFLAGS"
	enable_lcov='yes'
	enable_gcov='yes'
	enable_gprof='yes'
	enable_valgrind='yes'
	enable_kcachegrind='yes'
fi
if test "$enable_lcov" = 'yes';
then
	enable_gcov='yes'
	AC_CHECK_PROG(LCOV, lcov, lcov)
	AC_CHECK_PROG(GENHTML, genhtml, genhtml)
	LCOVFLAGS="--no-checksum --no-list-full-path"
	GENHTMLFLAGS="--title 'Test coverage reporting' --legend --show-details --highlight --keep-descriptions"
fi
AC_SUBST(LCOVFLAGS)
AC_SUBST(GENHTMLFLAGS)
if test "$enable_gcov" = 'yes';
then
	AC_CHECK_LIB(gcov,_gcov_init)
	AC_CHECK_LIB(gcov,__gcov_init)
	CFLAGS="--coverage $CFLAGS"
	CXXFLAGS="--coverage $CXXFLAGS"
	LDFLAGS="--coverage $LDFLAGS"
fi
if test "$enable_gprof" = 'yes';
then
	AC_CHECK_PROG(GPROF,gprof,gprof)
	CFLAGS="-pg $CFLAGS"
	CXXFLAGS="-pg $CXXFLAGS"
	LDFLAGS="-pg $LDFLAGS"
fi
if test "$enable_kcachegrind" = 'yes';
then
	enable_valgrind='yes'
	AC_CHECK_PROG(KCACHEGRIND,kcachegrind,kcachegrind)
fi
if test "$enable_valgrind" = 'yes';
then
	AC_CHECK_PROG(VALGRIND,valgrind,valgrind)
	CFLAGS="-pg $CFLAGS"
	CXXFLAGS="-pg $CXXFLAGS"
	LDFLAGS="-pg $LDFLAGS"
fi
AM_CONDITIONAL(HAVE_DEBUG, test "$enable_debug" = 'yes')
AM_CONDITIONAL(HAVE_GCOV, test "$enable_gcov" = 'yes')
AM_CONDITIONAL(HAVE_LCOV, test "$enable_lcov" = 'yes')
AM_CONDITIONAL(HAVE_GPROF, test "$enable_gprof" = 'yes')
AM_CONDITIONAL(HAVE_VALGRIND, test "$enable_valgrind" = 'yes')
AM_CONDITIONAL(HAVE_KCACHEGRIND, test "$enable_kcachegrind" = 'yes')

# Check for doxygen documentation
DX_DOXYGEN_FEATURE(ON)
DX_DOT_FEATURE(ON)
DX_HTML_FEATURE(ON)
DX_CHM_FEATURE(OFF)
DX_CHI_FEATURE(OFF)
DX_MAN_FEATURE(ON)
DX_RTF_FEATURE(OFF)
DX_XML_FEATURE(OFF)
DX_PDF_FEATURE(OFF)
DX_PS_FEATURE(OFF)
DX_INIT_DOXYGEN([centaurus], [.doxygenrc], [doc])

AC_CONFIG_FILES([
	Makefile
	po/Makefile.in
	src/Makefile
	tests/Makefile
])
AC_OUTPUT
