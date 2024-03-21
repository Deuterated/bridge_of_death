use rand::seq::SliceRandom;
use std::io::{self, Write};
use std::process;

// Define the Question struct
#[derive(Debug, Clone)]
struct Question {
    prompt: String,
    correct_answer: String,
}

impl Question {
    // Constructor for Question
    fn new(prompt: &str, correct_answer: &str) -> Question {
        Question {
            prompt: prompt.to_string(),
            correct_answer: correct_answer.to_string(),
        }
    }

    // Method to present the prompt to the user and compare the response
    fn ask_question(&self) {
            println!("{}", self.prompt);

        print!("");
        io::stdout().flush().unwrap();

        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Failed to read line");

        let user_answer = user_input.trim();

        // Check if the user's input ends with a question mark
        if user_answer.ends_with('?') {
            println!("I don't know that. AAAUGH!!");
            println!("*The bridge keeper has been cast into the gorge of eternal peril and has died. Congratulations, murderer.*");
	    process::exit(0);
        } else {
            // Check if the user's answer is correct
	    if self.correct_answer.is_empty() {
		println!("");
            } else if user_answer == self.correct_answer {
                println!("Alright, off you go!");
            } else {
                println!("*You have chosen poorly; an unseen force lifts you and casts you into the pit of eternal peril.*");
            }
        }
    }
}

fn main() {
    // Define two fixed questions with any answer considered correct
    let fixed_questions = [
        Question::new("What is your name?", ""),
        Question::new("What is your quest?", ""),
    ];

    println!("STOP!");
    println!("Who would cross the Bridge of Death must answer me these questions three, ere the other side ye see.");

    // Ask the fixed questions
    for question in &fixed_questions {
        question.ask_question();
    }

    // Define a list of questions
    let questions = [
        Question::new("What is 2 + 2?", "4"),
        Question::new("What is the capital of Assyria?", "assur"),
        Question::new("What is the airspeed velocity of an unladen swallow?", "32.4kph"),
	Question::new("What is your favorite color?", ""),
    ];

    // Use the rand crate for random number generation
    let mut rng = rand::thread_rng();

    // Shuffle the questions array
    let mut shuffled_questions = questions.to_vec();
    shuffled_questions.shuffle(&mut rng);

    // Select a random question
    if let Some(random_question) = shuffled_questions.choose(&mut rng) {
        // Ask the question and check the answer
	random_question.ask_question();
    } else {
        println!("No questions available.");
    }
}

