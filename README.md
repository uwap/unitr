UNITr
=====
UNITr is a library for rust designed for people having to use units a lot.  
Here is a great example of the power of this library:  
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
Now, this code has the output `You walked Millimeter(5)`. The method `.val()` used above  
will give you the value of this Millimeter type. So `distance_walked.val()` is `5f64`.  
Getting Started
===============
To start using this library, simply add the following 2 lines to your Cargo.toml:
```
[dependencies.unitr]
git = "https://github.com/uwap/unitr.git"
```
And add `extern crate unitr;` in your main.rs / lib.rs.
Distance Types
==============
There are several distance types implemented. Take a look at this list:
* `Kilometer<T>(T)` is accessible by `.km()`
* `Meter<T>(T)` is accessible by `.m()`
* `Decimeter<T>(T)` is accessible by `.dm()`
* `Centimeter<T>(T)` is accessible by `.cm()`
* `Millimeter<T>(T)` is accessible by `.mm()`

As you can see, those units are simple unit structs using generics.  
So, instead of `f64` you could also use `f32` or `i32` or even `u64`.  
Note, that we are not supporting anything below 32 bits, because it may  
cause some trouble when converting units. `1u8.mm().km().val()` is probably just 256.

As you might guess, `m.val()` is just syntactic sugar for `match m { Meter(v) => v }`.
Time Types
==========
As well as distances, UNITr also supports time.
* `Hour<T>(T)` is accessible by `.h()`
* `Minute<T>(T)` is accessible by `.min()`
* `Second<T>(T)` is accessible by `.s()`
* `Millisecond<T>(T)` is accessible by `.ms()`

Pretty straight forward, isn't it?
Velocity Types
==============
__These are just plans! None of them is implemented yet!__  
With velocities, UNITr shows its full potential.  
```
let distance  = 10f32.m();
let time      = 5f32.s();
let velocity  = distance / time;
print!("Your speed was {} km/h!", velocity.km_h().val())
```
Dividing distance by time will in fact give you velocities.  
There are many, many, many velocity types.  
Each of them is called `{distance}sPer{time}`, where {distance} is one of the distance  
types and {time} is one of the time types. For example those are possible types:  
* `MetersPerSecond<T>(T)` is accessible by `.m_s()`
* `KilometersPerHour<T>(T)` is accessible by `.km_h()`
* `MillimetersPerMillisecond<T>(T)` is accessible by `.mm_ms()`
* `CentimetersPerMinute<T>(T)` is accessible by `.cm_min()`

You can make up any combination you want.  
The conversation is done in the pattern of `.{distance}_{time}()`, `.m_s()` for example.
