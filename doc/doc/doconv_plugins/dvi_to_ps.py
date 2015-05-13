import posix
import os
import warnings
import subprocess
import shlex

from doconv_plugins import *

class DVI_to_PS_error(object):
	"""
	Class about a dvips error.
	It can be obtain if the DVI file does not exist for example.
	"""
	class State(object):
		INIT = 0
	def __init__(self, dvi_filename):
		self.filename = dvi_filename
		self.msg = ""
		self._state = self.State.INIT
		self._error_string = "dvips: "
	def _is_error(self, line):
		"""
		Check if the current line can be an error.
		An error starts with "dvips: ".
		"""
		if not line.startswith(self._error_string):
			return False
		return True
	def _parse_error(self, line):
		"""
		Parse the error line (dvips: <msg>) and return a message.
		"""
		return line.replace(self._error_string, '')
	def _check_INIT(self, line):
		"""
		What to do when the state machine is in state INIT.
		"""
		self.msg = ""
		if self._is_error(line):
			self.msg = self._parse_error(line)
			raise Error(self.filename, self.msg)
			# Do not change the state in this case
	def parse(self, line):
		"""
		Parse the xsltproc warnings and consider them as errors.
		"""
		if self._state is self.State.INIT:
			self._check_INIT(line)

class DVI_to_PS(Converter):
	def __init__(self, dvi_filename, ps_filename):
		self.dvi_filename = dvi_filename
		self.ps_filename = ps_filename
		self.parsers = [DVI_to_PS_error(self.dvi_filename)]
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
		if posix.access(self.ps_filename, posix.F_OK):
			if file_time(self.ps_filename) >= file_time(self.dvi_filename):
				return False
		return True
	def convert(self, **args):
		"""
		To convert DocBook file to a TeX file.
		"""
		self.args(**args)
		dvi_filename = os.path.relpath(self.dvi_filename)
		ps_filename = os.path.relpath(self.ps_filename)
		cwd = os.path.split(os.path.abspath(self.dvi_filename))[0]
		shell_cmd = shlex.split("dvips -o {0} {1}".format(ps_filename, dvi_filename))
		proc = subprocess.Popen(args=shell_cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE, cwd=cwd)
		self.stdout, self.stderr = proc.communicate()
		self.parse()
