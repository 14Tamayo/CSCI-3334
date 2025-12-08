println!("Intro to Concurrency");
let steps =  Arc::new(5);
let thread = {
    let steps = steps.clone();
    std::thread::spawn(move ||{
        for step in 1..=*steps {
            std::thread::sleep(std::time::Duration::from_secs(1));
            println!("Thread step {}",step);
        }
        panic!("Something went wrong");
        "Goodbye!" // important thread could return values
    })
};

println!("Spawned a thread!");

// Very important moment to understand closure captures
// the environment

println!("steps now available {}", steps);
std::thread::sleep(std::time::Duration::from_secs(3));
println!("Main thread slept for 3 seconds");
// Now we join our spawned thread with it's returned value, if we don't our function just returns
// without waiting for spawned thread
let result = thread.join(); //.unwrap(); // we need to unwrap result enum,because potentially thread could panick and we end up with err

println!("Thread returned: {:?}", result);

fn main() {
    println!("Hello, world!");
}
