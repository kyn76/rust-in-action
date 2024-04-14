use crate::inventory::Inventory;
use crate::streamed_print;

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
        "\n\n\nBienvenue dans cette aventure interactive... Appuyez sur entrée.\n"
    }

    fn next(&self, choice: &str, _: &mut Inventory) -> Box<dyn State> {
        match choice {
            _ => {
                streamed_print("\n--- Début de l'aventure ---\n\nVous vous trouvez enfermé dans une étrange maison.\n");
                Box::new(HallState)
            }
        }
    }
}

impl State for HallState {
    fn message(&self, _: &Inventory) -> &str {
        "\nVous êtes dans la pièce principale. Une large table occupe la pièce. Sur votre gauche, vous pouvez voir une cheminée ancienne. Devant celle-ci, au sol, un tapis attire votre attention. Au fond de la pièce vous distinguez une porte mystérieuse.\n\n1. Inspecter la table\n2. Inspecter la cheminée\n3. Se rendre devant la porte\n4. Inspecter le tapis\n"
    }

    fn next(&self, choice: &str, _: &mut Inventory) -> Box<dyn State> {
        match choice {
            "1" => {
                streamed_print("\nInspectons cette table...\n");
                Box::new(TableState)
            }
            "2" => {
                streamed_print("\nVous vous rapprochez de la cheminée.\n");
                Box::new(HearthState)
            }
            "3" => {
                streamed_print("\nAllons voir cette jolie porte.\n");
                Box::new(DoorState)
            }
            "4" => {
                streamed_print("\nVous inspectez le tapis de plus près.\n");
                Box::new(CarpetState)
            }
            _ => {
                streamed_print("\nChoix invalide.\n");
                Box::new(HallState)
            }
            
        }
    }
}

impl State for TableState  {
    fn message(&self, inventory: &Inventory) -> &str {
        if !inventory.has_key {
            "\nSur la table, vous observez un chandelier, un vase aux motifs détaillés et de la vaisselle onéreuse.\n\n1. Interagir avec le chandelier\n2. Interagir avec le vase\n3. Interagir avec la vaisselle\n4. Ignorer la table\n"
        } else {
            "\nSur la table, vous observez un chandelier et de la vaisselle onéreuse.\n\n1. Interagir avec le chandelier\n3. Interagir avec la vaisselle\n4. Ignorer la table\n"
        }
    }

    fn next(&self, choice: &str, inventory: &mut Inventory) -> Box<dyn State> {
        match choice {
            "1" => {
                streamed_print("\nC'est un beau chandelier, les bougies sont toutes éteintes. Rien à dire de plus.\n");
                Box::new(TableState)
            }
            "2" => {
                if !inventory.has_key {
                    streamed_print("\nCe vase semble renfermer un secret, inspectons le de plus près...\n");
                    Box::new(PotState)
                } else {
                    streamed_print("\nChoix invalide.\n");
                    Box::new(TableState)
                }
            }
            "3" => {
                streamed_print("\nJolie vaisselle, mais ce n'est pas l'heure de manger.\n");
                Box::new(TableState)
            }
            "4" => {
                streamed_print("\nVous ignorez la table.\n");
                Box::new(HallState)
            }
            _ => {
                streamed_print("\nChoix invalide.\n");
                Box::new(TableState)
            }
        }
    }
}

impl State for PotState  {
    fn message(&self, _: &Inventory) -> &str {
        "\nVous scrutez le vase en détail. Que faire ?\n\n1. Mettre sa main dans le vase\n2. Admirer le vase\n3. Casser le vase\n4. Ignorer le vase\n"
    }

    fn next(&self, choice: &str, inventory: &mut Inventory) -> Box<dyn State> {
        match choice {
            "1" => {
                streamed_print("\nVous mettez votre main dans le vase sans grande prudence. Votre main s'immerge dans un liquide inconnu. Une vive douleur vous fait hurler et retirer la main du vase. Trop tard ! Il était rempli d'acide chlorhydrique concentré ! Votre main se dissout à vue d'oeil, la douleur vous fait tomber dans les pommes. Pendant votre comma, on vous assaisonne et on vous sert en dîner à un cyclope nommé Rudy. Bon appétit à lui ! Mais RIP à vous :(\n");
                Box::new(DeadState)
            }
            "2" => {
                streamed_print("\nWow, ça c'est une jolie faïence.\n");
                Box::new(PotState)
            }
            "3" => {
                streamed_print("\nVous cassez le vase. Son contenu acide se répand sur le sol. Vous l'avez échappé belle ! Vous remarquez qu'il contenait une clef. Avec prudence, vous la ramassez. Vous obtenez une clef !\n");
                inventory.add_key();
                Box::new(TableState)
            }
            "4" => {
                streamed_print("\nVous ignorez le vase.\n");
                Box::new(TableState)
            }
            _ => {
                streamed_print("\nChoix invalide.\n");
                Box::new(PotState)
            }
        }
    }
}

