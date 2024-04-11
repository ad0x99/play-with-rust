mod create_new_thread;
mod transfer_data_using_channels;

fn main() {
    println!("============Create new thread============");
    create_new_thread::run_new_thread();

    println!("============Waiting for all threads to finish with join handles============");
    create_new_thread::wait_for_all_threads_to_finish_with_join_handles();

    println!("============Using move Closures with Threads============");
    create_new_thread::move_closure_with_thread();
    create_new_thread::invalid_reference();

    println!("============Transferring Data Between Threads Using Channels============");
    transfer_data_using_channels::run_channels();
}
