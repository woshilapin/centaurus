import sys
from doconv_plugins import *
from doconv_plugins.db_to_tex import *
from doconv_plugins.tex_to_dvi import *
from doconv_plugins.dvi_to_ps import *
from doconv_plugins.ps_to_pdf import *

myCompiler = Compiler()
myCompiler.args(sys.argv)
myCompiler.add_file("centaurus-doc.xml", myCompiler.DB)
myCompiler.add_file("latex-xsl/docbook-latex.xsl", myCompiler.XSL)
myCompiler.add_file("centaurus-doc.tex", myCompiler.TEX)
myCompiler.add_file("centaurus-doc.dvi", myCompiler.DVI)
myCompiler.add_file("centaurus-doc.ps", myCompiler.PS)
myCompiler.add_file("centaurus-doc.pdf", myCompiler.PDF)
myCompiler.add_converter(DB_to_TEX, myCompiler.DB, myCompiler.TEX, db_filename=myCompiler.files[myCompiler.DB], tex_filename=myCompiler.files[myCompiler.TEX], xsl_filename=myCompiler.files[myCompiler.XSL])
myCompiler.add_converter(TEX_to_DVI, myCompiler.TEX, myCompiler.DVI, tex_filename=myCompiler.files[myCompiler.TEX], dvi_filename=myCompiler.files[myCompiler.DVI])
myCompiler.add_converter(DVI_to_PS, myCompiler.DVI, myCompiler.PS, dvi_filename=myCompiler.files[myCompiler.DVI], ps_filename=myCompiler.files[myCompiler.PS])
myCompiler.add_converter(PS_to_PDF, myCompiler.PS, myCompiler.PDF, ps_filename=myCompiler.files[myCompiler.PS], pdf_filename=myCompiler.files[myCompiler.PDF])
#myCompiler.convert()