impl State for CarpetState {
    fn message(&self, _: &Inventory) -> &str {
        "\nC'est un beau tapis dîtes-donc.\n\n1. S'essuyer les pieds sur le tapis\n2. Soulever le tapis\n3. Ignorer le tapis\n"
    }

    fn next(&self, choice: &str, inventory: &mut Inventory) -> Box<dyn State> {
        match choice {
            "1" => {
                streamed_print("\nVous vous essuyez les pieds sur ce tapis qui n'est définitivement pas un paillasson. Où est donc passé votre respect ?\n");
                Box::new(HallState)
            }
            "2" => {
                if !inventory.has_sword {
                    streamed_print("\nVous soulevez le tapis. Il y a un escalier qui mène à un piédestal muni d'une épée.\nBravo, vous êtes maintenant équipé d'une arme ! Sus aux streu-mons !\n");
                    inventory.add_sword();
                } else {
                    streamed_print("\nVous soulevez le tapis. Il y a un escalier qui mène à un piédestal. Vous avez déjà récupéré l'épée ici. Bon vent !\n");
                }
                Box::new(HallState)
            }
            "3" => {
                streamed_print("\nVous ignorez le tapis.\n");
                Box::new(HallState)
            }
            _ => {
                streamed_print("\nChoix invalide.\n");
                Box::new(CarpetState)
            }
        }
        
    }
}

impl State for HearthState {
    fn message(&self, _: &Inventory) -> &str {
        "\nVous inspectez en détail la cheminée, il y a trois cercles de pierres numérotés de 1 à 3 que l'on peut enfoncer une fois chacun. Il faut sûrement les activer dans un ordre précis... Entrez une combinaison :\n"
    }

    fn next(&self, choice: &str, _: &mut Inventory) -> Box<dyn State> {
        match choice {
            "123" => {
                streamed_print("\nRien ne se passe...\n");
                Box::new(HearthState)
            }
            "132" => {
                streamed_print("\nRien ne se passe...\n");
                Box::new(HearthState)
            }
            "213" => {
                streamed_print("\nLa cheminée se met à trembler et une trappe de pierre s'ouvre devant vous.\nUn ours bleu gigantesque en sort. Il vous massacre sans pitié.\n");
                Box::new(DeadState)
            }
            "231" => {
                streamed_print("\nRien ne se passe...\n");
                Box::new(HearthState)
            }
            "312" => {
                streamed_print("\nRien ne se passe...\n");
                Box::new(HearthState)
            }
            "321" => {
                streamed_print("\nRien ne se passe...\n");
                Box::new(HearthState)
            }
            "retour" => {
                streamed_print("\nVous ignorez la cheminée.\n");
                Box::new(HallState)
            }
            _ => {
                streamed_print("\nChoix invalide. Entrez une combinaison des 3 chiffres 1, 2, 3 sans les répéter. Ou 'retour' pour ignorer cette énigme.\n");
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
                    streamed_print("\nVous tournez la poignée. La porte ne s'ouvre pas, elle semble vérouillée... Vous faîtes demi-tour.\n");
                    Box::new(HallState)
                } else {
                    streamed_print("\nVous tournez la poignée. La porte ne s'ouvre pas, elle semble vérouillée...\nMmmh, la clé que vous avez récupérée entre dans la serrure. Vous dévérouillez la porte et accédez à une nouvelle pièce !\n");
                    Box::new(FinalBossState)
                }
            }
            "2" => {
                streamed_print("\nVous faites demi-tour.\n");
                Box::new(HallState)
            }
            _ => {
                streamed_print("\nChoix invalide.\n");
                Box::new(DoorState)
            }
        }
    }
}

impl State for FinalBossState {
    fn message(&self, _: &Inventory) -> &str {
        "\nVous arrivez dans une pièce sinistre, une mygale géante tombe du plafond. Quelle monstruosité ! Le combat est inévitable, appuyez sur entrée pour l'affronter !\n"
    }

    fn next(&self, _: &str, inventory: &mut Inventory) -> Box<dyn State> {
        if inventory.has_sword {
            streamed_print("\nAprès un combat sans merci, vous terrassez l'arachnide à l'aide de votre épée d'argent ! Bravo !\n");
            Box::new(EndState)
        } else {
            streamed_print("\nVous n'avez rien pour vous défendre. La mygale vous dévore membre après membre...\n");
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
        "\n--- Fin de l'aventure ---\n"
    }

    fn next(&self, _: &str, _: &mut Inventory) ->  Box<dyn State> {
        Box::new(EndState)
    }

    fn is_terminal(&self) -> bool {
        true
    }
}