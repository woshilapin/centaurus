import posix
import os
import warnings
import subprocess
import shlex

from doconv_plugins import *

class DB_to_TEX_compilation_error(object):
	"""
	Class about the xsltproc compilation error.
	A compilation error can be obtain with the import of an unexisting file.
	"""
	class State(object):
		INIT = 0
		ERROR = 1
	def __init__(self, db_filename):
		self.filename = db_filename
		self.msg = ""
		self._state = self.State.INIT
		self._compilation_error_string = "compilation error: "
	def _is_compilation_error(self, line):
		"""
		Check if the current line can be a compilation error.
		A compilation error starts with "compilation error: ".
		"""
		if not line.startswith(self._compilation_error_string):
			return False
		return True
	def _parse_compilation_error(self, line):
		"""
		Parse an compilation error line (compilation error: <msg>) and return the message.
		"""
		return line.replace(self._compilation_error_string, '') + '.'
	def _parse_second_error_line(self, line):
		"""
		Parse an second error line (<msg>) and return the message.
		There is a concatenation of the old message and the new message.
		"""
		return line + '.'
	def _check_INIT(self, line):
		"""
		What to do when the state machine is in state INIT.
		"""
		self.msg = ""
		if self._is_compilation_error(line):
			self.msg = self._parse_compilation_error(line)
			self._state = self.State.ERROR
	def _check_ERROR(self, line):
		"""
		What to do when the state machine is in state ERROR.
		The next line when an error is founded must be read to complete the message.
		"""
		self.msg = self.msg + ' ' + self._parse_second_error_line(line)
		self._state = self.State.INIT
		raise Error(self.filename, self.msg)
	def parse(self, line):
		"""
		Parse compilation error happening most of the time when the arguments to 'xsltproc' are not allowed.
		"""
		if self._state is self.State.INIT:
			self._check_INIT(line)
		if self._state is self.State.ERROR:
			self._check_ERROR(line)

class DB_to_TEX_light_warning(object):
	"""
	Class about light warning.
	A light warning can be obtain with the import of an unexisting file.
	"""
	class State(object):
		INIT = 0
	def __init__(self, db_filename):
		self.filename = db_filename
		self.msg = ""
		self._state = self.State.INIT
		self._light_warning_string = "warning: "
	def _is_light_warning(self, line):
		"""
		Check if the current line can be a light warning.
		A light warning starts with "warning: ".
		"""
		if not line.startswith(self._light_warning_string):
			return False
		return True
	def _parse_light_warning(self, line):
		"""
		Parse an light warning line (warning: <msg>) and return the message.
		"""
		msg = line.replace(self._light_warning_string, '') + '.'
		return msg
	def _check_INIT(self, line):
		"""
		What to do when the state machine is in state INIT.
		"""
		self.msg = ""
		if self._is_light_warning(line):
			self.msg = self._parse_light_warning(line)
			raise Error(self.filename, self.msg)
			# Do not change the state in this case
	def parse(self, line):
		"""
		Parse compilation warning happening most of the time when the arguments to 'xsltproc' are not allowed.
		"""
		if self._state is self.State.INIT:
			self._check_INIT(line)

class DB_to_TEX_parser_error(object):
	"""
	Class about the parsing error of xsltproc.
	Each error about the parsing.
	"""
	class State(object):
		INIT = 0
		SOURCE = 1
		ERROR = 2
	def __init__(self):
		self.filename = ""
		self.msg = ""
		self.line_number = 0
		self.char_number = 0
		self._state = self.State.INIT
	def _is_parser_error(self, line):
		"""
		Check if the current line can be a parser error.
		A parser error is "<filename>:<num>:<msg>".
		"""
		if len(line.split(':')) < 3:
			return False
		return True
	def _is_source_line(self, line):
		"""
		Check if the current line is a source line.
		There is no marker to detect a source line so always return True.
		"""
		return True
	def _is_marker_line(self, line):
		"""
		Check if the current line is a marker line.
		The marker line contains only the spaces and '^'.
		"""
		if not line.find('^'):
			return False
		if len(line.replace(' ', '').replace("\t", '')) > 1:
			return False
		return True
	def _parse_parser_error(self, line):
		"""
		Parse a parser error line (<filename>:<num>:<msg>) and return a tuple.
		"""
		filename, line_number, err_type, *msg = line.split(':')
		line_number = int(line_number)
		msg = "".join(msg)
		return (filename, msg, line_number)
	def _parse_source_line(self, line):
		"""
		There is nothong to parse in a source line.
		"""
		return None
	def _parse_marker_line(self, line):
		"""
		Parse a marker line (   ^) and return the number of spaces.
		A marker line is only a well-placed '^' so this function count the number of spaces before the '^'.
		"""
		char_number = line.find('^') + 1
		return char_number
	def _check_INIT(self, line):
		"""
		What to do when the state machine is in state INIT.
		"""
		self.filename = ""
		self.msg = ""
		self.line_number = 0
		if self._is_parser_error(line):
			self.filename, self.msg, self.line_number = self._parse_parser_error(line)
			self._state = self.State.SOURCE
	def _check_SOURCE(self, line):
		"""
		What to do when the state machine is in state SOURCE.
		"""
		if self._is_source_line(line):
			self._parse_source_line(line)
			self._state = self.State.ERROR
	def _check_ERROR(self, line):
		"""
		What to do when the state machine is in state ERROR.
		"""
		if self._is_marker_line(line):
			self.char_number = self._parse_marker_line(line)
			self._state = self.State.INIT
			raise Error(self.filename, self.msg, self.line_number, self.char_number)
	def parse(self, line):
		"""
		Parse the xsltproc warnings and consider them as errors.
		"""
		if self._state is self.State.INIT:
			self._check_INIT(line)
		elif self._state is self.State.SOURCE:
			self._check_SOURCE(line)
		elif self._state is self.State.ERROR:
			self._check_ERROR(line)

class DB_to_TEX(Converter):
	def __init__(self, db_filename, tex_filename, xsl_filename):
		self.db_filename = db_filename
		self.tex_filename = tex_filename
		self.xsl_filename = xsl_filename
		self.parsers = [ DB_to_TEX_compilation_error(self.db_filename),DB_to_TEX_light_warning(self.db_filename),DB_to_TEX_parser_error()]
		self.exceptions = []

	def parse(self):
		"""
		Parse the output of LaTeX compilation.
		"""
		for line in self.stderr.decode("latin1").split('\n'):
			print(line)
			for parser in self.parsers:
				try:
					parser.parse(line)
				except (Error, Warn) as e:
					self.exceptions.append(e)
	def args(self, draft=False):
		"""
		Parse optional arguments given to the converter.
		"""
		pass
	def check_older(self):
		"""
		Check is the output file need to be update.
		"""
		if posix.access(self.tex_filename, posix.F_OK):
			if file_time(self.tex_filename) >= file_time(self.db_filename) and file_time(self.tex_filename) >= file_time(self.xsl_filename):
					return False
		return True
	def convert(self, **args):
		"""
		To convert DocBook file to a TeX file.
		"""
		self.args(**args)
		db_filename = os.path.relpath(self.db_filename)
		tex_filename = os.path.relpath(self.tex_filename)
		xsl_filename = os.path.relpath(self.xsl_filename)
		cwd = os.path.split(os.path.abspath(self.db_filename))[0]
		shell_cmd = shlex.split("xsltproc --output {0} {1} {2}".format(tex_filename, xsl_filename, db_filename))
		proc = subprocess.Popen(args=shell_cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE, cwd=cwd)
		self.stdout, self.stderr = proc.communicate()
		self.parse()
