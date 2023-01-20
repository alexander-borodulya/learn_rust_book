pub fn run () {
    println!("Chapter 006: Enums and Pattern Matching");
    section_6_1();
    section_6_2();
    section_6_3();
}

fn section_6_1 () {
    println!("Chapter 6.1: Defining an Enum");
    
    // 1
    #[derive(Debug)]
    enum IpAddrKind {
        V4,
        V6,
    }

    let v4 = IpAddrKind::V4;
    let v6 = IpAddrKind::V6;

    println!("v4: {:?}", v4);
    println!("v6: {:?}", v6);

    // 2
    println!("");

    #[derive(Debug)]
    struct IpAddr {
        kind: IpAddrKind,
        addr: String,
    }

    impl IpAddr {
        fn print(ip_addr: &IpAddr) {
            println!("IpAddr::print {{ kind: {:?}, addr: {:?} }}", ip_addr.kind, ip_addr.addr);
        }
    }

    let home = IpAddr {
        kind: IpAddrKind::V4,
        addr: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        addr: String::from("::1"),
    };

    println!("home: {:?}", home);
    println!("loopback: {:?}", loopback);
    
    IpAddr::print(&home);
    IpAddr::print(&loopback);

    // 3
    println!("");
    
    #[derive(Debug)]
    enum IpAddrConcise {
        V4(String),
        V6(String),
    }
    let v4 = IpAddrConcise::V4(String::from("127.0.0.1"));
    let v6 = IpAddrConcise::V6(String::from(":::1"));
    println!("IpAddrConcise: v4: {:?}, v6: {:?}", v4, v6);

    // 4
    println!("");
    
    #[derive(Debug)]
    enum IpAddr2 {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let v4 = IpAddr2::V4(127, 0, 0, 1);
    let v6 = IpAddr2::V6(String::from(":::1"));
    println!("IpAddr2: v4: {:?}, v6: {:?}", v4, v6);

    // 5
    println!("");
    
    #[derive(Debug)]
    struct IpAddrV4 {}
    
    #[derive(Debug)]
    struct IpAddrV6 {}
    
    #[derive(Debug)]
    enum IpAddr3 {
        V4(IpAddrV4),
        V6(IpAddrV6),
    }

    let v4 = IpAddr3::V4(IpAddrV4{});
    let v6 = IpAddr3::V6(IpAddrV6{});

    println!("IpAddr3: v4: {:?}, v6: {:?}", v4, v6);

    // 6
    
    {
        use crate::common::chapter_006::Message;
        println!("");
    
        let msg = Message::Quit;
        println!("msg: {:?}", msg);
        msg.print();
        
        let msg = Message::Move { x: 100, y: 200 };
        println!("msg: {:?}", msg);
        msg.print();
        
        let msg = Message::Write(String::from("enum message"));
        println!("msg: {:?}", msg);
        msg.print();
        
        let msg = Message::Color(255, 0, 128);
        println!("msg: {:?}", msg);
        msg.print();
    }

    // The Option enum
    println!("");
    
    {
        // Just Private Scope
        #[derive(Debug)]
        enum MyOption<T> {
            None,
            Some(T),
        }
        let mo1: MyOption<bool> = MyOption::None;
        let mo2 = MyOption::Some(false);
        println!("mo1: {:?}, mo2: {:?}", mo1, mo2);

    }
    let n = Some(10);
    let s = Some(String::from("a string"));
    let a: Option<i32> = None;

    println!("n: {:?}, s: {:?}, a: {:?}", n, s, a);

    if n.is_some() {
        println!("n: {:?}", n.unwrap());
    }

    if s.is_some() {
        println!("s: {:?}", s.unwrap());
    }

    if a.is_none() {
        println!("a: is_none: {:?}", a.is_none());
    }
}

fn section_6_2() {
    println!("Chapter 6.2: The match Control Flow Construct");

    // 1
    enum Coin {
        Peny,
        Nikel,
        Dime,
        Quater,
    }

    fn values_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Peny => 1,
            Coin::Nikel => 5,
            Coin::Dime => 10,
            Coin::Quater => 25,
        }
    }

    println!("Coin::Peny: {:?}", values_in_cents(Coin::Peny));
    println!("Coin::Nikel: {:?}", values_in_cents(Coin::Nikel));
    println!("Coin::Dime: {:?}", values_in_cents(Coin::Dime));
    println!("Coin::Quater: {:?}", values_in_cents(Coin::Quater));

    // 2. Patterns that binds to values
    println!("");
    
    #[derive(Debug)]
    enum GroupA {
        GA0,
        GA1,
        GA2,
    }

    #[derive(Debug)]
    enum GroupB {
        GB0,
        GB1,
        GB2,
    }

    #[derive(Debug)]
    enum ComplexGroup {
        CG0,
        CG1(GroupA),
        CG2(GroupB),
    }

    fn print_group(cg: ComplexGroup) {
        match cg {
            ComplexGroup::CG0 => println!("ComplexGroup::CG_0"),
            ComplexGroup::CG1(ga) => println!("ComplexGroup::CG_1: {:?}", ga),
            ComplexGroup::CG2(gb) => {
                println!("ComplexGroup::CG_2: {:?}", gb);
                // Subgrup match could be handled here
                match gb {
                    GroupB::GB0 => println!("    GroupB::GB0"),
                    GroupB::GB1 => println!("    GroupB::GB1"),
                    GroupB::GB2 => println!("    GroupB::GB2"),
                }
            },
        }
    }

    print_group(ComplexGroup::CG0);
    print_group(ComplexGroup::CG1(GroupA::GA0));
    print_group(ComplexGroup::CG1(GroupA::GA1));
    print_group(ComplexGroup::CG1(GroupA::GA2));
    print_group(ComplexGroup::CG2(GroupB::GB0));
    print_group(ComplexGroup::CG2(GroupB::GB1));
    print_group(ComplexGroup::CG2(GroupB::GB2));

    {
        #[derive(Debug)]
        enum GroupB {
            GB0,
            GB1,
            GB2,
        }
    
        #[derive(Debug)]
        enum ComplexGroup {
            CG0,
            CG2(GroupB),
        }
    
        fn print_group(cg: ComplexGroup) {
            match cg {
                ComplexGroup::CG0 => println!("ComplexGroup::CG_0"),
                ComplexGroup::CG2(gb) => {
                    println!("ComplexGroup::CG_2: {:?}", gb);
                    // Sub-group match could be handled here
                    match gb {
                        GroupB::GB0 => println!("    GroupB::GB0"),
                        GroupB::GB1 => println!("    GroupB::GB1"),
                        GroupB::GB2 => println!("    GroupB::GB2"),
                    }
                },
            }
        }
    
        print_group(ComplexGroup::CG0);
        print_group(ComplexGroup::CG2(GroupB::GB0));
        print_group(ComplexGroup::CG2(GroupB::GB1));
        print_group(ComplexGroup::CG2(GroupB::GB2));
    }

    // 3. Matching with Option<T>
    println!("");
    
    fn handle_option_t(ov: Option<i32>) {
        match ov {
            None => println!("handle None..."),
            Some(i) => println!("handle Some, i =  {}", i),
        }
    }

    handle_option_t(None);
    handle_option_t(Some(100));

    // 4. Catch All and the _ Placeholder
    println!("");
    
    fn handle_i32(i: i32) {
        match i {
            1 => println!("One, {}", i),
            10 => println!("Two, {}", i),
            all_the_rest_of_i32 => println!("all_the_rest_of_i32, {}...", all_the_rest_of_i32),
        }
    }
    handle_i32(0);
    handle_i32(1);
    handle_i32(2);
    handle_i32(10);
    handle_i32(100);

    fn handle_i32_underscore(i: i32) {
        match i {
             1 => println!("One, {}", i),
            10 => println!("Two, {}", i),
            _ => println!("Matches any other value and does not bind to that value..."),
        }
    }
    handle_i32_underscore(0);
    handle_i32_underscore(1);
    handle_i32_underscore(2);
    handle_i32_underscore(10);
    handle_i32_underscore(100);

    fn handle_i32_to_string(some_i32_value: i32) -> String {
        let string_value = match some_i32_value {
            10 => String::from("Ten"),
            11 => String::from("Eleven"),
            other => String::from(format!("Other i32 values, {}", other)),
        };
        println!("handle_i32_underscore: some_i32_value: {}, string_value: {}", some_i32_value, string_value);
        string_value
    }

    println!("");

    let s = handle_i32_to_string(0);
    println!("s: {}", s);
    let s = handle_i32_to_string(10);
    println!("s: {}", s);
    let s = handle_i32_to_string(11);
    println!("s: {}", s);
    let s = handle_i32_to_string(100);
    println!("s: {}", s);
}

