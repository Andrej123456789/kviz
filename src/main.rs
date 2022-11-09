#![allow(unused_must_use)]

use clearscreen;

use crossterm::{style, QueueableCommand};
use std::io::{stdin, stdout, Stdout, Write};

fn quiz(stdout: &mut Stdout) {
    let questions: [String; 11] = [
        String::from("Što je štednja [A - čuvanje materijalnih dobara od potrošnje u određenoj situaciji; B - neumjerena potrošnja novca; C - skakanje iz zrakoplova s instruktorom]: "), // A
        String::from("Kada se obilježava Svjetski dan štednje [A - 30.10; B - 31.10; C - 1.11: "), // B
        String::from("Tko je proglasio, započeo obilježavanje Svjetskog dana štednje [A - Filippo Ravizza; B - Mate Matić; C - Victor Ninov]: "), // A
        String::from("Koje godine je održan Prvi međunarodni kongres štedioničara [A - 1922. godine; B - 1950. godine; C - 1924. godine]: "), // C
        String::from("Navedi jedan simbol štednje [A - jabuka; B - kasica prasica; C - maslina]: "), // B
        String::from("Koji je njemaćki naziv za 'Happy Save', austrijsku maskotu Svjetskog dana štednje [A - Sparefroh; B - ne znam; C - linux tux]: "), // A
        String::from("U kojem vremenskom periodu se najaktivnije slavio prijašnje naveden dan [A - 2008. - 2010.; B - 1924. - 1926.; C - 1955. - 1970..]: "), // C
        String::from("Dopuni sljedeću rečenicu: 'Austrija je objavljivala časopis za mlade štediše u nakladi od ________'. [A - 400 000; B - 300 000; C - 450 000: "), // A
        String::from("Navedi barem 3 načina kako možemo štedjeti [A - bacanje smeća na ulici, paljenje plastike na ulici, krađa automobila; B - vikati; C - reciklirati, kupovati proizvode na akciji, kupovati onoliko koliko nam treba u nekom određenom trenutku]: "), // C
        String::from("Od navedenih država, koji stanovnici najviše štede (odaberi jedno slovo) [A - Hrvatska; B - Slovaci; C - Česi]: "), // B
        String::from("Navedite razloge zašto je dobro štediti [A - pregledi na TikTok-u; B - štednja novca, ekološki razlozi; C - upoznamo Victor Ninov-a]: "), // B
    ];

    let answears: [char; 11] = ['a', 'b', 'a', 'c', 'b', 'a', 'c', 'a', 'c', 'b', 'b'];

    let mut i: usize = 0;
    let mut bodovi: u32 = 0;

    let mut input: String = String::new();

    println!("Odgovor na sljedeća pitanja može biti slova 'A', 'a', 'B', 'b', 'C' ili 'c'.");
    println!("Svako pitanje ima jedan odgovor te donosi jedan bod osim ako nije drukčije navedeno u zadatku.");
    println!("Kao odgovor promatra se samo prvo slovo unošenog teksta.");
    println!("Sretno!");
    println!(" ");

    while i < questions.len() {
        print!("{}", questions[i]);
        std::io::Write::flush(&mut std::io::stdout()).expect("Failed to flush stdout!");
        std::io::stdout().flush().expect("Failed to flush stdout!");

        stdin()
            .read_line(&mut input)
            .expect("Failed to read user input!");

        input = input.to_lowercase().trim_end().to_string();

        if input.chars().nth(0).unwrap() == answears[i]
            || input.to_lowercase().chars().nth(0).unwrap() == answears[i]
        {
            stdout.queue(style::SetForegroundColor(style::Color::Green));
            println!("Odgovor {} je točan!", input.chars().nth(0).unwrap());
            stdout.queue(style::SetForegroundColor(style::Color::Reset));

            bodovi += 1;
        } else {
            stdout.queue(style::SetForegroundColor(style::Color::Red));
            println!(
                "Odgovor {} je netočan! Točan odgovor je {}!",
                input.chars().nth(0).unwrap(), answears[i]
            );
            stdout.queue(style::SetForegroundColor(style::Color::Reset));
        }

        input = String::from("");
        i += 1;
    }

    println!("Broj bodova: {}/11.", bodovi);
}

fn main() {
    clearscreen::clear().expect("Failed to clear the screen!");

    let mut stdout = stdout();
    stdout.queue(style::SetForegroundColor(style::Color::Rgb {
        r: (3),
        g: (252),
        b: (252),
    }));

    println!("Dobrodošli u kviz");
    println!("Public Domain - 2022 - Andrej Bartulin");

    stdout.queue(style::SetForegroundColor(style::Color::Reset));
    println!(" ");

    quiz(&mut stdout)
}

/*
 * Napravite novi cargo projekt, stavite ovaj kod u main.rs i pokrenite u release modu jer je brži
 * Btw, ovo je sadržaj Cargo.toml datoteke: 
    [package]
    name = "kviz"
    version = "0.1.0"
    edition = "2021"

    # See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

    [dependencies]
    clearscreen = "1.0.10"
    crossterm = "0.25.0"
*/
