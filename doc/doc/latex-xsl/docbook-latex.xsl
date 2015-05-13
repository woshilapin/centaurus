<?xml version="1.0" encoding="utf-8"?>
<xsl:stylesheet xmlns:xsl="http://www.w3.org/1999/XSL/Transform" xmlns:xhtml="http://www.w3.org/1999/xhtml" xmlns:db="http://docbook.org/ns/docbook" version="1.0">
	<xsl:import href="params.xsl" />
	<xsl:import href="book.xsl" />
	<xsl:output method="text" encoding="utf-8" omit-xml-declaration="no" 
		indent="no" media-type="text" />
	<xsl:strip-space elements="db:*" />
	<xsl:template match="/">
		<xsl:apply-templates select="/db:book" />
	</xsl:template>
</xsl:stylesheet>
