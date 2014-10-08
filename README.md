UNITr
=====
1. [Getting Started](#getting-started)
2. [A Qick Overview](#a-quick-overview)
3. [Distance Types](#distance-types)
4. [Time Types](#time-types)
5. [Velocity Types](#velocity-types)
6. [Area Types](#area-types)
7. [TODO](#todo)

Getting Started
===============
To start using this library, simply add the following 2 lines to your Cargo.toml:
```
[dependencies.unitr]
git = "https://github.com/uwap/unitr.git"
```
And add `extern crate unitr;` in your main.rs / lib.rs.
A Quick Overview
================
UNITr is a library for rust designed for people having to use units a lot.  
Here is a great example of the power of this library:  
```rust
let distance_walked = 32f64.km();
println!("You walked {} meters!", distance_walked.m().val())
```
As you can see the `.km()` denotes that the specific value is measured in kilometers.  
You can simply use `.m()` to convert it into meters.  
Another example of that:
```rust
let distance_walked = 5f64.mm();
println!("You walked {}", distance_walked)
```
Now, this code has the output `You walked 5mm`. The method `.val()` used above  
will give you the value of this Millimeter type. So `distance_walked.val()` is `5f64`.  
When calculating with operators, you may sometimes do mistakes.  
For example you have two values of different units and add them together.  
```rust
let my_centimeters = 100f32;
let my_meters = 0.4f32;
my_centimeters + my_meters // what is the unit now?
```
UNITr makes this way easier.
```rust
let my_centimeters = 100f32.cm();
let my_meters = 0.4f32.m();
my_centimeters + my_meters // what is the result? Well, it is 500cm.
```
When adding two units together the result will be of the same type as the left hand side expression.  
Though we allow adding and subtracting, multiplying and dividing is mostly forbidden.  
The reason is simple. What unit will you get when dividing meters by meters?  
Well, right. None.
Distance Types
==============
There are several distance types implemented. Take a look at this list:
* Kilometers are accessible by `.km()`
* Meters are accessible by `.m()`
* Decimeters are accessible by `.dm()`
* Centimeters are accessible by `.cm()`
* Millimeters are accessible by `.mm()`

Time Types
==========
As well as distances, UNITr also supports time.
* Hours are accessible by `.h()`
* Minutes are accessible by `.minute()`
* Seconds are accessible by `.s()`
* Milliseconds are accessible by `.ms()`
* Nanoseconds are accessible by `.ns()`

__Note: Because of the ambiguity between `fn min<T: Ord>(T)` and `.min()` the minutes are written `.minute()` instead of `.min()`__  
Though, the _velocity is still `.{distance}_min()`_ (See [Velocity Types](#velocity-types))
Velocity Types
==============
With velocities, UNITr shows its full potential.  
```rust
let distance  = 10f32.m();
let time      = 5f32.s();
let velocity  = distance / time;
println!("Your speed was {}", velocity.km_h())
```
Dividing distance by time will in fact give you velocities.  
There are many, many, many velocity units.  
These are made of a distance and a time unit. For example:  
* Meters per Second are accessible by `.m_s()`
* Kilometers per Hour are accessible by `.km_h()`
* Millimeters per Millisecond are accessible by `.mm_ms()`
* Centimeters per Hour are accessible by `.cm_min()`

You can make up any combination you want.  
The conversation is done in the pattern of `.{distance}_{time}()`, `.m_s()` for example.  
__Note: Instead of using `.minute()` as for the times, velocities use `.min()`__
Area Types
==========
Area types can be accessed on two ways. The usual method:
* Square Meters are accessible by `.m2()`
* Square Centimeters are accessible by `.cm2()`

And so on...  
Or they can be accessed by multiplying to distances together.
```rust
let a = 100.cm();
let b = 80.cm();
let A = a * b;
assert_eq!(A, 8000.cm2())
```
As well as non metric units are still missing, also `.a()` and `.ha()` aren't a thing yet.  
```rust
let a = 1.km();
let b = 2000.m();
let A = a*b;
assert_eq!(A, 9.km2())
```
As you can see, when multiplying two different distance units together, the result's unit  
will be the square of the left hand side unit.
TODO
====
Beside the previous named features we have a lot of things on our todo list.  
* Acceleration
* Temperature
* Non-metric units
