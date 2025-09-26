const FAH_FREEZING: f64  = 32.0;


fn main() {
    //Variable definitions
    let temperature: f64 = 100.0;
    let arr = [33, 34, 35, 36, 37];
    let mut arrTemp;

    //Calling functions & storing results
    let celsiusTemp: f64 = fahrenheit_to_celsius(temperature);
    let FahrenheitTemp: f64 = celsius_to_fahrenheit(celsiusTemp);

    //Printing results
    println!("The temperature in celsius is: {:.2}", celsiusTemp);
    println!("The temperature in fahrenheit is: {:.2}", FahrenheitTemp);

    
    //Loop for calling functions on temperatures in the array
    println!("Array Temperatures");
    for element in arr {
        arrTemp = fahrenheit_to_celsius(element as f64);
        println!("The temperature in celsius is: {:.2}", arrTemp);

        arrTemp = celsius_to_fahrenheit(element as f64);
        println!("The temperature in fahrenheit is: {:.2}", arrTemp);

    }

    
}


fn fahrenheit_to_celsius(fTemp: f64) -> f64 {
    //Operation
    let cTemp = (fTemp - 32.0) * (5.0/9.0);

    //return value
    cTemp
}


fn celsius_to_fahrenheit(cTemp: f64) -> f64 {
    //Operation
    let fTemp = (cTemp * (9.0/5.0)) + 32.0;

    //return value
    fTemp    
}
