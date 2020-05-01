pub fn q1() {

    let s = String::from("stressed");
    let t = s.chars().rev().collect::<String>();
    println!("{}", t);

}

pub fn q2() {

    let s = String::from("パタトクカシーー");
    let t = s.chars();
    let mut cnt =0;

    for i in t {
        if cnt % 2 != 0 {
            print!("{}", i);
        }
        cnt = cnt + 1;
    }
    println!();

}

pub fn q3() {

    let s1 = String::from("パトカー");
    let s2 = String::from("タクシー");
    let t1 = s1.chars();
    let t2 = s2.chars();

    for (i, j) in t1.zip(t2) {
        print!("{}", i);
        print!("{}", j);
    }
    println!();

}
