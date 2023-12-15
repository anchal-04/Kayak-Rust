use rand::distributions::{Distribution, Uniform};

fn main() {
    let mut range = rand::thread_rng();
    
    // Using Uniform distribution for generating random numbers between 100 and 250, where both 100 and 250 are included
    let student_weight_distribution = Uniform::new_inclusive(100, 250);

    // A student_id_weight_vector will be created which will contain 100 students.
    // Vector contains tuples with i32 (32 bit signed integer) datatype
    // Using map function of the vector to initialize the vector with student id and student weight tuple pair
    // Student weight is randomly genereated using Uniform distribution
    let mut student_id_weight_vector: Vec<(i32, i32)> = (1..=100)
        .map(|index| (index, student_weight_distribution.sample(&mut range)))
        .collect();
    println!("Students with their id and weight: {:?}", student_id_weight_vector);

    // Sorting the student_id_weight_vector based on student weigh i.e. the 2nd element of tuple (0 based indexing)
    student_id_weight_vector.sort_by_key(|k| k.1);

    let mut i = 0; // first student id
    let mut j = 99; // last student id
    let mut number_of_kayaks_required = 0; // iterator to assign kayak id

    // Since the student_id_weight_vector is sorted based on student weight,
    // we use 2 pointer approach here. We try to make the Kayak pair of lightest and heaviest student.
    // If their weights combined are less than or equal to 300, then they go to the Kayak together. 
    // Else the heavier student goes alone as its not possible to pair him up with anyone else for the Kayak.
    // We keep doing this unless each student is assigned a Kayak.
    while i < j {
        number_of_kayaks_required += 1;
        if student_id_weight_vector[i].1 + student_id_weight_vector[j].1 <= 300 {
            println!("Student with ID {:?} and weight {:?} will use Kayak with ID {:?}", 
            student_id_weight_vector[i].0, student_id_weight_vector[i].1, number_of_kayaks_required);
            
            println!("Student with ID {:?} and weight {:?} will use Kayak with ID {:?}", 
            student_id_weight_vector[j].0, student_id_weight_vector[j].1, number_of_kayaks_required);
            
            i += 1;
            j -= 1;
        }
        
        else {
            println!("Student with ID {:?} and weight {:?} will use Kayak with ID {:?}", 
            student_id_weight_vector[j].0, student_id_weight_vector[j].1, number_of_kayaks_required);
            
            j -= 1;
        }
    }

    println!("Minimum amount of dollars required such that all the 100 students could participate in this Kayak experience: {:?}$", number_of_kayaks_required*20);
    println!("Number of Kayaks required to allow all the students to enjoy this experience: {:?}", number_of_kayaks_required);
}
