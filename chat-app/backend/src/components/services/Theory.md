# Attributes

## Serialize (From Rust to JSON, BSON, etc.)

The Serialize trait (from serde) allows yout struct or enum to be converted into formats like JSON, BSON, YANL, etc.

### Example of code

<mark> 
use serde::Serialize;
use serde_json;

#[derive(Serialize)]
struct User {
name:String,
age: u8,
}
fn main() {
let User = User {
name: "Alice".to_string(),
age: 25,
};
let json = serde_json::to_string(&user).unwrap();
println("{}", json);
}

</mark>

## Deserialize (From JSON, BSON, etc.)

The Deserialize trait allows yout struct or enum to be converted from formats like JSONetc.

### Example of code

<mark>
use serde::Deserialize;
use serde_json;

#[derive(Deserialize)]
struct User {
name: String,
age: u8,
}

fn main() {
let json_data = r#"{"name": "Bob", "age": 30}"#;
let user: User = serde_json::from_str(json_data).unwrap();

    println!("Name: {}, Age: {}", user.name, user.age);

}

</mark>

## debug (For easy Debugging)

The debug trait allows you to print your struct or enum in a human-readable format using {:?} in println!().

Without `#[derive(Debug)]` - you would't be able to prin the struct using {:?}.

## Module collections

- Sequences: Vec, VecDeque, LinkedList
- Maps: HashMap, BTreeMap
- Sets: HashSet, BTreeSet
- Misc: BinaryHeap

### Vec<T> - dynamic Array

<mark> 
    let mut numbers: Vec<i32> = vec![10, 20, 30];
    numbers.push(40);
    numbers.pop(); // remove last element 
</mark>

### VecDeque<T> - efficient queue

- Is a double-ended queue, supporting fast push/pop at both ends.
  <mark>
  let mut queue: VecDeque<i32> = VecDeque::new()

      queue.push_back(10); // Enqueue (end)
      queue.push_frone(5); // Enqueue (front)

      println!("{:?}", queue,pop_front()); // DEqueue from front

  </mark>

### Responder trait

<mark>
Trait implemented by types can be converted to an HTTP response. Any Types that implement this trait can be used in the return type of a handler. Since handlers will only have one return type, it is idiomatic to use opaque return types.
</mark>

# Networking

`netstat -na | grep "8080"`

# Cryprography

use argon2::{self, Config} 

