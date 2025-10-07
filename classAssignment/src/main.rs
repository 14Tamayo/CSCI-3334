
struct Student {
    name: String,
    major: String,
}

//struct Student implementation
impl Student {
    fn new(n:String, m:String) -> Self {
        Student {
            name: n,
            major: m,
        }
    }

    fn get_major( &self ) -> &String {
        return &self.major
    }

    fn set_major(&mut self, new_major: String){
        self.major = new_major
    }
}


fn main() {
    let student = Student::new("Jesus Tamayo".to_string(), "Computer Science".to_string());

    println!("Student name: {}", student.name);
    println!("Major: {}", student.major);
}
