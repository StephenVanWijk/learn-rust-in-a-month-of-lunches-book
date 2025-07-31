use std::sync::Mutex;
use std::rc::Rc;
use std::sync::RwLock;
use std::borrow::Cow;

#[derive(Debug)]
struct ErrorInfo {
    error: LocalError,
    message: String,
}

#[derive(Debug)]
enum LocalError {
    TooBig,
    TooSmall,
}

fn generate_message(
    message: &'static str,
    error_info: Option<ErrorInfo>,
    ) -> Cow<'static, str> {
        match error_info {
            None => message.into(),
            Some(info) => format!("{}: {:?}", message, info).into(),
        }
    }

pub fn chapter_eleven_paragraph_1104_1(){
    let msg1: Cow<'static, str> = generate_message(
        "Everything is fine",
        None
        );

    let msg2: Cow<'static, str> = generate_message(
        "Got an error",
        Some(ErrorInfo {
            error: LocalError::TooBig,
            message: "It was too big.".to_string(),}),
        );

        for msg in [msg1, msg2] {
            match msg {
            Cow::Borrowed(msg) => {
                println!("Borrowed, didn't need an allocation:\n {:?}", msg)
            }
            Cow::Owned(msg) => {
                println!("Owned, because we needed an allocation:\n {:?}", msg)
            }
        }
    }
}

struct UserA {
    name: Cow<'static, str>,
}

pub fn chapter_eleven_paragraph_1104_2(){

    let user_name: &'static str = "User1";
    let other_user_name: String = "User10".to_string();

    let user1: UserA = UserA {
        name: user_name.into(),
    };

    let user2: UserA = UserA {
        name: other_user_name.into(),
    };

    for name in [user1.name, user2.name] {
        match name {
            Cow::Borrowed(n) => {
                println!("Borrowed name, didn't need an allocation:\n{}", n)
            }
            Cow::Owned(n) => {
                println!("Owned name because we needed an allocation:\n{}", n)
            }
        }
    }
}

#[derive(Debug)]
struct CityA {
    name: Rc<String>,
    population: u32,
    city_history: Rc<String>,
}

#[derive(Debug)]
struct CityData {
    names: Vec<Rc<String>>,
    histories: Vec<Rc<String>>,
}

fn takes_a_string(input: Rc<String>) {
    println!("It is: {}", input)
}

pub fn chapter_eleven_paragraph_1152_2() {
    let calgary_name: Rc<String> = Rc::new("Calgary".to_string());
    // 20250728 1223CET SDvW
    // let calgary_name= "Calgary".to_string();
    let calgary_history: Rc<String> = Rc::new("Calgary began as a fort called Fort Calgary that...".to_string());

    let calgary: CityA = CityA {
        name: Rc::clone(&calgary_name),
        population: 1_200_000,
        city_history: Rc::clone(&calgary_history),
    };

    let canada_cities: CityData = CityData {
        names: vec![Rc::clone(&calgary_name)],
        histories: vec![Rc::clone(&calgary_history)],
    };

    println!("Calgary's history is: {}", calgary.city_history);
    println!("{}", Rc::strong_count(&calgary.city_history));
}


pub fn chapter_eleven_paragraph_1152_3() {
    let user_name: Rc<String> = Rc::new(String::from("User MacUserSon"));

    takes_a_string(Rc::clone(&user_name));
    takes_a_string(Rc::clone(&user_name));
}

fn takes_a_string_borrowed(input: &String) {
    println!("It is: {}", input)
}

pub fn chapter_eleven_paragraph_1152_4() {
    let user_name: String = String::from("User MacUserSon");

    takes_a_string_borrowed(&user_name);
    takes_a_string_borrowed(&user_name);
}       