fn section_6_3 () {
    println!("Chapter 6.3: Concise Control Flow with if let");

    // Using match
    let some_int = Some(100);
    match some_int {
        Some(i) => println!("match some_int -> i: {}", i),
        _ => println!("None")
    }

    // Using if let
    let some_int: Option<i32> = None;
    if let Some(i) = some_int {
        println!("some_int => i: {}", i);
    } else {
        println!("some_int => None");
    }

    let some_int: Option<i32> = Some(1024);
    if let Some(i) = some_int {
        println!("some_int => i: {}", i);
    } else {
        println!("some_int => None");
    }

    #[derive(Debug)]
    enum ET {
        ET1,
        ET2,
        ET3,
    }
    
    #[derive(Debug)]
    enum ETC {
        ETC1,
        ETC2(ET),
    }

    fn print_etc2_only(etc: ETC) {
        if let ETC::ETC2(etc_etc2) = etc {
            println!("Sub-enum passed: {:?}", etc_etc2);
        } else {
            println!("Sub-enum else: {:?}", etc);
        }
    }

    print_etc2_only(ETC::ETC1);
    print_etc2_only(ETC::ETC2(ET::ET1));
    print_etc2_only(ETC::ETC2(ET::ET2));
    print_etc2_only(ETC::ETC2(ET::ET3));
}