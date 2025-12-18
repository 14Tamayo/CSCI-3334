use std::fs;
use std::fs::File;
use std::io;
use std::io::{Read, BufReader, BufRead};
use std::collections::HashMap;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::path::Path;
use std::path::PathBuf;
use std::fmt;
use std::time::{Duration, Instant};
use std::io::Write;

struct FileStats
{
	word_count: usize,
	line_count: usize,
	char_frequencies: HashMap<char, usize>,
	size_bytes: u64,
}

struct FileAnalysis
{
	filename: String,
	stats: FileStats,
	errors: Vec<ProcessingError>,
	processing_time: Duration,
}

#[derive(Debug)]
enum ProcessingError 
{
	IoError(String),
}

impl fmt::Display for ProcessingError
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			ProcessingError::IoError(msg) => write!(f, "IO error: {}", msg),
		}
	}
}


// Message to be sent to the workers
enum Message {
    NewJob(Job),
    Terminate,
}

// Job type is a boxed closure that can be sent across threads
type Job = Box<dyn FnOnce() + Send + 'static>;

// ThreadPool struct
struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    // Create a new ThreadPool with the specified size
    fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        
        // Create a channel for sending jobs
		let (sender, receiver) = mpsc::channel();
		
		//Wrap the reciver in Arc<Mutex<T>> to share it among workers
		let receiver = Arc::new(Mutex::new(receiver));
        
        
        // Create and store workers
		// For worker threads
        let mut workers = Vec::with_capacity(size);// Constructs new empty Vec<T> pre allocating enough memory on the heap for the specified amount
		
		//Create workers
		for id in 0..size 
		{
			//Create & store a new worker thread with a cloned reference to receiver
			workers.push(Worker::new(id, Arc::clone(&receiver)));
			
			println!("Worker {} created", id + 1);
            thread::sleep(Duration::from_millis(1000));
		}
        
        // Return the ThreadPool
        ThreadPool { workers, sender }
    }
    
    // Execute a job in the thread pool
    fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        // Create a job from the closure and send it to a worker
		// Box the closure
		let job = Box::new(f);
		
		//Send job as a message 
		self.sender.send(Message::NewJob(job)).unwrap();    
    }
}

// Clean up resources when ThreadPool is dropped
impl Drop for ThreadPool {
    fn drop(&mut self) {
        // Send terminate message to all workers
		println!("Sending terminate message to all workers...");
        thread::sleep(Duration::from_millis(1500));
		
		for _ in &self.workers
		{
			self.sender.send(Message::Terminate).unwrap();
		}
		
		println!("Shutting down all workers...");
        thread::sleep(Duration::from_millis(2000));
        
        // Wait for all workers to finish
		for worker in &mut self.workers
		{
			//Join each worker's thread 
			println!("Worker {} shutting down.", worker.id + 1);
            thread::sleep(Duration::from_millis(1000));
			
			if let Some(thread) = worker.thread.take() {
				thread.join().unwrap();
			}
		}
		
		
        
    }
}

// Worker struct represents a thread that can process jobs
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    // Create a new worker with the specified ID
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        // Create a thread that loops and receives jobs from the channel
		// Creating thread
		let thread = thread::spawn(move || {
			//Loop that receives messages
			loop
			{
				//Lock receiver & wait for messagesReceive jobs
				let message = receiver.lock().unwrap().recv().unwrap();
				
				//Use pattern matching on the message
				match message 
				{
					//NewJob display worker id
					Message::NewJob(job) => {
						println!("Worker {} is processing a job", id + 1);
                        thread::sleep(Duration::from_millis(1000));
						job();
					}
					
					//Terminate display closing message & break off the loop
					Message::Terminate => {
						println!("Worker {} received terminate signal", id + 1);
                        thread::sleep(Duration::from_millis(1000));
						break;
					}
				}
			}
		});
        
        
        // Return the Worker
		Worker
		{
			id,
			thread: Some(thread),
		}
        
    }
}

