<?xml version="1.0" encoding="utf-8"?>
<xsl:stylesheet xmlns:xsl="http://www.w3.org/1999/XSL/Transform" xmlns:xhtml="http://www.w3.org/1999/xhtml" xmlns:db="http://docbook.org/ns/docbook" version="1.0">
	<xsl:import href="part.xsl" />
	<xsl:import href="chapter.xsl" />
	<xsl:import href="section.xsl" />
	<xsl:import href="fancy.xsl" />
	<xsl:import href="pstricks.xsl" />
	<xsl:import href="hyperref.xsl" />
	<xsl:template match="/db:book">
		<xsl:text>\documentclass[a4paper</xsl:text>
		<xsl:if test="$draft = 'yes'">
			<xsl:text>,draft</xsl:text>
		</xsl:if>
		<xsl:text>]{book}&#xA;</xsl:text>
		<xsl:text>\usepackage[postscript]{ucs}&#xA;</xsl:text>
		<xsl:text>\usepackage[utf8x]{inputenc}&#xA;</xsl:text>
		<xsl:text>\usepackage{pifont}&#xA;</xsl:text>
		<xsl:text>\usepackage[T1]{fontenc}&#xA;</xsl:text>
		<xsl:text>\usepackage[frenchb]{babel}&#xA;</xsl:text>
		<xsl:text>\usepackage{lmodern}&#xA;</xsl:text>
		<xsl:if test="$fancy = 'yes'">
			<xsl:call-template name="fancy" />
		</xsl:if>
		<xsl:text>\usepackage[francais]{varioref}&#xA;</xsl:text>
		<xsl:if test="$pstricks = 'yes'">
			<xsl:call-template name="pstricks" />
		</xsl:if>
		<xsl:if test="$hyperref = 'yes'">
			<xsl:call-template name="hyperref" />
		</xsl:if>
		<xsl:text>\makeatletter&#xA;</xsl:text>
		<xsl:text>\setlength{\parindent}{0cm}&#xA;</xsl:text>
		<xsl:text>\setlength{\parskip}{1ex plus 0.5ex minus 0.2ex}&#xA;</xsl:text>
		<xsl:text>\clubpenalty=1000&#xA;</xsl:text>
		<xsl:text>\widowpenalty=1000&#xA;</xsl:text>
		<xsl:text>\newcommand{\subtitle}[1]{\def\@subtitle{#1}}&#xA;</xsl:text>
		<xsl:text>\makeatother&#xA;</xsl:text>
		<xsl:apply-templates select="db:info" />
		<xsl:text>\begin{document}&#xA;</xsl:text>
		<xsl:text>\maketitle&#xA;</xsl:text>
		<xsl:text>\tableofcontents&#xA;</xsl:text>
		<xsl:apply-templates select="db:info/db:abstract" />
		<xsl:apply-templates />
		<xsl:text>\end{document}</xsl:text>
	</xsl:template>
	<xsl:template match="db:book/db:info">
		<xsl:apply-templates select="db:title | db:subtitle | db:author" />
	</xsl:template>
	<xsl:template match="db:book/db:info/db:title">
		<xsl:text>\title{</xsl:text>
		<xsl:apply-templates />
		<xsl:text>}&#xA;</xsl:text>
	</xsl:template>
	<xsl:template match="db:book/db:info/db:subtitle">
		<xsl:text>\subtitle{</xsl:text>
		<xsl:apply-templates />
		<xsl:text>}&#xA;</xsl:text>
	</xsl:template>
	<xsl:template match="db:book/db:info/db:author">
		<xsl:text>\author{</xsl:text>
		<xsl:for-each select="db:personname">
			<xsl:apply-templates select="." />
			<xsl:if test="position() != last()">
				<xsl:text>~\and~</xsl:text>
			</xsl:if>
		</xsl:for-each>
		<xsl:text>}&#xA;</xsl:text>
	</xsl:template>
	<xsl:template match="db:book/db:info/db:abstract">
		<xsl:text>\chapter*{Résumé}&#xA;</xsl:text>
		<xsl:apply-templates />
	</xsl:template>
	<xsl:template match="db:abstract/db:info/db:title">
		<xsl:text>\textbf{</xsl:text>
		<xsl:apply-templates />
		<xsl:text>}&#xA;&#xA;</xsl:text>
	</xsl:template>
	<xsl:template match="db:abstract/db:info/db:subtitle">
		<xsl:apply-templates />
	</xsl:template>
	<xsl:template match="db:personname">
		<xsl:if test="db:surname">
			<xsl:if test="db:firstname">
				<xsl:value-of select="normalize-space(db:firstname)" /><xsl:text>~</xsl:text>
			</xsl:if>
			<xsl:text>\bsc{</xsl:text>
			<xsl:value-of select="normalize-space(db:surname)" />
			<xsl:text>}</xsl:text>
		</xsl:if>
	</xsl:template>
	<xsl:template match="db:para">
		<xsl:apply-templates />
		<xsl:text>&#xA;&#xA;</xsl:text>
	</xsl:template>
	<xsl:template match="db:xref">
		<xsl:text>\vref{</xsl:text>
		<xsl:value-of select="@linkend" />
		<xsl:text>}</xsl:text>
	</xsl:template>
	<xsl:template match="text()">
		<xsl:value-of select="normalize-space(.)" />
	</xsl:template>
</xsl:stylesheet>
