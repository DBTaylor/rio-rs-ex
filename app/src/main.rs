extern crate nifpga;

use nifpga::{NifpgaError};

#[allow(dead_code)]
mod fpga;

use fpga::Fpga;

fn main() -> Result<(), NifpgaError>{
    //open the session
    //it will be closed when it goes out of scope
    let fpga = Fpga::open()?;

    //open target-to-host FIFO with 100 element buffer requested
    //this configures the FIFO and starts it
    //it will be stopped when it goes out of scope
    let (reader, _) = fpga.open_target_to_host(100)?;
    
    //open a target-to-host FIFO with 100 element buffer requested
    //these FIFOs form a loopback through the FPGA
    let (writer, _) = fpga.open_host_to_target(100)?;
    
    //write 2 values to the FIFO
    writer.write(&[5.0, 5.0], 1000)?;

    unsafe{
        //acquire 4 elements in the host write buffer
        //these elements will not be sent to the target until they go out of scope
        //this method is unsafe because elements must be dropped in the order they are acquired
        let( elements, _, _) = writer.acquire_elements(4, 1000)?;
        elements.slice.iter_mut().for_each(|el| {*el = 1.0});
    }

    let mut read_buff: [f32; 3] = Default::default();
    //read 3 elements from the FIFO
    reader.read(&mut read_buff, 1000)?;
    println!("{:?}", read_buff);
    unsafe{
        //acquire 3 elements in the host read buffer
        //this section of the buffer will not be available for the FIFO until these elements are dropped
        //this method is unsafe because elements must be dropped in the order they are acquired
        let(elements, _, _) = reader.acquire_elements(3, 1000)?;
        println!("{:?}", elements.slice)
    }
    
    let mut arr = [true, true, true, false, false, false, true, false];
    loop {
        //loopback through the FPGA. The first element in this array sets the user LED state
        fpga.write_bools(&arr)?;
        println!("{:?}", fpga.read_bool_outs()?);

        //another loopback through the FPGA
        fpga.write_bool_arr(&arr)?;
        println!("{:?}", fpga.read_bool_arr_out()?);

        std::thread::sleep(std::time::Duration::from_millis(500));
        //not every element in arr
        for el in arr.iter_mut(){
            *el = !*el;
        }
    };
}