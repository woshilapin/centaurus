localedir = $(datadir)/locale
AM_CPPFLAGS = \
	-DLOCALEDIR=\"$(localedir)\"

bin_PROGRAMS = centaurus
centaurus_SOURCES = \
	buffer.cpp \
	buffer.hpp \
	centaurus.cpp \
	gettext.h \
	light.hpp \
	light_sun.cpp \
	light_sun.hpp \
	light_point.cpp \
	light_point.hpp \
	localization.cpp \
	localization.hpp \
	matrix.hpp \
	object.hpp \
	point.hpp \
	ray.cpp \
	ray.hpp \
	render.cpp \
	render.hpp \
	triangle.cpp \
	triangle.hpp \
	vector.hpp
# camera.cpp
# camera.hpp
# light.cpp
# light.hpp
# scene.cpp
# scene.hpp

centaurus_LDADD = \
	@LTLIBINTL@

astyle:
	ARTISTIC_STYLE_OPTIONS=$(top_builddir)/.astylerc ; astyle $(centaurus_SOURCES)

count:
	sloccount --datadir $(top_builddir)/.slocdata --wide $(srcdir)

splint:
	splint -fileextensions cpp $(centaurus_SOURCES)

cppcheck:
	cppcheck --enable=all --report-progress --template 'gcc' $(centaurus_SOURCES)
