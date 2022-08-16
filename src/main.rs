// fn main() {
//     println!("Hello, world!");
// }

//--------------------------------------------------------

// fn main() {
//     const THREE_AND_A_BIT : f32 = 3.4028236; 
//     println!("{}", THREE_AND_A_BIT);
// }

//--------------------------------------------------------

// use std::io::stdin;

// fn main() {
//     println!("What is 3+2? Type your answer and press enter.");
//     let mut input = String::new();
//     stdin()
//         .read_line(&mut input)
//         .expect("Unable to read standard input");

//     if input == "5" {
//         println!("Correct!");
//     } else {
//         println!("Incorrect!");
//     }
// }

//--------------------------------------------------------

// The surprise comes when we let it run for a while as it eventually consumes all of our computer’s memory or is terminated by the operating system.
// Memory unsafety is doing something with invalid data, a memory leak is not doing something with valid data.

// fn main() {
//     let x : u64 = 4_294_967_296; 
//     let y = x as u32;
//     if x == y as u64 {
//       println!("x equals y."); 
//     } else {
//       println!("x ({}) does not equal y ({}).", x, y); 
//     }
// }

//--------------------------------------------------------

// fn main() {
//     let mut counter : i8 = 0; 
//     loop {
//       println!("{}", counter);
//         counter += 1;
//     }
//   }

//--------------------------------------------------------

// const HELLO_WORLD : &'static str = "Halló heimur";

// fn main() {
//     println!("{} is {} characters long.",
//         HELLO_WORLD,
//         HELLO_WORLD.len()
//     );
// }

//--------------------------------------------------------

// const HELLO_WORLD : &'static str = "love: ❤";

// fn main() {
//     println!("{}", HELLO_WORLD.len());
//     println!("{} is {} characters long.", 
//     HELLO_WORLD,
//     HELLO_WORLD
//         .chars() // Convert to an iterator over a char sequence 
//         .count() // Count the characters in the sequence
//     );

//     let arr: Vec<char> = HELLO_WORLD.chars().collect();
//     println!("{:?}", arr);

// }

//--------------------------------------------------------

// fn main() {
//     let sum = 0.1+0.2;

//     if sum == 0.3 {
//         println!("Arithmetic still works.");
//     } else {
//         println!("Please reboot the universe. {}", sum);
//     }
// }

//--------------------------------------------------------

// use std::f32::consts::PI;

// pub struct Degrees(pub f32);
// pub struct Radians(pub f32);

// impl Degrees {
//     pub fn new(angle: f32) -> Self {
//         Self(angle)
//     }
// }

// impl From<Degrees> for Radians {
//     fn from(item : Degrees) -> Self {
//         Self(item.0 * PI / 180.0)
//     }
// }

// fn main() {
//     let one_eighty_degrees = Degrees::new(180.0);
//     let one_eighty_radians : Radians = one_eighty_degrees.into();
//     println!("180 Degrees in Radians = {}", one_eighty_radians.0);
// }


//--------------------------------------------------------
// use std::convert::TryFrom;

// #[derive(Debug)]
// struct ZeroToTen(i32);

// impl TryFrom<i32> for ZeroToTen {
//     type Error = &'static str;

//     fn try_from(num: i32) -> Result<Self, Self::Error>{
//         if num < 0 || num > 10 {
//             Err("Number should be > 10")
//         } else {
//             Ok(Self(num))
//         }
//     }
// }

// fn main() {
//     let result = ZeroToTen::try_from(6);

//     println!("6 - {:?}", result.unwrap());
// }

//--------------------------------------------------------

// Rust’s underscore (or placeholder) symbol has different meanings in different contexts:

// When used as a variable name prefix (for example, _ignore_me : i32), the underscore indicates to Rust that the variable is deliberately unused. Rust will suppress unused variable warnings.

// When we use it as an entire variable name, we tell Rust that we don’t intend to use the variable at all. When used in a match statement (for example, _ => { .. }), the underscore indicates a default action. If no other match arms are fired, then the default action will be evaluated.

// Underscores can be used with functions that return a value marked with #[must_use]. For example, the statement let _ = my_important_function() will ignore the result of the function, suppressing errors or warnings that aren’t being used in the result.

// fn double_it(n: u64, _: i32) -> u64 {
//     n * 2
// }

// fn main() {
//     let one : i32 = 1;
//     let n = double_it(one as _, 3);
//     println!("{}", n);
// }

