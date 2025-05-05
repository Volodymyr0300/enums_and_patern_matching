fn main() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    
    struct QuitMessage; // unit struct
    struct MoveMessage {
        x: i32,
        y: i32,
    }

    struct WriteMessage(String); // tuple struct
    struct ChangeColorMessage(i32, i32, i32); // tuple struct

    
    impl Message {
        fn call(&self) {
        // method body would be defined here
        }
        }
        let m = Message::Write(String::from("hello"));
        m.call();
    
    enum Option<T> {
        Some(T),
        None,
    }

    /* 
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
    */

    /* 
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    let sum = x + y;
    */

    

}
