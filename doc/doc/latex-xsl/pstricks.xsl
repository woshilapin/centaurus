<?xml version="1.0" encoding="utf-8"?>
<xsl:stylesheet xmlns:xsl="http://www.w3.org/1999/XSL/Transform" xmlns:xhtml="http://www.w3.org/1999/xhtml" xmlns:db="http://docbook.org/ns/docbook" version="1.0">
	<xsl:template name="pstricks">
		<xsl:call-template name="pstricks-packages" />
		<xsl:call-template name="pstricks-newpartstyle" />
		<xsl:call-template name="pstricks-newchapterstyle" />
	</xsl:template>
	<xsl:template name="pstricks-packages">
		<xsl:text>\usepackage{pst-bar}&#xA;</xsl:text>
		<xsl:text>\usepackage{pst-eucl}&#xA;</xsl:text>
		<xsl:text>\usepackage{pst-3dplot}&#xA;</xsl:text>
		<xsl:text>\usepackage{pst-all}&#xA;</xsl:text>
		<xsl:text>\usepackage{pstricks-add}&#xA;</xsl:text>
		<xsl:text>\usepackage{pst-blur}&#xA;</xsl:text>
	</xsl:template>
	<xsl:template name="pstricks-newpartstyle">
		<!-- Part style redefinition -->
		<xsl:text>\makeatletter&#xA;</xsl:text>
		<xsl:text>\def\@part[#1]#2{%&#xA;</xsl:text>
		<xsl:text>\ifnum \c@secnumdepth >-2\relax&#xA;</xsl:text>
		<xsl:text>\refstepcounter{part}%&#xA;</xsl:text>
		<xsl:text>\addcontentsline{toc}{part}{\thepart\hspace{1em}#1}%&#xA;</xsl:text>
		<xsl:text>\else&#xA;</xsl:text>
		<xsl:text>\addcontentsline{toc}{part}{#1}%&#xA;</xsl:text>
		<xsl:text>\fi&#xA;</xsl:text>
		<xsl:text>\markboth{}{}%&#xA;</xsl:text>
		<xsl:text>{\centering&#xA;</xsl:text>
		<xsl:text>\interlinepenalty \@M&#xA;</xsl:text>
		<xsl:text>\normalfont&#xA;</xsl:text>
		<xsl:text>\newrgbcolor{Red}{0.734 0 0}&#xA;</xsl:text>
		<xsl:text>\psset{unit=1cm}&#xA;</xsl:text>
		<xsl:text>\begin{pspicture}(-6,-5)(6,5)&#xA;</xsl:text>
		<xsl:text>\rput(0,0){&#xA;</xsl:text>
		<xsl:text>\psframebox[linewidth=0,framearc=.25,linecolor=Red,fillstyle=solid,fillcolor=Red]{&#xA;</xsl:text>
		<xsl:text>\ifnum \c@secnumdepth >-2\relax&#xA;</xsl:text>
		<xsl:text>\begin{tabular}{@{}c@{}}&#xA;</xsl:text>
		<xsl:text>\psframebox[linewidth=0,framearc=.25,linecolor=Red,fillstyle=solid,fillcolor=Red]{\huge\bfseries\sffamily\textcolor{white}{\partname\nobreakspace\thepart\newline}\par}\fi\\&#xA;</xsl:text>
		<xsl:text>\psframebox[linewidth=0,framearc=.25,linecolor=Red,fillstyle=solid,fillcolor=white]{\Huge\bfseries\sffamily\parbox{\textwidth}{\begin{center}#2\end{center}}\par}&#xA;</xsl:text>
		<xsl:text>\end{tabular}&#xA;</xsl:text>
		<xsl:text>}&#xA;</xsl:text>
		<xsl:text>}&#xA;</xsl:text>
		<xsl:text>\end{pspicture}&#xA;</xsl:text>
		<xsl:text>\@endpart}&#xA;</xsl:text>
		<xsl:text>}&#xA;</xsl:text>
		<xsl:text>\makeatother&#xA;</xsl:text>
	</xsl:template>
	<xsl:template name="pstricks-newchapterstyle">
		<!-- Chapter style redefinition -->
		<xsl:text>\makeatletter&#xA;</xsl:text>
		<xsl:text>\def\@makechapterhead#1{%&#xA;</xsl:text>
		<xsl:text>\addtocontents{tcs}{\protect\addvspace{10\p@}}&#xA;</xsl:text>
		<xsl:text>\addtocontents{tdo}{\protect\addvspace{10\p@}}&#xA;</xsl:text>
		<xsl:text>\vspace*{50\p@}%&#xA;</xsl:text>
		<xsl:text>{\parindent \z@ \raggedleft \normalfont&#xA;</xsl:text>
		<xsl:text>\ifnum \c@secnumdepth >\m@ne&#xA;</xsl:text>
		<xsl:text>\newrgbcolor{Red}{0.734 0 0}&#xA;</xsl:text>
		<xsl:text>\psset{unit=1cm}&#xA;</xsl:text>
		<xsl:text>\begin{pspicture}(-6,-1)(1,1.5)&#xA;</xsl:text>
		<xsl:text>\psframe[linewidth=0,linecolor=Red,fillstyle=solid,fillcolor=Red](-5,-1)(0,0)&#xA;</xsl:text>
		<xsl:text>\psframe[linewidth=0,framearc=.25,linecolor=Red,fillstyle=solid,fillcolor=Red](-1,-1)(1,1)&#xA;</xsl:text>
		<xsl:text>\psframe[linewidth=0,framearc=.25,linecolor=white,fillstyle=solid,fillcolor=white](-6,-0.95)(-1,1)&#xA;</xsl:text>
		<xsl:text>\uput[45](-5,-1){\large\bfseries\sffamily\textcolor{Red}{\@chapapp}}&#xA;</xsl:text>
		<xsl:text>\rput(0,0){\psscaleboxto(1.5,1.5){\white\selectfont\bfseries\sffamily\thechapter}}&#xA;</xsl:text>
		<xsl:text>\end{pspicture}&#xA;</xsl:text>
		<xsl:text>\par\nobreak&#xA;</xsl:text>
		<xsl:text>\fi&#xA;</xsl:text>
		<xsl:text>\interlinepenalty\@M&#xA;</xsl:text>
		<xsl:text>\Huge\bfseries\sffamily #1\par\nobreak&#xA;</xsl:text>
		<xsl:text>\vskip 40\p@&#xA;</xsl:text>
		<xsl:text>}%&#xA;</xsl:text>
		<xsl:text>}&#xA;</xsl:text>
		<xsl:text>\makeatother&#xA;</xsl:text>
	</xsl:template>
</xsl:stylesheet>
