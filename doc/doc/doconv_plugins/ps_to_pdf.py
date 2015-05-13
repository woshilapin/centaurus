import posix
import os
import warnings
import subprocess
import shlex

from doconv_plugins import *

class PS_to_PDF_error(object):
	"""
	Class about ps2pdf errors.
	Errors can be obtain be suppressing a number before a 'div' in the PostScript file for example.
	"""
	class State(object):
		INIT = 0
		ERROR = 1
	def __init__(self, ps_filename):
		self.filename = ps_filename
		self.msg = ""
		self._state = self.State.INIT
		self._error_string = "Error: "
		self._file_position_string = "Current file position is "
	def _is_error(self, line):
		"""
		Check if the current line can be an error line.
		An error line is structured as followed:
		Error: <msg>
		"""
		if not line.startswith(self._error_string):
			return False
		return True
	def _is_file_position(self, line):
		"""
		Check if the current line can be the line where there is the file position.
		A such line is structured as followed:
		Current file position is <num>
		"""
		if not line.startswith(self._file_position_string):
			return False
		return True
	def _parse_error(self, line):
		"""
		Parse the error line (Error: <msg>) and return the message.
		"""
		return line.replace(self._error_string, '')
	def _parse_file_position(self, line):
		"""
		Parse the line with file position (Current file position is <num>) and return nothing.
		We could find the line_number and the char_number with character position but perhaps it will be in next modifications of this code.
		"""
		char_pos = int(line.replace(self._file_position_string, ''))
		return None
	def _check_INIT(self, line):
		"""
		What to do when the state machine is in state INIT.
		"""
		if self._is_error(line):
			self.msg = self._parse_error(line)
			self._state = self.State.ERROR
	def _check_ERROR(self, line):
		"""
		What to do when the state machine is in state ERROR.
		"""
		if self._is_file_position(line):
			self._parse_file_position(line)
			self._state = self.State.INIT
			raise Error(self.filename, self.msg)
	def parse(self, line):
		"""
		Parse the xsltproc warnings and consider them as errors.
		"""
		if self._state is self.State.INIT:
			self._check_INIT(line)
		elif self._state is self.State.ERROR:
			self._check_ERROR(line)

class PS_to_PDF(Converter):
	def __init__(self, ps_filename, pdf_filename):
		self.ps_filename = ps_filename
		self.pdf_filename = pdf_filename
		self.parsers = [PS_to_PDF_error(self.ps_filename)]
		self.exceptions = []

	def parse(self):
		"""
		Parse the output of LaTeX compilation.
		"""
		for line in self.stderr.decode("latin1").split('\n'):
			for parser in self.parsers:
				try:
					parser.parse(line)
				except (Error, Warn) as e:
					self.exceptions.append(e)

	def args(self):
		"""
		Parse optional arguments given to the converter.
		"""
		pass
	def check_older(self):
		"""
		Check is the output file need to be update.
		"""
		if posix.access(self.pdf_filename, posix.F_OK):
			if file_time(self.pdf_filename) >= file_time(self.ps_filename):
				return False
		return True
	def convert(self, **args):
		"""
		To convert DocBook file to a TeX file.
		"""
		self.args(**args)
		ps_filename = os.path.relpath(self.ps_filename)
		pdf_filename = os.path.relpath(self.pdf_filename)
		cwd = os.path.split(os.path.abspath(self.ps_filename))[0]
		shell_cmd = shlex.split("ps2pdf {0} {1}".format(ps_filename, pdf_filename))
		proc = subprocess.Popen(args=shell_cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE, cwd=cwd)
		self.stdout, self.stderr = proc.communicate()
		self.parse()
