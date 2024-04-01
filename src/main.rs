use std::collections::HashMap;
use std::io;

#[derive(Debug, Clone)]
pub enum StaffType {
    Lecturer(Lecturer),
    OfficeSpecialist(OfficeSpecialist),
}

#[derive(Debug)]
pub enum ManagerSelection {
    AddEmployee,
    RemoveEmployee,
    AdjustEmployee,
    ShowEmployee,
}
#[derive(Debug, Clone)]
pub struct Lecturer {
    faculty: String,
    name: String,
    id: String,
    birthday: String,
    salary: i32,
    level: String,
    salary_coefficient: i32,
    teaching_hours: i32,
    basic_salary: i32,
    reward: i32,
    completion_level: String,
}
#[derive(Debug, Clone)]
pub struct OfficeSpecialist {
    office: String,
    name: String,
    id: String,
    birthday: String,
    salary: i32,
    level: String,
    salary_coefficient: i32,
    day_working: i32,
    basic_salary: i32,
    reward: i32,
    completion_level: String,
}
fn get_string() -> Option<String> {
    let mut buf = String::from("");
    while io::stdin().read_line(&mut buf).is_err() {
        println!("Wrong input");
    }
    let input = buf.trim().to_owned();
    if &input == "" {
        None
    } else {
        Some(input)
    }
}
fn get_integer() -> Option<i32> {
    let mut buf = String::from("");
    while io::stdin().read_line(&mut buf).is_err() {
        println!("Wrong input");
    }
    let input: Option<i32> = match buf.trim().parse() {
        Ok(num) => Some(num),
        Err(_) => None,
    };
    input
}
pub trait Staff {
    fn new() -> Self;
    fn enter_staff(&mut self) -> Self;
    fn print_staff(&self);
    fn get_salary(&mut self) -> Option<i32>;
    fn evaluate_staff(&mut self) -> (i32, String);
    fn edit_list(&self) -> i32;
}
impl Staff for Lecturer {
    fn new() -> Self {
        Lecturer {
            faculty: String::from(""),
            name: String::from(""),
            id: String::from(""),
            birthday: String::from(""),
            salary: 0,
            level: String::from(""),
            salary_coefficient: 0,
            teaching_hours: 0,
            basic_salary: 1490000,
            reward: 0,
            completion_level: String::from(""),
        }
    }
    fn enter_staff(&mut self) -> Self {
        println!("=======================");
        println!("Enter faculty:");
        self.faculty = get_string().unwrap();
        println!("Enter name:");
        self.name = get_string().unwrap();
        println!("Enter id:");
        self.id = get_string().unwrap();
        println!("Enter birthday:");
        self.birthday = get_string().unwrap();
        println!("Enter level:");
        self.level = get_string().unwrap();
        println!("Enter salary coefficient:");
        self.salary_coefficient = get_integer().unwrap_or(0);
        println!("Enter teaching hours:");
        self.teaching_hours = get_integer().unwrap_or(0);
        let (_reward, _completion_level) = self.evaluate_staff();
        println!("=======================");
        Lecturer {
            faculty: self.clone().faculty,
            name: self.clone().name,
            id: self.clone().id,
            birthday: self.clone().birthday,
            salary: self.clone().get_salary().unwrap(),
            level: self.clone().level,
            salary_coefficient: self.clone().salary_coefficient,
            reward: _reward,
            teaching_hours: self.clone().teaching_hours,
            completion_level: _completion_level,
            basic_salary: self.clone().basic_salary,
        }
    }
    fn get_salary(&mut self) -> Option<i32> {
        let level_of_lecturer = self.level.to_lowercase();
        let allowance = match level_of_lecturer.as_str() {
            "engineer" => 1200000,
            "master" => 2500000,
            "phd" => 4500000,
            _ => 0,
        };
        self.salary = self.basic_salary * self.salary_coefficient
            + self.teaching_hours * 120000
            + allowance
            + self.reward;
        Some(self.salary)
    }
    fn print_staff(&self) {
        println!("{:?}", self);
    }
    fn evaluate_staff(&mut self) -> (i32, String) {
        match self.teaching_hours {
            0..=29 => {
                self.reward = 0;
                self.completion_level = String::from("Don't complete");
            }
            30..=99 => {
                self.reward = 500000;
                self.completion_level = String::from("Complete");
            }
            100.. => {
                self.reward = 1500000;
                self.completion_level = String::from("Very good");
            }
            _ => {}
        }
        (self.clone().reward, self.clone().completion_level)
    }
    fn edit_list(&self) -> i32 {
        println!("=======================");
        println!("1.ID");
        println!("2.Faculty");
        println!("3.Birthday");
        println!("4.Level");
        println!("5.Salary coefficient");
        println!("6.Teaching hours");
        println!("=======================");
        print!("Choose your selection:");
        let mut selection = get_integer().unwrap_or(0);
        while selection <= 0 || selection >= 7 {
            println!("Wrong");
            selection = get_integer().unwrap_or(0);
        }
        selection
    }
}
impl Staff for OfficeSpecialist {
    fn new() -> Self {
        OfficeSpecialist {
            office: String::from(""),
            name: String::from(""),
            id: String::from(""),
            birthday: String::from(""),
            salary: 0,
            level: String::from(""),
            salary_coefficient: 0,
            day_working: 0,
            basic_salary: 1490000,
            reward: 0,
            completion_level: String::from(""),
        }
    }
    fn enter_staff(&mut self) -> Self {
        println!("=======================");
        println!("Enter faculty");
        self.office = get_string().unwrap();
        println!("Enter name");
        self.name = get_string().unwrap();
        println!("Enter id");
        self.id = get_string().unwrap();
        println!("Enter birthday");
        self.birthday = get_string().unwrap();
        println!("Enter level");
        self.level = get_string().unwrap();
        println!("Enter salary coefficient");
        self.salary_coefficient = get_integer().unwrap_or(0);
        println!("Enter teaching hours");
        self.day_working = get_integer().unwrap_or(0);
        let (_reward, _completion_level) = self.evaluate_staff();
        println!("=======================");
        OfficeSpecialist {
            office: self.clone().office,
            name: self.clone().name,
            id: self.clone().id,
            birthday: self.clone().birthday,
            salary: self.clone().get_salary().unwrap(),
            level: self.clone().level,
            salary_coefficient: self.clone().salary_coefficient,
            reward: _reward,
            day_working: self.clone().day_working,
            completion_level: _completion_level,
            basic_salary: self.clone().basic_salary,
        }
    }
    fn get_salary(&mut self) -> Option<i32> {
        let level_of_office_specialist = self.level.to_lowercase();
        let allowance = match level_of_office_specialist.as_str() {
            "officer" => 1200000,
            "deputy" => 2500000,
            "head" => 4200000,
            _ => 0,
        };
        self.salary = self.basic_salary * self.salary_coefficient
            + self.day_working * 110000
            + allowance
            + self.reward;
        Some(self.salary)
    }
    fn print_staff(&self) {
        println!("{:?}", self);
    }
    fn evaluate_staff(&mut self) -> (i32, String) {
        match self.day_working {
            0..=29 => {
                self.reward = 0;
                self.completion_level = String::from("Don't complete");
            }
            30..=99 => {
                self.reward = 500000;
                self.completion_level = String::from("Complete");
            }
            100.. => {
                self.reward = 1500000;
                self.completion_level = String::from("Very good");
            }
            _ => {}
        }
        (self.clone().reward, self.clone().completion_level)
    }
    fn edit_list(&self) -> i32 {
        println!("=======================");
        println!("1.ID");
        println!("2.Office");
        println!("3.Birthday");
        println!("4.Level");
        println!("5.Salary coefficient");
        println!("6.Teaching hours");
        println!("=======================");
        println!("Choose your selection");
        let mut selection = get_integer().unwrap_or(0);
        while selection <= 0 || selection >= 7 {
            selection = get_integer().unwrap_or(0);
        }
        selection
    }
}
#[derive(Debug, Clone)]
pub struct StaffList {
    officer_list: HashMap<String, OfficeSpecialist>,
    lecturer_list: HashMap<String, Lecturer>,
}
impl StaffList {
    fn new() -> Self {
        StaffList {
            officer_list: HashMap::new(),
            lecturer_list: HashMap::new(),
        }
    }
    fn add_staff(&mut self, staff: &StaffType) {
        match staff {
            StaffType::Lecturer(lec) => {
                self.lecturer_list.insert(lec.clone().name, lec.clone());
            }
            StaffType::OfficeSpecialist(officer) => {
                self.officer_list
                    .insert(officer.clone().name, officer.clone());
            }
        }
    }
    fn remove_staff(&mut self, name: &str, list: &str) -> bool {
        match list {
            "lecturer" => self.lecturer_list.remove(name).is_some(), // neu nhu k co gia tri la false, co la true
            "officer" => self.officer_list.remove(name).is_some(),
            _ => false,
        }
    }
    fn view_staff_lecturer(&self) -> Vec<&Lecturer> {
        self.lecturer_list.values().collect()
    }
    fn view_staff_officer(&self) -> Vec<&OfficeSpecialist> {
        self.officer_list.values().collect()
    }
    }
