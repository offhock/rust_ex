mod ex_vector; 
mod ex_string; 
mod ex_hashmap; 

use ex_vector as v;
use ex_string as s;
use ex_hashmap as h;

fn main() {

    v::collection_vector();
    s::collection_string();
    h::collection_hashmap();
}
