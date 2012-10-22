#!/bin/bash - 
#===============================================================================
#
#          FILE:  autogen.sh
#
#         USAGE:  ./autogen.sh
#
#   DESCRIPTION: Automatically generate the files for autotools
# 
#       OPTIONS:  ---
#  REQUIREMENTS:  ---
#          BUGS:  ---
#         NOTES:  ---
#        AUTHOR: woshilapin
#       COMPANY:
#       CREATED: 21.10.2012 23:00:00 CEST
#      REVISION: 0.1
#===============================================================================

set -o nounset                              # Treat unset variables as an error

function print_msg
{
	echo "autogen.sh: $1"
}

print_msg "running: autoreconf --install --verbose"
autoreconf --install --verbose || exit 1
