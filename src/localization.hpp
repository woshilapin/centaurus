# ifndef __LOCALIZATION_H__
# define __LOCALIZATION_H__

# include <locale.h>

# include "gettext.h"

# define _(string) gettext(string)

void init_localization (const char * const, const char * const);

# endif // __LOCALIZATION_H__
