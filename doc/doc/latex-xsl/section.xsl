<?xml version="1.0" encoding="utf-8"?>
<xsl:stylesheet xmlns:xsl="http://www.w3.org/1999/XSL/Transform" xmlns:xhtml="http://www.w3.org/1999/xhtml" xmlns:db="http://docbook.org/ns/docbook" version="1.0">
	<xsl:template match="db:section">
		<xsl:apply-templates />
	</xsl:template>
	<xsl:template match="db:section/db:info">
		<xsl:apply-templates />
	</xsl:template>
	<xsl:template match="db:section/db:info/db:title">
		<xsl:text>\section{</xsl:text>
		<xsl:apply-templates />
		<xsl:text>}&#xA;</xsl:text>
	</xsl:template>
	<xsl:template match="db:section/db:info/db:subtitle">
		<xsl:text>\textbf{</xsl:text>
		<xsl:apply-templates />
		<xsl:text>}&#xA;&#xA;</xsl:text>
	</xsl:template>
</xsl:stylesheet>
