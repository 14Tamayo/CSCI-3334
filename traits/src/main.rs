/*
    Define 2 struct undergrad & grad student

    Define a trait show_info

    Grad student should have a thesis component
    gpa & major will be shared

    Create another struct called Enrollment
    Inside enrollment store undergrad & grads together
    Implenet show_info for all enrolled student

    Everywhere use generics & traits, no if or match statement program behavior only
*/

pub trait Show_info
{
    fn show_info(&self) -> String;

}

pub struct grad_student
{
    pub thesis: String,
    pub gpa: String,
    pub mayor: String,
}

impl Show_info for grad_student 
{
    fn show_info(&self) -> String 
    {
        format!("Thesis: {}, GPA: {}, Mayor: {}", self.thesis, self.gpa, self.mayor)
    }
}

pub struct under_grad_student
{
    pub gpa: String,
    pub mayor: String,
}

impl Show_info for under_grad_student
{
    fn show_info(&self) -> String
    {
        format!("GPA: {}, Mayor: {}", self.gpa, self.mayor)
    }
}

pub struct Enrollment<T: Show_info, U: Show_info>
{
    pub grad: T,
    pub under_grad: U,
}

impl<T: Show_info, U: Show_info> Show_info for Enrollment<T, U>
{
    fn show_info(&self) -> String
    {
        format!("{}\n{}", self.grad.show_info(), self.under_grad.show_info())
    }
}


fn main()
{
    //Creating a grad_student struct
    let graduateStudent = grad_student 
    {
        thesis: String::from("Videogame Studies"),
        gpa: String::from("4.0"),
        mayor: String::from("Game Development"),
    };
    //println!("{}", graduateStudent.show_info());

    //Creating a under_grad_student struct
    let underGraduate = under_grad_student
    {
        gpa: String::from("3.0"),
        mayor: String::from("Computer Science"),
    };
    //println!("{}", underGraduate.show_info());

    //Creating Enrollment struct
    let enrollmentList = Enrollment
    {
        grad: graduateStudent,
        under_grad: underGraduate,
    };

    println!("{}", enrollmentList.show_info());

}


