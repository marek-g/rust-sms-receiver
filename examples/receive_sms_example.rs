use sms_receiver::SMSReceiver;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sms_receiver = SMSReceiver::connect()?;

    loop {
        std::thread::sleep(Duration::from_millis(100));

        if let Some(sms) = sms_receiver.check_for_message()? {
            println!("SMS received: {:?}", sms);
        }
    }
}
