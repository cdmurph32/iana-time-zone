pub(crate) mod bindings {
    wit_bindgen::generate!("interfaces");
}

use bindings::wasi::clocks;
use bindings::wasi::clocks::timezone::TimezoneDisplay;

pub(crate) fn get_timezone_inner() -> Result<String, crate::GetTimezoneError> {
    let datetime = clocks::wall_clock::now();
    let timezone_display: TimezoneDisplay = clocks::timezone::display(datetime);

    Ok(format!("{:?}", timezone_display.name))
}

#[cfg(test)]
mod tests {

    #[test]
    fn pass() {
        let tz = super::get_timezone_inner().unwrap();
        println!("tz={:?}", tz);
    }
}
