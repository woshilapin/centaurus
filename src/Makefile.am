localedir = $(datadir)/locale
AM_CPPFLAGS = \
	-DLOCALEDIR=\"$(localedir)\"

bin_PROGRAMS = centaurus
centaurus_SOURCES = \
	centaurus.c \
	localization.c \
	localization.h \
	factorial.c \
	factorial.h \
	render.cpp \
	render.hpp \
	buffer.cpp \
	buffer.hpp \
	camera.cpp \
	camera.hpp \
	light.cpp \
	light.hpp \
	object.cpp \
	object.hpp \
	scene.cpp \
	scene.hpp \
	gettext.h
centaurus_LDADD = \
	@LTLIBINTL@

astyle:
	ARTISTIC_STYLE_OPTIONS=$(top_builddir)/.astylerc ; astyle $(centaurus_SOURCES)

count:
	sloccount --datadir $(top_builddir)/.slocdata --wide $(srcdir)

splint:
	splint $(centaurus_SOURCES)

cppcheck:
	cppcheck --enable=all --report-progress --template 'gcc' $(centaurus_SOURCES)
