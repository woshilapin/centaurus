<?xml version="1.0" encoding="utf-8"?>
<xsl:stylesheet xmlns:xsl="http://www.w3.org/1999/XSL/Transform" xmlns:xhtml="http://www.w3.org/1999/xhtml" xmlns:db="http://docbook.org/ns/docbook" version="1.0">
	<xsl:template name="hyperref">
		<xsl:text>\usepackage[dvips]{hyperref}&#xA;</xsl:text>
		<xsl:call-template name="hyperref-setup" />
	</xsl:template>
	<xsl:template name="hyperref-setup">
		<xsl:text>\hypersetup{%&#xA;</xsl:text>
		<xsl:text>pdfnewwindow=true,%&#xA;</xsl:text>
		<xsl:text>colorlinks=true,%&#xA;</xsl:text>
		<xsl:text>breaklinks=true,%&#xA;</xsl:text>
		<xsl:text>pdfhighlight=/P,%&#xA;</xsl:text>
		<xsl:text>pdfcenterwindow=true,%&#xA;</xsl:text>
		<xsl:text>pdfdisplaydoctitle=true,%&#xA;</xsl:text>
		<xsl:text>pdffitwindow=true,%&#xA;</xsl:text>
		<xsl:text>pdfstartview=FitH,%&#xA;</xsl:text>
		<xsl:text>linkcolor=blue,%&#xA;</xsl:text>
		<xsl:text>citecolor=green,%&#xA;</xsl:text>
		<xsl:text>filecolor=magenta,%&#xA;</xsl:text>
		<xsl:text>urlcolor=red%&#xA;</xsl:text>
		<xsl:text>}%&#xA;</xsl:text>
	</xsl:template>
</xsl:stylesheet>
