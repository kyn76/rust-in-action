// enum State {
//     Start,
//     Dead,
//     Hall,
//     Table,
//     Pot,
//     FirePlace,
//     Carpet,
//     FinalBoss,
//     End,
// }

pub struct StartState;
pub struct Room1State;
pub struct Room2State;
pub struct EndState;

pub trait State {
    fn message(&self) -> &str;
    fn next(&self, choice: &str) -> Box<dyn State>;
    fn is_terminal(&self) -> bool {
        false
    }
}

impl State for StartState  {
    fn message(&self) -> &str {
        "Vous commencez le jeu.\n\n1. Aller dans la pièce 1\n2. Aller dans la pièce 2"
    }

    fn next(&self, choice: &str) -> Box<dyn State> {
        match choice {
            "1" => {
                println!("Vous allez dans la pièce 1.");
                Box::new(Room1State)
            }
            "2" => {
                println!("Vous allez dans la pièce 2.");
                Box::new(Room2State)
            }
            _ => {
                println!("Choix invalide.");
                Box::new(StartState)
            }
        }
    }
}

impl State for Room1State  {
    fn message(&self) -> &str {
        "Pièce 1.\n\n1. Finir le jeu"
    }

    fn next(&self, choice: &str) -> Box<dyn State> {
        match choice {
            "1" => {
                println!("Vous allez dans la pièce finale.");
                Box::new(EndState)
            }
            _ => {
                println!("Choix invalide.");
                Box::new(Room1State)
            }
        }
    }
}

impl State for Room2State  {
    fn message(&self) -> &str {
        "Pièce 2.\n\n1. Finir le jeu"
    }

    fn next(&self, choice: &str) -> Box<dyn State> {
        match choice {
            "1" => {
                println!("Vous allez dans la pièce finale.");
                Box::new(EndState)
            }
            _ => {
                println!("Choix invalide.");
                Box::new(Room2State)
            }
        }
    }
}

impl State for EndState {
    fn message(&self) -> &str {
        "Fin du jeu."
    }

    fn next(&self, _: &str) ->  Box<dyn State> {
        Box::new(EndState)
    }

    fn is_terminal(&self) -> bool {
        true
    }
}