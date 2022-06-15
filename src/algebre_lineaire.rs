/* Question 1 */
pub fn position(i: usize, j:usize, n:usize) -> usize {

    assert!(i < n && i > 0);
    assert!(j < n && j > 0);

    let mut position: i32 = -1;

    for column in 0..n {

        for row in 0..n {

            if column == i-1 && row == j-1 {
                position += 1;
                println!("position: {}", position);
                return position as usize;
            }
            position += 1;
        }
    }
    return 0;
}

/* Question 2 */
// Multiplication d'une matrice colonne par une matrice donnant une matrice colonne
// Résultat : c [1.0, 0.0, 3.0]

/* Question 3 */
// Les éléments de a sont parcourus de haut en bas
// Ordre de parcours : a11 -> a21 -> a31 -> a12 -> a22 -> a32 -> a13 -> a23 -> a33

/* Question 4 */
// L'ordre de parcours n'est pas efficace vis-à-vis du cache car lorsque l"on traitre des grandes matrix,
// le cache va devoir réserver un nouveau bloc mémmoire à chaque itération ce qui va le saturer et
// la ram plus lente va prendre le relais réduisant la vitesse de calcul et augmentant la consomation

/* Question 5 */
// Il faut parcourir la matrix horizontalement par bloc par conséquent les variables qui vont suivrent
// seront déjà alloué et prête pour le calcul