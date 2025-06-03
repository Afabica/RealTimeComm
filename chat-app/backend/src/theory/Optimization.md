# Optimization of program 

## Use Arc<Mutex<T>> instead of Vec<T> for shared state.
<mark> 

### Problem
Right now, the UserService stores users in a Vec<User>, which is not thread-safe. If multiple threads modify it, you'll face data races.

Solution: `Arc<Mutex<Vec<User>>>` - this allows multiple threads to read and write safely.


</mark>
