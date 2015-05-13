#!/bin/bash
#CENTAURUS  ________
#CENTAURUS /        \
#CENTAURUS|          |
#CENTAURUS|     ____/               |\
#CENTAURUS|    /                   _| |_
#CENTAURUS|    |                  |_   _|
#CENTAURUS|    |       _____  _  __ | |     __  _ _    _ _  ___ _    _  ____
#CENTAURUS|    |      /  _  \| |/  \| |    /  \| | |  | | |/___| |  | |/  __|
#CENTAURUS|    \____ |  (_)  |  __  | |   |  _   | |  | |  /   | |  | |  |_
#CENTAURUS|         \| _____/| /  \ | |  _| (_)  | |  | | |    | |  | |\_  \
#CENTAURUS|          | \____ | |  | | \_/ |      |  \/  | |    |  \/  |__|  |
#CENTAURUS \________/ \_____||_|  |_|\___/ \__/|_|\__/|_|_|     \__/|_|____/
#CENTAURUS

EFFACER_SORTIE="> TEMP >>2 TEMP"
EFFACER_SORTIE=

grep '^#CENTAURUS.*$' ScriptLogo.sh | sed 's/^#CENTAURUS\(.*\)$/\1/g'
echo -e "-------------------------------------------------------------------\n---------------- Creation d'un logo pour Centaurus ----------------\n-------------------------------------------------------------------"
echo -e "\n1 - Logo simple\n2 - Logo Complet\nChoix : \c "
read TYPE_LOGO;
echo -e "\n1 - Sans le nom\n2 - Avec le nom\nChoix : \c "
read NOM_LOGO;
LOGO_NB=0
if [[ $1 = "" ]]
then
	OUT_FILE="Logo.pdf"
else
	OUT_FILE=$1
fi
if [[ $TYPE_LOGO = "1" ]];
then
	echo -e "\nCouleur primaire du logo\n1 - Rouge\n2 - Vert\n3 - Bleu\n0 - Noir et blanc\nChoix : \c "
	read COULEUR_LOGO
	if [[ $COULEUR_LOGO = 0 ]] ;
	then
		LOGO_NB=1;
	fi ;
else
	echo -e "\nCouleur primaire du logo\n1 - Rouge\n2 - Vert\n3 - Bleu\n0 - Noir et blanc\nChoix : \c "
	read COULEUR_LOGO3;
	if [[ $COULEUR_LOGO1 != 0 ]] ;
	then
		echo -e "\nCouleur secondaire du logo\n1 - Rouge\n2 - Vert\n3 - Bleu\nChoix : \c "
		read COULEUR_LOGO2
		echo -e "\nCouleur tertiaire du logo\n1 - Rouge\n2 - Vert\n3 - Bleu\nChoix : \c "
		read COULEUR_LOGO1;
	else
		LOGO_NB=1
		COULEUR_LOGO2=0
		COULEUR_LOGO3=0 ;
	fi ;
fi
if [[ ( $NOM_LOGO = "2" ) ]];
then
	echo -e "\nCouleur du nom du logo\n1 - Rouge\n2 - Vert\n3 - Bleu\n0 - Noir et blanc\nChoix : \c "
	read CouleurNomLogo;
fi

mkdir ConstructionLogo
cd ConstructionLogo

CreationFichier()
{
	REMPLACEMENT_LARGEUR='s/<+LARGEUR+>/'$LARGEUR'/g'
	REMPLACEMENT_HAUTEUR='s/<+HAUTEUR+>/'$HAUTEUR'/g'
	REMPLACEMENT_NOUVELLES_COMMANDES='s/<+NOUVELLES_COMMANDES+>/'$NOUVELLES_COMMANDES'/g'
	REMPLACEMENT_GRAPHISME='s/<+GRAPHISME+>/'$GRAPHISME'/g'
	sed $REMPLACEMENT_LARGEUR ../Prototype.tex > TEMP_LARGEUR
	sed $REMPLACEMENT_HAUTEUR TEMP_LARGEUR > TEMP_HAUTEUR
	sed $REMPLACEMENT_NOUVELLES_COMMANDES TEMP_HAUTEUR > TEMP_NOUVELLES_COMMANDES
	sed $REMPLACEMENT_GRAPHISME TEMP_NOUVELLES_COMMANDES
}

