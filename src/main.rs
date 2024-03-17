use rand::Rng; // für zufällige Auswahl
use std::collections::HashMap;
use std::io;

fn main() {
    let mut gewinnbedingungen: HashMap<&str, Vec<&str>> = HashMap::new();
    gewinnbedingungen.insert("Schere", vec!["Papier", "Echse"]);
    gewinnbedingungen.insert("Stein", vec!["Schere", "Echse"]);
    gewinnbedingungen.insert("Papier", vec!["Stein", "Spock"]);
    gewinnbedingungen.insert("Echse", vec!["Spock", "Papier"]);
    gewinnbedingungen.insert("Spock", vec!["Schere", "Stein"]);

    let auswahlen = ["Schere", "Stein", "Papier", "Echse", "Spock"];
    let mut rng = rand::thread_rng();

    loop {
        println!("Wähle: Schere, Stein, Papier, Echse, Spock (oder 'ende' zum Beenden):");
        let mut benutzer_auswahl = String::new();
        io::stdin().read_line(&mut benutzer_auswahl).expect("Fehler beim Lesen der Eingabe");
        let benutzer_auswahl = benutzer_auswahl.trim();

        if benutzer_auswahl == "ende" {
            break;
        }

        if !auswahlen.contains(&benutzer_auswahl) {
            println!("Ungültige Auswahl.");
            continue;
        }

        let computer_auswahl = auswahlen[rng.gen_range(0..auswahlen.len())];
        println!("Computer wählt: {}", computer_auswahl);

        match gewinnbedingungen.get(benutzer_auswahl) {
            Some(gewinnt_gegen) if gewinnt_gegen.contains(&computer_auswahl) => {
                println!("{} schlägt {}. Du gewinnst!", benutzer_auswahl, computer_auswahl);
            }
            _ if benutzer_auswahl == computer_auswahl => {
                println!("Unentschieden!");
            }
            _ => {
                println!("{} schlägt {}. Du verlierst.", computer_auswahl, benutzer_auswahl);
            }
        }
    }
}