// When we combine type inference issues with the potential precision loss from the as keyword, we invite trouble. 
// In many cases, we should use into() (or try_into()) instead of as altogether.

// The most common use-case for as _ is low-level pointer code. 
// unsafe fn clone_ptr(&self) -> *mut () {
//     Box::into_raw(Box::new(self.clone())) as _
//  }

// Use () as a go-to choice because it’s precise and optimizes very well.

// The try_into() function lets us handle failed conversions.

// Use as type when we’re certain that conversions are safe.

// Use as _ when we’re really stuck.

//--------------------------------------------------------

// Since floating-point numbers aren’t always numbers, they aren’t always naturally sortable.

// Rust introduced the PartialOrd and PartialEq traits along with the accompanying partial_cmp() function. 
// The PartialOrd and PartialEq traits as well as the partial_cmp() function represent numeric types that are 
// generally comparable  but may feature cases in which two numbers cannot be naturally compared or ordered.

// fn main() {
//     let mut floats = vec![3.1, 1.2, 4.5, 0.3];
//     floats.sort();

//     println!("{:#?}", floats);
// }

// fn main() {
//     let mut floats = vec![3.1, 1.2, 4.5, 0.3];
//     floats.sort_by(|a, b| a.partial_cmp(b).unwrap());
//     println!("{:#?}", floats);
// }

// use std::cmp::Ordering::Less; 

// fn main() {
//     let mut floats = vec![
//         3.1, 1.2, 4.5, 0.3, std::f32::INFINITY, std::f32::NAN 
//     ];
    
//     floats.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Less));
//     println!("{:#?}", floats);
// }

// fn float_sort<T : PartialOrd>(data: &mut [T]) {
//     use std::cmp::Ordering::Less;
//     data.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Less));
// }

//--------------------------------------------------------

// fn main() {
//     if 'X' == 'X' {
//         println!("It matches!");
//     } else {
//         println!("It doesn't match.");
//     }
// }

//--------------------------------------------------------

// Every program running on a computer maintains its own stack—a small resource that uses two megabytes per thread by default.
// Because the stack is stored in the CPU’s memory cache, the program runs faster and alerts the operating system that it 
// should keep the program’s stack available instead of paging it out to disk.

// The heap—another area of memory that every program maintains—is limited by a computer’s available memory, 
// virtual memory, and operating system limitations. Heap memory is large and may or may not be contiguous depending on the operating system.

// Storing data on the heap requires more steps than storing data on the stack. 
// First, the Rust Standard Library needs to request a heap allocation that returns a pointer to a usable area of memory. 
// It then needs to store the pointer before it can write data to the heap.

// The Box type represents a smart pointer to heap memory, making it a natural way to store large arrays on the heap. 
// Unfortunately, constructing a box with Box::new first creates the array on the stack and then moves it to the heap.

// fn main() {
//     let c = Box::new([0u32; 10_000_000]);
//     println!("{}", c.len());
// }

// fn main() {
//     let c = vec![0u32;10_000_000];
//     println!("{}", c.len());
// }

// #![feature(box_syntax)]

// fn main() {
//     let c = box [0u32; 10_000_000];
//     println!("{}", c.len())
// }

//--------------------------------------------------------

// Memory leaks are not a violation of memory safety. 
// Memory unsafety is doing something with invalid data, a memory leak is not doing something with valid data.
// std::mem::drop

// fn main() {
//     loop {
//         let buffer = (0..1000).collect::<Vec<u32>>();
//         std::mem::forget(buffer); // memory leaks
//         print!(".");
//     }
// }

//--------------------------------------------------------

// Another use for shadowing is converting generic inputs into a concrete type and reusing the name for clarity.
// Another useful case for variable shadowing is providing internal mutability without requiring mutability in our function signature.
// This pattern is especially common when working with a local clone of a variable. 
// The statement let n = n.clone() is often more readable than let n_clone = n.clone().
// #![warn(clippy::shadow_same)]
// #![warn(clippy::shadow_unrelated)]

// #![warn(clippy::shadow_reuse)]

// fn display_neutron_flow(polarity: isize) {
//     println!(
//         "Neutron Flow is {}",
//         if polarity < 0 { "reversed"} else { "normal" }
//     );
// }

