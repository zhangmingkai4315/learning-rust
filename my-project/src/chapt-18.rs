use std::option::Option::Some;

fn main(){

    let a = 5;
    let (x, y, z) = (1, 2, 3);
    println!("{} {} {} {}", a, x, y, z);
    // let (x, y ) =  (1, 2, 3);
    // error : expected a tuple with 3 elements,
    // found one with 2 elements
    let (x, y, _ ) = (5, 6, 7);
    println!("{} {} {} {}", a, x, y, z);



    #[derive(Debug)]
    enum Status{
        Init,
        Starting,
        Started,
        Stopping,
        Stopped
    }

    let s = Status::Starting;
    match s {
        Status::Init | Status::Starting | Status::Started| Status::Stopping =>
        println!("not ready to start"),  // not ready to start
        Status::Stopped=> println!("ready to start")
    }
    let a = 10;
    match a {
        0..=10 => println!("value belong to [0,10]"),  // value belong to [0,10]
        _ => println!("value great than 10 or less than 0")
    }



    fn get_status(v: i32)->Result<Status, String>{
        if v == 10{
            Ok(Status::Init)
        }else{
            Err(String::from("erro"))
        }
    }
    if let Ok(s) = get_status(10){
        println!("{:?}", s)  // Init
    }


    let mut s = vec![1,2,3,4,5];
    while let Some(v) = s.pop() {
        println!("pop => {}", v)
    }
    // pop => 5
    // pop => 4
    // pop => 3
    // pop => 2
    // pop => 1


    fn print_coordinates(&(x, y ): &(i32, i32)){
        println!("{} {}", x, y); //3 5
    }

    let point = (3,5);
    print_coordinates(&point);

    enum Message{
        Quit,
        Move{x: i32, y: i32},
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let message = Message::ChangeColor(10,20,30);

    match message {
        Message::Quit => println!("quit"),
        Message::Move {x, ..} => println!("move to {} y is ignored", x),
        Message::Write(s) => println!("write {}", s),
        Message::ChangeColor(x,y,z) =>
            println!("change color to rgb({},{},{})", x,y,z)
    }

    let s = Some(String::from("hello"));

    // if let Some(_x) = s{
    //     println!("{}", _x)
    // }

    if let Some(_) = s{
        println!(" not bind to value, no ownership change")
    }

    println!("{:?}", s);


    let v = Some(10);
    match v {
        Some(v) if v >10 => println!(" v is great than 10"),
        Some(v) if v > 0 => println!(" v is great than 0"),
        //  v is great than 0
        Some(v) => println!(" v is valid"),
        None => println!(" v is invalid"),
    }

    let message = Message::Move {x: 10, y: 20};

    match message {
        Message::Move {x: x_extra @ 0..=9, ..} =>
            print!("x is less than 10, {}", x_extra),
        Message::Move {x: 10..=99, ..}=> print!("x is less 100"),
        Message::Move {x, ..}=> print!("x = {}", x),
        _ => println!("other message"),
    }



}

