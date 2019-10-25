use std::io;

struct Subject{
    class_section: String,
    course_code: String
}

struct Student{
    name: String,
    s_num: String,
    subjects: Vec<Subject>
}

fn printMainMenu(){
    println!("Main Menu");
    println!("[1] Add student");
    println!("[2] View all Students");
    println!("[3] Add subject to student");
    println!("[4] Delete student");
    println!("[5] Exit");
}

fn addStudent(mut studList: &mut Vec<Student>){
    println!("Add Student");
    println!("Enter name: ");
    let mut studName = String::new();
    io::stdin().read_line(&mut studName).expect("failed");
    println!("Enter student number: ");
    let mut studNum = String::new();
    io::stdin().read_line(&mut studNum).expect("failed");

    let name = &studName;
    let num = &studNum;
    let listStd = studList.iter();
    let mut lstStdnt:Vec<Student> = Vec::new();
    let mut exists = false;

    let mut stdHldr:Student = Student{
        name: "0".to_string(),
        s_num: "0".to_string(),
        subjects: Vec::new()
    };

    if studList.is_empty(){
        println!("Saving student");
        let newStd:Student = Student{
            name: name.to_string(),
            s_num: num.to_string(),
            subjects: Vec::new()
        };
        studList.push(newStd);
        println!("Student saved successfully");
    }else {
        for std in listStd {
            if std.s_num == num.to_string(){
                println!("Student with student number {} already exists", studNum);
                exists = true;
                break;
            }
        }
        if !exists{
            println!("Saving student");
            let newStd:Student = Student{
                name: name.to_string(),
                s_num: num.to_string(),
                subjects: Vec::new()
            };
            stdHldr = newStd;
            println!("Student saved successfully");
            studList.push(stdHldr);
            return;
        }
    }
}

fn viewAllStds(mut studList: &mut Vec<Student>){
    let listStd = studList.iter();
    if studList.is_empty(){
        println!("No students in the directory");
    }else{
        for std in listStd {
            println!("Student Name: {}", std.name);
            println!("Student Number: {}", std.s_num);
            println!("[Subjects]:");
            if !std.subjects.is_empty(){
                let listSubj = std.subjects.iter();
                for subj in listSubj{
                    println!("Class Section: {}", subj.class_section);
                    println!("Course Code: {}", subj.course_code);
                }
            }else{
                println!("No subjects in list");
            }
        }
    }
}

fn deleteStudent(mut studList: &mut Vec<Student>){
    println!("Delete Student");
    if studList.is_empty(){
        println!("no students in the directory");
    }else {
        let mut exists = false;
        println!("Enter student the student-to-be-deleted's information");

        println!("Enter student's student number: ");
        let mut studNum = String::new();
        io::stdin()
            .read_line(&mut studNum)
            .expect("failed to read from stdin");
			
        let num = &studNum;
        let listStd = studList.iter();
        let mut lstStdnt:Vec<Student> = Vec::new();
        let mut exists = false;
        let mut index = 0;

        for std in listStd {
            if std.s_num == num.to_string(){
                println!("Student failed");
                println!("Deleting student");
                exists = true;
                break;
            }
            index = index + 1;
        }

        if !exists{
            println!("Student with entered student number does not exist");
        }else{
            studList.remove(index);
        }
    }
}

fn main() {
    let mut studList:Vec<Student> = Vec::new();
    let mut strChoice:String = String::new();
    let mut choice:i32 = 0;

    loop{
        printMainMenu();
        println!("choice: ");
        let mut choiceStr=String::new();
        io::stdin().read_line(&mut choiceStr).expect("failed");

        let choice = choiceStr.trim();
        match choice.parse::<u32>() {
            Ok(i)=>println!("choice: {}", i),
            Err(..)=>println!("error in: {}", choice),
        }

        match choice.parse::<u32>() {
            Ok(1)=>addStudent(&mut studList),
            Ok(2)=>viewAllStds(&mut studList),
            Ok(3)=>addSubjToStd(&mut studList),
            Ok(4)=>deleteStudent(&mut studList),
            Ok(5)=>break,
            Ok(_)=>println!("Invalid Input"),
            Err(..)=>println!("Invalid Input"),
        }
    }
}