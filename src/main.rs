use srfng;

fn main() {

    let mut gen = srfng::Generator::new();

    println!("{}", gen.generate().as_str());

}
