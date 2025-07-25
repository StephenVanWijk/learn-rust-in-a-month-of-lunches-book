

pub fn hello() {
    println!("Hello from chapter nine!");
}

pub fn chapter_nine_paragraph_913_2() {
    // This function is a placeholder for the paragraph 913.2 content.
    // You can add your specific logic or content here.
    let mut big_vec: Vec<i32> = vec![6; 1000];
    big_vec.push(5);

    let mut iterator: std::iter::Rev<std::slice::Iter<'_, i32>> = big_vec.iter().rev();
    assert_eq!(iterator.next(), Some(&5));
    assert_eq!(iterator.next(), Some(&6));
}

pub fn chapter_nine_paragraph_913_3(){
    let mut big_vec = vec![6; 10];
    big_vec.push(5);
    println!("{:?}", big_vec.iter().rev().any(|&number| number == 5)); 
}

pub fn chapter_nine_paragraph_913_4() {
    // This function is a placeholder for the paragraph 913.4 content.
    let mut big_vec = vec![6; 10];
    big_vec.push(5);

    let mut num_loops = 0;
    // 20250723 1340CET SDvW testing reverted iterator.
    let mut big_iter: std::iter::Rev<std::vec::IntoIter<i32>> = big_vec.into_iter().rev();
    loop {
        num_loops +=1;
        println!("{:?}", num_loops);
        if big_iter.next() == Some(5) {
            break;
        }
    }
    if (assert_eq!(num_loops, 1)) == () {
        println!("The iterator found the number 5 in one loop.");
    } else {
        println!("The iterator did not find the number 5 in one loop.");
    }   
    println!("Number of loops: {num_loops}");
    // Amended
}

pub fn chapter_nine_paragraph_914_1(){
    // 20250723 1413CET SDvW It just zips a flipping iterator over the same same array ["even", "odd"], 
    // with the range (0..=5).
    let even_odd_iter = ["even", "odd"].into_iter().cycle();

    let even_odd_vec: Vec<(i32, &str)> = (0..=5)
        .zip(even_odd_iter)
        .collect();
    println!("{:?}", even_odd_vec);
}

pub fn chapter_nine_paragraph_914_2() {
    let ten_chars: Vec<char> = ('a'..).take(10).collect();
    let skip_then_ten_chars: Vec<char> = ('a'..).skip(1300).take(10).collect();

    println!("{ten_chars:?}");
    println!("{skip_then_ten_chars:?}");
}

pub fn chapter_nine_paragraph_914_3() {
    let some_numbers: Vec<i32> = vec![9, 6, 9, 10, 11, 3294587];

    println!("{}", some_numbers
        .iter()
        .fold(0, |total_so_far, next_number| total_so_far + next_number)
    );
}

#[derive(Debug)]
struct CombinedEvents {
    num_of_events: u32,
    data: Vec<String>,
}

pub fn chapter_nine_paragraph_914_4() {
    let events = [
        "Went to grocery store",
        "Came home",
        "Fed cat",
        "Fed cat again",
    ];

    let empty_events = CombinedEvents {
        num_of_events: 0,
        data: vec![]
    };

    let combined_events: CombinedEvents =
        events
        .iter()
        .fold(empty_events, |mut total_events: CombinedEvents, next_event: &&str| {
            total_events.num_of_events += 1;
            total_events.data.push(next_event.to_string());
            total_events
        });
    println!("{combined_events:?}");
}

pub fn chapter_nine_paragraph_914_8(){
    let num_vec = vec![1, 2, 3, 4, 5, 6, 7];

    let chunk_size = 0;

    if (assert!(chunk_size != 0, "Chunk size must be non-zero, 20250725 0905CET SDvW.") == ()) {
        for chunk in num_vec.chunks(3) {
            println!("{:?}", chunk);
        }
        println!();
        for window in num_vec.windows(3) {
            println!("{:?}", window);
        }
    }
}