use std::{time::{Duration, Instant}, fs::{File, self},cmp::max, path::Path, io::prelude::*, env};
mod insertion_sort;
mod mod_merge_sort;

fn time_n_merge_sorts_size_m(n:u32,m:usize,k: usize) -> Duration {
    let mut total_time: Duration = Duration::from_nanos(0);
    for _tests in 0..n{
        // print!("gen data for {:?} out of {:?}; ",tests+1,n);
        let mut test_ary: Vec<isize> = Vec::with_capacity(m);
        for _i in 0..m{
            test_ary.push(rand::random());
        };
        // println!("timing; ");
        test_ary.sort();
        test_ary.reverse();
        let now = Instant::now();
        mod_merge_sort::sort(& test_ary,k);
        total_time = total_time + now.elapsed();
        // println!("done");
        // println!("current average: {:?}",total_time/(tests+1));
    }
    return total_time/(n);
}

fn run_multiple_test(file_path_name: &String, test_amount: usize, max_n: usize, max_k: usize, timing_average_runs:u32){
    // Create folder for data
    _ = fs::create_dir_all(Path::new(file_path_name).parent().unwrap());
    
    // run test for test_amount, creating test_amount csv files
    for i in 1..=test_amount{
        // create file
        let mut data_file = File::create(format!("{}.{}.csv",file_path_name,i)).unwrap();
        // write header
        _ = data_file.write(b"n, k, time\n");
        // test for n 1000 to max_n with a step of 1000
        // example n1=1000, n2=2000, n3=3000
        for n in (1000..=max_n).step_by(1000){
            // test k values of 1 upp to 25
            for k in 1..=max_k{
                // run timing test 100 times and take average
                let avg_time: Duration = time_n_merge_sorts_size_m(timing_average_runs, n, k);
                // wire {n}, {k} {average time} to file
                let formatted_string: String = format!("{}, {}, {:?}\n", n, k, avg_time);
                _ = data_file.write(formatted_string.as_bytes());
            }

        }
    }
}
//  [200, 400, 800, 1600, 3_200, 6_400]
fn tmp_run_multiple_test(file_path_name: &String, test_amount: usize, timing_average_runs:u32, n_test_values: &[usize], max_k: usize){
    // Create folder for data
    _ = fs::create_dir_all(Path::new(file_path_name).parent().unwrap());
    
    // run test for test_amount, creating test_amount csv files
    for i in 1..=test_amount{
        // create file
        let mut data_file = File::create(format!("{}.{}.csv",file_path_name,i)).unwrap();
        // write header
        _ = data_file.write(b"k");
        for i in n_test_values{
            _ = data_file.write(format!(", n={}",i).as_bytes())
        }
        _ = data_file.write(b"\n");
        // test for n 1000 to max_n with a step of 1000
        // example n1=1000, n2=2000, n3=3000
        for k in 1..=max_k{
            // test k values of 1 upp to 25
            _ = data_file.write(format!("{k}").as_bytes());
            for n in n_test_values{
                // run timing test 100 times and take average
                let avg_time: Duration = time_n_merge_sorts_size_m(timing_average_runs, *n, k);
                // wire {n}, {k} {average time} to file
                _ = data_file.write(format!(", {:?}",avg_time).as_bytes());
            }
            _ = data_file.write(b"\n");
        }
    }
}
fn run_wide_k_test(file_path_name: &String,test_amount: usize, max_n: usize, timing_average_runs:u32){
    _ = fs::create_dir_all(Path::new(file_path_name).parent().unwrap());
    // create file
    for test_nr in 1..=test_amount{
        let mut data_file = File::create(format!("{}.wide_k.{}.csv",file_path_name,test_nr)).unwrap();
        // write header
        _ = data_file.write(b"n, k=n, k=n/2, k=n/4, k=n/16, k=n/32, k=12\n");
        for i in (10..=max_n).step_by(10){
            let time_merge_div_1: Duration = time_n_merge_sorts_size_m(timing_average_runs, i,i);
            let time_merge_div_2: Duration = time_n_merge_sorts_size_m(timing_average_runs, i,max(i/2,1));
            let time_merge_div_4: Duration = time_n_merge_sorts_size_m(timing_average_runs, i,max(i/4,1));
            let time_merge_div_16: Duration = time_n_merge_sorts_size_m(timing_average_runs, i,max(i/16,1));
            let time_merge_div_32: Duration = time_n_merge_sorts_size_m(timing_average_runs, i,max(i/32,1));
            let time_merge_12: Duration = time_n_merge_sorts_size_m(timing_average_runs, i,12);
            let formatted_string: String = format!("{}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}\n",
                                                  i,
                                                  time_merge_div_1,
                                                  time_merge_div_2,
                                                  time_merge_div_4,
                                                  time_merge_div_16,
                                                  time_merge_div_32,
                                                  time_merge_12);
            _ = data_file.write(formatted_string.as_bytes());
        }
    }
}

fn main() {


    let args: Vec<String> = env::args().collect();
    let index_of_output_flag: usize = args.iter().position(|r| r=="-o").unwrap();
    let file_path_name: &String = &args[index_of_output_flag+1];


    let is_wide_k: bool = args.contains(&"-wk".to_string());
    if is_wide_k{
        run_wide_k_test(file_path_name,1, 10_000, 100)
    } else {
        tmp_run_multiple_test(file_path_name, 1, 300,&[200, 400, 800, 1600, 3_200, 6_400,32_000],100)
        // run_multiple_test(file_path_name, 100, 50_000, 25, 100)
    }

}

#[cfg(test)]
mod unit_tests;
 