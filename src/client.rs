use std::io;
use greeter::greeter_client::GreeterClient;
use greeter::HelloRequest;
pub mod greeter {
    tonic::include_proto!("greeter");
}


struct Employee {
    name: String,
    gender: String,
    age: String,
    company: String,
    department: String,
    id: String,
    location: String,
}

impl Employee {
    fn new() -> Employee {
        Employee {
            name: String::new(),
            gender: String::new(),
            age: String::new(),
            company: String::new(),
            department: String::new(),
            id: String::new(),
            location: String::new(),
        }
    }

    fn get_input(prompt: &str) -> String {
        println!("{:?}", prompt);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        input.trim().to_string()
    }

    fn collect_info(&mut self) {
        self.name = Employee::get_input("Enter your Name");
        self.gender = Employee::get_input("Enter your gender");
        self.age = Employee::get_input("Enter your age");
        self.company = Employee::get_input("Enter your company");
        self.department = Employee::get_input("Enter your department");
        self.id = Employee::get_input("Enter your id");
        self.location = Employee::get_input("Enter your work location");
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;
    println!("If you want to start conversation enter START, to stop conversation enter QUIT");
    let mut conv_status = String::new();
    io::stdin().read_line(&mut conv_status).unwrap();
    if conv_status.trim() == "START".to_string(){
        
        let mut employee = Employee::new();
        employee.collect_info();
        let request = tonic::Request::new(HelloRequest {
            name: employee.name.parse().unwrap(),
            gender: employee.gender.parse().unwrap(),
            age :employee.age.parse().unwrap(),
            company: employee.company.parse().unwrap(),
            department: employee.department.parse().unwrap(),
            id: employee.id.parse().unwrap(),
            location: employee.location.parse().unwrap(),
        });
        println!("Sending request to gRPC Server...");
        let response = client.say_hello(request).await?;
    
        println!("RESPONSE={:?}", response);
    
        Ok(())
    }
    else{
        
        Err(panic!())
    }

}