UNITr
=====
UNITr is a library for rust designed for people having to use units a lot.  
For now it only contains distances. Here is a great example of the power of this library:  
```
let distance_walked = 32f64.km();  
println!("You walked {} meters!", distance_walked.m.val())
```
As you can see the `.km()` denotes that the specific value is measured in kilometers.  
You can simply use `.m()` to convert it into meters.  
Another example of that:
```
let distance_walked = 5f64.mm();
println!("You walked {}", distance_walked)
```
Now, this code has the output `Millimeter(5)`. The method `.val()` used above will  
give you the value of this Millimeter type. So `distance_walked.val()` is `5f64`.  
Distance Types
==============
There are several distance types implemented. Take a look at this list:
* Kilometer<T>(T)
* Meter<T>(T)
* Decimeter<T>(T)
* Centimeter<T>(T)
* Millimeter<T>(T)
As you can see, those units are simple unit structs using generics.  
So, instead of `f64` you could also use `f32` or `i32` or even `u64`.  
Note, that we are not supporting anything below 32 bits, because it may  
cause some trouble when converting units. `1u8.mm().km().val()` is probably just 256.

As you might guess, `m.val()` is just syntactic sugar for `match m { Meter(v) => v }`.
