use std::sync::Mutex;
use std::rc::Rc;
use std::sync::RwLock;
use std::borrow::Cow;
#[derive(Debug)]
struct City<'a> { // ①
    name: &'a str, // ②
    date_founded: u32,
}

pub fn chapter_ten_paragraph_1022_4(){
    let city_names: Vec<String> = vec!["Ichinomiya".to_string(), "Kurume".to_string()];

    let my_city = City {
        name: &city_names[0],
        date_founded: 1921,
    };

    println!("{} was founded in {}", my_city.name, my_city.date_founded);
}

struct Adventurer<'a> {
    name: &'a str,
    hit_points: u32,
}

impl Adventurer<'_>{
    fn take_damage(&mut self) {
        self.hit_points -= 20;
        println!("{} has {} hit points left!", self.name, self.hit_points);
    }
}

pub fn chapter_ten_paragraph_1022_10(){
    let mut adventurer = Adventurer {
        name: "Billy",
        hit_points: 100_000,
    };
    println!("{} has {} hit points.", adventurer.name, adventurer.hit_points);
    adventurer.take_damage();
}

impl std::fmt::Display for Adventurer<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} has {} hit points.", self.name, self.hit_points)
    }
}

pub fn chapter_ten_paragraph_1022_12(){
    let mut billy: Adventurer<'_> = Adventurer {
    name: "Billy",
    hit_points: 100_000,
    };
    println!("{}", billy);
    billy.take_damage();
}

struct PhoneModel {
    company_name: String,
    model_name: String,
    screen_size: f32,
    memory: usize,
    date_issued: u32,
    on_sale: bool,
}

// impl PhoneModel { // ①
//     fn method_one(&self) {}
//     fn method_two(&self) {}
// }

impl std::fmt::Display for PhoneModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} with {} inch screen and {} memory, on sale {} and issued in {}.", self.company_name, self.model_name, self.screen_size, self.memory, self.on_sale, self.date_issued)
    }
} 
  
pub fn chapter_nine_paragraph_103_1() {
    let super_phone_3000 = PhoneModel {
        company_name: "YY Electronics".to_string(),
        model_name: "Super Phone 3000".to_string(),
        screen_size: 7.5,
        memory: 4_000_000,
        date_issued: 2020,
        on_sale: true,
    };
   
    // println!("{} was issued in {}.", super_phone_3000, super_phone_3000.date_issued);
    println!("{}", super_phone_3000);
}

use std::cell::Cell;

#[derive(Debug)]
struct PhoneModelA {
    company_name: String,
    model_name: String,
    screen_size: f32,
    memory: usize,
    date_issued: u32,
    on_sale: Cell<bool>,
}

impl PhoneModelA {
    fn make_not_on_sale(&self, on_sale: bool) {
        self.on_sale.set(on_sale);
    }
}

impl std::fmt::Display for PhoneModelA {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let on_sale: &str = if self.on_sale.get() { "yes" } else { "no" };
        write!(f, "{} {} with {} inch screen and {} memory, on sale {} and issued in {}.", self.company_name, self.model_name, self.screen_size, self.memory, on_sale, self.date_issued)
    }
} 

pub fn chapter_ten_paragraph_1031_1() {
    let super_phone_3000 = PhoneModelA {
        company_name: "YY Electronics".to_string(),
        model_name: "Super Phone 3000".to_string(),
        screen_size: 7.5,
        memory: 4_000_000,
        date_issued: 2020,
        on_sale: Cell::new(true),
    };
    super_phone_3000.make_not_on_sale(false);
    println!("{}", super_phone_3000);
}

use std::cell::RefCell;
use std::sync::MutexGuard;
use std::sync::RwLockReadGuard;
use std::sync::RwLockWriteGuard;

#[derive(Debug)]
struct User {
    id: u32,
    year_registered: u32,
    username: String,
    active: RefCell<bool>,
}

impl User  {
    fn change_active_status(&self, status: bool) {
        if self.active.try_borrow_mut().is_ok() {
            *self.active.borrow_mut() = status;
        }
        else{     
            println!("Failed to borrow active status.");
            return;
        }
    } 
    
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "User {} (ID: {}) registered in {} is active: {}", self.username, self.id, self.year_registered, self.active.borrow())
    }
}

pub fn chapter_ten_paragraph_1031_2() {
    let user_1 = User {
        id: 1,
        year_registered: 2020,
        username: "User 1".to_string(),
        active: RefCell::new(true),
    };

    println!("{}", user_1);
    let referentie: std::cell::Ref<'_, bool> = user_1.active.borrow();
    println!("User 1 active status: {}", referentie);
    user_1.change_active_status(false);
    println!("{}", user_1);
}

pub fn chapter_ten_paragraph_1033_1() {
    let my_mutex: Mutex<i32> = Mutex::new(5);
    let mut mutex_changer:MutexGuard<'_, i32> = my_mutex.lock().unwrap();
    println!("{:?}", my_mutex);
    println!("{:?}", mutex_changer);
    // *mutex_changer = 6;
    println!("{:?}", mutex_changer);
}

pub fn chapter_ten_paragraph_1033_3() {
    let my_mutex: Mutex<i32> = Mutex::new(5);
    let mut mutex_changer:MutexGuard<'_, i32> = my_mutex.lock().unwrap();
    *mutex_changer = 6;
    drop(mutex_changer);

    println!("{:?}", my_mutex);
}

pub fn chapter_ten_paragraph_1033_5() {

    let my_mutex: Mutex<i32> = Mutex::new(5);
    let mut mutex_changer: MutexGuard<'_, i32> = my_mutex.lock().unwrap();
    let mut other_mutex_changer: Result<MutexGuard<'_, i32>, std::sync::TryLockError<MutexGuard<'_, i32>>> = 
    my_mutex.try_lock();

    dbg!(&mutex_changer);
    dbg!(&other_mutex_changer);
    dbg!(&my_mutex.try_lock());
    
    if let Ok(value) = my_mutex.try_lock() {
        println!("The MutexGuard has: {value}")
    } else {
        println!("Didn't get the lock")
    }
}

pub fn chapter_ten_paragraph_1034_1(){
    let my_rwlock: RwLock<i32> = RwLock::new(5);
    let read1: RwLockReadGuard<'_, i32> = my_rwlock.read().unwrap();
    let read2: RwLockReadGuard<'_, i32> = my_rwlock.read().unwrap();
    println!("read1:{:?}, read2:{:?}", read1, read2);
    let write1: RwLockWriteGuard<'_, i32> = my_rwlock.write().unwrap();
}


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