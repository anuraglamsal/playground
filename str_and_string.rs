fn main(){

    // Because of the vast possible size difference in different strings, it practically
    // doesn't make sense to have a string type of a fixed size. It would need to be pretty
    // big (what even is the size limit?) and in a lot of cases, most of the memory would 
    // just go to waste. Contrast this with ints; a 32-bit memory (which isn't that much) can
    // represent 2^32 integers. Even if some bits are wasted, it's practically insignificant. 
    // And if you care about this waste, there are usually other versions of int, like
    // "i8" (8-bit int) for instance. Imagine having this sort of paradigm for strings -- doesn't
    // seem practical. 

    // Therefore, having a dynamic type for strings makes sense. "String" is just that in rust. "String"
    // is used to allocate dynamic strings in the heap. 

    let string_1: String = String::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit. 
    Phasellus eget libero lobortis, rutrum ipsum ac, volutpat ex. Curabitur non magna 
    ac turpis fermentum vulputate. Pellentesque habitant morbi tristique senectus et 
    netus et malesuada fames ac turpis egestas. Mauris maximus mi quam, vel finibus 
    arcu rhoncus vel. Etiam eu magna ac eros imperdiet tincidunt. Fusce vel volutpat 
    tellus. Fusce blandit, neque ut auctor commodo, eros nisl porta dui, eget pretium 
    nunc ipsum ac nisl. Ut tristique nisi vel elit finibus, a pretium velit dapibus.");

    let string_2: String = String::from("Hello");
    
    println!("{}", string_1.len()); // Takes 573 bytes.
    println!("{}", string_2.len()); // Takes 5 bytes. The difference is pretty huge. 

    // What if you don't want/have to deal with the overhead and complexity of the heap? 
    // Maybe you have a string that doesn't change. You can define strings like this using
    // the "str" type in Rust. Obviously, the argument in the first paragraph still holds, 
    // therefore this type too can't have some fixed size. Rust bypasses this by making
    // "str" a reference type i.e. it references some memory location rather than actually
    // being the memory location. The "str" type basically stores the memory address of the
    // first character of the string and the length of the string, thus allowing you to 
    // access the whole string. All in all, when you define a string using "str" (or technically
    // "&str" as it is a reference type), the string literal is stored in some memory location,
    // and the "str" type variable's fields for address and length are updated. 

    let string_3: &str = "Hi";
    let ptr = string_3.as_ptr();
    let len = string_3.len();

    println!("{:?}", ptr); // 0x59ef32c1b31c or something like this.
    println!("{}", len); // 2

    // "str" type strings are immutable i.e. "str" is an immutable reference type.
    // They also have a static lifetime, which means the string is guranteed to be
    // valid for the duration of the entire program. This string is basically
    // stored in the read-only memory of the binary. 

    // Evidently this is complicated and there is more to the story. You can read
    // rust docs: 
    // https://doc.rust-lang.org/book/ch04-03-slices.html#string-literals-as-slices
    // https://doc.rust-lang.org/std/primitive.str.html
    // https://doc.rust-lang.org/rust-by-example/std/str.html
    // https://doc.rust-lang.org/std/string/struct.String.html
}