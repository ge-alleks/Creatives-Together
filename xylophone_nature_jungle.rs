fn main() {
    println!("Welcome to 'Creatives Together'!");

    // 1
    let greeting = "Hello everyone!";
    println!("{}", greeting);

    // 2
    let mut user_info = HashMap::new();
    user_info.insert("name", "Jane");
    user_info.insert("age", "25");
    user_info.insert("location", "New York City");

    // 3
    for (key, value) in &user_info {
        println!("{}: {}", key, value);
    }

    // 4
    let mut user_projects: Vec<String> = Vec::new();
    user_projects.push(String::from("The Chair"));
    user_projects.push(String::from("The Desk"));

    // 5
    for project in &user_projects {
        println!("{}", project);
    }

    // 6
    let mut post = String::new();
    post.push_str("Hi, I'm Jane.\n");
    post.push_str("I'm working on a few projects.\n");
    post.push_str("One of them is called The Chair.\n");
    println!("{}", post);

    // 7
    let activity = "watch a movie";
    let user_activity = format!(
        "Let's all {} to finish the day. Challenges await!",
        activity
    );
    println!("{}", user_activity);

    // 8
    let mut user_suggestions = Vec::new();
    user_suggestions.push(String::from("Try something new"));
    user_suggestions.push(String::from("Make something"));

    // 9
    for suggestion in &user_suggestions {
        println!("{}", suggestion);
    }

    // 10
    println!("Thanks for joining us!");

    // 11 
    let mut user_contributions: HashMap<String, i32> = HashMap::new();
    user_contributions.insert(String::from("Painting"), 5);
    user_contributions.insert(String::from("Sculpture"), 2);

    // 12 
    for (contribution, count) in &user_contributions {
        println!("I have contributed {} {} times.", contribution, count);
    }

    // 13
    let total_contributions: i32 = user_contributions.values().sum();
    println!("Total contributions I have made so far: {}", total_contributions);

    // 14
    let mut user_goals = HashMap::new();
    user_goals.insert("Create new art", true);
    user_goals.insert("Publish a book", false);

    // 15
    for (goal, achieved) in &user_goals {
        if *achieved {
            println!("{} has been achieved!", goal);
        } else {
            println!("{} is still in progress.", goal);
        }
    }

    // 16
    let mut user_ideas = HashMap::new();
    user_ideas.insert("Sculpture", "Make a giant wristwatch");
    user_ideas.insert("Book", "Write a novel about a family of rabbits");

    // 17
    for (genre, idea) in &user_ideas {
        println!("I'm thinking of {} a {}.", idea, genre);
    }

    // 18
    let mut user_ideas: Vec<String> = Vec::new();
    user_ideas.push(String::from("Design new illustrations"));
    user_ideas.push(String::from("Create a music album"));

    // 19
    for idea in &user_ideas {
        println!("{}", idea);
    }

    // 20
    println!("Let's keep creating together!");
}