############################
# Construction de Zodiaque #
############################
LARGEUR=1000.0
HAUTEUR=1618.033988749895
NOUVELLES_COMMANDES=
GRAPHISME='\\pspolygon*[linewidth=0pt,linecolor=white,linearc=50pt](400.0,0.0)(600.0,0.0)(600.0,400.0)(1000.0,400.0)(1000.0,600.0)(600.0,600.0)(600.0,1000.0)(800.0,1000.0)(500.0,\\Hauteur)(200.0,1000.0)(400.0,1000.0)(400.0,600.0)(0.0,600.0)(0.0,400.0)(400.0,400.0)'
CreationFichier > Zodiaque.tex

latex -jobname "Zodiaque" Zodiaque.tex $EFFACER_SORTIE
dvips -E -o "Zodiaque.ps" Zodiaque.dvi $EFFACER_SORTIE

######################
# Construction Titre #
######################
if [[ $NOM_LOGO = "2" ]];
then
	cp ../Distro.ttf ./
	LARGEUR=400.0
	HAUTEUR=100.0
	NOUVELLES_COMMANDES='\\DeclareFontFamily{T1}{Distro}{}'
	NOUVELLES_COMMANDES=$NOUVELLES_COMMANDES'\\DeclareFontShape{T1}{Distro}{m}{n}{<->EC_Distro}{}'
	NOUVELLES_COMMANDES=$NOUVELLES_COMMANDES'\\DeclareSymbolFont{Distro}{T1}{Distro}{m}{n}'
	case $CouleurNomLogo in
		1) COULEUR=Rouge ;;
		2) COULEUR=Vert ;;
		3) COULEUR=Bleu ;;
		0) COULEUR=Gris ;;
	esac
	GRAPHISME='\\rput(0.0,0.0){\\fontencoding{T1}\\fontfamily{Distro}\\fontsize{500pt}{2000pt}\\selectfont\\'$COULEUR'Fonce{}Cent\\'$COULEUR'Clair{}aurus}'
	CreationFichier > Titre.tex

	ttf2tfm Distro*.ttf -q -T T1-WGL4.enc -v EC_Distro.vpl REC_Distro.tfm >> ttfonts.map $EFFACER_SORTIE
	vptovf EC_Distro.vpl EC_Distro.vf EC_Distro.tfm $EFFACER_SORTIE
	latex -jobname "Titre" Titre.tex $EFFACER_SORTIE
	dvips -E -o "Titre.ps" Titre.dvi $EFFACER_SORTIE ;
fi

###############################
# Construction du Logo Simple #
###############################
LARGEUR=1618.033988749895
HAUTEUR=1000.0
NOUVELLES_COMMANDES=
GRAPHISME=
if [[ ( $NOM_LOGO = 2 ) && ( $TYPE_LOGO = 1 ) ]] ;
then
	GRAPHISME='\\rput(809.0169943749474,-202.25424859373686){\\includegraphics[width=1618.033988749895pt,height=250pt]{Titre.ps}}' ;
