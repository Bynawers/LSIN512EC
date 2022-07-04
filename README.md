# LSIN512EC
![Project](https://img.shields.io/badge/Personnal-Project-2F77DF?labelColor=679EEE&style=for-the-badge)
![JavaScript](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=ffffff)

Correction session 1 et 2 de l'UE IN512 - Programmation Système

## IN512 - Examen Session 1 (jan. 2022)

### I - Tortue Logo

Logo est un langage de programmation pédagogique congu
en 1967. Il permet de réaliser des dessins en déplaçant un
curseur nommé tortue

On vous demande de concevoir une fonction en Rust pour
calculer la position de la tortue après l'exécution d'un pro-
gramme Logo simple.

La tortue est sur un repère cartésien où ¿ est l'abscisse
et y l'ordonnée. Au début du programme la tortue est aux
coordonnées (0,0). Par ailleurs, la tortue regarde initialement
vers la droite, c'est à dire dans le sens des a croissants.

Chaque instruction logo est modélisée par l'énumération
suivante :

    enum Instruction {
    Avance (132),
    Tourne
    }

• L'instruction Avance déplace la tortue de n unités dans
la direction où la tortue regarde. Par exemple, si la
tortue regarde vers la droite, Avance(5) incrémentera
2 de 5 unités.

• L'instruction Tourne opère une rotation de la tortue
de 90° dans le sens anti-horaire. Par exemple, si la
tortue regarde vers la droite, après l'instruction Tourne
la tortue regardera vers le haut.

Un programme Logo est modélisé par un vecteur de type
Vec<Instruction>. 

Par exemple :

    let programme = vec! [Instruction: : Avance (10),
                          Instruction::Tourne,
                          Instruction: :Avance(5) ,
                          Instruction: :Tourne,
                          Instruction: : Avance (15)];
    
Exécutons, pas à pas, le programme précédent,
• Avance(10), la tortue se déplace en (10,0) et regarde
vers la droite.
Tourne, la tortue regarde vers le haut.
Avance(5), la tortue se déplace en (10,5) et regarde
vers le haut.
Tourne, la tortue regarde vers la gauche.
Avance (15), la tortue se déplace en (-5,5) et regarde
vers la gauche.

> 1. Implémentez la fonction 
> 
>    fn execute_logo(programme:&Vec<Instruction>) -› (132, i32) 
>    
> qui exécute le programme Logo passé en paramètre et retourne un 2-uplet avec les coordonnées finales de la tortue. Commentez succinctement
> votre programme.

### II - Algèbre linéaire

Soit une matrice A carrée de dimension n. Soit i l'indice
des lignes et j l'indice des colonnes, nous pouvons noter auj
l'élément à la ligne i et la colonne j.

Par exemple pour n = 3,

    (a11, a12, a13)
    (a21, a22, a23)
    (a31, a32, a33)

En Rust, nous représentons les matrices dans l'ordre des
lignes (row major) à l'aide d'un vecteur de n x n nombres
flottants. 

Par exemple pour représenter la matrice identité
en dimension 3, nous écrirons :

    let ident : Vec<f64>
    vec! [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0];

> 1. Écrivez une fonction
>
> fn position(i: usize, j: usize, n: usize) -› usize 
>
> qui à partir des indices d'une matrice i et j compris entre 1 et n calcule la po-
> sition de l'élement dans le vecteur Rust correspondant.
> Par exemple position(2,3,3) renverra la valeur 5.
>
> On vous donne maintenant la fonction suivante où a est un
> vecteur qui représente une matrice de n x n éléments et b
> est un vecteur de n éléments.

    fn fonc (a: &Vec<f64>, b: &Vec<f64>, n: usize) -> Vec<f64> {
      assert_eq!(a,len(), n * n);
      assert_eg/(b.len(),n);
      let mut c = vec![0.0; n]:
      for j in 1..=n {
        for i in 1..=n {
          c[i - 1] += a[position(i, j, n)] * b[i - 1];
        }
      }

    #[test]
    fn test() {
      let a = vec![1.0,-1.0,0.0,
                 1.0, 1.0, 0.0, 
                -1.0, 0.0, 1.01];
      let b = vec [1.0, 2.0, 3,0];
      let c = fonc(&a, &b, 3);
    }

> 2. Dans le cas du test donner le contenu du vecteur c
> retourné par l'appel à fonc. Quelle opération d'algèbre
> linéaire est implémentée par fonc?

> 3. Dans quel ordre sont parcourus les éléments de a lors
> de l'appel à fonc ? Dessinez l'ordre de parcours pour
> une matrice 3 x 3 à l'aide d'une flèche.

> 4. Est-ce que l'ordre de parcours est efficace vis à vis du
> cache mémoire ? Quel problème risque t'on de rencontrer? 
> Quel impact cela peut avoir sur l'énergie consommée
> et la vitesse de calcul ?

> 5. Que faut-il changer dans l'algorithme pour avoir un
> algorithme plus performant ?

### III - Personnage de JdR

Cet exercice est à rendre sur une feuille indépendante
des deux autres exercices.

Le but de cet exercice est de créer une structure permettant
d'implémenter une fiche de personnage de jeu de rôle.
Un personnage possède un nom ainsi que 6 caractéris-
tiques : Force, Dextérité, Constitution, Intelligence, Sagesse
et Charisme. Chaque caractéristique est associée à une valeur
comprise entre 2 et 20. Pour chaque valeur de caractéris-
tique, un modificateur de caractéristique est généré et
est égal à -4 pour une valeur de 2 ou 3, -3 pour une valeur
de 4 ou 5,..., 0 pour une valeur de 10 ou 11, +5 pour une valeur de 20.

De plus, un personnage possède un certain nombre de com-
pétences ( Acrobaties, Arcanes, ...) qui peuvent être dif-
férentes d'un personnage à l'autre et qui peuvent changer
au cours du temps. Chaque compétence est associée à une
valeur bonus (+1, +2,...).

> 1. Proposez un structure Pers permettant d'implémenter
> une fiche de personnage.

> 2. Définissez une méthode new qui prend en paramètre un
> nom, génère les valeurs de caractéristique et retourne une
> instance de Pers. 
> 
> Chaque valeur est générée en tirant
> 4 valeurs aléatoires entre 1 et 6 dont on conservera la
> somme des 3 plus élevées. Par exemple, pour un tirage
> de 2,3,4,5, on obtiendra la valeur 12 = 5 + 4 + 3. Pour
> rappel, pour générer un nombre aléatoire entre 1 et 6.
> on pourra utiliser thread_rng().gen_range(1..=6).

> 3. La méthode précédente peut générer des person-
> nages très déséquilibrés.

> Définissez une méthode
> new with check qui génère un personnage comme la
> méthode précédente mais qui réexécute le processus tant
> que la somme des 6 valeurs de caractéristique n'est pas
> comprise entre 60 et 80.

> 4. Définissez des accesseurs pour récupérer, pour chaque
> caractéristique, sa valeur ainsi que son modificateur.

> 5. Implémentez le trait 
> 
>     std::ops::Index 
>     
> pour récupérer directement le modificateur d'une caractéristique à partir de son nom. 
> 
> Par exemple, si gandalf est de type Pers, gandalf ["Force"] renverra le modificateur de
> force du personnage. Pour rappel, le trait est défini comme suit :

    pub trait Index‹Idx>
    where Idx: ?Sized,
    type Output: ?Sized:
    fn index (&self, index: Idx) -> &Self :Output;

> 6. Définissez une méthode effectuant un jet de carac-
> téristique. Cette méthode prendra en paramètre une
> caractéristique et un degré de difficulté. 
> 
> L'opération consiste à tirer un nombre aléatoire entre 1 et 20 et à y
> ajouter le modificateur de caractéristique. 
> 
> Si le résultat est supérieur ou égal au degré de difficulté, la méthode
> doit retourner true. Si le résultat est inférieur au degré
> de difficulté, la méthode doit retourner false. 
> 
> Enfin, si la caractéristique n'existe pas, la méthode doit retourner
> une erreur.

> 7. Définissez une méthode permettant d'attribuer une
> valeur bonus à une compétence. 
> 
> Si la compétence n'existait pas auparavant, elle est ajoutée à la fiche
> de personnage.

> 8. Définissez une méthode retournant le bonus pour une
> compétence

> 9. Définissez une méthode effectuant un jet de compé-
> tence. Cette méthode prendra en paramètre une com-
> pétence et un degré de difficulté. 
> 
> L'opération consiste à
> tirer un nombre aléatoire entre 1 et 20 et à y ajouter la
> valeur bonus de la compétence (ou 0 si le personnage ne
> possède pas la compétence). 
> 
> Si le résultat est supérieur
> ou égal au degré de difficulté, la méthode doit retourner
> true. Si le résultat est inférieur au degré de difficulté,
> la méthode doit retourner false.

> 10. Donnez la fonction main qui montrera l'usage des méth-
> odes développées.

## IN512 - Examen Session 2 (juin. 2022)

### Problème 1: Représentation d'une longueur

Vous réaliserez un type Longueur représentant une mesure
de longueur.
Une instance de ce type comporte deux
attributs: 
la valeur de la longueur de type double et
l'unité utilisée (m, dm, cm, ...)


Pour chaque question ci-dessous, vous donnerez le code
Rust correspondant.


> 1. L'énumération Unite représentant les unités utilisées
> (m, dm, cm).

> 2. Déclaration du type Longueur.

> 3. Méthode new qui prend en paramètre une valeur et
> une unité et retourne une instance de Longueur

> 4. Accesseurs pour la valeur et l'unité.

> 5. Dériver simplement le trait Debug pour la structure
> (pour l'affichage de messages de débogage par exem-
> ple).

> 6. Conversion d'une longueur dans une unité en une
> autre longueur d'une unité différente.

> 7. Implémenter le trait std: :ops: : Index pour con-
> vertir une longueur.
> Par exemple,

si Longueur{
valeur: 1.2, unite: Unite: :m} est une instance
nommée 1g, 1g[Unite::cm] renverra la valeur 120
(1.2m = 120cm). Pour rappel, le trait est défini
comme suit :

    pub trait Index<Idx>
        where Idx: ?Sized.
    {
        type Output: ?Sized;
        fn index (&self, index: Idx)
            -› &Self: : Output;
    }

> 8. Addition d'une longueur à une autre en précisant
> l'unité du résultat.

> 9. Fonction main qui montrera l'usage des méthodes
> développées.

### Problème 2: Machine à double ruban

Le professeur Shadoko a mis au point une machine à
calculer à double ruban. Il nous demande de simuler
son fonctionnement en Rust. La machine possède deux
rubans: 

le premier ruban contient des commandes, le
deuxième ruban des entiers.
Le premier ruban sera modélisé par un vecteur de type
Vec<Commande», où Commande est défini par l'énumération

    enum Commande {
        Échange (usize, usize),
        Incrémente(usize),
        Écrête (i32) ,
    }
    
Le deuxième ruban sera modélisé par un vecteur de type
Vec<i32>.

Les positions sur le deuxième ruban seront numérotées,
comme les vecteurs en Rust, en partant de zéro.
La commande Échange va échanger les valeurs
de deux cases du deuxième ruban.
Par exemple:

Échange (3,4) échangera les valeurs des cases 3 et 4
du deuxième ruban

La commande Incrémente va incrémenter de 1
l'une des valeur du deuxième ruban. Par exemple,
Incrémente(6) va incrémenter d'une unité la case 6
du deuxième ruban.

La commande Écrête parcourt l'ensemble du deux-
ième ruban et remplace toutes les valeurs strictement
supérieures au seuil passé en paramètre. Par exemple,
Écrête(3) remplacera toutes les valeurs strictement
supérieures à 3 par 3 dans le deuxième ruban.

Le professeur Shadoko a préparé les deux rubans suivants:

    let ruban1 = vec![
        Commande: :Échange(1, 2)
        Commande ::Incrémente(3),
        Commande: : Echange (3, 1),
        Commande: :Écrête(3),
    ]

    let mut ruban2 = vec! [0, 1, 2, 3, 4];

Après exécution de la machine à double ruban, le deux-
ième ruban aura les valeurs [0, 3, 1, 2, 3].

> 1. Pourquoi est-il nécessaire de rajouter le qualificatif
> mut lors de la définition de ruban2 mais pas lors de
> la définition de ruban1 ?

> 2. On décide d'exécuter la machine une deuxième fois.
> Les commandes du premier ruban n'ont pas changé.
> Mais le deuxième ruban contient maintenant les
> valeurs [0, 3, 1, 2, 3]. Que contiendra le deux-
> ième ruban après une nouvelle exécution de la ma-
> chine ? Justifiez étape par étape.

> 3. Implémentez la fonction execute (ruban1 : &Vec<Commande›, ruban2: &mut Vec<i32>)
> qui simule une exécution de la machine à double ruban.

### Problème 3: Pile LIFO

Dans ce problème nous souhaitons implémenter une pile
LIFO en rust. La pile contiendra des entiers signés 32
bits. Les deux structures suivantes sont données :

    pub struct Stack {
        head: Option<Box<Node>>
    }

    pub struct Node
        value: 132,
        next: Option<Box<Node>>
    }
    
- Stack modélise la pile. Le champ head est de type
Option. Si la pile est vide il prends la valeur None.
Sinon, head pointe vers le nœud en haut de la pile.
- Node modélise un élément dans la pile. Le champ
value contient un entier signé. Le champ next
pointe vers le nœud suivant (plus bas dans la pile).
    
> 1. Quel effet aura l'utilisation du type Box<>
> sur l'allocation mémoire des nœuds ? Que se passe t'il
> si on utilise directement le type Option<Node› dans
> les champs head et next?

> 2. Implémentez une méthode fn new() -› Self pour
> Stack qui initialise une pile vide.
Soit la méthode push ci-dessous:

    impl Stack {
        fn push (&mut self, value: 132) {
            let node = Box::new (Node {
                value: value,
                next: std::men::take (&mut self.head) ,
            });
            self.head = Some(node);
        }
    }
    
> 3. Que fait la méthode push?

> 4. Expliquer la syntaxe Some( ) ? À quoi correspond-
> elle?

> 5. Justifiez l'utilisation de std::mem::take ? Que se
> passe t'il si on ne l'utilise pas ?

> 6. Que signifie la syntaxe &mut devant self.head ? À
> quoi sert-elle?

    impl Stack {
        fn pop (&mut self) -> 132 { . . . }
        fn print (&self) { . . . }
    
> 7. Implémentez la méthode pop qui dépile le nœud en
> haut de la pile et retourne sa valeur. Lorsque la pile
> est vide, un message de panique sera affiché avec
> panic!, expect, ou équivalent.