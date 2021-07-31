// We need to first `use` the `HashMap` from the
// collections portion of the standard library.
use std::collections::HashMap;
// It's not included in the features brought
// into scope automatically in the prelude.

// By default, `HashMap` uses a hashing function called SipHash
// that can provide resistance to Denial of Service (DoS) attacks
// involving hash tables.
// You can switch to another function by specifying a different hasher.
// A hasher is a type that implements the `BuildHasher` trait.

pub mod hashmaps {
	pub fn main() {
		// The type `HashMap<K, V> stores a mapping of keys
		// of type `K` to values of type `V`.

		// You can create an empty hash map with `new`
		// and add elements with `insert`.
		let mut scores = HashMap::new();

		scores.insert(String::from("Blue"), 10);
		scores.insert(String::from("Yellow"), 50);
		// Just like vectors, hash maps store their data on the heap.
		// This `HashMap` has keys of type `String and values of type `i32`.
		// Like vectors, hash maps are homogenous:
		// all of the keys must have the same type,
		// and all of the values must have the same type.

		// Another way of constructing a hash map is by using
		// iterators and the `collect` method on a vector of tuples,
		// where each tuple consists of a key and its value.
		// The `collect` method gathers data into a number of
		// collection types, including `HashMap`.
		let teams = vec![String::from("Blue"), String::from("Yellow")];
		let initial_scores = vec![10, 50];

		let mut scores: HashMap<_, _> =
			teams.into_iter.zip(initial_scores.into_ter()).collect();
		// We could use the `zip` method to create a vector of tuples.
		// Then we could use the `collect` method to turn that into a hash map.

		// The type annotation `HashMap<_, _>` is needed because it's possible
		// to `collect` into many different data structures and Rust doesn't know
		// which you want unless you specify.
		// For the parameters for the key and value types, we use underscores,
		// and Rust can infer the types that the hash map contains based
		// on the types of the data in the vectors.
	}

	pub fn ownership() {
		// For types that implement the `Copy` trait, like `i32`,
		// the values are copied into the hash map.
		// For owned values like `String`, the values will be moved
		// and the hash map will be the owner of those values.
		let field_name = String::from("Favorite color");
		let field_value = String::from("Blue");

		let mut map = HashMap::new();
		map.insert(field_name, field_value);
		// field_name and field_value are invalid at this point

		// If we insert references to values into the hash map,
		// the values won't be moved into the hash map.
		// The values that the references point to must be valid
		// for at least as long as the hash map is valid.
	}
	
	pub fn accessing() {
		// We can get a value out of the hash map by providing
		// its key to the `get` method.
		let mut scores = HashMap::new();

		scores.insert(String::from("Blue"), 10);
		scores.insert(String::from("Yellow"), 50);

		let team_name = String::from("Blue");
		let score = scores.get(&team_name);
		// `score` will have the value that's associated with
		// the Blue team, and the result will be `Some(&10)`.
		// The result is wrapped in `Some` because `get`
		// returns an `Option<&V>`; if there's no value
		// for that key in the hash map, `get` will return `None`.

		// We can iterate over each key/value pair in a hash map
		// in a similar manner as we do with, using a `for` loop:
		for (key, value) in &scores {
			println!("{}: {}", key, value);
		}
	}

	pub fn updating() {
		// If we insert a key and a value into a hash map and then
		// insert that same key with a different value,
		// the value associated with that key will be replaced.
		let mut scores = HashMap::new();

		scores.insert(String::from("Blue"), 10);
		scores.insert(String::from("Blue"), 25);

		println!("{:?}", scores);

		// It's common to check whether a particular key has a value and,
		// if it doesn't, insert a value for it.
		// Hash maps have a special API for this called `entry` that
		// takes the key you want to check as a parameter.
		// The return value of the `entry` method is an enum called `Entry`
		// that represents a value that might or might not exist.
		scores.entry(String::from("Yellow")).or_insert(50);
		scores.entry(String::from("Blue")).or_insert(50);

		println!("{:?}", scores);
		// The `or_insert` method on `Entry` is defined to return a mutable
		// reference to the value for  the corresponding `Entry` key if that key exists,
		// and if not, inserts the parameter as the new value for this key
		// and returns a mutable reference to the new value.

		// Another common use case for hash maps is to look up a key's value
		// and then update it based on the old value.
		// The following code counts how many times each word appears in some text.
		let text = "hello world wonderful world";

		let mut map = HashMap::new();

		for word in text.split_whitespace() {
			let count = map.entry(word).or_insert(0);
			*count += 1;
		}

		println!("{:?}", map);
		// This code will print `{"world": 2, "hello": 1, "wonderful": 1}`.
		// The `or_insert` method actually returns a mutable reference (`&mut V`)
		// to the value for this key.
		// Here we store that mutable reference in the `count` variable,
		// so in order to assign to that value, we must first dereference count
		// using the asterisk(`*`).
	}
}