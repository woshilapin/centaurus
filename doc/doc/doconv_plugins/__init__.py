import io
import posix
import os.path
import warnings
import linecache

def file_time(filename):
	try:
		modified_time = os.path.getmtime(filename)
	except OSError:
		modified_time = 0
	return modified_time

class Converter(object):
	def __init__(self):
		raise NotImplementedError("A '__init__' must be implemented.")
	def check_older(self):
		raise NotImplementedError("A 'check_older' must be implemented.")
	def parse(self):
		raise NotImplementedError("A 'parse' must be implemented.")
	def convert(self):
		raise NotImplementedError("A 'convert' must be implemented.")

class DocumentTypes(object):
	TEX = "tex"
	LATEX = "tex"
	DOCBOOK = "db"
	DB = "db"
	XSL = "xsl"
	XSLT = "xsl"
	DVI = "dvi"
	PS = "ps"
	POSTSCRIPT = "ps"
	PDF = "pdf"

	def __init__(self):
		self._constants = [constant for constant in dir(self) if constant.isupper()]
		self.doc_types = [getattr(self, type) for type in self._constants]

class Shell_color(str):
	GRAY = "\033[0;30m"
	LIGHT_RED = "\033[0;31m"
	GREEN = "\033[0;32m"
	YELLOW = "\033[0;33m"
	BLUE = "\033[0;34m"
	MAGENTA = "\033[0;35m"
	CYAN = "\033[0;36m"
	WHITE = "\033[0;37m"
	RED = "\033[0;38m"
	DEFAULT = "\033[0;m"
	BOLD_GRAY = "\033[1;30m"
	BOLD_LIGHT_RED = "\033[1;31m"
	BOLD_GREEN = "\033[1;32m"
	BOLD_YELLOW = "\033[1;33m"
	BOLD_BLUE = "\033[1;34m"
	BOLD_MAGENTA = "\033[1;35m"
	BOLD_CYAN = "\033[1;36m"
	BOLD_WHITE = "\033[1;37m"
	BOLD_RED = "\033[1;38m"
	BOLD_DEFAULT = "\033[1;m"

class Error(Exception):
	"""
	The Error class for doconv
	"""
	def __init__(self, filename="", msg="", line_number=0, char_number=0, bibtex=False, reference=False):
		"""
		Initiate the Error class
		"""
		self.type = "error"
		self.filename = filename
		self.msg = msg
		self.line_number = line_number
		self.char_number = char_number
		self.bibtex = bibtex
		self.reference = reference

	def __str__(self):
		"""
		Format the output string for exception.
		"""
		return_str = ""
		return_str += self.filename
		return_str += ":"
		if self.line_number is not 0:
			return_str += str(self.line_number)
			return_str += ":"
		if self.char_number is not 0:
			return_str += str(self.char_number)
			return_str += ":"
		if self.type == "error":
			return_str += Shell_color.BOLD_LIGHT_RED
		elif self.type == "warning":
			return_str += Shell_color.BOLD_YELLOW
		return_str += " "+self.type
		return_str += Shell_color.DEFAULT
		return_str += ":"
		return_str += " "+self.msg.strip()
		src_line = ""
		if self.line_number is not 0:
			src_line = linecache.getline(self.filename, self.line_number)
			src_line = src_line.replace("\t", ' ')
		if src_line is not "":
			return_str += "\n{0}{1}{2}".format(Shell_color.WHITE, src_line[:-1], Shell_color.DEFAULT) # [:-1] removes the '\n' of the source line
		if self.char_number is not 0:
			mark_line = "\n"
			for x in list(range(self.char_number-1)):
				mark_line += ' '
			mark_line += "{0}^{1}".format(Shell_color.BOLD_MAGENTA, Shell_color.DEFAULT)
			return_str += mark_line
		return return_str

class Warn(Error):
	"""
	The Warn (herited from Error) class for doconv
	"""
	def __init__(self, filename="", msg="", line_number=0, char_number=0, bibtex=False, reference=False):
		"""
		Initiate the Warn class
		"""
		super(Warn, self).__init__(filename, msg, line_number, char_number, bibtex, reference)
		self.type = "warning"

class Compiler(DocumentTypes):
	class State(object):
		INIT = 0
		INFILE = 1
		OUTFILE = 2
		OTHERFILE = 3
	def __init__(self):
		DocumentTypes.__init__(self)
		self.converters = {}
		self.chain_convert = []
		self.files = {}
		self._state = self.State.INIT
		self._logs = {}
		self.exceptions = []
		self.max_errors = 2
		self.max_warns = 16
		self.from = None
		self.to = None
	def quit(self):
		print("doconv is quitting...")
		quit()
	def _is_file_with_extension(self, arg):
		for ext in self.doc_types:
			if arg.endswith('.'+ext):
				return True
		return False
	def _is_file_type_option(self, arg):
		for ext in self.doc_types:
			if arg is "--i-"+ext:
				return True
			if arg is "--o-"+ext:
				return True
		return False
	def _is_option(self, arg):
		if arg is "-d" or arg is "--draft":
			return True
		return False
	def _is_file(self, arg):
		return True
	def _parse_file_with_extension(self, arg):
		return arg
	def _parse_file_type_option(self, arg):
		pass
	def _parse_option(self, arg):
		draft = False
		if arg is "-d" or arg is "--draft":
			draft = True
		return (draft)
	def _parse_file(self, arg):
		pass
	def _check_INIT(self, arg):
		if self._is_file_with_extension(arg):
			self._parse_file_with_extension(arg)
		elif self._is_file_type_option(arg):
			self._parse_file_type_option(arg)
		elif self._is_option(arg):
			self._parse_option(arg)
	def _check_FILE(self, arg):
		if self._is_file(arg):
			self._parse_file(arg)
	def args(self, argv):
		for arg in argv:
			print(arg)
			if self._state is self.State.INIT:
				self._check_INIT(arg)
			elif self._state is self.State.FILE:
				self._check_FILE(arg)
	def add_converter(self, class_converter, conv_from, conv_to, **args):
		if conv_from in self.doc_types and conv_to in self.doc_types:
			if conv_from not in self.converters:
				self.converters[conv_from] = {}
			if conv_to not in self.converters:
				self.converters[conv_from][conv_to] = {}
			self.converters[conv_from][conv_to] = class_converter(**args)
	def add_file(self, filename, type):
		if type in self.doc_types:
			self.files[type] = filename
	def display_exceptions(self, exceptions):
		nb_errors = 0
		nb_warns = 0
		for e in exceptions:
			print(e)
			if e.type is "error":
				nb_errors += 1
			if e.type is "warning":
				nb_warns += 1
			if nb_errors >= self.max_errors or nb_warns >= self.max_warns:
				self.quit()
	def convert(self, **args):
		self.chain_convert.append(self.converters[self.DB][self.TEX])
		self.chain_convert.append(self.converters[self.TEX][self.DVI])
		self.chain_convert.append(self.converters[self.DVI][self.PS])
		self.chain_convert.append(self.converters[self.PS][self.PDF])
		for converter in self.chain_convert:
			if converter.check_older():
				converter.convert(**args)
				self.display_exceptions(converter.exceptions)
