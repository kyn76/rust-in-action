use crate::inventory::Inventory;

pub struct StartState;
pub struct HallState;
pub struct TableState;
pub struct PotState;
pub struct DoorState;
pub struct HearthState;
pub struct CarpetState;

pub struct FinalBossState;
pub struct EndState;
pub struct DeadState;

pub trait State {
    fn message(&self, inventory: &Inventory) -> &str;
    fn next(&self, choice: &str, inventory: &mut Inventory) -> Box<dyn State>;
    fn is_terminal(&self) -> bool {
        false
    }
}

impl State for StartState  {
    fn message(&self, _: &Inventory) -> &str {
        "\n\n\nBienvenue... Appuyez sur entrée pour démarrer l'aventure.\n"
    }

    fn next(&self, choice: &str, _: &mut Inventory) -> Box<dyn State> {
        match choice {
            _ => {
                println!("--- Début de l'aventure ---");
                Box::new(HallState)
            }
        }
    }
}

impl State for HallState {
    fn message(&self, _: &Inventory) -> &str {
        "\nHall.\n\n1. Inspecter la table\n2. Inspecter la cheminée\n3. Se rendre devant la porte\n4. Inspecter le tapis\n"
    }

    fn next(&self, choice: &str, _: &mut Inventory) -> Box<dyn State> {
        match choice {
            "1" => {
                println!("Inspectons cette table...");
                Box::new(TableState)
            }
            "2" => {
                println!("Vous vous rapprochez de la cheminée.");
                Box::new(HearthState)
            }
            "3" => {
                println!("Allons voir cette jolie porte.");
                Box::new(DoorState)
            }
            "4" => {
                println!("Vous regardez le tapis de plus près.");
                Box::new(CarpetState)
            }
            _ => {
                println!("Choix invalide.");
                Box::new(HallState)
            }
            
        }
    }
}

impl State for TableState  {
    fn message(&self, inventory: &Inventory) -> &str {
        if !inventory.has_key {
            "\nTable.\n\n1. Interagir avec le chandelier\n2. Interagir avec le vase\n3. Interagir avec la vaisselle\n4. Ignorer la table\n"
        } else {
            "\nTable.\n\n1. Interagir avec le chandelier\n3. Interagir avec la vaisselle\n4. Ignorer la table\n"
        }
    }

    fn next(&self, choice: &str, inventory: &mut Inventory) -> Box<dyn State> {
        match choice {
            "1" => {
                println!("Chandelier");
                Box::new(TableState)
            }
            "2" => {
                if !inventory.has_key {
                    println!("Vase");
                    Box::new(PotState)
                } else {
                    println!("Choix invalide.");
                    Box::new(TableState)
                }
            }
            "3" => {
                println!("Vaisselle");
                Box::new(TableState)
            }
            "4" => {
                println!("Vous ignorez la table.");
                Box::new(HallState)
            }
            _ => {
                println!("Choix invalide.");
                Box::new(TableState)
            }
        }
    }
}

impl State for PotState  {
    fn message(&self, _: &Inventory) -> &str {
        "\nVase.\n\n1. Mettre sa main dans le vase\n2. Admirer le vase\n3. Casser le vase\n4. Ignorer le vase\n"
    }

    fn next(&self, choice: &str, inventory: &mut Inventory) -> Box<dyn State> {
        match choice {
            "1" => {
                println!("Vous mettez votre main dans le vase.");
                Box::new(DeadState)
            }
            "2" => {
                println!("Vous admirez le vase.");
                Box::new(PotState)
            }
            "3" => {
                println!("Vous cassez le vase. L'acide chlorhydrique se répand sur le sol. Vous récupérez une clef.");
                inventory.add_key();
                Box::new(TableState)
            }
            "4" => {
                println!("Vous ignorez le vase.");
                Box::new(TableState)
            }
            _ => {
                println!("Choix invalide.");
                Box::new(PotState)
            }
        }
    }
}

impl State for CarpetState {
    fn message(&self, _: &Inventory) -> &str {
        "\nTapis.\n\n1. S'essuyer les pieds\n2. Soulever le tapis\n3. Ignorer le tapis\n"
    }

