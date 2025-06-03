# Issues and Handle 

## The tair bound `Vec<i32>: sqlx::Decode<_, _> is not satisfied. The trait sqlx::Decode<'_,_> ` is not implemented for Vec<i32>
### Add features = ["postgres", "array"] in Cargo.toml
Example: 
<mark>
[dependencies]
sqlx = { version = "0.6", features = ["postgres", "runtime-tokio-native-tls", "macros", "migrate", "array"] }
serde_json = "1.0"  # Needed if using JSONB storage

</mark>

###  Change participants: Vec<i32> to sqlx::types::Array<i32>
example: 
<mark>
use sqlx::types::Array;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ChatRoom {
    pub id: i32,             
    pub name: String,
    pub participants: Array<i32>, // Use sqlx::types::Array
    pub people: i32,          
}

</mark>
