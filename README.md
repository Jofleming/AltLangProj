# AltLangProj

Which programming language and version did you pick?
    * I picked Rust. Though I really wish I hadn't.

Why did you pick this programming language?
    * It is often talked about as a memory safe version of C++

How your programming language chosen handles: object-oriented programming, file ingestion, conditional statements, assignment statements, loops, subprograms (functions/methods), unit testing and exception handling. If one or more of these are not supported by your programming language, indicate it as so. 

**Object-Oriented Programming:** Rust supports object-oriented programming through structs and traits. Structs define the data structure of an object, and traits define the behavior of an object. Rust uses a different approach compared to traditional class-based inheritance in languages like Java. It utilizes composition and traits to achieve similar functionality.

**File Ingestion:** Rust provides libraries for file handling, such as the `std::fs` module. This module allows you to open, read, write, and manipulate files.

**Conditional Statements:** Rust supports conditional statements using if, else if, and else keywords. You can also use match expressions for more complex pattern matching.

**Assignment Statements:** Rust allows assignment statements using the `=` operator. The data type of the right side of the assignment must be compatible with the variable on the left.

**Loops:** Rust provides various loop constructs, including `for` loops for iterating over collections and `while` loops for repeating a block of code as long as a condition is true.

**Subprograms (Functions/Methods):** Rust supports functions and methods. Functions are standalone blocks of code that can be reused, while methods are functions associated with a particular struct type.

**Unit Testing:** Rust has a strong unit testing culture with frameworks like `cargo test`. Unit testing allows developers to write tests for individual units of code to ensure they work as expected.

**Exception Handling:** Rust uses a different approach to error handling compared to traditional exceptions in languages like Java. Rust employs a result type (`Result<T, E>`) that indicates either success (`Ok(value)`) or error (`Err(error)`). This forces developers to explicitly handle potential errors, leading to more robust and predictable code.

List out 3 libraries you used from your programming language (if applicable) and explain what they are, why you chose them and what you used them for.
    * Regex: Was used to parse the cells for only the information I wanted. This was to clean the data.
    * `std::fs::File` to be able to access the file.
    * `std::io::{BufReader, BufRead, Result}` to be able to read the file.
    * `std::collections::{HashSet, HashMap}` used to check for duplicates and to store data.

Answer the following questions (and provide a corresponding screen showing output answering them):

What company (oem) has the highest average weight of the phone body?
    = Company with highest average weight: Lenovo with average weight 655

Was there any phones that were announced in one year and released in another? What are they? Give me the oem and models.
    = Mitac MIO Leap G50 was announced in 2008 and released in 2009
Mitac MIO Leap K1 was announced in 2008 and released in 2009
Motorola XT701 was announced in 2009 and released in 2010
Sharp AQUOS  941SH was announced in 2009 and released in 2010
Sharp 940SH was announced in 2009 and released in 2010

How many phones have only one feature sensor?
    = Number of phones with only one feature sensor: 731

What year had the most phones launched in any year later than 1999? 
    = Year with the most phones launched after 1999: 2019 with 251 launches



