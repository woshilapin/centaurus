import posix
import subprocess
import shlex

from doconv_plugins import *

class BIBTEX_not_found(object):
	"""
	Class about not found command error.
	This error can be obtain when the commands '\bibliography' or '\bibliographystyle' is missing in the LaTeX file.
	"""
	class State(object):
		INIT = 0
	def __init__(self, tex_filename):
		self.filename = tex_filename
		self.msg = ""
		self._state = self.State.INIT
		self._missing_string = "I found no "
		self._missing_bibdata_string = self._missing_string + "\\bibdata"
		self._missing_bibstyle_string = self._missing_string + "\\bibstyle"
	def _is_missing_data_command(self, line):
		"""
		Check if the current line can be a missing BibTeX command.
		A missing command error line contains "I found no \bibdata".
		"""
		if not line.startswith(self._missing_bibdata_string):
			return False
		return True
	def _is_missing_style_command(self, line):
		"""
		Check if the current line can be a missing BibTeX command.
		A missing command error line contains "I found no \bibstyle".
		"""
		if not line.startswith(self._missing_bibstyle_string):
			return False
		return True
	def _parse_missing_data_command(self, line):
		"""
		Parse an missing command error line (I found no \bibdata: <msg>) and return the message.
		"""
		msg = "No data file ('\\bibliography' command) was defined in '" + self.filename + "'"
		return msg
	def _parse_missing_style_command(self, line):
		"""
		Parse an missing command error line (I found no \bibstyle: <msg>) and return the message.
		"""
		msg = "No style ('\\bibliographystyle' command) was defined in '" + self.filename + "'"
		return msg
	def _check_INIT(self, line):
		"""
		What to do when the state machine is in state INIT.
		"""
		self.msg = ""
		if self._is_missing_data_command(line):
			self.msg = self._parse_missing_data_command(line)
			raise Error(self.filename, self.msg)
			# Do not change the state in this case
		elif self._is_missing_style_command(line):
			self.msg = self._parse_missing_style_command(line)
			raise Warn(self.filename, self.msg)
			# Do not change the state in this case
	def parse(self, line):
		"""
		Parse the "not found" command for BibTex compilation.
		"""
		if self._state is self.State.INIT:
			self._check_INIT(line)

class BIBTEX_warning(object):
	class State(object):
		INIT = 0
	def __init__(self, bib_filename):
		self.filename = bib_filename
		self.msg = ""
		self._state = self.State.INIT
		self._warning_string = "Warning--"
		self._in_string = " in "
	def _is_warning(self, line):
		"""
		Check if the current line can be is a BibTeX warning.
		A BibTeX warning is structured as followed:
		Warning--<msg>
		"""
		if not line.startswith(self._warning_string):
			return False
		return True
	def _parse_warning(self, line):
		"""
		Parse an BibTeX warning line (Warning--<msg>) and return the message.
		"""
		msg = line
		in_num_start_index = msg.rfind(self._in_string)
		in_num_end_index = len(msg)
		in_width = len(self._in_string)
		citation_name = msg[in_num_start_index + in_width:in_num_start_index + in_width + in_num_end_index]
		citation_string = "Citation `" + citation_name + "' - "
		msg = msg[:in_num_start_index]
		msg = msg.replace(self._warning_string, citation_string)
		return msg
	def _check_INIT(self, line):
		"""
		What to do when the state machine is in state INIT.
		"""
		self.msg = ""
		if self._is_warning(line):
			self.msg = self._parse_warning(line)
			raise Warn(self.filename, self.msg)
			# Do not change the state in this case
	def parse(self, line):
		"""
		Parse the warning of BibTeX compilation.
		"""
		if self._state is self.State.INIT:
			self._check_INIT(line)

class BibTeX(Converter):
	"""
	To compile bibliography with BibTeX.
	"""
	def __init__(self, bib_filename, aux_filename):
		self.bib_filename = bib_filename
		self.aux_filename = aux_filename
		self.tex_filename = self.aux_filename.replace(".aux", ".tex")
		self.bbl_filename = self.aux_filename.replace(".aux", ".bbl")
		self.parsers = [BIBTEX_not_found(self.tex_filename), BIBTEX_warning(self.bib_filename)]
		self.exceptions = []
	def convert(self):
		"""
		To achieve the BibTeX compilation.
		"""
		cwd = os.path.split(os.path.abspath(self.aux_filename))[0]
		shell_cmd = shlex.split("bibtex {0}".format(self.aux_filename))
		proc = subprocess.Popen(args=shell_cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE, cwd=cwd)
		self.stdout, self.stderr = proc.communicate()
		self.parse()
	def check_older(self):
		if posix.access(self.bbl_filename, posix.F_OK) and posix.access(self.aux_filename, posix.F_OK):
			if file_time(self.dvi_filename) >= file_time(self.tex_filename) and file_time(self.bbl_filename) >= file_time(self.aux_filename) and file_time(self.aux_filename) >= file_time(self.bib_filename):
				return False
		return True
	def parse(self):
		"""
		Parse the output of BibTeX compilation.
		"""
		for line in self.stdout.decode("latin1").split('\n'):
			for parser in self.parsers:
				try:
					parser.parse(line)
				except (Error, Warn) as e:
					self.exceptions.append(e)
					continue
