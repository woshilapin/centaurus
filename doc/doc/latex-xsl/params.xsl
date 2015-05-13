<?xml version="1.0" encoding="utf-8"?>
<xsl:stylesheet xmlns:xsl="http://www.w3.org/1999/XSL/Transform" xmlns:xhtml="http://www.w3.org/1999/xhtml" xmlns:db="http://docbook.org/ns/docbook" version="1.0">
	<!-- 'draft' : Use the draft compilation for LaTeX (faster but do not create a final document -->
	<xsl:variable name="draft" select="'no'" />
	<!-- 'fancy' : Change the default headers and footers -->
	<xsl:variable name="fancy" select="'yes'" />
	<!-- 'pstricks' : With PSTricks activate -->
	<xsl:variable name="pstricks" select="'yes'" />
	<!-- 'hyperref' : To produce hyperlink in the final document -->
	<xsl:variable name="hyperref" select="'yes'" />
</xsl:stylesheet>
