# include "localization.hpp"

void init_localization (char * package, char * locale_dir)
{
    setlocale (LC_ALL, "");
    bindtextdomain (package, locale_dir);
    textdomain (package);
}
