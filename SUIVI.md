# Secret Sharing

## Mercredi 13 Mars

**TH1:**

* **Clémence, Gabrielle, Alexis:** Découverte de l'objectif 1, recherche et mise en commun des résultats.

**TH2:**

* **Clémence:** Documentation de l'objectif 1 en LaTeX sur Overleaf.
* **Alexis, Gabrielle:** Travail sur l'objectif 1 et contact avec M. Rambaud pour questions techniques.

## Mercredi 20 Mars

**TH1:**

* **Gabrielle, Alexis:** Recherche sur l'objectif 2.
* **Clémence:** Partage des résultats de l'objectif 1 et rédaction en LaTeX.

**TH2:**

* **Gabrielle, Alexis:** Recherche sur l'objectif 2 et rédaction en LaTeX.
* **Clémence:** Découverte de l'objectif 3.

## Mardi 26 Mars

**TH1:**

* **Clémence, Gabrielle, Alexis:** Recherche sur l'objectif 3, prise de rendez-vous avec M. Rambaud, lecture de bibliographie.

**TH2:**

* **Clémence:** Recherche sur l'objectif 3.
* **Gabrielle:** Recherche et documentation sur l'objectif 3.
* **Alexis:** Recherche sur l'objectif 3 et découverte des bibliothèques pour l'implémentation.

## Jeudi 28 Mars


* **Rendez-vous avec M. Rambaud:**
* **Gabrielle**: finalise la rédaction des objectifs 1 et 2.

## Mardi 02 Avril

**TH1:**

* **Alexis:** Travail sur la bibliothèque tfhe.rs.
    * Decouverte des fonctions `encrypt_lwe_ciphertext` et `decrypt_lwe_ciphertext`
* **Gabrielle, Clémence:** Recherche sur l'objectif 3.

**TH2:**

* **Alexis:** Travail sur tfhe.rs.
    * Test de code dans le fichier `lwe.rs`
* **Gabrielle, Clémence:** Découverte de Rust et travail sur le code.

## Lundi 08 Avril:


* **Rendez-vous avec Matthieu Rambaud pour discuter de l'objectif 3.**

## Mardi 09 Avril

**TH1:**

* **Mise en commun des connaissances après le rendez-vous.**

**TH2:**

* **Alexis:** Avance sur l'implémentation.
    * Creation des vecteurs et matrices.
    * Utilisation des libs `nalgebra` et `rand_distr`.
* **Clémence:** Apprentissage de Rust.
    * Apprentissage sur le site : `https://levelup.gitconnected.com/rust-has-a-dark-side-b05f3620b37c`
* **Gabrielle:** Documentation en LaTeX.

## Lundi 29 Avril

**TH1 & TH2:**

* **Alexis, Gabrielle, Clémence:** Travail sur le rendu de mi-parcours en LaTeX.

## Jeudi 02 Mai

**TH1 & TH2:**

* **Alexis:** Avance sur l'étape intermédiaire Rust.
    * Utilisation des libs `rand::distributions`
    * Creation des fonctions `polynome_evaluation_coeff`, `generator_polynome` et `share_generation`.
* **Gabrielle:** Aide sur l'étape intermédiaire et correction du document LaTeX.
* **Clémence:** Correction du document LaTeX.

**Après-midi:**

* **Alexis:** Avance sur l'étape intermédiaire Rust.
    * Finalisation des fonctions `polynome_evaluation_coeff` et `share_generation`.
* **Gabrielle, Clémence:** Finalisation du rendu de mi-parcours.

## Mardi 07 Mai

**TH1:**

* **Alexis, Gabrielle, Clémence:** Travail sur le rendu des impacts sociaux et environnementaux (brouillon).

**TH2:**

* **Alexis, Gabrielle:** Travail sur les étapes intermédiaires (tests Rust, recherche mathématique).
    * Fonction `projection_sur_a2` pour n = 2
* **Clémence:** Mise en forme du rendu sur les impacts.
* **Gabrielle:** Relecture et correction du rendu.

## Mercredi 15 Mai 

**TH1:**

* **Tout le groupe:** Travail sur l'étape intermédiaire 2 (partage de secret avec polynôme de degré 2). Résultats positifs sans modulo q, mais pas de preuve pour le cas avec modulo q.

**TH2:**

* **Alexis:** Finalisation de l'étape intermédiaire 3 (calcul du ième share de L(s,u,v)).
    * Fonctions `share_l_i` et `l`.
    * Utilisation de la lib `Vec`
* **Clémence:** Installation de Rust et aide mathématique.
* **Gabrielle:** Tests prouvant l'erreur dans l'étape 2, envoi d'un mail au professeur.

## Mercredi 22 Mai

**TH1:**

* **Clémence:** Rédaction du compte-rendu avec les codes Rust. (Dispo sur le Git)
* **Alexis, Gabrielle:** Travail sur l'implémentation (transmission de ciphertext chiffré).

**TH2:**

* **Clémence:** Suite du compte-rendu.
* **Alexis, Gabrielle:** Étude du paragraphe C.4 de "Share & Shrink" et correction de l'objectif 3 intermédiaire.

## Mardi 28 Mai:


* **Réunion et visioconférence avec M. Rambaud.**
* **Redéfinition des objectifs finaux:**
    * Étude de la linéarité de la fonction affine en sk et r.
    * Implémentation des fonctions Encrypt, Decrypt et EoD.
    * Réalisation de l'objectif avec n=3.

## Mercredi 5 Juin:
**TH1:**
* **Alexis:** Implémentation des fonctions Encrypt et Decrypt en Rust.
* **Clémence, Gabrielle:** Recherche mathématique: démonstration de l'affinité de EoD en (e1',e2',s).