pub mod manager {
    use crate::{get_integer, get_string, Staff, StaffList, StaffType};
    pub fn enter_staff(staff_list: &mut StaffList, enter_list: &str) {
        match enter_list {
            "lecturer" => {
                let mut lec = crate::Lecturer::new();
                lec = lec.enter_staff();
                staff_list.add_staff(&StaffType::Lecturer(lec));
            }
            "officer" => {
                let mut officer = crate::OfficeSpecialist::new();
                officer = officer.enter_staff();
                staff_list.add_staff(&StaffType::OfficeSpecialist(officer));
            }
            _ => (),
        }
        println!("{:?}", staff_list.lecturer_list);
        println!("{:?}", staff_list.officer_list);
    }
    pub fn remove_staff(staff_list: &mut StaffList, remove_list: &str) {
        println!("=======================");
        println!("Enter employee you want to remove");
        let name_remove_employee = get_string().unwrap();
        let is_true = match remove_list {
            "lecturer" => staff_list.remove_staff(name_remove_employee.as_str(), remove_list),
            "officer" => staff_list.remove_staff(name_remove_employee.as_str(), remove_list),
            _ => false,
        };
        if is_true == true {
            println!("Remove success");
        } else {
            println!("Don't find that employee");
        }
        println!("=======================");
    }
    pub fn view_staff(staff_list: &StaffList, view_staff: &str) {
        match view_staff {
            "lecturer" => {
                println!("=======================");
                println!("List of lecturer");
                for staff in staff_list.view_staff_lecturer() {
                    staff.print_staff();
                }
                println!("=======================");
            }
            "officer" => {
                println!("=======================");
                println!("List of office specialist");
                for staff in staff_list.view_staff_officer() {
                    staff.print_staff();
                }
                println!("=======================");
            }
            _ => (),
        }
    }
    pub fn adjust_staff(staff_list: &mut StaffList, adjust_list: &str) {
        println!("Enter staff for adjust");
        let find_employee_name = get_string().unwrap();

        match adjust_list {
            "lecturer" => {
                match staff_list.lecturer_list.get_mut(&find_employee_name) {
                    Some(staff) => {
                        staff.print_staff();
                        match staff.edit_list() {
                            1 => {
                                println!("Enter id:");
                                staff.id = get_string().unwrap();
                            }
                            2 => {
                                println!("Enter faculty:");
                                staff.faculty = get_string().unwrap();
                            }
                            3 => {
                                println!("Enter birthday:");
                                staff.birthday = get_string().unwrap();
                            }
                            4 => {
                                println!("Enter level:");
                                staff.level = get_string().unwrap();
                                staff.salary = staff.get_salary().unwrap();
                            }
                            5 => {
                                println!("Enter salary coefficient:");
                                staff.salary_coefficient = get_integer().unwrap();
                                staff.salary = staff.get_salary().unwrap();
                            }
                            6 => {
                                println!("Enter teaching hours");
                                staff.teaching_hours = get_integer().unwrap();
                                let (_reward, _completion_level) = staff.evaluate_staff();
                                staff.reward = _reward;
                                staff.completion_level =_completion_level;
                                staff.salary = staff.get_salary().unwrap();
                                

                            }
                            _ => (),
                        }
                    }
                    None => {
                        println!("There is no employee");
                    }
                }
            }
            "officer" => {
                match staff_list.officer_list.get_mut(&find_employee_name) {
                    Some(staff)=>{
                        staff.print_staff();
                        match staff.edit_list() {
                            1 => {
                                println!("Enter id:");
                                staff.id = get_string().unwrap();
                            }
                            2 => {
                                println!("Enter office:");
                                staff.office = get_string().unwrap();
                            }
                            3 => {
                                println!("Enter birthday:");
                                staff.birthday = get_string().unwrap();
                            }
                            4 => {
                                println!("Enter level:");
                                staff.level = get_string().unwrap();
                                staff.salary = staff.get_salary().unwrap();
                            }
                            5 => {
                                println!("Enter salary coefficient:");
                                staff.salary_coefficient = get_integer().unwrap();
                                staff.salary = staff.get_salary().unwrap();
                            }
                            6 => {
                                println!("Enter day working");
                                staff.day_working = get_integer().unwrap();
                                let (_reward, _completion_level) = staff.evaluate_staff();
                                staff.reward = _reward;
                                staff.completion_level =_completion_level;
                                staff.salary = staff.get_salary().unwrap();
                            }
                            _ => (),
                        }
                    }
                    None => {
                        println!("There is no employee");
                    }
                }
            }
            _ => (),
        }
    }
}
impl ManagerSelection {
    fn show_menu() {
        println!("==========MENU==========");
        println!("Hello, here is a menu for you");
        println!("==============================");
        println!("1. Add employee");
        println!("2. Adjust employee");
        println!("3. Remove employee");
        println!("4. Show list of employee");
        println!("5. Exit");
        println!("==============================");
    }
    fn enter_input_manager() -> Option<ManagerSelection> {
        let mut input = String::from("");
        let mut selection = ManagerSelection::get_input(&mut input).expect("Empty string");
        ManagerSelection::check_input(&mut selection);
        match selection {
            1 => Some(ManagerSelection::AddEmployee),
            2 => Some(ManagerSelection::AdjustEmployee),
            3 => Some(ManagerSelection::RemoveEmployee),
            4 => Some(ManagerSelection::ShowEmployee),
            5 => None,
            _ => None,
        }
    }
    fn select() {
        let mut staff_list = StaffList::new();
        let mut choice;
        loop {
            ManagerSelection::show_menu();
            choice = ManagerSelection::enter_input_manager();
            match choice {
                Some(ManagerSelection::AddEmployee) => match ManagerSelection::second_selection() {
                    1 => manager::enter_staff(&mut staff_list, "lecturer"),
                    2 => manager::enter_staff(&mut staff_list, "officer"),
                    _ => {}
                },
                Some(ManagerSelection::AdjustEmployee) => {
                    match ManagerSelection::second_selection() {
                        1 => manager::adjust_staff(&mut staff_list, "lecturer"),
                        2 => manager::adjust_staff(&mut staff_list, "officer"),
                        _ => {}
                    }
                }
                Some(ManagerSelection::RemoveEmployee) => {
                    match ManagerSelection::second_selection() {
                        1 => manager::remove_staff(&mut staff_list, "lecturer"),
                        2 => manager::remove_staff(&mut staff_list, "officer"),
                        _ => {}
                    }
                }
                Some(ManagerSelection::ShowEmployee) => {
                    match ManagerSelection::second_selection() {
                        1 => manager::view_staff(&staff_list, "lecturer"),
                        2 => manager::view_staff(&staff_list, "officer"),
                        _ => {}
                    }
                }
                None => {
                    println!("Exit program");
                    break;
                }
            }
        }
    }
    fn second_selection() -> i32 {
        println! {"Choose which staff to use"};
        println!("1.Office Specialist");
        println!("2.Staff Specialist");
        let mut choice = get_integer().unwrap_or(0);
        while choice <= 0 || choice >= 2 {
            println!("wrong");
            choice = get_integer().unwrap_or(0);
        }
        choice
    }

    fn get_input(input: &mut String) -> Option<i32> {
        input.clear();
        while io::stdin().read_line(input).is_err() {
            println!("Please try again");
        }
        let data = match input.trim().parse() {
            Ok(num) => Some(num),
            Err(_) => None,
        };
        data
    }
    fn check_input(data: &mut i32) {
        while data.clone() >= 6 || data.clone() <= 0 {
            println!("There is no selection");
            *data = ManagerSelection::get_input(&mut data.to_string()).unwrap();
        }
    }
}
fn main() {
    ManagerSelection::select();
}
