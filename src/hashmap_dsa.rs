
use std::collections::HashMap;

pub fn hashmap_dsa_examples(){
    //HashMap<K, V> stores a mapping of keys of type K to values of type V.
    /*
    Hash maps are useful when you want to look up data not by using an index, as you can with vectors, 
    but by using a key that can be of any type. 
    For example, in a game, you could keep track of each team’s score in a hash map in which each key is a team’s name and the values are each team’s score.
    Given a team name, you can retrieve its score.
     */

    //creating a new HashMap.
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"),10);
    scores.insert(String::from("Yellow"),50);

    // Like vectors, hash maps are homogeneous: all of the keys must have the same type as each other, and all of the values must have the same type.

    //accessing values in hashmap.
    //We can get a value out of the hash map by providing its key to the get method.
    let team_name = String::from("Blue");
    let _score = scores.get(&team_name).copied().unwrap_or(0);
    /*
    Here, score will have the value that’s associated with the Blue team, and the result will be 10. 
    The get method returns an Option<&V>; if there’s no value for that key in the hash map, get will return None. 
    This program handles the Option by calling copied to get an Option<i32> rather than an Option<&i32>, 
    then unwrap_or to set score to zero if scores doesn't have an entry for the key.
     */

    //We can iterate over each key/value pair in a hash map in a similar manner as we do with vectors
    for (key, value) in &scores {
        println!("{key}: {value}");
    }
    

    //HashMap ownership

    /* for types that implement copy trait like i32 the values are copied into the hashmap
    for owned values like String, the values will be moved into the hashmap. hashmap will be the owner of those values.

     */

    let field_name = String::from("Favorite Color");
    let field_value = String::from("Black");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!

    /* 
    We aren’t able to use the variables field_name and field_value after they’ve been moved into the hash map with the call to insert.

If we insert references to values into the hash map, the values won’t be moved into the hash map. The values that the references point to must be valid for at least as long as the hash map is valid.
     */

    /* 
    
    
     */
    //Overwriting a value.
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

    //Adding a key and value only if a key isn't present.
    /* 
    Hash maps have a special API for this called entry that takes the key you want to check as a parameter. The return value of the entry method is an enum called Entry that represents a value that might or might not exist. Let’s say we want to check whether the key for the Yellow team has a value associated with it. If it doesn’t, we want to insert the value 50, and the same for the Blue team. Using the entry API.
    The or_insert method on Entry is defined to return a mutable reference to the value for the corresponding Entry key if that key exists, and if not, inserts the parameter as the new value for this key and returns a mutable reference to the new value.

     */

    scores.entry(String::from("Blue")).or_insert(50);
    scores.entry(String::from("Red")).or_insert(30);

    println!("{:?}",scores);

}