mod codicefiscale;


fn main() {
    
    let name_check = "Mario";
    let lastname_check = "Rossi";
    let date_check = "1987-11-26";
    let gender_check = "F";
    let birthprovince = "NA";
    let birthplace = "NAPOLI";

    let result = codicefiscale::codicefiscale::calculate(
        name_check, lastname_check, date_check, gender_check, birthprovince, birthplace);

    println!("{}", result);
   
}