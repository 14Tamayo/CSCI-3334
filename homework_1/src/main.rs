const FAH_FREEZING: f64  = 32.0;


fn main() {
    let temperature: f64 = 100.0;
    let arr = [33, 34, 35, 36, 37];

    let celsiusTemp: f64 = fahrenheit_to_celsius(temperature);
    let FahrenheitTemp: f64 = celsius_to_fahrenheit(celsiusTemp);

    println!("The temperature in celsius is: {celsiusTemp}");
    println!("The temperature in fahrenheit is: {FahrenheitTemp}");

    

    
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