// fn main() {
//     let polarity = 1;
//     {
//         let polarity = polarity - 2;
//         display_neutron_flow(polarity);
//     }
//     display_neutron_flow(polarity);
// }

// fn examine_a_string<S: ToString>(my_string: S) -> String { 
//     let my_string = my_string.to_string();
//     // Perform lots of complicated processing here
//     my_string
// }

// fn main() {
//     let x = a+b;
// // Do something with x 

// let x = c/d;
// // Do something with x
// }

// fn main() {

//     let string1 = examine_a_string("Examining a String!");
//     let string2 = examine_a_string(55);
    
//     println!("{}, {}", string1, string2);
// }

// fn my_complex_function(base: f32, data: &[f32]) -> f32 {
//     let mut base = if base < 0.0 { base + 100.0 } else { base };
  
//     // Iteratively calculate base. You'd probably do something more useful
//     // here.
//     data.iter().for_each(|n| base += n);
//     base
//  }
  
//  fn main() {
//     let arr:[f32; 3] = [0.5, 1.3, 2.6];
//     println!("{}",  my_complex_function(10.0, &arr));
//  }

//--------------------------------------------------------
// Most modern CPUs align data on 32-bit boundaries for memory and cache.
// A 24-bit (three byte) structure does not naturally align to a 32-bit memory map.
// So, by default, Rust wastes 8-bits of memory per struct to ensure fast access to our computer’s memory structure.
// A structure decorated with #[repr(C, packed)] won’t rearrange or pad our structure.

// use std::mem::size_of;

// struct VeryImportantMessage {
//     _message_type : u8,
//     _destination : u16
// }

// fn main() {
//     println!(
//         "VeryImportantMessage occupies {} bytes.", 
//         size_of::<VeryImportantMessage>()
//     );
// }

// use std::mem::size_of;

// #[repr(C, packed)]
// struct ReallyThreeBytes {
//     a : u8,
//     b : u16 
// }
// fn main() {
//     println!("ReallyThreeBytes occupies {} bytes.", 
//     size_of::<ReallyThreeBytes>());
// }

//--------------------------------------------------------

// Rust does not support function overloading.

// fn double_it(n: i32) -> i32 {
//     n * 2
// }

// fn double_it(n: f32) -> f32 {
//     n * 2.0
// }

// fn main() {
//     println!("2 * 4 = {}", double_it(2));
// }

// fn double_it<T>(n: T) -> T 
// where T: std::ops:: Mul<Output = T> + From<i32> 
// {
//     n*2.into()
// }

// fn main() {
//     println!("2 * 2 = {}", double_it(2.13));
// }

//--------------------------------------------------------

// If we have a rough idea of how much data we might need to store, use Vec::with_capacity() to reserve an appropriate amount of space ahead of time. 

// If we’re adding lots of data, we can use Vec::extend() so that Rust can see the size of the data that’s being added and reallocate only once. 
// The extend() trait only avoids allocation when collecting data from a source with a known length. Copying from one vector to another allows Rust 
// to allocate exactly the space it needs for the new vector because the length is known. Likewise, any iterator that implements ExactSizeIterator benefits from this optimization. 
// An arbitrarily sized iterator may repeatedly allocate because the size of the data we’re copying isn’t known ahead of time.

// We can add elements to the end of our vectors using push rather than at a specific slot using insert. Although insert gives us more control, 
// it’s a lot slower than push because Rust needs to rearrange the vector to make room for your new element. If we need to insert an element at the front, 
// the VecDeque structure is a better choice.

// fn main() {
//     let mut my_vec = Vec::with_capacity(1);
//     my_vec.push("Hello");
//     println!("{}", my_vec.capacity());

//     my_vec.push("World");
//     println!("{}", my_vec.capacity());
// }

//--------------------------------------------------------

// fn main() {
//     let life_the_universe = &mut 41;
//     *life_the_universe += 1;
//     println!("Life, the Universe and Everything: {}", life_the_universe);
// }

// fn main() {
//     let mut life_the_universe = 41;
//     life_the_universe += 1;
//     println!("Life, the Universe and Everything: {}", life_the_universe);
// }

// It works the same either way. The pattern of a mutable variable—a label for an area of memory—accessed by other variables is a common one.

// fn main() {
//     let mut life = 40;
//     let the_universe = &mut life; 
//     *the_universe += 2; 

//     println!("{}", the_universe);
// }

//--------------------------------------------------------