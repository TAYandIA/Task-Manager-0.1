use std::io;
use ansi_term::*;

fn main() {

	let mut notes : Vec<String> = vec![];

	loop {

		print!("\x1b[2J\x1b[1;1H");
		
		println!("{}", Colour::Blue.paint("Biblothèque de Notes"));
		
		println!("Tapez Une commande : Regarder Notes (R), Ajouter une Note (A), Supprimer une note (S), Quitter (Q)");
		let input = demander_input();
		
	    print!("\x1b[2J\x1b[1;1H");
	    
	  	let input : &char = &mut input.trim().parse().expect("REASON");
		
	  	match input {
	  		'Q' => {
	  			return
	  		}
	  		'A' => {
	  			ajouter_list(&mut notes);
	  		}

	  		'R' => {
	  			regarder_list(&notes);
	  			demander_input();
	  		}
	  		
	  		'S' => {
	  			supprimer_list(&mut notes);
	  		}

	  		_ => {
	  			();
	  		}
	  	}
	  	
		
	}
    
	            
}

fn demander_input() -> String {
	let mut input = String::new();
	io::stdin().read_line(&mut input).expect("Erreur de lecture");
	input
}

fn regarder_list(list : &[String] ) {
	if !list.is_empty() {
		for (index,task) in list.iter().enumerate() {
			println!("{} - {}", index+1,Colour::Blue.paint(task));
		}			
	} else { 
		println!("{}", Colour::Yellow.paint("Liste Vide !!"));
	}
}

fn ajouter_list(list : &mut Vec<String>) {
	println!("{}", Colour::Green.paint("Tapez votre message :"));
	let  input = demander_input();
	if !input.trim().is_empty() {
		list.push(input.trim().to_string());		
	} else {
		println!("Note vide, veuillez réessayer");
	}	
}


fn supprimer_list(list : &mut Vec<String>) {
	
	regarder_list(list);

	println!("{}", Colour::Green.paint("Tapez le numéro correspondant à votre message :"));
	
	let input : usize  = demander_input().trim().parse().expect("Idk");

	if input-1 < list.len() {
		list.remove(input-1);
	}
	
}