fi
if [[  ( ( $TYPE_LOGO = 1 ) && ( ( $COULEUR_LOGO = 1 ) || ( $COULEUR_LOGO = 0 ) ) ) || ( $TYPE_LOGO = 2 ) ]] ;
then
	GRAPHISME=$GRAPHISME'\\psbezier*[linewidth=0pt,linecolor=RougeFonce]{-}(0.0,0.0)(161.80339887498948,\\Hauteur)(161.80339887498948,\\Hauteur)(\\Largeur,\\Hauteur)'
	GRAPHISME=$GRAPHISME'\\psbezier*[linewidth=0pt,linecolor=RougeClair]{-}(0.0,0.0)(1456.2305898749055,0.0)(1456.2305898749055,0.0)(\\Largeur,\\Hauteur)'
	GRAPHISME=$GRAPHISME'\\rput(809.0169943749474,500.0){\\includegraphics[width=618.033988749895pt,angle=-58.28252558853899]{Zodiaque.ps}}'
	CreationFichier > LogoRouge.tex
	if [[ $LOGO_NB = 1 ]] ;
	then
		sed 's/Rouge/Gris/g' LogoRouge.tex > TEMP
		cat TEMP > LogoGris.tex
		latex -jobname "LogoGris" LogoGris.tex $EFFACER_SORTIE
		dvips -E -o "LogoGris.ps" LogoGris.dvi $EFFACER_SORTIE ;
		if [[ ( $TYPE_LOGO = 1 ) && ( $NOM_LOGO = 2 ) ]] ;
		then
			sed 's/^\(.*BoundingBox: \)\(-[0-9]* -[0-9]*\)\(.*\)$/\169.96601125010511 -683.5084971874737\3/g' LogoGris.ps > TEMP;
		else
			sed 's/^\(.*BoundingBox: \)\(-[0-9]*\)\(.*\)$/\169.96601125010511\3/g' LogoGris.ps > TEMP;
		fi
		cat TEMP > LogoGris.ps ;
	else
		latex -jobname "LogoRouge" LogoRouge.tex $EFFACER_SORTIE
		dvips -E -o "LogoRouge.ps" LogoRouge.dvi $EFFACER_SORTIE ;
		if [[ ( $TYPE_LOGO = 1 ) && ( $NOM_LOGO = 2 ) ]] ;
		then
			sed 's/^\(.*BoundingBox: \)\(-[0-9]* -[0-9]*\)\(.*\)$/\169.96601125010511 -683.5084971874737\3/g' LogoRouge.ps > TEMP;
		else
			sed 's/^\(.*BoundingBox: \)\(-[0-9]*\)\(.*\)$/\169.96601125010511\3/g' LogoRouge.ps > TEMP;
		fi
		cat TEMP > LogoRouge.ps ;
	fi ;
fi
if [[  ( ( $TYPE_LOGO = 1 ) && ( $COULEUR_LOGO = 2 ) ) || ( $TYPE_LOGO = 2 ) ]] ;
then
	GRAPHISME=$GRAPHISME'\\psbezier*[linewidth=0pt,linecolor=VertFonce]{-}(0.0,0.0)(161.80339887498948,\\Hauteur)(161.80339887498948,\\Hauteur)(\\Largeur,\\Hauteur)'
	GRAPHISME=$GRAPHISME'\\psbezier*[linewidth=0pt,linecolor=VertClair]{-}(0.0,0.0)(1456.2305898749055,0.0)(1456.2305898749055,0.0)(\\Largeur,\\Hauteur)'
	GRAPHISME=$GRAPHISME'\\rput(809.0169943749474,500.0){\\includegraphics[width=618.033988749895pt,angle=-58.28252558853899]{Zodiaque.ps}}'
	CreationFichier > LogoVert.tex
	latex -jobname "LogoVert" LogoVert.tex $EFFACER_SORTIE
	dvips -E -o "LogoVert.ps" LogoVert.dvi $EFFACER_SORTIE ;
	if [[ ( $TYPE_LOGO = 1 ) && ( $NOM_LOGO = 2 ) ]] ;
	then
		sed 's/^\(.*BoundingBox: \)\(-[0-9]* -[0-9]*\)\(.*\)$/\169.96601125010511 -683.5084971874737\3/g' LogoVert.ps > TEMP;
	else
		sed 's/^\(.*BoundingBox: \)\(-[0-9]*\)\(.*\)$/\169.96601125010511\3/g' LogoVert.ps > TEMP;
	fi
	cat TEMP > LogoVert.ps
