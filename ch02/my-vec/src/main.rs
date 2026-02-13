macro_rules! my_vec {
    () => [
        Vec::new()
    ];
    (make an empty vec) => (
        Vec::new()
    );
    {$x:expr} => {
        {
            let mut v = Vec::new();
            v.push($x);
            v
        }
    };
    [$($x:expr),+] => {
        {
            let mut v = Vec::new();
            $(
                v.push($x);
            )+
            v
        }
    }
}

fn main() {
    let empty: Vec<i32> = my_vec![];
    println!("{:?}", empty);
    let also_empty: Vec<i32> = my_vec!(make an empty vec);
    println!("{:?}", also_empty);
    let three_numbers = my_vec!(1, 2, 3);
    println!("{:?}", three_numbers);
}
