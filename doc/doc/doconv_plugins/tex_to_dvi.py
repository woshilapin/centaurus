import posix
import os
import warnings
import subprocess
import shlex

from doconv_plugins import *
import doconv_plugins.bibtex as bibtex

class TEX_to_DVI_error(object):
	"""
	Class about the LaTeX error.
	"""
	class State(object):
		INIT = 0
		LATEX_ERROR = 1
		ERROR = 2
	def __init__(self):
		self.filename = ""
		self.line_number = 0
		self.char_number = 0
		self.msg = ""
		self._state = self.State.INIT
		self._latex_error_string = "LaTeX Error: "
		self._source_line_string = "l."
	def _is_latex_error(self, line):
		"""
		Check if the current line can be a latex error.
		A latex error line contains "LaTeX Error".
		"""
		if line.find(self._latex_error_string) == -1:
			return False
		return True
	def _is_error_line(self, line):
		"""
		Check if the current line can be a error line.
		A error line is structured as followed:
		<filename>:<line_number>: <error message>
		"""
		split_line = line.split(':')
		if len(split_line) < 3:
			return False
		if not split_line[1].isdigit():
			return False
		return True
	def _is_source_line(self, line):
		"""
		Check if the current line can be a source line.
		A source line is structured as followed:
		l.<num> <source part>
		"""
		if line.find(self._source_line_string) != 0:
			return False
		return True
	def _parse_latex_error(self, line):
		"""
		Parse an LaTeX error line (LaTeX Error: <msg>) and return the message.
		"""
		latex_error_index = line.find(self._latex_error_string)
		latex_error_width = len(self._latex_error_string)
		return line[latex_error_index+latex_error_width:]
	def _parse_error_line(self, line):
		"""
		Parse an error line (<filename>:<num>:<msg>) and return as a tuple.
		<msg> may contains "LaTeX Error: " so erase it.
		"""
		filename, line_number, *msg = line.split(':')
		filename = os.path.split(filename)[1]
		try:
			line_number = int(line_number)
		except ValueError:
			line_number = 0
		msg = ':'.join(msg)
		msg = msg.replace(self._latex_error_string, '')
		msg = self.msg + msg
		return (filename, msg, line_number)
	def _parse_source_line(self, line):
		"""
		Parse an source line (l.<num> <source part>) and return as a tuple.
		"""
		first_space_index = line.find(' ')
		line_number = line[:first_space_index].replace(self._source_line_string, '')
		try:
			line_number = int(line_number)
		except ValueError:
			line_number = 0
		char_number = len(line[first_space_index+1:])
		return (line_number, char_number)
	def _check_INIT(self, line):
		"""
		What to do when the state machine is in state INIT.
		"""
		self.filename = ""
		self.msg = ""
		self.line_number = 0
		self.char_number = 0
		if self._is_latex_error(line):
			self.msg = self._parse_latex_error(line)
			self._state = self.State.LATEX_ERROR
		elif self._is_error_line(line):
			self.filename, self.msg, self.line_number = self._parse_error_line(line)
			self._state = self.State.ERROR
	def _check_LATEX_ERROR(self, line):
		"""
		What to do when the state machine is in state LATEX_ERROR.
		"""
		if self._is_error_line(line):
			self.filename, self.msg, self.line_number = self._parse_error_line(line)
			self._state = self.State.ERROR
	def _check_ERROR(self, line):
		"""
		What to do when the state machine is in state ERROR.
		"""
		if self._is_source_line(line):
			self.line_number, self.char_number = self._parse_source_line(line)
			self._state = self.State.INIT
			raise Error(self.filename, self.msg, self.line_number, self.char_number)
	def parse(self, line):
		"""
		Parsing the LaTeX errors.
		"""
		if self._state is self.State.INIT:
			self._check_INIT(line)
		elif self._state is self.State.LATEX_ERROR:
			self._check_LATEX_ERROR(line)
		elif self._state is self.State.ERROR:
			self._check_ERROR(line)