fi
if [[  ( ( $TYPE_LOGO = 1 ) && ( $COULEUR_LOGO = 3 ) ) || ( $TYPE_LOGO = 2 ) ]] ;
then
	GRAPHISME=$GRAPHISME'\\psbezier*[linewidth=0pt,linecolor=BleuFonce]{-}(0.0,0.0)(161.80339887498948,\\Hauteur)(161.80339887498948,\\Hauteur)(\\Largeur,\\Hauteur)'
	GRAPHISME=$GRAPHISME'\\psbezier*[linewidth=0pt,linecolor=BleuClair]{-}(0.0,0.0)(1456.2305898749055,0.0)(1456.2305898749055,0.0)(\\Largeur,\\Hauteur)'
	GRAPHISME=$GRAPHISME'\\rput(809.0169943749474,500.0){\\includegraphics[width=618.033988749895pt,angle=-58.28252558853899]{Zodiaque.ps}}'
	CreationFichier > LogoBleu.tex
	latex -jobname "LogoBleu" LogoBleu.tex $EFFACER_SORTIE
	dvips -E -o "LogoBleu.ps" LogoBleu.dvi $EFFACER_SORTIE ;
	if [[ ( $TYPE_LOGO = 1 ) && ( $NOM_LOGO = 2 ) ]] ;
	then
		sed 's/^\(.*BoundingBox: \)\(-[0-9]* -[0-9]*\)\(.*\)$/\169.96601125010511 -683.5084971874737\3/g' LogoBleu.ps > TEMP;
	else
		sed 's/^\(.*BoundingBox: \)\(-[0-9]*\)\(.*\)$/\169.96601125010511\3/g' LogoBleu.ps > TEMP;
	fi
	cat TEMP > LogoBleu.ps
fi
if [[ $TYPE_LOGO = 1 ]] ;
then
	epstopdf --outfile=../$OUT_FILE Logo[GRVB]*.ps $EFFACER_SORTIE ;
fi
################################
# Construction du Logo Complet #
################################
if [[ $TYPE_LOGO = 2 ]] ;
then
	LARGEUR=2472.1359549995796
	HAUTEUR=1000.0
	NOUVELLES_COMMANDES=
	case $COULEUR_LOGO1 in
		1) COULEUR1=Rouge ;;
		2) COULEUR1=Vert ;;
		3) COULEUR1=Bleu ;;
		0) COULEUR1=Gris ;;
	esac
	GRAPHISME='\\rput(118.03398874989482,72.94901687515771){\\includegraphics[width=236.06797749978963pt]{Logo'$COULEUR1'.ps}}'
	case $COULEUR_LOGO2 in
		1) COULEUR2=Rouge ;;
		2) COULEUR2=Vert ;;
		3) COULEUR2=Bleu ;;
		0) COULEUR2=Gris ;;
	esac
	GRAPHISME=$GRAPHISME'\\rput(545.084971874737,190.98300562505256){\\includegraphics[width=618.0339887498948pt]{Logo'$COULEUR2'.ps}}'
	case $COULEUR_LOGO3 in
		1) COULEUR3=Rouge ;;
		2) COULEUR3=Vert ;;
		3) COULEUR3=Bleu ;;
		0) COULEUR3=Gris ;;
	esac
	GRAPHISME=$GRAPHISME'\\rput(1663.1189606246319,500.0){\\includegraphics[]{Logo'$COULEUR3'.ps}}'
	if [[ $NOM_LOGO = 2 ]] ;
	then
		GRAPHISME=$GRAPHISME'\\rput(1236.0679774997898,-309.0169943749474){\\includegraphics[width=2472.1359549995796pt,height=381.9660112501051pt]{Titre.ps}}' ;
	fi
	CreationFichier > LogoComplet.tex

	latex -jobname "LogoComplet" LogoComplet.tex $EFFACER_SORTIE
	dvips -E -o "LogoComplet.ps" LogoComplet.dvi $EFFACER_SORTIE
	if [[ $NOM_LOGO = 2 ]] ;
	then
		sed 's/^\(.*BoundingBox: \)\(-[0-9]* -[0-9]*\)\(.*\)$/\166.86404500042045 -897.0339887498949\3/g' LogoComplet.ps > TEMP;
	else
		sed 's/^\(.*BoundingBox: \)\(-[0-9]* -[0-9]*\)\(.*\)$/\166.86404500042045 -279\3/g' LogoComplet.ps > TEMP;
	fi
	echo LogoComplet
	cat TEMP > LogoComplet.ps
	epstopdf --outfile=../Logo.pdf LogoComplet.ps $EFFACER_SORTIE ;
fi

cd ..
rm -Rfv ConstructionLogo