    fn next(&self, choice: &str, inventory: &mut Inventory) -> Box<dyn State> {
        match choice {
            "1" => {
                println!("Vous essuyez les pieds. D'accord.");
                Box::new(HallState)
            }
            "2" => {
                if !inventory.has_sword {
                    println!("Vous soulevez le tapis. Il y a un escalier qui mène à un piédestal muni d'une épée.\nBravo, vous êtes maintenant équipé d'une arme !");
                    inventory.add_sword();
                } else {
                    println!("Vous soulevez le tapis. Il y a un escalier qui mène à un piédestal. Vous avez déjà récupéré l'épée ici. Bon vent !");
                }
                Box::new(HallState)
            }
            "3" => {
                println!("Vous ignorez le tapis.");
                Box::new(HallState)
            }
            _ => {
                println!("Choix invalide.");
                Box::new(CarpetState)
            }
        }
        
    }
}

impl State for HearthState {
    fn message(&self, _: &Inventory) -> &str {
        "\nVous inspectez en détail la cheminée, il y a trois cercles de pierres numérotés de 1 à 3 que l'on peut enfoncer une fois chacun. Il faut peut-être trouver la bonne combinaison...\n"
    }

    fn next(&self, choice: &str, _: &mut Inventory) -> Box<dyn State> {
        match choice {
            "123" => {
                println!("Rien ne se passe...");
                Box::new(HearthState)
            }
            "132" => {
                println!("Rien ne se passe...");
                Box::new(HearthState)
            }
            "213" => {
                println!("La cheminée se met à trembler et une trappe de pierre s'ouvre devant vous.\nUn ours bleu gigantesque en sort et vous tue sur le coup.");
                Box::new(DeadState)
            }
            "231" => {
                println!("Rien ne se passe...");
                Box::new(HearthState)
            }
            "312" => {
                println!("Rien ne se passe...");
                Box::new(HearthState)
            }
            "321" => {
                println!("Rien ne se passe...");
                Box::new(HearthState)
            }
            "retour" => {
                println!("Vous ignorez la cheminée.");
                Box::new(HallState)
            }
            _ => {
                println!("Choix invalide. Entrez une combinaison des 3 chiffres 1, 2, 3 sans les répéter. Ou 'retour' pour ignorer cette énigme.");
                Box::new(HearthState)
            }

        }
    }
}

impl State for DoorState {
    fn message(&self, _: &Inventory) -> &str {
        "\nPorte.\n\n1. Ouvrir la porte\n2. Faire demi-tour\n"
    }

    fn next(&self, choice: &str, inventory: &mut Inventory) -> Box<dyn State> {
        match choice {
            "1" => {
                if !inventory.has_key {
                    println!("Vous tournez la poignée. La porte ne s'ouvre pas, elle semble vérouillée... Vous faîtes demi-tour.");
                    Box::new(HallState)
                } else {
                    println!("Vous ouvrez la porte.");
                    Box::new(FinalBossState)
                }
            }
            "2" => {
                println!("Vous faites demi-tour.");
                Box::new(HallState)
            }
            _ => {
                println!("Choix invalide.");
                Box::new(DoorState)
            }
        }
    }
}

impl State for FinalBossState {
    fn message(&self, _: &Inventory) -> &str {
        "\nMygale géante. Tapez entrée pour l'affronter.\n"
    }

    fn next(&self, _: &str, inventory: &mut Inventory) -> Box<dyn State> {
        if inventory.has_sword {
            println!("Vous terrassez l'arachnide à l'aide de votre épée d'argent ! Bravo !");
            Box::new(EndState)
        } else {
            println!("Vous n'avez rien pour vous défendre. La mygale vous dévore.");
            Box::new(DeadState)
        }
    }
}

impl State for DeadState {
    fn message(&self, _: &Inventory) -> &str {
        "\nVous êtes mort. Quelle tragédie :'(\n"
    }

    fn next(&self, _: &str, _: &mut Inventory) ->  Box<dyn State> {
        Box::new(EndState)
    }
}

impl State for EndState {
    fn message(&self, _: &Inventory) -> &str {
        "\n--- Fin de l'aventure ---"
    }

    fn next(&self, _: &str, _: &mut Inventory) ->  Box<dyn State> {
        Box::new(EndState)
    }

    fn is_terminal(&self) -> bool {
        true
    }
}