fn main() {
	
	println!("Greetings, this program will process some work with a threadpool of workers.");
	let worker_count: usize = loop
	{
		//Prompt
		println!("How many worker threads do you want to use?");
		let mut input = String::new();
	
		io::stdin()
			.read_line(&mut input)
			.expect("Failed to read input.");
			
		//Ignore non numbers & ask for another input
		match input.trim().parse() {
			Ok(num) => break num,
			Err(_) => {
				println!("Please enter a valid number.");
			}
		};
		
		println!("Input: {input}");
	};
	
    // Create a new thread pool 
    let pool = ThreadPool::new(worker_count);
	
	//Shared storage for results
	let results = Arc::new(Mutex::new(Vec::<FileAnalysis>::new()));
    
	//Files
	let input_paths = vec![
	PathBuf::from("test_dir/file.txt"),
	PathBuf::from("test_dir/file2.txt"),
	PathBuf::from("test_dir/file3.txt"),
    PathBuf::from("test_dir2/test.txt"),
    PathBuf::from("test_dir2/test2.txt"),
    PathBuf::from("test_dir2/test3.txt"),
	];
	
	let files = match collect_files(&input_paths) 
	{
		Ok(files) => files,
		Err(e) => {
			eprintln!("Failed to collect files: {}", e);
			return;
		}
	};
	
	println!("Discovered {} files", files.len());
    thread::sleep(Duration::from_millis(1000));
	
	//Submit jobs
	for path in files
	{
        //Atomic reference counting
		let results = Arc::clone(&results);
		
		pool.execute(move || {
			let start = Instant::now();
			let filename = path.to_string_lossy().to_string();
			let mut errors = Vec::new();
			
			let stats = match process_file(&path) {
				Ok(stats) => stats,
				Err(e) => {
					errors.push(ProcessingError::IoError(e.to_string()));
					
					//Empty stats if processing failed
					FileStats {
						word_count: 0,
						line_count: 0,
						char_frequencies: HashMap::new(),
						size_bytes: 0,
					}
				}
			};
			
			let analysis = FileAnalysis {
				filename,
				stats,
				errors,
				processing_time: start.elapsed(),
			};
			
			results.lock().unwrap().push(analysis);
		});
	}
	
	
    
    println!("Main thread waiting for tasks to complete...");
    thread::sleep(Duration::from_millis(3000));
    
	// Drop pool to wait for all workers
	drop(pool);
	
	
	
	//Write results to a file
	let results = results.lock().unwrap();
	
	//Create file
	let mut output = File::create("analysis_output.txt")
									.expect("Failed to create output file.");
									
	for data in results.iter()
	{
		writeln!(output, "{}", data).unwrap();
		writeln!(output).unwrap();
	}
	
}

//Gets reference to file path
fn process_file(path: &Path) -> io::Result<FileStats>
{
	//Open file
	let file = File::open(path)?;		//Returns error if unable to open
	
	//Reader
	let reader = BufReader::new(file);
	
	//Initial values
	let mut word_count = 0;
	let mut line_count = 0;
	let mut char_frequencies = HashMap::new();
	
	//Read file line by line
	for line in reader.lines()
	{
		let line = line?;
		line_count += 1;
		word_count += line.split_whitespace().count();
		
		//Check character frequency
		for c in line.chars() {
			*char_frequencies.entry(c).or_insert(0) += 1;
		}
	}
	
	
	
	//Get file size
	let size_bytes = fs::metadata(path)?.len();
	
	Ok(FileStats {
		word_count,
		line_count,
		char_frequencies,
		size_bytes,
	})
}

impl fmt::Display for FileStats
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		writeln!(f, "File Stats {{")?;
		writeln!(f, "Word count: {}", self.word_count)?;
		writeln!(f, "Line count: {}", self.line_count)?;
		writeln!(f, "Character frequencies: {{")?;
		
		for (ch, count) in &self.char_frequencies
		{
			writeln!(f, "  {:?}: {},", ch, count)?;
		}
		
		writeln!(f, "  }},")?;
		writeln!(f, "  Byte size: {}", self.size_bytes)?;
		writeln!(f, "}}")
	}
}

impl fmt::Display for FileAnalysis
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		writeln!(f, "File Analysis {{")?;
		writeln!(f, "File name: {:?},", self.filename)?;
		writeln!(f, "Stats: {},", self.stats)?;
		
		if self.errors.is_empty()
		{
			writeln!(f, "Errors: [],")?;
			
		}
		else
		{
			writeln!(f, "Errors: [")?;
			for err in &self.errors
			{
				writeln!(f, "  {},", err)?;
			}
			writeln!(f, "  ],")?;
		}
		
		writeln!(f, "  Processing time: {:?},", self.processing_time)?;
		write!(f, "}}")
	}
}

fn collect_files(paths: &[PathBuf]) -> io::Result<Vec<PathBuf>> 
{
    let mut files = Vec::new();

    for path in paths {
        if path.is_file() {
            files.push(path.clone());
        } else if path.is_dir() {
            visit_dir(path, &mut files)?;
        }
    }

    Ok(files)
}

fn visit_dir(dir: &Path, files: &mut Vec<PathBuf>) -> io::Result<()> 
{
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            visit_dir(&path, files)?;
        } else if path.is_file() {
            files.push(path);
        }
    }
    Ok(())
}