class TEX_to_DVI_warning(object):
	"""
	Class about the LaTeX Warning.
	"""
	class State(object):
		INIT = 0
	def __init__(self, tex_filename):
		self.filename = tex_filename
		self.line_number = 0
		self.reference = False
		self.bibtex = False
		self.msg = ""
		self._state = self.State.INIT
		self._latex_warning_string = "LaTeX Warning: "
		self._input_line_string = " on input line "
		self._reference_string = "Reference "
		self._bibtex_string = "Citation "
	def _is_warning_line(self, line):
		"""
		Check if the current line can be a LaTeX Warning.
		A source line is structured as followed:
		LaTeX Warning: <msg>
		"""
		if not line.startswith(self._latex_warning_string):
			return False
		return True
	def _parse_warning_line(self, line):
		"""
		Parse an error line (LaTeX Warning: <msg>) and return as a tuple.
		<msg> may contains " on input line <num>" so parse the line number and erase all.
		If the LaTeX Warning <msg> starts with Reference, activate the reference tag.
		If the LaTeX Warning <msg> starts with Citation, activate the bibtex tag.
		"""
		msg = line.replace(self._latex_warning_string, '')
		line_number = 0
		input_line_index = msg.find(self._input_line_string)
		input_line_width = len(self._input_line_string)
		if input_line_index != -1:
			dot_index = msg[input_line_index+input_line_width:].find('.')+input_line_index+input_line_width
			line_number = int(msg[input_line_index+input_line_width:dot_index])
			msg = msg[:input_line_index]+'.'
		reference = False
		if msg.startswith(self._reference_string):
			reference = True
		bibtex = False
		if msg.startswith(self._bibtex_string):
			bibtex = True
			msg = "(bibtex) "+msg
		return (msg, line_number, reference, bibtex)
	def _check_INIT(self, line):
		"""
		What to do when the state machine is in state INIT.
		"""
		self.msg = ""
		self.line_number = 0
		if self._is_warning_line(line):
			self.msg, self.line_number, self.reference, self.bibtex = self._parse_warning_line(line)
			raise Warn(self.filename, self.msg, self.line_number, reference=self.reference, bibtex=self.bibtex)
			# Do not change the state in this case
	def parse(self, line):
		"""
		To detect LaTeX warnings.
		"""
		if self._state is self.State.INIT:
			self._check_INIT(line)

class TEX_to_DVI_overfull_hbox(object):
	"""
	Class about the overfull hbox warnings.
	"""
	class State(object):
		INIT = 0
	def __init__(self, tex_filename):
		self.filename = tex_filename
		self.line_number = 0
		self.msg = ""
		self._state = self.State.INIT
		self._overfull_hbox_string = "Overfull \\hbox"
		self._at_lines_string = " at lines "
	def _is_overfull_hbox(self, line):
		"""
		Check if the current line can be an overfull hbox warning.
		"""
		if not line.startswith(self._overfull_hbox_string):
			return False
		return True
	def _parse_overfull_hbox(self, line):
		"""
		Parse an overfull hbox warning (Overfull \hbox <msg>) and return it as a tuple.
		"""
		at_lines_index = line.find(self._at_lines_string)
		at_lines_width = len(self._at_lines_string)
		line_numbers = line[at_lines_index+at_lines_width:].split("--")
		line_number = int(line_numbers[0])
		msg = line[:at_lines_index]+'.'
		return (msg, line_number)
	def _check_INIT(self, line):
		"""
		What to do when the state machine is in state INIT.
		"""
		self.msg = ""
		self.line_number = 0
		if self._is_overfull_hbox(line):
			self.msg, self.line_number = self._parse_overfull_hbox(line)
			raise Warn(self.filename, self.msg, self.line_number)
			# Do not change the state in this case
	def parse(self, line):
		"""
		Parse 'Overfull \hbox' warnings lines of the LaTeX compilation.
		"""
		if self._state is self.State.INIT:
			self._check_INIT(line)

