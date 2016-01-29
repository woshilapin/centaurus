[![Travis](https://img.shields.io/travis/woshilapin/centaurus.svg)](https://travis-ci.org/woshilapin/centaurus)

centaurus
=========

The relativist ray-tracer

# Prerequisite

You should install a bunch of dependencies to compile `centaurus`:
* autoconf (> 2.68)
* automake (> 1.9)
* autopoint (> 0.18.1)
* libboost-dev
* libmagick++-dev
* [optional] doxygen
* [optional] libboost-test-dev

# Compile

To compile `centaurus`, you'll have to use first Autotools to generate Makefiles
and then you can compile

```
git clone https://github.com/woshilapin/centaurus.git
cd centaurus/
./autogen.sh
make
```

# Test

You can also launch the testing procedures.

```
make check
```

# Install

You should be able to install it on your system with the usual install
procedures of Autotools.

```
make install
```
