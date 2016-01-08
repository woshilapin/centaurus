# include "localization.hpp"

void init_localization (
		const char * const package,
		const char * const locale_dir)
{
    setlocale (LC_ALL, "");
    bindtextdomain (package, locale_dir);
    textdomain (package);
}
