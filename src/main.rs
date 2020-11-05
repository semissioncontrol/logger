// semissioncontrol/logger - (c) 2020 SEMC

// This module reads the /semc/logger directory
// for error and warning files. It then outputs them
// and alerts the user if a module has spit one out.

// Theory
// Each module gets its own directory at /semc/logger.
// This will be managed by semissioncontrol/core, right
// after the installation process of each module. Each
// module must spit out error files to /semc/logger/{module}.
// Furthermore, commands from the cli will always use 
// `2>` to output errors. 

// Written by: Bobbbay Bobbayan

use std::{thread, time, fs, env};
use std::time::{SystemTime, UNIX_EPOCH};
fn main() {
    let args: Vec<String> = env::args().collect();

    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    
    let mut time = 10; // Set time in ms
    
    let mut root_addr: String = "/semc/logger".to_owned();

    // If args are larger than 1 then parse these args 
    if(args.len() > 1) {
      if(args[1] == "-e") {
        time = args[2].parse::<u64>().unwrap();
      } 
      else if (args[1] == "-m") {
        let path: &str = &args[2];
        root_addr.push_str(path);     // I believe this is how you concatenate strings in rust so this may or may not work 
      }
    }

    // Find all logger directories on startup. As
    // mentioned in theory, each module should have 
    // its own logger directory.
    let mut directories = vec![];
    for entry in fs::read_dir(root_addr) {
        directories.extend(entry)
    }

    // Now that we know all the directories in /semc/logger,
    // we can have an infinite loop that will read these
    // for files.
    loop {

        if(args.len() > 1) {
          if(args[1] == "-t") {
            let start2 = SystemTime::now();
            let since_the_epoch2 = start2
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards"); 
            if(since_the_epoch2.as_millis() - since_the_epoch.as_millis() > args[2].parse::<u128>().unwrap()) {
              break;
            }
          }
        }


        // file_num stores the amount of status files
        // detected. This number should hopefully be as
        // little as possible, because status files are
        // currently only outputted on error or warning.
        let mut file_num = 0;
        let mut err_files = vec![];
        for i in &directories {
            for entry in fs::read_dir( i.as_ref().unwrap().path() ) {
                err_files.extend(entry);
            }
            // If a file was indeed found, add 1 to file_num
            if err_files.len() != 0 {
                file_num += 1;
            }
        }
        // Finally, print out the status files and wait 1ms

        if(file_num > 0) {
          println!("{:?}", err_files);
        }
        thread::sleep(time::Duration::from_millis(time));
    }
}