**TH2:**
* **Alexis:** suite TH1.
* **Clémence, Gabrielle:** 
    * Rédaction Latex de la démonstration. 
    * Discussion sur les notations, la signification de plusieurs variables.
    * Validation par l'encadrant.
    * Recherche sur le cas n=3: étude des consignes de M. Rambaud: anneau Gal(2,e), Lagrange sur des polynômes, plongement des shares dans cet anneau.


## Mercredi 12 Juin:
**TH1 et TH2:**
* **Alexis:** la fonction EoD ne renvoie pas le bon output, il essaye de comprendre pourquoi
* **Clémence, Gabrielle:** Recherche mathématique: démonstration de l'affinité de EoD en (e1',e2',s). Nous n'arrivons pas a trouver en quoi les elements des matrices N et M sont connus publiquement... + aide sur le code d'alexis 
* **Tous :** Commencement du poster 

## Mardi 18 Juin:
**17h35 à 20h:**
* **Alexis, Clémence, Gabrielle:** Conception du poster.
* **Clémence :** Echange avec Matthieu Rambaud: quels coefficients sont publics dans la formule de EoD, comment exprimer cette dernière uniquement à partir des coefficients publics.

## Lundi 24 juin:
**Matin**
* **Alexis, Gabrielle:** codage de la génération aléatoire gaussienne de clé, débogage. Découverte en particulier de l'article "Semi-Homomorphic Encryption and Multiparty Computation" qui met en lumière les erreurs.


* **Clémence :** Mise au propre de la formule de EoD trouvée grâce à Gabrielle.
Echange avec Matthieu Rambaud pour une prise de rdv + questions sur la théorie de Galois (cas n=3). Bilan:
 GR(2,e) = Z/qZ[Y]/(Y²+Y+1) 
Il a notamment 4 éléments x_i dont les différences x_i-x_j sont inversibles:
0
1
Y
Y+1
Ainsi, ces 4 éléments peuvent être utilisés comme points d'évaluation de polynômes pour faire du Shamir secret sharing pour n=3.

Tirer un polynôme de degré 1 
s+a1.X avec a1 aléatoire dans  GR(2,e) 
Si vous forcez à 0 la 2e coordonnée de a1, ce n'est plus aléatoire donc vous perdez la secrecy
 

**Après-midi** 
* **Clémence, Gabrielle:** Trouvent la règle de multiplication modulo Gal(e,2), validée par Mr Rambaud. 
Aident Alexis au debogage des fonctions E et D, avec D renvoyant \lceil  (2/q)*(c2-sc1 mod(q)) \rfloor.
* **Alexis :** Deboge les fonctions E et D pour que DoE donne bien l'identité.

Nous n'avons pas encore commencé l'implémentation de la forme affine. 
On a envoyé un mail au groupe broadcast encryption, sous les conseils de Mr Rambaud, pour qu'ils nous envoient leur rendu théorique, il nous aidera sûrement pour l'implémentation...
****

## Mardi 25 Juin:
**Matin:**
* **Alexis:** Debogage d'Encrypt et Decrypt. Découverte: cela fonctionne uniquement lorsque l'erreur e < q/4.
Tests avec les paramètres suivant des lois gaussiennes.
* **Clémence, Gabrielle :** Recherche théorique pour essayer de comprendre les bugs du code (majoration de l'erreur - on ne trouve que 3(q-1) comme majorant, étude de GLH22 (=> lois gaussiennes à valeurs dans Z/qZ))
Echange avec M. Rambaud et prise de rendez-vous.
Implémentation de EoD. Problème: e'1, e'2, r' ne peuvent être mis en inputs car générés par E...


**Après-midi** 
* **Gabrielle** Mise au propre du Git + aide au débogage
* **Clémence** Recherche théorique + aide au débogage
* **Alexis** Debogage EoD ok, a eu l'idée de mettre un .abs à l'output de D

- Sinon, rdv avec le prof : mise en place de mesure de sécurité sur tous les aléas gaussiens, pour pas qu'ils ne dépassent q/4. => mettre min(q/4, output) pour tous
- Confirmation de la formule EoD implémentée qui ne ressemble pas a une matrice mais qui en est finalement une.
- Pour plus de sécurité, on choisit la clé secrete uniforme dans {-1,0,1}. Pas délirant, se fait en pratique...


## Mercredi 26 Juin:
**Matin:**
* **Alexis:** Réécriture d'Encrypt et Decrypt notamment avec l'astuce du "min(q/4, output)". Cela fonctionne !
Ecriture de EoD grâce à ces nouvelles fonctions. 
Ecriture du Secret Sharing de la clé, pour n = 2.
* **Gabrielle :** Aide à la création de l'algo Secret Sharing.
Preuve de la nécessité concrète d'une erreur inférieure à q/4 en valeur absolue (car sinon, le décodage de Delta.m est faux - cf. les notions de COM105).
Mise au propre du rendu LateX.
* **Clémence:** Aide à la preuve de la nécessité concrète d'une erreur inférieure à q/4 en valeur absolue et à la rédaction du document LateX.
Preuve de la nécessité abstraite d'une erreur inférieure à q/4 en |.| : comme e = Er + e2 -se1, e suit finalement une loi gaussienne. En exprimant P(|e| > q/4) à l'aide d'intégrales, on obtient une majoration d'au plus 10^{-82} !!

**Après-midi** 
* **Gabrielle** Ecriture de l'algo en Latex pour n=2 + aide au debogage + avancée sur n=3
* **Clémence** avancée sur n=3 (adaptation de EoD pour l'anneau quotient) + aide au débogage
* **Alexis** Debogage du cas n=2, il y a un pb au niveau de la reconstruction des cypher-texts, on en a parlé avec Mr Rambaud 