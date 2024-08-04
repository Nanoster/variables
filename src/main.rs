 fn main() {
    //--------------------mutable
    // const THREE_HOUR_IN_SECONDS: u32 = 60 * 60 * 3; //const
    // let mut x = 5;
    // println!("The calue of x is: {x}");
    // x = 6;
    // println!("The calue of x is: {x}");

    //--------------------shadowing
    // let x = 5;

    // let x = x + 1;

    // {
    //     let x = x * 2;
    //     println!("The value of x in the inner scope is: {x}");
    // }

    // println!("The value of x is: {x}");

    //--------------------diffrent type let
    let spaces = "   ";
    let spaces = spaces.len();
    
    println!("spaces: {spaces}");
    //--------------------diffrent type let(type error)
    //--------------------add question(let -> let mut shadowing)
    
        
    let mut spaces = "   ";
    println!("spaces: {spaces}");
    // spaces = spaces.len();       //type error
    spaces = "mut";
    
    println!("spaces: {spaces}");
        
    //---------------------tuple test
    let a: (u32,u32,i32) = (0,0,1);

    println!("{:?}",a);

    //---------------------array test
    let b = [2; 5];

    println!("{:?}",b);

}
