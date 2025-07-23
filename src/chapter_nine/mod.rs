

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
    let mut big_iter = big_vec.into_iter().rev();
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
}