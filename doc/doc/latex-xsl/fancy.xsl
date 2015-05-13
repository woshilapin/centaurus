<?xml version="1.0" encoding="utf-8"?>
<xsl:stylesheet xmlns:xsl="http://www.w3.org/1999/XSL/Transform" xmlns:xhtml="http://www.w3.org/1999/xhtml" xmlns:db="http://docbook.org/ns/docbook" version="1.0">
	<xsl:template name="fancy">
		<xsl:text>\usepackage{fancyhdr}&#xA;</xsl:text>
		<xsl:call-template name="fancy-setup" />
	</xsl:template>
	<xsl:template name="fancy-setup">
		<xsl:text>\makeatletter&#xA;</xsl:text>
		<xsl:text>\pagestyle{fancy}&#xA;</xsl:text>
		<xsl:text>\renewcommand{\chaptermark}[1]{\def\@Null{0}\markboth{\ifx\@Null\thechapter{#1}\else{\chaptername~\thechapter{} -- #1}\fi}{}}&#xA;</xsl:text>
		<xsl:text>\renewcommand{\sectionmark}[1]{\markright{\thesection{} -- #1}}&#xA;</xsl:text>
		<xsl:text>\fancyhf{}&#xA;</xsl:text>
		<xsl:text>\fancyhead[LE,RO]{\bfseries\thepage}&#xA;</xsl:text>
		<xsl:text>\fancyhead[LO]{\bfseries\rightmark}&#xA;</xsl:text>
		<xsl:text>\fancyhead[RE]{\bfseries\leftmark}&#xA;</xsl:text>
	</xsl:template>
</xsl:stylesheet>
