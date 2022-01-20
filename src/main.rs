use srfng;

fn main() {

    let mut gen = srfng::generator::Generator::new();

    println!("{}", gen.generate().as_str());

}
