pub struct Stack {
    head: Option<Box<Node>>
}

#[derive(Debug)]
pub struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

impl Stack {
    pub fn new() -> Stack {
        return Stack {
            head: None,
        }
    }

    pub fn push(&mut self, value: i32) {
        let node = Box::new(Node {
            value: value,
            next: std::mem::take(&mut self.head),
        });
        self.head = Some(node);
    }

    pub fn pop(&mut self) -> i32 {
        match &self.head {
            Some(node) => {
                /*
                let node = Box::new(Node { 
                    value: node.next.as_ref().unwrap().value.clone(),
                    next: std::mem::take(&mut self.head.as_ref().unwrap().next),
                });
                self.head = Some(node);*/
                
                return 1;
            },
            None => { panic!("no value to depop"); }
        }
    }
}

/* Question 1 */
// L'élément Box<> permet d'indiquer que la mémoire de ce bloc
// sera allouer plus tard,
// Si on enlève le Box<>, il y aura une erreur indiquant que l'espace doit alloué
// doit être connu avant durant la compilation

/* Question 3 */
// La méthode push va à partir d'une valeur i32 rentré dans la fonction,
// initialiser une nouveau Node possédant cette valeur et prenant le pointeur
// de l'ancien pointeur suivant du sommet de la pile, puis l'ancien head va récupéré
// le pointeur vers le nouveau noeud crée.
// Plus simplement : un nouveau noeud au sommet de la pile est ajouté.

/* Question 4 */
// Some( ) permet d'assigner une valeur à un type Option<T>

/* Question 5 */
// Take permet de s'approprier un champ struct en le remplaçant par une valeur "vide"
// si on le l'utilise pas, un move out d'une dereference d'un pointeur

/* Question 6 */ 
// la syntaxe &mut, indique que c'est une référence mutable, permettant ainsi de modifier 
// la valeur d'un type dans pour autant déroger aux règles de borrow

/* Question 7 */