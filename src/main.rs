mod location;
mod transaction;

use transaction::Transaction;
use std::fs::File;
use std::io::{self, BufReader, BufRead};

fn main() {
    // a. create file variable by passing "./transactions.csv" into the File::open function,
    // followed by calling the unwrap method
    let file: File = File::open("./transactions.csv").unwrap();
    //No graceful error handling here

    // b. create reader variable by passing file variable into the BufReader::new function
    let reader: BufReader<File> = BufReader::new(file);
    
    // c. create mutable transactions variable of type Vec<Transaction> by calling Vec::new
    // method
    let mut transactions:Vec<Transaction> = Vec::new(); 

    // d. create mutable skipped_lines variable of no explicit type simply calling Vec::new method
    let mut skipped_lines: Vec<_> = Vec::new();

    //e. run a for loop destructured into arbitrary variables of (idx, line) using the reader variable
    // which calls lines method followed by enumerate method
    for (index, line) in reader.lines().enumerate() {//calling enumerate() provides the index
        // - if idx equals to 0, continue
        if index == 0{
            continue;
        }

        // - create line_str variable by using line to call the unwrap method
        let line_str = line.unwrap();

        // - create parsed_transaction variable by passing line_str into
        // Transaction::fram_csv_line method
        let parsed_transaction = Transaction::from_csv_line(&line_str);

        // - match on parsed_transaction
        // - if matches on Ok variant, push value within Ok into transactions
        // - If matches on Err variant, push the tuple of (idx, value within Err, line_str)
        // into skipped_lines
        match parsed_transaction {
            Ok(transaction) => transactions.push(transaction),
            Err(errorString) => skipped_lines.push((index, errorString, line_str))
        }

        


    }

    // f. run a for loop by using transactions to call the iter method
    // - print every item in transactions
    for transaction in transactions.iter(){
        println!("{:?} was successfully parsed", transaction);
    };

    // g. do the same for skipped_lines
    for skipped_line in skipped_lines.iter(){
        println!("{:?} was not successfully parsed", skipped_line);
    };

    
}