class TEX_to_DVI_fancy(object):
	"""
	Class about errors from fancy package.
	"""
	class State(object):
		INIT = 0
		FANCY = 1
	def __init__(self, tex_filename):
		self.filename = tex_filename
		self.msg = ""
		self._state = self.State.INIT
		self._fancy_string = "Package Fancyhdr Warning: "
	def _is_fancy(self, line):
		"""
		Check if the current line can be a fancy package error.
		A latex error line contains "Package Fancyhdr Warning: "
		"""
		if not line.startswith(self._fancy_string):
			return False
		return True
	def _parse_fancy(self, line):
		"""
		Parse a fancy error line (Package Fancyhdr Warning: <msg>) and return the message.
		<msg> contains "Package Fancyhdr Warning: " so erase it.
		"""
		return "(fancy package) "+line.replace(self._fancy_string, "")
	def _check_INIT(self, line):
		"""
		What to do when the state machine is in state INIT.
		"""
		self.msg = ""
		if self._is_fancy(line):
			self.msg = self._parse_fancy(line)
			self._state = self.State.FANCY
	def _check_FANCY(self, line):
		"""
		What to do when the state machine is in state FANCY.
		"""
		if line is "":
			self._state = self.State.INIT
			raise Warn(self.filename, self.msg)
		else:
			self.msg += line
			# Do not change the state in this case
	def parse(self, line):
		"""
		Parse the fancy package warnings.
		"""
		if self._state is self.State.INIT:
			self._check_INIT(line)
		elif self._state is self.State.FANCY:
			self._check_FANCY(line)

class TEX_to_DVI(Converter):
	"""
	To compile a LaTeX file to a DVI file.
	"""
	def __init__(self, tex_filename, dvi_filename, bib_filename=None):
		self.tex_filename = tex_filename
		self.dvi_filename = dvi_filename
		if not bib_filename:
			self.bib_filename = "biblio.bib"
		else:
			self._bibtex = True
		self.aux_filename = self.tex_filename.replace(".tex", ".aux")
		self.bbl_filename = self.aux_filename.replace(".aux", ".bbl")
		self.parsers = [TEX_to_DVI_error(), TEX_to_DVI_warning(self.tex_filename), TEX_to_DVI_overfull_hbox(self.tex_filename), TEX_to_DVI_fancy(self.tex_filename)]
		self.bibtex_compiler = bibtex.BibTeX(self.bib_filename, self.aux_filename)
		self.exceptions = []
		self._bibtex = False
		self._reference = False
		self._draft = False
	def parse(self):
		"""
		Parse the output of LaTeX compilation.
		"""
		stuck_line = ""
		for line in self.stdout.decode("latin1").split('\n'):
			stuck_line += line
			# LaTeX log are truncated at 80 characters
			# If the line is 80 characters long, continue to list lines to have the complete line in one peace.
			if len(line) >= 79:
				continue
			for parser in self.parsers:
				try:
					parser.parse(stuck_line)
				except (Error, Warn) as e:
					if e.bibtex:
						self._bibtex = True
					if e.reference:
						self._reference = True
					self.exceptions.append(e)
					continue
			stuck_line = ""
	def args(self, draft=False):
		"""
		Parse optional arguments given to the converter.
		"""
		if draft:
			self._draft = True
		pass
	def check_older(self):
		"""
		Check is the output file need to be update.
		"""
		if posix.access(self.dvi_filename, posix.F_OK) and posix.access(self.aux_filename, posix.F_OK):
			if file_time(self.dvi_filename) >= file_time(self.tex_filename) and file_time(self.aux_filename) >= file_time(self.bib_filename):
				return False
		return True
	def convert(self, **args):
		"""
		To convert TeX file to a DVI file.
		"""
		tex_filename = os.path.split(self.tex_filename)[1]
		dvi_filename = os.path.split(self.dvi_filename)[1]

		self.args(**args)
		cwd = os.path.split(os.path.abspath(self.tex_filename))[0]
		shell_cmd = shlex.split("latex -shell-escape -output-format=dvi -file-line-error -interaction nonstopmode -jobname {0} {1}".format(dvi_filename[:-4], tex_filename))
		proc = subprocess.Popen(args=shell_cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE, cwd=cwd)
		self.stdout, self.stderr = proc.communicate()
		self.parse()
		if self._bibtex and not self._draft:
			if self.bibtex_compiler.check_older():
				self.bibtex_compiler.convert()
				self.exceptions = self.bibtex_compiler.exceptions
				proc = subprocess.Popen(args=shell_cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE, cwd=cwd)
				self.stdout, self.stderr = proc.communicate()
			proc = subprocess.Popen(args=shell_cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE, cwd=cwd)
			self.stdout, self.stderr = proc.communicate()
			self.parse()
