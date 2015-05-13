<?xml version="1.0" encoding="utf-8"?>
<xsl:stylesheet xmlns:xsl="http://www.w3.org/1999/XSL/Transform" xmlns:xhtml="http://www.w3.org/1999/xhtml" xmlns:db="http://docbook.org/ns/docbook" version="1.0">
	<xsl:template match="db:part">
		<xsl:apply-templates />
	</xsl:template>
	<xsl:template match="db:part/db:info">
		<xsl:apply-templates />
	</xsl:template>
	<xsl:template match="db:part/db:info/db:title">
		<xsl:text>\part{</xsl:text>
		<xsl:apply-templates />
		<xsl:text>}</xsl:text>
		<xsl:text>\label{</xsl:text>
		<xsl:value-of select="normalize-space(parent::db:info/parent::db:part/@xml:id)" />
		<xsl:text>}&#xA;</xsl:text>
	</xsl:template>
	<xsl:template match="db:part/db:info/db:subtitle">
	</xsl:template>
	<xsl:template match="db:partintro">
		<xsl:text>\section*{</xsl:text>
		<xsl:value-of select="normalize-space(parent::db:part/db:info/db:subtitle)" />
		<xsl:text>}&#xA;</xsl:text>
		<xsl:apply-templates />
	</xsl:template>
</xsl:stylesheet>
